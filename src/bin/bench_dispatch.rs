//! M3.7 benchmark gate (ticket #37): headless dispatcher scale test.
//! Districts of warehouses (one stocked supplier, two starving demanders, a
//! depot with one truck) generate ~4 concurrent freight orders each; at the
//! full size that is ≥1000 live orders. The whole tick — matching pass,
//! assignment, and every truck's trip — must stay within the 2 ms budget, and
//! the run is repeated at half size to show the dispatcher's cost scales
//! sub-linearly-ish (bucketed by road component) rather than quadratically.
//!
//! Run: cargo run --release --bin bench_dispatch

use std::time::{Duration, Instant};

use bevy::prelude::*;

use soviet_simulator::sim::buildings::{
    Building, BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
};
use soviet_simulator::sim::clock::SECS_PER_PASS;
use soviet_simulator::sim::dispatch::{DispatchQueue, DispatchSimPlugin};
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::storage::StorageSimPlugin;
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
use soviet_simulator::sim::{SimPlugin, TickIndex};

const DISTRICTS: u32 = 250;
const SPACING: f32 = 300.0;
const WARMUP_TICKS: u32 = 400;
const MEASURE_TICKS: u32 = 1_000;
const GATE_MS: f64 = 2.0;
const ORDER_GATE: usize = 1_000;

fn tick(app: &mut App) {
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f64(SECS_PER_PASS + 1e-9));
    app.update();
}

/// Build a world of `districts` and run the measurement; returns
/// (mean ms, p95 ms, live orders at measure start). With `trucks` false the
/// fleet stays empty, isolating the dispatcher (matching + queue bookkeeping)
/// from trip execution — that is the sub-linearity claim under test.
fn run(districts: u32, trucks: bool) -> (f64, f64, usize) {
    let mut app = App::new();
    app.insert_resource(Time::<()>::default());
    app.add_plugins((
        SimPlugin,
        RoadSimPlugin,
        BuildingSimPlugin,
        StorageSimPlugin,
        VehicleSimPlugin,
        DispatchSimPlugin,
    ));

    // District: road x∈[0,120] at its own z; stocked supplier warehouse at the
    // west node, two empty demander warehouses at the east node, depot + one
    // bulk truck. Coal only — every district's orders hit the same matching
    // frame, the worst case for the pass.
    for i in 0..districts {
        let z = i as f32 * SPACING;
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::new(0.0, 0.0, z),
                to: Vec3::new(120.0, 0.0, z),
                class: RoadClass::Dirt,
            });
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        for (kind, pos) in [
            (BuildingKind::Warehouse, Vec3::new(-8.0, 0.0, z)),
            (BuildingKind::Warehouse, Vec3::new(128.0, 0.0, z - 8.0)),
            (BuildingKind::Warehouse, Vec3::new(128.0, 0.0, z + 8.0)),
            (BuildingKind::Depot, Vec3::new(0.0, 0.0, z + 20.0)),
        ] {
            buildings.0.push(BuildingEdit::Place { kind, pos });
        }
    }
    tick(&mut app);

    {
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        let found: Vec<(Entity, BuildingKind, Vec3)> =
            q.iter(world).map(|(e, b)| (e, b.kind, b.pos)).collect();
        for (entity, kind, pos) in found {
            match kind {
                // The western warehouse is the stocked supplier.
                BuildingKind::Warehouse if pos.x < 0.0 => {
                    world
                        .get_mut::<Inventory>(entity)
                        .unwrap()
                        .add(ResourceKind::Coal, 120.0);
                }
                BuildingKind::Depot if trucks => {
                    world
                        .resource_mut::<VehicleEditQueue>()
                        .0
                        .push(VehicleEdit::BuyTruck {
                            depot: entity,
                            class: TransportClass::Bulk,
                        });
                }
                _ => {}
            }
        }
    }

    for _ in 0..WARMUP_TICKS {
        tick(&mut app);
    }
    let live_orders = app.world().resource::<DispatchQueue>().orders.len();

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
    let storages = districts * 3;
    println!(
        "[bench] {districts} districts ({storages} storages): {live_orders} live orders, \
         {MEASURE_TICKS} measured ticks (total {ticks}): mean {mean:.4} ms  p95 {p95:.4} ms"
    );
    (mean, p95, live_orders)
}

fn main() {
    // Claim 1 — dispatcher cost sub-linear in storages: matcher-only runs
    // (no fleet, so nothing but matching + queue bookkeeping ticks).
    println!("[bench] dispatcher-only scaling:");
    let (half_mean, ..) = run(DISTRICTS / 2, false);
    let (matcher_mean, ..) = run(DISTRICTS, false);
    let scaling = matcher_mean / half_mean.max(1e-9);
    println!("[bench]   2x storages -> {scaling:.2}x dispatcher cost (gate < 2x)");

    // Claim 2 — 1k concurrent freight orders within the 2 ms tick budget,
    // full load: every district's truck out on its trips.
    println!("[bench] full load (fleet live):");
    let (mean, p95, orders) = run(DISTRICTS, true);
    println!("[bench]   {orders} live orders: mean {mean:.4} ms  p95 {p95:.4} ms  (gate {GATE_MS} ms)");

    if scaling >= 2.0 {
        println!("[bench] FAIL: dispatcher scaling {scaling:.2}x is not sub-linear");
        std::process::exit(1);
    }
    if orders < ORDER_GATE {
        println!("[bench] FAIL: only {orders} live orders (need {ORDER_GATE})");
        std::process::exit(1);
    }
    if mean > GATE_MS {
        println!("[bench] FAIL: mean {mean:.4} ms exceeds the {GATE_MS} ms gate");
        std::process::exit(1);
    }
    println!("[bench] PASS");
}
