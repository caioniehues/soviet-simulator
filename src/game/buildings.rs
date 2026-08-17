//! Building tool + rendering (presentation side). Placement goes through the
//! sim's edit queue. P1 "First Light": each kind gets a silhouette per
//! docs/art-direction.md — multi-part procedural meshes over a worn yard pad,
//! with chimney smoke on powered plants. Sim entities stay untouched; all
//! visual parts are children of the building entity.

use bevy::math::Affine2;
use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use super::world::load_tiled;
use crate::sim::buildings::{Building, BuildingEdit, BuildingEditQueue, BuildingKind, PowerOutput};

fn kind_color(kind: BuildingKind) -> Color {
    match kind {
        BuildingKind::Mine => Color::srgb(0.43, 0.29, 0.23),
        BuildingKind::Quarry => Color::srgb(0.60, 0.55, 0.45),
        BuildingKind::PowerPlant => Color::srgb(0.60, 0.59, 0.55),
        BuildingKind::Factory => Color::srgb(0.55, 0.52, 0.48),
        BuildingKind::Dwelling => Color::srgb(0.58, 0.56, 0.52),
        BuildingKind::Warehouse => Color::srgb(0.52, 0.47, 0.40),
        BuildingKind::Depot => Color::srgb(0.48, 0.50, 0.46),
        BuildingKind::BusStop => Color::srgb(0.55, 0.58, 0.52),
    }
}

pub(crate) fn kind_height(kind: BuildingKind) -> f32 {
    match kind {
        BuildingKind::Mine => 6.0,
        BuildingKind::Quarry => 3.0,
        BuildingKind::PowerPlant => 12.0,
        BuildingKind::Factory => 9.0,
        BuildingKind::Dwelling => 11.0,
        BuildingKind::Warehouse => 7.0,
        BuildingKind::Depot => 6.0,
        BuildingKind::BusStop => 3.0,
    }
}

/// Shared palette materials (art-direction.md § Material rules).
#[derive(Resource)]
struct BuildingMaterials {
    concrete: Handle<StandardMaterial>,
    brick: Handle<StandardMaterial>,
    rust: Handle<StandardMaterial>,
    timber: Handle<StandardMaterial>,
    coal: Handle<StandardMaterial>,
    gravel: Handle<StandardMaterial>,
    yard: Handle<StandardMaterial>,
    smoke: [Handle<StandardMaterial>; 3],
}

#[derive(Component)]
struct SmokeStack {
    /// World offset of the chimney tip relative to the building origin.
    tips: [Vec3; 2],
    cooldown: f32,
}

#[derive(Component)]
struct SmokePuff {
    age: f32,
    drift: Vec3,
}

pub struct BuildingToolPlugin;

impl Plugin for BuildingToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_materials).add_systems(
            Update,
            (
                drive_building_tool,
                preview_footprint,
                sync_building_meshes,
                emit_smoke,
                animate_smoke,
            ),
        );
    }
}

fn setup_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let opaque = |color: Color, rough: f32| StandardMaterial {
        base_color: color,
        perceptual_roughness: rough,
        ..default()
    };
    let smoke = |alpha: f32| StandardMaterial {
        base_color: Color::srgba(0.72, 0.72, 0.74, alpha),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    };
    commands.insert_resource(BuildingMaterials {
        concrete: materials.add(opaque(Color::srgb(0.48, 0.47, 0.44), 0.95)),
        brick: materials.add(opaque(Color::srgb(0.34, 0.21, 0.16), 0.95)),
        rust: materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.25, 0.15),
            perceptual_roughness: 0.8,
            metallic: 0.35,
            ..default()
        }),
        timber: materials.add(opaque(Color::srgb(0.37, 0.29, 0.20), 1.0)),
        coal: materials.add(opaque(Color::srgb(0.10, 0.10, 0.11), 1.0)),
        gravel: materials.add(opaque(Color::srgb(0.62, 0.60, 0.56), 1.0)),
        yard: materials.add(StandardMaterial {
            base_color: Color::srgb(0.64, 0.58, 0.48),
            base_color_texture: Some(load_tiled(&asset_server, "textures/dirt.png")),
            perceptual_roughness: 1.0,
            uv_transform: Affine2::from_scale(Vec2::splat(2.5)),
            ..default()
        }),
        smoke: [
            materials.add(smoke(0.34)),
            materials.add(smoke(0.18)),
            materials.add(smoke(0.07)),
        ],
    });
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

/// A child part: mesh, material, local transform.
struct Part {
    mesh: Mesh,
    material: Handle<StandardMaterial>,
    transform: Transform,
}

fn boxed(m: &Handle<StandardMaterial>, size: Vec3, at: Vec3) -> Part {
    Part {
        mesh: Cuboid::new(size.x, size.y, size.z).into(),
        material: m.clone(),
        transform: Transform::from_translation(at + Vec3::Y * (size.y * 0.5)),
    }
}

fn pile(m: &Handle<StandardMaterial>, radius: f32, height: f32, at: Vec3) -> Part {
    Part {
        mesh: Cone::new(radius, height).into(),
        material: m.clone(),
        transform: Transform::from_translation(at + Vec3::Y * (height * 0.5)),
    }
}

fn chimney(m: &Handle<StandardMaterial>, radius: f32, height: f32, at: Vec3) -> Part {
    Part {
        mesh: Cylinder::new(radius, height).into(),
        material: m.clone(),
        transform: Transform::from_translation(at + Vec3::Y * (height * 0.5)),
    }
}

/// Silhouette per kind (art-direction.md § Silhouette language).
fn parts(kind: BuildingKind, m: &BuildingMaterials) -> Vec<Part> {
    match kind {
        BuildingKind::Mine => vec![
            boxed(
                &m.brick,
                Vec3::new(9.0, 5.0, 8.0),
                Vec3::new(-2.0, 0.0, 0.0),
            ),
            boxed(&m.rust, Vec3::new(9.6, 0.6, 8.6), Vec3::new(-2.0, 5.0, 0.0)),
            // headframe tower + angled brace
            boxed(&m.rust, Vec3::new(2.6, 9.0, 2.6), Vec3::new(4.5, 0.0, 2.0)),
            Part {
                mesh: Cuboid::new(0.8, 8.0, 0.8).into(),
                material: m.rust.clone(),
                transform: Transform::from_translation(Vec3::new(2.2, 3.6, 2.0))
                    .with_rotation(Quat::from_rotation_z(0.5)),
            },
            pile(&m.coal, 3.2, 2.6, Vec3::new(-3.5, 0.0, 5.5)),
        ],
        BuildingKind::Quarry => vec![
            boxed(
                &m.timber,
                Vec3::new(6.0, 3.0, 4.0),
                Vec3::new(-4.0, 0.0, -3.0),
            ),
            boxed(
                &m.timber,
                Vec3::new(6.6, 0.5, 4.6),
                Vec3::new(-4.0, 3.0, -3.0),
            ),
            pile(&m.gravel, 3.4, 2.8, Vec3::new(3.5, 0.0, 2.0)),
            pile(&m.gravel, 2.4, 1.9, Vec3::new(0.0, 0.0, 4.0)),
            pile(&m.gravel, 1.8, 1.4, Vec3::new(5.5, 0.0, -2.5)),
        ],
        BuildingKind::PowerPlant => vec![
            boxed(
                &m.concrete,
                Vec3::new(13.0, 9.0, 10.0),
                Vec3::new(-1.5, 0.0, 0.0),
            ),
            boxed(
                &m.rust,
                Vec3::new(13.6, 0.7, 10.6),
                Vec3::new(-1.5, 9.0, 0.0),
            ),
            chimney(&m.concrete, 1.3, 16.0, Vec3::new(5.6, 0.0, -2.5)),
            chimney(&m.concrete, 1.3, 16.0, Vec3::new(5.6, 0.0, 2.5)),
            boxed(&m.coal, Vec3::new(4.0, 2.2, 5.0), Vec3::new(-7.5, 0.0, 3.0)),
        ],
        BuildingKind::Factory => {
            let mut v = vec![
                boxed(&m.concrete, Vec3::new(17.0, 7.0, 13.0), Vec3::ZERO),
                chimney(&m.brick, 0.9, 12.0, Vec3::new(7.5, 0.0, -5.0)),
            ];
            // sawtooth roof: four angled rust slabs
            for i in 0..4 {
                v.push(Part {
                    mesh: Cuboid::new(4.6, 0.5, 13.0).into(),
                    material: m.rust.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        -6.3 + i as f32 * 4.2,
                        7.8,
                        0.0,
                    ))
                    .with_rotation(Quat::from_rotation_z(0.45)),
                });
            }
            v
        }
        // Khrushchyovka slab block: concrete bar, banded floors, low entry.
        BuildingKind::Dwelling => {
            let mut v = vec![
                boxed(&m.concrete, Vec3::new(10.0, 11.0, 7.0), Vec3::ZERO),
                boxed(
                    &m.rust,
                    Vec3::new(10.4, 0.4, 7.4),
                    Vec3::new(0.0, 11.0, 0.0),
                ),
                boxed(
                    &m.timber,
                    Vec3::new(2.4, 2.6, 1.4),
                    Vec3::new(0.0, 0.0, 4.0),
                ),
            ];
            // floor bands: thin dark strips across the facade
            for i in 0..4 {
                v.push(boxed(
                    &m.coal,
                    Vec3::new(10.1, 0.25, 7.1),
                    Vec3::new(0.0, 2.4 + i as f32 * 2.4, 0.0),
                ));
            }
            v
        }
        // Long low storage shed: brick bar, shallow rust roof, loading doors
        // down the flank, crates and a spill pile on the apron.
        BuildingKind::Warehouse => {
            let mut v = vec![
                boxed(&m.brick, Vec3::new(17.0, 6.0, 9.0), Vec3::ZERO),
                boxed(&m.rust, Vec3::new(17.8, 0.6, 9.8), Vec3::new(0.0, 6.0, 0.0)),
                pile(&m.gravel, 2.2, 1.6, Vec3::new(7.0, 0.0, 6.5)),
            ];
            // three timber loading doors along the road-facing flank
            for i in 0..3 {
                v.push(boxed(
                    &m.timber,
                    Vec3::new(3.0, 3.6, 0.4),
                    Vec3::new(-5.0 + i as f32 * 5.0, 0.0, 4.6),
                ));
            }
            // crate stack by the west gable
            v.push(boxed(
                &m.timber,
                Vec3::new(2.4, 1.6, 2.4),
                Vec3::new(-8.0, 0.0, 6.2),
            ));
            v
        }
        // Vehicle depot: garage bar north of the apron, three open bays, a
        // fuel drum at the gable. The apron itself is the yard pad; parked
        // trucks render there per slot (game/vehicles.rs).
        BuildingKind::Depot => {
            let mut v = vec![
                boxed(&m.concrete, Vec3::new(19.0, 5.5, 8.0), Vec3::new(0.0, 0.0, -8.0)),
                boxed(
                    &m.rust,
                    Vec3::new(19.8, 0.6, 8.8),
                    Vec3::new(0.0, 5.5, -8.0),
                ),
                chimney(&m.rust, 1.1, 3.2, Vec3::new(8.0, 0.0, -1.5)),
            ];
            // open bay mouths along the south face of the bar
            for i in 0..3 {
                v.push(boxed(
                    &m.coal,
                    Vec3::new(4.2, 4.0, 0.4),
                    Vec3::new(-6.0 + i as f32 * 6.0, 0.0, -3.8),
                ));
            }
            v
        }
        // Bus shelter: concrete slab roof on two posts, timber bench.
        BuildingKind::BusStop => vec![
            boxed(&m.rust, Vec3::new(0.4, 2.8, 0.4), Vec3::new(-1.8, 0.0, -1.0)),
            boxed(&m.rust, Vec3::new(0.4, 2.8, 0.4), Vec3::new(1.8, 0.0, -1.0)),
            boxed(&m.concrete, Vec3::new(5.0, 0.4, 3.0), Vec3::new(0.0, 2.8, -0.4)),
            boxed(&m.timber, Vec3::new(4.0, 0.9, 0.8), Vec3::new(0.0, 0.0, -1.2)),
        ],
    }
}

fn sync_building_meshes(
    mut commands: Commands,
    added: Query<(Entity, &Building), Added<Building>>,
    mut meshes: ResMut<Assets<Mesh>>,
    palette: Res<BuildingMaterials>,
) {
    for (entity, building) in &added {
        let fp = building.kind.footprint();
        commands
            .entity(entity)
            .insert((
                Transform::from_translation(building.pos),
                Visibility::default(),
                Name::new(format!("{:?}", building.kind)),
            ))
            .with_children(|parent| {
                // worn yard pad anchoring the building to the field
                parent.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(fp.x + 8.0, fp.y + 8.0))),
                    MeshMaterial3d(palette.yard.clone()),
                    Transform::from_xyz(0.0, 0.04, 0.0),
                    Name::new("YardPad"),
                ));
                for (i, part) in parts(building.kind, &palette).into_iter().enumerate() {
                    parent.spawn((
                        Mesh3d(meshes.add(part.mesh)),
                        MeshMaterial3d(part.material),
                        part.transform,
                        Name::new(format!("Part{i}")),
                    ));
                }
            });
        if building.kind == BuildingKind::PowerPlant {
            commands.entity(entity).insert(SmokeStack {
                tips: [Vec3::new(5.6, 16.2, -2.5), Vec3::new(5.6, 16.2, 2.5)],
                cooldown: 0.0,
            });
        }
    }
}

/// Powered plants breathe: a puff per chimney every beat.
fn emit_smoke(
    mut commands: Commands,
    time: Res<Time>,
    mut stacks: Query<(&Building, &PowerOutput, &mut SmokeStack)>,
    mut meshes: ResMut<Assets<Mesh>>,
    palette: Res<BuildingMaterials>,
) {
    for (building, output, mut stack) in &mut stacks {
        stack.cooldown -= time.delta_secs();
        if output.0 <= 0.0 || stack.cooldown > 0.0 {
            continue;
        }
        stack.cooldown = 0.55;
        for (i, tip) in stack.tips.iter().enumerate() {
            commands.spawn((
                SmokePuff {
                    age: i as f32 * 0.13,
                    drift: Vec3::new(-1.1, 2.2, -0.6),
                },
                Mesh3d(meshes.add(Sphere::new(0.9))),
                MeshMaterial3d(palette.smoke[0].clone()),
                Transform::from_translation(building.pos + *tip),
                Name::new("SmokePuff"),
            ));
        }
    }
}

fn animate_smoke(
    mut commands: Commands,
    time: Res<Time>,
    palette: Res<BuildingMaterials>,
    mut puffs: Query<(
        Entity,
        &mut SmokePuff,
        &mut Transform,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
) {
    let dt = time.delta_secs();
    for (entity, mut puff, mut transform, mut material) in &mut puffs {
        puff.age += dt;
        if puff.age > 4.5 {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation += puff.drift * dt;
        transform.scale = Vec3::splat(1.0 + puff.age * 0.9);
        let band = if puff.age < 1.5 {
            0
        } else if puff.age < 3.0 {
            1
        } else {
            2
        };
        if material.0 != palette.smoke[band] {
            material.0 = palette.smoke[band].clone();
        }
    }
}
