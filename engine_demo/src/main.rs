use common::history::History;
use engine::{
    AudioKind, Context, FrameContext, GfxSettings, InstancedMeshBuilder, Key, MeshInstance,
    MouseButton, ShadowQuality,
};
use geom::{vec3, Camera, Degrees, InfiniteFrustrum, LinearColor, Plane, Radians, Vec2, Vec3};

use crate::capture::CaptureScene;
use crate::helmet::Helmet;
use crate::spheres::Spheres;
use crate::terrain::Terrain;

mod capture;
mod helmet;
mod spheres;
mod terrain;

trait DemoElement {
    fn name(&self) -> &'static str;
    fn init(ctx: &mut Context) -> Self
    where
        Self: Sized;
    fn update(&mut self, ctx: &mut Context, cam: &Camera);
    fn render(&mut self, fc: &mut FrameContext, cam: &Camera);
    fn render_gui(&mut self, _ui: &mut egui::Ui) {}
}

struct State {
    demo_elements: Vec<(Box<dyn DemoElement>, bool)>,

    is_captured: bool,

    camera: Camera,
    camera_speed: f32,

    delta: f32,
    play_queue: Vec<&'static str>,

    ms_hist: History,

    gfx_settings: GfxSettings,
    sun_angle: Degrees,

    /// `Some` for a fixed capture run. While set, nothing may read the wall clock or the input.
    capture: Option<&'static CaptureScene>,
}

impl engine::framework::State for State {
    fn new(ctx: &mut Context) -> Self {
        let gfx = &mut ctx.gfx;

        let mut meshes = vec![];

        if let Ok(m) = gfx.mesh("DamagedHelmet.glb".as_ref()) {
            let mut i = InstancedMeshBuilder::<true>::new_ref(&m);
            i.instances.push(MeshInstance {
                pos: vec3(50.0, 00.0, 0.0),
                dir: Vec3::X,
                tint: LinearColor::WHITE,
            });
            meshes.push(i.build(gfx).unwrap());
        }

        let scene = crate::capture::args().map(|a| a.scene);

        let mut camera = match scene {
            Some(s) => Camera::new(s.cam_pos, s.width as f32, s.height as f32),
            None => Camera::new(vec3(9.0, -30.0, 13.0), 1000.0, 1000.0),
        };
        camera.dist = 0.0;
        camera.pitch = Radians(scene.map_or(0.0, |s| s.cam_pitch));
        camera.yaw = Radians(scene.map_or(-std::f32::consts::PI / 2.0, |s| s.cam_yaw));

        ctx.audio.set_settings(100.0, 100.0, 100.0, 100.0);

        let gfx_settings = match scene {
            Some(s) => s.settings,
            None => GfxSettings {
                shader_debug: true,
                ..Default::default()
            },
        };

        let mut demo_elements: Vec<(Box<dyn DemoElement>, bool)> = vec![
            (Box::new(Spheres::init(ctx)), true),
            (Box::new(Helmet::init(ctx)), true),
            (Box::new(Terrain::init(ctx)), true),
        ];
        // A scene names exactly which elements draw, so adding a new demo element cannot change
        // an existing capture.
        if let Some(s) = scene {
            for (de, enabled) in &mut demo_elements {
                *enabled = s.elements.contains(&de.name());
            }
        }

        Self {
            demo_elements,
            camera,
            is_captured: false,
            delta: 0.0,
            play_queue: vec![],
            camera_speed: 100.0,
            ms_hist: History::new(128),
            gfx_settings,
            sun_angle: Degrees(scene.map_or(0.0, |s| s.sun_angle_deg)),
            capture: scene,
        }
    }

    fn update(&mut self, ctx: &mut Context) {
        self.delta = ctx.delta;

        ctx.gfx.update_settings(self.gfx_settings);
        self.ms_hist.add_value(ctx.delta);

        // Capture runs never sample input: the camera is the scene's, not the mouse's. Input
        // events are already cut off in engine::framework, so this is belt and braces.
        let delta = match self.capture {
            Some(scene) => scene.delta,
            None => self.camera_movement(ctx),
        };

        let sun = Vec2::from_angle(self.sun_angle.into())
            .z0()
            .rotate_up(vec3(1.0, 0.0, 1.0).normalize())
            .normalize();

        let gfx = &mut ctx.gfx;

        self.camera.update();
        gfx.set_camera(self.camera);

        let params = gfx.render_params.value_mut();
        // Shader time is the other wall-clock path into a pixel: it accumulates delta and drives
        // anything animated. A capture pins it instead of advancing it.
        params.time_always = match self.capture {
            Some(scene) => scene.time,
            None => (params.time_always + delta) % 3600.0,
        };
        if let Some(scene) = self.capture {
            params.time = scene.time;
        }
        params.sun_col = 4.0
            * sun.z.max(0.0).sqrt().sqrt()
            * LinearColor::new(1.0, 0.95 + sun.z * 0.05, 0.95 + sun.z * 0.05, 1.0);
        params.cam_pos = self.camera.eye();
        params.cam_dir = self.camera.dir();
        params.sun = sun;
        params.viewport = Vec2::new(gfx.size.0 as f32, gfx.size.1 as f32);
        self.camera.dist = 300.0;
        params.sun_shadow_proj = self
            .camera
            .build_sun_shadowmap_matrix(
                sun,
                params.shadow_mapping_resolution as f32,
                &InfiniteFrustrum::new([Plane::X; 5]),
            )
            .try_into()
            .unwrap();
        self.camera.dist = 0.0;

        for (de, enabled) in &mut self.demo_elements {
            if !*enabled {
                continue;
            }
            de.update(ctx, &self.camera);
        }

        for v in self.play_queue.drain(..) {
            ctx.audio.play(v, AudioKind::Ui);
        }
    }

    fn render(&mut self, fc: &mut FrameContext) {
        for (de, enabled) in &mut self.demo_elements {
            if !*enabled {
                continue;
            }
            de.render(fc, &self.camera);
        }
    }

    fn on_capture(
        &mut self,
        record: &engine::capture::CaptureRecord,
        frame: Result<&engine::capture::CapturedFrame, &str>,
    ) {
        let Some(args) = crate::capture::args() else {
            return;
        };
        let enabled: Vec<(&str, bool)> = self
            .demo_elements
            .iter()
            .map(|(de, on)| (de.name(), *on))
            .collect();

        let frame = match frame {
            Ok(f) => f,
            Err(reason) => {
                // A concrete reason, not a panic. Nothing else in the process is disturbed.
                eprintln!("capture FAILED: {reason}");
                eprintln!(
                    "adapter: {} ({:?})",
                    record.adapter.name, record.adapter.backend
                );
                std::process::exit(2);
            }
        };

        match crate::capture::write_outputs(args, record, frame, &enabled) {
            Ok((png, json)) => {
                println!("capture ok");
                println!("  image:  {}", png.display());
                println!("  record: {}", json.display());
                println!(
                    "  adapter: {} ({:?}, {:?})",
                    record.adapter.name, record.adapter.backend, record.adapter.device_type
                );
                println!("  resolution: {}x{}", record.width, record.height);
                println!("  passes: {}", record.passes.join(", "));
                println!("  gpu timing: {}", record.gpu_timing_status);
                for t in &record.gpu_timings {
                    println!(
                        "    {:<18} n={:<3} min={:>8.1}us median={:>8.1}us max={:>8.1}us",
                        t.pass, t.samples, t.min_us, t.median_us, t.max_us
                    );
                }
            }
            Err(e) => {
                eprintln!("capture FAILED while writing output: {e}");
                std::process::exit(2);
            }
        }
    }

    fn resized(&mut self, _: &mut Context, size: (u32, u32, f64)) {
        self.camera.set_viewport(size.0 as f32, size.1 as f32);
    }

    fn render_gui(&mut self, ui: &egui::Context) {
        // The settings window prints a frames-per-second average. Drawing it would put a
        // wall-clock number into the captured pixels.
        if self.capture.is_some() {
            return;
        }

        egui::Window::new("Demo elements")
            .resizable(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.sun_angle.0, 0.0..=360.0).text("Sun angle"));

                for (de, enabled) in &mut self.demo_elements {
                    ui.checkbox(enabled, de.name());
                    de.render_gui(ui);
                }

                if ui.button("play sound: road_lay").clicked() {
                    self.play_queue.push("road_lay");
                }
            });

        egui::Window::new("Settings")
            .resizable(true)
            .show(ui, |ui| {
                let avg_ms = self.ms_hist.avg();
                ui.label(format!(
                    "Avg (128 frames): {:.1}ms {:.0}FPS",
                    1000.0 * avg_ms,
                    1.0 / avg_ms
                ));

                ui.add(egui::Slider::new(&mut self.camera_speed, 1.0..=100.0).text("Camera speed"));

                ui.checkbox(&mut self.gfx_settings.fullscreen, "Fullscreen");
                ui.checkbox(&mut self.gfx_settings.vsync, "VSync");
                ui.checkbox(&mut self.gfx_settings.fog, "Fog");
                ui.checkbox(&mut self.gfx_settings.ssao, "SSAO");
                ui.checkbox(&mut self.gfx_settings.terrain_grid, "Terrain grid");
                ui.checkbox(&mut self.gfx_settings.parallel_render, "Threaded rendering");

                let mut shadows = self.gfx_settings.shadows.size().is_some();
                ui.checkbox(&mut shadows, "Shadows");
                self.gfx_settings.shadows = if shadows {
                    ShadowQuality::High
                } else {
                    ShadowQuality::NoShadows
                };

                ui.checkbox(&mut self.gfx_settings.shader_debug, "Shader debug");
                ui.checkbox(&mut self.gfx_settings.pbr_enabled, "PBR Environment Update");
            });
    }
}

impl State {
    fn camera_movement(&mut self, ctx: &mut Context) -> f32 {
        if ctx.input.mouse.pressed.contains(&MouseButton::Left) {
            let _ = ctx
                .gfx
                .window()
                .set_cursor_grab(engine::CursorGrabMode::Confined);
            ctx.gfx.window().set_cursor_visible(false);
            self.is_captured = true;
        }

        if ctx.input.cursor_left {
            let _ = ctx
                .gfx
                .window()
                .set_cursor_grab(engine::CursorGrabMode::None);
            ctx.gfx.window().set_cursor_visible(true);
            self.is_captured = false;
        }

        if ctx.input.keyboard.pressed.contains(&Key::Escape) {
            let _ = ctx
                .gfx
                .window()
                .set_cursor_grab(engine::CursorGrabMode::None);
            ctx.gfx.window().set_cursor_visible(true);
            self.is_captured = false;
        }

        let delta = ctx.delta;
        let cam_speed = if ctx.input.keyboard.pressed_scancode.contains(&42) {
            3.0
        } else {
            30.0
        } * delta
            * self.camera_speed;

        if ctx.input.keyboard.pressed_scancode.contains(&17) {
            self.camera.pos -= self
                .camera
                .dir()
                .xy()
                .z0()
                .try_normalize()
                .unwrap_or(Vec3::ZERO)
                * cam_speed;
        }
        if ctx.input.keyboard.pressed_scancode.contains(&31) {
            self.camera.pos += self
                .camera
                .dir()
                .xy()
                .z0()
                .try_normalize()
                .unwrap_or(Vec3::ZERO)
                * cam_speed;
        }
        if ctx.input.keyboard.pressed_scancode.contains(&30) {
            self.camera.pos += self
                .camera
                .dir()
                .perp_up()
                .try_normalize()
                .unwrap_or(Vec3::ZERO)
                * cam_speed;
        }
        if ctx.input.keyboard.pressed_scancode.contains(&32) {
            self.camera.pos -= self
                .camera
                .dir()
                .perp_up()
                .try_normalize()
                .unwrap_or(Vec3::ZERO)
                * cam_speed;
        }
        if ctx.input.keyboard.pressed_scancode.contains(&57) {
            self.camera.pos += vec3(0.0, 0.0, 0.5) * cam_speed;
        }
        if ctx.input.keyboard.pressed_scancode.contains(&29) {
            self.camera.pos -= vec3(0.0, 0.0, 0.5) * cam_speed;
        }

        if self.is_captured {
            let delta = ctx.input.mouse.screen_delta;

            self.camera.yaw.0 -= 0.001 * delta.x;
            self.camera.pitch.0 += 0.001 * delta.y;
            self.camera.pitch.0 = self.camera.pitch.0.clamp(-1.5, 1.5);
        }
        delta
    }
}

fn main() {
    let args = match crate::capture::parse_args(std::env::args()) {
        Ok(v) => v,
        // --help and --list-scenes arrive here too; they are not failures.
        Err(msg) => {
            println!("{msg}");
            return;
        }
    };

    engine::framework::init();

    let Some(args) = args else {
        // Interactive demo, exactly as before.
        engine::framework::start::<State>();
        return;
    };

    let opts = engine::framework::FrameworkOptions {
        fixed_size: Some((args.scene.width, args.scene.height)),
        fixed_delta: Some(args.scene.delta),
        freeze_input: true,
        validation: args.validation,
        capture: Some(engine::framework::CaptureOptions {
            warmup_frames: args.scene.warmup_frames,
            gpu_timing_samples: args.gpu_timings.then_some(args.gpu_samples),
        }),
    };
    crate::capture::set_args(args);
    engine::framework::start_with_options::<State>(opts);
}
