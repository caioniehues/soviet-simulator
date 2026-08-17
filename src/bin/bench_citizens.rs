//! M2.7 benchmark gate (ticket #29, bench_chain style): headless,
//! graphics-independent. The full M1 chain load (100 mine→plant→factory
//! chains with trucks and wires) plus 50,000 citizen identities — housed,
//! labour-planned, commuting, with needs decay — must tick in ≤2 ms.
//!
//! Run: cargo run --release --bin bench_citizens

use std::time::{Duration, Instant};

use bevy::prelude::*;

use soviet_simulator::sim::buildings::{
    Building, BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
};
use soviet_simulator::sim::citizens::Citizen;
use soviet_simulator::sim::clock::SECS_PER_PASS;
use soviet_simulator::sim::commute::CommuteSimPlugin;
use soviet_simulator::sim::households::HouseholdSimPlugin;
use soviet_simulator::sim::households::RecruitmentPlan;
use soviet_simulator::sim::labour::LabourSimPlugin;
use soviet_simulator::sim::needs::NeedsSimPlugin;
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::vehicles::{
    ActiveVehicle, VehicleEdit, VehicleEditQueue, VehicleSimPlugin,
};
use soviet_simulator::sim::wires::{WireEdit, WireEditQueue, WireSimPlugin};
use soviet_simulator::sim::{SimPlugin, TickIndex};

const CHAINS: u32 = 100;
const CHAIN_SPACING: f32 = 250.0;
/// 21 dwellings × 8 flats × 100 chains = 16,800 flats for 16,667 households.
const DWELLINGS_PER_CHAIN: u32 = 21;
/// The deterministic size cycle [3,2,4,1,5] deals 15 citizens per 5
/// households: 16,665 → 49,995, plus sizes 3 and 2 ⇒ exactly 50,000.
const TARGET_HOUSEHOLDS: u32 = 16_667;
const TARGET_CITIZENS: usize = 50_000;
/// 2.5 game days: housing + labour settle, commute cycle reaches steady state.
const WARMUP_TICKS: u32 = 1_500;
const MEASURE_TICKS: u32 = 2_000;
const GATE_MS: f64 = 2.0;

fn tick(app: &mut App) {
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f64(SECS_PER_PASS + 1e-9));
    app.update();
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Time::<()>::default());
    app.add_plugins((
        SimPlugin,
        RoadSimPlugin,
        BuildingSimPlugin,
        HouseholdSimPlugin,
        LabourSimPlugin,
        CommuteSimPlugin,
        NeedsSimPlugin,
        VehicleSimPlugin,
        soviet_simulator::sim::storage::StorageSimPlugin,
        soviet_simulator::sim::dispatch::DispatchSimPlugin,
        WireSimPlugin,
    ));

    // The bench_chain layout — 100 disjoint chains stacked along z — plus a
    // dwelling cluster docked at each chain's x=0 node so every chain's jobs
    // are commute-feasible only from its own homes (keeps Dijkstra chain-local).
    for i in 0..CHAINS {
        let z = i as f32 * CHAIN_SPACING;
        let mut roads = app.world_mut().resource_mut::<RoadEditQueue>();
        roads.0.push(RoadEdit::Place {
            from: Vec3::new(0.0, 0.0, z),
            to: Vec3::new(120.0, 0.0, z),
            class: RoadClass::Dirt,
        });
        roads.0.push(RoadEdit::Place {
            from: Vec3::new(120.0, 0.0, z),
            to: Vec3::new(240.0, 0.0, z),
            class: RoadClass::Dirt,
        });
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Mine,
            pos: Vec3::new(-10.0, 0.0, z),
        });
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::PowerPlant,
            pos: Vec3::new(125.0, 0.0, z),
        });
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Factory,
            pos: Vec3::new(245.0, 0.0, z),
        });
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Depot,
            pos: Vec3::new(-60.0, 0.0, z),
        });
        // 7×3 grid inside DOCK_RADIUS (40 m) of the x=0 node: max offset
        // √(24² + 30²) ≈ 38.4 m.
        for d in 0..DWELLINGS_PER_CHAIN {
            let col = (d % 7) as f32;
            let row = (d / 7) as f32;
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Dwelling,
                pos: Vec3::new(-24.0 + col * 8.0, 0.0, z + 10.0 + row * 10.0),
            });
        }
        let mut wires = app.world_mut().resource_mut::<WireEditQueue>();
        wires.0.push(WireEdit::Place {
            from: Vec3::new(125.0, 0.0, z),
            to: Vec3::new(245.0, 0.0, z),
        });
    }
    tick(&mut app); // flush placements

    // Per chain: seed the mine yard, buy the depot truck, assign the shuttle.
    let mut per_chain: Vec<(Option<Entity>, Option<Entity>, Option<Entity>)> =
        vec![(None, None, None); CHAINS as usize];
    {
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        let found: Vec<(Entity, BuildingKind, f32)> =
            q.iter(world).map(|(e, b)| (e, b.kind, b.pos.z)).collect();
        for (entity, kind, z) in found {
            let chain = (z / CHAIN_SPACING).round() as usize;
            match kind {
                BuildingKind::Mine => {
                    per_chain[chain].0 = Some(entity);
                    world
                        .get_mut::<Inventory>(entity)
                        .unwrap()
                        .add(ResourceKind::Coal, 40.0);
                }
                BuildingKind::PowerPlant => per_chain[chain].1 = Some(entity),
                BuildingKind::Depot => per_chain[chain].2 = Some(entity),
                _ => {}
            }
        }
    }
    for (mine, plant, depot) in &per_chain {
        let (Some(mine), Some(plant), Some(depot)) = (mine, plant, depot) else {
            panic!("[bench] chain assembly failed: missing mine, plant or depot");
        };
        let mut edits = app.world_mut().resource_mut::<VehicleEditQueue>();
        edits.0.push(VehicleEdit::BuyTruck {
            depot: *depot,
            class: TransportClass::Bulk,
        });
        edits.0.push(VehicleEdit::CreateShuttle {
            from: *mine,
            to: *plant,
            resource: ResourceKind::Coal,
        });
    }
    app.world_mut()
        .resource_mut::<RecruitmentPlan>()
        .target_households = TARGET_HOUSEHOLDS;

    for _ in 0..WARMUP_TICKS {
        tick(&mut app);
    }
    {
        let world = app.world_mut();
        let citizens = world.query::<&Citizen>().iter(world).count();
        assert_eq!(citizens, TARGET_CITIZENS, "[bench] population mismatch");
        let employed = world
            .query::<&Citizen>()
            .iter(world)
            .filter(|c| c.work.is_some())
            .count();
        let jobs = (CHAINS
            * (BuildingKind::Mine.workers_needed()
                + BuildingKind::PowerPlant.workers_needed()
                + BuildingKind::Factory.workers_needed())) as usize;
        assert_eq!(employed, jobs, "[bench] not every vacancy filled");
        let trucks = world.query::<&ActiveVehicle>().iter(world).count();
        assert_eq!(
            trucks, CHAINS as usize,
            "[bench] expected one live truck per chain"
        );
    }

    let mut samples: Vec<f64> = Vec::with_capacity(MEASURE_TICKS as usize);
    for _ in 0..MEASURE_TICKS {
        let start = Instant::now();
        tick(&mut app);
        samples.push(start.elapsed().as_secs_f64() * 1e3);
    }

    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let mut sorted = samples.clone();
    sorted.sort_by(f64::total_cmp);
    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
    let max = sorted[sorted.len() - 1];
    let ticks = app.world().resource::<TickIndex>().0;
    println!(
        "[bench] {TARGET_CITIZENS} citizens, {CHAINS} chains, {MEASURE_TICKS} measured ticks \
         (total {ticks}): mean {mean:.4} ms  p95 {p95:.4} ms  max {max:.4} ms  (gate {GATE_MS} ms)"
    );
    if mean > GATE_MS {
        println!("[bench] FAIL: mean {mean:.4} ms exceeds the {GATE_MS} ms gate");
        std::process::exit(1);
    }
    println!("[bench] PASS");
}
