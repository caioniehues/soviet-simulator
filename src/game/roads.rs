//! Road tool (presentation side): click chains push `RoadEdit`s into the sim's
//! queue — the graph itself lives in `sim::roads` and mutates only at the
//! ApplyCommands barrier. Rendering reads compiled segments and keeps a ribbon
//! mesh per segment.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use super::tools::{GroundCursor, ToolMode};
use crate::sim::roads::{RoadEdit, RoadEditQueue, RoadNode, RoadSegment};

/// Last laid point while a click chain is open.
#[derive(Resource, Default)]
struct ChainStart(Option<Vec3>);

#[derive(Resource)]
struct RoadMaterials {
    dirt: Handle<StandardMaterial>,
    paved: Handle<StandardMaterial>,
}

pub struct RoadToolPlugin;

impl Plugin for RoadToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChainStart>()
            .add_systems(Startup, setup_materials)
            .add_systems(
                Update,
                (drive_road_tool, preview_chain, sync_segment_meshes),
            );
    }
}

fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(RoadMaterials {
        dirt: materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.36, 0.26),
            perceptual_roughness: 1.0,
            ..default()
        }),
        paved: materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.28, 0.30),
            perceptual_roughness: 0.9,
            ..default()
        }),
    });
}

fn drive_road_tool(
    mode: Res<ToolMode>,
    cursor: Res<GroundCursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut chain: ResMut<ChainStart>,
    mut edits: ResMut<RoadEditQueue>,
) {
    let ToolMode::Road(class) = *mode else {
        chain.0 = None;
        return;
    };
    // X cuts the segment under the cursor regardless of chain state
    if keys.just_pressed(KeyCode::KeyX)
        && let Some(pos) = cursor.0
    {
        edits.0.push(RoadEdit::RemoveNear { pos });
        return;
    }
    if buttons.just_pressed(MouseButton::Right) {
        chain.0 = None;
        return;
    }
    if buttons.just_pressed(MouseButton::Left)
        && let Some(point) = cursor.0
    {
        match chain.0 {
            None => chain.0 = Some(point),
            Some(start) => {
                if start.distance(point) > 1.0 {
                    edits.0.push(RoadEdit::Place {
                        from: start,
                        to: point,
                        class,
                    });
                    chain.0 = Some(point);
                }
            }
        }
    }
}

fn preview_chain(chain: Res<ChainStart>, cursor: Res<GroundCursor>, mut gizmos: Gizmos) {
    if let (Some(start), Some(end)) = (chain.0, cursor.0) {
        let lift = Vec3::Y * 0.3;
        gizmos.line(start + lift, end + lift, Color::srgb(0.95, 0.85, 0.2));
    }
}

/// One flat ribbon quad per compiled segment, refreshed whenever the segment
/// recompiles (Changed<RoadSegment> — the compile pass rewrites it in place).
fn sync_segment_meshes(
    mut commands: Commands,
    changed: Query<(Entity, &RoadSegment), Changed<RoadSegment>>,
    nodes: Query<&RoadNode>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<RoadMaterials>,
    existing: Query<&Mesh3d>,
) {
    for (entity, segment) in &changed {
        let (Ok(a), Ok(b)) = (nodes.get(segment.a), nodes.get(segment.b)) else {
            continue;
        };
        let mesh = ribbon(a.pos, b.pos, segment.class.width());
        if let Ok(old) = existing.get(entity) {
            meshes.insert(&old.0, mesh);
        } else {
            let material = match segment.class {
                crate::sim::roads::RoadClass::Dirt => materials.dirt.clone(),
                crate::sim::roads::RoadClass::Paved => materials.paved.clone(),
            };
            commands.entity(entity).insert((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(material),
                Transform::IDENTITY,
                Name::new("RoadSegmentMesh"),
            ));
        }
    }
}

fn ribbon(a: Vec3, b: Vec3, width: f32) -> Mesh {
    let dir = (b - a).normalize_or_zero();
    let side = dir.cross(Vec3::Y) * (width * 0.5);
    let lift = Vec3::Y * 0.05;
    let corners = [
        a - side + lift,
        a + side + lift,
        b + side + lift,
        b - side + lift,
    ];
    let positions: Vec<[f32; 3]> = corners.iter().map(|v| v.to_array()).collect();
    let normals = vec![[0.0, 1.0, 0.0]; 4];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(vec![0, 2, 1, 0, 3, 2]))
}
