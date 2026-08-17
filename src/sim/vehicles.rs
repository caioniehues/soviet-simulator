//! Vehicles stage 1 (ticket #13): the truck is an owned persistent asset with
//! a 0..1 `ActiveVehicle` pawn linked through a first-class relationship
//! (architecture/ecs.md § Persistent state versus active pawn). Movement is
//! lane-following over the compiled road graph — the lane network *is* the
//! routing graph. The mine→plant shuttle loop here is an ad-hoc stub the B3
//! dispatcher replaces; routing is plain BFS until the packed search mirror
//! lands (ADR 0005).

use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::resources::{Inventory, ResourceKind, TransportClass};
use super::storage::{StorageBand, StoragePolicies};
use super::roads::{LaneDir, RoadNode, RoadSegment};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};

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
            .add_systems(
                SimTick,
                apply_vehicle_edits
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            );
    }
}

fn apply_vehicle_edits(
    mut commands: Commands,
    mut queue: ResMut<VehicleEditQueue>,
    mut ids: ResMut<VehicleIds>,
    buildings: Query<&Building>,
    mut policies: Query<&mut StoragePolicies>,
    fleet: Query<&VehicleAsset>,
) {
    // Slot occupancy as of this tick, extended in-drain so purchases earlier
    // in the queue count against the slots.
    let mut homed: HashMap<Entity, u32> = HashMap::new();
    for asset in &fleet {
        *homed.entry(asset.home_depot).or_default() += 1;
    }
    for edit in queue.0.drain(..) {
        match edit {
            VehicleEdit::BuyTruck { depot, class } => {
                if !buildings
                    .get(depot)
                    .is_ok_and(|b| b.kind == BuildingKind::Depot)
                {
                    warn!("BuyTruck dropped: {depot:?} is not a depot");
                    continue;
                }
                let occupied = homed.entry(depot).or_default();
                if *occupied >= DEPOT_SLOTS {
                    warn!("BuyTruck dropped: depot {depot:?} has no free slot");
                    continue;
                }
                *occupied += 1;
                ids.next += 1;
                commands.spawn(VehicleAsset {
                    id: VehicleId(ids.next),
                    kind: VehicleKind::Truck,
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
pub(crate) fn advance_along_route(
    vehicle: &mut ActiveVehicle,
    dt: f32,
    nodes: &Query<(Entity, &RoadNode)>,
    segments: &Query<&RoadSegment>,
) -> bool {
    let mut budget = f32::MAX; // set from lane speed on the first leg below
    loop {
        let Some(leg) = vehicle.route.get(vehicle.leg).copied() else {
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
        if budget < remaining {
            vehicle.s += budget;
            place_on_leg(vehicle, segment, leg.dir, nodes);
            return false;
        }
        // Finish this leg and continue onto the next with the leftover budget.
        budget -= remaining;
        vehicle.s = segment.length;
        place_on_leg(vehicle, segment, leg.dir, nodes);
        vehicle.leg += 1;
        vehicle.s = 0.0;
    }
}

fn place_on_leg(
    vehicle: &mut ActiveVehicle,
    segment: &RoadSegment,
    dir: LaneDir,
    nodes: &Query<(Entity, &RoadNode)>,
) {
    let (Ok((_, a)), Ok((_, b))) = (nodes.get(segment.a), nodes.get(segment.b)) else {
        return;
    };
    let (start, end) = match dir {
        LaneDir::Forward => (a.pos, b.pos),
        LaneDir::Backward => (b.pos, a.pos),
    };
    let travel = (end - start).normalize_or_zero();
    let right = travel.cross(Vec3::Y);
    let offset = segment
        .lanes
        .iter()
        .find(|l| l.dir == dir)
        .map_or(0.0, |l| l.offset);
    vehicle.heading = travel;
    vehicle.pos = start + travel * vehicle.s.min(segment.length) + right * offset;
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

/// Unweighted BFS over node adjacency — fine at M1 scale; the B3 dispatcher
/// brings cost-aware search over the packed mirror (ADR 0005).
pub fn find_route(
    start: Entity,
    goal: Entity,
    nodes: &Query<(Entity, &RoadNode)>,
    segments: &Query<&RoadSegment>,
) -> Option<Vec<RouteLeg>> {
    if start == goal {
        return Some(Vec::new());
    }
    let mut prev: HashMap<Entity, (Entity, RouteLeg)> = HashMap::new();
    let mut queue = VecDeque::from([start]);
    while let Some(node) = queue.pop_front() {
        let Ok((_, n)) = nodes.get(node) else {
            continue;
        };
        for &seg_entity in &n.segments {
            let Ok(segment) = segments.get(seg_entity) else {
                continue;
            };
            let (next, dir) = if segment.a == node {
                (segment.b, LaneDir::Forward)
            } else {
                (segment.a, LaneDir::Backward)
            };
            if next == start || prev.contains_key(&next) {
                continue;
            }
            prev.insert(
                next,
                (
                    node,
                    RouteLeg {
                        segment: seg_entity,
                        dir,
                    },
                ),
            );
            if next == goal {
                let mut legs = Vec::new();
                let mut at = goal;
                while at != start {
                    let (parent, leg) = prev[&at];
                    legs.push(leg);
                    at = parent;
                }
                legs.reverse();
                return Some(legs);
            }
            queue.push_back(next);
        }
    }
    None
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
