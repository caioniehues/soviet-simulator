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
            sim::vehicles::VehicleSimPlugin,
        ))
        .add_plugins(game::GamePlugin)
        .run();
}
