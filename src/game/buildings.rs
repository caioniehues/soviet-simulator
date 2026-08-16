//! Building tool + rendering (presentation side). Placement goes through the
//! sim's edit queue; boxes are color-coded primitives per the M1 charter (Q12).

use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use crate::sim::buildings::{Building, BuildingEdit, BuildingEditQueue, BuildingKind};

fn kind_color(kind: BuildingKind) -> Color {
    match kind {
        BuildingKind::Mine => Color::srgb(0.25, 0.20, 0.16),
        BuildingKind::Quarry => Color::srgb(0.65, 0.58, 0.45),
        BuildingKind::PowerPlant => Color::srgb(0.55, 0.22, 0.18),
        BuildingKind::Factory => Color::srgb(0.35, 0.42, 0.52),
    }
}

pub(crate) fn kind_height(kind: BuildingKind) -> f32 {
    match kind {
        BuildingKind::Mine => 6.0,
        BuildingKind::Quarry => 3.0,
        BuildingKind::PowerPlant => 12.0,
        BuildingKind::Factory => 9.0,
    }
}

pub struct BuildingToolPlugin;

impl Plugin for BuildingToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (drive_building_tool, preview_footprint, sync_building_meshes),
        );
    }
}

fn drive_building_tool(
    mode: Res<ToolMode>,
    cursor: Res<GroundCursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut edits: ResMut<BuildingEditQueue>,
) {
    let ToolMode::Building(kind) = *mode else {
        return;
    };
    if buttons.just_pressed(MouseButton::Left)
        && let Some(pos) = cursor.0
    {
        edits.0.push(BuildingEdit::Place { kind, pos });
    }
}

fn preview_footprint(mode: Res<ToolMode>, cursor: Res<GroundCursor>, mut gizmos: Gizmos) {
    let ToolMode::Building(kind) = *mode else {
        return;
    };
    let Some(pos) = cursor.0 else { return };
    let fp = kind.footprint();
    gizmos.rect(
        Isometry3d::new(
            pos + Vec3::Y * 0.1,
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        fp,
        kind_color(kind).with_alpha(0.9),
    );
}

fn sync_building_meshes(
    mut commands: Commands,
    added: Query<(Entity, &Building), Added<Building>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, building) in &added {
        let fp = building.kind.footprint();
        let height = kind_height(building.kind);
        commands.entity(entity).insert((
            Mesh3d(meshes.add(Cuboid::new(fp.x, height, fp.y))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: kind_color(building.kind),
                perceptual_roughness: 0.85,
                ..default()
            })),
            Transform::from_translation(building.pos + Vec3::Y * (height * 0.5)),
            Name::new(format!("{:?}", building.kind)),
        ));
    }
}
