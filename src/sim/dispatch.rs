//! The dispatcher (tickets #33/#34, spec/logistics.md stages 1–2): the shared
//! matching pass that turns storage-band deficits into `FreightOrder`s, and
//! the trip state machine that executes them with the finite depot fleet.
//!
//! Matching is bucket-driven (W&R's confirmed shape): below the min band is a
//! demand, above the max band is a supply — no offer objects. The published
//! score rule fixes CS1's 1E-07 cargo-distance bug with a weight that makes
//! hauling distance genuinely matter, and resources are round-robined across
//! matching frames (CS1's proven amortisation trick). Unmatched deficits and
//! unassigned orders persist visibly — the queue *is* the planning signal.

use std::collections::HashMap;

use bevy::prelude::*;

use super::buildings::Building;
use super::clock::{SECS_PER_PASS, TickIndex};
use super::resources::{Inventory, ResourceKind};
use super::roads::{RoadNode, RoadSegment};
use super::stages::{SimStage, SimTick};
use super::storage::StoragePolicies;
use super::vehicles::{
    ActivePawn, ActiveVehicle, PawnOf, ShuttleAssignment, TRUCK_CARGO_CAPACITY,
    TRUCK_TRANSFER_RATE, VehicleAsset, advance_along_route, find_route, nearest_node,
    nearest_node_unbounded,
};

/// One resource is matched every this many ticks (round-robin across the
/// resource set), so a full sweep costs `COUNT × MATCH_INTERVAL` ticks and the
/// per-tick matching cost stays flat no matter how many resources exist.
pub const MATCH_INTERVAL: u32 = 15;

/// The cargo distance weight in the published score rule
/// `score = priority / (1 + d² · w)`. CS1 ships 1E-07 here, which is why its
/// freight teleport-hauls across the map; ours halves a candidate's score at
/// 150 m so the nearer supplier genuinely wins.
pub const DIST_WEIGHT: f32 = 1.0 / (150.0 * 150.0);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OrderId(pub u64);

/// A matched haul: `qty` tonnes of `resource` from one yard to another. Lives
/// in the queue until the delivering truck finishes unloading; `assigned`
/// makes truck scarcity legible (unassigned orders wait visibly).
#[derive(Clone, Copy, Debug)]
pub struct FreightOrder {
    pub id: OrderId,
    pub resource: ResourceKind,
    pub qty: f32,
    pub from: Entity,
    pub to: Entity,
    /// Deficit fraction of the destination's min line at match time.
    pub priority: f32,
    pub assigned: Option<Entity>,
    pub issued_tick: u32,
}

#[derive(Resource, Default)]
pub struct DispatchQueue {
    pub orders: Vec<FreightOrder>,
    pub next_order: u64,
    /// Round-robin cursor into `ResourceKind::ALL`.
    pub cursor: usize,
}

/// A deficit the matcher could not cover (no supplier, no route, no dock):
/// the starvation readout. Refreshed for a resource on its matching frame.
#[derive(Clone, Copy, Debug)]
pub struct StarvingDeficit {
    pub building: Entity,
    pub resource: ResourceKind,
    pub deficit: f32,
    pub since_tick: u32,
}

#[derive(Resource, Default)]
pub struct DeficitBoard(pub Vec<StarvingDeficit>);

/// Trip state, carried by the *asset* (persistent identity) while its pawn
/// drives. ToPickup → Loading → ToDropoff → Unloading → ReturnToDepot, then
/// the pawn despawns and the truck is parked again.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FreightPhase {
    ToPickup,
    Loading,
    ToDropoff,
    Unloading,
    ReturnToDepot,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct FreightJob {
    pub order: OrderId,
    pub phase: FreightPhase,
}

pub struct DispatchSimPlugin;

impl Plugin for DispatchSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DispatchQueue>()
            .init_resource::<DeficitBoard>()
            .add_systems(
                SimTick,
                (match_freight, assign_freight)
                    .chain()
                    .in_set(SimStage::AllocationAndDispatch),
            )
            .add_systems(SimTick, run_freight.in_set(SimStage::MovementAndTransfers));
    }
}

/// Connected-component label per road node, one BFS flood per matching frame.
/// Route *existence* checks then cost O(1) per candidate pair — this is what
/// keeps the matcher sub-linear in storages instead of one BFS per pair.
fn node_components(
    nodes: &Query<(Entity, &RoadNode)>,
    segments: &Query<&RoadSegment>,
) -> HashMap<Entity, u32> {
    let mut label: HashMap<Entity, u32> = HashMap::new();
    let mut next = 0u32;
    for (start, _) in nodes.iter() {
        if label.contains_key(&start) {
            continue;
        }
        let mut stack = vec![start];
        label.insert(start, next);
        while let Some(node) = stack.pop() {
            let Ok((_, n)) = nodes.get(node) else {
                continue;
            };
            for &seg in &n.segments {
                let Ok(segment) = segments.get(seg) else {
                    continue;
                };
                for other in [segment.a, segment.b] {
                    if !label.contains_key(&other) {
                        label.insert(other, next);
                        stack.push(other);
                    }
                }
            }
        }
        next += 1;
    }
    label
}

/// The matching pass (#33): scan buckets for this frame's resource, post one
/// truckload order per starving demander from its best-scoring supplier.
fn match_freight(
    tick: Res<TickIndex>,
    mut queue: ResMut<DispatchQueue>,
    mut board: ResMut<DeficitBoard>,
    storages: Query<(Entity, &Building, &Inventory, &StoragePolicies)>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    if tick.0 % MATCH_INTERVAL != 0 {
        return;
    }
    let resource = ResourceKind::ALL[queue.cursor % ResourceKind::COUNT];
    queue.cursor = (queue.cursor + 1) % ResourceKind::COUNT;

    // Tonnes already promised by live orders: a deficit that is being fed is
    // not re-matched; a surplus that is earmarked is never double-loaded.
    let mut inbound: HashMap<Entity, f32> = HashMap::new();
    let mut outbound: HashMap<Entity, f32> = HashMap::new();
    for order in queue.orders.iter().filter(|o| o.resource == resource) {
        *inbound.entry(order.to).or_default() += order.qty;
        *outbound.entry(order.from).or_default() += order.qty;
    }

    let components = node_components(&nodes, &segments);
    let dock = |pos: Vec3| nearest_node(pos, &nodes).and_then(|n| components.get(&n).copied());

    struct Demander {
        entity: Entity,
        pos: Vec3,
        component: Option<u32>,
        deficit: f32,
        priority: f32,
    }
    struct Supplier {
        entity: Entity,
        pos: Vec3,
        component: Option<u32>,
        surplus: f32,
        frac: f32,
    }
    let mut demanders: Vec<Demander> = Vec::new();
    let mut suppliers: Vec<Supplier> = Vec::new();
    for (entity, building, inventory, policies) in &storages {
        let deficit =
            policies.deficit(resource, inventory) - inbound.get(&entity).copied().unwrap_or(0.0);
        if deficit > 1e-3 {
            let min_line = policies
                .band(resource)
                .map_or(0.0, |b| b.min_pct * inventory.capacity);
            demanders.push(Demander {
                entity,
                pos: building.pos,
                component: dock(building.pos),
                deficit,
                priority: deficit / min_line.max(1e-3),
            });
        }
        let surplus =
            policies.surplus(resource, inventory) - outbound.get(&entity).copied().unwrap_or(0.0);
        if surplus > 1e-3 {
            suppliers.push(Supplier {
                entity,
                pos: building.pos,
                component: dock(building.pos),
                surplus,
                frac: surplus / inventory.capacity.max(1e-3),
            });
        }
    }
    // Starving-first: the emptiest bucket relative to its min line matches first.
    demanders.sort_by(|a, b| b.priority.total_cmp(&a.priority));

    let previous_board: HashMap<Entity, u32> = board
        .0
        .iter()
        .filter(|d| d.resource == resource)
        .map(|d| (d.building, d.since_tick))
        .collect();
    board.0.retain(|d| d.resource != resource);

    for demander in &mut demanders {
        if demander.component.is_some() {
            // Published score rule: score = priority / (1 + d²·w), with the
            // supplier's surplus fraction as its priority term.
            let best = suppliers
                .iter_mut()
                .filter(|s| {
                    s.entity != demander.entity
                        && s.surplus > 1e-3
                        && s.component == demander.component
                })
                .map(|s| {
                    let d2 = s.pos.distance_squared(demander.pos);
                    let score = s.frac / (1.0 + d2 * DIST_WEIGHT);
                    (s, score)
                })
                .max_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((supplier, _)) = best {
                let qty = TRUCK_CARGO_CAPACITY
                    .min(demander.deficit)
                    .min(supplier.surplus);
                supplier.surplus -= qty;
                demander.deficit -= qty;
                queue.next_order += 1;
                let id = OrderId(queue.next_order);
                queue.orders.push(FreightOrder {
                    id,
                    resource,
                    qty,
                    from: supplier.entity,
                    to: demander.entity,
                    priority: demander.priority,
                    assigned: None,
                    issued_tick: tick.0,
                });
            }
        }
        // Whatever the pass could not cover stays on the board, keeping its
        // original starvation timestamp.
        if demander.deficit > 1e-3 {
            board.0.push(StarvingDeficit {
                building: demander.entity,
                resource,
                deficit: demander.deficit,
                since_tick: previous_board
                    .get(&demander.entity)
                    .copied()
                    .unwrap_or(tick.0),
            });
        }
    }
}

/// Truck assignment (#34): each unassigned order gets an idle class-compatible
/// truck from the nearest depot that has one. No idle truck ⇒ the order waits
/// in the queue — bounded throughput is the point.
fn assign_freight(
    mut commands: Commands,
    mut queue: ResMut<DispatchQueue>,
    fleet: Query<
        (Entity, &VehicleAsset, Has<ActivePawn>, Has<FreightJob>),
        Without<ShuttleAssignment>,
    >,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &RoadNode)>,
) {
    // Self-heal: an order whose truck lost its job (despawned asset, load
    // boundary) goes back to waiting.
    for order in queue.orders.iter_mut() {
        if let Some(asset) = order.assigned
            && !fleet.get(asset).is_ok_and(|(.., has_job)| has_job)
        {
            order.assigned = None;
        }
    }
    let mut idle: Vec<(Entity, &VehicleAsset)> = fleet
        .iter()
        .filter(|&(_, _, on_road, has_job)| !on_road && !has_job)
        .map(|(e, a, ..)| (e, a))
        .collect();
    if idle.is_empty() {
        return;
    }
    let mut waiting: Vec<usize> = (0..queue.orders.len())
        .filter(|&i| queue.orders[i].assigned.is_none())
        .collect();
    waiting.sort_by(|&a, &b| queue.orders[b].priority.total_cmp(&queue.orders[a].priority));
    for i in waiting {
        let order = queue.orders[i];
        let class = order.resource.transport_class();
        let Ok(pickup) = buildings.get(order.from) else {
            continue;
        };
        // Nearest depot with an idle compatible truck.
        let best = idle
            .iter()
            .enumerate()
            .filter(|(_, (_, asset))| asset.cargo_class == class)
            .filter_map(|(slot, (entity, asset))| {
                let depot = buildings.get(asset.home_depot).ok()?;
                Some((slot, *entity, depot.pos.distance_squared(pickup.pos)))
            })
            .min_by(|a, b| a.2.total_cmp(&b.2));
        let Some((slot, asset, _)) = best else {
            continue;
        };
        idle.swap_remove(slot);
        queue.orders[i].assigned = Some(asset);
        // The pawn rolls out from the depot's road node; a depot off the road
        // network starts the trip at the pickup dock (fiat stub until depots
        // require their own hookup).
        let depot_pos = fleet.get(asset).map(|(_, a, ..)| a.home_depot).ok();
        let start = depot_pos
            .and_then(|d| buildings.get(d).ok())
            .and_then(|b| nearest_node(b.pos, &nodes))
            .or_else(|| nearest_node(pickup.pos, &nodes));
        let Some(start) = start else {
            queue.orders[i].assigned = None;
            continue;
        };
        let pos = nodes.get(start).unwrap().1.pos;
        commands.entity(asset).insert(FreightJob {
            order: order.id,
            phase: FreightPhase::ToPickup,
        });
        commands.spawn((
            ActiveVehicle {
                pos,
                heading: Vec3::X,
                phase: super::vehicles::ShuttlePhase::Loading,
                route: Vec::new(),
                leg: 0,
                s: 0.0,
                cargo: Inventory::new(TRUCK_CARGO_CAPACITY),
            },
            PawnOf(asset),
        ));
        if idle.is_empty() {
            return;
        }
    }
}

enum Progress {
    Arrived,
    Moving,
    NoPath,
}

/// Ensure a route toward `goal` exists (recomputing after a severed segment)
/// and advance along it. Recovery re-enters at the nearest node — cargo stays
/// aboard through a re-route.
fn drive_toward(
    vehicle: &mut ActiveVehicle,
    goal: Entity,
    dt: f32,
    nodes: &Query<(Entity, &RoadNode)>,
    segments: &Query<&RoadSegment>,
) -> Progress {
    let severed = vehicle
        .route
        .get(vehicle.leg)
        .is_some_and(|leg| segments.get(leg.segment).is_err());
    let exhausted = vehicle.leg >= vehicle.route.len();
    if severed || (exhausted && !vehicle.route.is_empty()) || vehicle.route.is_empty() {
        // Arrived already?
        if let Ok((_, goal_node)) = nodes.get(goal)
            && vehicle.pos.distance_squared(goal_node.pos) < 1.0
        {
            vehicle.route = Vec::new();
            vehicle.leg = 0;
            vehicle.s = 0.0;
            return Progress::Arrived;
        }
        let Some(start) = nearest_node_unbounded(vehicle.pos, nodes) else {
            return Progress::NoPath;
        };
        let Some(route) = find_route(start, goal, nodes, segments) else {
            return Progress::NoPath;
        };
        vehicle.pos = nodes.get(start).unwrap().1.pos;
        vehicle.route = route;
        vehicle.leg = 0;
        vehicle.s = 0.0;
        if vehicle.route.is_empty() {
            return Progress::Arrived;
        }
    }
    if advance_along_route(vehicle, dt, nodes, segments) {
        Progress::Arrived
    } else {
        Progress::Moving
    }
}

/// The freight trip state machine (#34). Dock transfer rate is the loading
/// bottleneck; blocked states (empty source, full destination, severed route)
/// wait in place — capacity exhaustion is a planning signal, never a delete.
#[allow(clippy::too_many_arguments)]
fn run_freight(
    mut commands: Commands,
    mut queue: ResMut<DispatchQueue>,
    mut pawns: Query<(Entity, &mut ActiveVehicle, &PawnOf)>,
    mut jobs: Query<&mut FreightJob>,
    assets: Query<&VehicleAsset>,
    buildings: Query<&Building>,
    mut yards: Query<&mut Inventory, With<Building>>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    let dt = SECS_PER_PASS as f32;
    for (pawn, mut vehicle, pawn_of) in &mut pawns {
        let Ok(mut job) = jobs.get_mut(pawn_of.0) else {
            continue;
        };
        let order = queue.orders.iter().position(|o| o.id == job.order);
        // Order gone or reassigned (severed-trip requeue): head home empty.
        if job.phase != FreightPhase::ReturnToDepot
            && order.is_none_or(|i| queue.orders[i].assigned != Some(pawn_of.0))
            && vehicle.cargo.total() < 1e-3
        {
            job.phase = FreightPhase::ReturnToDepot;
        }
        match job.phase {
            FreightPhase::ToPickup => {
                let Some(i) = order else { continue };
                let Some(goal) = buildings
                    .get(queue.orders[i].from)
                    .ok()
                    .and_then(|b| nearest_node(b.pos, &nodes))
                else {
                    requeue(&mut queue, i, &mut job);
                    continue;
                };
                match drive_toward(&mut vehicle, goal, dt, &nodes, &segments) {
                    Progress::Arrived => job.phase = FreightPhase::Loading,
                    Progress::Moving => {}
                    // No route to the pickup: give the order back so another
                    // depot's truck (or a rebuilt road) can serve it.
                    Progress::NoPath => requeue(&mut queue, i, &mut job),
                }
            }
            FreightPhase::Loading => {
                let Some(i) = order else { continue };
                let o = queue.orders[i];
                let Ok(mut yard) = yards.get_mut(o.from) else {
                    requeue(&mut queue, i, &mut job);
                    continue;
                };
                let want = (o.qty - vehicle.cargo.total()).max(0.0);
                let got = yard.take(o.resource, TRUCK_TRANSFER_RATE.min(want));
                vehicle.cargo.add(o.resource, got);
                let full = vehicle.cargo.total() >= o.qty - 1e-3;
                // A dried-up yard with something aboard departs with what it
                // has (partial fulfilment is normal, never an error).
                if full || (got < 1e-6 && vehicle.cargo.total() > 1e-3) {
                    job.phase = FreightPhase::ToDropoff;
                }
            }
            FreightPhase::ToDropoff => {
                let Some(i) = order else {
                    job.phase = FreightPhase::ReturnToDepot;
                    continue;
                };
                let Some(goal) = buildings
                    .get(queue.orders[i].to)
                    .ok()
                    .and_then(|b| nearest_node(b.pos, &nodes))
                else {
                    continue;
                };
                match drive_toward(&mut vehicle, goal, dt, &nodes, &segments) {
                    Progress::Arrived => job.phase = FreightPhase::Unloading,
                    // Severed with cargo aboard: hold and retry — the cargo
                    // stays physical, the delivery resumes when a path exists.
                    Progress::Moving | Progress::NoPath => {}
                }
            }
            FreightPhase::Unloading => {
                let Some(i) = order else {
                    job.phase = FreightPhase::ReturnToDepot;
                    continue;
                };
                let o = queue.orders[i];
                let Ok(mut yard) = yards.get_mut(o.to) else {
                    continue;
                };
                let carried = vehicle.cargo.amount(o.resource);
                let taken = vehicle.cargo.take(o.resource, TRUCK_TRANSFER_RATE.min(carried));
                let fit = yard.add(o.resource, taken);
                // Destination full: put the remainder back and wait.
                vehicle.cargo.add(o.resource, taken - fit);
                if vehicle.cargo.total() < 1e-3 {
                    queue.orders.remove(i);
                    job.phase = FreightPhase::ReturnToDepot;
                }
            }
            FreightPhase::ReturnToDepot => {
                let home = assets
                    .get(pawn_of.0)
                    .ok()
                    .and_then(|a| buildings.get(a.home_depot).ok())
                    .and_then(|b| nearest_node(b.pos, &nodes));
                let done = match home {
                    Some(goal) => matches!(
                        drive_toward(&mut vehicle, goal, dt, &nodes, &segments),
                        Progress::Arrived
                    ),
                    // Depot off the network: the truck parks where it stands
                    // (asset teleports to its slot — fiat stub, as spawn-side).
                    None => true,
                };
                if done {
                    commands.entity(pawn).despawn();
                    commands.entity(pawn_of.0).remove::<FreightJob>();
                }
            }
        }
    }
}

/// Give an order back to the queue and send the truck home empty.
fn requeue(queue: &mut DispatchQueue, order: usize, job: &mut FreightJob) {
    queue.orders[order].assigned = None;
    job.phase = FreightPhase::ReturnToDepot;
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
    };
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
    use super::super::storage::{StorageSimPlugin, StoragePolicies};
    use super::super::resources::TransportClass;
    use super::super::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
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

    fn place_road(app: &mut App, from: Vec3, to: Vec3) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from,
                to,
                class: RoadClass::Dirt,
            });
    }

    fn place(app: &mut App, kind: BuildingKind, pos: Vec3) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place { kind, pos });
    }

    fn find(app: &mut App, kind: BuildingKind) -> Entity {
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        q.iter(world).find(|(_, b)| b.kind == kind).unwrap().0
    }

    fn fill(app: &mut App, building: Entity, resource: ResourceKind, amount: f32) {
        app.world_mut()
            .get_mut::<Inventory>(building)
            .unwrap()
            .add(resource, amount);
    }

    /// Road x∈[0,60]; depot docked at the west node, mine (coal supplier)
    /// docked at the east node, power plant (coal demander) at the west node.
    fn coal_world(app: &mut App, trucks: u32, class: TransportClass) -> (Entity, Entity) {
        place_road(app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        place(app, BuildingKind::Mine, Vec3::new(65.0, 0.0, 0.0));
        place(app, BuildingKind::PowerPlant, Vec3::new(-5.0, 0.0, 0.0));
        place(app, BuildingKind::Depot, Vec3::new(0.0, 0.0, 15.0));
        ticks(app, 2);
        let (mine, depot) = (
            find(app, BuildingKind::Mine),
            find(app, BuildingKind::Depot),
        );
        let plant = find(app, BuildingKind::PowerPlant);
        fill(app, mine, ResourceKind::Coal, 60.0);
        for _ in 0..trucks {
            app.world_mut()
                .resource_mut::<VehicleEditQueue>()
                .0
                .push(VehicleEdit::BuyTruck { depot, class });
        }
        ticks(app, 2);
        (mine, plant)
    }

    #[test]
    fn matcher_posts_a_truckload_order_for_the_starving_plant() {
        let mut app = app();
        let (mine, plant) = coal_world(&mut app, 0, TransportClass::Bulk);
        ticks(&mut app, MATCH_INTERVAL + 1);
        let queue = app.world().resource::<DispatchQueue>();
        let order = queue
            .orders
            .iter()
            .find(|o| o.resource == ResourceKind::Coal)
            .expect("coal deficit matched");
        assert_eq!(order.from, mine);
        assert_eq!(order.to, plant);
        assert!((order.qty - TRUCK_CARGO_CAPACITY).abs() < 1e-3);
        assert!(order.priority > 0.9, "empty bucket is maximum priority");
    }

    #[test]
    fn no_order_without_a_route_and_the_deficit_stays_on_the_board() {
        let mut app = app();
        // Mine on its own island road, plant on another — no path between.
        place_road(&mut app, Vec3::ZERO, Vec3::new(40.0, 0.0, 0.0));
        place_road(
            &mut app,
            Vec3::new(200.0, 0.0, 0.0),
            Vec3::new(240.0, 0.0, 0.0),
        );
        place(&mut app, BuildingKind::Mine, Vec3::new(-5.0, 0.0, 0.0));
        place(
            &mut app,
            BuildingKind::PowerPlant,
            Vec3::new(245.0, 0.0, 0.0),
        );
        ticks(&mut app, 2);
        let mine = find(&mut app, BuildingKind::Mine);
        fill(&mut app, mine, ResourceKind::Coal, 60.0);
        ticks(&mut app, 3 * MATCH_INTERVAL + 1);
        let world = app.world();
        assert!(world.resource::<DispatchQueue>().orders.is_empty());
        let board = world.resource::<DeficitBoard>();
        assert!(
            board
                .0
                .iter()
                .any(|d| d.resource == ResourceKind::Coal && d.deficit > 20.0),
            "unmet deficit persists visibly"
        );
    }

    #[test]
    fn distance_weight_prefers_the_near_supplier() {
        let mut app = app();
        place_road(&mut app, Vec3::ZERO, Vec3::new(300.0, 0.0, 0.0));
        place(&mut app, BuildingKind::PowerPlant, Vec3::new(-5.0, 0.0, 0.0));
        // Two stocked mines, one 10 m away, one 290 m away (same road).
        place(&mut app, BuildingKind::Mine, Vec3::new(10.0, 0.0, 10.0));
        place(&mut app, BuildingKind::Mine, Vec3::new(290.0, 0.0, 10.0));
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building, &mut Inventory)>();
        let mut near = None;
        for (e, b, mut inventory) in q.iter_mut(world) {
            if b.kind == BuildingKind::Mine {
                inventory.add(ResourceKind::Coal, 60.0);
                if b.pos.x < 100.0 {
                    near = Some(e);
                }
            }
        }
        ticks(&mut app, MATCH_INTERVAL + 1);
        let queue = app.world().resource::<DispatchQueue>();
        assert!(!queue.orders.is_empty());
        assert_eq!(
            queue.orders[0].from,
            near.unwrap(),
            "no cross-map hauling: the near supplier wins the score"
        );
    }

    #[test]
    fn round_robin_reaches_every_resource() {
        let mut app = app();
        place_road(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        place(&mut app, BuildingKind::Mine, Vec3::new(-5.0, 0.0, 0.0));
        place(&mut app, BuildingKind::PowerPlant, Vec3::new(65.0, 0.0, 5.0));
        place(&mut app, BuildingKind::Factory, Vec3::new(5.0, 0.0, 10.0));
        place(&mut app, BuildingKind::Dwelling, Vec3::new(65.0, 0.0, -5.0));
        ticks(&mut app, 2);
        let (mine, factory) = (
            find(&mut app, BuildingKind::Mine),
            find(&mut app, BuildingKind::Factory),
        );
        fill(&mut app, mine, ResourceKind::Coal, 60.0);
        fill(&mut app, factory, ResourceKind::Goods, 40.0);
        // A full sweep visits every resource once.
        ticks(&mut app, ResourceKind::COUNT as u32 * MATCH_INTERVAL + 1);
        let queue = app.world().resource::<DispatchQueue>();
        for resource in [ResourceKind::Coal, ResourceKind::Goods] {
            assert!(
                queue.orders.iter().any(|o| o.resource == resource),
                "{resource:?} deficit was matched in the sweep"
            );
        }
    }

    #[test]
    fn reservations_never_promise_the_same_tonnes_twice() {
        let mut app = app();
        place_road(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        // A warehouse supplier (produces nothing, unlike a mine): surplus is
        // exactly 84 − 0.6 × 120 = 12 t, less than two truckloads.
        place(&mut app, BuildingKind::Warehouse, Vec3::new(-5.0, 0.0, 0.0));
        place(&mut app, BuildingKind::PowerPlant, Vec3::new(65.0, 0.0, 5.0));
        place(&mut app, BuildingKind::PowerPlant, Vec3::new(65.0, 0.0, -5.0));
        ticks(&mut app, 2);
        let warehouse = find(&mut app, BuildingKind::Warehouse);
        fill(&mut app, warehouse, ResourceKind::Coal, 84.0);
        ticks(&mut app, 3 * MATCH_INTERVAL + 1);
        let queue = app.world().resource::<DispatchQueue>();
        let promised: f32 = queue
            .orders
            .iter()
            .filter(|o| o.from == warehouse)
            .map(|o| o.qty)
            .sum();
        assert!(promised <= 12.0 + 1e-3, "promised = {promised}");
        assert!(queue.orders.len() >= 2, "the remainder was still matched");
    }

    #[test]
    fn transport_class_gates_assignment() {
        let mut app = app();
        // The only truck is a covered bed; coal needs bulk.
        coal_world(&mut app, 1, TransportClass::Covered);
        ticks(&mut app, 2 * MATCH_INTERVAL + 2);
        let world = app.world_mut();
        let queue = world.resource::<DispatchQueue>();
        assert!(!queue.orders.is_empty());
        assert!(
            queue.orders.iter().all(|o| o.assigned.is_none()),
            "a covered truck must never take a bulk order"
        );
        assert_eq!(world.query::<&ActiveVehicle>().iter(world).count(), 0);
    }

    #[test]
    fn full_trip_delivers_the_order_and_parks_the_truck() {
        let mut app = app();
        let (_, plant) = coal_world(&mut app, 1, TransportClass::Bulk);
        // ~545 ticks per 60 m leg on dirt + docking; give the round trip room.
        // Sample as we go — the plant burns delivered coal, so the yard peak
        // (not the final level) is the delivery evidence.
        let mut peak: f32 = 0.0;
        for _ in 0..36 {
            ticks(&mut app, 50);
            peak = peak.max(
                app.world()
                    .get::<Inventory>(plant)
                    .unwrap()
                    .amount(ResourceKind::Coal),
            );
        }
        assert!(peak > 5.0, "peak plant coal = {peak}");
        // The plant's deficit persists (24 t min line), so the truck keeps
        // cycling — but there is never more than the one pawn for one asset.
        let world = app.world_mut();
        assert!(world.query::<&ActiveVehicle>().iter(world).count() <= 1);
    }

    #[test]
    fn small_order_completes_and_the_fleet_goes_idle() {
        let mut app = app();
        place_road(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        place(&mut app, BuildingKind::Mine, Vec3::new(65.0, 0.0, 0.0));
        place(&mut app, BuildingKind::Warehouse, Vec3::new(-5.0, 0.0, 0.0));
        place(&mut app, BuildingKind::Depot, Vec3::new(0.0, 0.0, 15.0));
        ticks(&mut app, 2);
        let (mine, warehouse, depot) = (
            find(&mut app, BuildingKind::Mine),
            find(&mut app, BuildingKind::Warehouse),
            find(&mut app, BuildingKind::Depot),
        );
        fill(&mut app, mine, ResourceKind::Coal, 60.0);
        // Narrow warehouse policy: coal only, min 5 t of 120 — one small order.
        let mut policies = StoragePolicies::default();
        policies.set(
            ResourceKind::Coal,
            Some(super::super::storage::StorageBand::new(5.0 / 120.0, 0.5)),
        );
        app.world_mut().entity_mut(warehouse).insert(policies);
        app.world_mut()
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
        ticks(&mut app, 2500);
        let world = app.world_mut();
        let coal = world
            .get::<Inventory>(warehouse)
            .unwrap()
            .amount(ResourceKind::Coal);
        assert!(coal >= 5.0 - 1e-2, "warehouse refilled to its min line");
        assert!(world.resource::<DispatchQueue>().orders.is_empty());
        assert_eq!(
            world.query::<&ActiveVehicle>().iter(world).count(),
            0,
            "truck returned to its depot slot"
        );
        assert_eq!(world.query::<&FreightJob>().iter(world).count(), 0);
    }

    #[test]
    fn orders_queue_when_the_fleet_is_busy() {
        let mut app = app();
        place_road(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        place(&mut app, BuildingKind::Mine, Vec3::new(65.0, 0.0, 0.0));
        place(&mut app, BuildingKind::PowerPlant, Vec3::new(-5.0, 0.0, 5.0));
        place(&mut app, BuildingKind::PowerPlant, Vec3::new(-5.0, 0.0, -5.0));
        place(&mut app, BuildingKind::Depot, Vec3::new(0.0, 0.0, 15.0));
        ticks(&mut app, 2);
        let (mine, depot) = (
            find(&mut app, BuildingKind::Mine),
            find(&mut app, BuildingKind::Depot),
        );
        fill(&mut app, mine, ResourceKind::Coal, 60.0);
        app.world_mut()
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
        ticks(&mut app, 2 * MATCH_INTERVAL + 3);
        let queue = app.world().resource::<DispatchQueue>();
        assert!(queue.orders.len() >= 2, "both plants matched");
        assert_eq!(
            queue.orders.iter().filter(|o| o.assigned.is_some()).count(),
            1,
            "one truck ⇒ exactly one order in flight; the rest wait visibly"
        );
    }

    #[test]
    fn severed_route_requeues_the_order() {
        let mut app = app();
        coal_world(&mut app, 1, TransportClass::Bulk);
        // Let the truck get well onto the road toward the pickup.
        ticks(&mut app, MATCH_INTERVAL + 100);
        {
            let world = app.world_mut();
            assert_eq!(world.query::<&ActiveVehicle>().iter(world).count(), 1);
            let queue = world.resource::<DispatchQueue>();
            assert!(queue.orders.iter().any(|o| o.assigned.is_some()));
        }
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::RemoveNear {
                pos: Vec3::new(30.0, 0.0, 0.0),
            });
        ticks(&mut app, 30);
        let queue = app.world().resource::<DispatchQueue>();
        assert!(
            queue.orders.iter().all(|o| o.assigned.is_none()),
            "no path to the pickup ⇒ the order goes back to the queue"
        );
    }
}
