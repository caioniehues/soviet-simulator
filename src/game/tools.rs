use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::sim::buildings::BuildingKind;
use crate::sim::roads::RoadClass;

/// Active build tool. The road/building/wire tools (M1.3–M1.6) plug their
/// behavior into this state machine; switching or Esc must always be safe.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToolMode {
    #[default]
    Inspect,
    Road(RoadClass),
    Building(BuildingKind),
    Wire,
    /// Click a source building, then a destination: creates a truck shuttle.
    Shuttle,
    /// Click bus stops in order; right-click closes the loop into a line.
    TransitLine,
    /// Two clicks span a land-use district; repeat presses cycle the use.
    Zone(crate::sim::zoning::ZoneKind),
}

/// Where the cursor ray hits the ground plane this frame, if it does.
#[derive(Resource, Default, Clone, Copy, PartialEq, Debug)]
pub struct GroundCursor(pub Option<Vec3>);

/// The ground cursor puck. Public so `juice.rs` can shake it on a refusal.
#[derive(Component)]
pub struct CursorMarker;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToolMode>()
            .init_resource::<GroundCursor>()
            .add_systems(Startup, spawn_cursor_marker)
            .add_systems(
                Update,
                (switch_tool, update_ground_cursor, update_cursor_marker).chain(),
            );
    }
}

/// Repeated Digit-3 presses walk the build tools in `BuildingKind::ALL`'s
/// order and wrap at the end; arriving from any other tool starts at the
/// first. That is the order the toolbar's BUILD flyout lists, so the two doors
/// into the tool state stay one door — they used to be two hand-written lists
/// that agreed only as long as nobody edited one of them.
pub(crate) fn next_building_kind(mode: ToolMode) -> BuildingKind {
    match mode {
        ToolMode::Building(kind) => BuildingKind::ALL[(kind as usize + 1) % BuildingKind::COUNT],
        _ => BuildingKind::ALL[0],
    }
}

fn switch_tool(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<ToolMode>) {
    let next = if keys.just_pressed(KeyCode::Escape) {
        Some(ToolMode::Inspect)
    } else if keys.just_pressed(KeyCode::Digit1) {
        Some(ToolMode::Road(RoadClass::Dirt))
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(ToolMode::Road(RoadClass::Paved))
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(ToolMode::Building(next_building_kind(*mode)))
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some(ToolMode::Wire)
    } else if keys.just_pressed(KeyCode::Digit5) {
        Some(ToolMode::Shuttle)
    } else if keys.just_pressed(KeyCode::Digit6) {
        Some(ToolMode::TransitLine)
    } else if keys.just_pressed(KeyCode::Digit7) {
        use crate::sim::zoning::ZoneKind;
        Some(ToolMode::Zone(match *mode {
            ToolMode::Zone(ZoneKind::Residential) => ZoneKind::Industrial,
            _ => ZoneKind::Residential,
        }))
    } else {
        None
    };
    if let Some(next) = next
        && *mode != next
    {
        info!("tool: {next:?}");
        *mode = next;
    }
}

fn update_ground_cursor(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    ui_hover: Res<super::toolbar::UiHover>,
    mut cursor: ResMut<GroundCursor>,
) {
    cursor.0 = (|| {
        // Pointer on the toolbar: park the cursor so a UI click can never
        // also fire the world tool underneath it.
        if ui_hover.0 {
            return None;
        }
        let window = window.single().ok()?;
        let viewport_pos = window.cursor_position()?;
        let (camera, camera_transform) = camera.single().ok()?;
        let ray = camera
            .viewport_to_world(camera_transform, viewport_pos)
            .ok()?;
        let dist = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))?;
        Some(ray.get_point(dist))
    })();
}

fn spawn_cursor_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        CursorMarker,
        Mesh3d(meshes.add(Cylinder::new(1.5, 0.2))),
        MeshMaterial3d(
            super::palette::Mat::new(super::palette::Role::SignalOk)
                .unlit()
                .add_to(&mut materials),
        ),
        Visibility::Hidden,
        Name::new("CursorMarker"),
    ));
}

fn update_cursor_marker(
    cursor: Res<GroundCursor>,
    rig: Res<super::camera::CameraRig>,
    mut marker: Query<(&mut Transform, &mut Visibility), With<CursorMarker>>,
) {
    let Ok((mut transform, mut visibility)) = marker.single_mut() else {
        return;
    };
    match cursor.0 {
        Some(point) => {
            transform.translation = point;
            // constant apparent size regardless of zoom
            transform.scale = Vec3::splat((rig.dist / 120.0).max(0.4));
            *visibility = Visibility::Visible;
        }
        None => *visibility = Visibility::Hidden,
    }
}
