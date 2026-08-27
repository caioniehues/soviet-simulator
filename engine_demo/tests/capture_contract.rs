//! Guards for the fixed-capture contract (sov-uy2).
//!
//! These cover the CPU-side parts of the capture path that can be wrong while still producing a
//! picture that looks plausible. The GPU-side parts are proved by running the capture twice and
//! comparing the bytes; see the ticket for that command.

use engine::capture::{unpad_and_swizzle, Swizzle};

/// `copy_texture_to_buffer` forces `bytes_per_row` to a multiple of
/// `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT`, so every row read back carries trailing bytes that are
/// not pixels. Keeping them shears the image. Keeping BGRA order swaps red and blue. Both failures
/// still yield a viewable PNG, which is exactly why they need a guard.
#[test]
fn unpad_and_swizzle_drops_padding_and_orders_channels() {
    // 2 pixels per row = 8 real bytes, padded out to 12 with 4 bytes of junk.
    let padded_row = 12;
    let mapped: Vec<u8> = vec![
        // row 0: two pixels, then padding
        1, 2, 3, 4, 5, 6, 7, 8, 0xAA, 0xAA, 0xAA, 0xAA, //
        // row 1: two pixels, then padding
        9, 10, 11, 12, 13, 14, 15, 16, 0xBB, 0xBB, 0xBB, 0xBB,
    ];

    let rgba = unpad_and_swizzle(&mapped, 2, 2, padded_row, Swizzle::Rgba);
    assert_eq!(
        rgba,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        "an RGBA source should only lose its row padding"
    );

    let bgra = unpad_and_swizzle(&mapped, 2, 2, padded_row, Swizzle::Bgra);
    assert_eq!(
        bgra,
        vec![3, 2, 1, 4, 7, 6, 5, 8, 11, 10, 9, 12, 15, 14, 13, 16],
        "a BGRA source should lose its padding and have R and B exchanged"
    );
}

/// The default framework options are the interactive demo: wall-clock delta, live input, no
/// capture, no forced window size. If any of these flip, the normal demo silently changes
/// behaviour, which the ticket forbids.
#[test]
fn default_framework_options_are_the_interactive_contract() {
    let opts = engine::framework::FrameworkOptions::default();
    assert_eq!(
        opts.fixed_size, None,
        "interactive runs size to the monitor"
    );
    assert_eq!(
        opts.fixed_delta, None,
        "interactive runs use wall-clock delta"
    );
    assert!(!opts.freeze_input, "interactive runs sample input");
    assert!(!opts.validation, "validation layers are opt-in");
    assert!(opts.capture.is_none(), "capture is opt-in");
}

/// The GPU-timing slot table is the thing that decides which number gets which pass name in the
/// evidence file. If a name and its query slot ever drift apart, the record still looks complete
/// and every figure is attributed to the wrong pass.
#[test]
fn gpu_pass_names_match_their_query_slots() {
    use engine::gpu_timing::{GpuPass, GPU_PASS_NAMES, N_GPU_PASSES};

    let all = [
        GpuPass::DepthPrepass,
        GpuPass::ShadowCascade0,
        GpuPass::ShadowCascade1,
        GpuPass::ShadowCascade2,
        GpuPass::ShadowCascade3,
        GpuPass::Ssao,
        GpuPass::Fog,
        GpuPass::Main,
        GpuPass::Background,
    ];
    assert_eq!(all.len(), N_GPU_PASSES, "every pass must have a slot");

    for (slot, pass) in all.iter().enumerate() {
        assert_eq!(
            *pass as usize,
            slot,
            "{} must occupy slot {slot}",
            pass.name()
        );
        assert_eq!(
            pass.name(),
            GPU_PASS_NAMES[slot],
            "name table and slot disagree at {slot}"
        );
    }

    let mut names = GPU_PASS_NAMES.to_vec();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), N_GPU_PASSES, "pass names must be unique");

    // Cascade lookup must land on the matching cascade slot, not merely on some shadow pass.
    for i in 0..4 {
        assert_eq!(
            GpuPass::shadow_cascade(i).map(|p| p as usize),
            Some(i + 1),
            "cascade {i} maps to the wrong slot"
        );
    }
    assert!(
        GpuPass::shadow_cascade(4).is_none(),
        "only 4 cascades exist"
    );
}

/// The whole point of a capture record is that a later reader can tell which machine and build
/// produced the frame. If a required field silently stops being written, the record still looks
/// complete and the capture stops being evidence.
///
/// It must also be byte-identical for identical input: two captures are only comparable if their
/// records are, so nothing wall-clock may leak into the file.
#[test]
fn capture_record_json_is_complete_and_deterministic() {
    let record = engine::capture::CaptureRecord {
        adapter: engine::wgpu::AdapterInfo {
            name: "AMD Radeon RX 7800 XT (RADV NAVI32)".to_string(),
            vendor: 0x1002,
            device: 0x747e,
            device_type: engine::wgpu::DeviceType::DiscreteGpu,
            driver: "radv".to_string(),
            driver_info: "Mesa 25.2.1".to_string(),
            backend: engine::wgpu::Backend::Vulkan,
        },
        enabled_features: engine::wgpu::Features::empty(),
        width: 1280,
        height: 720,
        requested_size: Some((1280, 720)),
        surface_format: engine::wgpu::TextureFormat::Bgra8UnormSrgb,
        present_mode: engine::wgpu::PresentMode::Fifo,
        msaa_samples: 1,
        validation_requested: false,
        passes: vec!["depth_prepass", "main"],
        warmup_frames: 90,
        fixed_delta: 1.0 / 60.0,
        total_drawcalls: 52,
        total_triangles: 123456,
        gpu_timings: Vec::new(),
        gpu_timing_status: "disabled: opt-in gate off".to_string(),
    };

    let build = engine::capture::BuildInfo {
        crate_name: "engine_demo",
        crate_version: "0.1.0",
        engine_version: "0.6.1",
        profile: "debug",
        target: "x86_64-unknown-linux-gnu",
        rustc: "rustc 1.89.0",
        git_commit: "2550026",
        git_dirty: true,
    };
    let json = record.to_json(&build, &[("scene", "\"baseline\"".to_string())]);

    // The four things sov-uy2 requires the run to record.
    for key in [
        "\"adapter\"",    // which GPU
        "\"resolution\"", // what size
        "\"build\"",      // which build
        "\"passes\"",     // what actually rendered
    ] {
        assert!(json.contains(key), "record is missing {key}:\n{json}");
    }
    assert!(
        json.contains("AMD Radeon RX 7800 XT (RADV NAVI32)"),
        "adapter name must appear verbatim:\n{json}"
    );
    assert!(
        json.contains("\"scene\": \"baseline\""),
        "caller fields must be merged in:\n{json}"
    );
    assert!(
        json.contains("\"git_commit\": \"2550026\""),
        "build provenance must appear:\n{json}"
    );

    let again = record.to_json(&build, &[("scene", "\"baseline\"".to_string())]);
    assert_eq!(
        json, again,
        "an identical record must serialise identically"
    );
    assert!(
        !json.contains("timestamp") && !json.contains("captured_at"),
        "no wall-clock field may enter the record, or two captures can never match:\n{json}"
    );
}
