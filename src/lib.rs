pub mod game;
pub mod sim;

use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Soviet Simulator".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            sim::SimPlugin,))
        .add_plugins((
            sim::roads::RoadSimPlugin,
            sim::buildings::BuildingSimPlugin,
            sim::storage::StorageSimPlugin,
            sim::households::HouseholdSimPlugin,
            sim::labour::LabourSimPlugin,
            sim::commute::CommuteSimPlugin,
            sim::needs::NeedsSimPlugin,
            sim::vehicles::VehicleSimPlugin,
            // DispatchSimPlugin auto-adds Pathfinding + Traffic.
            sim::dispatch::DispatchSimPlugin,
            sim::transit::TransitSimPlugin,
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
        ))
        .add_plugins(game::GamePlugin)
        .run();
}
