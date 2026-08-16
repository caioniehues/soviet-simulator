//! Shuttle tool + truck rendering (presentation side). Shuttle creation goes
//! through the sim's edit queue; the truck is a color-coded primitive box per
//! the M1 charter (Q12). Easing writes only rendered transforms (ADR 0003).

use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use crate::sim::PostSimEasing;
use crate::sim::buildings::{Building, BuildingKind};
use crate::sim::resources::ResourceKind;
use crate::sim::vehicles::{ActiveVehicle, VehicleEdit, VehicleEditQueue};

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
        BuildingKind::Factory => ResourceKind::Goods,
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

fn sync_truck_meshes(
    mut commands: Commands,
    added: Query<(Entity, &ActiveVehicle), Added<ActiveVehicle>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, vehicle) in &added {
        commands.entity(entity).insert((
            // long axis on -Z so look_to() points the nose along the heading
            Mesh3d(meshes.add(Cuboid::new(2.4, 2.2, 5.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.45, 0.48, 0.30),
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_translation(vehicle.pos + Vec3::Y * 1.1),
            Name::new("Truck"),
        ));
    }
}

fn ease_truck_transforms(time: Res<Time>, mut trucks: Query<(&ActiveVehicle, &mut Transform)>) {
    // frame-rate-independent exponential approach to the authoritative position
    let alpha = 1.0 - (-14.0 * time.delta_secs()).exp();
    for (vehicle, mut transform) in &mut trucks {
        let target = vehicle.pos + Vec3::Y * 1.1;
        transform.translation = transform.translation.lerp(target, alpha);
        if vehicle.heading.length_squared() > 1e-6 {
            let facing = Transform::default().looking_to(vehicle.heading, Vec3::Y);
            transform.rotation = transform.rotation.slerp(facing.rotation, alpha);
        }
    }
}
