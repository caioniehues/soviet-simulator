//! Legibility pass: HUD overlay (active tool, key legend, sim speed),
//! inspect-click panel, yard/cargo fill bars, and time controls. Presentation
//! only — reads sim state, writes nothing but `SimSpeed` (a player control,
//! not sim state).

use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use crate::sim::buildings::{Building, BuildingKind, PowerOutput, Powered};
use crate::sim::citizens::Citizen;
use crate::sim::dispatch::{DeficitBoard, DispatchQueue, FreightJob, FreightPhase};
use crate::sim::households::{Household, HousingQueue, RecruitmentPlan};
use crate::sim::labour::Staffing;
use crate::sim::resources::{Inventory, ResourceKind};
use crate::sim::roads::RoadBuildFeedback;
use crate::sim::resources::TransportClass;
use crate::sim::storage::StoragePolicies;
use crate::sim::vehicles::{
    ActivePawn, ActiveVehicle, DEPOT_SLOTS, VehicleAsset, VehicleEdit, VehicleEditQueue,
};
use crate::sim::{SimSpeed, TickIndex};

#[derive(Component)]
struct ToolReadout;

#[derive(Component)]
struct InspectReadout;

#[derive(Component)]
struct PopulationReadout;

/// The fleet-legibility panel (#36): pending orders, busy/idle per depot,
/// oldest starving deficit.
#[derive(Component)]
struct DispatchReadout;

/// The inspect panel's chrome node; hidden while nothing is selected.
#[derive(Component)]
struct InspectPanel;

/// Building picked with the Inspect tool.
#[derive(Resource, Default)]
struct Selected(Option<Entity>);

/// Which resource the band-tuning keys act on for the selected storage.
#[derive(Resource, Default)]
struct BandFocus(usize);

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .init_resource::<BandFocus>()
            .add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                (
                    drive_time_controls,
                    drive_recruitment_controls,
                    update_population_readout,
                    update_dispatch_readout,
                    drive_inspect_tool,
                    drive_depot_purchase,
                    drive_band_tuning,
                    hint_starving_deficits,
                    update_tool_readout,
                    update_inspect_readout,
                    draw_selection_ring,
                    draw_fill_bars,
                ),
            );
    }
}

/// Panel chrome per the art direction: near-black concrete panel, thin rust
/// accent along the left edge, Fira Sans (OFL, bundled).
fn panel_node() -> (Node, BackgroundColor, BorderColor) {
    (
        Node {
            position_type: PositionType::Absolute,
            padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
            border: UiRect::left(Val::Px(3.0)),
            border_radius: BorderRadius::px(2.0, 6.0, 6.0, 2.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.055, 0.06, 0.07, 0.86)),
        BorderColor::all(Color::srgb(0.63, 0.35, 0.20)),
    )
}

fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = TextFont {
        font: asset_server.load("fonts/FiraSans-Regular.ttf").into(),
        font_size: bevy::text::FontSize::Px(15.0),
        ..default()
    };
    let (node, bg, border) = panel_node();
    commands
        .spawn((
            Node {
                left: Val::Px(12.0),
                top: Val::Px(10.0),
                ..node
            },
            bg,
            border,
            Name::new("HudToolPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ToolReadout,
                Text::new(""),
                font.clone(),
                TextColor(Color::srgb(0.92, 0.90, 0.82)),
            ));
        });
    let (node, bg, border) = panel_node();
    commands
        .spawn((
            Node {
                right: Val::Px(12.0),
                top: Val::Px(10.0),
                ..node
            },
            bg,
            border,
            Name::new("HudPopulationPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                PopulationReadout,
                Text::new(""),
                font.clone(),
                TextColor(Color::srgb(0.92, 0.90, 0.82)),
            ));
        });
    let (node, bg, border) = panel_node();
    commands
        .spawn((
            Node {
                right: Val::Px(12.0),
                top: Val::Px(96.0),
                ..node
            },
            bg,
            border,
            Name::new("HudDispatchPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                DispatchReadout,
                Text::new(""),
                font.clone(),
                TextColor(Color::srgb(0.88, 0.87, 0.80)),
            ));
        });
    let (node, bg, border) = panel_node();
    commands
        .spawn((
            Node {
                left: Val::Px(12.0),
                bottom: Val::Px(12.0),
                display: Display::None,
                ..node
            },
            bg,
            border,
            InspectPanel,
            Name::new("HudInspectPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                InspectReadout,
                Text::new(""),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
                    ..font
                },
                TextColor(Color::srgb(0.95, 0.93, 0.85)),
            ));
        });
}

fn drive_time_controls(keys: Res<ButtonInput<KeyCode>>, mut speed: ResMut<SimSpeed>) {
    if keys.just_pressed(KeyCode::Space) {
        *speed = match *speed {
            SimSpeed::Paused => SimSpeed::Normal,
            _ => SimSpeed::Paused,
        };
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        *speed = match *speed {
            SimSpeed::Paused => SimSpeed::Normal,
            SimSpeed::Normal => SimSpeed::Double,
            _ => SimSpeed::Quad,
        };
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        *speed = match *speed {
            SimSpeed::Quad => SimSpeed::Double,
            SimSpeed::Double => SimSpeed::Normal,
            _ => SimSpeed::Paused,
        };
    }
}

/// The plan's immigration lever: +/- adjusts the recruitment target.
fn drive_recruitment_controls(keys: Res<ButtonInput<KeyCode>>, mut plan: ResMut<RecruitmentPlan>) {
    if keys.just_pressed(KeyCode::Equal) {
        plan.target_households += 1;
    }
    if keys.just_pressed(KeyCode::Minus) {
        plan.target_households = plan.target_households.saturating_sub(1);
    }
}

fn update_population_readout(
    citizens: Query<&Citizen>,
    households: Query<&Household>,
    queue: Res<HousingQueue>,
    plan: Res<RecruitmentPlan>,
    mut readout: Query<&mut Text, With<PopulationReadout>>,
) {
    let Ok(mut text) = readout.single_mut() else {
        return;
    };
    let total = households.iter().count();
    let housed = households.iter().filter(|h| h.dwelling.is_some()).count();
    let next = format!(
        "POPULATION {}\nhouseholds {housed}/{total} housed   queue {}\nplan target {}   (+/- adjusts)",
        citizens.iter().count(),
        queue.0.len(),
        plan.target_households,
    );
    if text.0 != next {
        text.0 = next;
    }
}

/// Fiat truck purchase (manufacture is B10): with a depot selected in the
/// Inspect tool, `T` buys a bulk tipper, `Y` a covered bed. The slot gate
/// lives sim-side — a full apron drops the edit with a warn.
fn drive_depot_purchase(
    keys: Res<ButtonInput<KeyCode>>,
    selected: Res<Selected>,
    buildings: Query<&Building>,
    mut edits: ResMut<VehicleEditQueue>,
) {
    let Some(depot) = selected.0.filter(|&e| {
        buildings
            .get(e)
            .is_ok_and(|b| b.kind == BuildingKind::Depot)
    }) else {
        return;
    };
    let class = if keys.just_pressed(KeyCode::KeyT) {
        TransportClass::Bulk
    } else if keys.just_pressed(KeyCode::KeyY) {
        TransportClass::Covered
    } else {
        return;
    };
    edits.0.push(VehicleEdit::BuyTruck { depot, class });
    info!("depot purchase queued: {class:?} truck at {depot:?}");
}

/// Player policy control (the B3 dial): with a storage selected, `B` cycles
/// the focused resource, `,`/`.` lower/raise its min line by 5%, and with
/// Shift the max line instead. Writing `StoragePolicies` here is player
/// intent, not sim state — the same standing as `SimSpeed`.
fn drive_band_tuning(
    keys: Res<ButtonInput<KeyCode>>,
    selected: Res<Selected>,
    mut focus: ResMut<BandFocus>,
    mut policies: Query<&mut StoragePolicies>,
    inventories: Query<&Inventory>,
) {
    let Some(entity) = selected.0 else { return };
    let Ok(mut policies) = policies.get_mut(entity) else {
        return;
    };
    if inventories.get(entity).is_ok_and(|i| i.capacity <= 0.0) {
        return; // depots store nothing — nothing to band
    }
    if keys.just_pressed(KeyCode::KeyB) {
        focus.0 = (focus.0 + 1) % ResourceKind::COUNT;
    }
    let step = if keys.just_pressed(KeyCode::Comma) {
        -0.05
    } else if keys.just_pressed(KeyCode::Period) {
        0.05
    } else {
        return;
    };
    let resource = ResourceKind::ALL[focus.0];
    let band = policies.band(resource);
    let (mut min, mut max) = band.map_or((0.0, 0.0), |b| (b.min_pct, b.max_pct));
    let shift =
        keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    if shift {
        max += step;
    } else {
        min += step;
    }
    policies.set(
        resource,
        Some(crate::sim::storage::StorageBand::new(min, max.max(min))),
    );
}

/// A starving deficit is never invisible: the building the matcher cannot
/// feed pulses a red ring (the starvation board drives it, so it clears the
/// moment supply reaches the bucket).
fn hint_starving_deficits(
    time: Res<Time>,
    board: Res<DeficitBoard>,
    buildings: Query<&Building>,
    mut gizmos: Gizmos,
) {
    let pulse = 0.5 + 0.5 * (time.elapsed_secs() * 4.0).sin();
    let color = Color::srgb(0.9, 0.15, 0.1).with_alpha(0.25 + 0.6 * pulse);
    for deficit in &board.0 {
        let Ok(building) = buildings.get(deficit.building) else {
            continue;
        };
        gizmos.circle(
            Isometry3d::new(
                building.pos + Vec3::Y * 0.2,
                Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ),
            building.kind.footprint().length() * 0.5 + 2.5 + pulse,
            color,
        );
    }
}

/// Ticks → game hours (600 frames per game day).
fn game_hours(ticks: u32) -> f32 {
    ticks as f32 * 24.0 / crate::sim::clock::FRAMES_PER_GAME_DAY as f32
}

fn update_dispatch_readout(
    queue: Res<DispatchQueue>,
    board: Res<DeficitBoard>,
    tick: Res<TickIndex>,
    fleet: Query<(&VehicleAsset, Has<ActivePawn>)>,
    buildings: Query<&Building>,
    mut readout: Query<&mut Text, With<DispatchReadout>>,
) {
    let Ok(mut text) = readout.single_mut() else {
        return;
    };
    let (mut busy, mut idle) = (0, 0);
    let mut per_depot: std::collections::HashMap<Entity, (u32, u32)> =
        std::collections::HashMap::new();
    for (asset, on_road) in &fleet {
        let slot = per_depot.entry(asset.home_depot).or_default();
        if on_road {
            busy += 1;
            slot.0 += 1;
        } else {
            idle += 1;
            slot.1 += 1;
        }
    }
    let waiting = queue.orders.iter().filter(|o| o.assigned.is_none()).count();
    let mut lines = format!(
        "DISPATCH   fleet {busy} busy / {idle} idle\norders {} in flight, {waiting} waiting",
        queue.orders.len() - waiting,
    );
    for (depot, (out, parked)) in &per_depot {
        if let Ok(building) = buildings.get(*depot) {
            lines.push_str(&format!(
                "\n  depot #{}: {parked} parked, {out} out",
                building.id.0
            ));
        }
    }
    // The queue is the planning signal: the oldest waiting orders read first.
    let mut pending: Vec<_> = queue.orders.iter().filter(|o| o.assigned.is_none()).collect();
    pending.sort_by_key(|o| o.issued_tick);
    for order in pending.iter().take(4) {
        lines.push_str(&format!(
            "\n  {:?} {:.0} t — waiting {:.1} h",
            order.resource,
            order.qty,
            game_hours(tick.0.saturating_sub(order.issued_tick)),
        ));
    }
    if pending.len() > 4 {
        lines.push_str(&format!("\n  … and {} more", pending.len() - 4));
    }
    if let Some(starving) = board.0.iter().min_by_key(|d| d.since_tick) {
        let name = buildings
            .get(starving.building)
            .map(|b| format!("{:?} #{}", b.kind, b.id.0))
            .unwrap_or_else(|_| "?".into());
        lines.push_str(&format!(
            "\nSTARVING: {name} short {:.1} t {:?} for {:.1} h",
            starving.deficit,
            starving.resource,
            game_hours(tick.0.saturating_sub(starving.since_tick)),
        ));
    }
    if text.0 != lines {
        text.0 = lines;
    }
}

fn tool_label(mode: ToolMode) -> String {
    match mode {
        ToolMode::Inspect => "INSPECT - click a building".into(),
        ToolMode::Road(class) => {
            format!("ROAD ({class:?}) - click-chain, right-click ends, X cuts, R rebuilds last cut")
        }
        ToolMode::Building(kind) => format!("BUILD ({kind:?}) - click places, 3 cycles kind"),
        ToolMode::Wire => "WIRE - click-chain hops, right-click ends, X cuts".into(),
        ToolMode::Shuttle => {
            "HAUL POLICY - click source (export all), then destination (import to 90%)".into()
        }
    }
}

fn speed_label(speed: SimSpeed) -> &'static str {
    match speed {
        SimSpeed::Paused => "PAUSED",
        SimSpeed::Normal => "1x",
        SimSpeed::Double => "2x",
        SimSpeed::Quad => "4x",
    }
}

fn update_tool_readout(
    mode: Res<ToolMode>,
    speed: Res<SimSpeed>,
    feedback: Res<RoadBuildFeedback>,
    mut readout: Query<&mut Text, With<ToolReadout>>,
) {
    if !mode.is_changed() && !speed.is_changed() && !feedback.is_changed() {
        return;
    }
    let Ok(mut text) = readout.single_mut() else {
        return;
    };
    text.0 = format!(
        "{}   |   speed {}\n\
         1 dirt road   2 paved road   3 building   4 wire   5 haul policy   Esc inspect\n\
         WASD/arrows pan   Q/E rotate   wheel zoom   Space pause   [ ] speed",
        tool_label(*mode),
        speed_label(*speed),
    );
    if let Some(shortfall) = feedback.0 {
        text.0.push_str(&format!(
            "\nNOT ENOUGH GRAVEL DELIVERED: paving needs {:.1} t, {:.1} t in yards nearby",
            shortfall.needed,
            shortfall.available.max(0.0),
        ));
    }
}

fn drive_inspect_tool(
    mode: Res<ToolMode>,
    cursor: Res<GroundCursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    buildings: Query<(Entity, &Building)>,
    mut selected: ResMut<Selected>,
) {
    if *mode != ToolMode::Inspect {
        selected.0 = None;
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(pos) = cursor.0 else { return };
    selected.0 = buildings
        .iter()
        .map(|(e, b)| {
            let reach = b.kind.footprint().length() * 0.5 + 2.0;
            (e, b.pos.distance_squared(pos), reach)
        })
        .filter(|(_, d2, reach)| *d2 <= reach * reach)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, ..)| e);
}

/// A ten-cell text bar with the band's min/max marked: `[##·····|··]`.
/// `|` is the min line, fill runs to the current stock.
fn band_bar(current: f32, capacity: f32, min_pct: f32) -> String {
    let cells = 10usize;
    let fill = ((current / capacity.max(1e-3)) * cells as f32).round() as usize;
    let min_cell = (min_pct * cells as f32).round() as usize;
    let mut bar = String::from("[");
    for i in 0..cells {
        if i == min_cell && min_cell > 0 {
            bar.push('|');
        }
        bar.push(if i < fill { '#' } else { '·' });
    }
    bar.push(']');
    bar
}

#[allow(clippy::type_complexity)]
fn update_inspect_readout(
    selected: Res<Selected>,
    focus: Res<BandFocus>,
    buildings: Query<(
        &Building,
        &Inventory,
        Option<&Powered>,
        Option<&PowerOutput>,
        Option<&Staffing>,
        Option<&StoragePolicies>,
    )>,
    fleet: Query<(&VehicleAsset, Has<ActivePawn>, Option<&FreightJob>)>,
    mut readout: Query<&mut Text, With<InspectReadout>>,
    mut panel: Query<&mut Node, With<InspectPanel>>,
) {
    let Ok(mut text) = readout.single_mut() else {
        return;
    };
    let Ok(mut panel) = panel.single_mut() else {
        return;
    };
    let Some((building, inventory, powered, output, staffing, policies)) =
        selected.0.and_then(|e| buildings.get(e).ok())
    else {
        if !text.0.is_empty() {
            text.0.clear();
        }
        if panel.display != Display::None {
            panel.display = Display::None;
        }
        return;
    };
    if panel.display != Display::Flex {
        panel.display = Display::Flex;
    }
    let mut lines = format!(
        "{:?} #{}\nyard {:.1} / {:.0} t",
        building.kind,
        building.id.0, // struct display: kind + stable id
        inventory.total(),
        inventory.capacity,
    );
    let mut any_band = false;
    for (i, kind) in ResourceKind::ALL.into_iter().enumerate() {
        let amount = inventory.amount(kind);
        let band = policies.and_then(|p| p.band(kind));
        let marker = if inventory.capacity > 0.0 && i == focus.0 {
            ">"
        } else {
            " "
        };
        match band {
            // Banded resource: bar against the band plus the bucket's role.
            Some(band) => {
                any_band = true;
                let role = if amount < band.min_pct * inventory.capacity - 1e-3 {
                    "DEMANDING"
                } else if amount > band.max_pct * inventory.capacity + 1e-3 {
                    "SUPPLYING"
                } else {
                    "in band"
                };
                lines.push_str(&format!(
                    "\n{marker} {kind:?} {} {amount:.1} t  {:.0}–{:.0}  {role}",
                    band_bar(amount, inventory.capacity, band.min_pct),
                    band.min_pct * inventory.capacity,
                    band.max_pct * inventory.capacity,
                ));
            }
            None if amount > 0.05 || (inventory.capacity > 0.0 && i == focus.0) => {
                lines.push_str(&format!("\n{marker} {kind:?}: {amount:.1} t  (no band)"));
            }
            None => {}
        }
    }
    if any_band || inventory.capacity > 0.0 {
        lines.push_str("\nB next resource   , . min -/+   Shift+, . max -/+");
    }
    if building.kind == BuildingKind::Depot {
        let mut parked = 0;
        let mut trucks = String::new();
        for (asset, on_road, job) in &fleet {
            if asset.home_depot != selected.0.unwrap() {
                continue;
            }
            if !on_road {
                parked += 1;
            }
            let state = match job.map(|j| j.phase) {
                None => "parked",
                Some(FreightPhase::ToPickup) => "→ pickup",
                Some(FreightPhase::Loading) => "loading",
                Some(FreightPhase::ToDropoff) => "→ dropoff",
                Some(FreightPhase::Unloading) => "unloading",
                Some(FreightPhase::ReturnToDepot) => "→ home",
            };
            trucks.push_str(&format!(
                "\n  truck #{} {:?}: {state}",
                asset.id.0, asset.cargo_class
            ));
        }
        lines.push_str(&format!(
            "\nslots {parked}/{DEPOT_SLOTS} parked{trucks}\nT buy bulk truck   Y buy covered truck"
        ));
    }
    if let Some(staffing) = staffing {
        lines.push_str(&format!(
            "\nstaff {} present / {} assigned of {}",
            staffing.present,
            staffing.assigned.len(),
            building.kind.workers_needed(),
        ));
    }
    if let Some(powered) = powered {
        lines.push_str(if powered.0 { "\nPOWERED" } else { "\nNO POWER" });
    }
    if let Some(output) = output {
        lines.push_str(&format!("\noutput {:.0} MW", output.0));
    }
    text.0 = lines;
}

fn draw_selection_ring(selected: Res<Selected>, buildings: Query<&Building>, mut gizmos: Gizmos) {
    let Some(building) = selected.0.and_then(|e| buildings.get(e).ok()) else {
        return;
    };
    gizmos.circle(
        Isometry3d::new(
            building.pos + Vec3::Y * 0.15,
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        building.kind.footprint().length() * 0.5 + 1.5,
        Color::srgb(0.95, 0.95, 0.95),
    );
}

/// One horizontal bar per yard and per truck: dark backing line, colored fill
/// proportional to inventory. Cheap, zoom-independent-enough legibility.
fn draw_fill_bars(
    buildings: Query<(&Building, &Inventory)>,
    trucks: Query<&ActiveVehicle>,
    mut gizmos: Gizmos,
) {
    let mut bar = |center: Vec3, width: f32, fill: f32, color: Color| {
        let half = Vec3::X * (width * 0.5);
        gizmos.line(center - half, center + half, Color::srgb(0.12, 0.12, 0.12));
        if fill > 0.01 {
            let tip = center - half + Vec3::X * (width * fill.min(1.0));
            gizmos.line(center - half + Vec3::Y * 0.15, tip + Vec3::Y * 0.15, color);
        }
    };
    for (building, inventory) in &buildings {
        let height = super::buildings::kind_height(building.kind) + 1.2;
        bar(
            building.pos + Vec3::Y * height,
            building.kind.footprint().x,
            inventory.total() / inventory.capacity,
            Color::srgb(0.30, 0.75, 0.35),
        );
    }
    for truck in &trucks {
        bar(
            truck.pos + Vec3::Y * 3.2,
            4.0,
            truck.cargo.total() / truck.cargo.capacity,
            Color::srgb(0.85, 0.65, 0.20),
        );
    }
}
