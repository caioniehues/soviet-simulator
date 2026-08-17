//! Render-cost gate for the R0.1 grounding stack (#111).
//!
//! The charter pins 60 fps and **cut user-facing quality options**, so whether
//! SSAO + contact shadows + TAA can be afforded is a decision made once, here,
//! from a measurement rather than from taste.
//!
//! It renders the same offscreen 1280×720 target the capture bins use, driving
//! a built town at full sim speed with no frame pacing, so wall-time per frame
//! is the combined CPU+GPU cost of a frame. Two modes:
//!
//! - `full` — the shipping stack.
//! - `bare` — the same scene with `ScreenSpaceAmbientOcclusion`,
//!   `ContactShadows` and `TemporalAntiAliasing` stripped off the camera.
//!
//! The difference between them is the grounding pass's price. Run both:
//!
//!   cargo run --release --bin bench_render -- full 600
//!   cargo run --release --bin bench_render -- bare 600
//!
//! Note what this does *not* measure: it is an offscreen render target with no
//! swapchain present and no vsync, on whatever adapter is present. It bounds
//! the cost of the stack; it is not a substitute for the windowed frame budget
//! at 250k identities, which belongs to the sim gates.

use std::time::{Duration, Instant};

use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::app::ScheduleRunnerPlugin;
use bevy::camera::RenderTarget;
use bevy::pbr::{ContactShadows, ScreenSpaceAmbientOcclusion};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::window::ExitCondition;
use bevy::winit::WinitPlugin;

use soviet_simulator::game::GamePlugin;
use soviet_simulator::game::camera::CameraRig;
use soviet_simulator::sim::buildings::{
    Building, BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
};
use soviet_simulator::sim::construction::{ConstructionSimPlugin, ConstructionSite};
use soviet_simulator::sim::resources::{Inventory, ResourceKind};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::storage::default_policies;
use soviet_simulator::sim::vehicles::VehicleSimPlugin;
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
/// Frames rendered before the clock starts, so shader compilation, asset
/// loads and TAA's history buffer are all warm.
const WARMUP: u32 = 180;

/// A town dense enough that the grounding stack has geometry to work on.
const BLOCKS: i32 = 5;

#[derive(Resource)]
struct BenchConfig {
    target: Handle<Image>,
    frames: u32,
    grounded: bool,
}

#[derive(Resource)]
struct Clock {
    frame: u32,
    started: Option<Instant>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (grounded, frames) = match args.as_slice() {
        [_, mode, count] if mode == "full" || mode == "bare" => (
            mode == "full",
            count.parse::<u32>().expect("[bench] bad frame count"),
        ),
        _ => {
            eprintln!("[bench] usage: bench_render <full|bare> <frames>");
            std::process::exit(2);
        }
    };

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .disable::<WinitPlugin>(),
    )
    .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::ZERO))
    .add_plugins((
        SimPlugin,
        RoadSimPlugin,
        BuildingSimPlugin,
        soviet_simulator::sim::storage::StorageSimPlugin,
        soviet_simulator::sim::households::HouseholdSimPlugin,
        soviet_simulator::sim::labour::LabourSimPlugin,
        soviet_simulator::sim::commute::CommuteSimPlugin,
        soviet_simulator::sim::needs::NeedsSimPlugin,
        VehicleSimPlugin,
        soviet_simulator::sim::dispatch::DispatchSimPlugin,
        ConstructionSimPlugin,
        soviet_simulator::sim::zoning::ZoningSimPlugin,
        soviet_simulator::sim::water::WaterSimPlugin,
        soviet_simulator::sim::heat::HeatSimPlugin,
    ))
    .add_plugins((
        soviet_simulator::sim::plan::PlanSimPlugin,
        soviet_simulator::sim::customs::CustomsSimPlugin,
        soviet_simulator::sim::wires::WireSimPlugin,
        soviet_simulator::sim::save::SaveSimPlugin,
    ))
    .add_plugins(GamePlugin);

    let target = app
        .world_mut()
        .resource_mut::<Assets<Image>>()
        .add(Image::new_target_texture(
            WIDTH,
            HEIGHT,
            TextureFormat::Rgba8UnormSrgb,
            None,
        ));
    app.insert_resource(BenchConfig {
        target,
        frames,
        grounded,
    })
    .insert_resource(Clock {
        frame: 0,
        started: None,
    })
    .add_systems(PostStartup, prepare_camera)
    .add_systems(Update, (build_town, tick).chain());

    app.run();
}

fn prepare_camera(
    mut commands: Commands,
    config: Res<BenchConfig>,
    camera: Query<Entity, With<Camera>>,
    hud_roots: Query<Entity, (With<Node>, Without<ChildOf>)>,
) {
    let camera = camera.single().expect("[bench] no camera");
    let mut entity = commands.entity(camera);
    entity.insert(RenderTarget::Image(config.target.clone().into()));
    if !config.grounded {
        entity.remove::<ScreenSpaceAmbientOcclusion>();
        entity.remove::<ContactShadows>();
        entity.remove::<TemporalAntiAliasing>();
    }
    for root in &hud_roots {
        commands.entity(root).insert(UiTargetCamera(camera));
    }
}

fn build_town(world: &mut World) {
    if world.resource::<Clock>().frame != 20 {
        return;
    }
    let kinds = [
        BuildingKind::Mine,
        BuildingKind::Warehouse,
        BuildingKind::Factory,
        BuildingKind::Dwelling,
        BuildingKind::PowerPlant,
        BuildingKind::Depot,
    ];
    for x in -BLOCKS..=BLOCKS {
        world
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::new(x as f32 * 60.0, 0.0, -BLOCKS as f32 * 60.0),
                to: Vec3::new(x as f32 * 60.0, 0.0, BLOCKS as f32 * 60.0),
                class: RoadClass::Paved,
            });
        for z in -BLOCKS..=BLOCKS {
            let kind = kinds[((x + z).unsigned_abs() as usize) % kinds.len()];
            world
                .resource_mut::<BuildingEditQueue>()
                .0
                .push(BuildingEdit::Place {
                    kind,
                    pos: Vec3::new(x as f32 * 60.0 + 24.0, 0.0, z as f32 * 60.0),
                });
        }
    }
    *world.resource_mut::<SimSpeed>() = SimSpeed::Quad;
}

fn tick(world: &mut World) {
    let frame = {
        let mut clock = world.resource_mut::<Clock>();
        clock.frame += 1;
        clock.frame
    };

    if frame == 40 {
        // Finish the sites so the bench renders buildings, not scaffolding.
        let found: Vec<(Entity, BuildingKind)> = {
            let mut q = world.query_filtered::<(Entity, &Building), With<ConstructionSite>>();
            q.iter(world).map(|(e, b)| (e, b.kind)).collect()
        };
        for (entity, kind) in found {
            world.entity_mut(entity).remove::<ConstructionSite>();
            world.entity_mut(entity).insert((
                Inventory::new(kind.inventory_capacity()),
                default_policies(kind),
            ));
            if kind == BuildingKind::PowerPlant {
                world
                    .get_mut::<Inventory>(entity)
                    .unwrap()
                    .add(ResourceKind::Coal, 400.0);
            }
        }
    }

    // Keep the camera moving: a static view lets TAA converge to a still
    // image, which is not the cost the player pays.
    let t = frame as f32 * 0.01;
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(t.sin() * 120.0, 0.0, t.cos() * 120.0);
    rig.yaw = t * 0.3;
    rig.pitch = (-38.0f32).to_radians();
    rig.dist = 210.0;

    if frame == WARMUP {
        world.resource_mut::<Clock>().started = Some(Instant::now());
    }
    let config_frames = world.resource::<BenchConfig>().frames;
    if frame == WARMUP + config_frames {
        let grounded = world.resource::<BenchConfig>().grounded;
        let elapsed = world
            .resource::<Clock>()
            .started
            .expect("[bench] clock never started")
            .elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0 / config_frames as f64;
        let buildings = world.query::<&Building>().iter(world).count();
        println!(
            "[bench_render] {} \u{2014} {buildings} buildings, {config_frames} frames: \
             mean {ms:.2} ms/frame ({:.0} fps)",
            if grounded { "full" } else { "bare" },
            1000.0 / ms,
        );
        world.write_message(AppExit::Success);
    }
}
