use bevy::prelude::*;

/// Ground half-extent in metres; the buildable field is `±HALF` on X/Z.
pub const GROUND_HALF: f32 = 1024.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.62, 0.72, 0.82)))
            .add_systems(Startup, spawn_world)
            .add_systems(Update, draw_grid);
    }
}

/// 32 m reference grid; without it the flat plane reads as a blank screen.
fn draw_grid(mut gizmos: Gizmos) {
    let cells = (GROUND_HALF as u32 * 2) / 32;
    gizmos
        .grid(
            Isometry3d::new(
                Vec3::new(0.0, 0.02, 0.0),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ),
            UVec2::splat(cells),
            Vec2::splat(32.0),
            Color::srgba(1.0, 1.0, 1.0, 0.08),
        )
        .outer_edges();
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Flat plane per M1 charter (Q13): terrain factor exists in the speed
    // formula but the heightmap does not.
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(GROUND_HALF * 2.0, GROUND_HALF * 2.0),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.42, 0.47, 0.36),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Name::new("Ground"),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 400.0, 0.0).looking_to(Vec3::new(-0.4, -1.0, -0.3), Vec3::Y),
        Name::new("Sun"),
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.85, 0.88, 1.0),
        brightness: 250.0,
        ..default()
    });
}
