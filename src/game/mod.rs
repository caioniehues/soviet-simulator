use bevy::prelude::*;

pub mod buildings;
pub mod camera;
pub mod roads;
pub mod tools;
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
        ));
    }
}
