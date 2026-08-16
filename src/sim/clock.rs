use bevy::prelude::*;

use super::stages::SimTick;

// Pinned starting constants (architecture/simulation-clock.md). Revisable; keep
// every derived rate expressed through these, never inlined elsewhere.

/// Scheduler passes per real second (sim rate at speed 1).
pub const SIM_HZ: f64 = 60.0;
pub const SECS_PER_PASS: f64 = 1.0 / SIM_HZ;
/// Sim frames per game day.
pub const FRAMES_PER_GAME_DAY: u32 = 600;
/// Game-seconds each sim frame represents (derived: 24h game day).
pub const GAME_SECONDS_PER_FRAME: f64 = 86_400.0 / FRAMES_PER_GAME_DAY as f64;
/// Per-frame simulation budget; band budgets are denominated in this.
pub const SIM_BUDGET_MS: f64 = 8.0;
/// Scheduler passes executed per render frame before excess real time is
/// dropped (anti-spiral: a slow render frame slows the sim instead of
/// snowballing the backlog).
pub const MAX_PASSES_PER_UPDATE: u32 = 3;

/// Advances once per `SimTick` run; the authority every frame-band phases on.
/// Wrap-safe: compare with `wrapping_sub`, never `<`.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct FrameIndex(pub u32);

/// Advances once per scheduler pass regardless of speed — including while
/// paused. Drives the speed-independent Housekeeping band (autosave must fire
/// on pause). Wrap-safe like `FrameIndex`.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TickIndex(pub u32);

/// Speed is a substep multiplier, never a time rescale: speed n runs the whole
/// `SimTick` schedule n times per scheduler pass, so every spec rate is
/// speed-invariant. Pause = 0 substeps.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SimSpeed {
    Paused,
    #[default]
    Normal,
    Double,
    Quad,
}

impl SimSpeed {
    pub fn substeps(self) -> u32 {
        match self {
            SimSpeed::Paused => 0,
            SimSpeed::Normal => 1,
            SimSpeed::Double => 2,
            SimSpeed::Quad => 4,
        }
    }
}

/// Real-time accumulator pacing scheduler passes at `SIM_HZ`.
#[derive(Resource, Default)]
pub(super) struct SimPacing {
    accumulated_secs: f64,
}

/// Exclusive driver, runs in `Update`: converts elapsed real time into
/// scheduler passes, each pass advancing `TickIndex` once and running
/// `SimTick` `substeps()` times with `FrameIndex` advancing per run.
pub(super) fn drive_sim(world: &mut World) {
    let delta = world.resource::<Time>().delta_secs_f64();
    world.resource_mut::<SimPacing>().accumulated_secs += delta;

    let mut passes = 0;
    while world.resource::<SimPacing>().accumulated_secs >= SECS_PER_PASS
        && passes < MAX_PASSES_PER_UPDATE
    {
        world.resource_mut::<SimPacing>().accumulated_secs -= SECS_PER_PASS;
        passes += 1;
        world.resource_mut::<TickIndex>().0 = world.resource::<TickIndex>().0.wrapping_add(1);
        for _ in 0..world.resource::<SimSpeed>().substeps() {
            world.resource_mut::<FrameIndex>().0 = world.resource::<FrameIndex>().0.wrapping_add(1);
            world.run_schedule(SimTick);
        }
    }
    if passes == MAX_PASSES_PER_UPDATE {
        // Backlogged: drop the excess instead of spiraling.
        world.resource_mut::<SimPacing>().accumulated_secs = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::*;
    use crate::sim::stages::SimStage;
    use std::time::Duration;

    /// Headless app with a hand-advanced clock (no TimePlugin).
    fn test_app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        app.add_plugins(SimPlugin);
        app
    }

    fn advance(app: &mut App, secs: f64) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f64(secs));
        app.update();
    }

    /// Run `n` updates of ~1/60s each (small epsilon avoids float shortfall).
    fn run_passes(app: &mut App, n: u32) {
        for _ in 0..n {
            advance(app, SECS_PER_PASS + 1e-9);
        }
    }

    #[test]
    fn speed_is_a_substep_multiplier() {
        let mut app = test_app();
        run_passes(&mut app, 60);
        assert_eq!(app.world().resource::<FrameIndex>().0, 60);
        assert_eq!(app.world().resource::<TickIndex>().0, 60);

        *app.world_mut().resource_mut::<SimSpeed>() = SimSpeed::Quad;
        run_passes(&mut app, 60);
        assert_eq!(app.world().resource::<FrameIndex>().0, 60 + 240);
        assert_eq!(app.world().resource::<TickIndex>().0, 120);
    }

    #[test]
    fn pause_freezes_frames_but_ticks_keep_advancing() {
        let mut app = test_app();
        *app.world_mut().resource_mut::<SimSpeed>() = SimSpeed::Paused;
        run_passes(&mut app, 30);
        assert_eq!(app.world().resource::<FrameIndex>().0, 0);
        assert_eq!(app.world().resource::<TickIndex>().0, 30);
    }

    #[test]
    fn slow_render_frame_clamps_instead_of_spiraling() {
        let mut app = test_app();
        advance(&mut app, 10.0); // one huge render frame
        assert_eq!(
            app.world().resource::<FrameIndex>().0,
            MAX_PASSES_PER_UPDATE
        );
        // backlog dropped: the next normal pass runs exactly one more
        run_passes(&mut app, 1);
        assert_eq!(
            app.world().resource::<FrameIndex>().0,
            MAX_PASSES_PER_UPDATE + 1
        );
    }

    #[test]
    fn stage_pipeline_runs_in_declared_order() {
        #[derive(Resource, Default)]
        struct Trace(Vec<&'static str>);

        fn mark(name: &'static str) -> impl FnMut(ResMut<Trace>) {
            move |mut trace| trace.0.push(name)
        }

        let mut app = test_app();
        app.init_resource::<Trace>();
        // Deliberately registered out of order; the set chain must sort them.
        app.add_systems(
            SimTick,
            (
                mark("derived").in_set(SimStage::BuildDerivedIndicesAndPresentation),
                mark("movement").in_set(SimStage::MovementAndTransfers),
                mark("calendar").in_set(SimStage::CalendarEdges),
                mark("commit").in_set(SimStage::CommitInventoriesAndCondition),
                mark("needs").in_set(SimStage::NeedsAndServiceDemand),
            ),
        );
        run_passes(&mut app, 1);
        assert_eq!(
            app.world().resource::<Trace>().0,
            vec!["calendar", "needs", "movement", "commit", "derived"]
        );
    }
}
