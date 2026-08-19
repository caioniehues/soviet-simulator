//! M1.7 acceptance-demo capture (ticket #15): offscreen deterministic render
//! of the full chain — coal hauled by truck powers the plant that powers the
//! factory; a failed pave shows the gravel gate, delivered gravel pays for the
//! paved road, cutting the haul road cascades into a blackout, rebuild
//! recovers. Frames land in the given directory for ffmpeg assembly.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture -- frames <outdir> <count>

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

use soviet_simulator::SimPlugins;
use soviet_simulator::game::GamePlugins;
use soviet_simulator::game::camera::CameraRig;
use soviet_simulator::game::tools::ToolMode;
use soviet_simulator::sim::SimSpeed;
use soviet_simulator::sim::buildings::{Building, BuildingEdit, BuildingEditQueue, BuildingKind};
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue};
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue};
use soviet_simulator::sim::wires::{WireEdit, WireEditQueue};

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
            eprintln!("[capture] usage: capture frames <outdir> <count>");
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
    // The M1.7 coal-chain scenario: no construction sites (buildings place
    // finished), no zones, no water/heat webs, no customs.
    .add_plugins(
        SimPlugins
            .build()
            .disable::<soviet_simulator::sim::construction::ConstructionSimPlugin>()
            .disable::<soviet_simulator::sim::zoning::ZoningSimPlugin>()
            .disable::<soviet_simulator::sim::water::WaterSimPlugin>()
            .disable::<soviet_simulator::sim::heat::HeatSimPlugin>()
            .disable::<soviet_simulator::sim::customs::CustomsSimPlugin>(),
    )
    .add_plugins(GamePlugins);

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

/// Demo layout, all on one east–west corridor:
/// mine(-70,0) —A(-60,0)—road—B(20,0)— plant(30,0) —road—C(110,0)— factory(120,0)
/// quarry(30,90) —Q(20,90)—spur—B— (gravel route dodges the cut)
fn build_world(roads: &mut RoadEditQueue, buildings: &mut BuildingEditQueue) {
    let a = Vec3::new(-60.0, 0.0, 0.0);
    let b = Vec3::new(20.0, 0.0, 0.0);
    let c = Vec3::new(110.0, 0.0, 0.0);
    let q = Vec3::new(20.0, 0.0, 90.0);
    for (from, to) in [(a, b), (b, c), (q, b)] {
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
        (BuildingKind::Quarry, Vec3::new(30.0, 0.0, 90.0)),
        (BuildingKind::Depot, Vec3::new(-40.0, 0.0, 55.0)),
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
            // Seed yards (the pre-demo economy ran off screen) and stand up
            // the two shuttles: coal mine→plant, gravel quarry→factory.
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
            let (quarry, factory) = (get(BuildingKind::Quarry), get(BuildingKind::Factory));
            for (e, kind, tonnes) in [
                (mine, ResourceKind::Coal, 40.0),
                (plant, ResourceKind::Coal, 25.0),
                (quarry, ResourceKind::Gravel, 30.0),
            ] {
                world.get_mut::<Inventory>(e).unwrap().add(kind, tonnes);
            }
            let depot = get(BuildingKind::Depot);
            let mut vehicles = world.resource_mut::<VehicleEditQueue>();
            for _ in 0..2 {
                vehicles.0.push(VehicleEdit::BuyTruck {
                    depot,
                    class: TransportClass::Bulk,
                });
            }
            vehicles.0.push(VehicleEdit::CreateShuttle {
                from: mine,
                to: plant,
                resource: ResourceKind::Coal,
            });
            vehicles.0.push(VehicleEdit::CreateShuttle {
                from: quarry,
                to: factory,
                resource: ResourceKind::Gravel,
            });
        }
        // Pipelines warmed: freeze the sim through the warmup, then run hot.
        4 => *world.resource_mut::<SimSpeed>() = SimSpeed::Paused,
        WARMUP => *world.resource_mut::<SimSpeed>() = SimSpeed::Quad,
        _ => {}
    }
    // Clip actions, keyed off frames since the clip started.
    match f.saturating_sub(WARMUP) {
        // Pave attempt east of the factory: no gravel delivered there yet —
        // rejected, the HUD shows the shortfall.
        55 => *world.resource_mut::<ToolMode>() = ToolMode::Road(RoadClass::Paved),
        60 => push_road(
            world,
            RoadEdit::Place {
                from: Vec3::new(110.0, 0.0, 0.0),
                to: Vec3::new(200.0, 0.0, 0.0),
                class: RoadClass::Paved,
            },
        ),
        // Cut the coal haul road: the plant burns through its yard and the
        // factory blacks out (lamp goes dark red).
        130 => push_road(
            world,
            RoadEdit::RemoveNear {
                pos: Vec3::new(-20.0, 0.0, 0.0),
            },
        ),
        // Wire the factory late so its yard stays clear of goods until the
        // gravel delivery lands: the lamp lights on the plant's last coal —
        // then dies as the cascade from the cut reaches it.
        190 => world
            .resource_mut::<WireEditQueue>()
            .0
            .push(WireEdit::Place {
                kind: soviet_simulator::sim::wires::NetKind::Power,
                from: Vec3::new(30.0, 0.0, 0.0),
                to: Vec3::new(120.0, 0.0, 0.0),
            }),
        // Rebuild hotkey: the cut road returns, the held truck re-routes,
        // coal flows again and the lamp relights before the clip ends.
        240 => push_road(world, RoadEdit::RebuildLast),
        // By now the quarry shuttle has delivered gravel into the factory
        // yard: the same pave goes through and the dark ribbon appears.
        250 => push_road(
            world,
            RoadEdit::Place {
                from: Vec3::new(110.0, 0.0, 0.0),
                to: Vec3::new(200.0, 0.0, 0.0),
                class: RoadClass::Paved,
            },
        ),
        _ => {}
    }

    // Slow push-in over the whole clip, keeping the corridor in frame.
    let t = (f.saturating_sub(WARMUP) as f32 / total as f32).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(25.0, 0.0, 30.0).lerp(Vec3::new(30.0, 0.0, 8.0), t);
    rig.dist = 300.0 + (190.0 - 300.0) * t;
    rig.yaw = -0.55 + 0.25 * t;
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
