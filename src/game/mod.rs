use bevy::prelude::*;

/// Root plugin: every feature plugin under `src/game/` registers here,
/// keeping `lib.rs` the single App-wiring point.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
