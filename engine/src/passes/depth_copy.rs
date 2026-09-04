use crate::GfxContext;
use wgpu::{CommandEncoder, ImageCopyTexture, Origin3d, TextureAspect};

/// Copy the depth-prepass result into the sampled-only depth texture.
///
/// sov-ejz: SSAO, fog, and water sample depth, but the live depth texture is a
/// write attachment in the depth prepass, the main pass, and the background
/// pass. Sampling it inside the main pass (water) is a RESOURCE vs
/// DEPTH_STENCIL_WRITE conflict that kills the game, and declaring the main
/// pass read-only instead breaks capture validation (WRITE_AFTER_WRITE: wgpu
/// still stores the read-only attachment but only emits READ barriers). So the
/// samplers read this lossless per-frame copy, and the live texture stays a
/// pure attachment. Must run after the depth prepass has stored and before any
/// sampler reads: callers record it at the head of the before-main encoder,
/// which submits after the prepass and before the main pass.
pub fn copy_depth_for_sampling(gfx: &GfxContext, enc: &mut CommandEncoder) {
    profiling::scope!("depth copy");
    enc.copy_texture_to_texture(
        ImageCopyTexture {
            texture: &gfx.fbos.depth.texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::DepthOnly,
        },
        ImageCopyTexture {
            texture: &gfx.fbos.depth_sample.texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::DepthOnly,
        },
        gfx.fbos.depth.extent,
    );
}
