//! The generic production pass (`.planning/2026-08-17-data-driven-buildings`,
//! phase 4; [ADR 0014](../../docs/adr/0014-production-gate-component.md)).
//!
//! Four systems used to each re-derive their own Liebig chain — the mine and
//! quarry from staffing alone, the power plant from staffing and fuel, the
//! factory from power AND water AND staff, the heat plant from fuel alone —
//! and each then discarded the scarcest factor, so no caller could learn *why*
//! output was zero. They are now one pair of systems reading `spec.recipe`,
//! and the binding constraint survives as a `Gated` component for R1's inspect
//! panel to render.
//!
//! **Why a pair and not one system.** `solve_power` is scheduled
//! `.after(run_power_plants).before(run_factories)` (`wires.rs`): the grid
//! solve sits *between* generation and consumption, so the plant has this
//! tick's output before the factory's gate is read. One system cannot be both
//! sides of that sandwich. The split is data, not a match: a kind with a
//! `flow_output` feeds a net and must produce before the solves, a kind
//! without one fills a yard and produces after.
//!
//! **The inertness filter is structural here.** Both queries carry
//! `Without<ConstructionSite>` once, where the old code applied it by hand at
//! each site and missed two: a half-built heat plant burned coal and warmed
//! the district, and a half-built pump supplied the water net.

use bevy::prelude::*;

use super::buildings::{Building, PowerOutput, Powered};
use super::catalogue::{FlowOutput, spec};
use super::citizens::Citizen;
use super::construction::ConstructionSite;
use super::heat::HeatOutput;
use super::labour::{Staffing, labour_factor};
use super::resources::{Inventory, ResourceKind};
use super::water::Watered;

/// Why a building is not running at nominal. `Nothing` is the healthy case.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Bound {
    #[default]
    Nothing,
    Labour,
    Power,
    Water,
    /// The recipe called for a material the yard could not fully supply.
    Input(ResourceKind),
}

/// The availability gate, computed once per producer per frame: `rate` is the
/// fraction of nominal the building can run at, `bound_by` names what is
/// holding it back. Attached at spawn to producers only, so a future producer
/// that forgets to earn one is skipped by both queries — it fails loudly with
/// no output rather than quietly running ungated.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct Gated {
    pub rate: f32,
    pub bound_by: Bound,
}

/// A kind produces if its recipe moves anything or it feeds a utility net.
fn is_producer(kind: super::buildings::BuildingKind) -> bool {
    let s = spec(kind);
    !s.recipe.inputs.is_empty() || !s.recipe.outputs.is_empty() || s.flow_output.is_some()
}

/// Producers gain their gate the moment they exist (the `attach_watered`
/// pattern).
fn attach_gated(
    add: On<Add, Building>,
    buildings: Query<(&Building, Has<Gated>)>,
    mut commands: Commands,
) {
    if let Ok((building, has)) = buildings.get(add.entity)
        && is_producer(building.kind)
        && !has
    {
        commands.entity(add.entity).insert(Gated::default());
    }
}

/// The availability half of ADR 0014's seam: it reads the staffing curve and
/// the power and water gates and names the scarcest. It never takes anything
/// from an inventory — the producer still burns its own coal.
fn availability(
    building: &Building,
    staffing: Option<&Staffing>,
    citizens: &Query<&Citizen>,
    powered: Option<&Powered>,
    watered: Option<&Watered>,
) -> Gated {
    // `run_factories` *required* `&Powered` in its query, so a kind that draws
    // power and carries no gate component was skipped entirely. Blocked, not
    // free — reproduced here rather than softened.
    if spec(building.kind).power.is_some() && !powered.is_some_and(|p| p.0) {
        return Gated {
            rate: 0.0,
            bound_by: Bound::Power,
        };
    }
    // `Option<&Watered>` the other way round: no component means the water
    // plugin is absent and water is not yet a requirement (the fiat fixture
    // path), so absence is permission.
    if watered.is_some_and(|w| !w.0) {
        return Gated {
            rate: 0.0,
            bound_by: Bound::Water,
        };
    }
    // A building with no `Staffing` ledger (headless fixtures without the
    // labour plugin) runs free. `labour_factor` is 1.0 for a kind that needs
    // no workers, which is why the heat plant's flat burn survives being run
    // through the same curve as everything else.
    let f = staffing.map_or(1.0, |s| labour_factor(s, building.kind, citizens));
    if f <= 0.0 {
        return Gated {
            rate: 0.0,
            bound_by: Bound::Labour,
        };
    }
    Gated {
        rate: f,
        bound_by: Bound::Nothing,
    }
}

/// One frame of a kind's recipe at the gated rate. Returns the flow it pours
/// onto its utility net, which is zero unless the recipe was fully satisfied.
///
/// The partial-take behaviour is `run_power_plants`' and is deliberate: a
/// plant short of fuel still burns what it has and generates nothing. Moving
/// the shortfall check before the take would leave that coal in the yard,
/// which is a different game.
fn run_recipe(building: &Building, gated: &mut Gated, inventory: &mut Inventory) -> f32 {
    let s = spec(building.kind);
    if gated.rate <= 0.0 {
        return 0.0;
    }
    let mut short = None;
    for &(resource, amount) in s.recipe.inputs {
        let demand = amount * gated.rate;
        let burned = inventory.take(resource, demand);
        if burned < demand * 0.999 {
            short = Some(resource);
        }
    }
    if let Some(resource) = short {
        gated.bound_by = Bound::Input(resource);
        return 0.0;
    }
    for &(resource, amount) in s.recipe.outputs {
        inventory.add(resource, amount * gated.rate);
    }
    match s.flow_output {
        Some(FlowOutput::Power(mw)) => mw * gated.rate,
        Some(FlowOutput::Heat(heat)) => heat * gated.rate,
        None => 0.0,
    }
}

/// Generation: the kinds that feed a utility net, before the solves read them.
#[allow(clippy::type_complexity)]
pub(crate) fn produce_flows(
    mut producers: Query<
        (
            &Building,
            &mut Inventory,
            &mut Gated,
            Option<&Staffing>,
            Option<&mut PowerOutput>,
            Option<&mut HeatOutput>,
        ),
        Without<ConstructionSite>,
    >,
    citizens: Query<&Citizen>,
) {
    for (building, mut inventory, mut gated, staffing, power, heat) in &mut producers {
        if spec(building.kind).flow_output.is_none() {
            continue;
        }
        *gated = availability(building, staffing, &citizens, None, None);
        let flow = run_recipe(building, &mut gated, &mut inventory);
        if let Some(mut out) = power {
            out.0 = flow;
        }
        if let Some(mut out) = heat {
            out.0 = flow;
        }
    }
}

/// Consumption: the kinds that fill a yard, after the grid and water solves,
/// so the gates they read are this frame's.
#[allow(clippy::type_complexity)]
pub(crate) fn produce_goods(
    mut producers: Query<
        (
            &Building,
            &mut Inventory,
            &mut Gated,
            Option<&Staffing>,
            Option<&Powered>,
            Option<&Watered>,
        ),
        Without<ConstructionSite>,
    >,
    citizens: Query<&Citizen>,
) {
    for (building, mut inventory, mut gated, staffing, powered, watered) in &mut producers {
        if spec(building.kind).flow_output.is_some() {
            continue;
        }
        *gated = availability(building, staffing, &citizens, powered, watered);
        run_recipe(building, &mut gated, &mut inventory);
    }
}

/// Registered by `BuildingSimPlugin`, not a plugin of its own: these systems
/// replace four that already lived there, so every existing fixture keeps
/// working untouched and there is no new plugin for a future app to forget.
pub(crate) fn register(app: &mut App) {
    app.add_observer(attach_gated).add_systems(
        super::stages::SimTick,
        (produce_flows, produce_goods)
            .chain()
            .in_set(super::stages::SimStage::ProductionAndUtilities),
    );
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin, PLANT_OUTPUT_MW,
    };
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin));
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

    fn place(app: &mut App, kind: BuildingKind, x: f32) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind,
                pos: Vec3::new(x, 0.0, 0.0),
            });
    }

    fn entity_of(app: &mut App, kind: BuildingKind) -> Entity {
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        q.iter(world)
            .find(|(_, b)| b.kind == kind)
            .map(|(e, _)| e)
            .expect("the building was placed")
    }

    /// ADR 0014's whole point: the scarcest factor used to be computed and then
    /// thrown away, so nothing could say *why* a building was idle. Each of
    /// these is a different arm of `availability`, and each names a constraint
    /// a player can act on.
    #[test]
    fn the_gate_names_what_is_binding() {
        let mut app = app();
        place(&mut app, BuildingKind::Mine, 0.0);
        place(&mut app, BuildingKind::PowerPlant, 60.0);
        place(&mut app, BuildingKind::Factory, 120.0);
        ticks(&mut app, 2);

        let mine = entity_of(&mut app, BuildingKind::Mine);
        assert_eq!(
            *app.world().get::<Gated>(mine).unwrap(),
            Gated {
                rate: 1.0,
                bound_by: Bound::Nothing
            },
            "an unstaffed fixture mine runs free and nothing binds it"
        );

        let plant = entity_of(&mut app, BuildingKind::PowerPlant);
        assert_eq!(
            app.world().get::<Gated>(plant).unwrap().bound_by,
            Bound::Input(ResourceKind::Coal),
            "an empty coal yard is what stops the plant, and the gate says so"
        );

        let factory = entity_of(&mut app, BuildingKind::Factory);
        assert_eq!(
            app.world().get::<Gated>(factory).unwrap().bound_by,
            Bound::Power,
            "an unwired factory is bound by power, not by its empty input list"
        );
    }

    /// The flow half of a recipe, grounded in behaviour rather than pinned:
    /// `PLANT_OUTPUT_MW` is a balance number and R4 will rebalance it, so the
    /// assertion has to read what the plant actually poured onto the grid.
    #[test]
    fn a_fuelled_plant_pours_its_nominal_flow_and_an_empty_one_pours_none() {
        let mut app = app();
        place(&mut app, BuildingKind::PowerPlant, 0.0);
        ticks(&mut app, 2);
        let plant = entity_of(&mut app, BuildingKind::PowerPlant);
        assert_eq!(
            app.world().get::<PowerOutput>(plant).unwrap().0,
            0.0,
            "no coal, no megawatts"
        );

        app.world_mut()
            .get_mut::<Inventory>(plant)
            .unwrap()
            .add(ResourceKind::Coal, 10.0);
        ticks(&mut app, 1);
        assert_eq!(
            app.world().get::<PowerOutput>(plant).unwrap().0,
            PLANT_OUTPUT_MW,
            "fuelled and fully staffed, the plant pours its nominal flow"
        );
        assert_eq!(
            app.world().get::<Gated>(plant).unwrap().bound_by,
            Bound::Nothing
        );
    }

    /// The structural guarantee ADR 0014 chose a component for: a producer that
    /// never earned a `Gated` is skipped by both queries, so a future kind
    /// added without one is silently *inert* rather than silently *ungated*.
    /// Failing closed is the whole reason this is a component and not a
    /// function every producer must remember to call.
    #[test]
    fn a_producer_stripped_of_its_gate_produces_nothing() {
        let mut app = app();
        place(&mut app, BuildingKind::Mine, 0.0);
        ticks(&mut app, 2);
        let mine = entity_of(&mut app, BuildingKind::Mine);
        let before = app
            .world()
            .get::<Inventory>(mine)
            .unwrap()
            .amount(ResourceKind::Coal);
        assert!(before > 0.0, "baseline: the gated mine was producing");

        app.world_mut().entity_mut(mine).remove::<Gated>();
        ticks(&mut app, 2);
        assert_eq!(
            app.world()
                .get::<Inventory>(mine)
                .unwrap()
                .amount(ResourceKind::Coal),
            before,
            "no gate, no output — the query requires one"
        );
    }
}
