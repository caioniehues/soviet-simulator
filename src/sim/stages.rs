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

/// The declared total order among same-tick edit appliers (ADR 0013): each
/// applier's system(s) join exactly one variant here in place of ad-hoc
/// `.before`/`.after` edges against sibling appliers, so which applier sees
/// which other applier's writes this tick is one fact readable in one place
/// rather than an emergent property of registration order. All variants nest
/// inside `SimStage::ApplyCommands`, after the tick-opening flush — see
/// `configure` below.
///
/// The order follows the direction dependencies actually run: a building may
/// frontage-snap to a road placed this tick, storage-policy edits may adjust
/// bands on a building placed this tick, a wire may snap to a building, a
/// household may spawn into a dwelling, a vehicle may be bought at a depot,
/// and a transit line may need a stop placed this tick.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ApplierOrder {
    Zones,
    Roads,
    Buildings,
    Policy,
    Wires,
    Households,
    Vehicles,
    Transit,
}

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
    app.configure_sets(
        SimTick,
        (
            ApplierOrder::Zones,
            ApplierOrder::Roads,
            ApplierOrder::Buildings,
            ApplierOrder::Policy,
            ApplierOrder::Wires,
            ApplierOrder::Households,
            ApplierOrder::Vehicles,
            ApplierOrder::Transit,
        )
            .chain()
            .in_set(SimStage::ApplyCommands)
            .after(ApplyCommandsFlush),
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
    };
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSegment, RoadSimPlugin};

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, RoadSimPlugin, BuildingSimPlugin));
        a
    }

    fn tick(app: &mut App) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
        app.update();
    }

    /// R1 case (ADR 0013): with the total order declared, a road placed this
    /// tick has its segments compiled before `ApplierOrder::Buildings` runs —
    /// so a building placed the same tick, or any future frontage-snap gate
    /// on it, sees the road rather than racing it.
    #[derive(Resource, Default)]
    struct RoadSegmentsSeenByBuildings(usize);

    #[test]
    fn a_road_placed_this_tick_is_visible_to_the_buildings_applier_the_same_tick() {
        let mut app = app();
        app.init_resource::<RoadSegmentsSeenByBuildings>();
        app.add_systems(
            SimTick,
            (|segments: Query<&RoadSegment>, mut seen: ResMut<RoadSegmentsSeenByBuildings>| {
                seen.0 = segments.iter().count();
            })
            .in_set(ApplierOrder::Buildings),
        );
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(50.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Depot,
                pos: Vec3::new(25.0, 0.0, 20.0),
            });
        tick(&mut app);
        assert!(
            app.world().resource::<RoadSegmentsSeenByBuildings>().0 > 0,
            "the buildings applier ran without the same-tick road's segments compiled"
        );
    }
}
