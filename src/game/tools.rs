use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Active build tool. The road/building/wire tools (M1.3–M1.6) plug their
/// behavior into this state machine; switching or Esc must always be safe.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToolMode {
    #[default]
    Inspect,
    Road(RoadClass),
    Building,
    Wire,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoadClass {
    Dirt,
    Paved,
}

/// Where the cursor ray hits the ground plane this frame, if it does.
#[derive(Resource, Default, Clone, Copy, PartialEq, Debug)]
pub struct GroundCursor(pub Option<Vec3>);

#[derive(Component)]
struct CursorMarker;

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

fn switch_tool(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<ToolMode>) {
    let next = if keys.just_pressed(KeyCode::Escape) {
        Some(ToolMode::Inspect)
    } else if keys.just_pressed(KeyCode::Digit1) {
        Some(ToolMode::Road(RoadClass::Dirt))
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(ToolMode::Road(RoadClass::Paved))
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(ToolMode::Building)
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some(ToolMode::Wire)
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
    mut cursor: ResMut<GroundCursor>,
) {
    cursor.0 = (|| {
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
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.85, 0.2),
            unlit: true,
            ..default()
        })),
        Visibility::Hidden,
        Name::new("CursorMarker"),
    ));
}

fn update_cursor_marker(
    cursor: Res<GroundCursor>,
    mut marker: Query<(&mut Transform, &mut Visibility), With<CursorMarker>>,
) {
    let Ok((mut transform, mut visibility)) = marker.single_mut() else {
        return;
    };
    match cursor.0 {
        Some(point) => {
            transform.translation = point;
            *visibility = Visibility::Visible;
        }
        None => *visibility = Visibility::Hidden,
    }
}
