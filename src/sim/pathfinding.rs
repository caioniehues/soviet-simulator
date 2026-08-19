//! B4.1 pathfinding engine (ticket #39, spec/pathfinding.md): cost-aware A*
//! over an immutable packed snapshot of the lane graph, solved on the
//! `AsyncComputeTaskPool` — never on the sim thread. Requests are per-trip
//! (CS1's `PathManager.CreatePath` lifecycle): a consumer submits start/goal,
//! receives a ticket, and holds in place until the ticket resolves. Cost is
//! time-like — `length / (class speed modifier)` — times the per-edge
//! congestion multiplier (B4.2 feeds it; 1.0 until then), with CS1's
//! scatter-randomness so drivers spread over near-equal alternates.
//!
//! Invalidation stays cheap: routes reference segment *entities*, movement
//! reads live segment data, so a despawned segment severs the route at the
//! consumer (which re-requests) — a stale snapshot can only make a fresh
//! route briefly suboptimal, never wrong.

use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on};

use super::roads::{LaneDir, RoadNode, RoadSegment};
use super::stages::{SimStage, SimTick};
use super::traffic::SegmentTraffic;
use super::vehicles::RouteLeg;

/// Congestion scatter band (CS1 `PathFind.cs:969`): an edge's cost is scaled
/// by a random factor in `[JITTER_MIN, congestion]` so saturated corridors
/// both cost more and scatter traffic onto alternates.
pub const JITTER_MIN: f32 = 0.9;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CostProfile {
    /// Time-like: length / lane speed modifier × congestion × jitter.
    Vehicle,
    /// Walkers ignore road class and congestion: plain distance, no jitter.
    Pedestrian,
}

#[derive(Clone, Copy, Debug)]
struct SnapEdge {
    to: u32,
    segment: Entity,
    dir: LaneDir,
    length: f32,
    speed_modifier: f32,
    /// ≥ 1.0; the density-derived cost multiplier (B4.2).
    congestion: f32,
}

/// Immutable packed lane-graph mirror (CSR adjacency) shared with solver
/// tasks by `Arc`. Rebuilt only when the graph or the congestion field
/// actually changed.
pub struct GraphSnapshot {
    index: HashMap<Entity, u32>,
    positions: Vec<Vec3>,
    offsets: Vec<u32>,
    edges: Vec<SnapEdge>,
}

impl GraphSnapshot {
    fn build(
        nodes: &Query<(Entity, &RoadNode)>,
        segments: &Query<(Entity, &RoadSegment, Option<&SegmentTraffic>)>,
    ) -> Self {
        let mut index = HashMap::new();
        let mut positions = Vec::new();
        for (entity, node) in nodes.iter() {
            index.insert(entity, positions.len() as u32);
            positions.push(node.pos);
        }
        let mut adjacency: Vec<Vec<SnapEdge>> = vec![Vec::new(); positions.len()];
        for (seg_entity, segment, traffic) in segments.iter() {
            let (Some(&a), Some(&b)) = (index.get(&segment.a), index.get(&segment.b)) else {
                continue;
            };
            let speed = segment
                .lanes
                .first()
                .map_or(segment.class.speed_modifier(), |l| l.speed_modifier);
            let c = traffic.map_or(1.0, |t| t.cost_multiplier()).max(1.0);
            adjacency[a as usize].push(SnapEdge {
                to: b,
                segment: seg_entity,
                dir: LaneDir::Forward,
                length: segment.length,
                speed_modifier: speed,
                congestion: c,
            });
            adjacency[b as usize].push(SnapEdge {
                to: a,
                segment: seg_entity,
                dir: LaneDir::Backward,
                length: segment.length,
                speed_modifier: speed,
                congestion: c,
            });
        }
        let mut offsets = Vec::with_capacity(positions.len() + 1);
        let mut edges = Vec::new();
        offsets.push(0);
        for list in adjacency {
            edges.extend(list);
            offsets.push(edges.len() as u32);
        }
        Self {
            index,
            positions,
            offsets,
            edges,
        }
    }

    /// A* over the snapshot. `seed` drives the congestion scatter (vary it
    /// per request); pedestrians get a deterministic pure-distance search.
    pub fn astar(
        &self,
        start: Entity,
        goal: Entity,
        profile: CostProfile,
        seed: u64,
    ) -> Option<Vec<RouteLeg>> {
        let (&s, &g) = (self.index.get(&start)?, self.index.get(&goal)?);
        if s == g {
            return Some(Vec::new());
        }
        let goal_pos = self.positions[g as usize];
        let n = self.positions.len();
        let mut best = vec![f32::INFINITY; n];
        let mut prev: Vec<Option<(u32, u32)>> = vec![None; n]; // (parent node, edge idx)
        let mut open = BinaryHeap::new();
        best[s as usize] = 0.0;
        open.push(OpenItem {
            cost: self.positions[s as usize].distance(goal_pos),
            node: s,
        });
        let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
        while let Some(OpenItem { node, .. }) = open.pop() {
            if node == g {
                let mut legs = Vec::new();
                let mut at = g;
                while let Some((parent, edge)) = prev[at as usize] {
                    let e = &self.edges[edge as usize];
                    legs.push(RouteLeg {
                        segment: e.segment,
                        dir: e.dir,
                    });
                    at = parent;
                }
                legs.reverse();
                return Some(legs);
            }
            let g_here = best[node as usize];
            let (lo, hi) = (
                self.offsets[node as usize] as usize,
                self.offsets[node as usize + 1] as usize,
            );
            for (i, edge) in self.edges[lo..hi].iter().enumerate() {
                let step = match profile {
                    CostProfile::Pedestrian => edge.length,
                    CostProfile::Vehicle => {
                        // xorshift; scatter factor in [JITTER_MIN, congestion]
                        rng ^= rng << 13;
                        rng ^= rng >> 7;
                        rng ^= rng << 17;
                        let t = (rng >> 40) as f32 / (1u64 << 24) as f32;
                        let factor = JITTER_MIN + t * (edge.congestion - JITTER_MIN).max(0.0);
                        edge.length / edge.speed_modifier * factor
                    }
                };
                let tentative = g_here + step;
                if tentative < best[edge.to as usize] {
                    best[edge.to as usize] = tentative;
                    prev[edge.to as usize] = Some((node, (lo + i) as u32));
                    // Admissible for vehicles: straight-line at best modifier
                    // 1.0 with jitter floor < 1 — scale by JITTER_MIN.
                    let h = self.positions[edge.to as usize].distance(goal_pos)
                        * match profile {
                            CostProfile::Pedestrian => 1.0,
                            CostProfile::Vehicle => JITTER_MIN,
                        };
                    open.push(OpenItem {
                        cost: tentative + h,
                        node: edge.to,
                    });
                }
            }
        }
        None
    }
}

struct OpenItem {
    cost: f32,
    node: u32,
}
impl PartialEq for OpenItem {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for OpenItem {}
impl PartialOrd for OpenItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OpenItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.total_cmp(&self.cost) // min-heap
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PathTicket(pub u64);

pub enum PathPoll {
    /// Still solving — hold in place.
    Pending,
    /// Solved; `None` means no route exists on the snapshot searched.
    Ready(Option<Vec<RouteLeg>>),
}

/// The background path solver front desk. Consumers `request` and then
/// `poll` on later ticks; solving happens on the async compute pool against
/// the current `Arc<GraphSnapshot>`.
#[derive(Resource, Default)]
pub struct PathService {
    snapshot: Option<Arc<GraphSnapshot>>,
    snapshot_key: (u64, u64, usize, u64),
    next_ticket: u64,
    submitted: Vec<(PathTicket, Entity, Entity, CostProfile)>,
    in_flight: Vec<(PathTicket, u32, Task<Option<Vec<RouteLeg>>>)>,
    ready: HashMap<u64, Option<Vec<RouteLeg>>>,
    /// Bumped by the congestion pass (B4.2) to force a snapshot refresh.
    pub congestion_version: u64,
    /// Live solver-task count, for readouts and benches.
    pub in_flight_count: usize,
}

impl PathService {
    pub fn request(&mut self, start: Entity, goal: Entity, profile: CostProfile) -> PathTicket {
        self.next_ticket += 1;
        let ticket = PathTicket(self.next_ticket);
        self.submitted.push((ticket, start, goal, profile));
        ticket
    }

    pub fn poll(&mut self, ticket: PathTicket) -> PathPoll {
        match self.ready.remove(&ticket.0) {
            Some(result) => PathPoll::Ready(result),
            None => PathPoll::Pending,
        }
    }

    /// Synchronous solve on the current snapshot — for commuter spawning and
    /// tests, where the request/poll round-trip has no value.
    pub fn route_now(
        &self,
        start: Entity,
        goal: Entity,
        profile: CostProfile,
        seed: u64,
    ) -> Option<Vec<RouteLeg>> {
        self.snapshot
            .as_ref()
            .and_then(|s| s.astar(start, goal, profile, seed))
    }

    pub fn has_snapshot(&self) -> bool {
        self.snapshot.is_some()
    }
}

/// Public marker for the pathfinding systems inside `SimStage::Routing`, so
/// same-stage consumers (commuter departures) can order after the snapshot
/// refresh without naming private systems.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PathSystems;

pub struct PathfindingSimPlugin;

impl Plugin for PathfindingSimPlugin {
    fn build(&self, app: &mut App) {
        // Headless apps (tests, benches) run without TaskPoolPlugin; the
        // solver needs the pool either way.
        AsyncComputeTaskPool::get_or_init(bevy::tasks::TaskPool::new);
        app.init_resource::<PathService>().add_systems(
            SimTick,
            (refresh_snapshot, pump_paths)
                .chain()
                .in_set(SimStage::Routing)
                .in_set(PathSystems),
        );
    }
}

/// Rebuild the packed mirror when the graph actually changed. The key folds
/// the monotonic id counters, the live node count and the summed segment
/// modification indices — any structural edit or recompile moves it.
pub fn refresh_snapshot(
    mut svc: ResMut<PathService>,
    ids: Res<super::roads::RoadIds>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<(Entity, &RoadSegment, Option<&SegmentTraffic>)>,
) {
    let mod_sum: u64 = segments
        .iter()
        .map(|(_, s, _)| s.modification_index as u64)
        .sum::<u64>()
        .wrapping_add(svc.congestion_version);
    let key = (
        ids.next_node,
        ids.next_segment,
        nodes.iter().count(),
        mod_sum,
    );
    if svc.snapshot.is_some() && svc.snapshot_key == key {
        return;
    }
    svc.snapshot_key = key;
    svc.snapshot = Some(Arc::new(GraphSnapshot::build(&nodes, &segments)));
}

/// A ticket resolves exactly this many ticks after submission — never
/// earlier, and (short of a solver overrunning ~66 ms) never later. Without
/// the fixed latency the install tick depended on thread scheduling: under
/// CPU contention identical runs diverged at the first route request and
/// never re-converged. Solves are microseconds against a 33 ms tick, so the
/// deadline `block_on` below virtually never actually blocks.
const RESOLVE_LATENCY_TICKS: u32 = 2;

/// Spawn solver tasks for new requests and harvest the ones due this tick.
fn pump_paths(mut svc: ResMut<PathService>, frame: Res<super::clock::FrameIndex>) {
    let Some(snapshot) = svc.snapshot.clone() else {
        return;
    };
    let pool = AsyncComputeTaskPool::get();
    let submitted = std::mem::take(&mut svc.submitted);
    for (ticket, start, goal, profile) in submitted {
        let snap = snapshot.clone();
        let task = pool.spawn(async move { snap.astar(start, goal, profile, ticket.0) });
        svc.in_flight.push((ticket, frame.0, task));
    }
    let mut still = Vec::new();
    for (ticket, submitted_at, task) in std::mem::take(&mut svc.in_flight) {
        // Wrap-safe age check, matching `FrameIndex`'s own discipline.
        if frame.0.wrapping_sub(submitted_at) < RESOLVE_LATENCY_TICKS {
            still.push((ticket, submitted_at, task));
            continue;
        }
        let result = block_on(task);
        svc.ready.insert(ticket.0, result);
    }
    svc.in_flight = still;
    svc.in_flight_count = svc.in_flight.len();
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, RoadSimPlugin, PathfindingSimPlugin));
        a
    }

    fn tick(app: &mut App) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
        app.update();
    }

    fn place(app: &mut App, from: Vec3, to: Vec3, class: RoadClass) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place { from, to, class });
    }

    fn node_at(app: &mut App, pos: Vec3) -> Entity {
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &RoadNode)>();
        q.iter(world)
            .find(|(_, n)| n.pos.distance(pos) < 1.0)
            .unwrap()
            .0
    }

    /// Two routes A→B: a short dirt leg and a long paved detour. The
    /// time-like vehicle cost must pick the faster one appropriately.
    fn fork_world(app: &mut App) -> (Entity, Entity) {
        // direct dirt: 200 m at 0.55 → ~364 s-units
        place(app, Vec3::ZERO, Vec3::new(200.0, 0.0, 0.0), RoadClass::Dirt);
        // paved detour via (100, 120): 2 × ~156 m at 1.0 → ~312 s-units
        place(
            app,
            Vec3::ZERO,
            Vec3::new(100.0, 0.0, 120.0),
            RoadClass::Dirt,
        );
        place(
            app,
            Vec3::new(100.0, 0.0, 120.0),
            Vec3::new(200.0, 0.0, 0.0),
            RoadClass::Dirt,
        );
        tick(app);
        (
            node_at(app, Vec3::ZERO),
            node_at(app, Vec3::new(200.0, 0.0, 0.0)),
        )
    }

    #[test]
    fn vehicle_route_prefers_travel_time_over_distance() {
        let mut app = app();
        // Direct 200 m dirt (0.55) vs 240 m paved detour (1.0): the detour
        // is faster (240 < 364) and must win for vehicles.
        place(
            &mut app,
            Vec3::ZERO,
            Vec3::new(200.0, 0.0, 0.0),
            RoadClass::Dirt,
        );
        {
            // stock unlimited gravel for the paved legs
            use super::super::buildings::{Building, BuildingId, BuildingKind};
            use super::super::resources::{Inventory, ResourceKind};
            let mut inv = Inventory::new(1000.0);
            inv.add(ResourceKind::Gravel, 1000.0);
            app.world_mut().spawn((
                Building {
                    id: BuildingId(998),
                    kind: BuildingKind::Quarry,
                    pos: Vec3::new(100.0, 0.0, 60.0),
                },
                inv,
            ));
        }
        place(
            &mut app,
            Vec3::ZERO,
            Vec3::new(100.0, 0.0, 60.0),
            RoadClass::Paved,
        );
        place(
            &mut app,
            Vec3::new(100.0, 0.0, 60.0),
            Vec3::new(200.0, 0.0, 0.0),
            RoadClass::Paved,
        );
        tick(&mut app);
        let (start, goal) = (
            node_at(&mut app, Vec3::ZERO),
            node_at(&mut app, Vec3::new(200.0, 0.0, 0.0)),
        );
        let svc = app.world().resource::<PathService>();
        let route = svc
            .route_now(start, goal, CostProfile::Vehicle, 7)
            .expect("route exists");
        assert_eq!(route.len(), 2, "vehicle takes the faster paved detour");
        // A pedestrian ignores road class: direct 200 m beats 233 m detour.
        let walk = svc
            .route_now(start, goal, CostProfile::Pedestrian, 7)
            .expect("route exists");
        assert_eq!(walk.len(), 1, "walker takes the shorter direct leg");
    }

    #[test]
    fn async_request_resolves_within_a_few_ticks() {
        let mut app = app();
        let (start, goal) = fork_world(&mut app);
        let ticket = app.world_mut().resource_mut::<PathService>().request(
            start,
            goal,
            CostProfile::Vehicle,
        );
        let mut result = None;
        for _ in 0..300 {
            tick(&mut app);
            if let PathPoll::Ready(r) = app.world_mut().resource_mut::<PathService>().poll(ticket) {
                result = Some(r);
                break;
            }
        }
        let route = result.expect("solver finished").expect("route found");
        assert!(!route.is_empty());
    }

    #[test]
    fn disconnected_goal_reports_no_route() {
        let mut app = app();
        place(
            &mut app,
            Vec3::ZERO,
            Vec3::new(100.0, 0.0, 0.0),
            RoadClass::Dirt,
        );
        place(
            &mut app,
            Vec3::new(500.0, 0.0, 0.0),
            Vec3::new(600.0, 0.0, 0.0),
            RoadClass::Dirt,
        );
        tick(&mut app);
        let (a, b) = (
            node_at(&mut app, Vec3::ZERO),
            node_at(&mut app, Vec3::new(600.0, 0.0, 0.0)),
        );
        let ticket =
            app.world_mut()
                .resource_mut::<PathService>()
                .request(a, b, CostProfile::Vehicle);
        for _ in 0..300 {
            tick(&mut app);
            if let PathPoll::Ready(r) = app.world_mut().resource_mut::<PathService>().poll(ticket) {
                assert!(r.is_none(), "islands must not route");
                return;
            }
        }
        panic!("solver never finished");
    }

    #[test]
    fn snapshot_refreshes_after_a_cut() {
        let mut app = app();
        let (start, goal) = fork_world(&mut app);
        // Cut the direct leg; the snapshot key must move and the new route
        // must use the remaining detour.
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::RemoveNear {
                pos: Vec3::new(100.0, 0.0, 0.0),
            });
        tick(&mut app);
        tick(&mut app);
        let svc = app.world().resource::<PathService>();
        let route = svc
            .route_now(start, goal, CostProfile::Pedestrian, 1)
            .expect("detour still routes");
        assert_eq!(route.len(), 2);
    }
}
