//! Shuttle tool + truck rendering (presentation side). Shuttle creation goes
//! through the sim's edit queue; the truck is a color-coded primitive box per
//! the M1 charter (Q12). Easing writes only rendered transforms (ADR 0003).

use bevy::prelude::*;

use super::art::art;
use super::palette::{Mat, Role};
use super::tools::{GroundCursor, ToolMode};
use crate::sim::PostSimEasing;
use crate::sim::buildings::{Building, BuildingKind};
use crate::sim::resources::{Inventory, ResourceKind};
use crate::sim::vehicles::{
    ActivePawn, ActiveVehicle, VehicleAsset, VehicleEdit, VehicleEditQueue, depot_slot_pos,
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
                // Dress before the sim driver: a pawn that dies this frame
                // must never receive our deferred commands post-mortem.
                sync_truck_meshes.before(crate::sim::SimDriver),
                sync_parked_trucks.before(crate::sim::SimDriver),
                toggle_parked_visibility,
                ease_truck_transforms.in_set(PostSimEasing),
            ),
        );
    }
}

/// What a source ships when the haul-policy tool pairs it with a sink: its
/// biggest stock if it holds anything, else the kind's canonical product.
fn shipped_resource(kind: BuildingKind, inventory: Option<&Inventory>) -> ResourceKind {
    if let Some(inventory) = inventory {
        let stocked = ResourceKind::ALL
            .into_iter()
            .map(|r| (r, inventory.amount(r)))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .filter(|(_, amount)| *amount > 0.05);
        if let Some((resource, _)) = stocked {
            return resource;
        }
    }
    art(kind).shipped
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
    inventories: Query<&Inventory>,
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
                .map(|(_, b)| shipped_resource(b.kind, inventories.get(source).ok()))
                .unwrap_or_else(|_| shipped_resource(kind, None));
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
        Role::SignalOk.color(),
    );
    if let Some(pos) = cursor.0 {
        gizmos.line(anchor, pos + Vec3::Y * 2.0, Role::SignalOk.color());
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
    added: Query<
        (
            Entity,
            &ActiveVehicle,
            Option<&crate::sim::vehicles::PawnOf>,
        ),
        Added<ActiveVehicle>,
    >,
    fleet: Query<&VehicleAsset>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, vehicle, pawn_of) in &added {
        let kind = pawn_of
            .and_then(|p| fleet.get(p.0).ok())
            .map_or(crate::sim::vehicles::VehicleKind::Truck, |a| a.kind);
        dress_vehicle(
            &mut commands,
            entity,
            kind,
            vehicle.pos,
            &mut meshes,
            &mut materials,
        );
        commands.entity(entity).insert(Name::new(match kind {
            crate::sim::vehicles::VehicleKind::Bus => "Bus",
            crate::sim::vehicles::VehicleKind::Truck => "Truck",
            crate::sim::vehicles::VehicleKind::Excavator => "Excavator",
            crate::sim::vehicles::VehicleKind::Crane => "Crane",
        }));
    }
}

/// Parked fleet: every asset gets the same truck body, standing on its home
/// depot's slot (index = rank of its id within that depot's fleet). The fleet
/// is countable on the apron — that's the point of owned vehicles.
fn sync_parked_trucks(
    mut commands: Commands,
    added: Query<Entity, Added<VehicleAsset>>,
    fleet: Query<&VehicleAsset>,
    buildings: Query<&Building>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &added {
        let Ok(asset) = fleet.get(entity) else {
            continue;
        };
        let Ok(depot) = buildings.get(asset.home_depot) else {
            continue;
        };
        let slot = fleet
            .iter()
            .filter(|a| a.home_depot == asset.home_depot && a.id.0 < asset.id.0)
            .count() as u32;
        dress_vehicle(
            &mut commands,
            entity,
            asset.kind,
            depot_slot_pos(depot.pos, slot),
            &mut meshes,
            &mut materials,
        );
        commands.entity(entity).insert(Name::new("ParkedVehicle"));
    }
}

/// A parked body shows only while the asset has no live pawn; on the road the
/// pawn entity carries the visible truck.
fn toggle_parked_visibility(
    mut parked: Query<(Option<&ActivePawn>, &mut Visibility), With<VehicleAsset>>,
) {
    for (pawn, mut visibility) in &mut parked {
        let wanted = if pawn.is_some_and(|p| p.pawn().is_some()) {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}

/// LiAZ-ish city bus: one long teal box with a window band, front on -Z.
fn dress_bus(
    commands: &mut Commands,
    entity: Entity,
    pos: Vec3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let body = Mat::new(Role::CabGreen).add_to(materials);
    let glass = Mat::new(Role::Glass).polished(0.3).add_to(materials);
    let tire = Mat::new(Role::Coal).shade(0.6).add_to(materials);
    let wheel_mesh = meshes.add(Cylinder::new(0.55, 0.45));
    commands
        .entity(entity)
        .insert((
            Transform::from_translation(pos + Vec3::Y * 0.55),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.3, 2.2, 6.4))),
                MeshMaterial3d(body),
                Transform::from_xyz(0.0, 1.25, 0.0),
            ));
            for x in [-1.16, 1.16] {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.05, 0.7, 5.6))),
                    MeshMaterial3d(glass.clone()),
                    Transform::from_xyz(x, 1.75, 0.0),
                ));
            }
            // windshield
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.9, 0.7, 0.05))),
                MeshMaterial3d(glass),
                Transform::from_xyz(0.0, 1.75, -3.2),
            ));
            for (x, z) in [(-1.1, -2.2), (1.1, -2.2), (-1.1, 2.2), (1.1, 2.2)] {
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

fn dress_vehicle(
    commands: &mut Commands,
    entity: Entity,
    kind: crate::sim::vehicles::VehicleKind,
    pos: Vec3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    match kind {
        crate::sim::vehicles::VehicleKind::Bus => {
            dress_bus(commands, entity, pos, meshes, materials)
        }
        crate::sim::vehicles::VehicleKind::Truck => {
            dress_truck(commands, entity, pos, meshes, materials)
        }
        crate::sim::vehicles::VehicleKind::Excavator => {
            dress_machine(commands, entity, pos, false, meshes, materials)
        }
        crate::sim::vehicles::VehicleKind::Crane => {
            dress_machine(commands, entity, pos, true, meshes, materials)
        }
    }
}

/// Construction machine: ochre body on tracks; the crane variant carries a
/// tall lattice jib, the excavator a low boom.
fn dress_machine(
    commands: &mut Commands,
    entity: Entity,
    pos: Vec3,
    crane: bool,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let ochre = Mat::new(Role::MachineOchre)
        .roughness(0.8)
        .add_to(materials);
    let track = Mat::new(Role::Coal).shade(0.6).add_to(materials);
    commands
        .entity(entity)
        .insert((
            Transform::from_translation(pos + Vec3::Y * 0.4),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for x in [-1.0, 1.0] {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.7, 0.8, 3.4))),
                    MeshMaterial3d(track.clone()),
                    Transform::from_xyz(x, 0.0, 0.0),
                ));
            }
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.2, 1.6, 2.4))),
                MeshMaterial3d(ochre.clone()),
                Transform::from_xyz(0.0, 1.0, 0.3),
            ));
            if crane {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.5, 7.0, 0.5))),
                    MeshMaterial3d(ochre.clone()),
                    Transform::from_xyz(0.0, 1.8, 0.0),
                ));
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 5.0))),
                    MeshMaterial3d(ochre),
                    Transform::from_xyz(0.0, 8.4, -1.8),
                ));
            } else {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 3.6))),
                    MeshMaterial3d(ochre),
                    Transform::from_xyz(0.0, 2.0, -2.4),
                ));
            }
        });
}

fn dress_truck(
    commands: &mut Commands,
    entity: Entity,
    pos: Vec3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    {
        let olive = Mat::new(Role::CabGreen).shade(1.15).add_to(materials);
        let boards = Mat::new(Role::Timber).shade(1.1).add_to(materials);
        let tire = Mat::new(Role::Coal).shade(0.6).add_to(materials);
        let load = Mat::new(Role::Coal).roughness(1.0).add_to(materials);
        let wheel_mesh = meshes.add(Cylinder::new(0.55, 0.45));
        commands
            .entity(entity)
            .insert((
                Transform::from_translation(pos + Vec3::Y * 0.55),
                Visibility::default(),
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
