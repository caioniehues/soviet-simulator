//! District heating + climate (B8.3, spec/heating.md): heat is a produced
//! resource — plants burn real coal into heat pumped over `NetKind::Heat`
//! pipes — and demand is temperature-driven: the colder the season, the more
//! every dwelling draws. A cold home is an unmet need with real consequences
//! (rest collapses, attendance follows) — winter is a systems event, not
//! weather décor. Opt-in like water: no plugin, no `Heated` gate.

use bevy::prelude::*;

use super::buildings::Building;
use super::clock::{FRAMES_PER_GAME_DAY, FrameIndex};
use super::network::{Components, PriorityClass, allocate};
use super::stages::{SimStage, SimTick};
use super::wires::{NetKind, WireSpan};

/// Game days per climate year (the seasonal sinusoid's period).
pub const YEAR_DAYS: f32 = 12.0;
/// Yearly mean and swing, °C: +20 midsummer, −10 midwinter.
pub const MEAN_TEMP: f32 = 5.0;
pub const TEMP_SWING: f32 = 15.0;
/// Below this temperature homes start drawing heat.
pub const HEATING_THRESHOLD: f32 = 15.0;
/// A dwelling's draw at the deepest cold.
pub const DWELLING_HEAT_MAX: f32 = 5.0;
/// Plant output while fuelled, heat units per tick.
pub const HEAT_PLANT_OUTPUT: f32 = 60.0;
/// Coal burned per tick while producing.
pub const HEAT_PLANT_COAL_BURN: f32 = 0.02;

/// The shared weather state. `auto` lets tests pin the temperature.
#[derive(Resource)]
pub struct Climate {
    pub temperature: f32,
    pub auto: bool,
}

impl Default for Climate {
    fn default() -> Self {
        Self {
            temperature: MEAN_TEMP,
            auto: true,
        }
    }
}

/// Seasonal temperature for a frame: a year-long cosine, coldest mid-cycle.
pub fn temperature_at(frame: u32) -> f32 {
    let year = frame as f32 / (FRAMES_PER_GAME_DAY as f32 * YEAR_DAYS);
    MEAN_TEMP + TEMP_SWING * (std::f32::consts::TAU * year).cos()
}

/// Per-dwelling heat draw at `temperature` — zero in the warm months,
/// full at 25° below the threshold.
pub fn dwelling_heat_demand(temperature: f32) -> f32 {
    DWELLING_HEAT_MAX * ((HEATING_THRESHOLD - temperature) / 25.0).clamp(0.0, 1.0)
}

/// This tick's heat output of a plant (fuel-gated, like power).
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct HeatOutput(pub f32);

/// Whether the dwelling's heat demand is met this tick. Trivially true in
/// the warm months (zero demand).
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Heated(pub bool);

pub struct HeatSimPlugin;

impl Plugin for HeatSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Climate>()
            .add_observer(attach_heat_components)
            .add_systems(
                SimTick,
                (update_climate, solve_heat)
                    .chain()
                    .in_set(SimStage::ProductionAndUtilities)
                    // the plant pours this tick's heat before the pipes are solved
                    .after(super::production::produce_flows),
            );
    }
}

fn attach_heat_components(
    add: On<Add, Building>,
    buildings: Query<(&Building, Has<Heated>, Has<HeatOutput>)>,
    mut commands: Commands,
) {
    use super::catalogue::{HeatDemand, spec};
    let Ok((building, has_heated, has_output)) = buildings.get(add.entity) else {
        return;
    };
    match spec(building.kind).heat {
        Some(HeatDemand::Consumer) if !has_heated => {
            commands.entity(add.entity).insert(Heated::default());
        }
        Some(HeatDemand::Producer) if !has_output => {
            commands.entity(add.entity).insert(HeatOutput::default());
        }
        _ => {}
    }
}

fn update_climate(frame: Res<FrameIndex>, mut climate: ResMut<Climate>) {
    if climate.auto {
        climate.temperature = temperature_at(frame.0);
    }
}

fn solve_heat(
    climate: Res<Climate>,
    spans: Query<&WireSpan>,
    outputs: Query<(Entity, &HeatOutput)>,
    mut consumers: Query<(Entity, &Building, &mut Heated)>,
) {
    let demand = dwelling_heat_demand(climate.temperature);
    let mut components = Components::from_spans(
        spans
            .iter()
            .filter(|s| s.kind == NetKind::Heat)
            .map(|s| (s.a, s.b)),
    );
    let demands: Vec<(Entity, u64, PriorityClass, f32)> = consumers
        .iter()
        .map(|(e, b, _)| (e, b.id.0, PriorityClass::Housing, demand))
        .collect();
    let served = allocate(
        &mut components,
        outputs.iter().map(|(e, o)| (e, o.0)),
        &demands,
    );
    for (entity, _, mut heated) in &mut consumers {
        // Zero demand (warm season) is trivially met even off-grid.
        let ok = demand <= 1e-6 || served.get(&entity).copied().unwrap_or(false);
        if heated.0 != ok {
            heated.0 = ok;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin};
    use super::super::resources::{Inventory, ResourceKind};
    use super::super::wires::{PoleId, SpanId, WirePole};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, HeatSimPlugin));
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

    fn place_building(app: &mut App, kind: BuildingKind, pos: Vec3) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place { kind, pos });
    }

    fn building_entity(app: &mut App, kind: BuildingKind) -> Entity {
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        q.iter(world).find(|(_, b)| b.kind == kind).unwrap().0
    }

    #[test]
    fn demand_rises_with_the_cold_and_vanishes_in_summer() {
        assert_eq!(dwelling_heat_demand(20.0), 0.0);
        assert!(dwelling_heat_demand(0.0) > 0.0);
        assert!(dwelling_heat_demand(-10.0) > dwelling_heat_demand(0.0));
        // the sinusoid actually swings across the year
        let coldest = (0..(FRAMES_PER_GAME_DAY as f32 * YEAR_DAYS) as u32)
            .step_by(100)
            .map(temperature_at)
            .fold(f32::INFINITY, f32::min);
        assert!(coldest < -5.0, "midwinter must bite, got {coldest}");
    }

    #[test]
    fn winter_home_is_cold_until_the_fuelled_plant_is_piped_in() {
        let mut app = app();
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::Dwelling,
                pos: Vec3::ZERO,
            });
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::HeatPlant,
                pos: Vec3::new(60.0, 0.0, 0.0),
            });
        }
        ticks(&mut app, 2);
        // pin midwinter
        {
            let mut climate = app.world_mut().resource_mut::<Climate>();
            climate.auto = false;
            climate.temperature = -10.0;
        }
        ticks(&mut app, 2);
        let world = app.world_mut();
        let (dwelling, plant) = {
            let mut q = world.query::<(Entity, &Building)>();
            let mut d = None;
            let mut p = None;
            for (e, b) in q.iter(world) {
                match b.kind {
                    BuildingKind::Dwelling => d = Some(e),
                    BuildingKind::HeatPlant => p = Some(e),
                    _ => {}
                }
            }
            (d.unwrap(), p.unwrap())
        };
        assert!(
            !world.get::<Heated>(dwelling).unwrap().0,
            "midwinter, no pipe: the home is cold"
        );
        // pipe them together and fuel the plant
        world.spawn(WireSpan {
            id: SpanId(1),
            a: plant,
            b: dwelling,
            kind: NetKind::Heat,
        });
        let _ = world.spawn(WirePole {
            id: PoleId(1),
            pos: Vec3::new(30.0, 0.0, 0.0),
            kind: NetKind::Heat,
        });
        world
            .get_mut::<Inventory>(plant)
            .unwrap()
            .add(ResourceKind::Coal, 10.0);
        ticks(&mut app, 2);
        assert!(app.world().get::<Heated>(dwelling).unwrap().0);
        // summer: even off-grid homes are trivially warm
        {
            let mut climate = app.world_mut().resource_mut::<Climate>();
            climate.temperature = 20.0;
        }
        // cut the pipe by draining the plant's coal — doesn't matter in June
        app.world_mut()
            .get_mut::<Inventory>(plant)
            .unwrap()
            .take(ResourceKind::Coal, f32::INFINITY);
        ticks(&mut app, 2);
        assert!(app.world().get::<Heated>(dwelling).unwrap().0);
    }

    /// The inertness contract (`construction::ConstructionSite`: "present ⇒ the
    /// building is inert"). `extract_resources`, `run_power_plants` and
    /// `run_factories` all filter on it; `run_heat_plants` never did, so a
    /// half-built heat plant burned its coal and warmed the district at full
    /// output. Phase 4 of the catalogue refactor makes the filter structural —
    /// this is the sanctioned behaviour change, and this test is what proves it
    /// happened rather than being claimed.
    #[test]
    fn a_heat_plant_under_construction_burns_nothing_and_warms_nobody() {
        let mut app = app();
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::HeatPlant,
                pos: Vec3::ZERO,
            });
        }
        ticks(&mut app, 2);
        let plant = {
            let world = app.world_mut();
            let mut q = world.query::<(Entity, &Building)>();
            q.iter(world)
                .find(|(_, b)| b.kind == BuildingKind::HeatPlant)
                .map(|(e, _)| e)
                .expect("the plant was placed")
        };
        // fuel it, then put it back under construction
        app.world_mut()
            .get_mut::<Inventory>(plant)
            .unwrap()
            .add(ResourceKind::Coal, 10.0);
        app.world_mut().entity_mut(plant).insert(
            super::super::construction::ConstructionSite::for_kind(BuildingKind::HeatPlant),
        );
        let before = app
            .world()
            .get::<Inventory>(plant)
            .unwrap()
            .amount(ResourceKind::Coal);
        ticks(&mut app, 2);
        let after = app
            .world()
            .get::<Inventory>(plant)
            .unwrap()
            .amount(ResourceKind::Coal);
        assert_eq!(
            after, before,
            "a site is not a plant: it must not burn coal while it is still being built"
        );
        assert_eq!(
            app.world().get::<HeatOutput>(plant).unwrap().0,
            0.0,
            "a site is not a plant: it must not emit heat while it is still being built"
        );
    }

    /// Mirrors `wires`'s `the_grid_serves_exactly_the_draw_the_power_column_claims`:
    /// membership on the district net must come from the `heat` column alone, not
    /// from a kind's identity — a `None` row earns no `Heated` gate, `Producer`
    /// earns no `Heated` gate either (that's `HeatOutput`'s job), and `Consumer`
    /// both earns the gate and is servable through exactly the pool it claims.
    #[test]
    fn the_district_heats_exactly_the_draw_the_heat_column_claims() {
        use super::super::catalogue::{HeatDemand, spec};
        for kind in BuildingKind::ALL {
            let mut app = app();
            {
                // midwinter: a consumer's demand is nonzero, so the servable
                // check below actually exercises the grid instead of the
                // "zero demand is trivially met" shortcut in `solve_heat`.
                let mut climate = app.world_mut().resource_mut::<Climate>();
                climate.auto = false;
                climate.temperature = -10.0;
            }
            place_building(&mut app, kind, Vec3::ZERO);
            ticks(&mut app, 2);
            let building = building_entity(&mut app, kind);
            match spec(kind).heat {
                None => {
                    assert!(
                        app.world().get::<Heated>(building).is_none(),
                        "{kind:?} claims no heat role, so it must carry no `Heated` gate — \
                         `solve_heat`'s query requires one, and a gate here would put the \
                         kind on the district net the table says it is off"
                    );
                }
                Some(HeatDemand::Producer) => {
                    assert!(
                        app.world().get::<Heated>(building).is_none(),
                        "{kind:?} produces heat, it does not draw it — it must not carry \
                         a `Heated` gate"
                    );
                }
                Some(HeatDemand::Consumer) => {
                    assert!(
                        app.world().get::<Heated>(building).is_some(),
                        "{kind:?} claims a heat draw but never earned a `Heated` gate, \
                         so `solve_heat` cannot see it at all"
                    );
                    let demand = dwelling_heat_demand(-10.0);
                    assert!(demand > 0.0, "midwinter must give a real draw to test against");
                    // A bare source: no `Building`, so `produce_flows` never
                    // overwrites it and the component's pool is exactly what
                    // this test says.
                    let source = app.world_mut().spawn(HeatOutput(demand)).id();
                    app.world_mut().spawn(WireSpan {
                        id: SpanId(9_000),
                        a: source,
                        b: building,
                        kind: NetKind::Heat,
                    });
                    ticks(&mut app, 1);
                    assert!(
                        app.world().get::<Heated>(building).unwrap().0,
                        "{kind:?}: a pool of exactly its claimed {demand} heat must serve it"
                    );
                    app.world_mut().get_mut::<HeatOutput>(source).unwrap().0 = demand * 0.99;
                    ticks(&mut app, 1);
                    assert!(
                        !app.world().get::<Heated>(building).unwrap().0,
                        "{kind:?}: one percent short of its claimed {demand} heat must starve it"
                    );
                }
            }
        }
    }
}
