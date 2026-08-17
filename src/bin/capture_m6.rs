//! B6.6 acceptance-demo capture (ticket #58): offscreen deterministic render
//! of phased construction. A dwelling blueprint east of the yard cluster
//! rises for real: the bulk truck hauls the gravel pad, the excavator grades
//! it, and the structure phase then starves — the warehouse holds no goods,
//! so the site pulses its amber NO MATERIAL ring and the inspect panel names
//! the stall. A goods delivery lands mid-clip and the crane (bought moments
//! earlier) works the structure and roof; the building snaps to full height,
//! activated. Duration is emergent from supply throughout — no timer.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_m6 -- frames <outdir> <count>

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

/// Clip frame where the goods shipment reaches the warehouse (the structure
/// phase has been starving in amber until now).
const GOODS_ARRIVE_AT: u32 = 150;
/// Clip frame where the crane is bought (drives out to meet the material).
const CRANE_AT: u32 = 155;

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
            eprintln!("[capture] usage: capture_m6 frames <outdir> <count>");
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

/// Demo layout: yard cluster west (quarry, warehouse, depot, office), the
/// dwelling blueprint 55 m east — every trip is short enough to watch.
fn build_world(roads: &mut RoadEditQueue, buildings: &mut BuildingEditQueue) {
    roads.0.push(RoadEdit::Place {
        from: Vec3::ZERO,
        to: Vec3::new(55.0, 0.0, 0.0),
        class: RoadClass::Dirt,
    });
    for (kind, pos) in [
        (BuildingKind::Quarry, Vec3::new(-14.0, 0.0, -6.0)),
        (BuildingKind::Warehouse, Vec3::new(-4.0, 0.0, -22.0)),
        (BuildingKind::Depot, Vec3::new(2.0, 0.0, 22.0)),
        (BuildingKind::ConstructionOffice, Vec3::new(26.0, 0.0, 24.0)),
        (BuildingKind::Dwelling, Vec3::new(62.0, 0.0, -10.0)),
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
            // Fiat-complete the yard cluster; the dwelling stays a genuine
            // site. Stock gravel only — the structure phase will starve.
            let found: Vec<(Entity, BuildingKind)> = {
                let mut q = world.query::<(Entity, &Building)>();
                q.iter(world).map(|(e, b)| (e, b.kind)).collect()
            };
            for (entity, kind) in found {
                if kind == BuildingKind::Dwelling {
                    continue;
                }
                world.entity_mut(entity).remove::<ConstructionSite>();
                world.entity_mut(entity).insert((
                    Inventory::new(kind.inventory_capacity().max(60.0)),
                    default_policies(kind),
                ));
                match kind {
                    BuildingKind::Quarry => {
                        world
                            .get_mut::<Inventory>(entity)
                            .unwrap()
                            .add(ResourceKind::Gravel, 30.0);
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
                        world
                            .resource_mut::<VehicleEditQueue>()
                            .0
                            .push(VehicleEdit::BuyMachine {
                                office: entity,
                                kind: VehicleKind::Excavator,
                            });
                    }
                    _ => {}
                }
            }
            // Pin the inspect panel to the site for the whole clip.
            let site = world
                .query::<(Entity, &Building)>()
                .iter(world)
                .find(|(_, b)| b.kind == BuildingKind::Dwelling)
                .map(|(e, _)| e);
            world.resource_mut::<soviet_simulator::game::hud::Selected>().0 = site;
        }
        4 => *world.resource_mut::<SimSpeed>() = SimSpeed::Paused,
        ROLL_OUT => *world.resource_mut::<SimSpeed>() = SimSpeed::Quad,
        _ => {}
    }
    match f.saturating_sub(WARMUP) {
        // Beat 3: the goods shipment finally lands at the warehouse — the
        // amber NO MATERIAL stall breaks and the covered truck rolls.
        GOODS_ARRIVE_AT => {
            let warehouse = world
                .query::<(Entity, &Building)>()
                .iter(world)
                .find(|(_, b)| b.kind == BuildingKind::Warehouse)
                .map(|(e, _)| e);
            if let Some(warehouse) = warehouse {
                // Past the warehouse's max band, so the surplus genuinely
                // supplies the starving site.
                world
                    .get_mut::<Inventory>(warehouse)
                    .unwrap()
                    .add(ResourceKind::Goods, 100.0);
            }
        }
        // Beat 4: the crane joins the fleet and drives out to the site.
        CRANE_AT => {
            let office = world
                .query::<(Entity, &Building)>()
                .iter(world)
                .find(|(_, b)| b.kind == BuildingKind::ConstructionOffice)
                .map(|(e, _)| e);
            if let Some(office) = office {
                world
                    .resource_mut::<VehicleEditQueue>()
                    .0
                    .push(VehicleEdit::BuyMachine {
                        office,
                        kind: VehicleKind::Crane,
                    });
            }
        }
        _ => {}
    }

    // Hold the whole cluster; slight drift toward the rising site.
    let t = (f.saturating_sub(WARMUP) as f32 / total as f32).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(24.0, 0.0, 0.0).lerp(Vec3::new(40.0, 0.0, -4.0), t);
    rig.dist = 150.0 - 20.0 * t;
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
