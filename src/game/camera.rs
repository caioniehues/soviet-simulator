use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

use super::world::GROUND_HALF;

/// RTS camera rig: a focus point on the ground plus yaw/pitch/distance; the
/// camera transform is derived every frame, never mutated directly.
#[derive(Resource)]
pub struct CameraRig {
    pub focus: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub dist: f32,
}

impl Default for CameraRig {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            yaw: 0.0,
            pitch: -45f32.to_radians(),
            dist: 120.0,
        }
    }
}

const PAN_SPEED: f32 = 0.9; // metres per second per metre of zoom distance
const ROTATE_SPEED: f32 = 1.8; // radians per second (Q/E)
const ZOOM_RATE: f32 = 0.12; // exponential per scroll line
const DIST_RANGE: (f32, f32) = (15.0, 500.0);
const DRAG_PAN: f32 = 0.0016; // metres per pixel per metre of zoom distance

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraRig>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (control_rig, apply_rig).chain());
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        // Filmic look per docs/art-direction.md: neutral AgX, gentle bloom,
        // haze that closes the world instead of a hard horizon line.
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        DistanceFog {
            color: Color::srgb(0.78, 0.82, 0.85),
            directional_light_color: Color::srgba(1.0, 0.93, 0.82, 0.25),
            directional_light_exponent: 40.0,
            falloff: FogFalloff::Linear {
                start: 500.0,
                end: 1600.0,
            },
        },
        Name::new("RtsCamera"),
    ));
}

fn control_rig(
    mut rig: ResMut<CameraRig>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let (sin, cos) = rig.yaw.sin_cos();
    let forward = Vec3::new(-sin, 0.0, -cos);
    let right = Vec3::new(cos, 0.0, -sin);

    let mut pan = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        pan += forward;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        pan -= forward;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        pan += right;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        pan -= right;
    }
    let dist = rig.dist;
    rig.focus += pan.normalize_or_zero() * PAN_SPEED * dist * dt;

    if buttons.pressed(MouseButton::Middle) {
        let d = motion.delta;
        rig.focus += (right * -d.x + forward * d.y) * DRAG_PAN * dist;
    }

    if keys.pressed(KeyCode::KeyQ) {
        rig.yaw += ROTATE_SPEED * dt;
    }
    if keys.pressed(KeyCode::KeyE) {
        rig.yaw -= ROTATE_SPEED * dt;
    }

    if scroll.delta.y != 0.0 {
        rig.dist =
            (rig.dist * (-scroll.delta.y * ZOOM_RATE).exp()).clamp(DIST_RANGE.0, DIST_RANGE.1);
    }

    rig.focus.x = rig.focus.x.clamp(-GROUND_HALF, GROUND_HALF);
    rig.focus.z = rig.focus.z.clamp(-GROUND_HALF, GROUND_HALF);
    rig.focus.y = 0.0;
}

fn apply_rig(rig: Res<CameraRig>, mut camera: Query<&mut Transform, With<Camera3d>>) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    let rot = Quat::from_euler(EulerRot::YXZ, rig.yaw, rig.pitch, 0.0);
    let offset = rot * Vec3::new(0.0, 0.0, rig.dist);
    *transform = Transform::from_translation(rig.focus + offset).with_rotation(rot);
}
