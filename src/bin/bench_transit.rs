//! B5.6 benchmark gate (ticket #51): headless transit at scale. 125 isolated
//! districts, each an 1800 m corridor whose factories are beyond walking
//! tolerance — every worker's commute is a genuine multi-leg transit trip
//! (walk → queue → ride → walk). At the morning peak ≥5000 such trips are in
//! flight at once, and the whole tick must stay inside the 2 ms medium-band
//! budget alongside the labour/needs/dispatch machinery.
//!
//! Run: cargo run --release --bin bench_transit

use std::time::{Duration, Instant};

use bevy::prelude::*;

use soviet_simulator::SimPlugins;
use soviet_simulator::sim::TickIndex;
use soviet_simulator::sim::buildings::{Building, BuildingEdit, BuildingEditQueue, BuildingKind};
use soviet_simulator::sim::clock::SECS_PER_PASS;
use soviet_simulator::sim::commute::{CommutePhase, CommuterPawn};
use soviet_simulator::sim::households::RecruitmentPlan;
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue};
use soviet_simulator::sim::transit::{TransitEdit, TransitEditQueue, TransitLine};
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue};

const DISTRICTS: u32 = 125;
const SPACING: f32 = 300.0;
const CORRIDOR: f32 = 1800.0;
/// Assignment settles before the 150-frame morning window opens.
const WARMUP_TICKS: u32 = 148;
const MEASURE_TICKS: u32 = 300;
const GATE_MS: f64 = 2.0;
const RIDER_GATE: usize = 5_000;

fn tick(app: &mut App) {
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f64(SECS_PER_PASS + 1e-9));
    app.update();
}

fn main() {
    let mut app = App::new();
    // Pre-G1 fiat economy: these scenarios predate the rouble; an
    // infinite treasury keeps them reproducing their recorded stories.
    app.insert_resource(soviet_simulator::sim::plan::Treasury {
        roubles: f32::INFINITY,
    });
    app.insert_resource(Time::<()>::default());
    // Isolates transit-borne commuting: no storage/dispatch (no freight),
    // no construction, no zones, no utility solvers, no customs, no
    // save/load. CommuteSimPlugin auto-adds Pathfinding + Transit.
    app.add_plugins(
        SimPlugins
            .build()
            .disable::<soviet_simulator::sim::storage::StorageSimPlugin>()
            .disable::<soviet_simulator::sim::needs::NeedsSimPlugin>()
            .disable::<soviet_simulator::sim::dispatch::DispatchSimPlugin>()
            .disable::<soviet_simulator::sim::construction::ConstructionSimPlugin>()
            .disable::<soviet_simulator::sim::zoning::ZoningSimPlugin>()
            .disable::<soviet_simulator::sim::water::WaterSimPlugin>()
            .disable::<soviet_simulator::sim::heat::HeatSimPlugin>()
            .disable::<soviet_simulator::sim::customs::CustomsSimPlugin>()
            .disable::<soviet_simulator::sim::wires::WireSimPlugin>()
            .disable::<soviet_simulator::sim::save::SaveSimPlugin>(),
    );

    // District: dirt corridor with two dwellings west (16 households ≈ 40
    // citizens), four factories east (40 slots), stops docking both end
    // nodes, one depot with a single bus on the line (undersized on purpose: the queue at the shelter is part of the load).
    {
        let mut roads = app.world_mut().resource_mut::<RoadEditQueue>();
        for i in 0..DISTRICTS {
            let z = i as f32 * SPACING;
            roads.0.push(RoadEdit::Place {
                from: Vec3::new(0.0, 0.0, z),
                to: Vec3::new(CORRIDOR, 0.0, z),
                class: RoadClass::Dirt,
            });
        }
    }
    {
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        for i in 0..DISTRICTS {
            let z = i as f32 * SPACING;
            for (kind, pos) in [
                (BuildingKind::Dwelling, Vec3::new(0.0, 0.0, z + 15.0)),
                (BuildingKind::Dwelling, Vec3::new(-14.0, 0.0, z + 15.0)),
                (BuildingKind::Factory, Vec3::new(CORRIDOR, 0.0, z + 15.0)),
                (
                    BuildingKind::Factory,
                    Vec3::new(CORRIDOR + 14.0, 0.0, z + 15.0),
                ),
                (BuildingKind::Factory, Vec3::new(CORRIDOR, 0.0, z - 15.0)),
                (
                    BuildingKind::Factory,
                    Vec3::new(CORRIDOR + 14.0, 0.0, z - 15.0),
                ),
                (BuildingKind::BusStop, Vec3::new(5.0, 0.0, z - 8.0)),
                (
                    BuildingKind::BusStop,
                    Vec3::new(CORRIDOR - 5.0, 0.0, z - 8.0),
                ),
                (BuildingKind::Depot, Vec3::new(0.0, 0.0, z + 40.0)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
    }
    app.world_mut()
        .resource_mut::<RecruitmentPlan>()
        .target_households = DISTRICTS * 16;
    tick(&mut app);

    // Wire each district's line and buses.
    {
        let world = app.world_mut();
        let mut stops: Vec<(i64, f32, Entity)> = Vec::new();
        let mut depots: Vec<(i64, Entity)> = Vec::new();
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            let district = (b.pos.z / SPACING).round() as i64;
            match b.kind {
                BuildingKind::BusStop => stops.push((district, b.pos.x, e)),
                BuildingKind::Depot => depots.push((district, e)),
                _ => {}
            }
        }
        let mut edits = world.resource_mut::<TransitEditQueue>();
        for &(district, _) in &depots {
            let mut pair: Vec<(f32, Entity)> = stops
                .iter()
                .filter(|(d, ..)| *d == district)
                .map(|&(_, x, e)| (x, e))
                .collect();
            pair.sort_by(|a, b| a.0.total_cmp(&b.0));
            edits.0.push(TransitEdit::CreateLine {
                stops: pair.into_iter().map(|(_, e)| e).collect(),
            });
        }
        let mut vehicles = world.resource_mut::<VehicleEditQueue>();
        for &(_, depot) in &depots {
            vehicles.0.push(VehicleEdit::BuyBus { depot });
        }
    }
    tick(&mut app);
    {
        let world = app.world_mut();
        let lines: Vec<(Entity, Vec3)> = world
            .query::<(Entity, &TransitLine)>()
            .iter(world)
            .filter_map(|(e, l)| {
                l.stops
                    .first()
                    .and_then(|&s| world.get::<Building>(s))
                    .map(|b| (e, b.pos))
            })
            .collect();
        let depots: Vec<(Entity, Vec3)> = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .filter(|(_, b)| b.kind == BuildingKind::Depot)
            .map(|(e, b)| (e, b.pos))
            .collect();
        let mut edits = world.resource_mut::<TransitEditQueue>();
        for (line, line_pos) in &lines {
            // the district's depot is the nearest one
            let depot = depots
                .iter()
                .min_by(|a, b| {
                    a.1.distance_squared(*line_pos)
                        .total_cmp(&b.1.distance_squared(*line_pos))
                })
                .unwrap()
                .0;
            edits.0.push(TransitEdit::AssignBus { line: *line, depot });
        }
    }

    for _ in 0..WARMUP_TICKS {
        tick(&mut app);
    }
    {
        let world = app.world_mut();
        let citizens = world
            .query::<&soviet_simulator::sim::citizens::Citizen>()
            .iter(world)
            .count();
        let assigned = world
            .query::<&soviet_simulator::sim::citizens::Citizen>()
            .iter(world)
            .filter(|c| c.work.is_some())
            .count();
        println!("[bench] {citizens} citizens, {assigned} assigned via transit catchment");
    }

    // The morning wave: measure through it, tracking the peak number of
    // multi-leg transit trips in flight (planned, queued or aboard).
    let mut samples: Vec<f64> = Vec::with_capacity(MEASURE_TICKS as usize);
    let mut peak_riders = 0usize;
    for _ in 0..MEASURE_TICKS {
        let start = Instant::now();
        tick(&mut app);
        samples.push(start.elapsed().as_secs_f64() * 1e3);
        let world = app.world_mut();
        let in_flight = world
            .query::<&CommuterPawn>()
            .iter(world)
            .filter(|p| {
                p.transit.is_some()
                    || matches!(
                        p.phase,
                        CommutePhase::Wait { .. } | CommutePhase::Ride { .. }
                    )
            })
            .count();
        peak_riders = peak_riders.max(in_flight);
    }
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let mut sorted = samples;
    sorted.sort_by(f64::total_cmp);
    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
    let ticks_total = app.world().resource::<TickIndex>().0;
    println!(
        "[bench] peak {peak_riders} concurrent transit trips, {MEASURE_TICKS} measured ticks \
         (total {ticks_total}): mean {mean:.4} ms  p95 {p95:.4} ms  (gate {GATE_MS} ms)"
    );

    if peak_riders < RIDER_GATE {
        println!("[bench] FAIL: peak {peak_riders} concurrent riders (need {RIDER_GATE})");
        std::process::exit(1);
    }
    if mean > GATE_MS {
        println!("[bench] FAIL: mean {mean:.4} ms exceeds the {GATE_MS} ms gate");
        std::process::exit(1);
    }
    println!("[bench] PASS");
}
