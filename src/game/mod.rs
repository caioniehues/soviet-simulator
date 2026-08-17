use bevy::prelude::*;

pub mod buildings;
pub mod camera;
pub mod citizens;
pub mod hud;
pub mod roads;
pub mod saveload;
pub mod tools;
pub mod transit;
pub mod vehicles;
pub mod wires;
pub mod world;
pub mod zoning;

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
            transit::TransitToolPlugin,
            wires::WireToolPlugin,
            saveload::SaveLoadPlugin,
            zoning::ZoningToolPlugin,
            hud::HudPlugin,
        ));
    }
}
