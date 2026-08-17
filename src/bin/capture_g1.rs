//! G1 acceptance-record capture: The Weight of the Plan. Opens on the
//! prebuilt border customs — the republic's front door — then pulls east to
//! empty land where the First Five-Year Plan begins: a mine, a warehouse
//! and a depot rise; trucks are bought with roubles and drive in from the
//! border; coal climbs its quota bar in the PLAN block while households
//! move in; and the clip closes on the fullscreen Plan ledger — targets,
//! fulfillment, treasury, the next tranche forecast. The real acceptance is
//! the unscripted 30-minute session; this is the record.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_g1 -- frames <outdir> <count>

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
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const WARMUP: u32 = 90;
const ROLL_OUT: u32 = WARMUP + 20;

/// Beat 1: the town site — road, mine, warehouse, depot, people.
const TOWN_AT: u32 = 20;
/// Beat 2: trucks bought at the border, coal starts climbing its quota.
const TRUCKS_AT: u32 = 90;
/// Beat 3: dwellings — the housed quota answers.
const HOMES_AT: u32 = 180;
/// Beat 4: the ledger — hold the signature screen to the end.
const LEDGER_AT: u32 = 330;

const CUSTOMS_POS: Vec3 = Vec3::new(-260.0, 0.0, 0.0);
const MINE_POS: Vec3 = Vec3::new(-20.0, 0.0, -20.0);
const WAREHOUSE_POS: Vec3 = Vec3::new(14.0, 0.0, -22.0);
const DEPOT_POS: Vec3 = Vec3::new(-2.0, 0.0, 18.0);

const DWELLINGS: u32 = 8;

fn dwelling_pos(d: u32) -> Vec3 {
    Vec3::new(
        40.0 + (d % 4) as f32 * 12.0,
        0.0,
        4.0 + (d / 4) as f32 * 12.0,
    )
}

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
            eprintln!("[capture] usage: capture_g1 frames <outdir> <count>");
            std::process::exit(2);
        }
    };
    std::fs::create_dir_all(&dir).expect("[capture] cannot create output dir");

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
    .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        1.0 / FPS,
    )))
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
        // DispatchSimPlugin auto-adds Pathfinding + Traffic.
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

/// Fiat-complete construction sites: this clip is about the Plan's loop,
/// not the phased build (M7 told that story).
fn fiat_complete(world: &mut World) {
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
        if kind == BuildingKind::Mine {
            // A worked seam: the yard starts with coal for the first hauls.
            world
                .get_mut::<Inventory>(entity)
                .unwrap()
                .add(ResourceKind::Coal, 30.0);
            world
                .entity_mut(entity)
                .remove::<soviet_simulator::sim::labour::Staffing>();
        }
    }
}

fn drive_script(world: &mut World) {
    world.resource_mut::<FrameNo>().0 += 1;
    let f = world.resource::<FrameNo>().0;
    let total = world.resource::<CaptureConfig>().frames;

    if f == 4 {
        *world.resource_mut::<SimSpeed>() = SimSpeed::Paused;
    }
    if f == ROLL_OUT {
        *world.resource_mut::<SimSpeed>() = SimSpeed::Quad;
    }

    let clip = f.saturating_sub(WARMUP);
    match clip {
        // Beat 1: the town site goes down east of the border road.
        TOWN_AT => {
            world
                .resource_mut::<RoadEditQueue>()
                .0
                .push(RoadEdit::Place {
                    from: CUSTOMS_POS + Vec3::new(12.0, 0.0, 0.0),
                    to: Vec3::new(70.0, 0.0, 0.0),
                    class: RoadClass::Dirt,
                });
            let mut buildings = world.resource_mut::<BuildingEditQueue>();
            for (kind, pos) in [
                (BuildingKind::Mine, MINE_POS),
                (BuildingKind::Warehouse, WAREHOUSE_POS),
                (BuildingKind::Depot, DEPOT_POS),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
        // Beat 2: the treasury spends — trucks enter at the border and the
        // mine-to-warehouse shuttle starts the coal quota climbing.
        TRUCKS_AT => {
            fiat_complete(world);
            world
                .resource_mut::<soviet_simulator::sim::households::RecruitmentPlan>()
                .target_households = DWELLINGS;
            let (depot, mine, warehouse) = {
                let mut q = world.query::<(Entity, &Building)>();
                let mut depot = None;
                let mut mine = None;
                let mut warehouse = None;
                for (e, b) in q.iter(world) {
                    match b.kind {
                        BuildingKind::Depot => depot = Some(e),
                        BuildingKind::Mine => mine = Some(e),
                        BuildingKind::Warehouse => warehouse = Some(e),
                        _ => {}
                    }
                }
                (depot.unwrap(), mine.unwrap(), warehouse.unwrap())
            };
            let mut vehicles = world.resource_mut::<VehicleEditQueue>();
            vehicles.0.push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
            vehicles.0.push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
            vehicles.0.push(VehicleEdit::CreateShuttle {
                from: mine,
                to: warehouse,
                resource: ResourceKind::Coal,
            });
        }
        // Beat 3: dwellings — the housed quota answers the housing office.
        HOMES_AT => {
            let mut buildings = world.resource_mut::<BuildingEditQueue>();
            for d in 0..DWELLINGS {
                buildings.0.push(BuildingEdit::Place {
                    kind: BuildingKind::Dwelling,
                    pos: dwelling_pos(d),
                });
            }
        }
        n if n == HOMES_AT + 20 => {
            fiat_complete(world);
        }
        // Beat 4: the signature screen — hold the ledger to the end.
        LEDGER_AT => {
            let panel = world
                .query_filtered::<Entity, With<soviet_simulator::game::hud::PlanLedgerPanel>>()
                .iter(world)
                .next();
            if let Some(panel) = panel {
                world.get_mut::<Node>(panel).unwrap().display = Display::Flex;
            }
        }
        _ => {}
    }

    // Camera: open on the border gatehouse, pull east to the town site,
    // then settle as the ledger takes the screen.
    let t = (clip as f32 / total as f32).clamp(0.0, 1.0);
    let mut rig = world.resource_mut::<CameraRig>();
    let ease = t * t * (3.0 - 2.0 * t);
    rig.focus = CUSTOMS_POS.lerp(Vec3::new(10.0, 0.0, 0.0), (ease * 1.6).min(1.0));
    rig.dist = 120.0 + 45.0 * ease;
    rig.yaw = -0.30;
    rig.pitch = (-46f32).to_radians();
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
