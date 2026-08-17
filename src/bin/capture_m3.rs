//! M3.7 acceptance-demo capture (ticket #37): offscreen deterministic render
//! of the dispatcher. A coal network — mine, two warehouses, power plant —
//! self-balances to its storage bands with a two-truck depot fleet. Mid-clip
//! warehouse A is drained and the dispatcher refills it; later everything is
//! drained at once, so demand outruns the fleet and the order queue grows
//! visibly in the HUD while both trucks run flat out. The parked fleet is
//! countable on the depot apron in the opening shot.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_m3 -- frames <outdir> <count>

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
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::storage::{StorageBand, StoragePolicies};
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
/// Render frames (sim paused) before the clip starts, so mesh/material
/// pipelines finish compiling — early frames otherwise capture gizmos only.
const WARMUP: u32 = 90;
/// Clip frame where the sim unpauses: the opening second shows the parked,
/// countable fleet on the depot apron.
const ROLL_OUT: u32 = WARMUP + 25;

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
            eprintln!("[capture] usage: capture_m3 frames <outdir> <count>");
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

/// Demo layout — a compact coal district:
/// mine(-8,0) —A(0,0)—road—B(60,0)—road—C(120,0); spur B—S(60,50) to the depot.
/// Warehouse A sits at B's node, warehouse B + plant at C's. Two bulk trucks.
fn build_world(roads: &mut RoadEditQueue, buildings: &mut BuildingEditQueue) {
    let a = Vec3::new(0.0, 0.0, 0.0);
    let b = Vec3::new(40.0, 0.0, 0.0);
    let c = Vec3::new(80.0, 0.0, 0.0);
    let s = Vec3::new(40.0, 0.0, 36.0);
    for (from, to) in [(a, b), (b, c), (b, s)] {
        roads.0.push(RoadEdit::Place {
            from,
            to,
            class: RoadClass::Dirt,
        });
    }
    for (kind, pos) in [
        (BuildingKind::Mine, Vec3::new(-10.0, 0.0, 0.0)),
        (BuildingKind::Warehouse, Vec3::new(42.0, 0.0, -18.0)),
        (BuildingKind::Warehouse, Vec3::new(82.0, 0.0, -18.0)),
        (BuildingKind::PowerPlant, Vec3::new(86.0, 0.0, 16.0)),
        (BuildingKind::Depot, Vec3::new(40.0, 0.0, 46.0)),
    ] {
        buildings.0.push(BuildingEdit::Place { kind, pos });
    }
}

fn warehouses(world: &mut World) -> Vec<Entity> {
    let mut q = world.query::<(Entity, &Building)>();
    let mut found: Vec<(f32, Entity)> = q
        .iter(world)
        .filter(|(_, b)| b.kind == BuildingKind::Warehouse)
        .map(|(e, b)| (b.pos.x, e))
        .collect();
    found.sort_by(|a, b| a.0.total_cmp(&b.0));
    found.into_iter().map(|(_, e)| e).collect()
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
            // Stock the mine, focus the warehouses on coal (0.3–0.6 band so
            // the self-balance target is legible), buy the two-truck fleet.
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
            let mine = get(BuildingKind::Mine);
            world
                .get_mut::<Inventory>(mine)
                .unwrap()
                .add(ResourceKind::Coal, 60.0);
            // Plant pre-stocked into its band so it sits inert: the demo's
            // demand budget (2 × 18 t warehouse min) stays inside the mine's
            // 60 t yard and the network genuinely reaches balance.
            let plant = get(BuildingKind::PowerPlant);
            world
                .get_mut::<Inventory>(plant)
                .unwrap()
                .add(ResourceKind::Coal, 25.0);
            for warehouse in warehouses(world) {
                let mut policies = StoragePolicies::default();
                policies.set(ResourceKind::Coal, Some(StorageBand::new(0.15, 0.4)));
                world.entity_mut(warehouse).insert(policies);
            }
            let depot = get(BuildingKind::Depot);
            let mut vehicles = world.resource_mut::<VehicleEditQueue>();
            for _ in 0..2 {
                vehicles.0.push(VehicleEdit::BuyTruck {
                    depot,
                    class: TransportClass::Bulk,
                });
            }
        }
        4 => *world.resource_mut::<SimSpeed>() = SimSpeed::Paused,
        // Hold the opening shot paused for ~1 s of clip: two parked trucks,
        // countable on the apron, before the fleet rolls out.
        ROLL_OUT => *world.resource_mut::<SimSpeed>() = SimSpeed::Quad,
        _ => {}
    }
    match f.saturating_sub(WARMUP) {
        // Beat 2: warehouse A has been filling toward its band — drain it to
        // zero. The dispatcher notices the deficit and refills it.
        210 => {
            if let Some(&a) = warehouses(world).first() {
                let mut inventory = world.get_mut::<Inventory>(a).unwrap();
                let coal = inventory.amount(ResourceKind::Coal);
                inventory.take(ResourceKind::Coal, coal);
            }
        }
        // Beat 3: drain everything at once — far more deficit than two trucks
        // can serve, so the order queue grows visibly in the dispatch panel.
        340 => {
            let stores: Vec<Entity> = warehouses(world);
            for e in stores {
                let mut inventory = world.get_mut::<Inventory>(e).unwrap();
                let coal = inventory.amount(ResourceKind::Coal);
                inventory.take(ResourceKind::Coal, coal);
            }
        }
        _ => {}
    }

    // Open tight on the depot apron (two parked trucks countable), then pull
    // back to hold the whole district for the balance/drain/queue beats.
    let t = (f.saturating_sub(WARMUP) as f32 / (total as f32 * 0.35)).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(40.0, 0.0, 34.0).lerp(Vec3::new(40.0, 0.0, 2.0), t);
    rig.dist = 110.0 + (230.0 - 110.0) * t;
    rig.yaw = -0.4 + 0.15 * t;
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
