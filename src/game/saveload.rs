//! Quicksave hotkeys (ticket #28): F5 saves, F9 loads. The keys only file a
//! request; the sim-side exclusive system performs it on the next tick
//! boundary, so the game layer never touches world state directly.

use bevy::prelude::*;

use crate::sim::save::{QUICKSAVE_PATH, SaveLoadRequests};

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        // Idempotent next to SaveSimPlugin; keeps capture/bench binaries that
        // add GamePlugins without the sim save plugin from panicking.
        app.init_resource::<SaveLoadRequests>()
            .add_systems(Update, quicksave_hotkeys);
    }
}

fn quicksave_hotkeys(keys: Res<ButtonInput<KeyCode>>, mut requests: ResMut<SaveLoadRequests>) {
    if keys.just_pressed(KeyCode::F5) {
        requests.save = Some(QUICKSAVE_PATH.into());
    }
    if keys.just_pressed(KeyCode::F9) {
        requests.load = Some(QUICKSAVE_PATH.into());
    }
}
