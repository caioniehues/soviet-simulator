//! B8.4 acceptance-demo capture: offscreen deterministic render of The Web.
//! A dwelling block sits dark while winter closes in (the SEASON line on the
//! HUD falls frame by frame). The planner answers with utilities in three
//! beats: a power plant and wires light the homes; a water pump and sewage
//! works close the water cycle with pipes; and finally — with the cold now
//! deep enough that unheated homes are suffering — a heat plant is piped in
//! and the block warms up. Same solver, three webs, one component each.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_m8 -- frames <outdir> <count>

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
use soviet_simulator::sim::heat::Climate;
use soviet_simulator::sim::resources::{Inventory, ResourceKind};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
use soviet_simulator::sim::storage::default_policies;
use soviet_simulator::sim::vehicles::VehicleSimPlugin;
use soviet_simulator::sim::wires::{NetKind, WireEdit, WireEditQueue};
use soviet_simulator::sim::{SimPlugin, SimSpeed};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const WARMUP: u32 = 90;
const ROLL_OUT: u32 = WARMUP + 20;

/// Beat 1: the grid — plant down, then wires, homes light up.
const POWER_AT: u32 = 20;
const POWER_WIRES_AT: u32 = 40;
/// Beat 2: the water cycle — pump + treatment works, then pipes.
const WATER_AT: u32 = 150;
const WATER_PIPES_AT: u32 = 170;
/// Beat 3: district heat, landed only once winter has real teeth.
const HEAT_AT: u32 = 280;
const HEAT_PIPES_AT: u32 = 340;

/// Season drive across the clip: autumn into midwinter.
const TEMP_START: f32 = 12.0;
const TEMP_END: f32 = -12.0;

/// The dwelling block: a 4×3 grid east of the plant row.
const DWELLINGS: u32 = 12;

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

fn dwelling_pos(d: u32) -> Vec3 {
    Vec3::new(20.0 + (d % 4) as f32 * 12.0, 0.0, -18.0 + (d / 4) as f32 * 12.0)
}

const POWER_PLANT_POS: Vec3 = Vec3::new(-24.0, 0.0, -30.0);
const WATER_PUMP_POS: Vec3 = Vec3::new(-24.0, 0.0, -6.0);
const SEWAGE_POS: Vec3 = Vec3::new(-24.0, 0.0, 16.0);
const HEAT_PLANT_POS: Vec3 = Vec3::new(-24.0, 0.0, 40.0);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (dir, frames) = match args.as_slice() {
        [_, mode, dir, count] if mode == "frames" => (
            PathBuf::from(dir),
            count.parse::<u32>().expect("[capture] bad frame count"),
        ),
        _ => {
            eprintln!("[capture] usage: capture_m8 frames <outdir> <count>");
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

/// Fiat-complete anything still a construction site: this clip is about the
/// webs, not the phased build (that was M7's story). Burners get a bunker
/// of coal so fuel never gates the demo.
fn fiat_complete(world: &mut World) {
    let found: Vec<(Entity, BuildingKind)> = {
        let mut q = world.query_filtered::<(Entity, &Building), With<ConstructionSite>>();
        q.iter(world).map(|(e, b)| (e, b.kind)).collect()
    };
    for (entity, kind) in found {
        world.entity_mut(entity).remove::<ConstructionSite>();
        let capacity = match kind {
            BuildingKind::PowerPlant | BuildingKind::HeatPlant => 200.0,
            _ => kind.inventory_capacity(),
        };
        world
            .entity_mut(entity)
            .insert((Inventory::new(capacity), default_policies(kind)));
        if matches!(kind, BuildingKind::PowerPlant | BuildingKind::HeatPlant) {
            world
                .get_mut::<Inventory>(entity)
                .unwrap()
                .add(ResourceKind::Coal, 200.0);
        }
        // The utility plants run free of shift cycles: this clip is about
        // the webs, not attendance (removing Staffing is the labour-plugin
        // fiat equivalent of the headless-fixture path).
        if matches!(
            kind,
            BuildingKind::PowerPlant
                | BuildingKind::HeatPlant
                | BuildingKind::WaterPump
                | BuildingKind::SewagePlant
        ) {
            world
                .entity_mut(entity)
                .remove::<soviet_simulator::sim::labour::Staffing>();
        }
    }
}

/// Daisy-chain a plant through the whole block on one NetKind.
fn chain(wires: &mut WireEditQueue, kind: NetKind, plant: Vec3) {
    let mut prev = plant;
    for d in 0..DWELLINGS {
        let next = dwelling_pos(d);
        wires.0.push(WireEdit::Place {
            from: prev,
            to: next,
            kind,
        });
        prev = next;
    }
}

fn drive_script(world: &mut World) {
    world.resource_mut::<FrameNo>().0 += 1;
    let f = world.resource::<FrameNo>().0;
    let total = world.resource::<CaptureConfig>().frames;

    match f {
        1 => {
            world.resource_mut::<RoadEditQueue>().0.push(RoadEdit::Place {
                from: Vec3::new(-40.0, 0.0, 0.0),
                to: Vec3::new(70.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
            let mut buildings = world.resource_mut::<BuildingEditQueue>();
            for d in 0..DWELLINGS {
                buildings.0.push(BuildingEdit::Place {
                    kind: BuildingKind::Dwelling,
                    pos: dwelling_pos(d),
                });
            }
        }
        3 => {
            fiat_complete(world);
            // A lived-in block: households recruited into the flats.
            world
                .resource_mut::<soviet_simulator::sim::households::RecruitmentPlan>()
                .target_households = 12;
            // The season is ours to script.
            let mut climate = world.resource_mut::<Climate>();
            climate.auto = false;
            climate.temperature = TEMP_START;
        }
        4 => {
            *world.resource_mut::<SimSpeed>() = SimSpeed::Paused;
            // The population panel grew a SEASON line this milestone; drop
            // the dispatch panel below it so the temperature stays readable.
            let panel = world
                .query::<(Entity, &Name)>()
                .iter(world)
                .find(|(_, n)| n.as_str() == "HudDispatchPanel")
                .map(|(e, _)| e);
            if let Some(panel) = panel {
                world.get_mut::<Node>(panel).unwrap().top = Val::Px(230.0);
            }
        }
        ROLL_OUT => *world.resource_mut::<SimSpeed>() = SimSpeed::Quad,
        _ => {}
    }

    let clip = f.saturating_sub(WARMUP);
    match clip {
        // Beat 1: electricity. Plant down...
        POWER_AT => {
            world.resource_mut::<BuildingEditQueue>().0.push(BuildingEdit::Place {
                kind: BuildingKind::PowerPlant,
                pos: POWER_PLANT_POS,
            });
        }
        // ...wires strung — the block lights up.
        POWER_WIRES_AT => {
            fiat_complete(world);
            world.resource_scope(|_, mut wires: Mut<WireEditQueue>| {
                chain(&mut wires, NetKind::Power, POWER_PLANT_POS);
            });
            // Pin the inspect panel to a home: its gates flip on camera.
            let home = world
                .query::<(Entity, &Building)>()
                .iter(world)
                .find(|(_, b)| b.kind == BuildingKind::Dwelling)
                .map(|(e, _)| e);
            world.resource_mut::<soviet_simulator::game::hud::Selected>().0 = home;
        }
        // Beat 2: the water cycle — both ends, then pipes.
        WATER_AT => {
            let mut buildings = world.resource_mut::<BuildingEditQueue>();
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::WaterPump,
                pos: WATER_PUMP_POS,
            });
            buildings.0.push(BuildingEdit::Place {
                kind: BuildingKind::SewagePlant,
                pos: SEWAGE_POS,
            });
        }
        WATER_PIPES_AT => {
            fiat_complete(world);
            world.resource_scope(|_, mut wires: Mut<WireEditQueue>| {
                chain(&mut wires, NetKind::Water, WATER_PUMP_POS);
                // Close the cycle: the treatment works joins the same web.
                wires.0.push(WireEdit::Place {
                    from: SEWAGE_POS,
                    to: dwelling_pos(0),
                    kind: NetKind::Water,
                });
            });
        }
        // Beat 3: winter has teeth now — the heat plant answers.
        HEAT_AT => {
            world.resource_mut::<BuildingEditQueue>().0.push(BuildingEdit::Place {
                kind: BuildingKind::HeatPlant,
                pos: HEAT_PLANT_POS,
            });
        }
        HEAT_PIPES_AT => {
            fiat_complete(world);
            world.resource_scope(|_, mut wires: Mut<WireEditQueue>| {
                chain(&mut wires, NetKind::Heat, HEAT_PLANT_POS);
            });
        }
        _ => {}
    }

    // Winter closes in across the whole clip: the SEASON line falls from
    // autumn to deep midwinter while the webs go up one by one.
    let t = (clip as f32 / total as f32).clamp(0.0, 1.0);
    if !world.resource::<Climate>().auto {
        world.resource_mut::<Climate>().temperature =
            TEMP_START + (TEMP_END - TEMP_START) * t;
    }

    // Hold the block; slight drift from the homes toward the plant row.
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(24.0, 0.0, -2.0).lerp(Vec3::new(6.0, 0.0, 2.0), t);
    rig.dist = 150.0 - 15.0 * t;
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
