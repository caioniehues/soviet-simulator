use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod art;
pub mod buildings;
pub mod camera;
pub mod citizens;
pub mod hud;
pub mod juice;
pub mod notify;
pub mod palette;
pub mod roads;
pub mod saveload;
pub mod toolbar;
pub mod tools;
pub mod transit;
pub mod ui;
pub mod vehicles;
pub mod wires;
pub mod world;
pub mod zoning;

/// Every presentation plugin under `src/game/`, as a `PluginGroup` (ADR
/// 0012) — the same inversion as `SimPlugins`: a binary excludes what it
/// doesn't want rather than hand-typing what it does. This also removes the
/// 15-element `add_plugins` tuple ceiling that used to need a second call.
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ui::UiPlugin)
            .add(notify::NotifyPlugin)
            .add(world::WorldPlugin)
            .add(camera::CameraPlugin)
            .add(tools::ToolsPlugin)
            .add(toolbar::ToolbarPlugin)
            .add(roads::RoadToolPlugin)
            .add(buildings::BuildingToolPlugin)
            .add(citizens::CitizenViewPlugin)
            .add(vehicles::VehicleToolPlugin)
            .add(transit::TransitToolPlugin)
            .add(wires::WireToolPlugin)
            .add(saveload::SaveLoadPlugin)
            .add(zoning::ZoningToolPlugin)
            .add(hud::HudPlugin)
            .add(juice::JuicePlugin)
    }
}
