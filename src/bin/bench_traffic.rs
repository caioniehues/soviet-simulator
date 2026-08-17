//! B4.6 benchmark gate (ticket #44): headless traffic at scale. A grid road
//! network carries 10 000 simultaneous vehicle pawns through the full B4
//! pipeline — async A* routing (requests resolve on the compute pool, never
//! the sim thread), lane occupancy + car-following, and the congestion
//! density pass. The whole tick must stay within a 60 fps equivalent budget.
//!
//! Run: cargo run --release --bin bench_traffic

use std::time::{Duration, Instant};

use bevy::prelude::*;

use soviet_simulator::sim::clock::SECS_PER_PASS;
use soviet_simulator::sim::pathfinding::{
    CostProfile, PathPoll, PathService, PathfindingSimPlugin,
};
use soviet_simulator::sim::roads::{
    RoadClass, RoadEdit, RoadEditQueue, RoadNode, RoadSegment, RoadSimPlugin,
};
use soviet_simulator::sim::stages::{SimStage, SimTick};
use soviet_simulator::sim::traffic::{LaneOccupancy, LanePrep, TrafficSimPlugin};
use soviet_simulator::sim::vehicles::{
    ActiveVehicle, advance_along_route, nearest_node_unbounded,
};
use soviet_simulator::sim::{SimPlugin, TickIndex};

const VEHICLES: usize = 10_000;
/// Grid nodes per side; 24² nodes → 1 104 segments at 100 m spacing.
const GRID: i32 = 24;
const SPACING: f32 = 100.0;
const WARMUP_TICKS: u32 = 300;
const MEASURE_TICKS: u32 = 1_000;
/// 60 fps equivalent: the whole sim tick fits a frame with headroom to render.
const GATE_MS: f64 = 16.0;

fn tick(app: &mut App) {
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f64(SECS_PER_PASS + 1e-9));
    app.update();
}

/// All grid node entities, captured once after the road compile.
#[derive(Resource)]
struct Destinations(Vec<Entity>);

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Bench driver mirroring the dispatcher's drive shape: an idle pawn requests
/// an async route to a random grid node, holds until the ticket resolves, and
/// then drives it under the same car-following rules the freight fleet uses.
fn drive_bench_traffic(
    mut svc: ResMut<PathService>,
    occupancy: Res<LaneOccupancy>,
    destinations: Res<Destinations>,
    // (Destinations is inserted empty before the app first ticks.)
    mut pawns: Query<(Entity, &mut ActiveVehicle)>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    if destinations.0.is_empty() {
        return;
    }
    let dt = SECS_PER_PASS as f32;
    for (entity, mut vehicle) in &mut pawns {
        if vehicle.route.is_empty() {
            if let Some(ticket) = vehicle.pending_path {
                match svc.poll(ticket) {
                    PathPoll::Pending => continue,
                    PathPoll::Ready(None) => vehicle.pending_path = None,
                    PathPoll::Ready(Some(route)) => {
                        vehicle.pending_path = None;
                        vehicle.route = route;
                        vehicle.leg = 0;
                        vehicle.s = 0.0;
                    }
                }
            }
            if vehicle.route.is_empty() {
                let Some(start) = nearest_node_unbounded(vehicle.pos, &nodes) else {
                    continue;
                };
                let mut seed = entity.index().index() as u64 + 1;
                let goal =
                    destinations.0[xorshift(&mut seed) as usize % destinations.0.len()];
                if goal != start {
                    vehicle.pending_path =
                        Some(svc.request(start, goal, CostProfile::Vehicle));
                }
                continue;
            }
        }
        if advance_along_route(entity, &mut vehicle, dt, &occupancy, &segments) {
            vehicle.route = Vec::new();
            vehicle.leg = 0;
            vehicle.s = 0.0;
        }
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Time::<()>::default());
    app.add_plugins((
        SimPlugin,
        RoadSimPlugin,
        PathfindingSimPlugin,
        TrafficSimPlugin,
    ));
    app.add_systems(
        SimTick,
        drive_bench_traffic
            .in_set(SimStage::MovementAndTransfers)
            .after(LanePrep),
    );

    app.world_mut().insert_resource(Destinations(Vec::new()));

    // Grid streets: dirt rows and columns, shared nodes snapped at crossings.
    {
        let mut roads = app.world_mut().resource_mut::<RoadEditQueue>();
        for i in 0..GRID {
            for j in 0..(GRID - 1) {
                let (fi, fj) = (i as f32 * SPACING, j as f32 * SPACING);
                roads.0.push(RoadEdit::Place {
                    from: Vec3::new(fj, 0.0, fi),
                    to: Vec3::new(fj + SPACING, 0.0, fi),
                    class: RoadClass::Dirt,
                });
                roads.0.push(RoadEdit::Place {
                    from: Vec3::new(fi, 0.0, fj),
                    to: Vec3::new(fi, 0.0, fj + SPACING),
                    class: RoadClass::Dirt,
                });
            }
        }
    }
    tick(&mut app);

    let world = app.world_mut();
    let node_list: Vec<(Entity, Vec3)> = world
        .query::<(Entity, &RoadNode)>()
        .iter(world)
        .map(|(e, n)| (e, n.pos))
        .collect();
    let segment_count = world.query::<&RoadSegment>().iter(world).count();
    println!(
        "[bench] grid: {} nodes, {segment_count} segments",
        node_list.len()
    );

    // 10k pawns scattered across the grid nodes.
    let mut seed = 0xB4_u64;
    for i in 0..VEHICLES {
        let (_, pos) = node_list[xorshift(&mut seed) as usize % node_list.len()];
        let mut v = ActiveVehicle::at(pos);
        v.heading = Vec3::X;
        let _ = i;
        world.spawn(v);
    }
    world.insert_resource(Destinations(
        node_list.iter().map(|(e, _)| *e).collect(),
    ));

    for _ in 0..WARMUP_TICKS {
        tick(&mut app);
    }
    let world = app.world_mut();
    let moving = world
        .query::<&ActiveVehicle>()
        .iter(world)
        .filter(|v| !v.route.is_empty())
        .count();
    println!("[bench] after warmup: {moving}/{VEHICLES} pawns en route");

    let mut samples: Vec<f64> = Vec::with_capacity(MEASURE_TICKS as usize);
    for _ in 0..MEASURE_TICKS {
        let start = Instant::now();
        tick(&mut app);
        samples.push(start.elapsed().as_secs_f64() * 1e3);
    }
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let mut sorted = samples;
    sorted.sort_by(f64::total_cmp);
    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
    let ticks = app.world().resource::<TickIndex>().0;
    println!(
        "[bench] {VEHICLES} vehicles, {MEASURE_TICKS} measured ticks (total {ticks}): \
         mean {mean:.4} ms  p95 {p95:.4} ms  (gate {GATE_MS} ms)"
    );

    if moving < VEHICLES / 2 {
        println!("[bench] FAIL: only {moving} pawns en route — the driver is not exercising traffic");
        std::process::exit(1);
    }
    if mean > GATE_MS {
        println!("[bench] FAIL: mean {mean:.4} ms exceeds the {GATE_MS} ms gate");
        std::process::exit(1);
    }
    println!("[bench] PASS");
}
