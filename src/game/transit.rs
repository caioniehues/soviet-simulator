//! Bus-line tool (presentation side, B5.1): key 6, click bus stops in order,
//! right-click closes the loop and pushes a `CreateLine` through the sim's
//! edit queue. Line paths render as a tinted overlay in B5.5; here the draft
//! chain previews as gizmo lines between the picked shelters.

use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use crate::sim::buildings::{Building, BuildingKind};
use crate::sim::transit::{TransitEdit, TransitEditQueue, TransitLine};

/// Click-picking radius around a stop's centre, metres.
const PICK_RADIUS: f32 = 10.0;

/// Stops picked so far while a line draft is open.
#[derive(Resource, Default)]
struct LineDraft(Vec<Entity>);

pub struct TransitToolPlugin;

impl Plugin for TransitToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LineDraft>()
            .add_systems(Update, (drive_line_tool, preview_line_draft, overlay_lines));
    }
}

fn stop_under_cursor(pos: Vec3, buildings: &Query<(Entity, &Building)>) -> Option<Entity> {
    buildings
        .iter()
        .filter(|(_, b)| b.kind == BuildingKind::BusStop)
        .map(|(e, b)| (e, b.pos.distance_squared(pos)))
        .filter(|(_, d2)| *d2 <= PICK_RADIUS * PICK_RADIUS)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

fn drive_line_tool(
    mode: Res<ToolMode>,
    cursor: Res<GroundCursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    buildings: Query<(Entity, &Building)>,
    mut draft: ResMut<LineDraft>,
    mut edits: ResMut<TransitEditQueue>,
) {
    if *mode != ToolMode::TransitLine {
        draft.0.clear();
        return;
    }
    if buttons.just_pressed(MouseButton::Right) {
        if draft.0.len() >= 2 {
            info!("bus line: {} stops", draft.0.len());
            edits.0.push(TransitEdit::CreateLine {
                stops: std::mem::take(&mut draft.0),
            });
        } else {
            draft.0.clear();
        }
        return;
    }
    if buttons.just_pressed(MouseButton::Left)
        && let Some(pos) = cursor.0
        && let Some(stop) = stop_under_cursor(pos, &buildings)
        && draft.0.last() != Some(&stop)
        && !draft.0.contains(&stop)
    {
        draft.0.push(stop);
    }
}

fn preview_line_draft(
    draft: Res<LineDraft>,
    cursor: Res<GroundCursor>,
    buildings: Query<&Building>,
    mut gizmos: Gizmos,
) {
    let color = Color::srgb(0.25, 0.7, 0.9);
    let lift = Vec3::Y * 2.0;
    let anchors: Vec<Vec3> = draft
        .0
        .iter()
        .filter_map(|&e| buildings.get(e).ok().map(|b| b.pos + lift))
        .collect();
    for a in &anchors {
        gizmos.circle(
            Isometry3d::new(*a, Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            PICK_RADIUS * 0.6,
            color,
        );
    }
    for w in anchors.windows(2) {
        gizmos.line(w[0], w[1], color);
    }
    if let (Some(last), Some(pos)) = (anchors.last(), cursor.0) {
        gizmos.line(*last, pos + lift, color);
    }
}

/// Standing overlay: each live line traces its stop loop in its own hue so
/// the network reads at a glance (B5.5).
fn overlay_lines(lines: Query<&TransitLine>, buildings: Query<&Building>, mut gizmos: Gizmos) {
    for line in &lines {
        let hue = (line.id.0 as f32 * 71.0) % 360.0;
        let color = Color::hsl(hue, 0.55, 0.55).with_alpha(0.8);
        let lift = Vec3::Y * 1.2;
        let anchors: Vec<Vec3> = line
            .stops
            .iter()
            .filter_map(|&s| buildings.get(s).ok().map(|b| b.pos + lift))
            .collect();
        for a in &anchors {
            gizmos.circle(
                Isometry3d::new(*a, Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                4.0,
                color,
            );
        }
        for i in 0..anchors.len() {
            let next = (i + 1) % anchors.len();
            gizmos.line(anchors[i], anchors[next], color);
        }
    }
}
