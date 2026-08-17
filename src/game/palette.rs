//! The palette factory (R0.2, #112) — the single sanctioned way to build a
//! `StandardMaterial`, and the sky dome that closes the world.
//!
//! `docs/art-direction.md` is normative, and it had already drifted once: the
//! ground read as saturated lawn-green, which the doc forbids by name. A
//! document nothing enforces drifts again, so every colour in `src/game/` now
//! comes from a [`Role`] here and every material is built through [`Mat`],
//! which applies the doc's § Material rules on the way out:
//!
//! - perceptual roughness ≥ 0.75 (except the explicitly [`Mat::polished`] —
//!   glass and lamps),
//! - metallic ≤ 0.6,
//! - albedo clamped to `[0.05, 0.85]` — no pure white, no pure black.
//!
//! **Textured materials are the one exception to the albedo clamp**, and they
//! earn it: their albedo lives in the image, not in `base_color`, so clamping
//! the multiplier would only darken an already-correct texture. The textures
//! are instead baked on-palette offline by `tools/bake_ground.py`, which is
//! also the reason the lawn-green could not be fixed here — reaching the doc's
//! olive from the ambientCG source needed linear tints of (2.5, 1.5, 3.6) and
//! `Color::srgb` clamps at 1.0.

use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::math::Affine2;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

/// A named surface from `docs/art-direction.md` § Palette. The ten rows of the
/// doc's table, plus the extended rows it grew to cover materials the table
/// never named (coal, gravel, glass, cloth, machine ochre, cab green, smoke)
/// and the third alarm severity R0.4 needs.
///
/// Adding a variant means adding a row to the doc first. That ordering is the
/// whole point of this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    FieldGround,
    WornEarth,
    Concrete,
    SootBrick,
    RustedSteel,
    Timber,
    Asphalt,
    DirtRoad,
    Coal,
    Gravel,
    Glass,
    Cloth,
    MachineOchre,
    CabGreen,
    Smoke,
    /// Utility-network overlay lines. Schematic rather than material — the
    /// player has to tell three networks apart at a glance — so they sit
    /// beside the signals rather than among the surfaces.
    WaterLine,
    HeatLine,
    /// Map-overlay colours: land-use districts and transit lines. Same
    /// reasoning as the utility lines — schematic legibility, not material.
    ZoneResidential,
    ZoneIndustrial,
    TransitLine,
    SignalOk,
    SignalAttention,
    SignalFail,
}

impl Role {
    /// The role's authored colour, straight from the doc's hex.
    pub const fn color(self) -> Color {
        let (r, g, b) = self.rgb8();
        Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    const fn rgb8(self) -> (u8, u8, u8) {
        match self {
            Role::FieldGround => (0x6b, 0x70, 0x50),
            Role::WornEarth => (0x7a, 0x6a, 0x52),
            Role::Concrete => (0x9a, 0x96, 0x8c),
            Role::SootBrick => (0x6e, 0x4a, 0x3c),
            Role::RustedSteel => (0x8a, 0x5a, 0x3a),
            Role::Timber => (0x5f, 0x4a, 0x34),
            Role::Asphalt => (0x3a, 0x3a, 0x3c),
            Role::DirtRoad => (0x8a, 0x73, 0x55),
            Role::Coal => (0x1f, 0x1f, 0x22),
            Role::Gravel => (0x8f, 0x8a, 0x80),
            Role::Glass => (0x7a, 0x88, 0x94),
            Role::Cloth => (0x4a, 0x46, 0x40),
            Role::MachineOchre => (0x8a, 0x6a, 0x2e),
            Role::CabGreen => (0x3f, 0x4f, 0x44),
            Role::Smoke => (0xb8, 0xb8, 0xbc),
            Role::WaterLine => (0x3d, 0x77, 0xa8),
            Role::HeatLine => (0xc2, 0x5b, 0x2c),
            Role::ZoneResidential => (0x4e, 0x8a, 0x4a),
            Role::ZoneIndustrial => (0xa8, 0x72, 0x2e),
            Role::TransitLine => (0x3d, 0x8f, 0xa8),
            Role::SignalOk => (0xff, 0xd3, 0x4d),
            Role::SignalAttention => (0xd9, 0x8f, 0x2b),
            Role::SignalFail => (0xa1, 0x2a, 0x1c),
        }
    }

    /// Signal colours are the doc's one sanctioned saturation, and lamps are
    /// allowed to reach past the albedo ceiling — they emit rather than
    /// reflect.
    pub const fn is_signal(self) -> bool {
        matches!(
            self,
            Role::SignalOk
                | Role::SignalAttention
                | Role::SignalFail
                | Role::WaterLine
                | Role::HeatLine
                | Role::ZoneResidential
                | Role::ZoneIndustrial
                | Role::TransitLine
        )
    }
}

/// Sky gradient, `docs/art-direction.md` § Palette last row. Pale at the
/// horizon so distant ground dissolves into it, deeper overhead.
pub const SKY_HORIZON: Color = Color::srgb(
    0xc7 as f32 / 255.0,
    0xd0 as f32 / 255.0,
    0xd8 as f32 / 255.0,
);
pub const SKY_ZENITH: Color = Color::srgb(
    0x8f as f32 / 255.0,
    0xa3 as f32 / 255.0,
    0xb5 as f32 / 255.0,
);

/// Albedo bounds from § Material rules: no pure white, no pure black.
const ALBEDO: (f32, f32) = (0.05, 0.85);
/// Everything man-made here is worn; only glass and lamps opt out.
const MIN_ROUGHNESS: f32 = 0.75;
const MAX_METALLIC: f32 = 0.6;

/// A material request. Build one, adjust it, hand it to [`Mat::add`]; the
/// doc's rules are applied in [`Mat::build`], not by the caller.
#[derive(Clone)]
pub struct Mat {
    role: Role,
    /// Linear multiplier on the role colour, for shading a role into a family
    /// (a darker base band, a lighter panel) without inventing a new colour.
    shade: f32,
    texture: Option<Handle<Image>>,
    normal: Option<Handle<Image>>,
    uv_scale: f32,
    roughness: f32,
    metallic: f32,
    alpha: Option<f32>,
    unlit: bool,
    /// Opts out of the roughness floor — glass and lamps only.
    polished: bool,
    emissive: f32,
}

impl Mat {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            shade: 1.0,
            texture: None,
            normal: None,
            uv_scale: 1.0,
            roughness: 0.95,
            metallic: 0.0,
            alpha: None,
            unlit: false,
            polished: false,
            emissive: 0.0,
        }
    }

    /// Darken (`< 1`) or lighten (`> 1`) the role colour. Applied in linear
    /// light, then re-clamped, so a shade can never escape the palette.
    pub fn shade(mut self, factor: f32) -> Self {
        self.shade = factor;
        self
    }

    /// Attach a baked-on-palette texture set. `uv_scale` is tiles per mesh UV
    /// unit. Textured materials keep a white multiplier — see the module doc.
    pub fn textured(mut self, texture: Handle<Image>, uv_scale: f32) -> Self {
        self.texture = Some(texture);
        self.uv_scale = uv_scale;
        self
    }

    pub fn normal(mut self, normal: Handle<Image>) -> Self {
        self.normal = Some(normal);
        self
    }

    pub fn roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness;
        self
    }

    pub fn metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic;
        self
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    /// Unlit *and* polished: lamps, ghosts, smoke, cursor markers — anything
    /// that is a readable mark rather than a lit surface.
    pub fn unlit(mut self) -> Self {
        self.unlit = true;
        self.polished = true;
        self
    }

    /// Glass and lamps only: skips the roughness floor.
    pub fn polished(mut self, roughness: f32) -> Self {
        self.polished = true;
        self.roughness = roughness;
        self
    }

    /// Glow strength in role colour, for powered lamps and lit windows.
    pub fn emissive(mut self, strength: f32) -> Self {
        self.emissive = strength;
        self
    }

    /// The enforcement point. Everything the doc mandates is applied here and
    /// nowhere else.
    pub fn build(self) -> StandardMaterial {
        let mut rgb = self.role.color().to_linear();
        rgb = LinearRgba::rgb(
            rgb.red * self.shade,
            rgb.green * self.shade,
            rgb.blue * self.shade,
        );

        // Albedo clamp. Textured materials are exempt (the texture carries the
        // albedo); signals and lamps may pass the ceiling but not the floor.
        let base_color = if self.texture.is_some() {
            Color::WHITE
        } else {
            let ceiling = if self.role.is_signal() || self.unlit {
                1.0
            } else {
                ALBEDO.1
            };
            let clamp = |c: f32| c.clamp(ALBEDO.0, ceiling);
            let mut c = Color::from(LinearRgba::rgb(
                clamp(rgb.red),
                clamp(rgb.green),
                clamp(rgb.blue),
            ));
            if let Some(a) = self.alpha {
                c.set_alpha(a);
            }
            c
        };

        let roughness = if self.polished {
            self.roughness.clamp(0.0, 1.0)
        } else {
            self.roughness.clamp(MIN_ROUGHNESS, 1.0)
        };

        StandardMaterial {
            base_color,
            base_color_texture: self.texture,
            normal_map_texture: self.normal,
            uv_transform: Affine2::from_scale(Vec2::splat(self.uv_scale)),
            perceptual_roughness: roughness,
            metallic: self.metallic.clamp(0.0, MAX_METALLIC),
            emissive: LinearRgba::rgb(
                rgb.red * self.emissive,
                rgb.green * self.emissive,
                rgb.blue * self.emissive,
            ),
            alpha_mode: match self.alpha {
                Some(a) if a < 1.0 => AlphaMode::Blend,
                _ => AlphaMode::Opaque,
            },
            ..default()
        }
    }

    /// Build and register in one step — the shape nearly every call site wants.
    pub fn add_to(self, materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
        materials.add(self.build())
    }
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

// ---------------------------------------------------------------- sky dome

/// Marks the dome so it can be kept centred on the camera.
#[derive(Component)]
pub struct SkyDome;

/// Radius: past the fog's 1600 m close-out and past the ±1024 m ground, so no
/// world geometry is ever occluded by the sky; the camera's far plane is
/// widened to match in `camera.rs`.
const SKY_RADIUS: f32 = 3000.0;

/// A vertex-coloured hemisphere-plus-skirt, drawn from the inside.
///
/// Bevy 0.19 ships a physical (Bruneton) atmosphere, and it is the wrong tool
/// here: it renders a clear blue sky, while the art doc asks for an
/// overcast-but-bright pale grey-blue where "buildings carry the value range".
/// A gradient dome is both closer to the reference and one unlit draw call.
fn sky_mesh() -> Mesh {
    const RINGS: usize = 16;
    const SECTORS: usize = 32;
    // Start below the horizon so the seam is never visible over distant ground.
    const START_LAT: f32 = -0.25;

    let mut positions = Vec::with_capacity((RINGS + 1) * (SECTORS + 1));
    let mut normals = Vec::with_capacity((RINGS + 1) * (SECTORS + 1));
    let mut uvs = Vec::with_capacity((RINGS + 1) * (SECTORS + 1));
    let mut colors = Vec::with_capacity((RINGS + 1) * (SECTORS + 1));
    let mut indices = Vec::with_capacity(RINGS * SECTORS * 6);

    // The dome is unlit but still passes through TonyMcMapface, which pulls a
    // pale sky down roughly a fifth of a stop — measured at #a8aeb4 against an
    // authored #c7d0d8. The stops are pre-brightened so the *rendered* sky is
    // the colour the art doc names, which is the one that matters.
    const TONEMAP_LIFT: f32 = 1.62;
    let lift = |c: LinearRgba| {
        LinearRgba::rgb(
            c.red * TONEMAP_LIFT,
            c.green * TONEMAP_LIFT,
            c.blue * TONEMAP_LIFT,
        )
    };
    let horizon = lift(SKY_HORIZON.to_linear());
    let zenith = lift(SKY_ZENITH.to_linear());

    for ring in 0..=RINGS {
        let t = ring as f32 / RINGS as f32;
        let lat = START_LAT + t * (std::f32::consts::FRAC_PI_2 - START_LAT);
        let (sin_lat, cos_lat) = lat.sin_cos();
        // Ease the blend so the gradient concentrates near the horizon, the way
        // an overcast sky does, instead of ramping linearly to the zenith. The
        // exponent is well under 1 because the RTS camera spends most of its
        // life below 20° of elevation, and a linear ramp puts the whole
        // gradient in sky the player never looks at.
        let blend = lat.max(0.0).sin().powf(0.42);
        let c = LinearRgba::rgb(
            horizon.red + (zenith.red - horizon.red) * blend,
            horizon.green + (zenith.green - horizon.green) * blend,
            horizon.blue + (zenith.blue - horizon.blue) * blend,
        );
        for sector in 0..=SECTORS {
            let lon = sector as f32 / SECTORS as f32 * std::f32::consts::TAU;
            let (sin_lon, cos_lon) = lon.sin_cos();
            let dir = [cos_lat * cos_lon, sin_lat, cos_lat * sin_lon];
            positions.push([
                SKY_RADIUS * dir[0],
                SKY_RADIUS * dir[1],
                SKY_RADIUS * dir[2],
            ]);
            // Normals point inward, at the camera. Without them the mesh has
            // no normal attribute at all, which the depth prepass warns about
            // and which left the first dome invisible — the "sky" in that
            // capture was the ClearColor showing through.
            normals.push([-dir[0], -dir[1], -dir[2]]);
            uvs.push([sector as f32 / SECTORS as f32, t]);
            colors.push([c.red, c.green, c.blue, 1.0]);
        }
    }

    let stride = SECTORS + 1;
    for ring in 0..RINGS {
        for sector in 0..SECTORS {
            let a = (ring * stride + sector) as u32;
            let (b, c, d) = (a + 1, a + stride as u32, a + stride as u32 + 1);
            // Wound to face inward — the camera lives inside the dome.
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn spawn_sky(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.spawn((
        SkyDome,
        Mesh3d(meshes.add(sky_mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            // The dome is the fog's destination, not one of its subjects.
            fog_enabled: false,
            // Seen from inside, so winding must not be able to hide it.
            cull_mode: None,
            double_sided: true,
            ..default()
        })),
        // The dome is scenery, never an occluder or a shadow caster.
        NotShadowCaster,
        NotShadowReceiver,
        Name::new("SkyDome"),
    ));
}

/// Keep the dome centred on the camera so its horizon never slides.
pub fn follow_camera(
    camera: Query<&GlobalTransform, (With<Camera3d>, Without<SkyDome>)>,
    mut dome: Query<&mut Transform, With<SkyDome>>,
) {
    let (Ok(cam), Ok(mut dome)) = (camera.single(), dome.single_mut()) else {
        return;
    };
    dome.translation = cam.translation();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn albedo(mat: &StandardMaterial) -> [f32; 3] {
        let c = mat.base_color.to_linear();
        [c.red, c.green, c.blue]
    }

    #[test]
    fn roughness_floor_is_enforced_but_glass_may_opt_out() {
        assert_eq!(
            Mat::new(Role::Concrete)
                .roughness(0.1)
                .build()
                .perceptual_roughness,
            MIN_ROUGHNESS
        );
        assert_eq!(
            Mat::new(Role::Glass)
                .polished(0.25)
                .build()
                .perceptual_roughness,
            0.25
        );
    }

    #[test]
    fn metallic_is_capped() {
        assert_eq!(
            Mat::new(Role::RustedSteel).metallic(1.0).build().metallic,
            MAX_METALLIC
        );
    }

    #[test]
    fn albedo_never_reaches_pure_black_or_white() {
        // Coal is the darkest role and would otherwise sink past the floor.
        for c in albedo(&Mat::new(Role::Coal).shade(0.0).build()) {
            assert!(c >= ALBEDO.0, "{c} below the albedo floor");
        }
        for c in albedo(&Mat::new(Role::Concrete).shade(10.0).build()) {
            assert!(c <= ALBEDO.1, "{c} above the albedo ceiling");
        }
    }

    #[test]
    fn signals_keep_their_saturation_past_the_ceiling() {
        let ok = albedo(&Mat::new(Role::SignalOk).build());
        assert!(
            ok[0] > ALBEDO.1,
            "signal red channel was clamped to {}",
            ok[0]
        );
    }

    #[test]
    fn textured_materials_keep_a_white_multiplier() {
        let mat = Mat::new(Role::FieldGround)
            .textured(Handle::default(), 4.0)
            .build();
        assert_eq!(mat.base_color, Color::WHITE);
    }

    #[test]
    fn sky_mesh_is_closed_and_graded() {
        let mesh = sky_mesh();
        let colors = mesh.attribute(Mesh::ATTRIBUTE_COLOR).unwrap();
        assert!(!colors.is_empty());
        assert_eq!(colors.len(), mesh.count_vertices());
        // Two triangles per quad over the whole ring/sector grid.
        assert_eq!(mesh.indices().map(|i| i.len()), Some(16 * 32 * 6));
    }
}
