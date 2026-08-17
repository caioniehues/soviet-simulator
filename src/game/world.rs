//! World dressing per docs/art-direction.md (P1 "First Light"): warm low sun,
//! cool ambient, filmic tonemapping, distance fog, and a tiled CC0 field
//! material instead of the flat debug green.

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Affine2;
use bevy::prelude::*;

/// Ground half-extent in metres; the buildable field is `±HALF` on X/Z.
pub const GROUND_HALF: f32 = 1024.0;

/// Metres per texture tile on the ground.
const FIELD_TILE: f32 = 26.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Pale haze, matches the fog color so the world edge dissolves.
        app.insert_resource(ClearColor(Color::srgb(0.78, 0.82, 0.85)))
            .add_systems(Startup, spawn_world)
            .add_systems(Update, draw_grid);
    }
}

/// 32 m reference grid, now barely-there: the textured ground carries scale.
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
            Color::srgba(1.0, 1.0, 1.0, 0.025),
        )
        .outer_edges();
}

/// Load a texture set to tile (repeat addressing) instead of clamping.
pub fn load_tiled(asset_server: &AssetServer, path: &'static str) -> Handle<Image> {
    asset_server
        .load_builder()
        .with_settings(|s: &mut ImageLoaderSettings| {
            s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..ImageSamplerDescriptor::default()
            });
        })
        .load(path)
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut buildings: ResMut<crate::sim::buildings::BuildingEditQueue>,
) {
    // The border customs stands before the first plan does (G1.5): state
    // infrastructure, prebuilt — every imported vehicle drives in from here
    // and exports sell here. West of the starting view, its own trek away.
    buildings
        .0
        .push(crate::sim::buildings::BuildingEdit::PlacePrebuilt {
            kind: crate::sim::buildings::BuildingKind::CustomsOffice,
            pos: Vec3::new(-260.0, 0.0, 0.0),
        });
    // Flat plane per M1 charter (Q13); the material does the work now:
    // CC0 field texture (ambientCG Grass001) tinted toward desaturated olive.
    let tiles = (GROUND_HALF * 2.0) / FIELD_TILE;
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(GROUND_HALF * 2.0, GROUND_HALF * 2.0),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.80, 0.78, 0.62),
            base_color_texture: Some(load_tiled(&asset_server, "textures/field.png")),
            normal_map_texture: Some(load_tiled(&asset_server, "textures/field_n.png")),
            perceptual_roughness: 1.0,
            uv_transform: Affine2::from_scale(Vec2::splat(tiles)),
            ..default()
        })),
        Name::new("Ground"),
    ));

    // Warm low sun (~35° elevation) for long readable shadows.
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.93, 0.82),
            illuminance: 11_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 400.0, 0.0).looking_to(Vec3::new(-0.55, -0.7, -0.45), Vec3::Y),
        Name::new("Sun"),
    ));
    // Cool ambient fill against the warm key.
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.75, 0.82, 0.95),
        brightness: 300.0,
        ..default()
    });
}
