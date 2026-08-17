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
            sim::SimPlugin,
            sim::roads::RoadSimPlugin,
            sim::buildings::BuildingSimPlugin,
            sim::storage::StorageSimPlugin,
            sim::households::HouseholdSimPlugin,
            sim::labour::LabourSimPlugin,
            sim::commute::CommuteSimPlugin,
            sim::needs::NeedsSimPlugin,
            sim::vehicles::VehicleSimPlugin,
            sim::wires::WireSimPlugin,
            sim::save::SaveSimPlugin,
        ))
        .add_plugins(game::GamePlugin)
        .run();
}
