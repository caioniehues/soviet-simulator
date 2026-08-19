pub mod game;
pub mod sim;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// Every sim plugin the game runs, as a `PluginGroup` (ADR 0012) so a binary
/// states what it *excludes* — `SimPlugins.build().disable::<HeatSimPlugin>()`
/// — rather than hand-typing an inclusion list that rots the moment a rung
/// adds a plugin here. `run()` and the wiring test below build the identical
/// group, so the shipped game and its test both see this exact order.
///
/// Several plugins auto-add their own dependencies via `is_plugin_added`
/// (`CommuteSimPlugin` pulls in `PathfindingSimPlugin` + `TransitSimPlugin`;
/// `DispatchSimPlugin` pulls in `PathfindingSimPlugin` + `TrafficSimPlugin`),
/// so a plugin listed here that an earlier one already pulled in would be a
/// duplicate `add` — none are, but a binary that disables `CommuteSimPlugin`
/// or `DispatchSimPlugin` and still wants their auto-added dependency must
/// add it back explicitly.
pub struct SimPlugins;

impl PluginGroup for SimPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(sim::SimPlugin)
            .add(sim::roads::RoadSimPlugin)
            .add(sim::buildings::BuildingSimPlugin)
            .add(sim::storage::StorageSimPlugin)
            .add(sim::households::HouseholdSimPlugin)
            .add(sim::labour::LabourSimPlugin)
            .add(sim::commute::CommuteSimPlugin)
            .add(sim::needs::NeedsSimPlugin)
            .add(sim::vehicles::VehicleSimPlugin)
            .add(sim::dispatch::DispatchSimPlugin)
            .add(sim::construction::ConstructionSimPlugin)
            .add(sim::zoning::ZoningSimPlugin)
            .add(sim::water::WaterSimPlugin)
            .add(sim::heat::HeatSimPlugin)
            .add(sim::plan::PlanSimPlugin)
            .add(sim::customs::CustomsSimPlugin)
            .add(sim::wires::WireSimPlugin)
            .add(sim::save::SaveSimPlugin)
    }
}

pub fn run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Soviet Simulator".into(),
            ..default()
        }),
        ..default()
    }));
    app.add_plugins(SimPlugins).add_plugins(game::GamePlugins);
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression: `TransitSimPlugin` was listed explicitly *after*
    /// `CommuteSimPlugin`, which auto-adds it — so the real binary panicked on
    /// startup with "plugin was already added in application" and never
    /// reached a window. Nothing caught it because no test and none of the
    /// bin targets ever built the shipped plugin set; every one hand-assembled
    /// a subset. This test is the thing that was missing.
    #[test]
    fn the_shipped_sim_plugin_set_registers_without_duplicates() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        app.add_plugins(SimPlugins);
    }

    /// The play path's resources — one from each stage the group wires
    /// together — exist after building the group alone, with no bin-specific
    /// setup. If this ever needs a bin-specific workaround to pass, the group
    /// has stopped being the single source of truth it's meant to be.
    #[test]
    fn sim_plugins_group_provides_the_play_path_resources() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        app.add_plugins(SimPlugins);
        let world = app.world();
        assert!(world.contains_resource::<sim::plan::Treasury>());
        assert!(world.contains_resource::<sim::plan::StatePlan>());
        assert!(world.contains_resource::<sim::plan::AllocationFeedback>());
        assert!(world.contains_resource::<sim::zoning::ZoningFeedback>());
        assert!(world.contains_resource::<sim::buildings::BuildingEditQueue>());
        assert!(world.contains_resource::<sim::save::SaveLoadRequests>());
    }

    /// A disabled plugin is genuinely absent, not silently re-added by
    /// another plugin's own dependency guard — the property `.disable::<T>()`
    /// promises a bench.
    #[test]
    fn a_disabled_sim_plugin_is_genuinely_absent() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        app.add_plugins(SimPlugins.build().disable::<sim::plan::PlanSimPlugin>());
        assert!(!app.world().contains_resource::<sim::plan::StatePlan>());
    }
}
