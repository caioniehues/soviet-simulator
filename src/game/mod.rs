use bevy::prelude::*;

pub mod buildings;
pub mod camera;
pub mod citizens;
pub mod hud;
pub mod roads;
pub mod saveload;
pub mod tools;
pub mod vehicles;
pub mod wires;
pub mod world;

/// Root plugin: every feature plugin under `src/game/` registers here,
/// keeping `lib.rs` the single App-wiring point.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            world::WorldPlugin,
            camera::CameraPlugin,
            tools::ToolsPlugin,
            roads::RoadToolPlugin,
            buildings::BuildingToolPlugin,
            citizens::CitizenViewPlugin,
            vehicles::VehicleToolPlugin,
            wires::WireToolPlugin,
            saveload::SaveLoadPlugin,
            hud::HudPlugin,
        ));
    }
}
