//! M2.7 acceptance-demo capture (ticket #29): offscreen deterministic render
//! of staffing gating the chain. Day 1: commuters stream from the dwelling
//! cluster to mine/plant/factory, the lamp lights. Late day 1: the commute
//! spur is cut — the haul road stays intact, but day 2 nobody reaches work,
//! so nothing runs and the lamp stays dark. Rebuild that evening: day 3 the
//! workers return and the lamp relights. Frames land in the given directory
//! for ffmpeg assembly.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_m2 -- frames <outdir> <count>

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
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
use soviet_simulator::sim::wires::{WireEdit, WireEditQueue, WireSimPlugin};
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
/// Render frames (sim paused) before the clip starts, so mesh/material
/// pipelines finish compiling — early frames otherwise capture gizmos only.
const WARMUP: u32 = 90;

#[derive(Resource)]
struct CaptureConfig {
    target: Handle<Image>,
    dir: PathBuf,
    frames: u32,
}

/// Render frames elapsed; the whole script is keyed off this.
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
            eprintln!("[capture] usage: capture_m2 frames <outdir> <count>");
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
        soviet_simulator::sim::dispatch::DispatchSimPlugin,
        WireSimPlugin,
        soviet_simulator::sim::save::SaveSimPlugin,
    ))
    .add_plugins(GamePlugin);

    // Allocate the render target directly in the world before any camera
    // system runs — Commands-deferred creation yields black captures.
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
    // Without a primary window the HUD needs an explicit camera to land on.
    for root in &hud_roots {
        commands.entity(root).insert(UiTargetCamera(camera));
    }
}

/// Demo layout — the M1 corridor plus a residential spur:
/// mine(-70,0) —A(-60,0)—road—B(20,0)— plant(30,0) —road—C(110,0)— factory(120,0)
/// spur S(-60,80)—A carries only commuters; four dwellings dock at S.
/// Cutting the spur severs the commute while freight keeps rolling — the
/// degradation on screen is purely a staffing cascade.
fn build_world(roads: &mut RoadEditQueue, buildings: &mut BuildingEditQueue) {
    let a = Vec3::new(-60.0, 0.0, 0.0);
    let b = Vec3::new(20.0, 0.0, 0.0);
    let c = Vec3::new(110.0, 0.0, 0.0);
    let s = Vec3::new(-60.0, 0.0, 80.0);
    for (from, to) in [(a, b), (b, c), (s, a)] {
        roads.0.push(RoadEdit::Place {
            from,
            to,
            class: RoadClass::Dirt,
        });
    }
    for (kind, pos) in [
        (BuildingKind::Mine, Vec3::new(-70.0, 0.0, 0.0)),
        (BuildingKind::PowerPlant, Vec3::new(30.0, 0.0, 0.0)),
        (BuildingKind::Factory, Vec3::new(120.0, 0.0, 0.0)),
        (BuildingKind::Dwelling, Vec3::new(-75.0, 0.0, 70.0)),
        (BuildingKind::Dwelling, Vec3::new(-75.0, 0.0, 90.0)),
        (BuildingKind::Dwelling, Vec3::new(-45.0, 0.0, 70.0)),
        (BuildingKind::Dwelling, Vec3::new(-45.0, 0.0, 90.0)),
        (BuildingKind::Depot, Vec3::new(60.0, 0.0, -40.0)),
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
        }
        3 => {
            // Seed yards so fuel is never the constraint (staffing is), wire
            // the lamp, stand up the coal shuttle, and recruit the workforce:
            // 10 households (30 citizens) for 24 jobs across the chain.
            let mut found: Vec<(Entity, BuildingKind)> = Vec::new();
            let mut q = world.query::<(Entity, &Building)>();
            for (e, b) in q.iter(world) {
                found.push((e, b.kind));
            }
            let get = |kind: BuildingKind| {
                found
                    .iter()
                    .find(|(_, k)| *k == kind)
                    .map(|(e, _)| *e)
                    .expect("[capture] building missing")
            };
            let (mine, plant) = (get(BuildingKind::Mine), get(BuildingKind::PowerPlant));
            for (e, tonnes) in [(mine, 40.0), (plant, 25.0)] {
                world
                    .get_mut::<Inventory>(e)
                    .unwrap()
                    .add(ResourceKind::Coal, tonnes);
            }
            world.resource_mut::<WireEditQueue>().0.push(WireEdit::Place {
            kind: soviet_simulator::sim::wires::NetKind::Power,
                from: Vec3::new(30.0, 0.0, 0.0),
                to: Vec3::new(120.0, 0.0, 0.0),
            });
            let depot = get(BuildingKind::Depot);
            let mut vehicles = world.resource_mut::<VehicleEditQueue>();
            vehicles.0.push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
            vehicles.0.push(VehicleEdit::CreateShuttle {
                from: mine,
                to: plant,
                resource: ResourceKind::Coal,
            });
            world.resource_mut::<RecruitmentPlan>().target_households = 10;
        }
        // Pipelines warmed: freeze the sim through the warmup, then run hot.
        4 => *world.resource_mut::<SimSpeed>() = SimSpeed::Paused,
        // Double speed = 4 sim frames per 30 fps render frame (two 60 Hz
        // passes × 2 substeps): a 600-frame game day is 150 render frames,
        // so the 450-frame clip is a three-day arc — run, idle, recover.
        WARMUP => *world.resource_mut::<SimSpeed>() = SimSpeed::Double,
        _ => {}
    }
    // Clip actions, keyed off frames since the clip started. The workday runs
    // game-day frames ~150–450 (render frames ~38–113 of each day).
    match f.saturating_sub(WARMUP) {
        // Day 1 evening, workers home: cut the commute spur. The haul road
        // is untouched — day 2 the chain sits idle purely for lack of staff.
        140 => push_road(
            world,
            RoadEdit::RemoveNear {
                pos: Vec3::new(-60.0, 0.0, 40.0),
            },
        ),
        // Day 2 evening: rebuild the spur. Day 3 morning the commute resumes
        // and the lamp relights.
        290 => push_road(world, RoadEdit::RebuildLast),
        _ => {}
    }

    // Start tight on the dwellings and spur (day-1 commuters readable), then
    // pull out to hold the whole corridor for the cut and the recovery.
    let t = (f.saturating_sub(WARMUP) as f32 / total as f32).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(-25.0, 0.0, 45.0).lerp(Vec3::new(20.0, 0.0, 25.0), t);
    rig.dist = 240.0 + (330.0 - 240.0) * t;
    rig.yaw = -0.55 + 0.2 * t;
    rig.pitch = (-55f32).to_radians();
}

fn push_road(world: &mut World, edit: RoadEdit) {
    world.resource_mut::<RoadEditQueue>().0.push(edit);
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
