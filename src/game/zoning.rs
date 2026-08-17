//! Zone-paint tool (B7.3, presentation side): key 7 (repeat cycles the
//! land use), two clicks span a district rect, `X` erases the district under
//! the cursor. Districts render as tinted gizmo rects — green for
//! residential, rust for industrial — planner intent made visible.

use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use crate::sim::zoning::{Zone, ZoneEdit, ZoneEditQueue, ZoneKind};

/// First corner of a district being painted.
#[derive(Resource, Default)]
struct ZoneDraft(Option<Vec3>);

pub struct ZoningToolPlugin;

impl Plugin for ZoningToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZoneDraft>()
            .add_systems(Update, (drive_zone_tool, overlay_zones));
    }
}

fn zone_color(kind: ZoneKind) -> Color {
    match kind {
        ZoneKind::Residential => Color::srgb(0.35, 0.75, 0.35),
        ZoneKind::Industrial => Color::srgb(0.75, 0.45, 0.20),
    }
}

fn drive_zone_tool(
    mode: Res<ToolMode>,
    cursor: Res<GroundCursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut draft: ResMut<ZoneDraft>,
    mut edits: ResMut<ZoneEditQueue>,
    mut gizmos: Gizmos,
) {
    let ToolMode::Zone(kind) = *mode else {
        draft.0 = None;
        return;
    };
    if keys.just_pressed(KeyCode::KeyX)
        && let Some(pos) = cursor.0
    {
        edits.0.push(ZoneEdit::Erase { pos });
        return;
    }
    if buttons.just_pressed(MouseButton::Right) {
        draft.0 = None;
        return;
    }
    if buttons.just_pressed(MouseButton::Left)
        && let Some(point) = cursor.0
    {
        match draft.0 {
            None => draft.0 = Some(point),
            Some(a) => {
                edits.0.push(ZoneEdit::Paint {
                    kind,
                    min: Vec2::new(a.x, a.z),
                    max: Vec2::new(point.x, point.z),
                });
                draft.0 = None;
            }
        }
    }
    if let (Some(a), Some(b)) = (draft.0, cursor.0) {
        rect_gizmo(
            &mut gizmos,
            Vec2::new(a.x, a.z),
            Vec2::new(b.x, b.z),
            zone_color(kind),
            0.6,
        );
    }
}

fn rect_gizmo(gizmos: &mut Gizmos, min: Vec2, max: Vec2, color: Color, lift: f32) {
    let (min, max) = (min.min(max), min.max(max));
    let y = lift;
    let corners = [
        Vec3::new(min.x, y, min.y),
        Vec3::new(max.x, y, min.y),
        Vec3::new(max.x, y, max.y),
        Vec3::new(min.x, y, max.y),
    ];
    for i in 0..4 {
        gizmos.line(corners[i], corners[(i + 1) % 4], color);
    }
    // A hatch line so the fill reads as area, not just an outline.
    gizmos.line(corners[0], corners[2], color.with_alpha(0.35));
}

fn overlay_zones(zones: Query<&Zone>, mut gizmos: Gizmos) {
    for zone in &zones {
        rect_gizmo(
            &mut gizmos,
            zone.min,
            zone.max,
            zone_color(zone.kind).with_alpha(0.65),
            0.45,
        );
    }
}
