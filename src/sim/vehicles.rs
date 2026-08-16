//! Vehicles stage 1 (ticket #13): the truck is an owned persistent asset with
//! a 0..1 `ActiveVehicle` pawn linked through a first-class relationship
//! (architecture/ecs.md § Persistent state versus active pawn). Movement is
//! lane-following over the compiled road graph — the lane network *is* the
//! routing graph. The mine→plant shuttle loop here is an ad-hoc stub the B3
//! dispatcher replaces; routing is plain BFS until the packed search mirror
//! lands (ADR 0005).

use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use super::buildings::Building;
use super::clock::SECS_PER_PASS;
use super::resources::{Inventory, ResourceKind};
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

/// Persistent owned asset: survives pawn release, carries identity.
#[derive(Component, Debug)]
pub struct VehicleAsset {
    pub id: VehicleId,
    pub kind: VehicleKind,
}

/// Standing order stub: haul `resource` from one building's yard to another's.
/// Replaced wholesale by the B3 dispatcher.
#[derive(Component, Clone, Copy, Debug)]
pub struct ShuttleAssignment {
    pub from: Entity,
    pub to: Entity,
    pub resource: ResourceKind,
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShuttlePhase {
    Loading,
    Outbound,
    Unloading,
    Returning,
}

/// The live pawn: authoritative motion + trip state + carried stock.
/// Presentation eases rendered transforms toward `pos` (ADR 0003).
#[derive(Component, Debug)]
pub struct ActiveVehicle {
    pub pos: Vec3,
    /// Unit travel direction, for presentation orientation.
    pub heading: Vec3,
    pub phase: ShuttlePhase,
    pub route: Vec<RouteLeg>,
    pub leg: usize,
    /// Metres travelled along the current leg.
    pub s: f32,
    pub cargo: Inventory,
}

#[derive(Resource, Default)]
pub struct VehicleEditQueue(pub Vec<VehicleEdit>);

#[derive(Clone, Copy, Debug)]
pub enum VehicleEdit {
    CreateShuttle {
        from: Entity,
        to: Entity,
        resource: ResourceKind,
    },
}

#[derive(Resource, Default)]
struct VehicleIds {
    next: u64,
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
            )
            .add_systems(
                SimTick,
                dispatch_shuttles.in_set(SimStage::AllocationAndDispatch),
            )
            .add_systems(SimTick, run_shuttles.in_set(SimStage::MovementAndTransfers));
    }
}

fn apply_vehicle_edits(
    mut commands: Commands,
    mut queue: ResMut<VehicleEditQueue>,
    mut ids: ResMut<VehicleIds>,
) {
    for edit in queue.0.drain(..) {
        let VehicleEdit::CreateShuttle { from, to, resource } = edit;
        ids.next += 1;
        commands.spawn((
            VehicleAsset {
                id: VehicleId(ids.next),
                kind: VehicleKind::Truck,
            },
            ShuttleAssignment { from, to, resource },
        ));
    }
}

/// Assets with a standing order and no live pawn get one, spawned at the road
/// node nearest the source yard. The spawn flushes at the post-Commit barrier,
/// so the pawn participates from the next tick.
fn dispatch_shuttles(
    mut commands: Commands,
    idle: Query<(Entity, &ShuttleAssignment), (With<VehicleAsset>, Without<ActivePawn>)>,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    for (asset, order) in &idle {
        let (Ok(from_b), Ok(to_b)) = (buildings.get(order.from), buildings.get(order.to)) else {
            continue;
        };
        let (Some(start), Some(goal)) = (
            nearest_node(from_b.pos, &nodes),
            nearest_node(to_b.pos, &nodes),
        ) else {
            continue;
        };
        // No path yet: leave the asset parked and retry next tick.
        if find_route(start, goal, &nodes, &segments).is_none() {
            continue;
        }
        let pos = nodes.get(start).unwrap().1.pos;
        commands.spawn((
            ActiveVehicle {
                pos,
                heading: Vec3::X,
                phase: ShuttlePhase::Loading,
                route: Vec::new(),
                leg: 0,
                s: 0.0,
                cargo: Inventory::new(TRUCK_CARGO_CAPACITY),
            },
            PawnOf(asset),
        ));
    }
}

/// The whole shuttle loop: dock-load at the source, drive out, dock-unload at
/// the destination, drive back. Blocked states (empty source, full
/// destination, severed route) wait in place — capacity exhaustion is a
/// planning signal, never a delete.
fn run_shuttles(
    mut pawns: Query<(&mut ActiveVehicle, &PawnOf)>,
    assignments: Query<&ShuttleAssignment>,
    buildings: Query<&Building>,
    mut yards: Query<&mut Inventory, With<Building>>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    let dt = SECS_PER_PASS as f32;
    for (mut vehicle, pawn_of) in &mut pawns {
        // M1 has no tool that removes assignments or buildings; if either is
        // gone the pawn simply holds position (cargo is physical stock).
        let Ok(order) = assignments.get(pawn_of.0) else {
            continue;
        };
        match vehicle.phase {
            ShuttlePhase::Loading => {
                let Ok(mut yard) = yards.get_mut(order.from) else {
                    continue;
                };
                let space = vehicle.cargo.capacity - vehicle.cargo.total();
                let got = yard.take(order.resource, TRUCK_TRANSFER_RATE.min(space));
                vehicle.cargo.add(order.resource, got);
                if vehicle.cargo.capacity - vehicle.cargo.total() < 1e-3 {
                    depart(
                        &mut vehicle,
                        order.from,
                        order.to,
                        ShuttlePhase::Outbound,
                        &buildings,
                        &nodes,
                        &segments,
                    );
                }
            }
            ShuttlePhase::Unloading => {
                let Ok(mut yard) = yards.get_mut(order.to) else {
                    continue;
                };
                let carried = vehicle.cargo.amount(order.resource);
                let taken = vehicle
                    .cargo
                    .take(order.resource, TRUCK_TRANSFER_RATE.min(carried));
                let fit = yard.add(order.resource, taken);
                // Destination yard full: put the remainder back and wait.
                vehicle.cargo.add(order.resource, taken - fit);
                if vehicle.cargo.total() < 1e-3 {
                    depart(
                        &mut vehicle,
                        order.to,
                        order.from,
                        ShuttlePhase::Returning,
                        &buildings,
                        &nodes,
                        &segments,
                    );
                }
            }
            ShuttlePhase::Outbound | ShuttlePhase::Returning => {
                // A cut road severs the cached route mid-drive: re-route toward
                // the same destination (from the source node — an M1 stub; lane
                // re-entry arrives with the B3 dispatcher). No path yet means
                // hold and retry next tick, so a rebuild resumes the trip.
                let severed = vehicle
                    .route
                    .get(vehicle.leg)
                    .is_some_and(|leg| segments.get(leg.segment).is_err());
                if severed {
                    let (from, to) = match vehicle.phase {
                        ShuttlePhase::Outbound => (order.from, order.to),
                        _ => (order.to, order.from),
                    };
                    let phase = vehicle.phase;
                    depart(&mut vehicle, from, to, phase, &buildings, &nodes, &segments);
                } else {
                    advance_along_route(&mut vehicle, dt, &nodes, &segments);
                }
            }
        }
    }
}

/// Compute a fresh route and switch phase; if the graph has no path right
/// now, stay docked and retry next tick.
fn depart(
    vehicle: &mut ActiveVehicle,
    from: Entity,
    to: Entity,
    phase: ShuttlePhase,
    buildings: &Query<&Building>,
    nodes: &Query<(Entity, &RoadNode)>,
    segments: &Query<&RoadSegment>,
) {
    let (Ok(from_b), Ok(to_b)) = (buildings.get(from), buildings.get(to)) else {
        return;
    };
    let (Some(start), Some(goal)) = (
        nearest_node(from_b.pos, nodes),
        nearest_node(to_b.pos, nodes),
    ) else {
        return;
    };
    let Some(route) = find_route(start, goal, nodes, segments) else {
        return;
    };
    vehicle.pos = nodes.get(start).unwrap().1.pos;
    vehicle.route = route;
    vehicle.leg = 0;
    vehicle.s = 0.0;
    vehicle.phase = phase;
}

fn advance_along_route(
    vehicle: &mut ActiveVehicle,
    dt: f32,
    nodes: &Query<(Entity, &RoadNode)>,
    segments: &Query<&RoadSegment>,
) {
    let mut budget = f32::MAX; // set from lane speed on the first leg below
    loop {
        let Some(leg) = vehicle.route.get(vehicle.leg).copied() else {
            // Route exhausted: arrived.
            vehicle.phase = match vehicle.phase {
                ShuttlePhase::Outbound => ShuttlePhase::Unloading,
                _ => ShuttlePhase::Loading,
            };
            return;
        };
        // A recompiled-away segment severs the route: hold and let the next
        // depart() recompute (M1 keeps this rare; no roads are auto-removed).
        let Ok(segment) = segments.get(leg.segment) else {
            return;
        };
        let Some(lane) = segment.lanes.iter().find(|l| l.dir == leg.dir) else {
            return;
        };
        if budget == f32::MAX {
            budget = TRUCK_SPEED * lane.speed_modifier * dt;
        }
        let remaining = segment.length - vehicle.s;
        if budget < remaining {
            vehicle.s += budget;
            place_on_leg(vehicle, segment, leg.dir, nodes);
            return;
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

fn nearest_node(pos: Vec3, nodes: &Query<(Entity, &RoadNode)>) -> Option<Entity> {
    nodes
        .iter()
        .map(|(e, n)| (e, n.pos.distance_squared(pos)))
        .filter(|(_, d2)| *d2 <= DOCK_RADIUS * DOCK_RADIUS)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

/// Unweighted BFS over node adjacency — fine at M1 scale; the B3 dispatcher
/// brings cost-aware search over the packed mirror (ADR 0005).
fn find_route(
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
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            VehicleSimPlugin,
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

    /// Straight dirt road x∈[0,100], quarry near x=0, factory near x=100,
    /// quarry yard pre-filled, one shuttle assigned. Returns (quarry, factory).
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
        ticks(app, 2); // place + let spawns land
        let world = app.world_mut();
        let mut found = (None, None);
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::Quarry => found.0 = Some(e),
                BuildingKind::Factory => found.1 = Some(e),
                _ => {}
            }
        }
        let (quarry, factory) = (found.0.unwrap(), found.1.unwrap());
        world
            .get_mut::<Inventory>(quarry)
            .unwrap()
            .add(ResourceKind::Gravel, 50.0);
        world
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::CreateShuttle {
                from: quarry,
                to: factory,
                resource: ResourceKind::Gravel,
            });
        (quarry, factory)
    }

    #[test]
    fn dispatcher_spawns_a_linked_pawn_for_the_idle_asset() {
        let mut app = app();
        shuttle_world(&mut app);
        ticks(&mut app, 3); // create asset, dispatch, flush
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
    fn no_pawn_is_dispatched_without_a_road() {
        let mut app = app();
        // buildings + assignment but zero road nodes
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Quarry,
            pos: Vec3::ZERO,
        });
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Factory,
            pos: Vec3::new(100.0, 0.0, 0.0),
        });
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        let mut it = q.iter(world);
        let (a, b) = (it.next().unwrap().0, it.next().unwrap().0);
        world
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::CreateShuttle {
                from: a,
                to: b,
                resource: ResourceKind::Gravel,
            });
        ticks(&mut app, 5);
        let world = app.world_mut();
        assert_eq!(world.query::<&ActiveVehicle>().iter(world).count(), 0);
    }

    #[test]
    fn shuttle_delivers_a_full_truckload_to_the_destination_yard() {
        let mut app = app();
        let (_, factory) = shuttle_world(&mut app);
        // load 10 t at 0.5 t/frame = 20 ticks; drive 100 m at 12 × 0.55 m/s
        // ≈ 909 ticks; unload 20 ticks. 1100 ticks covers one full delivery.
        ticks(&mut app, 1100);
        let delivered = app
            .world_mut()
            .get::<Inventory>(factory)
            .unwrap()
            .amount(ResourceKind::Gravel);
        assert!(
            delivered >= TRUCK_CARGO_CAPACITY - 1e-2,
            "delivered = {delivered}"
        );
    }

    #[test]
    fn cut_road_holds_the_truck_and_rebuild_resumes_the_delivery() {
        let mut app = app();
        let (_, factory) = shuttle_world(&mut app);
        ticks(&mut app, 100); // mid-drive, outbound
        {
            let world = app.world_mut();
            let vehicle = world.query::<&ActiveVehicle>().single(world).unwrap();
            assert_eq!(vehicle.phase, ShuttlePhase::Outbound);
        }
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::RemoveNear {
                pos: Vec3::new(50.0, 0.0, 0.0),
            });
        ticks(&mut app, 200); // severed: holds, nothing delivered
        assert_eq!(
            app.world()
                .get::<Inventory>(factory)
                .unwrap()
                .amount(ResourceKind::Gravel),
            0.0
        );
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::RebuildLast);
        ticks(&mut app, 1200); // reroute + full drive + unload
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
    fn moving_pawn_rides_the_forward_lane_offset() {
        let mut app = app();
        shuttle_world(&mut app);
        ticks(&mut app, 100); // load (20) + well into the outbound drive
        let world = app.world_mut();
        let vehicle = world.query::<&ActiveVehicle>().single(world).unwrap();
        assert_eq!(vehicle.phase, ShuttlePhase::Outbound);
        assert!(vehicle.pos.x > 0.0 && vehicle.pos.x < 100.0);
        // dirt road: width 6 → forward-lane offset magnitude 1.5, laterally in z
        assert!(
            (vehicle.pos.z.abs() - 1.5).abs() < 1e-3,
            "z = {}",
            vehicle.pos.z
        );
        assert_eq!(vehicle.heading, Vec3::X);
    }
}
