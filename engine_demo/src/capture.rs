//! The fixed capture scene (sov-uy2).
//!
//! A scene is a *contract*: it pins everything that can change a pixel, so two runs of the same
//! named scene on the same machine and build produce the same bytes. The pins are
//!
//! - camera position and orientation, and the window size that fixes the projection,
//! - the frame delta and the shader time, so nothing animates with the wall clock,
//! - input, cut off in `engine::framework` so neither the camera nor the terrain raycast can move,
//! - which demo elements draw, and the graphics settings that decide which passes run,
//! - the GUI, which is not drawn at all: it shows a frames-per-second readout.
//!
//! Anything outside that list — the output directory, the GPU timing gate, validation — is
//! recorded in the sidecar rather than being allowed to change the frame.

use std::path::PathBuf;
use std::sync::OnceLock;

use engine::capture::{BuildInfo, CaptureRecord, CapturedFrame};
use engine::{GfxSettings, ShadowQuality};
use geom::{vec3, Vec3};

/// How this binary was built. Values come from `build.rs`.
pub const BUILD: BuildInfo = BuildInfo {
    crate_name: env!("CARGO_PKG_NAME"),
    crate_version: env!("CARGO_PKG_VERSION"),
    engine_version: include_str!("../../VERSION"),
    profile: env!("SOV_PROFILE"),
    target: env!("SOV_TARGET"),
    rustc: env!("SOV_RUSTC"),
    git_commit: env!("SOV_GIT_COMMIT"),
    git_dirty: matches!(env!("SOV_GIT_DIRTY").as_bytes(), b"true"),
};

/// A named, fully pinned scene.
pub struct CaptureScene {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    /// Frames drawn before the captured one, so shader compilation and mipmap generation settle.
    pub warmup_frames: u32,
    /// Fixed per-frame delta in seconds. Nothing may read the wall clock.
    pub delta: f32,
    /// Fixed value for both shader time uniforms.
    pub time: f32,
    pub cam_pos: Vec3,
    pub cam_yaw: f32,
    pub cam_pitch: f32,
    pub sun_angle_deg: f32,
    /// Demo elements to draw, by their `DemoElement::name`. Anything absent is switched off.
    pub elements: &'static [&'static str],
    pub settings: GfxSettings,
}

/// Graphics settings for the baseline scene.
///
/// Spelled out field by field rather than built from `Default`, so a change to the interactive
/// defaults cannot silently move the capture contract.
const BASELINE_SETTINGS: GfxSettings = GfxSettings {
    vsync: true,
    fullscreen: false,
    shadows: ShadowQuality::High,
    fog: true,
    ssao: true,
    terrain_grid: true,
    shader_debug: false,
    pbr_enabled: true,
    fog_shader_debug: false,
    // Off so command buffers are recorded in one fixed order.
    parallel_render: false,
    msaa: false,
};

pub static SCENES: &[CaptureScene] = &[CaptureScene {
    name: "baseline",
    width: 1280,
    height: 720,
    warmup_frames: 90,
    delta: 1.0 / 60.0,
    time: 0.0,
    // The interactive demo's opening viewpoint, kept as-is so the capture shows what the demo
    // has always shown.
    cam_pos: vec3(9.0, -30.0, 13.0),
    cam_yaw: -std::f32::consts::FRAC_PI_2,
    cam_pitch: 0.0,
    sun_angle_deg: 0.0,
    elements: &["Spheres", "Helmet", "Terrain"],
    settings: BASELINE_SETTINGS,
}];

pub fn scene_by_name(name: &str) -> Option<&'static CaptureScene> {
    SCENES.iter().find(|s| s.name == name)
}

/// Everything the capture run was asked to do, outside the scene contract.
pub struct CaptureArgs {
    pub scene: &'static CaptureScene,
    pub out_dir: PathBuf,
    pub gpu_timings: bool,
    pub gpu_samples: u32,
    pub validation: bool,
}

static ARGS: OnceLock<CaptureArgs> = OnceLock::new();

/// The active capture run, or `None` for an ordinary interactive run.
pub fn args() -> Option<&'static CaptureArgs> {
    ARGS.get()
}

pub fn set_args(a: CaptureArgs) {
    let _ = ARGS.set(a);
}

pub const USAGE: &str = "\
engine_demo — interactive renderer demo, and its fixed capture mode.

USAGE:
    engine_demo                       run the interactive demo (unchanged)
    engine_demo capture [OPTIONS]     render one pinned frame and exit

CAPTURE OPTIONS:
    --scene <name>      named scene to render (default: baseline)
    --out <dir>         where to write the capture (default: target/renderer-evidence)
    --gpu-timings       opt in to per-pass GPU timestamps (default: OFF)
    --gpu-samples <n>   frames to sample timings over (default: 30)
    --validation        ask wgpu for its validation layers (default: OFF)
    --list-scenes       print the known scenes and exit
    -h, --help          print this message

The scene fixes camera, window size, frame delta, shader time, input and enabled drawables.
None of those are settable from the command line: a capture whose contract can be changed per
run is not a capture contract.";

/// Parse `capture` arguments. `Ok(None)` means run the interactive demo.
pub fn parse_args<I: Iterator<Item = String>>(mut argv: I) -> Result<Option<CaptureArgs>, String> {
    let _exe = argv.next();
    let Some(first) = argv.next() else {
        return Ok(None);
    };
    match first.as_str() {
        "-h" | "--help" => return Err(USAGE.to_string()),
        "--list-scenes" => {
            return Err(SCENES
                .iter()
                .map(|s| {
                    format!(
                        "{} ({}x{}, {} warmup frames)",
                        s.name, s.width, s.height, s.warmup_frames
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"))
        }
        "capture" => {}
        other => return Err(format!("unknown argument '{other}'\n\n{USAGE}")),
    }

    let mut scene_name = "baseline".to_string();
    let mut out_dir = PathBuf::from("target/renderer-evidence");
    let mut gpu_timings = false;
    let mut gpu_samples = 30u32;
    let mut validation = false;

    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "--scene" => scene_name = argv.next().ok_or("--scene needs a name")?,
            "--out" => out_dir = PathBuf::from(argv.next().ok_or("--out needs a directory")?),
            "--gpu-timings" => gpu_timings = true,
            "--validation" => validation = true,
            "--gpu-samples" => {
                gpu_samples = argv
                    .next()
                    .ok_or("--gpu-samples needs a number")?
                    .parse()
                    .map_err(|e| format!("--gpu-samples: {e}"))?;
            }
            "-h" | "--help" => return Err(USAGE.to_string()),
            other => return Err(format!("unknown capture option '{other}'\n\n{USAGE}")),
        }
    }

    if gpu_samples == 0 {
        return Err("--gpu-samples must be at least 1".to_string());
    }
    let scene = scene_by_name(&scene_name).ok_or_else(|| {
        let known: Vec<&str> = SCENES.iter().map(|s| s.name).collect();
        format!(
            "unknown scene '{scene_name}'; known scenes: {}",
            known.join(", ")
        )
    })?;

    Ok(Some(CaptureArgs {
        scene,
        out_dir,
        gpu_timings,
        gpu_samples,
        validation,
    }))
}

/// Scene-level facts, merged into the engine's record so one file describes the whole contract.
fn scene_fields(args: &CaptureArgs, enabled: &[(&str, bool)]) -> Vec<(&'static str, String)> {
    let s = args.scene;
    let drawables: Vec<String> = enabled
        .iter()
        .map(|(n, on)| format!("{{ \"name\": \"{n}\", \"enabled\": {on} }}"))
        .collect();
    vec![
        ("scene", format!("\"{}\"", s.name)),
        (
            "fixed_inputs",
            format!(
                "{{ \"camera_pos\": [{}, {}, {}], \"camera_yaw_rad\": {}, \"camera_pitch_rad\": {}, \
                 \"sun_angle_deg\": {}, \"shader_time_s\": {}, \"input\": \"frozen: no window or \
                 device events are delivered\" }}",
                s.cam_pos.x, s.cam_pos.y, s.cam_pos.z, s.cam_yaw, s.cam_pitch, s.sun_angle_deg, s.time
            ),
        ),
        ("drawables", format!("[{}]", drawables.join(", "))),
        (
            "gpu_timing_policy",
            format!(
                "{{ \"requested\": {}, \"sample_frames\": {}, \"policy\": \"one sample per armed \
                 frame, on the {} frames ending with the captured frame; min/median/max reported \
                 per pass\", \"instrumented_passes\": \"frame-level passes only; drawable, egui, \
                 pbr, blur and mipmap passes are not instrumented\" }}",
                args.gpu_timings, args.gpu_samples, args.gpu_samples
            ),
        ),
    ]
}

/// Write the PNG and its sidecar record. Returns the paths written.
pub fn write_outputs(
    args: &CaptureArgs,
    record: &CaptureRecord,
    frame: &CapturedFrame,
    enabled: &[(&str, bool)],
) -> Result<(PathBuf, PathBuf), String> {
    let png = args.out_dir.join(format!("{}.png", args.scene.name));
    let json = args.out_dir.join(format!("{}.json", args.scene.name));
    frame.write_png(&png)?;
    let fields = scene_fields(args, enabled);
    let borrowed: Vec<(&str, String)> = fields.into_iter().collect();
    std::fs::write(&json, record.to_json(&BUILD, &borrowed))
        .map_err(|e| format!("could not write {}: {e}", json.display()))?;
    Ok((png, json))
}
