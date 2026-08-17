//! World dressing per docs/art-direction.md (P1 "First Light"): warm low sun,
//! cool ambient, filmic tonemapping, distance fog, and a tiled CC0 field
//! material instead of the flat debug green.

use bevy::light::CascadeShadowConfigBuilder;
use bevy::prelude::*;

use super::palette::{self, Mat, Role, SKY_HORIZON};

/// Ground half-extent in metres; the buildable field is `±HALF` on X/Z.
pub const GROUND_HALF: f32 = 1024.0;

/// Metres per texture tile on the ground.
const FIELD_TILE: f32 = 26.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Fallback only: the sky dome covers every pixel the world does not.
        // Keeping it on the horizon colour means a dropped dome frame reads as
        // haze rather than as a hole.
        app.insert_resource(ClearColor(SKY_HORIZON))
            .add_systems(Startup, spawn_world)
            .add_systems(Update, (draw_grid, palette::follow_camera));
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
    // Flat plane per M1 charter (Q13). The lawn-green regression lived on the
    // line below as a `(0.80, 0.78, 0.62)` tint over the raw ambientCG grass;
    // the olive now lives in the texture itself (tools/bake_ground.py) and the
    // factory keeps the multiplier white.
    let tiles = (GROUND_HALF * 2.0) / FIELD_TILE;
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(GROUND_HALF * 2.0, GROUND_HALF * 2.0),
            ),
        ),
        MeshMaterial3d(
            Mat::new(Role::FieldGround)
                .textured(
                    palette::load_tiled(&asset_server, "textures/field.png"),
                    tiles,
                )
                .normal(palette::load_tiled(&asset_server, "textures/field_n.png"))
                .roughness(1.0)
                .add_to(&mut materials),
        ),
        Name::new("Ground"),
    ));

    palette::spawn_sky(&mut commands, &mut meshes, &mut materials);

    // Warm low sun (~35° elevation) for long readable shadows.
    commands.spawn((
        DirectionalLight {
            // Cooled from (1.0, 0.93, 0.82): the art doc asks for
            // "overcast-but-bright", and at that warmth the olive field lit up
            // as desert khaki (#847e59 measured against a #6b7050 albedo). The
            // sun stays warm relative to the ambient fill; it just stops
            // dragging the whole world yellow.
            color: Color::srgb(1.0, 0.965, 0.90),
            illuminance: 9_600.0,
            shadow_maps_enabled: true,
            ..default()
        },
        // The shadows were enabled since P1 but invisible in play: Bevy's default
        // cascade covers 150 m, while the RTS rig sits 15–500 m out (camera.rs
        // DIST_RANGE), so at any normal zoom the ground fell past the last cascade.
        // Cover the whole fog-visible range instead, and push the first bound out
        // so near cascades aren't wasted on the 10 m nobody plays at.
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 1.0,
            maximum_distance: 900.0,
            first_cascade_far_bound: 60.0,
            overlap_proportion: 0.2,
        }
        .build(),
        // ~30° elevation, same azimuth. The old direction sat at ~45°, which at
        // this camera pitch dropped each building's shadow behind the building
        // itself; a lower sun throws it clear onto the ground where it does the
        // grounding work the art doc asks of it.
        Transform::from_xyz(0.0, 400.0, 0.0).looking_to(Vec3::new(-0.670, -0.5, -0.548), Vec3::Y),
        Name::new("Sun"),
    ));
    // Cool ambient *fill*, not flood: at 300 it washed the shadow side back to
    // nearly lit and cancelled both the cast shadows and the SSAO.
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.72, 0.80, 0.96),
        // 130 -> 175. Still well under the 300 that flattened the shadow side
        // and cancelled the SSAO, but enough cool fill to hold the field on
        // the olive side of khaki.
        brightness: 175.0,
        ..default()
    });
}
