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

fn setup_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    use super::palette::{self, Mat, Role};
    // CC0 surfaces (ambientCG Ground048 / Asphalt010) through the palette
    // factory; ribbon UVs tile along the segment length, so uv_scale stays 1.
    commands.insert_resource(RoadMaterials {
        dirt: Mat::new(Role::DirtRoad)
            .textured(
                palette::load_tiled(&asset_server, "textures/road_dirt.png"),
                1.0,
            )
            .normal(palette::load_tiled(
                &asset_server,
                "textures/road_dirt_n.png",
            ))
            .roughness(1.0)
            .add_to(&mut materials),
        paved: Mat::new(Role::Asphalt)
            .textured(
                palette::load_tiled(&asset_server, "textures/asphalt.png"),
                1.0,
            )
            .normal(palette::load_tiled(&asset_server, "textures/asphalt_n.png"))
            .roughness(0.92)
            .add_to(&mut materials),
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
    // R rebuilds the most recently cut segment (paved pays gravel again)
    if keys.just_pressed(KeyCode::KeyR) {
        edits.0.push(RoadEdit::RebuildLast);
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
        gizmos.line(
            start + lift,
            end + lift,
            super::palette::Role::SignalOk.color(),
        );
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
        // Prefer the compiled centreline curve; fall back to the chord for a
        // segment that has not hit the compile pass yet.
        let points: Vec<Vec3> = if segment.curve.len() >= 2 {
            segment.curve.clone()
        } else {
            let (Ok(a), Ok(b)) = (nodes.get(segment.a), nodes.get(segment.b)) else {
                continue;
            };
            vec![a.pos, b.pos]
        };
        let mesh = ribbon(&points, segment.class.width());
        if let Ok(old) = existing.get(entity) {
            if meshes.insert(&old.0, mesh).is_err() {
                warn!("road mesh update failed for {entity:?}");
            }
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

/// Ribbon strip along the segment's centreline polyline (B4.5): one pair of
/// verts per curve point, side vectors from the local tangent (central
/// difference at interior points so joints between quads stay watertight).
fn ribbon(points: &[Vec3], width: f32) -> Mesh {
    let lift = Vec3::Y * 0.05;
    let half = width * 0.5;
    let n = points.len();
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n * 2);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n * 2);
    let mut walked = 0.0;
    for i in 0..n {
        let tangent = if i == 0 {
            points[1] - points[0]
        } else if i == n - 1 {
            points[n - 1] - points[n - 2]
        } else {
            points[i + 1] - points[i - 1]
        }
        .normalize_or_zero();
        let side = tangent.cross(Vec3::Y) * half;
        positions.push((points[i] - side + lift).to_array());
        positions.push((points[i] + side + lift).to_array());
        if i > 0 {
            walked += points[i - 1].distance(points[i]);
        }
        // v tiles along the ribbon so the surface texture doesn't stretch
        let v = walked / 16.0;
        uvs.push([0.0, v]);
        uvs.push([1.0, v]);
    }
    let normals = vec![[0.0, 1.0, 0.0]; n * 2];
    let mut indices: Vec<u32> = Vec::with_capacity((n - 1) * 6);
    for i in 0..(n as u32 - 1) {
        let (l0, r0, l1, r1) = (i * 2, i * 2 + 1, i * 2 + 2, i * 2 + 3);
        // counter-clockwise seen from +Y, or the ribbon is back-face culled
        indices.extend([l0, r0, r1, l0, r1, l1]);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}
