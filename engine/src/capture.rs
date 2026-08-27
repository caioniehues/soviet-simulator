//! Frame capture: read one rendered surface texture back to the CPU and write it as a PNG.
//!
//! A frame is only evidence if the run that produced it was pinned. [`crate::framework::
//! FrameworkOptions`] pins window size, frame delta and input; [`CaptureRecord`] records what the
//! machine actually did, so a later reader can tell one adapter's output from another's.

use std::path::Path;

use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Device, Extent3d,
    ImageCopyBuffer, ImageDataLayout, Queue, Texture, TextureFormat,
};

/// Which channel order the source rows arrive in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Swizzle {
    Rgba,
    Bgra,
}

/// One frame read back from the GPU, always stored as 8-bit RGBA regardless of surface format.
pub struct CapturedFrame {
    pub width: u32,
    pub height: u32,
    /// The surface format the frame was drawn in, before conversion to RGBA8.
    pub source_format: TextureFormat,
    pub rgba: Vec<u8>,
}

impl CapturedFrame {
    /// Write the frame as an 8-bit RGBA PNG, creating the parent directory if needed.
    pub fn write_png(&self, path: &Path) -> Result<(), String> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("could not create {}: {e}", dir.display()))?;
        }
        let buf: image::ImageBuffer<image::Rgba<u8>, _> =
            image::ImageBuffer::from_raw(self.width, self.height, self.rgba.clone())
                .ok_or_else(|| "captured buffer does not match its declared size".to_string())?;
        buf.save_with_format(path, image::ImageFormat::Png)
            .map_err(|e| format!("could not write {}: {e}", path.display()))
    }
}

/// Drop the row padding wgpu requires on texture-to-buffer copies and put the channels in RGBA
/// order.
///
/// `copy_texture_to_buffer` demands `bytes_per_row` be a multiple of
/// [`wgpu::COPY_BYTES_PER_ROW_ALIGNMENT`], so every row of `mapped` carries trailing bytes that
/// are not pixels. Keeping them shears the image; keeping BGRA order swaps red and blue.
pub fn unpad_and_swizzle(
    mapped: &[u8],
    width: u32,
    height: u32,
    padded_row: u32,
    swizzle: Swizzle,
) -> Vec<u8> {
    let unpadded_row = (width * 4) as usize;
    let mut rgba = Vec::with_capacity(unpadded_row * height as usize);
    for row in 0..height as usize {
        let start = row * padded_row as usize;
        let src = &mapped[start..start + unpadded_row];
        match swizzle {
            Swizzle::Rgba => rgba.extend_from_slice(src),
            Swizzle::Bgra => {
                for px in src.chunks_exact(4) {
                    rgba.extend_from_slice(&[px[2], px[1], px[0], px[3]]);
                }
            }
        }
    }
    rgba
}

/// Copy a rendered texture back to the CPU.
///
/// `tex` must carry [`wgpu::TextureUsages::COPY_SRC`]. The surface only gets that usage when
/// capture mode asked for it, so ordinary runs keep their original configuration.
///
/// Every failure returns a concrete reason rather than panicking: a capture that cannot be taken
/// must not take the process down with it.
pub fn capture_texture(
    device: &Device,
    queue: &Queue,
    tex: &Texture,
) -> Result<CapturedFrame, String> {
    let format = tex.format();
    let swizzle = match format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => Swizzle::Rgba,
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => Swizzle::Bgra,
        other => {
            return Err(format!(
            "capture does not handle surface format {other:?}; only 8-bit RGBA/BGRA are supported"
        ))
        }
    };

    let width = tex.width();
    let height = tex.height();
    if width == 0 || height == 0 {
        return Err(format!("surface is {width}x{height}, nothing to capture"));
    }

    let unpadded_row = width * 4;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_row = unpadded_row.div_ceil(align) * align;

    let readback: Buffer = device.create_buffer(&BufferDescriptor {
        label: Some("frame capture readback"),
        size: (padded_row as u64) * (height as u64),
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut enc = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("frame capture encoder"),
    });
    enc.copy_texture_to_buffer(
        tex.as_image_copy(),
        ImageCopyBuffer {
            buffer: &readback,
            layout: ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(padded_row),
                rows_per_image: Some(height),
            },
        },
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(enc.finish()));

    let slice = readback.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    device.poll(wgpu::Maintain::Wait);
    match rx.recv() {
        Ok(Ok(())) => {}
        Ok(Err(e)) => return Err(format!("could not map the capture buffer: {e}")),
        Err(e) => return Err(format!("capture buffer mapping never reported back: {e}")),
    }

    let mapped = slice.get_mapped_range();
    let rgba = unpad_and_swizzle(&mapped, width, height, padded_row, swizzle);
    drop(mapped);
    readback.unmap();

    Ok(CapturedFrame {
        width,
        height,
        source_format: format,
        rgba,
    })
}

/// What the engine knows about a capture run. The caller adds whatever scene-level facts it has
/// and writes the whole thing out; the engine never invents a file format.
///
/// Every field here is stable across two runs on the same machine and build, which is what makes
/// two captures comparable.
pub struct CaptureRecord {
    /// Adapter as wgpu reports it: name, backend, device type, driver.
    pub adapter: wgpu::AdapterInfo,
    /// Device features actually granted, which is not the same as the features requested.
    pub enabled_features: wgpu::Features,
    /// Physical pixels actually rendered, read back from the surface rather than requested.
    pub width: u32,
    pub height: u32,
    /// The size that was asked for, when capture mode pinned one. A compositor may refuse it.
    pub requested_size: Option<(u32, u32)>,
    pub surface_format: TextureFormat,
    pub present_mode: wgpu::PresentMode,
    pub msaa_samples: u32,
    /// Whether wgpu was asked for its validation layers on this run.
    pub validation_requested: bool,
    /// Render passes that actually ran, derived from live settings rather than a fixed list.
    pub passes: Vec<&'static str>,
    /// Frames drawn before the captured one.
    pub warmup_frames: u32,
    /// The fixed per-frame delta, in seconds.
    pub fixed_delta: f32,
    /// Deterministic draw statistics for the captured frame.
    pub total_drawcalls: usize,
    pub total_triangles: usize,
    /// Per-pass GPU timings, empty unless the opt-in gate was on and the adapter supported it.
    pub gpu_timings: Vec<crate::gpu_timing::PassTimingSummary>,
    /// Why GPU timing produced nothing, when it produced nothing.
    pub gpu_timing_status: String,
}

/// How the binary taking the capture was built.
///
/// The engine cannot know this: only the crate being compiled sees its own version, profile and
/// commit. [`CaptureRecord::to_json`] takes it by value rather than leaving it to `extra`, so a
/// record physically cannot be written without build provenance.
pub struct BuildInfo {
    pub crate_name: &'static str,
    pub crate_version: &'static str,
    /// Contents of the repository VERSION file at compile time.
    pub engine_version: &'static str,
    /// "debug" or "release".
    pub profile: &'static str,
    pub target: &'static str,
    pub rustc: &'static str,
    pub git_commit: &'static str,
    /// Whether the working tree had uncommitted changes when this binary was built.
    pub git_dirty: bool,
}

/// Escape a string for embedding in a JSON string literal.
fn esc(v: &str) -> String {
    let mut out = String::with_capacity(v.len() + 2);
    for c in v.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

impl CaptureRecord {
    /// Render the record as JSON, merging in `extra` fields the caller owns.
    ///
    /// `extra` values are raw JSON, so a caller can pass an object or an array; string values must
    /// arrive already quoted.
    ///
    /// Deliberately hand-rolled rather than derived: this record must stay byte-identical between
    /// two runs, and it must never gain a timestamp field by accident.
    pub fn to_json(&self, build: &BuildInfo, extra: &[(&str, String)]) -> String {
        let mut out = String::from("{\n");
        out.push_str(&format!(
            "  \"build\": {{\n    \"crate\": \"{}\",\n    \"crate_version\": \"{}\",\n    \"engine_version\": \"{}\",\n    \"profile\": \"{}\",\n    \"target\": \"{}\",\n    \"rustc\": \"{}\",\n    \"git_commit\": \"{}\",\n    \"git_dirty\": {}\n  }},\n",
            esc(build.crate_name),
            esc(build.crate_version),
            esc(build.engine_version.trim()),
            esc(build.profile),
            esc(build.target),
            esc(build.rustc),
            esc(build.git_commit),
            build.git_dirty,
        ));
        out.push_str(&format!(
            "  \"adapter\": {{\n    \"name\": \"{}\",\n    \"backend\": \"{:?}\",\n    \"device_type\": \"{:?}\",\n    \"driver\": \"{}\",\n    \"driver_info\": \"{}\",\n    \"vendor_id\": {},\n    \"device_id\": {}\n  }},\n",
            esc(&self.adapter.name),
            self.adapter.backend,
            self.adapter.device_type,
            esc(&self.adapter.driver),
            esc(&self.adapter.driver_info),
            self.adapter.vendor,
            self.adapter.device,
        ));
        out.push_str(&format!(
            "  \"resolution\": {{\n    \"width\": {},\n    \"height\": {},\n    \"requested\": {}\n  }},\n",
            self.width,
            self.height,
            match self.requested_size {
                Some((w, h)) => format!("[{w}, {h}]"),
                None => "null".to_string(),
            },
        ));
        out.push_str(&format!(
            "  \"surface\": {{\n    \"format\": \"{:?}\",\n    \"present_mode\": \"{:?}\",\n    \"msaa_samples\": {}\n  }},\n",
            self.surface_format, self.present_mode, self.msaa_samples,
        ));
        out.push_str(&format!(
            "  \"device\": {{\n    \"enabled_features\": \"{:?}\",\n    \"validation_requested\": {}\n  }},\n",
            self.enabled_features, self.validation_requested,
        ));
        out.push_str(&format!(
            "  \"frame\": {{\n    \"warmup_frames\": {},\n    \"fixed_delta_s\": {},\n    \"total_drawcalls\": {},\n    \"total_triangles\": {}\n  }},\n",
            self.warmup_frames, self.fixed_delta, self.total_drawcalls, self.total_triangles,
        ));
        let passes: Vec<String> = self.passes.iter().map(|p| format!("\"{p}\"")).collect();
        out.push_str(&format!("  \"passes\": [{}],\n", passes.join(", ")));
        out.push_str(&format!(
            "  \"gpu_timing\": {{\n    \"status\": \"{}\",\n    \"units\": \"microseconds of GPU time per pass\",\n    \"passes\": [",
            esc(&self.gpu_timing_status),
        ));
        let timings: Vec<String> = self
            .gpu_timings
            .iter()
            .map(|t| {
                format!(
                    "\n      {{ \"pass\": \"{}\", \"samples\": {}, \"min_us\": {:.3}, \"median_us\": {:.3}, \"max_us\": {:.3} }}",
                    t.pass, t.samples, t.min_us, t.median_us, t.max_us
                )
            })
            .collect();
        if timings.is_empty() {
            out.push_str("]\n  }");
        } else {
            out.push_str(&timings.join(","));
            out.push_str("\n    ]\n  }");
        }
        for (k, v) in extra {
            out.push_str(&format!(",\n  \"{k}\": {v}"));
        }
        out.push_str("\n}\n");
        out
    }
}
