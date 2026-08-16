use bevy::prelude::*;

/// Ground half-extent in metres; the buildable field is `±HALF` on X/Z.
pub const GROUND_HALF: f32 = 1024.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
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
