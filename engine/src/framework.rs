use rayon::ThreadPoolBuilder;
use std::sync::Arc;
use std::time::Instant;

use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::capture::{CaptureRecord, CapturedFrame};
use crate::egui::EguiWrapper;
use crate::{get_cursor_icon, AudioContext, FrameContext, GfxContext, GfxOptions, InputContext};

/// How to run the frame loop. Every field defaults to the interactive behaviour the game already
/// had, so `FrameworkOptions::default()` is exactly the old `start`.
///
/// A capture is only evidence if the run is pinned. Fixing the camera is not enough: wall-clock
/// delta feeds shader time, and live input moves the camera and the terrain raycast. These options
/// are where both are cut off, at the source, so no caller can forget one.
#[derive(Clone, Debug, Default)]
pub struct FrameworkOptions {
    /// Window inner size in physical pixels. `None` sizes to 80% of the monitor, as before.
    pub fixed_size: Option<(u32, u32)>,
    /// Per-frame delta in seconds. `None` uses the wall clock, as before.
    pub fixed_delta: Option<f32>,
    /// Stop feeding window and device events to input and egui. Nothing the mouse or keyboard
    /// does can then reach a pixel.
    pub freeze_input: bool,
    /// Ask wgpu for its validation layers.
    pub validation: bool,
    /// Render a fixed number of frames, hand the last one to [`State::on_capture`], then exit.
    pub capture: Option<CaptureOptions>,
}

/// A fixed capture: render `warmup_frames` frames, then capture frame number `warmup_frames`.
#[derive(Clone, Debug)]
pub struct CaptureOptions {
    /// Frames drawn before the captured one. They let shader compilation, mipmap generation and
    /// streaming settle, so the captured frame is not the first-frame special case.
    pub warmup_frames: u32,
    /// How many frames to sample GPU timings over, ending on the captured frame. `None` leaves
    /// the timestamp gate off, which is the default.
    ///
    /// Must not exceed `warmup_frames + 1`: a wider window does not exist, and saturating down
    /// to frame 0 would both drag cold start-up frames into the timings and report a count the
    /// run did not take (sov-h4y). The `engine_demo` CLI rejects larger requests with this bound.
    pub gpu_timing_samples: Option<u32>,
}

/// First frame of the GPU-timing sample window: the `samples` frames ending with the captured
/// frame (`warmup_frames`).
///
/// Shared by the windowed (wasm) loop in `run` and the offscreen loop in `run_offscreen`, so the
/// two paths cannot arm different windows (sov-j3p). Callers must keep `samples` within
/// `warmup_frames + 1`; anything larger saturates to frame 0, which is exactly the cold-frame
/// contamination the bound on [`CaptureOptions::gpu_timing_samples`] exists to prevent.
fn gpu_timing_window_first_frame(warmup_frames: u32, samples: u32) -> u32 {
    warmup_frames.saturating_sub(samples.saturating_sub(1))
}

impl FrameworkOptions {
    pub fn requires_window(&self) -> bool {
        self.capture.is_none()
    }
}

#[allow(unused_variables)]
pub trait State: 'static {
    fn new(ctx: &mut Context) -> Self;

    /// Called every frame to update the game state.
    fn update(&mut self, ctx: &mut Context);

    /// Called every frame to prepare the game rendering.
    fn render(&mut self, fc: &mut FrameContext);

    /// Called when the window is resized.
    fn resized(&mut self, ctx: &mut Context, size: (u32, u32, f64)) {}

    /// Called when the window asks to exit (e.g ALT+F4) to be able to control the flow, for example to ask "save before exit?".
    /// Return true to exit, false to cancel.
    fn exit(&mut self) -> bool {
        true
    }

    /// Called every frame to prepare the gui rendering.
    fn render_gui(&mut self, ui: &egui::Context) {}

    /// Called once, on the frame a fixed capture was asked for, before the process exits.
    ///
    /// `frame` carries the reason on failure rather than a panic, so a capture that cannot be
    /// taken reports why instead of bringing the process down. The default does nothing, which is
    /// what every non-capturing state wants.
    fn on_capture(&mut self, record: &CaptureRecord, frame: Result<&CapturedFrame, &str>) {}

    /// Called every frame to prepare the gui rendering.
    #[cfg(feature = "yakui")]
    fn render_yakui(&mut self) {}
}

async fn run<S: State>(el: EventLoop<()>, window: Arc<Window>, opts: FrameworkOptions) {
    let gfx_opts = GfxOptions {
        validation: opts.validation,
        allow_capture: opts.capture.is_some(),
        gpu_timings: opts
            .capture
            .as_ref()
            .is_some_and(|c| c.gpu_timing_samples.is_some()),
    };
    let mut ctx = Context::new(window, &el, gfx_opts).await;
    let mut state = S::new(&mut ctx);
    ctx.gfx.defines_changed = false;

    let mut scale_factor = ctx.gfx.window().scale_factor();
    log::info!("initial scale factor: {:?}", scale_factor);
    let mut last_update = Instant::now();
    let mut frame_ix: u32 = 0;

    el.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);

        if let Event::WindowEvent { event, .. } = &event {
            if !opts.freeze_input {
                ctx.egui.handle_event(ctx.gfx.window(), event);
            }
        }

        #[cfg(feature = "yakui")]
        if ctx.yakui.handle_event(&event) && !ctx.keybind_mode {
            return;
        }

        match event {
            Event::DeviceEvent { event, .. } => {
                if !opts.freeze_input {
                    ctx.input.handle_device(&event);
                }
            }
            Event::WindowEvent { event, .. } => {
                if !opts.freeze_input {
                    ctx.input.handle(&event);
                }

                if ctx.gfx.update_sc {
                    ctx.gfx.update_sc = false;
                    let size = (ctx.gfx.size.0, ctx.gfx.size.1, scale_factor);
                    ctx.gfx.resize(size);
                    state.resized(&mut ctx, size);
                }

                match event {
                    WindowEvent::Resized(physical_size) => {
                        log::info!("resized: {:?}", physical_size);
                        let size = (physical_size.width, physical_size.height, scale_factor);
                        ctx.gfx.resize(size);
                        state.resized(&mut ctx, size);
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor: sf, ..
                    } => {
                        log::info!("scale_factor: {:?}", scale_factor);
                        scale_factor = sf;
                        let size = (ctx.gfx.size.0, ctx.gfx.size.1, scale_factor);
                        ctx.gfx.resize(size);
                        state.resized(&mut ctx, size);
                    }
                    WindowEvent::CloseRequested => {
                        if state.exit() {
                            target.exit();
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        let sco = match ctx.gfx.surface().get_current_texture() {
                            Ok(swapchainframe) => swapchainframe,
                            Err(wgpu::SurfaceError::Timeout) => ctx
                                .gfx
                                .surface()
                                .get_current_texture()
                                .expect("Failed to acquire next swap chain texture after timeout"),
                            Err(wgpu::SurfaceError::Outdated)
                            | Err(wgpu::SurfaceError::Lost)
                            | Err(wgpu::SurfaceError::OutOfMemory) => {
                                let size = ctx.gfx.size;
                                ctx.gfx.resize(size);
                                state.resized(&mut ctx, size);
                                ctx.gfx
                                    .surface()
                                    .get_current_texture()
                                    .expect("Failed to acquire next swap chain texture after losing surface")
                            }
                        };

                        profiling::finish_frame!();
                        profiling::scope!("frame");
                        let d = last_update.elapsed();
                        last_update = Instant::now();
                        ctx.delta = opts.fixed_delta.unwrap_or_else(|| d.as_secs_f32());

                        // Arm the timestamp queries only for the sampled tail, so warm-up frames
                        // and their shader compilation never enter the timings.
                        if let (Some(cap), Some(t)) = (&opts.capture, &ctx.gfx.gpu_timings) {
                            if let Some(samples) = cap.gpu_timing_samples {
                                let first =
                                    gpu_timing_window_first_frame(cap.warmup_frames, samples);
                                t.set_armed(frame_ix >= first && frame_ix <= cap.warmup_frames);
                            }
                        }

                        state.update(&mut ctx);

                        let (mut enc, view) = ctx.gfx.start_frame(&sco.texture);
                        (ctx.times.render_time, ctx.times.gui_time) = ctx
                            .gfx
                            .render(&mut enc, &view, &mut state, |state, mut gctx| {
                                #[cfg(feature = "yakui")]
                                ctx.yakui.render(&mut gctx, || {
                                    state.render_yakui();
                                });
                                ctx.egui.render(gctx, |ui| {
                                    state.render_gui(ui);
                                });
                            });

                        ctx.gfx.finish_frame(enc);

                        if let Some(t) = &ctx.gfx.gpu_timings {
                            t.collect_frame(&ctx.gfx.device, &ctx.gfx.queue);
                        }

                        // Windowed capture-and-exit, wasm32-only (sov-d3a, sov-j3p). On native,
                        // `start_with_options` routes every capture to `run_offscreen`, so this
                        // block is unreachable there and is compiled out: there is no `--windowed`
                        // flag because a windowed comparison was never reproducible on demand
                        // (the surface path negotiates its own format, e.g. Bgra, while the
                        // offscreen record pins Rgba). The single live caller is the wasm branch
                        // of `start_with_options`, which spawns `run` unguarded. It cannot share
                        // `run_offscreen`'s implementation: this path reads back a swapchain
                        // surface texture and presents it to a window, while offscreen renders to
                        // a headless target texture with no surface at all.
                        #[cfg(target_arch = "wasm32")]
                        if let Some(cap) = &opts.capture {
                            if frame_ix >= cap.warmup_frames {
                                let record = ctx.gfx.capture_record(
                                    opts.fixed_size,
                                    cap.warmup_frames,
                                    opts.fixed_delta.unwrap_or(0.0),
                                    opts.validation,
                                );
                                match crate::capture::capture_texture(
                                    &ctx.gfx.device,
                                    &ctx.gfx.queue,
                                    &sco.texture,
                                ) {
                                    Ok(frame) => state.on_capture(&record, Ok(&frame)),
                                    Err(e) => {
                                        log::error!("capture failed: {e}");
                                        state.on_capture(&record, Err(e.as_str()));
                                    }
                                }
                                sco.present();
                                target.exit();
                                return;
                            }
                        }
                        frame_ix += 1;

                        let (icon, changed) = get_cursor_icon();
                        if changed {
                            ctx.gfx.window().set_cursor_icon(icon);
                        }
                        ctx.input.end_frame();
                        ctx.times.total_cpu_time = last_update.elapsed().as_secs_f32();

                        sco.present();
                        ctx.gfx.window().request_redraw();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    })
    .expect("Failed to run event loop");
}

#[cfg(not(target_arch = "wasm32"))]
async fn run_offscreen<S: State>(opts: FrameworkOptions) {
    let cap = opts
        .capture
        .clone()
        .expect("offscreen rendering requires capture options");
    let (width, height) = opts
        .fixed_size
        .expect("offscreen capture requires a fixed size");
    let gfx_opts = GfxOptions {
        validation: opts.validation,
        allow_capture: true,
        gpu_timings: cap.gpu_timing_samples.is_some(),
    };
    let mut ctx = Context::new_offscreen(width, height, gfx_opts).await;
    let mut state = S::new(&mut ctx);
    ctx.gfx.defines_changed = false;
    let size = (width, height, 1.0);
    state.resized(&mut ctx, size);
    let target = ctx.gfx.create_offscreen_target();

    for frame_ix in 0..=cap.warmup_frames {
        ctx.delta = opts.fixed_delta.unwrap_or(0.0);
        if let (Some(samples), Some(timings)) = (cap.gpu_timing_samples, &ctx.gfx.gpu_timings) {
            let first = gpu_timing_window_first_frame(cap.warmup_frames, samples);
            timings.set_armed(frame_ix >= first && frame_ix <= cap.warmup_frames);
        }

        state.update(&mut ctx);
        let (mut enc, view) = ctx.gfx.start_frame(&target.texture);
        (ctx.times.render_time, ctx.times.gui_time) =
            ctx.gfx
                .render(&mut enc, &view, &mut state, |state, mut gctx| {
                    #[cfg(feature = "yakui")]
                    ctx.yakui.render(&mut gctx, || {
                        state.render_yakui();
                    });
                    ctx.egui.render(gctx, |ui| {
                        state.render_gui(ui);
                    });
                });
        ctx.gfx.finish_frame(enc);
        if let Some(timings) = &ctx.gfx.gpu_timings {
            timings.collect_frame(&ctx.gfx.device, &ctx.gfx.queue);
        }
    }

    let record = ctx.gfx.capture_record(
        opts.fixed_size,
        cap.warmup_frames,
        opts.fixed_delta.unwrap_or(0.0),
        opts.validation,
    );
    match crate::capture::capture_texture(&ctx.gfx.device, &ctx.gfx.queue, &target.texture) {
        Ok(frame) => state.on_capture(&record, Ok(&frame)),
        Err(error) => {
            log::error!("capture failed: {error}");
            state.on_capture(&record, Err(error.as_str()));
        }
    }
}

pub fn init() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("Failed to initialize logger");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        common::logger::MyLog::init();
    }
}

pub fn start<S: State>() {
    start_with_options::<S>(FrameworkOptions::default())
}

pub fn start_with_options<S: State>(opts: FrameworkOptions) {
    let _ = ThreadPoolBuilder::new().num_threads(8).build_global();
    #[cfg(not(target_arch = "wasm32"))]
    if !opts.requires_window() {
        beul::execute(run_offscreen::<S>(opts));
        return;
    }
    let el = EventLoop::new().expect("Failed to create event loop");

    #[cfg(target_arch = "wasm32")]
    {
        let window = WindowBuilder::new()
            .with_transparent(true)
            .with_title("Egregoria")
            .with_inner_size(winit::dpi::PhysicalSize {
                width: 1422,
                height: 700,
            })
            .build(&el)
            .unwrap();

        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("Failed to append canvas to body");
        wasm_bindgen_futures::spawn_local(run(el, Arc::new(window), opts));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let size = match el.primary_monitor() {
            Some(monitor) => monitor.size(),
            None => el.available_monitors().next().unwrap().size(),
        };

        let wb = winit::window::WindowBuilder::new();

        let window;
        #[cfg(target_os = "windows")]
        {
            // Disable drag and drop on windows to allow cpal to init on the main thread
            // https://github.com/rust-windowing/winit/issues/1185
            use winit::platform::windows::WindowBuilderExtWindows;
            window = wb.with_drag_and_drop(false);
        }
        #[cfg(not(target_os = "windows"))]
        {
            window = wb;
        }
        // A pinned size must also be unresizable: a compositor that lets the user drag the edge
        // would change the resolution mid-capture.
        let window = match opts.fixed_size {
            Some((w, h)) => window
                .with_inner_size(PhysicalSize::new(w, h))
                .with_resizable(false),
            None => window.with_inner_size(PhysicalSize::new(
                size.width as f32 * 0.8,
                size.height as f32 * 0.8,
            )),
        };
        let window = window
            .with_title(format!("Egregoria {}", include_str!("../../VERSION")))
            .build(&el)
            .expect("Failed to create window");
        let window = Arc::new(window);
        beul::execute(run::<S>(el, window, opts))
    }
}

#[derive(Default)]
pub struct EngineTimes {
    /// Time taken by the engine to process the render commands
    pub render_time: f32,
    /// Time taken to update/render the gui
    pub gui_time: f32,
    /// Total time taken to do CPU work: update/render prepare/render/gui
    pub total_cpu_time: f32,
}

/// Context is the main struct that contains all the context of the game.
/// It holds the necessary state for graphics, input, audio, and the window.
pub struct Context {
    pub gfx: GfxContext,
    pub input: InputContext,
    pub audio: AudioContext,
    pub delta: f32,
    /// Makes sure all events go to InputContext even if catched by yakui
    pub keybind_mode: bool,
    pub times: EngineTimes,
    pub egui: EguiWrapper,
    #[cfg(feature = "yakui")]
    pub yakui: crate::yakui::YakuiWrapper,
}

impl Context {
    pub async fn new(window: Arc<Window>, el: &EventLoop<()>, opts: GfxOptions) -> Self {
        let gfx = GfxContext::new(window, opts).await;
        let input = InputContext::default();
        let audio = AudioContext::new();
        let egui = EguiWrapper::new(&gfx, el);

        Self {
            input,
            audio,
            delta: 0.0,
            keybind_mode: false,
            times: EngineTimes::default(),
            egui,
            #[cfg(feature = "yakui")]
            yakui: crate::yakui::YakuiWrapper::new(&gfx, gfx.window()),
            gfx,
        }
    }

    pub async fn new_offscreen(width: u32, height: u32, opts: GfxOptions) -> Self {
        let gfx = GfxContext::new_offscreen(width, height, opts).await;
        let egui = EguiWrapper::new_headless(&gfx);

        Self {
            input: InputContext::default(),
            audio: AudioContext::empty("headless capture"),
            delta: 0.0,
            keybind_mode: false,
            times: EngineTimes::default(),
            egui,
            #[cfg(feature = "yakui")]
            yakui: crate::yakui::YakuiWrapper::new_headless(&gfx),
            gfx,
        }
    }
}
