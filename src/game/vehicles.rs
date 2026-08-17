//! Shuttle tool + truck rendering (presentation side). Shuttle creation goes
//! through the sim's edit queue; the truck is a color-coded primitive box per
//! the M1 charter (Q12). Easing writes only rendered transforms (ADR 0003).

use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use crate::sim::PostSimEasing;
use crate::sim::buildings::{Building, BuildingKind};
use crate::sim::resources::ResourceKind;
use crate::sim::vehicles::{
    ActivePawn, ActiveVehicle, ShuttleAssignment, VehicleAsset, VehicleEdit, VehicleEditQueue,
};

/// Click-picking radius around a building's centre, metres.
const PICK_RADIUS: f32 = 14.0;

/// First click of the shuttle tool, waiting for the destination click.
#[derive(Resource, Default)]
struct PendingShuttleSource(Option<Entity>);

pub struct VehicleToolPlugin;

impl Plugin for VehicleToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingShuttleSource>().add_systems(
            Update,
            (
                drive_shuttle_tool,
                preview_shuttle_source,
                hint_blocked_shuttles,
                sync_truck_meshes,
                ease_truck_transforms.in_set(PostSimEasing),
            ),
        );
    }
}

/// What a source yard of this kind ships. The B3 dispatcher replaces this
/// with real order matching.
fn shipped_resource(kind: BuildingKind) -> ResourceKind {
    match kind {
        BuildingKind::Quarry => ResourceKind::Gravel,
        BuildingKind::Factory | BuildingKind::Dwelling | BuildingKind::Warehouse => {
            ResourceKind::Goods
        }
        BuildingKind::Mine | BuildingKind::PowerPlant => ResourceKind::Coal,
    }
}

fn building_under_cursor(
    pos: Vec3,
    buildings: &Query<(Entity, &Building)>,
) -> Option<(Entity, BuildingKind)> {
    buildings
        .iter()
        .map(|(e, b)| (e, b.kind, b.pos.distance_squared(pos)))
        .filter(|(_, _, d2)| *d2 <= PICK_RADIUS * PICK_RADIUS)
        .min_by(|a, b| a.2.total_cmp(&b.2))
        .map(|(e, kind, _)| (e, kind))
}

fn drive_shuttle_tool(
    mode: Res<ToolMode>,
    cursor: Res<GroundCursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    buildings: Query<(Entity, &Building)>,
    mut pending: ResMut<PendingShuttleSource>,
    mut edits: ResMut<VehicleEditQueue>,
) {
    if *mode != ToolMode::Shuttle {
        pending.0 = None;
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(pos) = cursor.0 else { return };
    let Some((hit, kind)) = building_under_cursor(pos, &buildings) else {
        return;
    };
    match pending.0 {
        None => pending.0 = Some(hit),
        Some(source) if source != hit => {
            let resource = buildings
                .get(source)
                .map(|(_, b)| shipped_resource(b.kind))
                .unwrap_or(shipped_resource(kind));
            edits.0.push(VehicleEdit::CreateShuttle {
                from: source,
                to: hit,
                resource,
            });
            info!("shuttle: {source:?} -> {hit:?} ({resource:?})");
            pending.0 = None;
        }
        Some(_) => {}
    }
}

fn preview_shuttle_source(
    pending: Res<PendingShuttleSource>,
    cursor: Res<GroundCursor>,
    buildings: Query<&Building>,
    mut gizmos: Gizmos,
) {
    let Some(source) = pending.0 else { return };
    let Ok(building) = buildings.get(source) else {
        return;
    };
    let anchor = building.pos + Vec3::Y * 2.0;
    gizmos.circle(
        Isometry3d::new(anchor, Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        PICK_RADIUS,
        Color::srgb(0.9, 0.7, 0.2),
    );
    if let Some(pos) = cursor.0 {
        gizmos.line(anchor, pos + Vec3::Y * 2.0, Color::srgb(0.9, 0.7, 0.2));
    }
}

/// A shuttle whose truck can't be dispatched (usually: no road path between
/// its yards) shows as a pulsing red line between them, so a parked
/// assignment is never invisible.
fn hint_blocked_shuttles(
    time: Res<Time>,
    blocked: Query<&ShuttleAssignment, (With<VehicleAsset>, Without<ActivePawn>)>,
    buildings: Query<&Building>,
    mut gizmos: Gizmos,
) {
    let pulse = 0.55 + 0.45 * (time.elapsed_secs() * 4.0).sin();
    let color = Color::srgb(0.9, 0.15, 0.1).with_alpha(pulse);
    for order in &blocked {
        let (Ok(from), Ok(to)) = (buildings.get(order.from), buildings.get(order.to)) else {
            continue;
        };
        let (a, b) = (from.pos + Vec3::Y * 3.0, to.pos + Vec3::Y * 3.0);
        gizmos.line(a, b, color);
        for anchor in [a, b] {
            gizmos.circle(
                Isometry3d::new(anchor, Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                3.0,
                color,
            );
        }
    }
}

#[derive(Component)]
struct Wheel;

#[derive(Component)]
struct CargoMound;

/// P1 truck: olive cab + rust-boarded flatbed + four wheels, nose on -Z so
/// look_to() points it along the heading. Cargo shows as a dark mound.
fn sync_truck_meshes(
    mut commands: Commands,
    added: Query<(Entity, &ActiveVehicle), Added<ActiveVehicle>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, vehicle) in &added {
        let olive = materials.add(StandardMaterial {
            base_color: Color::srgb(0.36, 0.38, 0.24),
            perceptual_roughness: 0.75,
            ..default()
        });
        let boards = materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.30, 0.20),
            perceptual_roughness: 0.9,
            ..default()
        });
        let tire = materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.08, 0.09),
            perceptual_roughness: 0.95,
            ..default()
        });
        let load = materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.12, 0.13),
            perceptual_roughness: 1.0,
            ..default()
        });
        let wheel_mesh = meshes.add(Cylinder::new(0.55, 0.45));
        commands
            .entity(entity)
            .insert((
                Transform::from_translation(vehicle.pos + Vec3::Y * 0.55),
                Visibility::default(),
                Name::new("Truck"),
            ))
            .with_children(|parent| {
                // cab (front = -Z)
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(2.2, 1.7, 1.8))),
                    MeshMaterial3d(olive.clone()),
                    Transform::from_xyz(0.0, 1.0, -1.9),
                ));
                // bed floor + sideboards
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(2.3, 0.4, 3.4))),
                    MeshMaterial3d(olive),
                    Transform::from_xyz(0.0, 0.5, 0.85),
                ));
                for x in [-1.05, 1.05] {
                    parent.spawn((
                        Mesh3d(meshes.add(Cuboid::new(0.18, 0.8, 3.4))),
                        MeshMaterial3d(boards.clone()),
                        Transform::from_xyz(x, 1.05, 0.85),
                    ));
                }
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(2.3, 0.8, 0.18))),
                    MeshMaterial3d(boards),
                    Transform::from_xyz(0.0, 1.05, 2.5),
                ));
                // cargo mound, shown only when loaded
                parent.spawn((
                    CargoMound,
                    Mesh3d(meshes.add(Cuboid::new(1.9, 0.7, 3.0))),
                    MeshMaterial3d(load),
                    Transform::from_xyz(0.0, 1.05, 0.85),
                    Visibility::Hidden,
                ));
                // wheels: cylinder axis is Y; roll axis must be X
                for (x, z) in [(-1.1, -1.7), (1.1, -1.7), (-1.1, 1.6), (1.1, 1.6)] {
                    parent.spawn((
                        Wheel,
                        Mesh3d(wheel_mesh.clone()),
                        MeshMaterial3d(tire.clone()),
                        Transform::from_xyz(x, 0.0, z)
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ));
                }
            });
    }
}

fn ease_truck_transforms(
    time: Res<Time>,
    mut trucks: Query<(&ActiveVehicle, &mut Transform, &Children)>,
    mut wheels: Query<&mut Transform, (With<Wheel>, Without<ActiveVehicle>)>,
    mut mounds: Query<&mut Visibility, (With<CargoMound>, Without<Wheel>, Without<ActiveVehicle>)>,
) {
    // frame-rate-independent exponential approach to the authoritative position
    let alpha = 1.0 - (-14.0 * time.delta_secs()).exp();
    for (vehicle, mut transform, children) in &mut trucks {
        let target = vehicle.pos + Vec3::Y * 0.55;
        let travelled = transform.translation.distance(target);
        transform.translation = transform.translation.lerp(target, alpha);
        if vehicle.heading.length_squared() > 1e-6 {
            let facing = Transform::default().looking_to(vehicle.heading, Vec3::Y);
            transform.rotation = transform.rotation.slerp(facing.rotation, alpha);
        }
        let loaded = vehicle.cargo.total() > 0.5;
        let spin = Quat::from_rotation_y(-travelled * alpha / 0.55);
        for child in children {
            if let Ok(mut wheel) = wheels.get_mut(*child) {
                // local Y is the roll axis after the Z-rotation into place
                let rot = wheel.rotation * spin;
                wheel.rotation = rot;
            } else if let Ok(mut visibility) = mounds.get_mut(*child) {
                *visibility = if loaded {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
