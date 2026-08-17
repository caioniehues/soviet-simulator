//! B5.6 acceptance-demo capture (ticket #51): offscreen deterministic render
//! of public transit's player-facing beats. A 700 m corridor separates the
//! dwelling cluster from four factories — beyond anyone's walking tolerance,
//! so the bus line *is* the labour supply. Day 1: commuters stream to the
//! shelter, the single 30-seat bus boards a full load and leaves the
//! overflow visibly queued (TRANSIT panel counts), riders arrive and the
//! inspected factory staffs up. Mid-clip the line is deleted: the bus
//! dead-heads home to its depot slot, and on day 2 nobody can reach the
//! factory — the inspect panel shows presence collapse while tenure holds.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_m5 -- frames <outdir> <count>

use std::path::PathBuf;
use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use bevy::window::ExitCondition;
use bevy::winit::WinitPlugin;

use soviet_simulator::game::GamePlugin;
use soviet_simulator::game::camera::CameraRig;
use soviet_simulator::sim::buildings::{
    Building, BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
};
use soviet_simulator::sim::households::RecruitmentPlan;
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::transit::{TransitEdit, TransitEditQueue, TransitLine};
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const WARMUP: u32 = 90;
const ROLL_OUT: u32 = WARMUP + 20;

/// Clip frame where the line is deleted (after day 1's evening rides).
const DELETE_AT: u32 = 250;

#[derive(Resource)]
struct CaptureConfig {
    target: Handle<Image>,
    dir: PathBuf,
    frames: u32,
}

#[derive(Resource, Default)]
struct FrameNo(u32);

#[derive(Resource, Default)]
struct Saved(u32);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (dir, frames) = match args.as_slice() {
        [_, mode, dir, count] if mode == "frames" => (
            PathBuf::from(dir),
            count.parse::<u32>().expect("[capture] bad frame count"),
        ),
        _ => {
            eprintln!("[capture] usage: capture_m5 frames <outdir> <count>");
            std::process::exit(2);
        }
    };
    std::fs::create_dir_all(&dir).expect("[capture] cannot create output dir");

    let mut app = App::new();
    // Pre-G1 fiat economy: these scenarios predate the rouble; an
    // infinite treasury keeps them reproducing their recorded stories.
    app.insert_resource(soviet_simulator::sim::plan::Treasury {
        roubles: f32::INFINITY,
    });
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
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        1.0 / FPS,
    )))
    .add_plugins((
        SimPlugin,
        RoadSimPlugin,
        BuildingSimPlugin,
        soviet_simulator::sim::households::HouseholdSimPlugin,
        soviet_simulator::sim::labour::LabourSimPlugin,
        soviet_simulator::sim::commute::CommuteSimPlugin,
        soviet_simulator::sim::needs::NeedsSimPlugin,
        VehicleSimPlugin,
        soviet_simulator::sim::storage::StorageSimPlugin,
        // DispatchSimPlugin auto-adds Pathfinding + Traffic.
        soviet_simulator::sim::dispatch::DispatchSimPlugin,
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
    app.insert_resource(CaptureConfig {
        target,
        dir,
        frames,
    })
    .init_resource::<FrameNo>()
    .init_resource::<Saved>()
    .add_systems(PostStartup, aim_camera_at_target)
    .add_systems(Update, (drive_script, shoot_frame, finish).chain());

    app.run();
}

fn aim_camera_at_target(
    mut commands: Commands,
    config: Res<CaptureConfig>,
    camera: Query<Entity, With<Camera>>,
    hud_roots: Query<Entity, (With<Node>, Without<ChildOf>)>,
) {
    let camera = camera.single().expect("[capture] no camera");
    commands
        .entity(camera)
        .insert(RenderTarget::Image(config.target.clone().into()));
    for root in &hud_roots {
        commands.entity(root).insert(UiTargetCamera(camera));
    }
}

/// Demo layout: dwellings west, four factories 700 m east (beyond walking
/// tolerance), shelters docking both end nodes, depot with one 30-seat bus.
fn build_world(roads: &mut RoadEditQueue, buildings: &mut BuildingEditQueue) {
    roads.0.push(RoadEdit::Place {
        from: Vec3::ZERO,
        to: Vec3::new(700.0, 0.0, 0.0),
        class: RoadClass::Dirt,
    });
    for (kind, pos) in [
        (BuildingKind::Dwelling, Vec3::new(-2.0, 0.0, 18.0)),
        (BuildingKind::Dwelling, Vec3::new(-18.0, 0.0, 18.0)),
        (BuildingKind::Factory, Vec3::new(700.0, 0.0, 18.0)),
        (BuildingKind::Factory, Vec3::new(722.0, 0.0, 18.0)),
        (BuildingKind::Factory, Vec3::new(700.0, 0.0, -20.0)),
        (BuildingKind::Factory, Vec3::new(722.0, 0.0, -20.0)),
        (BuildingKind::BusStop, Vec3::new(8.0, 0.0, -7.0)),
        (BuildingKind::BusStop, Vec3::new(692.0, 0.0, -7.0)),
        (BuildingKind::Depot, Vec3::new(0.0, 0.0, 36.0)),
    ] {
        buildings.0.push(BuildingEdit::Place { kind, pos });
    }
}

fn drive_script(world: &mut World) {
    world.resource_mut::<FrameNo>().0 += 1;
    let f = world.resource::<FrameNo>().0;
    let total = world.resource::<CaptureConfig>().frames;

    match f {
        1 => {
            world.resource_scope(|world, mut roads: Mut<RoadEditQueue>| {
                let mut buildings = world.resource_mut::<BuildingEditQueue>();
                build_world(&mut roads, &mut buildings);
            });
            world.resource_mut::<RecruitmentPlan>().target_households = 16; // ≈48 citizens: one busload plus overflow
        }
        3 => {
            // Line over the two shelters; one bus in the depot slot.
            let mut stops: Vec<(f32, Entity)> = Vec::new();
            let mut depot = None;
            let mut q = world.query::<(Entity, &Building)>();
            for (e, b) in q.iter(world) {
                match b.kind {
                    BuildingKind::BusStop => stops.push((b.pos.x, e)),
                    BuildingKind::Depot => depot = Some(e),
                    _ => {}
                }
            }
            stops.sort_by(|a, b| a.0.total_cmp(&b.0));
            let depot = depot.expect("[capture] depot missing");
            world
                .resource_mut::<VehicleEditQueue>()
                .0
                .push(VehicleEdit::BuyBus { depot });
            world
                .resource_mut::<TransitEditQueue>()
                .0
                .push(TransitEdit::CreateLine {
                    stops: stops.into_iter().map(|(_, e)| e).collect(),
                });
        }
        4 => *world.resource_mut::<SimSpeed>() = SimSpeed::Paused,
        ROLL_OUT => *world.resource_mut::<SimSpeed>() = SimSpeed::Quad,
        _ => {}
    }
    // One frame after roll-out the line entity exists: put the bus on it and
    // pin the inspect panel to a far factory so staffing reads all clip.
    if f == ROLL_OUT + 2 {
        let line = {
            let mut q = world.query_filtered::<Entity, With<TransitLine>>();
            q.iter(world).next()
        };
        let mut depot = None;
        let mut factory = None;
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::Depot => depot = Some(e),
                BuildingKind::Factory if b.pos.z > 0.0 && b.pos.x < 710.0 => factory = Some(e),
                _ => {}
            }
        }
        if let (Some(line), Some(depot)) = (line, depot) {
            world
                .resource_mut::<TransitEditQueue>()
                .0
                .push(TransitEdit::AssignBus { line, depot });
        }
        world
            .resource_mut::<soviet_simulator::game::hud::Selected>()
            .0 = factory;
    }
    if f.saturating_sub(WARMUP) == DELETE_AT {
        let line = {
            let mut q = world.query_filtered::<Entity, With<TransitLine>>();
            q.iter(world).next()
        };
        if let Some(line) = line {
            world
                .resource_mut::<TransitEditQueue>()
                .0
                .push(TransitEdit::DeleteLine { line });
        }
    }

    // Open on the west shelter (queue + boarding), pull back until the whole
    // corridor with both shelters and the factory cluster is in frame.
    let t = (f.saturating_sub(WARMUP) as f32 / (total as f32 * 0.45)).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(40.0, 0.0, 5.0).lerp(Vec3::new(350.0, 0.0, 0.0), t);
    rig.dist = 140.0 + (620.0 - 140.0) * t;
    rig.yaw = -0.3 + 0.1 * t;
    rig.pitch = (-52f32).to_radians();
}

fn shoot_frame(mut commands: Commands, config: Res<CaptureConfig>, frame: Res<FrameNo>) {
    if frame.0 <= WARMUP || frame.0 > WARMUP + config.frames {
        return;
    }
    let path = config
        .dir
        .join(format!("frame{:05}.png", frame.0 - WARMUP - 1));
    commands
        .spawn(Screenshot::image(config.target.clone()))
        .observe(save_to_disk(path))
        .observe(|_: On<ScreenshotCaptured>, mut saved: ResMut<Saved>| saved.0 += 1);
}

fn finish(
    config: Res<CaptureConfig>,
    frame: Res<FrameNo>,
    saved: Res<Saved>,
    mut exit: MessageWriter<AppExit>,
) {
    if saved.0 >= config.frames {
        println!("[capture] done: {} frames saved", saved.0);
        exit.write(AppExit::Success);
    } else if frame.0 > WARMUP + config.frames + 600 {
        println!(
            "[capture] TIMEOUT: only {}/{} frames saved",
            saved.0, config.frames
        );
        exit.write(AppExit::error());
    }
}
