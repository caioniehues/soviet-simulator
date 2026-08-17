pub mod game;
pub mod sim;

use bevy::prelude::*;

/// Every sim plugin the game runs, in one place so `run()` and the wiring test
/// register the identical set. Several plugins auto-add their own dependencies
/// via `is_plugin_added`, so a plugin listed here that an earlier one already
/// pulled in is a panic, not a no-op — see the test below.
///
/// ADR 0012 replaces this with a `SimPlugins` group; until then this function is
/// the single owner of the list.
pub fn add_sim_plugins(app: &mut App) {
    app.add_plugins((sim::SimPlugin,))
        .add_plugins((
            sim::roads::RoadSimPlugin,
            sim::buildings::BuildingSimPlugin,
            sim::storage::StorageSimPlugin,
            sim::households::HouseholdSimPlugin,
            sim::labour::LabourSimPlugin,
            // CommuteSimPlugin auto-adds Pathfinding + Transit.
            sim::commute::CommuteSimPlugin,
            sim::needs::NeedsSimPlugin,
            sim::vehicles::VehicleSimPlugin,
            // DispatchSimPlugin auto-adds Pathfinding + Traffic.
            sim::dispatch::DispatchSimPlugin,
        ))
        .add_plugins((
            sim::construction::ConstructionSimPlugin,
            sim::zoning::ZoningSimPlugin,
            sim::water::WaterSimPlugin,
            sim::heat::HeatSimPlugin,
            sim::plan::PlanSimPlugin,
            sim::customs::CustomsSimPlugin,
            sim::wires::WireSimPlugin,
            sim::save::SaveSimPlugin,
        ));
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
    add_sim_plugins(&mut app);
    app.add_plugins(game::GamePlugin).run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression: `TransitSimPlugin` was listed explicitly *after*
    /// `CommuteSimPlugin`, which auto-adds it — so the real binary panicked on
    /// startup with "plugin was already added in application" and never reached
    /// a window. Nothing caught it because no test and none of the 18 bin
    /// targets ever built the shipped plugin set; every one hand-assembles a
    /// subset. This test is the thing that was missing.
    #[test]
    fn the_shipped_sim_plugin_set_registers_without_duplicates() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        add_sim_plugins(&mut app);
    }
}
