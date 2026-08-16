use bevy::ecs::schedule::{ApplyDeferred, ScheduleLabel};
use bevy::prelude::*;

/// The simulation schedule. Runs zero or more times per render frame under the
/// control of the clock driver (`clock::drive_sim`) — never directly by Bevy.
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SimTick;

/// The per-tick stage pipeline (architecture/ecs.md § System ownership and ordering).
/// Every simulation system lives in exactly one of these sets; the chain below is
/// the only source of cross-band happens-before edges.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimStage {
    ApplyCommands,
    CalendarEdges,
    NeedsAndServiceDemand,
    AllocationAndDispatch,
    Routing,
    MovementAndTransfers,
    ProductionAndUtilities,
    CommitInventoriesAndCondition,
    AccountingAndCausalHistory,
    BuildDerivedIndicesAndPresentation,
}

/// Render-side set in `Update`, ordered after the sim driver: eases rendered
/// transforms toward authoritative sim state. Presentation reads sim state and
/// never writes it (ADR 0003) — enforce by putting easing systems here only.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostSimEasing;

pub(super) fn configure(app: &mut App) {
    app.init_schedule(SimTick);
    app.configure_sets(
        SimTick,
        (
            SimStage::ApplyCommands,
            SimStage::CalendarEdges,
            SimStage::NeedsAndServiceDemand,
            SimStage::AllocationAndDispatch,
            SimStage::Routing,
            SimStage::MovementAndTransfers,
            SimStage::ProductionAndUtilities,
            SimStage::CommitInventoriesAndCondition,
            SimStage::AccountingAndCausalHistory,
            SimStage::BuildDerivedIndicesAndPresentation,
        )
            .chain(),
    );
    // The only two structural-change barriers in the tick (ADR 0002): commands
    // queued since the last tick land in ApplyCommands; commands emitted during
    // the tick land after Commit. Auto sync points are not relied on.
    app.add_systems(
        SimTick,
        (
            ApplyDeferred.in_set(SimStage::ApplyCommands),
            ApplyDeferred
                .after(SimStage::CommitInventoriesAndCondition)
                .before(SimStage::AccountingAndCausalHistory),
        ),
    );
}
