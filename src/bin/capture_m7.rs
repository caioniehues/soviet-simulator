//! B7.4 acceptance-demo capture (ticket #63): offscreen deterministic render
//! of housing-and-the-plan. Twenty-four recruited households face sixteen
//! doubled-up flats: the PLAN dashboard reads the homeless queue, and the
//! planner answers it by *building* — a dwelling blueprint goes down and
//! rises through the B6 phased pipeline (hauled gravel, excavator, crane).
//! On activation the housing office drains the queue into the new block by
//! its proximity policy. Late in the clip the old block is demolished: its
//! households are evicted back into the visible queue — never deleted — and
//! the office re-doubles them into the block that remains.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_m7 -- frames <outdir> <count>

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
use soviet_simulator::sim::construction::{ConstructionSimPlugin, ConstructionSite};
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::storage::default_policies;
use soviet_simulator::sim::vehicles::{
    VehicleEdit, VehicleEditQueue, VehicleKind, VehicleSimPlugin,
};
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const WARMUP: u32 = 90;
const ROLL_OUT: u32 = WARMUP + 20;

/// Clip frame where the planner answers the queue with a new blueprint.
const BLUEPRINT_AT: u32 = 60;
/// Clip frame where the old block is demolished — eviction feeds the queue.
const DEMOLISH_AT: u32 = 400;
const BLUEPRINT_AT_PLUS_2: u32 = BLUEPRINT_AT + 2;

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
            eprintln!("[capture] usage: capture_m7 frames <outdir> <count>");
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
        ConstructionSimPlugin,
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

/// Demo layout: yard cluster + the old dwelling block west; the answer to
/// the queue goes down 60 m east once the clip starts.
fn build_world(roads: &mut RoadEditQueue, buildings: &mut BuildingEditQueue) {
    roads.0.push(RoadEdit::Place {
        from: Vec3::ZERO,
        to: Vec3::new(60.0, 0.0, 0.0),
        class: RoadClass::Dirt,
    });
    for (kind, pos) in [
        (BuildingKind::Quarry, Vec3::new(-14.0, 0.0, -6.0)),
        (BuildingKind::Warehouse, Vec3::new(-4.0, 0.0, -24.0)),
        (BuildingKind::Depot, Vec3::new(2.0, 0.0, 22.0)),
        (BuildingKind::ConstructionOffice, Vec3::new(28.0, 0.0, 24.0)),
        (BuildingKind::Dwelling, Vec3::new(24.0, 0.0, -18.0)),
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
            // Fiat-complete everything placed at t0 (the old block included);
            // the *new* blueprint mid-clip is the genuine site.
            let found: Vec<(Entity, BuildingKind)> = {
                let mut q = world.query::<(Entity, &Building)>();
                q.iter(world).map(|(e, b)| (e, b.kind)).collect()
            };
            for (entity, kind) in found {
                world.entity_mut(entity).remove::<ConstructionSite>();
                // Suppliers get roomy yards; everything else keeps its kind's
                // capacity (an inflated dwelling yard would hoard goods).
                let capacity = match kind {
                    BuildingKind::Quarry | BuildingKind::Warehouse => {
                        kind.inventory_capacity().max(120.0)
                    }
                    _ => kind.inventory_capacity(),
                };
                world
                    .entity_mut(entity)
                    .insert((Inventory::new(capacity), default_policies(kind)));
                match kind {
                    BuildingKind::Quarry => {
                        world
                            .get_mut::<Inventory>(entity)
                            .unwrap()
                            .add(ResourceKind::Gravel, 30.0);
                    }
                    BuildingKind::Warehouse => {
                        world
                            .get_mut::<Inventory>(entity)
                            .unwrap()
                            .add(ResourceKind::Goods, 100.0);
                    }
                    BuildingKind::Depot => {
                        let mut edits = world.resource_mut::<VehicleEditQueue>();
                        edits.0.push(VehicleEdit::BuyTruck {
                            depot: entity,
                            class: TransportClass::Bulk,
                        });
                        edits.0.push(VehicleEdit::BuyTruck {
                            depot: entity,
                            class: TransportClass::Covered,
                        });
                    }
                    BuildingKind::ConstructionOffice => {
                        let mut edits = world.resource_mut::<VehicleEditQueue>();
                        edits.0.push(VehicleEdit::BuyMachine {
                            office: entity,
                            kind: VehicleKind::Excavator,
                        });
                        edits.0.push(VehicleEdit::BuyMachine {
                            office: entity,
                            kind: VehicleKind::Crane,
                        });
                    }
                    _ => {}
                }
            }
            // 24 households against 16 doubled-up flats: a visible queue.
            world
                .resource_mut::<soviet_simulator::sim::households::RecruitmentPlan>()
                .target_households = 24;
        }
        4 => *world.resource_mut::<SimSpeed>() = SimSpeed::Paused,
        ROLL_OUT => *world.resource_mut::<SimSpeed>() = SimSpeed::Quad,
        _ => {}
    }
    match f.saturating_sub(WARMUP) {
        // Beat 2: the queue on the PLAN panel gets its answer — a new
        // dwelling blueprint, built for real through the B6 pipeline.
        BLUEPRINT_AT => {
            world
                .resource_mut::<BuildingEditQueue>()
                .0
                .push(BuildingEdit::Place {
                    kind: BuildingKind::Dwelling,
                    pos: Vec3::new(66.0, 0.0, -14.0),
                });
        }
        BLUEPRINT_AT_PLUS_2 => {
            // Pin the inspect panel to the rising block for the middle act.
            let site = world
                .query::<(Entity, &Building)>()
                .iter(world)
                .filter(|(_, b)| b.kind == BuildingKind::Dwelling)
                .find(|(_, b)| b.pos.x > 50.0)
                .map(|(e, _)| e);
            world
                .resource_mut::<soviet_simulator::game::hud::Selected>()
                .0 = site;
        }
        // Beat 4: the old block comes down — eviction feeds the queue,
        // never deletes anyone.
        DEMOLISH_AT => {
            let old = world
                .query::<(Entity, &Building)>()
                .iter(world)
                .filter(|(_, b)| b.kind == BuildingKind::Dwelling)
                .find(|(_, b)| b.pos.x < 50.0)
                .map(|(e, _)| e);
            if let Some(old) = old {
                world
                    .resource_mut::<BuildingEditQueue>()
                    .0
                    .push(BuildingEdit::Demolish { building: old });
            }
        }
        _ => {}
    }

    // Hold the whole cluster; slight drift toward the rising site.
    let t = (f.saturating_sub(WARMUP) as f32 / total as f32).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(22.0, 0.0, -2.0).lerp(Vec3::new(42.0, 0.0, -8.0), t);
    rig.dist = 160.0 - 15.0 * t;
    rig.yaw = -0.35;
    rig.pitch = (-48f32).to_radians();
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
