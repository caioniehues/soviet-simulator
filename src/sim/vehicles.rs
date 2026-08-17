//! Vehicles stage 1 (ticket #13): the truck is an owned persistent asset with
//! a 0..1 `ActiveVehicle` pawn linked through a first-class relationship
//! (architecture/ecs.md § Persistent state versus active pawn). Movement is
//! lane-following over the compiled road graph — the lane network *is* the
//! routing graph. Routes come from the async A* `PathService` (B4.1,
//! sim/pathfinding.rs); movement respects car-following lane space (B4.3,
//! `LaneOccupancy` in sim/traffic.rs).

use std::collections::HashMap;

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::resources::{Inventory, ResourceKind, TransportClass};
use super::storage::{StorageBand, StoragePolicies};
use super::roads::{LaneDir, RoadNode, RoadSegment};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};
use super::traffic::LaneOccupancy;

/// Base speed on a 1.0-modifier lane, m/s. Effective speed is
/// vehicle × road-class (× terrain, flat in M1).
pub const TRUCK_SPEED: f32 = 12.0;
pub const TRUCK_CARGO_CAPACITY: f32 = 10.0;
/// Tonnes moved per sim frame while docked at a yard.
pub const TRUCK_TRANSFER_RATE: f32 = 0.5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VehicleId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VehicleKind {
    Truck,
    /// Passenger transit vehicle (B5): serves a `transit::TransitLine`.
    Bus,
    /// Earthworks machine (B6): works `Skill::Groundworks` site phases.
    Excavator,
    /// Lifting machine (B6): works `Skill::Crane` site phases.
    Crane,
}

impl VehicleKind {
    /// Construction skill and work throughput per tick (W&R's
    /// `$SKILL_CONSTRUCTION_*` numeric skill — more/better machines on a
    /// site finish its phase faster; no matching machine stalls it).
    pub fn construction_skill(self) -> Option<(super::construction::Skill, f32)> {
        match self {
            VehicleKind::Excavator => Some((super::construction::Skill::Groundworks, 0.4)),
            VehicleKind::Crane => Some((super::construction::Skill::Crane, 0.5)),
            VehicleKind::Truck | VehicleKind::Bus => None,
        }
    }
}

/// Physical parking slots per depot (W&R's rule: fleet size *is* the slot
/// count — no abstract cap, a parked vehicle needs a painted slot).
pub const DEPOT_SLOTS: u32 = 6;

/// Persistent owned asset: survives pawn release, carries identity. The fleet
/// is finite — assets exist only through a depot purchase, never spawned by a
/// job. Parked/OnJob is derived, not stored: an asset with a live pawn
/// (`ActivePawn`) is on the road, one without sits in its depot slot.
#[derive(Component, Debug)]
pub struct VehicleAsset {
    pub id: VehicleId,
    pub kind: VehicleKind,
    /// The depot whose slot this asset occupies when parked.
    pub home_depot: Entity,
    /// Exactly one cargo class per vehicle (spec/vehicles.md) — the hard
    /// compatibility gate: a bulk tipper never carries boxed goods.
    pub cargo_class: TransportClass,
}

/// World position of a depot's parking slot `index` (two rows of three on
/// the apron south of the shed). Shared by the sim (future depot trips) and
/// the parked-truck rendering.
pub fn depot_slot_pos(depot_pos: Vec3, index: u32) -> Vec3 {
    depot_pos
        + Vec3::new(
            -7.0 + (index % 3) as f32 * 7.0,
            0.0,
            10.0 + (index / 3) as f32 * 5.5,
        )
}

/// Pawn → asset link. A proper relationship pair so despawn cleanup and
/// referential integrity come from the engine; creation/destruction still
/// goes through the named command barriers only.
#[derive(Component, Debug)]
#[relationship(relationship_target = ActivePawn)]
pub struct PawnOf(pub Entity);

/// Asset → its live pawn. 0..1 by construction (the dispatcher only spawns a
/// pawn for assets without one); `linked_spawn` despawns the pawn with the asset.
#[derive(Component, Debug)]
#[relationship_target(relationship = PawnOf, linked_spawn)]
pub struct ActivePawn(Vec<Entity>);

impl ActivePawn {
    pub fn pawn(&self) -> Option<Entity> {
        self.0.first().copied()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RouteLeg {
    pub segment: Entity,
    pub dir: LaneDir,
}

/// The live pawn: authoritative motion + carried stock. Trip *state* lives on
/// the asset (`dispatch::FreightJob`); presentation eases rendered transforms
/// toward `pos` (ADR 0003).
#[derive(Component, Debug)]
pub struct ActiveVehicle {
    pub pos: Vec3,
    /// Unit travel direction, for presentation orientation.
    pub heading: Vec3,
    pub route: Vec<RouteLeg>,
    pub leg: usize,
    /// Metres travelled along the current leg.
    pub s: f32,
    pub cargo: Inventory,
    /// Outstanding background path request (B4.1). Transient — never saved;
    /// a loaded pawn with an empty route simply re-requests.
    pub pending_path: Option<super::pathfinding::PathTicket>,
    /// Consecutive movement ticks in which car-following ate (nearly) the
    /// whole speed budget (B4.3). The B4.4 stall machine reads this to decide
    /// wait → re-route → register-stall. Transient, like `pending_path`.
    pub blocked_ticks: u32,
}

impl ActiveVehicle {
    /// A fresh pawn standing at `pos` with an empty truck bed.
    pub fn at(pos: Vec3) -> Self {
        Self {
            pos,
            heading: Vec3::X,
            route: Vec::new(),
            leg: 0,
            s: 0.0,
            cargo: Inventory::new(TRUCK_CARGO_CAPACITY),
            pending_path: None,
            blocked_ticks: 0,
        }
    }
}

#[derive(Resource, Default)]
pub struct VehicleEditQueue(pub Vec<VehicleEdit>);

#[derive(Clone, Copy, Debug)]
pub enum VehicleEdit {
    /// Fiat truck purchase (vehicle manufacture arrives in B10): applies only
    /// while the depot has a free slot — the sole way a vehicle enters the world.
    BuyTruck {
        depot: Entity,
        class: TransportClass,
    },
    /// Fiat bus purchase (B5): a Passenger-class asset in a depot slot —
    /// the freight dispatcher can never seize it.
    BuyBus { depot: Entity },
    /// Fiat construction-machine purchase (B6): slotted at a construction
    /// office; `kind` must carry a construction skill.
    BuyMachine {
        office: Entity,
        kind: VehicleKind,
    },
    /// Legacy shuttle, reimplemented as policy sugar (#35): sets a paired
    /// export band (0,0) on the source and an import band (0.9,1) on the sink
    /// for `resource`. The dispatcher does the hauling — no truck is seized,
    /// no standing order exists.
    CreateShuttle {
        from: Entity,
        to: Entity,
        resource: ResourceKind,
    },
}

#[derive(Resource, Default)]
pub struct VehicleIds {
    pub next: u64,
}

pub struct VehicleSimPlugin;

impl Plugin for VehicleSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VehicleEditQueue>()
            .init_resource::<VehicleIds>()
            // Treasury lives in PlanSimPlugin; init here too so plugin-subset
            // test apps still validate (init_resource is idempotent).
            .init_resource::<super::plan::Treasury>()
            .init_resource::<super::plan::AllocationFeedback>()
            .add_systems(
                SimTick,
                apply_vehicle_edits
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            );
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_vehicle_edits(
    mut commands: Commands,
    mut queue: ResMut<VehicleEditQueue>,
    mut ids: ResMut<VehicleIds>,
    buildings: Query<&Building>,
    mut policies: Query<&mut StoragePolicies>,
    fleet: Query<&VehicleAsset>,
    frame: Res<super::clock::FrameIndex>,
    mut budget: ResMut<super::plan::Treasury>,
    mut feedback: ResMut<super::plan::AllocationFeedback>,
) {
    // Slot occupancy as of this tick, extended in-drain so purchases earlier
    // in the queue count against the slots.
    let mut homed: HashMap<Entity, u32> = HashMap::new();
    for asset in &fleet {
        *homed.entry(asset.home_depot).or_default() += 1;
    }
    for edit in queue.0.drain(..) {
        match edit {
            VehicleEdit::BuyTruck { .. }
            | VehicleEdit::BuyBus { .. }
            | VehicleEdit::BuyMachine { .. } => {
                let (depot, kind, class, home_kind) = match edit {
                    VehicleEdit::BuyTruck { depot, class } => {
                        (depot, VehicleKind::Truck, class, BuildingKind::Depot)
                    }
                    VehicleEdit::BuyBus { depot } => (
                        depot,
                        VehicleKind::Bus,
                        TransportClass::Passenger,
                        BuildingKind::Depot,
                    ),
                    VehicleEdit::BuyMachine { office, kind } => {
                        if kind.construction_skill().is_none() {
                            warn!("BuyMachine dropped: {kind:?} has no construction skill");
                            continue;
                        }
                        (
                            office,
                            kind,
                            TransportClass::Machine,
                            BuildingKind::ConstructionOffice,
                        )
                    }
                    _ => unreachable!(),
                };
                if !buildings.get(depot).is_ok_and(|b| b.kind == home_kind) {
                    warn!("vehicle purchase dropped: {depot:?} is not a {home_kind:?}");
                    continue;
                }
                let occupied = homed.entry(depot).or_default();
                if *occupied >= DEPOT_SLOTS {
                    warn!("vehicle purchase dropped: depot {depot:?} has no free slot");
                    continue;
                }
                // The Plan's purse (G1.1): every vehicle costs allocation
                // points — a valid purchase the budget can't cover is
                // refused, not queued.
                let cost = match class {
                    TransportClass::Passenger => super::plan::BUS_COST,
                    TransportClass::Machine => super::plan::MACHINE_COST,
                    _ => super::plan::TRUCK_COST,
                };
                if !budget.try_spend(cost) {
                    feedback.0 = Some((frame.0, "vehicle"));
                    warn!("vehicle purchase refused: {cost:.0} pts, {:.0} available", budget.roubles);
                    continue;
                }
                *occupied += 1;
                ids.next += 1;
                commands.spawn(VehicleAsset {
                    id: VehicleId(ids.next),
                    kind,
                    home_depot: depot,
                    cargo_class: class,
                });
            }
            VehicleEdit::CreateShuttle { from, to, resource } => {
                // Paired policy sugar: everything at the source is surplus,
                // the sink demands up to 90% of its yard. The dispatcher
                // notices both on its next matching frame for the resource.
                if let Ok(mut source) = policies.get_mut(from) {
                    source.set(resource, Some(StorageBand::new(0.0, 0.0)));
                } else {
                    warn!("CreateShuttle: source {from:?} has no storage policies");
                }
                if let Ok(mut sink) = policies.get_mut(to) {
                    sink.set(resource, Some(StorageBand::new(0.9, 1.0)));
                } else {
                    warn!("CreateShuttle: sink {to:?} has no storage policies");
                }
            }
        }
    }
}

/// Move along the cached route; returns `true` once the route is exhausted
/// (arrived). Phase transitions are the caller's business.
///
/// B4.3 car-following: the advance on each leg is capped by the gap to the
/// leader in the same directed lane, and a leg transition only happens while
/// the next lane's mouth is clear — followers pile up at footprint spacing
/// and jams emerge from that alone. A tick spent held behind traffic bumps
/// `blocked_ticks`; free running resets it.
pub fn advance_along_route(
    me: Entity,
    vehicle: &mut ActiveVehicle,
    dt: f32,
    occupancy: &LaneOccupancy,
    segments: &Query<&RoadSegment>,
) -> bool {
    let mut budget = f32::MAX; // set from lane speed on the first leg below
    loop {
        let Some(leg) = vehicle.route.get(vehicle.leg).copied() else {
            vehicle.blocked_ticks = 0;
            return true;
        };
        // A recompiled-away segment severs the route: hold and let the next
        // depart() recompute (M1 keeps this rare; no roads are auto-removed).
        let Ok(segment) = segments.get(leg.segment) else {
            return false;
        };
        let Some(lane) = segment.lanes.iter().find(|l| l.dir == leg.dir) else {
            return false;
        };
        if budget == f32::MAX {
            budget = TRUCK_SPEED * lane.speed_modifier * dt;
        }
        let remaining = segment.length - vehicle.s;
        let gap = occupancy.gap_ahead(leg.segment, leg.dir, vehicle.s, me);
        if gap < remaining.min(budget) {
            // Leader ahead in reservation range: creep up to its reserved
            // space and hold — the whole leftover budget is forfeited.
            vehicle.s += gap;
            place_on_leg(vehicle, segment, leg.dir);
            vehicle.blocked_ticks += 1;
            return false;
        }
        if budget < remaining {
            vehicle.s += budget;
            place_on_leg(vehicle, segment, leg.dir);
            vehicle.blocked_ticks = 0;
            return false;
        }
        // Finishing this leg: only cross the junction if the next lane's
        // mouth has room, else wait at the segment end (spillback).
        if let Some(next) = vehicle.route.get(vehicle.leg + 1).copied()
            && !occupancy.entry_free(next.segment, next.dir, me)
        {
            vehicle.s = segment.length;
            place_on_leg(vehicle, segment, leg.dir);
            vehicle.blocked_ticks += 1;
            return false;
        }
        // Continue onto the next leg with the leftover budget.
        budget -= remaining;
        vehicle.s = segment.length;
        place_on_leg(vehicle, segment, leg.dir);
        vehicle.leg += 1;
        vehicle.s = 0.0;
    }
}

fn place_on_leg(vehicle: &mut ActiveVehicle, segment: &RoadSegment, dir: LaneDir) {
    let (pos, travel) = segment.point_at(vehicle.s, dir);
    let offset = segment
        .lanes
        .iter()
        .find(|l| l.dir == dir)
        .map_or(0.0, |l| l.offset);
    vehicle.heading = travel;
    vehicle.pos = pos + travel.cross(Vec3::Y) * offset;
}

/// A yard docks only onto a road node within this range: cutting a building's
/// access road really severs it (no cross-map teleport to the next node).
pub const DOCK_RADIUS: f32 = 40.0;

pub fn nearest_node(pos: Vec3, nodes: &Query<(Entity, &RoadNode)>) -> Option<Entity> {
    nodes
        .iter()
        .map(|(e, n)| (e, n.pos.distance_squared(pos)))
        .filter(|(_, d2)| *d2 <= DOCK_RADIUS * DOCK_RADIUS)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

/// Nearest node with no dock-radius gate: re-entry point for a vehicle that
/// is already out on the network (possibly mid-segment, far from any node).
pub fn nearest_node_unbounded(pos: Vec3, nodes: &Query<(Entity, &RoadNode)>) -> Option<Entity> {
    nodes
        .iter()
        .map(|(e, n)| (e, n.pos.distance_squared(pos)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
    };
    use super::super::dispatch::DispatchSimPlugin;
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
    use super::super::storage::StorageSimPlugin;
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            StorageSimPlugin,
            VehicleSimPlugin,
            DispatchSimPlugin,
        ));
        a
    }

    fn ticks(app: &mut App, n: u32) {
        for _ in 0..n {
            app.world_mut()
                .resource_mut::<Time>()
                .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
            app.update();
        }
    }

    /// Place a depot at `pos` and buy `trucks` of `class`; returns the depot.
    fn depot_with_trucks(app: &mut App, pos: Vec3, class: TransportClass, trucks: u32) -> Entity {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Depot,
                pos,
            });
        ticks(app, 2);
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        let depot = q
            .iter(world)
            .find(|(_, b)| b.kind == BuildingKind::Depot)
            .unwrap()
            .0;
        for _ in 0..trucks {
            world
                .resource_mut::<VehicleEditQueue>()
                .0
                .push(VehicleEdit::BuyTruck { depot, class });
        }
        depot
    }

    /// Straight dirt road x∈[0,100], quarry near x=0 (pre-filled with gravel),
    /// factory near x=100, docked depot with one bulk truck, legacy shuttle
    /// sugar quarry → factory. Returns (quarry, factory).
    fn shuttle_world(app: &mut App) -> (Entity, Entity) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Quarry,
            pos: Vec3::new(-10.0, 0.0, 0.0),
        });
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Factory,
            pos: Vec3::new(110.0, 0.0, 0.0),
        });
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Depot,
            pos: Vec3::new(0.0, 0.0, 15.0),
        });
        ticks(app, 2);
        let world = app.world_mut();
        let mut found = (None, None, None);
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::Quarry => found.0 = Some(e),
                BuildingKind::Factory => found.1 = Some(e),
                BuildingKind::Depot => found.2 = Some(e),
                _ => {}
            }
        }
        let (quarry, factory, depot) = (found.0.unwrap(), found.1.unwrap(), found.2.unwrap());
        world
            .get_mut::<Inventory>(quarry)
            .unwrap()
            .add(ResourceKind::Gravel, 50.0);
        let mut edits = world.resource_mut::<VehicleEditQueue>();
        edits.0.push(VehicleEdit::BuyTruck {
            depot,
            class: TransportClass::Bulk,
        });
        edits.0.push(VehicleEdit::CreateShuttle {
            from: quarry,
            to: factory,
            resource: ResourceKind::Gravel,
        });
        (quarry, factory)
    }

    #[test]
    fn purchases_stop_at_the_slot_count() {
        let mut app = app();
        depot_with_trucks(&mut app, Vec3::ZERO, TransportClass::Bulk, DEPOT_SLOTS + 3);
        ticks(&mut app, 2);
        let world = app.world_mut();
        assert_eq!(
            world.query::<&VehicleAsset>().iter(world).count(),
            DEPOT_SLOTS as usize,
            "a parked vehicle needs a physical slot"
        );
    }

    #[test]
    fn a_broke_treasury_refuses_the_purchase() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<super::super::plan::Treasury>()
            .roubles = super::super::plan::TRUCK_COST + 0.5;
        depot_with_trucks(&mut app, Vec3::ZERO, TransportClass::Bulk, 2);
        ticks(&mut app, 2);
        let world = app.world_mut();
        assert_eq!(
            world.query::<&VehicleAsset>().iter(world).count(),
            1,
            "the second truck exceeds the treasury and is refused"
        );
        assert!(
            world
                .resource::<super::super::plan::AllocationFeedback>()
                .0
                .is_some(),
            "the refusal is surfaced for the HUD"
        );
    }

    #[test]
    fn create_shuttle_sets_the_paired_policy_bands() {
        let mut app = app();
        let (quarry, factory) = shuttle_world(&mut app);
        ticks(&mut app, 1);
        let world = app.world();
        let source = world.get::<StoragePolicies>(quarry).unwrap();
        let band = source.band(ResourceKind::Gravel).expect("export band set");
        assert_eq!((band.min_pct, band.max_pct), (0.0, 0.0));
        let sink = world.get::<StoragePolicies>(factory).unwrap();
        let band = sink.band(ResourceKind::Gravel).expect("import band set");
        assert_eq!((band.min_pct, band.max_pct), (0.9, 1.0));
    }

    #[test]
    fn shuttle_sugar_hauls_a_truckload_through_the_dispatcher() {
        let mut app = app();
        let (_, factory) = shuttle_world(&mut app);
        // match + assign + drive 100 m out/back on dirt + dock both ends
        ticks(&mut app, 2600);
        let delivered = app
            .world()
            .get::<Inventory>(factory)
            .unwrap()
            .amount(ResourceKind::Gravel);
        assert!(
            delivered >= TRUCK_CARGO_CAPACITY - 1e-2,
            "delivered = {delivered}"
        );
    }

    #[test]
    fn dispatched_pawn_is_linked_and_dies_with_its_asset() {
        let mut app = app();
        shuttle_world(&mut app);
        ticks(&mut app, 40); // matching frame + assignment + spawn flush
        let world = app.world_mut();
        let (pawn, pawn_of) = world.query::<(Entity, &PawnOf)>().single(world).unwrap();
        let asset = pawn_of.0;
        assert_eq!(
            world.get::<ActivePawn>(asset).unwrap().pawn(),
            Some(pawn),
            "relationship target must point back at the pawn"
        );
        // linked_spawn: despawning the asset takes the pawn with it
        world.despawn(asset);
        assert!(world.get_entity(pawn).is_err());
    }

    /// Hand-park a stationary pawn (no `PawnOf` → the freight machine never
    /// moves it) on `segment`'s forward lane at metre `s`.
    fn park_blocker(app: &mut App, segment: Entity, s: f32) {
        let mut blocker = ActiveVehicle::at(Vec3::new(s, 0.0, 1.5));
        blocker.route = vec![RouteLeg {
            segment,
            dir: LaneDir::Forward,
        }];
        blocker.s = s;
        app.world_mut().spawn(blocker);
    }

    #[test]
    fn follower_queues_behind_a_parked_leader_at_footprint_distance() {
        use super::super::traffic::VEHICLE_FOOTPRINT_M;
        let mut app = app();
        shuttle_world(&mut app);
        ticks(&mut app, 2);
        let world = app.world_mut();
        let seg = world
            .query::<(Entity, &RoadSegment)>()
            .single(world)
            .unwrap()
            .0;
        park_blocker(&mut app, seg, 50.0);
        // plenty of time to load and drive into the back of the queue
        ticks(&mut app, 1200);
        let world = app.world_mut();
        let (truck, _) = world
            .query::<(&ActiveVehicle, &PawnOf)>()
            .single(world)
            .unwrap();
        assert!(
            truck.s <= 50.0 - VEHICLE_FOOTPRINT_M + 1e-3,
            "must never enter the leader's reserved space, s = {}",
            truck.s
        );
        assert!(
            truck.s > 50.0 - VEHICLE_FOOTPRINT_M - 1.0,
            "must creep right up to the queue point, s = {}",
            truck.s
        );
        assert!(
            truck.blocked_ticks > 0,
            "held ticks must register for the stall machine"
        );
    }

    #[test]
    fn junction_entry_waits_for_a_clear_mouth() {
        let mut app = app();
        // Two chained dirt segments 0→100→200; shuttle quarry(west)→factory(east).
        {
            let mut roads = app.world_mut().resource_mut::<RoadEditQueue>();
            roads.0.push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
            roads.0.push(RoadEdit::Place {
                from: Vec3::new(100.0, 0.0, 0.0),
                to: Vec3::new(200.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        }
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Quarry,
                pos: Vec3::new(-10.0, 0.0, 0.0),
            });
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Factory,
                pos: Vec3::new(210.0, 0.0, 0.0),
            });
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Depot,
                pos: Vec3::new(0.0, 0.0, 15.0),
            });
        }
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut found = (None, None, None);
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::Quarry => found.0 = Some(e),
                BuildingKind::Factory => found.1 = Some(e),
                BuildingKind::Depot => found.2 = Some(e),
                _ => {}
            }
        }
        let (quarry, _factory, depot) = (found.0.unwrap(), found.1.unwrap(), found.2.unwrap());
        world
            .get_mut::<Inventory>(quarry)
            .unwrap()
            .add(ResourceKind::Gravel, 50.0);
        // the eastern segment (both nodes at x ≥ 100)
        let segs: Vec<(Entity, Entity, Entity)> = world
            .query::<(Entity, &RoadSegment)>()
            .iter(world)
            .map(|(e, s)| (e, s.a, s.b))
            .collect();
        let east = segs
            .into_iter()
            .find(|(_, a, b)| {
                let xa = world.get::<RoadNode>(*a).unwrap().pos.x;
                let xb = world.get::<RoadNode>(*b).unwrap().pos.x;
                xa.min(xb) > 50.0
            })
            .expect("eastern segment exists")
            .0;
        {
            let mut edits = world.resource_mut::<VehicleEditQueue>();
            edits.0.push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
            edits.0.push(VehicleEdit::CreateShuttle {
                from: quarry,
                to: found.1.unwrap(),
                resource: ResourceKind::Gravel,
            });
        }
        // blocker sits in the eastern lane's mouth (s < footprint)
        park_blocker(&mut app, east, 2.0);
        ticks(&mut app, 1500);
        let world = app.world_mut();
        let (truck, _) = world
            .query::<(&ActiveVehicle, &PawnOf)>()
            .single(world)
            .unwrap();
        let current = truck.route.get(truck.leg).expect("still en route");
        assert_ne!(
            current.segment, east,
            "must not cross the junction while the mouth is occupied"
        );
        assert!(
            truck.pos.x <= 100.0 + 1e-3,
            "held at the junction, x = {}",
            truck.pos.x
        );
        assert!(truck.blocked_ticks > 0);
        // B4.4: with no alternative corridor, re-routes come back identical
        // and are discarded — the counter climbs into a StallBoard entry and
        // the truck is still alive (never despawn).
        use super::super::traffic::{STALL_AFTER, StallBoard};
        assert!(
            truck.blocked_ticks >= STALL_AFTER,
            "no-alternative reroutes must not reset the counter, held {}",
            truck.blocked_ticks
        );
        let west = current.segment;
        let board = app.world().resource::<StallBoard>();
        let stall = board.0.get(&west).expect("held segment registers a stall");
        assert!(stall.vehicles >= 1);
    }

    #[test]
    fn blocked_truck_reroutes_around_a_jammed_corridor() {
        let mut app = app();
        // Direct corridor A(0,0)→J(100,0)→B(200,0); detour J→D(100,60)→B.
        // The eastern direct segment is physically plugged at the mouth and
        // saturated for density, so the held truck's re-route must take the
        // detour once congestion prices in — and the delivery must complete.
        {
            let mut roads = app.world_mut().resource_mut::<RoadEditQueue>();
            for (from, to) in [
                (Vec3::ZERO, Vec3::new(100.0, 0.0, 0.0)),
                (Vec3::new(100.0, 0.0, 0.0), Vec3::new(200.0, 0.0, 0.0)),
                (Vec3::new(100.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 60.0)),
                (Vec3::new(100.0, 0.0, 60.0), Vec3::new(200.0, 0.0, 0.0)),
            ] {
                roads.0.push(RoadEdit::Place {
                    from,
                    to,
                    class: RoadClass::Dirt,
                });
            }
        }
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Quarry,
                pos: Vec3::new(-10.0, 0.0, 0.0),
            });
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Factory,
                pos: Vec3::new(210.0, 0.0, 0.0),
            });
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Depot,
                pos: Vec3::new(0.0, 0.0, 15.0),
            });
        }
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut found = (None, None, None);
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::Quarry => found.0 = Some(e),
                BuildingKind::Factory => found.1 = Some(e),
                BuildingKind::Depot => found.2 = Some(e),
                _ => {}
            }
        }
        let (quarry, factory, depot) = (found.0.unwrap(), found.1.unwrap(), found.2.unwrap());
        world
            .get_mut::<Inventory>(quarry)
            .unwrap()
            .add(ResourceKind::Gravel, 50.0);
        // the direct eastern segment: both nodes on z ≈ 0, both x ≥ 100
        let segs: Vec<(Entity, Entity, Entity)> = world
            .query::<(Entity, &RoadSegment)>()
            .iter(world)
            .map(|(e, s)| (e, s.a, s.b))
            .collect();
        let east_direct = segs
            .into_iter()
            .find(|(_, a, b)| {
                let pa = world.get::<RoadNode>(*a).unwrap().pos;
                let pb = world.get::<RoadNode>(*b).unwrap().pos;
                pa.x.min(pb.x) > 50.0 && pa.z.abs() < 1.0 && pb.z.abs() < 1.0
            })
            .expect("direct eastern segment exists")
            .0;
        {
            let mut edits = world.resource_mut::<VehicleEditQueue>();
            edits.0.push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
            edits.0.push(VehicleEdit::CreateShuttle {
                from: quarry,
                to: factory,
                resource: ResourceKind::Gravel,
            });
        }
        // 30 parked pawns: mouth physically blocked + density → 100 over time
        for _ in 0..30 {
            park_blocker(&mut app, east_direct, 0.0);
        }
        ticks(&mut app, 5000);
        let delivered = app
            .world()
            .get::<Inventory>(factory)
            .unwrap()
            .amount(ResourceKind::Gravel);
        assert!(
            delivered >= TRUCK_CARGO_CAPACITY - 1e-2,
            "re-route around the jam must complete the delivery, delivered = {delivered}"
        );
    }

    #[test]
    fn moving_pawn_rides_the_forward_lane_offset() {
        let mut app = app();
        shuttle_world(&mut app);
        // truck starts at the depot's west node and drives east to the quarry
        // pickup... which is also the west node; so it loads, then hauls east.
        ticks(&mut app, 120); // load (20) + well into the eastbound drive
        let world = app.world_mut();
        let vehicle = world.query::<&ActiveVehicle>().single(world).unwrap();
        assert!(
            vehicle.pos.x > 0.0 && vehicle.pos.x < 100.0,
            "x = {}",
            vehicle.pos.x
        );
        // dirt road: width 6 → forward-lane offset magnitude 1.5, laterally in z
        assert!(
            (vehicle.pos.z.abs() - 1.5).abs() < 1e-3,
            "z = {}",
            vehicle.pos.z
        );
        assert_eq!(vehicle.heading, Vec3::X);
    }
}
