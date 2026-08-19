//! B8.4 benchmark gate: the utility web at district scale, headless. Four
//! districts, each with 50 dwellings + 5 factories and its own plant row
//! (power, water, sewage, heat), wired as one component per NetKind per
//! district — all three solvers (grid, water⇄sewage cycle, district heat)
//! live every tick, with Climate pinned to midwinter so heat demand bites.
//! Steady-state SimTick must stay inside the 1 ms budget.
//!
//! Run: cargo run --release --bin bench_networks

use std::time::{Duration, Instant};

use bevy::prelude::*;

use soviet_simulator::SimPlugins;
use soviet_simulator::sim::TickIndex;
use soviet_simulator::sim::buildings::{
    Building, BuildingEdit, BuildingEditQueue, BuildingKind, Powered,
};
use soviet_simulator::sim::clock::SECS_PER_PASS;
use soviet_simulator::sim::heat::{Climate, Heated};
use soviet_simulator::sim::resources::{Inventory, ResourceKind};
use soviet_simulator::sim::water::Watered;
use soviet_simulator::sim::wires::{NetKind, WireEdit, WireEditQueue};

const DISTRICTS: u32 = 4;
const SPACING: f32 = 600.0;
const DWELLINGS: u32 = 50; // per district
const FACTORIES: u32 = 5; // per district
// Plant rows sized to genuinely cover midwinter demand per district:
// power 50×1 + 5×4 = 70 MW → 8×10 MW; water/sewage 50×1 + 5×2 = 60 → 4×20;
// heat at −10 °C = 50×5 = 250 → 5×60.
const POWER_PLANTS: u32 = 8;
const WATER_PUMPS: u32 = 4;
const SEWAGE_PLANTS: u32 = 4;
const HEAT_PLANTS: u32 = 5;
const WARMUP_TICKS: u32 = 300;
const MEASURE_TICKS: u32 = 2_000;
const GATE_MS: f64 = 1.0;

fn tick(app: &mut App) {
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f64(SECS_PER_PASS + 1e-9));
    app.update();
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Time::<()>::default());
    // The utility web only: buildings are placed already-finished (no
    // construction plugin, so no ConstructionSite ever attaches), no roads,
    // no fleet, no citizens, no customs, no save/load.
    app.add_plugins(
        SimPlugins
            .build()
            .disable::<soviet_simulator::sim::roads::RoadSimPlugin>()
            .disable::<soviet_simulator::sim::storage::StorageSimPlugin>()
            .disable::<soviet_simulator::sim::households::HouseholdSimPlugin>()
            .disable::<soviet_simulator::sim::labour::LabourSimPlugin>()
            .disable::<soviet_simulator::sim::commute::CommuteSimPlugin>()
            .disable::<soviet_simulator::sim::needs::NeedsSimPlugin>()
            .disable::<soviet_simulator::sim::vehicles::VehicleSimPlugin>()
            .disable::<soviet_simulator::sim::dispatch::DispatchSimPlugin>()
            .disable::<soviet_simulator::sim::construction::ConstructionSimPlugin>()
            .disable::<soviet_simulator::sim::zoning::ZoningSimPlugin>()
            .disable::<soviet_simulator::sim::customs::CustomsSimPlugin>()
            .disable::<soviet_simulator::sim::save::SaveSimPlugin>(),
    );

    // Layout per district: a 10×5 dwelling grid, a factory row south of it,
    // and the plant rows east. Positions are what the wire edits snap to.
    let mut positions: Vec<Vec<Vec3>> = Vec::new(); // per district, all consumers+plants in wiring order
    {
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        for i in 0..DISTRICTS {
            let z0 = i as f32 * SPACING;
            let mut district: Vec<Vec3> = Vec::new();
            for d in 0..DWELLINGS {
                let pos = Vec3::new((d % 10) as f32 * 14.0, 0.0, z0 + (d / 10) as f32 * 14.0);
                buildings.0.push(BuildingEdit::Place {
                    kind: BuildingKind::Dwelling,
                    pos,
                });
                district.push(pos);
            }
            for f in 0..FACTORIES {
                let pos = Vec3::new(f as f32 * 30.0, 0.0, z0 + 90.0);
                buildings.0.push(BuildingEdit::Place {
                    kind: BuildingKind::Factory,
                    pos,
                });
                district.push(pos);
            }
            for (kind, count, x) in [
                (BuildingKind::PowerPlant, POWER_PLANTS, 170.0),
                (BuildingKind::WaterPump, WATER_PUMPS, 200.0),
                (BuildingKind::SewagePlant, SEWAGE_PLANTS, 230.0),
                (BuildingKind::HeatPlant, HEAT_PLANTS, 260.0),
            ] {
                for n in 0..count {
                    let pos = Vec3::new(x, 0.0, z0 + n as f32 * 20.0);
                    buildings.0.push(BuildingEdit::Place { kind, pos });
                    district.push(pos);
                }
            }
            positions.push(district);
        }
    }
    tick(&mut app); // flush placements so wire endpoints snap to buildings

    // Wire every district into one component per NetKind: a daisy chain
    // through every building in the district, three parallel webs.
    {
        let mut wires = app.world_mut().resource_mut::<WireEditQueue>();
        for district in &positions {
            for kind in [NetKind::Power, NetKind::Water, NetKind::Heat] {
                for pair in district.windows(2) {
                    wires.0.push(WireEdit::Place {
                        from: pair[0],
                        to: pair[1],
                        kind,
                    });
                }
            }
        }
    }
    tick(&mut app); // flush wires

    // Fuel the burners (power + heat plants) with a roomy coal bunker that
    // outlasts warmup + measurement, and pin the climate to midwinter so
    // every dwelling draws full heat.
    {
        let world = app.world_mut();
        let plants: Vec<Entity> = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .filter(|(_, b)| matches!(b.kind, BuildingKind::PowerPlant | BuildingKind::HeatPlant))
            .map(|(e, _)| e)
            .collect();
        for plant in plants {
            let mut inv = Inventory::new(500.0);
            inv.add(ResourceKind::Coal, 500.0);
            world.entity_mut(plant).insert(inv);
        }
        let mut climate = world.resource_mut::<Climate>();
        climate.auto = false;
        climate.temperature = -10.0;
    }

    for _ in 0..WARMUP_TICKS {
        tick(&mut app);
    }

    // Coverage proof: the three gates genuinely hold across the whole scene.
    let (spans, powered, watered, heated, dwellings, factories) = {
        let world = app.world_mut();
        let spans = world
            .query::<&soviet_simulator::sim::wires::WireSpan>()
            .iter(world)
            .count();
        let powered = world
            .query::<&Powered>()
            .iter(world)
            .filter(|p| p.0)
            .count();
        let watered = world
            .query::<&Watered>()
            .iter(world)
            .filter(|w| w.0)
            .count();
        let heated = world.query::<&Heated>().iter(world).filter(|h| h.0).count();
        let mut q = world.query::<&Building>();
        let dwellings = q
            .iter(world)
            .filter(|b| b.kind == BuildingKind::Dwelling)
            .count();
        let factories = q
            .iter(world)
            .filter(|b| b.kind == BuildingKind::Factory)
            .count();
        (spans, powered, watered, heated, dwellings, factories)
    };
    println!(
        "[bench] {spans} spans; after warmup: {powered} powered, {watered} watered, \
         {heated} heated ({dwellings} dwellings, {factories} factories, midwinter)"
    );
    assert_eq!(dwellings, (DISTRICTS * DWELLINGS) as usize);
    assert_eq!(factories, (DISTRICTS * FACTORIES) as usize);
    assert_eq!(powered, dwellings + factories, "[bench] grid not fully lit");
    assert_eq!(
        watered,
        dwellings + factories,
        "[bench] water cycle not closed"
    );
    assert_eq!(
        heated, dwellings,
        "[bench] heat not covering midwinter demand"
    );

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
    let max = sorted[sorted.len() - 1];
    let ticks_total = app.world().resource::<TickIndex>().0;
    println!(
        "[bench] {DISTRICTS} districts ({dwellings} dwellings, {factories} factories, \
         3 nets), {MEASURE_TICKS} measured ticks (total {ticks_total}): \
         mean {mean:.4} ms  p95 {p95:.4} ms  max {max:.4} ms  (gate {GATE_MS} ms)"
    );
    if mean > GATE_MS {
        println!("[bench] FAIL: mean {mean:.4} ms exceeds the {GATE_MS} ms gate");
        std::process::exit(1);
    }
    println!("[bench] PASS");
}
