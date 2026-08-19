//! R0 acceptance record: The State Document.
//!
//! The rung is about how the game *looks and reads*, so the clip is staged to
//! put each of R0's five claims on screen where they can be judged:
//!
//! 1. **The sky and the ground.** Opens low over empty field so the horizon,
//!    the gradient dome and the desaturated olive are all in frame — the shot
//!    the G1 capture could not give, because its camera never left −46°.
//! 2. **Grounding.** Buildings drop in against a low sun; the contact
//!    shadows and ambient occlusion are the point of the beat.
//! 3. **Juice.** Each placement pops; then a hover outline and a selection
//!    outline, one after the other, on the same building.
//! 4. **Alarm.** A warehouse is starved on purpose: the critical line, the
//!    toast, and the event log behind `L`.
//! 5. **The document.** The restyled Plan ledger closes the clip.
//!
//! Run from the crate root:
//!   cargo run --release --bin capture_r0 -- frames <outdir> <count>

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
use soviet_simulator::game::hud::{PlanLedgerPanel, Selected};
use soviet_simulator::game::juice::Hovered;
use soviet_simulator::sim::SimSpeed;
use soviet_simulator::sim::buildings::{Building, BuildingEdit, BuildingEditQueue, BuildingKind};
use soviet_simulator::sim::construction::ConstructionSite;
use soviet_simulator::sim::resources::{Inventory, ResourceKind, TransportClass};
use soviet_simulator::sim::roads::{RoadClass, RoadEdit, RoadEditQueue};
use soviet_simulator::sim::storage::default_policies;
use soviet_simulator::sim::vehicles::{VehicleEdit, VehicleEditQueue};

const FPS: f64 = 30.0;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const WARMUP: u32 = 90;

/// Beat 2: the town goes down, one building at a time so each pop is visible.
const TOWN_AT: u32 = 60;
/// Beat 3: hover, then select.
const HOVER_AT: u32 = 190;
const SELECT_AT: u32 = 240;
/// Beat 4: the fleet arrives and the warehouse's gravel demand goes unmet.
const TRUCKS_AT: u32 = 285;
const LOG_AT: u32 = 360;
/// Beat 5: the ledger holds to the end.
const LEDGER_AT: u32 = 420;

const MINE_POS: Vec3 = Vec3::new(-24.0, 0.0, -18.0);
const WAREHOUSE_POS: Vec3 = Vec3::new(16.0, 0.0, -22.0);
const DEPOT_POS: Vec3 = Vec3::new(-4.0, 0.0, 20.0);
const PLANT_POS: Vec3 = Vec3::new(44.0, 0.0, 6.0);

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
            eprintln!("[capture] usage: capture_r0 frames <outdir> <count>");
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
    // `SimPlugins` is every sim plugin the game runs — the HUD hard-requires
    // their resources, and a capture bin silently rots the moment one is
    // missed by hand-picking a subset instead.
    .add_plugins(SimPlugins)
    .add_plugins(GamePlugins);

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
    .add_systems(Update, (drive_script, shoot_frame, finish).chain())
    // After every Update system, so it beats `juice::track_hover`, which
    // clears the hover each frame from a cursor no windowless run can have.
    .add_systems(PostUpdate, force_hover);

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

/// Fiat-complete construction sites: this clip is about how the game reads,
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
            world
                .get_mut::<Inventory>(entity)
                .unwrap()
                .add(ResourceKind::Coal, 40.0);
            world
                .entity_mut(entity)
                .remove::<soviet_simulator::sim::labour::Staffing>();
        }
        if kind == BuildingKind::PowerPlant {
            // Fuelled, so its chimneys smoke at load for the grounding beat.
            world
                .get_mut::<Inventory>(entity)
                .unwrap()
                .add(ResourceKind::Coal, 60.0);
            world
                .entity_mut(entity)
                .remove::<soviet_simulator::sim::labour::Staffing>();
        }
    }
}

fn building_of(world: &mut World, want: BuildingKind) -> Option<Entity> {
    let mut q = world.query::<(Entity, &Building)>();
    q.iter(world).find(|(_, b)| b.kind == want).map(|(e, _)| e)
}

fn drive_script(world: &mut World) {
    world.resource_mut::<FrameNo>().0 += 1;
    let f = world.resource::<FrameNo>().0;
    let total = world.resource::<CaptureConfig>().frames;

    if f == 4 {
        *world.resource_mut::<SimSpeed>() = SimSpeed::Paused;
    }
    if f == WARMUP + TOWN_AT {
        *world.resource_mut::<SimSpeed>() = SimSpeed::Double;
    }

    let clip = f.saturating_sub(WARMUP);
    match clip {
        // Beat 2: one building per beat, so each placement pop lands alone.
        TOWN_AT => {
            world
                .resource_mut::<RoadEditQueue>()
                .0
                .push(RoadEdit::Place {
                    from: Vec3::new(-70.0, 0.0, 0.0),
                    to: Vec3::new(80.0, 0.0, 0.0),
                    class: RoadClass::Dirt,
                });
            place(world, BuildingKind::Mine, MINE_POS);
        }
        n if n == TOWN_AT + 22 => place(world, BuildingKind::Warehouse, WAREHOUSE_POS),
        n if n == TOWN_AT + 44 => place(world, BuildingKind::Depot, DEPOT_POS),
        n if n == TOWN_AT + 66 => {
            place(world, BuildingKind::PowerPlant, PLANT_POS);
            fiat_complete(world);
        }
        n if n == TOWN_AT + 80 => fiat_complete(world),
        // Beat 3: hover is forced from `PostUpdate` (see `force_hover`) —
        // writing it here would be overwritten the same frame by the real
        // tracker, which reads a ground cursor that is always `None` in a
        // windowless capture.
        SELECT_AT => {
            if let Some(plant) = building_of(world, BuildingKind::PowerPlant) {
                world.resource_mut::<Selected>().0 = Some(plant);
            }
        }
        // Beat 4: the shuttle runs and the warehouse's gravel band goes unmet
        // — the deficit board raises the critical line and its toast.
        TRUCKS_AT => {
            world.resource_mut::<Selected>().0 = None;
            world.resource_mut::<Hovered>().0 = None;
            let (Some(depot), Some(mine), Some(warehouse)) = (
                building_of(world, BuildingKind::Depot),
                building_of(world, BuildingKind::Mine),
                building_of(world, BuildingKind::Warehouse),
            ) else {
                return;
            };
            let mut vehicles = world.resource_mut::<VehicleEditQueue>();
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
        // The event log: proof the toasts are recorded, not just flashed.
        LOG_AT => press(world, KeyCode::KeyL),
        n if n == LOG_AT + 45 => press(world, KeyCode::KeyL),
        LEDGER_AT => {
            let panel = world
                .query_filtered::<Entity, With<PlanLedgerPanel>>()
                .iter(world)
                .next();
            if let Some(panel) = panel {
                world.get_mut::<Node>(panel).unwrap().display = Display::Flex;
            }
        }
        _ => {}
    }

    // Camera. Beat 1 sits low so the sky dome and the horizon are in frame —
    // the grounding pass and the gradient are both judged from this shot —
    // then rises to the working RTS angle for the rest.
    let t = (clip as f32 / total as f32).clamp(0.0, 1.0);
    let rise = ((clip as f32 - TOWN_AT as f32) / 90.0).clamp(0.0, 1.0);
    let ease = rise * rise * (3.0 - 2.0 * rise);
    let mut rig = world.resource_mut::<CameraRig>();
    rig.focus = Vec3::new(-30.0, 0.0, 30.0).lerp(Vec3::new(6.0, 0.0, -4.0), ease);
    rig.dist = 95.0 + 75.0 * ease;
    rig.yaw = -0.30 + 0.22 * t;
    rig.pitch = (-13.0 - 30.0 * ease).to_radians();
}

/// Hold the plant hovered for beat 3, so the pre-highlight outline is on
/// screen long enough to be judged against the selection outline that follows.
fn force_hover(
    frame: Res<FrameNo>,
    plants: Query<(Entity, &Building)>,
    mut hovered: ResMut<Hovered>,
) {
    let clip = frame.0.saturating_sub(WARMUP);
    if !(HOVER_AT..SELECT_AT).contains(&clip) {
        return;
    }
    let plant = plants
        .iter()
        .find(|(_, b)| b.kind == BuildingKind::PowerPlant)
        .map(|(e, _)| e);
    if hovered.0 != plant {
        hovered.0 = plant;
    }
}

fn place(world: &mut World, kind: BuildingKind, pos: Vec3) {
    world
        .resource_mut::<BuildingEditQueue>()
        .0
        .push(BuildingEdit::Place { kind, pos });
}

/// Drive a keypress from the script — capture bins have no window, so input
/// is written into `ButtonInput` directly.
fn press(world: &mut World, key: KeyCode) {
    world.resource_mut::<ButtonInput<KeyCode>>().press(key);
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
