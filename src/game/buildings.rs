//! Building tool + rendering (presentation side). Placement goes through the
//! sim's edit queue. P1 "First Light": each kind gets a silhouette per
//! docs/art-direction.md — multi-part procedural meshes over a worn yard pad,
//! with chimney smoke on powered plants. Sim entities stay untouched; all
//! visual parts are children of the building entity.

use bevy::prelude::*;

use super::palette::{self, Mat, Role};
use super::tools::{GroundCursor, ToolMode};
use crate::sim::buildings::{Building, BuildingEdit, BuildingEditQueue, BuildingKind, PowerOutput};

/// Each kind's primary wall material, as a palette role plus a shade — the
/// families read apart at RTS zoom (brick mine, concrete plant, timber depot)
/// without any kind inventing a colour of its own. The old table held thirteen
/// hand-picked greys, two of which (the blue pump, the pink heat plant) were
/// saturation the art doc reserves for signals.
fn kind_material(kind: BuildingKind) -> Mat {
    let (role, shade) = match kind {
        BuildingKind::Mine => (Role::SootBrick, 1.0),
        BuildingKind::Quarry => (Role::Concrete, 0.8),
        BuildingKind::PowerPlant => (Role::Concrete, 0.72),
        BuildingKind::Factory => (Role::Concrete, 0.75),
        BuildingKind::Dwelling => (Role::Concrete, 0.82),
        BuildingKind::Warehouse => (Role::WornEarth, 0.9),
        BuildingKind::Depot => (Role::Timber, 1.15),
        BuildingKind::BusStop => (Role::Concrete, 0.85),
        BuildingKind::ConstructionOffice => (Role::MachineOchre, 0.85),
        BuildingKind::WaterPump => (Role::Concrete, 0.95),
        BuildingKind::SewagePlant => (Role::Concrete, 0.65),
        BuildingKind::HeatPlant => (Role::SootBrick, 1.2),
        BuildingKind::CustomsOffice => (Role::Concrete, 0.7),
    };
    Mat::new(role).shade(shade)
}

/// Roofs: rusted steel over anything industrial, tarred concrete over
/// anything civic. The old single rust roof was the other half of why every
/// building read the same.
fn roof_material(kind: BuildingKind) -> Mat {
    match kind {
        BuildingKind::Dwelling | BuildingKind::BusStop | BuildingKind::CustomsOffice => {
            Mat::new(Role::Asphalt).shade(1.25)
        }
        BuildingKind::Quarry | BuildingKind::Depot | BuildingKind::ConstructionOffice => {
            Mat::new(Role::Concrete).shade(0.55)
        }
        _ => Mat::new(Role::RustedSteel).shade(0.75).metallic(0.3),
    }
}

impl BuildingMaterials {
    fn wall(
        &mut self,
        kind: BuildingKind,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        self.walls
            .entry(kind)
            .or_insert_with(|| materials.add(kind_material(kind).build()))
            .clone()
    }

    fn roof(
        &mut self,
        kind: BuildingKind,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        self.roofs
            .entry(kind)
            .or_insert_with(|| materials.add(roof_material(kind).build()))
            .clone()
    }
}

fn kind_color(kind: BuildingKind) -> Color {
    kind_material(kind).build().base_color
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
        BuildingKind::ConstructionOffice => 5.0,
        BuildingKind::WaterPump => 4.0,
        BuildingKind::SewagePlant => 4.0,
        BuildingKind::HeatPlant => 11.0,
        BuildingKind::CustomsOffice => 5.0,
    }
}

/// Shared palette materials (art-direction.md § Material rules).
///
/// `walls` and `roofs` are per kind and filled lazily the first time a kind
/// is built. Before R0.2 every hall shared `brick` and every roof shared
/// `rust`, which made a warehouse, a mine and a heat plant the same red mass
/// at RTS zoom — the silhouettes differed but nothing else did.
#[derive(Resource)]
struct BuildingMaterials {
    walls: std::collections::HashMap<BuildingKind, Handle<StandardMaterial>>,
    roofs: std::collections::HashMap<BuildingKind, Handle<StandardMaterial>>,
    concrete: Handle<StandardMaterial>,
    brick: Handle<StandardMaterial>,
    rust: Handle<StandardMaterial>,
    timber: Handle<StandardMaterial>,
    coal: Handle<StandardMaterial>,
    gravel: Handle<StandardMaterial>,
    yard: Handle<StandardMaterial>,
    smoke: [Handle<StandardMaterial>; 3],
}

/// The worn ground pad under a building. Marked because it is part of the
/// building's entity tree but not part of its silhouette — `juice.rs` has to
/// keep the selection outline off it.
#[derive(Component)]
pub struct YardPad;

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
                rise_construction_sites,
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
    let smoke = |alpha: f32| Mat::new(Role::Smoke).unlit().alpha(alpha);
    commands.insert_resource(BuildingMaterials {
        walls: default(),
        roofs: default(),
        concrete: Mat::new(Role::Concrete).shade(0.8).add_to(&mut materials),
        brick: Mat::new(Role::SootBrick).shade(0.7).add_to(&mut materials),
        rust: Mat::new(Role::RustedSteel)
            .shade(0.7)
            .roughness(0.8)
            .metallic(0.35)
            .add_to(&mut materials),
        timber: Mat::new(Role::Timber).roughness(1.0).add_to(&mut materials),
        coal: Mat::new(Role::Coal).roughness(1.0).add_to(&mut materials),
        gravel: Mat::new(Role::Gravel).roughness(1.0).add_to(&mut materials),
        yard: Mat::new(Role::WornEarth)
            .textured(palette::load_tiled(&asset_server, "textures/dirt.png"), 2.5)
            .roughness(1.0)
            .add_to(&mut materials),
        smoke: [
            smoke(0.34).add_to(&mut materials),
            smoke(0.18).add_to(&mut materials),
            smoke(0.07).add_to(&mut materials),
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

/// Silhouette per kind (art-direction.md § Silhouette language). `wall` and
/// `roof` are the kind's own materials; the rest of `m` is shared detail.
fn parts(
    kind: BuildingKind,
    m: &BuildingMaterials,
    wall: &Handle<StandardMaterial>,
    roof: &Handle<StandardMaterial>,
) -> Vec<Part> {
    match kind {
        BuildingKind::Mine => vec![
            boxed(wall, Vec3::new(9.0, 5.0, 8.0), Vec3::new(-2.0, 0.0, 0.0)),
            boxed(roof, Vec3::new(9.6, 0.6, 8.6), Vec3::new(-2.0, 5.0, 0.0)),
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
            boxed(wall, Vec3::new(6.0, 3.0, 4.0), Vec3::new(-4.0, 0.0, -3.0)),
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
            boxed(wall, Vec3::new(13.0, 9.0, 10.0), Vec3::new(-1.5, 0.0, 0.0)),
            boxed(roof, Vec3::new(13.6, 0.7, 10.6), Vec3::new(-1.5, 9.0, 0.0)),
            chimney(&m.concrete, 1.3, 16.0, Vec3::new(5.6, 0.0, -2.5)),
            chimney(&m.concrete, 1.3, 16.0, Vec3::new(5.6, 0.0, 2.5)),
            boxed(&m.coal, Vec3::new(4.0, 2.2, 5.0), Vec3::new(-7.5, 0.0, 3.0)),
        ],
        BuildingKind::Factory => {
            let mut v = vec![
                boxed(wall, Vec3::new(17.0, 7.0, 13.0), Vec3::ZERO),
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
                boxed(wall, Vec3::new(10.0, 11.0, 7.0), Vec3::ZERO),
                boxed(roof, Vec3::new(10.4, 0.4, 7.4), Vec3::new(0.0, 11.0, 0.0)),
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
                boxed(wall, Vec3::new(17.0, 6.0, 9.0), Vec3::ZERO),
                boxed(roof, Vec3::new(17.8, 0.6, 9.8), Vec3::new(0.0, 6.0, 0.0)),
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
                boxed(wall, Vec3::new(19.0, 5.5, 8.0), Vec3::new(0.0, 0.0, -8.0)),
                boxed(roof, Vec3::new(19.8, 0.6, 8.8), Vec3::new(0.0, 5.5, -8.0)),
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
        // Site office: timber hut, rust flagpole, gravel-coloured apron pad.
        BuildingKind::ConstructionOffice => vec![
            boxed(wall, Vec3::new(9.0, 4.0, 7.0), Vec3::new(-4.0, 0.0, -6.0)),
            boxed(roof, Vec3::new(9.6, 0.5, 7.6), Vec3::new(-4.0, 4.0, -6.0)),
            boxed(&m.rust, Vec3::new(0.3, 6.5, 0.3), Vec3::new(2.5, 0.0, -8.0)),
            boxed(
                &m.concrete,
                Vec3::new(16.0, 0.3, 10.0),
                Vec3::new(0.0, 0.0, 5.0),
            ),
        ],
        // Pumphouse: brick box with an intake stack.
        BuildingKind::WaterPump => vec![
            boxed(wall, Vec3::new(7.0, 3.5, 5.5), Vec3::new(0.0, 0.0, 0.0)),
            boxed(roof, Vec3::new(7.6, 0.5, 6.1), Vec3::new(0.0, 3.5, 0.0)),
            chimney(&m.rust, 0.9, 3.0, Vec3::new(2.5, 0.0, -1.5)),
        ],
        // Treatment works: low concrete basin pair beside a shed.
        BuildingKind::SewagePlant => vec![
            boxed(
                &m.concrete,
                Vec3::new(6.0, 1.2, 6.0),
                Vec3::new(-4.0, 0.0, 0.0),
            ),
            boxed(
                &m.concrete,
                Vec3::new(6.0, 1.2, 6.0),
                Vec3::new(3.0, 0.0, 0.0),
            ),
            boxed(
                &m.timber,
                Vec3::new(4.5, 3.2, 3.5),
                Vec3::new(0.0, 0.0, -4.0),
            ),
        ],
        // District heating plant: boiler house with tall stack for heat distribution.
        BuildingKind::HeatPlant => vec![
            boxed(
                &m.brick,
                Vec3::new(12.0, 8.0, 9.0),
                Vec3::new(-1.0, 0.0, 0.0),
            ),
            boxed(roof, Vec3::new(12.6, 0.7, 9.6), Vec3::new(-1.0, 8.0, 0.0)),
            chimney(&m.concrete, 1.4, 14.0, Vec3::new(4.5, 0.0, -2.0)),
            boxed(&m.coal, Vec3::new(4.0, 2.2, 5.0), Vec3::new(-6.5, 0.0, 3.0)),
        ],
        // Border customs: gatehouse beside a barrier arm over the road,
        // flag mast, inspection canopy — the republic's front door.
        BuildingKind::CustomsOffice => vec![
            boxed(
                &m.brick,
                Vec3::new(8.0, 4.5, 6.0),
                Vec3::new(-5.0, 0.0, -2.0),
            ),
            boxed(
                &m.concrete,
                Vec3::new(8.6, 0.6, 6.6),
                Vec3::new(-5.0, 4.5, -2.0),
            ),
            // canopy over the inspection lane
            boxed(&m.rust, Vec3::new(0.5, 5.5, 0.5), Vec3::new(2.0, 0.0, -4.5)),
            boxed(&m.rust, Vec3::new(0.5, 5.5, 0.5), Vec3::new(2.0, 0.0, 1.5)),
            boxed(
                &m.concrete,
                Vec3::new(6.0, 0.5, 8.0),
                Vec3::new(2.0, 5.5, -1.5),
            ),
            // barrier arm
            boxed(&m.rust, Vec3::new(0.4, 1.2, 0.4), Vec3::new(6.5, 0.0, 2.5)),
            boxed(
                &m.timber,
                Vec3::new(0.3, 0.3, 6.0),
                Vec3::new(6.5, 1.2, -0.5),
            ),
            // flag mast
            chimney(&m.rust, 0.25, 8.0, Vec3::new(-9.5, 0.0, -5.5)),
        ],
        // Bus shelter: concrete slab roof on two posts, timber bench.
        BuildingKind::BusStop => vec![
            boxed(
                &m.rust,
                Vec3::new(0.4, 2.8, 0.4),
                Vec3::new(-1.8, 0.0, -1.0),
            ),
            boxed(&m.rust, Vec3::new(0.4, 2.8, 0.4), Vec3::new(1.8, 0.0, -1.0)),
            boxed(
                &m.concrete,
                Vec3::new(5.0, 0.4, 3.0),
                Vec3::new(0.0, 2.8, -0.4),
            ),
            boxed(
                &m.timber,
                Vec3::new(4.0, 0.9, 0.8),
                Vec3::new(0.0, 0.0, -1.2),
            ),
        ],
    }
}

/// A building under construction reads as rising out of the ground: its
/// render root squashes vertically with *real* site progress (never a
/// timer), and a stalled site draws a pulsing ring — amber for a missing
/// material, grey-blue for a missing machine.
fn rise_construction_sites(
    time: Res<Time>,
    mut sites: Query<(
        &Building,
        &crate::sim::construction::ConstructionSite,
        &mut Transform,
    )>,
    mut finished: Query<
        &mut Transform,
        (
            With<Building>,
            Without<crate::sim::construction::ConstructionSite>,
        ),
    >,
    mut gizmos: Gizmos,
) {
    use crate::sim::construction::Bottleneck;
    for (building, site, mut transform) in &mut sites {
        let rise = 0.12 + 0.88 * site.progress();
        if (transform.scale.y - rise).abs() > 1e-3 {
            transform.scale.y = rise;
        }
        if let Some(bottleneck) = site.bottleneck {
            let pulse = 0.5 + 0.5 * (time.elapsed_secs() * 3.0).sin();
            let color = match bottleneck {
                // A blocked site is the state's problem, not the world's:
                // signal colours, the doc's one sanctioned saturation.
                Bottleneck::NoMaterial => Role::SignalAttention.color(),
                Bottleneck::NoMachine => Role::SignalOk.color(),
            }
            .with_alpha(0.3 + 0.5 * pulse);
            gizmos.circle(
                Isometry3d::new(
                    building.pos + Vec3::Y * 0.25,
                    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ),
                building.kind.footprint().length() * 0.5 + 3.0,
                color,
            );
        }
    }
    // Activation snaps the structure to full height.
    for mut transform in &mut finished {
        if (transform.scale.y - 1.0).abs() > 1e-3 {
            transform.scale.y = 1.0;
        }
    }
}

fn sync_building_meshes(
    mut commands: Commands,
    added: Query<(Entity, &Building), Added<Building>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut palette: ResMut<BuildingMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, building) in &added {
        let fp = building.kind.footprint();
        let wall = palette.wall(building.kind, &mut materials);
        let roof = palette.roof(building.kind, &mut materials);
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
                    YardPad,
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(fp.x + 8.0, fp.y + 8.0))),
                    MeshMaterial3d(palette.yard.clone()),
                    Transform::from_xyz(0.0, 0.04, 0.0),
                    Name::new("YardPad"),
                ));
                for (i, part) in parts(building.kind, &palette, &wall, &roof)
                    .into_iter()
                    .enumerate()
                {
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
        // Smoke reads as load (R0.5): a plant at full output puffs three
        // times as often as one barely turning over. It used to run at a
        // fixed rate whenever `PowerOutput > 0`, which made a struggling
        // plant look identical to a healthy one.
        let load = (output.0 / crate::sim::buildings::PLANT_OUTPUT_MW).clamp(0.15, 1.0);
        stack.cooldown = 0.30 + 0.90 * (1.0 - load);
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
