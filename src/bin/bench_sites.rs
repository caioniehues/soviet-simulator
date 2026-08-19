//! B6.6 benchmark gate (ticket #58): headless construction at scale. 100
//! isolated districts each raise a factory through the full phased pipeline
//! at once — the dispatcher hauls each site's bill from the district quarry
//! and warehouse while the construction office's excavator and crane drive
//! out and work the phases. The whole tick must stay inside the 2 ms
//! medium-band budget with all 100 sites live.
//!
//! Run: cargo run --release --bin bench_sites

use std::time::{Duration, Instant};

use bevy::prelude::*;

use soviet_simulator::SimPlugins;
use soviet_simulator::sim::TickIndex;
use soviet_simulator::sim::buildings::{Building, BuildingEdit, BuildingEditQueue, BuildingKind};
use soviet_simulator::sim::clock::SECS_PER_PASS;
use soviet_simulator::sim::construction::ConstructionSite;
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue};
use soviet_simulator::sim::storage::default_policies;
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue, VehicleKind};

const DISTRICTS: u32 = 100;
const SPACING: f32 = 300.0;
const WARMUP_TICKS: u32 = 300;
const MEASURE_TICKS: u32 = 1_000;
const GATE_MS: f64 = 2.0;

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
    // Isolates construction: no citizens, no zones, no utility solvers, no
    // customs, no wires, no save/load.
    app.add_plugins(
        SimPlugins
            .build()
            .disable::<soviet_simulator::sim::households::HouseholdSimPlugin>()
            .disable::<soviet_simulator::sim::labour::LabourSimPlugin>()
            .disable::<soviet_simulator::sim::commute::CommuteSimPlugin>()
            .disable::<soviet_simulator::sim::needs::NeedsSimPlugin>()
            .disable::<soviet_simulator::sim::zoning::ZoningSimPlugin>()
            .disable::<soviet_simulator::sim::water::WaterSimPlugin>()
            .disable::<soviet_simulator::sim::heat::HeatSimPlugin>()
            .disable::<soviet_simulator::sim::customs::CustomsSimPlugin>()
            .disable::<soviet_simulator::sim::wires::WireSimPlugin>()
            .disable::<soviet_simulator::sim::save::SaveSimPlugin>(),
    );

    // District: road x∈[0,120]; suppliers + depot + office west (completed by
    // fiat and stocked), the factory construction site east.
    {
        let mut roads = app.world_mut().resource_mut::<RoadEditQueue>();
        for i in 0..DISTRICTS {
            let z = i as f32 * SPACING;
            roads.0.push(RoadEdit::Place {
                from: Vec3::new(0.0, 0.0, z),
                to: Vec3::new(120.0, 0.0, z),
                class: RoadClass::Dirt,
            });
        }
    }
    {
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        for i in 0..DISTRICTS {
            let z = i as f32 * SPACING;
            for (kind, pos) in [
                (BuildingKind::Quarry, Vec3::new(-10.0, 0.0, z)),
                (BuildingKind::Warehouse, Vec3::new(0.0, 0.0, z - 20.0)),
                (BuildingKind::Depot, Vec3::new(0.0, 0.0, z + 20.0)),
                (
                    BuildingKind::ConstructionOffice,
                    Vec3::new(20.0, 0.0, z + 20.0),
                ),
                (BuildingKind::Factory, Vec3::new(130.0, 0.0, z)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
    }
    tick(&mut app);

    // Fiat-complete the infrastructure, stock the suppliers, buy the fleets.
    {
        let world = app.world_mut();
        let found: Vec<(Entity, BuildingKind)> = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .map(|(e, b)| (e, b.kind))
            .collect();
        for (entity, kind) in found {
            if kind == BuildingKind::Factory {
                continue; // the one genuine site per district
            }
            world.entity_mut(entity).remove::<ConstructionSite>();
            world.entity_mut(entity).insert((
                Inventory::new(kind.inventory_capacity().max(60.0)),
                default_policies(kind),
            ));
            match kind {
                BuildingKind::Quarry => {
                    world
                        .get_mut::<Inventory>(entity)
                        .unwrap()
                        .add(ResourceKind::Gravel, 50.0);
                }
                BuildingKind::Warehouse => {
                    world
                        .get_mut::<Inventory>(entity)
                        .unwrap()
                        .add(ResourceKind::Goods, 50.0);
                }
                BuildingKind::Depot => {
                    let mut edits = world.resource_mut::<VehicleEditQueue>();
                    edits.0.push(VehicleEdit::BuyTruck {
                        depot: entity,
                        class: TransportClass::Bulk,
                    });
                    edits.0.push(VehicleEdit::BuyTruck {
                        depot: entity,
                        class: TransportClass::Covered,
                    });
                }
                BuildingKind::ConstructionOffice => {
                    let mut edits = world.resource_mut::<VehicleEditQueue>();
                    edits.0.push(VehicleEdit::BuyMachine {
                        office: entity,
                        kind: VehicleKind::Excavator,
                    });
                    edits.0.push(VehicleEdit::BuyMachine {
                        office: entity,
                        kind: VehicleKind::Crane,
                    });
                }
                _ => {}
            }
        }
    }

    for _ in 0..WARMUP_TICKS {
        tick(&mut app);
    }
    let live_sites = {
        let world = app.world_mut();
        world.query::<&ConstructionSite>().iter(world).count()
    };
    println!("[bench] {live_sites} live construction sites after warmup");

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
    let ticks_total = app.world().resource::<TickIndex>().0;
    // Coverage proof: sites genuinely progressed during the window.
    let (still_building, total_progress) = {
        let world = app.world_mut();
        let sites: Vec<f32> = world
            .query::<&ConstructionSite>()
            .iter(world)
            .map(|s| s.progress())
            .collect();
        (sites.len(), sites.iter().sum::<f32>())
    };
    println!(
        "[bench] {DISTRICTS} districts, {MEASURE_TICKS} measured ticks (total {ticks_total}): \
         mean {mean:.4} ms  p95 {p95:.4} ms  (gate {GATE_MS} ms); \
         {still_building} sites still building, summed progress {total_progress:.1}"
    );

    if live_sites < DISTRICTS as usize {
        println!("[bench] FAIL: only {live_sites} live sites (need {DISTRICTS})");
        std::process::exit(1);
    }
    if mean > GATE_MS {
        println!("[bench] FAIL: mean {mean:.4} ms exceeds the {GATE_MS} ms gate");
        std::process::exit(1);
    }
    println!("[bench] PASS");
}
