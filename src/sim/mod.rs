//! Simulation core: the SimTick schedule, its stage pipeline, and the clock
//! driver. See architecture/simulation-clock.md and ADRs 0001–0002.

pub mod buildings;
pub mod citizens;
pub mod dispatch;
pub mod clock;
pub mod commute;
pub mod households;
pub mod labour;
pub mod needs;
pub mod pathfinding;
pub mod resources;
pub mod roads;
pub mod save;
pub mod stages;
pub mod storage;
pub mod vehicles;
pub mod wires;

pub use clock::{FrameIndex, SimSpeed, TickIndex};
pub use stages::{PostSimEasing, SimStage, SimTick};

use bevy::prelude::*;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameIndex>()
            .init_resource::<TickIndex>()
            .init_resource::<SimSpeed>()
            .init_resource::<clock::SimPacing>();
        stages::configure(app);
        app.add_systems(Update, clock::drive_sim);
        app.configure_sets(Update, PostSimEasing.after(clock::drive_sim));
    }
}
