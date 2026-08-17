//! First juice (R0.5, #115): the feedback that makes a click land.
//!
//! Scope discipline — this is *first* juice, not the ambient-life pass (R7).
//! Four things only, each answering a specific complaint from the playtest:
//!
//! - **Hover and selection outlines.** The selection marker was a flat red
//!   ellipse on the ground, which reads as a decal rather than as "this
//!   building is chosen", and nothing at all happened on hover — the player
//!   could not tell what a click was about to hit.
//! - **A placement pop.** A building appeared at full size in one frame, so a
//!   successful placement felt like nothing.
//! - **A refusal shake.** A refusal was text and only text. It now moves.
//! - **Smoke that means something.** Chimneys puffed at a fixed rate whether
//!   the plant was at full output or barely turning over.

use bevy::prelude::*;
use bevy_mod_outline::{InheritOutline, OutlinePlugin, OutlineVolume};

use super::hud::Selected;
use super::palette::Role;
use super::tools::GroundCursor;
use crate::sim::buildings::Building;
use crate::sim::roads::RoadBuildFeedback;

/// Outline widths in screen pixels. Hover is a whisper; selection is a
/// statement.
const HOVER_WIDTH: f32 = 2.0;
const SELECT_WIDTH: f32 = 4.0;

/// How long a newly placed building takes to settle into its full size.
const POP_SECONDS: f32 = 0.28;
/// How long a refusal shakes the ground cursor.
const SHAKE_SECONDS: f32 = 0.34;
const SHAKE_AMPLITUDE: f32 = 1.6;

/// The building under the cursor this frame, if any. Separate from
/// [`Selected`] so hover can pre-highlight what a click would pick.
#[derive(Resource, Default)]
pub struct Hovered(pub Option<Entity>);

/// A building still growing into place.
#[derive(Component)]
struct PlacementPop {
    age: f32,
}

/// A live refusal, counted down by [`shake_cursor`].
#[derive(Resource, Default)]
pub struct RefusalShake(pub f32);

pub struct JuicePlugin;

impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        // Jump flood over vertex extrusion: our silhouettes are procedural
        // boxes and cones with hard edges and no outline normals, which is
        // precisely the case extrusion handles badly.
        app.add_plugins(OutlinePlugin::JUMP_FLOOD)
            .init_resource::<Hovered>()
            .init_resource::<RefusalShake>()
            .add_systems(
                Update,
                (
                    track_hover,
                    dress_outlines,
                    inherit_outlines,
                    paint_outlines,
                    start_placement_pop,
                    animate_placement_pop,
                    catch_refusals,
                    shake_cursor,
                ),
            );
    }
}

/// What the cursor is over — the same reach test the inspect tool uses to
/// decide what a click picks, so the pre-highlight never lies.
fn track_hover(
    cursor: Res<GroundCursor>,
    buildings: Query<(Entity, &Building)>,
    mut hovered: ResMut<Hovered>,
) {
    let next = cursor.0.and_then(|pos| {
        buildings
            .iter()
            .map(|(e, b)| {
                let reach = b.kind.footprint().length() * 0.5 + 2.0;
                (e, b.pos.distance_squared(pos), reach)
            })
            .filter(|(_, d2, reach)| *d2 <= reach * reach)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, ..)| e)
    });
    if hovered.0 != next {
        hovered.0 = next;
    }
}

/// Give each new building an outline volume for its parts to inherit.
fn dress_outlines(mut commands: Commands, added: Query<Entity, Added<Building>>) {
    for entity in &added {
        commands.entity(entity).insert(OutlineVolume {
            visible: false,
            colour: Color::WHITE,
            width: HOVER_WIDTH,
        });
    }
}

/// Mark each building part to inherit its root's outline.
///
/// Two traps here, both found by capturing rather than by reading code:
///
/// - The parts are spawned by `buildings::sync_building_meshes` through
///   deferred commands, so on the frame `Added<Building>` fires the root has
///   no `Children` yet. Keying on `Added<Mesh3d>` instead catches each part as
///   it actually arrives.
/// - `PropagateOutline` would be tidier, but it also reaches the yard pad —
///   a ground plane wider than the building — and drew a square on the field
///   instead of the silhouette. `StopPropagateOutline` does not help: it halts
///   propagation *below* an entity, not to it.
fn inherit_outlines(
    mut commands: Commands,
    parts: Query<
        (Entity, &ChildOf),
        (
            Added<Mesh3d>,
            Without<crate::game::buildings::YardPad>,
            Without<Building>,
        ),
    >,
    buildings: Query<(), With<Building>>,
) {
    for (entity, parent) in &parts {
        if buildings.get(parent.parent()).is_ok() {
            commands.entity(entity).insert(InheritOutline);
        }
    }
}

fn paint_outlines(
    hovered: Res<Hovered>,
    selected: Res<Selected>,
    mut outlines: Query<(Entity, &mut OutlineVolume)>,
) {
    if !hovered.is_changed() && !selected.is_changed() {
        return;
    }
    for (entity, mut outline) in &mut outlines {
        // Selection wins over hover on the same building: the louder state is
        // the one the player has committed to.
        let (visible, colour, width) = if selected.0 == Some(entity) {
            (true, Color::srgb(0.96, 0.94, 0.86), SELECT_WIDTH)
        } else if hovered.0 == Some(entity) {
            (true, Role::SignalOk.color(), HOVER_WIDTH)
        } else {
            (false, outline.colour, outline.width)
        };
        if outline.visible != visible || outline.width != width {
            outline.visible = visible;
            outline.colour = colour;
            outline.width = width;
        }
    }
}

/// A building that has just been placed starts squat and springs up. Only new
/// buildings pop; construction sites already have their own rise animation,
/// and popping them too would fight it.
fn start_placement_pop(
    mut commands: Commands,
    added: Query<
        Entity,
        (
            Added<Building>,
            Without<crate::sim::construction::ConstructionSite>,
        ),
    >,
) {
    for entity in &added {
        commands.entity(entity).insert(PlacementPop { age: 0.0 });
    }
}

fn animate_placement_pop(
    mut commands: Commands,
    time: Res<Time>,
    mut popping: Query<(Entity, &mut PlacementPop, &mut Transform)>,
) {
    for (entity, mut pop, mut transform) in &mut popping {
        pop.age += time.delta_secs();
        let t = (pop.age / POP_SECONDS).clamp(0.0, 1.0);
        if t >= 1.0 {
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<PlacementPop>();
            continue;
        }
        // Overshoot slightly past 1 and settle back: the difference between
        // "appeared" and "landed".
        let eased = 1.0 - (1.0 - t).powi(3);
        let overshoot = 1.0 + 0.16 * (t * std::f32::consts::PI).sin();
        transform.scale = Vec3::new(
            eased * overshoot,
            (0.55 + 0.45 * eased) * overshoot,
            eased * overshoot,
        );
    }
}

/// Any refusal the player can see starts a shake. The road-paving shortfall
/// is the one that fires most; the treasury refusal joins it through the same
/// resource.
fn catch_refusals(
    road_feedback: Res<RoadBuildFeedback>,
    zone_feedback: Res<crate::sim::zoning::ZoningFeedback>,
    mut shake: ResMut<RefusalShake>,
) {
    let refused = (road_feedback.is_changed() && road_feedback.0.is_some())
        || (zone_feedback.is_changed() && zone_feedback.0.is_some());
    if refused {
        shake.0 = SHAKE_SECONDS;
    }
}

/// The cursor marker is what the player was looking at when the click was
/// refused, so it is what shakes. A decaying sine, not a random jitter — a
/// jitter reads as a rendering fault.
fn shake_cursor(
    time: Res<Time>,
    mut shake: ResMut<RefusalShake>,
    mut marker: Query<&mut Transform, With<super::tools::CursorMarker>>,
) {
    if shake.0 <= 0.0 {
        return;
    }
    shake.0 = (shake.0 - time.delta_secs()).max(0.0);
    let Ok(mut transform) = marker.single_mut() else {
        return;
    };
    let decay = shake.0 / SHAKE_SECONDS;
    let offset = (shake.0 * 58.0).sin() * SHAKE_AMPLITUDE * decay;
    transform.translation.x += offset;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pop_starts_squat_and_ends_exactly_at_one() {
        // A pop that does not land on 1.0 leaves every building permanently
        // the wrong size — the failure mode worth a test.
        let scale_at = |t: f32| {
            let eased = 1.0 - (1.0f32 - t).powi(3);
            let overshoot = 1.0 + 0.16 * (t * std::f32::consts::PI).sin();
            Vec3::new(
                eased * overshoot,
                (0.55 + 0.45 * eased) * overshoot,
                eased * overshoot,
            )
        };
        assert!(scale_at(0.0).y < 0.6, "the pop should start squat");
        let end = scale_at(1.0);
        assert!((end.x - 1.0).abs() < 1e-3, "x settled at {}", end.x);
        assert!((end.y - 1.0).abs() < 1e-3, "y settled at {}", end.y);
        // And it must actually overshoot in between, or it is just a fade.
        assert!(scale_at(0.5).x > 1.0);
    }
}
