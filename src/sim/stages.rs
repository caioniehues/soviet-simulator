use bevy::ecs::schedule::{ApplyDeferred, ScheduleBuildSettings, ScheduleLabel};
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

/// The tick-opening barrier inside [`SimStage::ApplyCommands`]: commands queued
/// since the last tick land here, before any edit-applier system runs. Edit
/// appliers order themselves `.after(ApplyCommandsFlush)`, so commands they
/// emit flush at the post-Commit barrier and take effect next tick.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ApplyCommandsFlush;

/// Public handle on the sim driver system in `Update`: presentation systems
/// that queue commands against sim-owned entities order themselves
/// `.before(SimDriver)` so a same-frame despawn can never race their apply.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SimDriver;

/// Render-side set in `Update`, ordered after the sim driver: eases rendered
/// transforms toward authoritative sim state. Presentation reads sim state and
/// never writes it (ADR 0003) — enforce by putting easing systems here only.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostSimEasing;

pub(super) fn configure(app: &mut App) {
    app.init_schedule(SimTick);
    // ADR 0002: the two named barriers below are the ONLY sync points in the
    // tick — auto-inserted apply_deferred would flush edit-applier commands
    // mid-tick and break "commands emitted during the tick land after Commit".
    app.edit_schedule(SimTick, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            auto_insert_apply_deferred: false,
            ..Default::default()
        });
    });
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
    app.configure_sets(SimTick, ApplyCommandsFlush.in_set(SimStage::ApplyCommands));
    app.add_systems(
        SimTick,
        (
            ApplyDeferred.in_set(ApplyCommandsFlush),
            ApplyDeferred
                .after(SimStage::CommitInventoriesAndCondition)
                .before(SimStage::AccountingAndCausalHistory),
        ),
    );
}
