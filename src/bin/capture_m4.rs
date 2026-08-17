//! B4.6 acceptance-demo capture (ticket #44): offscreen deterministic render
//! of traffic at scale's player-facing beats. A quarry→factory freight loop
//! runs over an S-curved corridor (B4.5 node-fan bezier ribbons). Mid-clip a
//! breakdown jams the middle bend: following trucks queue at footprint
//! spacing (B4.3), the corridor registers on the StallBoard — red glow +
//! HUD STALL line (B4.4). A bypass built moments later is priced against the
//! congested bend (B4.2) and the fleet re-routes onto it (B4.1 async A*);
//! the stall clears and flow resumes. Nothing ever despawns.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_m4 -- frames <outdir> <count>

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
use soviet_simulator::sim::roads::{
    LaneDir, RoadClass, RoadEdit, RoadEditQueue, RoadNode, RoadSegment, RoadSimPlugin,
};
use soviet_simulator::sim::vehicles::{
    ActiveVehicle, RouteLeg, VehicleEdit, VehicleEditQueue, VehicleSimPlugin,
};
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
/// Render frames (sim paused) before the clip starts, so mesh/material
/// pipelines finish compiling — early frames otherwise capture gizmos only.
const WARMUP: u32 = 90;
const ROLL_OUT: u32 = WARMUP + 20;

/// Clip frame where the breakdown jams the middle bend.
const JAM_AT: u32 = 130;
/// Clip frame where the bypass goes down (stall alarm has been up ~2 s).
const BYPASS_AT: u32 = 270;

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
            eprintln!("[capture] usage: capture_m4 frames <outdir> <count>");
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

/// Demo layout — an S-curved main corridor with a depot spur:
/// A(0,0) — M1(50,-25) — M2(100,25) — B(150,0), all 2-valence through-nodes
/// so the bezier fan bends every joint. Quarry docks A, factory docks B.
fn build_world(roads: &mut RoadEditQueue, buildings: &mut BuildingEditQueue) {
    let a = Vec3::new(0.0, 0.0, 0.0);
    let m1 = Vec3::new(50.0, 0.0, -25.0);
    let m2 = Vec3::new(100.0, 0.0, 25.0);
    let b = Vec3::new(150.0, 0.0, 0.0);
    let spur = Vec3::new(0.0, 0.0, 40.0);
    for (from, to) in [(a, m1), (m1, m2), (m2, b), (a, spur)] {
        roads.0.push(RoadEdit::Place {
            from,
            to,
            class: RoadClass::Dirt,
        });
    }
    for (kind, pos) in [
        (BuildingKind::Quarry, Vec3::new(-12.0, 0.0, 0.0)),
        (BuildingKind::Factory, Vec3::new(162.0, 0.0, 0.0)),
        (BuildingKind::Depot, Vec3::new(0.0, 0.0, 50.0)),
    ] {
        buildings.0.push(BuildingEdit::Place { kind, pos });
    }
}

/// The middle-bend segment M1→M2 (both endpoints off the z=0 line).
fn middle_bend(world: &mut World) -> Option<Entity> {
    let list: Vec<(Entity, Entity, Entity)> = world
        .query::<(Entity, &RoadSegment)>()
        .iter(world)
        .map(|(e, s)| (e, s.a, s.b))
        .collect();
    list.into_iter()
        .find(|(_, a, b)| {
            let za = world.get::<RoadNode>(*a).map(|n| n.pos.z).unwrap_or(0.0);
            let zb = world.get::<RoadNode>(*b).map(|n| n.pos.z).unwrap_or(0.0);
            za.abs() > 1.0 && zb.abs() > 1.0 && (za - zb).abs() > 1.0
        })
        .map(|(e, ..)| e)
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
            let quarry = get(BuildingKind::Quarry);
            world
                .get_mut::<Inventory>(quarry)
                .unwrap()
                .add(ResourceKind::Gravel, 200.0);
            let depot = get(BuildingKind::Depot);
            let factory = get(BuildingKind::Factory);
            let mut vehicles = world.resource_mut::<VehicleEditQueue>();
            for _ in 0..4 {
                vehicles.0.push(VehicleEdit::BuyTruck {
                    depot,
                    class: TransportClass::Bulk,
                });
            }
            vehicles.0.push(VehicleEdit::CreateShuttle {
                from: quarry,
                to: factory,
                resource: ResourceKind::Gravel,
            });
        }
        4 => *world.resource_mut::<SimSpeed>() = SimSpeed::Paused,
        ROLL_OUT => *world.resource_mut::<SimSpeed>() = SimSpeed::Quad,
        _ => {}
    }
    match f.saturating_sub(WARMUP) {
        // Beat 2: breakdown — a convoy dies on the middle bend. Density
        // saturates the segment, its mouth is physically blocked, and the
        // running trucks pile up behind at footprint spacing.
        JAM_AT => {
            if let Some(seg) = middle_bend(world) {
                let (length, count) = {
                    let s = world.get::<RoadSegment>(seg).unwrap();
                    (s.length, 16u32)
                };
                for i in 0..count {
                    let s_pos = 2.0 + (length - 8.0) * i as f32 / count as f32;
                    let (pos, travel) = world
                        .get::<RoadSegment>(seg)
                        .unwrap()
                        .point_at(s_pos, LaneDir::Forward);
                    let mut pawn = ActiveVehicle::at(pos + travel.cross(Vec3::Y) * 1.5);
                    pawn.heading = travel;
                    pawn.route = vec![RouteLeg {
                        segment: seg,
                        dir: LaneDir::Forward,
                    }];
                    pawn.s = s_pos;
                    world.spawn(pawn);
                }
            }
        }
        // Beat 4: the planner answers the stall alarm with a bypass around
        // the dead bend; the async solver prices the jam and the fleet
        // re-routes onto the new corridor.
        BYPASS_AT => {
            let mut roads = world.resource_mut::<RoadEditQueue>();
            let m1 = Vec3::new(50.0, 0.0, -25.0);
            let p = Vec3::new(90.0, 0.0, -60.0);
            let b = Vec3::new(150.0, 0.0, 0.0);
            for (from, to) in [(m1, p), (p, b)] {
                roads.0.push(RoadEdit::Place {
                    from,
                    to,
                    class: RoadClass::Dirt,
                });
            }
        }
        _ => {}
    }

    // Open tight on the S-bend ribbons, then pull back to hold the whole
    // corridor for the jam, the red stall glow, and the bypass beats.
    let t = (f.saturating_sub(WARMUP) as f32 / (total as f32 * 0.4)).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(70.0, 0.0, 0.0).lerp(Vec3::new(75.0, 0.0, -10.0), t);
    rig.dist = 130.0 + (250.0 - 130.0) * t;
    rig.yaw = -0.35 + 0.1 * t;
    rig.pitch = (-50f32).to_radians();
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
