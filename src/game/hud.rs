//! The state document: HUD panels, the inspect panel, the fullscreen Plan
//! ledger, fill bars and time controls. Presentation only — reads sim state,
//! writes nothing but `SimSpeed` (a player control, not sim state).
//!
//! R0.3 restructured this file around three rules, because the old HUD was a
//! debug readout that had grown four panels of undifferentiated text:
//!
//! 1. **Chrome comes from [`super::ui`]**, never from local colour literals.
//!    Every panel here, the ledger included, is built from the same
//!    vocabulary, so the interface reads as one document.
//! 2. **State and hints are separate surfaces.** The key legend was pinned to
//!    the top of the live-state panel, where it pushed the thing that changes
//!    below the thing that never does; it is its own panel now, on `F1`.
//! 3. **Every live line carries a [`Severity`]**, so a starving warehouse is
//!    loud and an idle-truck count is not.
//!
//! The Plan's quotas also moved onto the main HUD as drawn meters. They were
//! only visible behind `P`, which meant the game's central pressure was the
//! one thing the player could not see while playing.

use bevy::prelude::*;

use super::notify::{Notice, NoticeBoard};
use super::tools::{GroundCursor, ToolMode};
use super::ui::{
    Line, Meter, MeterLabel, Readout, SIZE_LEAD, Severity, Theme, header, panel, rule, spawn_meter,
    spawn_readout, write_readout,
};
use crate::sim::buildings::{Building, BuildingKind, PowerOutput, Powered};
use crate::sim::citizens::Citizen;
use crate::sim::dispatch::{DeficitBoard, DispatchQueue, FreightJob, FreightPhase};
use crate::sim::households::{Household, HousingQueue, RecruitmentPlan};
use crate::sim::labour::Staffing;
use crate::sim::resources::TransportClass;
use crate::sim::resources::{Inventory, ResourceKind};
use crate::sim::roads::{RoadBuildFeedback, RoadNode, RoadSegment};
use crate::sim::storage::StoragePolicies;
use crate::sim::traffic::StallBoard;
use crate::sim::vehicles::{
    ActivePawn, ActiveVehicle, DEPOT_SLOTS, VehicleAsset, VehicleEdit, VehicleEditQueue,
};
use crate::sim::{SimSpeed, TickIndex};

/// The key legend, split off from live state and toggled with `F1`.
#[derive(Component)]
struct HintPanel;

/// Panel markers. A readout is found through its panel rather than by a
/// unique component, so every panel shares one `Readout` implementation.
#[derive(Component)]
struct ToolPanel;

#[derive(Component)]
struct PopulationPanel;

#[derive(Component)]
struct DispatchPanel;

/// The Plan panel's quota rows, pre-spawned to a fixed pool.
#[derive(Component)]
struct QuotaRow(usize);

/// The same, inside the fullscreen ledger.
#[derive(Component)]
struct LedgerQuotaRow(usize);

/// The ledger's overall-fulfilment meter.
#[derive(Component)]
struct LedgerFulfilment;

/// The ledger's sheet node — the parent of its readout, so the ledger can
/// find its own text and nobody else's.
#[derive(Component)]
struct LedgerSheet;

/// Live text at the top of the Plan panel: period, deadline, treasury.
#[derive(Component)]
struct PlanReadout;

/// The most quotas any authored period declares; rows past the live count are
/// hidden rather than despawned.
const MAX_QUOTA_ROWS: usize = 4;

/// The inspect panel's chrome node; hidden while nothing is selected.
#[derive(Component)]
struct InspectPanel;

/// Fullscreen Plan ledger overlay (G1.4), toggled with `P`. Public so
/// capture scripts can reveal it for the record.
#[derive(Component)]
pub struct PlanLedgerPanel;

#[derive(Component)]
struct PlanLedgerReadout;

/// Building picked with the Inspect tool.
#[derive(Resource, Default)]
pub struct Selected(pub Option<Entity>);

/// Which resource the band-tuning keys act on for the selected storage.
#[derive(Resource, Default)]
struct BandFocus(usize);

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .init_resource::<BandFocus>()
            .add_systems(Startup, spawn_hud)
            // Grouped by what they do, not by when they were written — the
            // tuple has a 20-element ceiling and reads better split anyway.
            .add_systems(
                Update,
                (
                    drive_time_controls,
                    drive_recruitment_controls,
                    drive_inspect_tool,
                    drive_depot_purchase,
                    drive_office_purchase,
                    drive_bus_purchase,
                    drive_demolition,
                    drive_band_tuning,
                    toggle_plan_ledger,
                    toggle_hints,
                ),
            )
            .add_systems(
                Update,
                (
                    update_population_readout,
                    update_dispatch_readout,
                    update_tool_readout,
                    update_inspect_readout,
                    update_plan_panel,
                    update_plan_ledger,
                    notice_refusals,
                ),
            )
            .add_systems(
                Update,
                (
                    hint_starving_deficits,
                    hint_stalled_corridors,
                    draw_fill_bars,
                ),
            );
    }
}

/// The persistent key legend. Separated from live state (rule 2 above) and
/// written once at spawn — nothing here ever changes.
const KEY_HINTS: &[(&str, &str)] = &[
    ("1 / 2", "dirt / paved road"),
    ("3", "building (repeat cycles kind)"),
    ("4", "network (repeat cycles utility)"),
    ("5 / 6 / 7", "haul policy / bus line / zone"),
    ("Esc", "inspect"),
    ("WASD / Q E / wheel", "pan, rotate, zoom"),
    ("Space, [ ]", "pause, speed"),
    ("P / L / F1", "plan ledger / event log / these keys"),
];

fn spawn_hud(mut commands: Commands, theme: Res<Theme>) {
    // ---- left column: what the tool is doing, then how to drive it.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            Name::new("HudLeftColumn"),
        ))
        .with_children(|column| {
            column
                .spawn((panel(), ToolPanel, Name::new("HudToolPanel")))
                .with_children(|parent| {
                    parent.spawn(header(&theme, "state"));
                    spawn_readout(parent, &theme, 4);
                });
            column
                .spawn((panel(), HintPanel, Name::new("HudHintPanel")))
                .with_children(|parent| {
                    parent.spawn(header(&theme, "controls"));
                    for (keys, what) in KEY_HINTS {
                        parent
                            .spawn(Node {
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                            .with_children(|row| {
                                row.spawn((
                                    Node {
                                        // Wide enough for the longest chord
                                        // in KEY_HINTS; a narrower column
                                        // wraps it over the next row.
                                        min_width: Val::Px(150.0),
                                        flex_shrink: 0.0,
                                        ..default()
                                    },
                                    Text::new(*keys),
                                    theme.font(super::ui::SIZE_BODY, true),
                                    TextColor(Theme::INK),
                                ));
                                row.spawn((
                                    Text::new(*what),
                                    theme.font(super::ui::SIZE_BODY, false),
                                    TextColor(Theme::INK_DIM),
                                ));
                            });
                    }
                });
        });

    // ---- right column: the Plan first, because it is the pressure; then the
    // population it feeds and the fleet that moves it. Flex-stacked, so a
    // growing panel pushes the next down instead of overlapping it.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                width: Val::Px(330.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            Name::new("HudRightColumn"),
        ))
        .with_children(|column| {
            column
                .spawn((panel(), Name::new("HudPlanPanel")))
                .with_children(|parent| {
                    parent.spawn(header(&theme, "the state plan"));
                    parent
                        .spawn((
                            Text::new(""),
                            theme.font(super::ui::SIZE_BODY, false),
                            TextColor(Theme::INK),
                            PlanReadout,
                        ))
                        .insert(Node::default());
                    parent.spawn(rule());
                    for i in 0..MAX_QUOTA_ROWS {
                        spawn_meter(parent, &theme, "", 296.0, QuotaRow(i));
                    }
                });
            column
                .spawn((panel(), PopulationPanel, Name::new("HudPopulationPanel")))
                .with_children(|parent| {
                    parent.spawn(header(&theme, "population"));
                    spawn_readout(parent, &theme, 10);
                });
            column
                .spawn((panel(), DispatchPanel, Name::new("HudDispatchPanel")))
                .with_children(|parent| {
                    parent.spawn(header(&theme, "dispatch"));
                    spawn_readout(parent, &theme, 14);
                });
        });

    // ---- the Plan ledger (G1.4): the signature screen, on the same
    // vocabulary as everything else. R0.3 restyled rather than restructured
    // it — the layout was already right; what it lacked was drawn bars and a
    // hierarchy, both of which the vocabulary now supplies.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.025, 0.03, 0.86)),
            // Above every other panel. Without this the dim backdrop covered
            // the world but the HUD drew straight through the sheet — the
            // ledger's own title was occluded by the controls panel.
            GlobalZIndex(10),
            PlanLedgerPanel,
            Name::new("HudPlanLedger"),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(660.0),
                        padding: UiRect::axes(Val::Px(34.0), Val::Px(28.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(3.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(9.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.055, 0.060, 0.070, 0.98)),
                    BorderColor::all(Theme::ACCENT),
                    LedgerSheet,
                ))
                .with_children(|sheet| {
                    sheet.spawn((
                        Text::new(super::ui::letterspace("the state plan")),
                        theme.font(SIZE_LEAD, true),
                        TextColor(Theme::INK),
                    ));
                    sheet.spawn(rule());
                    sheet.spawn((
                        Text::new(""),
                        theme.font(super::ui::SIZE_BODY, false),
                        TextColor(Theme::INK),
                        PlanLedgerReadout,
                    ));
                    for i in 0..MAX_QUOTA_ROWS {
                        spawn_meter(sheet, &theme, "", 592.0, LedgerQuotaRow(i));
                    }
                    sheet.spawn(rule());
                    spawn_meter(sheet, &theme, "", 592.0, LedgerFulfilment);
                    sheet.spawn(rule());
                    spawn_readout(sheet, &theme, 5);
                    sheet.spawn((
                        Text::new("[P] close"),
                        theme.font(super::ui::SIZE_BODY, false),
                        TextColor(Theme::INK_DIM),
                    ));
                });
        });

    // ---- the inspect panel: bottom left, hidden until something is picked.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                bottom: Val::Px(56.0),
                max_width: Val::Px(420.0),
                display: Display::None,
                ..panel().0
            },
            panel().1,
            panel().2,
            InspectPanel,
            Name::new("HudInspectPanel"),
        ))
        .with_children(|parent| {
            parent.spawn(header(&theme, "inspect"));
            spawn_readout(parent, &theme, 22);
        });
}

/// `F1` hides the key legend once the player no longer needs it — the charter
/// asks for a HUD a stranger can learn from, not one they must live with.
fn toggle_hints(keys: Res<ButtonInput<KeyCode>>, mut panel: Query<&mut Node, With<HintPanel>>) {
    if !keys.just_pressed(KeyCode::F1) {
        return;
    }
    for mut node in &mut panel {
        node.display = match node.display {
            Display::None => Display::Flex,
            _ => Display::None,
        };
    }
}

fn toggle_plan_ledger(
    keys: Res<ButtonInput<KeyCode>>,
    mut panel: Query<&mut Node, With<PlanLedgerPanel>>,
) {
    if !keys.just_pressed(KeyCode::KeyP) {
        return;
    }
    for mut node in &mut panel {
        node.display = match node.display {
            Display::None => Display::Flex,
            _ => Display::None,
        };
    }
}

/// Push a value into one of a panel's pre-spawned meter rows: caption on the
/// label child, value and severity on the track child.
fn set_meter_row(
    children: &Children,
    labels: &mut Query<&mut Text, With<MeterLabel>>,
    meters: &mut Query<&mut Meter>,
    caption: &str,
    value: f32,
    severity: Severity,
) {
    for child in children.iter() {
        if let Ok(mut text) = labels.get_mut(child) {
            if text.0 != caption {
                text.0 = caption.to_string();
            }
        } else if let Ok(mut meter) = meters.get_mut(child)
            && (meter.value != value || meter.severity != severity)
        {
            // Guarded so change detection stays honest: an untouched meter
            // must not re-run `sync_meters` every frame.
            meter.value = value;
            meter.severity = severity;
        }
    }
}

/// A quota's caption and how hard it is pressing. Behind on a quota with the
/// deadline close is exactly the situation the game exists to create, so it
/// gets the attention severity — and critical once the period cannot be
/// recovered by ordinary means.
fn quota_line(quota: &crate::sim::plan::Quota, days_left: u32) -> (String, f32, Severity) {
    use crate::sim::plan::QuotaKind;
    let caption = match quota.kind {
        QuotaKind::Stockpile(kind, target) => {
            format!("{kind:?}  {:.0} / {target:.0} t", quota.progress * target,)
        }
        QuotaKind::Housed(target) => format!(
            "Households housed  {:.0} / {target}",
            quota.progress * target as f32,
        ),
    };
    // Pace: how far through the period we are versus how far through the
    // quota. `PERIOD_DAYS` is the authored length, so this is honest even in
    // the first period.
    let elapsed = crate::sim::plan::PLAN_PERIOD_DAYS.saturating_sub(days_left) as f32
        / crate::sim::plan::PLAN_PERIOD_DAYS as f32;
    let severity = if quota.progress >= 1.0 {
        Severity::Routine
    } else if quota.progress + 0.35 < elapsed {
        Severity::Critical
    } else if quota.progress + 0.15 < elapsed {
        Severity::Attention
    } else {
        Severity::Routine
    };
    (caption, quota.progress, severity)
}

/// The Plan panel on the main HUD (R0.3): the quotas the player is judged on,
/// visible while playing rather than only behind `P`.
fn update_plan_panel(
    plan: Option<Res<crate::sim::plan::StatePlan>>,
    treasury: Option<Res<crate::sim::plan::Treasury>>,
    frame: Res<crate::sim::clock::FrameIndex>,
    mut readout: Query<&mut Text, (With<PlanReadout>, Without<MeterLabel>)>,
    mut rows: Query<(&QuotaRow, &mut Node, &Children)>,
    mut labels: Query<&mut Text, With<MeterLabel>>,
    mut meters: Query<&mut Meter>,
) {
    let (Some(plan), Some(treasury)) = (plan, treasury) else {
        return;
    };
    let days_left = plan.days_left(frame.0);
    if let Ok(mut text) = readout.single_mut() {
        let next = format!(
            "Period {}   {days_left} days left   {:.0} rbl",
            plan.period, treasury.roubles,
        );
        if text.0 != next {
            text.0 = next;
        }
    }
    for (row, mut node, children) in &mut rows {
        match plan.quotas.get(row.0) {
            Some(quota) => {
                node.display = Display::Flex;
                let (caption, value, severity) = quota_line(quota, days_left);
                set_meter_row(
                    children,
                    &mut labels,
                    &mut meters,
                    &caption,
                    value,
                    severity,
                );
            }
            None => node.display = Display::None,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update_plan_ledger(
    plan: Option<Res<crate::sim::plan::StatePlan>>,
    treasury: Option<Res<crate::sim::plan::Treasury>>,
    frame: Res<crate::sim::clock::FrameIndex>,
    theme: Res<Theme>,
    panel: Query<&Node, With<PlanLedgerPanel>>,
    mut lead: Query<&mut Text, (With<PlanLedgerReadout>, Without<MeterLabel>)>,
    mut rows: Query<(&LedgerQuotaRow, &mut Node, &Children), Without<PlanLedgerPanel>>,
    fulfilment: Query<&Children, With<LedgerFulfilment>>,
    sheets: Query<&Children, With<LedgerSheet>>,
    readouts: Query<&Readout>,
    mut labels: Query<&mut Text, With<MeterLabel>>,
    mut meters: Query<&mut Meter>,
    mut spans: Query<(&mut TextSpan, &mut TextColor, &mut TextFont)>,
) {
    let (Some(plan), Some(treasury)) = (plan, treasury) else {
        return;
    };
    // Skip the whole rebuild while hidden.
    if !panel.iter().any(|n| n.display != Display::None) {
        return;
    }
    use crate::sim::plan::{BASE_TRANCHE, FULFILLMENT_BONUS};
    let days_left = plan.days_left(frame.0);
    let fulfillment = plan.fulfillment();

    if let Ok(mut text) = lead.single_mut() {
        let next = format!(
            "Period {}   \u{2014}   {days_left} days to the deadline",
            plan.period
        );
        if text.0 != next {
            text.0 = next;
        }
    }
    for (row, mut node, children) in &mut rows {
        match plan.quotas.get(row.0) {
            Some(quota) => {
                node.display = Display::Flex;
                let (caption, value, severity) = quota_line(quota, days_left);
                set_meter_row(
                    children,
                    &mut labels,
                    &mut meters,
                    &caption,
                    value,
                    severity,
                );
            }
            None => node.display = Display::None,
        }
    }
    if let Ok(children) = fulfilment.single() {
        let severity = if fulfillment >= 1.0 {
            Severity::Routine
        } else if fulfillment < 0.5 {
            Severity::Critical
        } else {
            Severity::Attention
        };
        set_meter_row(
            children,
            &mut labels,
            &mut meters,
            &format!("FULFILMENT  {:.0}%", fulfillment * 100.0),
            fulfillment,
            severity,
        );
    }

    let mut lines = vec![
        Line::routine(format!("Treasury  {:.0} roubles", treasury.roubles)),
        Line::dim(format!(
            "Next tranche (forecast)  {:.0} roubles",
            BASE_TRANCHE + FULFILLMENT_BONUS * fulfillment
        )),
    ];
    if let (Some(last), Some(tranche)) = (plan.last_fulfillment, plan.last_tranche) {
        lines.push(Line::dim(format!(
            "Last period: fulfilled {:.0}%, granted {tranche:.0} roubles",
            last * 100.0
        )));
    }
    // Only the ledger's own readout: an earlier version wrote to every
    // `Readout` in the world, so the treasury lines appeared in the tool
    // panel too.
    if let Some(readout) = sheets
        .iter()
        .flat_map(|c| c.iter())
        .find_map(|c| readouts.get(c).ok())
    {
        write_readout(readout, &theme, &lines, &mut spans);
    }
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

#[allow(clippy::too_many_arguments)]
fn update_population_readout(
    citizens: Query<&Citizen>,
    households: Query<&Household>,
    queue: Res<HousingQueue>,
    plan: Res<RecruitmentPlan>,
    dwellings: Query<&crate::sim::households::Dwelling>,
    staffed: Query<(&Building, &Staffing)>,
    zones: Query<&crate::sim::zoning::Zone>,
    buildings: Query<&Building>,
    zone_feedback: Res<crate::sim::zoning::ZoningFeedback>,
    // Optional: capture bins assemble sim plugins piecemeal — the HUD
    // drops these blocks rather than requiring every plugin.
    climate: Option<Res<crate::sim::heat::Climate>>,
    heated: Query<&crate::sim::heat::Heated>,
    theme: Res<Theme>,
    panel: Query<&Children, With<PopulationPanel>>,
    readouts: Query<&Readout>,
    mut spans: Query<(&mut TextSpan, &mut TextColor, &mut TextFont)>,
) {
    let Some(readout) = panel
        .iter()
        .flat_map(|c| c.iter())
        .find_map(|c| readouts.get(c).ok())
    else {
        return;
    };
    let total = households.iter().count();
    let housed = households.iter().filter(|h| h.dwelling.is_some()).count();
    let homeless = queue.0.len();
    let vacancies: u32 = dwellings.iter().map(|d| d.free_flats()).sum();
    let jobs_open: u32 = staffed.iter().map(|(b, s)| s.vacancies(b.kind)).sum();
    let unemployed = citizens
        .iter()
        .filter(|c| c.work.is_none() && c.home.is_some())
        .count();

    let mut lines = vec![
        Line::routine(format!(
            "{} citizens   {housed}/{total} households housed",
            citizens.iter().count(),
        )),
        Line::dim(format!(
            "recruitment target {}   (+/- adjusts)",
            plan.target_households
        )),
    ];
    // Homeless is the queue-as-demand signal: it is never routine, because
    // the housing office cannot clear it without the player building.
    lines.push(if homeless > 0 {
        Line::attention(format!("homeless {homeless}   flats free {vacancies}"))
    } else {
        Line::dim(format!("homeless 0   flats free {vacancies}"))
    });
    lines.push(Line::dim(format!(
        "jobs open {jobs_open}   unemployed {unemployed}"
    )));

    let (zone_count, backlog) = crate::sim::zoning::unfulfilled_zones(&zones, &buildings);
    if zone_count > 0 {
        lines.push(Line::dim(format!(
            "districts {zone_count}, {backlog} not yet fulfilled"
        )));
    }
    if let Some((kind, zone_kind)) = zone_feedback.0 {
        lines.push(Line::critical(format!(
            "REFUSED: {kind:?} in a {zone_kind:?} district"
        )));
    }
    if let Some(climate) = climate {
        let cold_homes = heated.iter().filter(|h| !h.0).count();
        lines.push(Line::dim(format!(
            "season {:+.0}\u{b0}C",
            climate.temperature
        )));
        // A cold home in winter is the failure the heat network exists to
        // prevent, and it is invisible from the outside — so it is loud here.
        if cold_homes > 0 {
            lines.push(Line::critical(format!("{cold_homes} homes without heat")));
        }
    }
    write_readout(readout, &theme, &lines, &mut spans);
}

/// The treasury refusal is an event, not a state: it fires once, so it wants
/// a toast rather than a line that lingers for a game day.
fn notice_refusals(
    alloc_feedback: Option<Res<crate::sim::plan::AllocationFeedback>>,
    road_feedback: Res<RoadBuildFeedback>,
    zone_feedback: Res<crate::sim::zoning::ZoningFeedback>,
    mut notices: ResMut<NoticeBoard>,
    mut last_alloc: Local<Option<u32>>,
) {
    if zone_feedback.is_changed()
        && let Some((kind, zone_kind)) = zone_feedback.0
    {
        notices.push(Notice::critical(format!(
            "Refused: a {kind:?} may not stand in a {zone_kind:?} district"
        )));
    }
    if let Some(feedback) = alloc_feedback
        && let Some((when, what)) = feedback.0
        && *last_alloc != Some(when)
    {
        *last_alloc = Some(when);
        notices.push(Notice::critical(format!("No funds: {what} refused")));
    }
    if road_feedback.is_changed()
        && let Some(shortfall) = road_feedback.0
    {
        notices.push(Notice::critical(format!(
            "Not enough gravel delivered: paving needs {:.1} t, {:.1} t in nearby yards",
            shortfall.needed,
            shortfall.available.max(0.0),
        )));
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

/// `Delete` demolishes the selected building (B6.5) — physically gone,
/// everything referencing it self-heals through the retention passes.
fn drive_demolition(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<Selected>,
    buildings: Query<&Building>,
    mut edits: ResMut<crate::sim::buildings::BuildingEditQueue>,
) {
    if !keys.just_pressed(KeyCode::Delete) {
        return;
    }
    let Some(building) = selected.0.filter(|&e| buildings.get(e).is_ok()) else {
        return;
    };
    edits
        .0
        .push(crate::sim::buildings::BuildingEdit::Demolish { building });
    selected.0 = None;
    info!("demolition queued: {building:?}");
}

/// With a construction office selected: `T` buys an excavator, `Y` a crane.
fn drive_office_purchase(
    keys: Res<ButtonInput<KeyCode>>,
    selected: Res<Selected>,
    buildings: Query<&Building>,
    mut edits: ResMut<VehicleEditQueue>,
) {
    use crate::sim::vehicles::VehicleKind;
    let Some(office) = selected.0.filter(|&e| {
        buildings
            .get(e)
            .is_ok_and(|b| b.kind == BuildingKind::ConstructionOffice)
    }) else {
        return;
    };
    let kind = if keys.just_pressed(KeyCode::KeyT) {
        VehicleKind::Excavator
    } else if keys.just_pressed(KeyCode::KeyY) {
        VehicleKind::Crane
    } else {
        return;
    };
    edits.0.push(VehicleEdit::BuyMachine { office, kind });
    info!("office purchase queued: {kind:?} at {office:?}");
}

/// A depot selected? `U` buys a bus (transit fleet shares the depot).
fn drive_bus_purchase(
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
    if keys.just_pressed(KeyCode::KeyU) {
        edits.0.push(VehicleEdit::BuyBus { depot });
        info!("depot purchase queued: bus at {depot:?}");
    }
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
    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
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

/// A registered corridor stall is never invisible (spec traffic.md G4: jams
/// are information, not garbage): the jammed segment glows pulsing red until
/// its traffic moves again.
fn hint_stalled_corridors(
    time: Res<Time>,
    board: Res<StallBoard>,
    segments: Query<&RoadSegment>,
    nodes: Query<&RoadNode>,
    mut gizmos: Gizmos,
) {
    let pulse = 0.5 + 0.5 * (time.elapsed_secs() * 4.0).sin();
    let color = Color::srgb(0.95, 0.2, 0.1).with_alpha(0.35 + 0.5 * pulse);
    for &seg in board.0.keys() {
        let Ok(segment) = segments.get(seg) else {
            continue;
        };
        let lift = Vec3::Y * 0.6;
        // Trace the compiled centreline so the glow hugs a curved ribbon;
        // chord fallback for a segment that has not compiled yet.
        let chord;
        let points: &[Vec3] = if segment.curve.len() >= 2 {
            &segment.curve
        } else {
            let (Ok(a), Ok(b)) = (nodes.get(segment.a), nodes.get(segment.b)) else {
                continue;
            };
            chord = [a.pos, b.pos];
            &chord
        };
        for w in points.windows(2) {
            let side =
                (w[1] - w[0]).normalize_or_zero().cross(Vec3::Y) * (segment.class.width() * 0.5);
            gizmos.line(w[0] + lift, w[1] + lift, color);
            gizmos.line(w[0] + side + lift, w[1] + side + lift, color);
            gizmos.line(w[0] - side + lift, w[1] - side + lift, color);
        }
    }
}

/// Ticks → game hours (600 frames per game day).
fn game_hours(ticks: u32) -> f32 {
    ticks as f32 * 24.0 / crate::sim::clock::FRAMES_PER_GAME_DAY as f32
}

#[allow(clippy::too_many_arguments)]
fn update_dispatch_readout(
    queue: Res<DispatchQueue>,
    board: Res<DeficitBoard>,
    stalls: Res<StallBoard>,
    sites_q: Query<&crate::sim::construction::ConstructionSite>,
    transit_lines: Query<&crate::sim::transit::TransitLine>,
    duties: Query<&crate::sim::transit::BusDuty>,
    stop_queues: Res<crate::sim::transit::StopQueues>,
    tick: Res<TickIndex>,
    fleet: Query<(&VehicleAsset, Has<ActivePawn>)>,
    buildings: Query<&Building>,
    theme: Res<Theme>,
    mut notices: ResMut<NoticeBoard>,
    panel: Query<&Children, With<DispatchPanel>>,
    readouts: Query<&Readout>,
    mut spans: Query<(&mut TextSpan, &mut TextColor, &mut TextFont)>,
    mut notified: Local<std::collections::HashSet<Entity>>,
) {
    // Buildings that have recovered can alarm again.
    notified.retain(|e| board.0.iter().any(|d| d.building == *e));
    let Some(readout) = panel
        .iter()
        .flat_map(|c| c.iter())
        .find_map(|c| readouts.get(c).ok())
    else {
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
    let mut lines = vec![
        Line::routine(format!("fleet {busy} busy / {idle} idle")),
        // Orders waiting with no truck to take them is the fleet-shortage
        // signal — routine while it is zero, attention the moment it is not.
        if waiting > 0 {
            Line::attention(format!(
                "{} orders in flight, {waiting} waiting",
                queue.orders.len() - waiting,
            ))
        } else {
            Line::dim(format!("{} orders in flight", queue.orders.len()))
        },
    ];
    for (depot, (out, parked)) in &per_depot {
        if let Ok(building) = buildings.get(*depot) {
            lines.push(Line::dim(format!(
                "depot #{}: {parked} parked, {out} out",
                building.id.0
            )));
        }
    }
    // The queue is the planning signal: the oldest waiting orders read first.
    let mut pending: Vec<_> = queue
        .orders
        .iter()
        .filter(|o| o.assigned.is_none())
        .collect();
    pending.sort_by_key(|o| o.issued_tick);
    for order in pending.iter().take(3) {
        lines.push(Line::dim(format!(
            "{:?} {:.0} t \u{2014} waiting {:.1} h",
            order.resource,
            order.qty,
            game_hours(tick.0.saturating_sub(order.issued_tick)),
        )));
    }
    if pending.len() > 3 {
        lines.push(Line::dim(format!(
            "\u{2026} and {} more",
            pending.len() - 3
        )));
    }
    let site_count = sites_q.iter().count();
    if site_count > 0 {
        let (mut no_material, mut no_machine) = (0, 0);
        for site in &sites_q {
            match site.bottleneck {
                Some(crate::sim::construction::Bottleneck::NoMaterial) => no_material += 1,
                Some(crate::sim::construction::Bottleneck::NoMachine) => no_machine += 1,
                None => {}
            }
        }
        // A site that is merely building is progress; a site that is *blocked*
        // is the plan stalling, and the player is the only one who can unblock
        // it.
        let blocked = no_material + no_machine;
        lines.push(if blocked > 0 {
            Line::critical(format!(
                "{blocked} of {site_count} sites blocked \u{2014} {no_material} want material, {no_machine} want machines"
            ))
        } else {
            Line::dim(format!("{site_count} sites building"))
        });
    }
    let line_count = transit_lines.iter().count();
    if line_count > 0 {
        let buses = duties.iter().count();
        let aboard: usize = duties.iter().map(|d| d.riders.len()).sum();
        let queued: usize = stop_queues.0.values().map(|q| q.len()).sum();
        lines.push(Line::dim(format!(
            "transit: {line_count} line(s), {buses} out, {aboard} aboard, {queued} waiting"
        )));
    }
    if !stalls.0.is_empty() {
        let held: u32 = stalls.0.values().map(|s| s.vehicles).sum();
        let oldest = stalls
            .0
            .values()
            .map(|s| s.since_tick)
            .min()
            .unwrap_or(tick.0);
        lines.push(Line::attention(format!(
            "jam: {held} trucks held on {} segment(s), oldest {:.1} h",
            stalls.0.len(),
            game_hours(tick.0.saturating_sub(oldest)),
        )));
    }
    // The defect R0.4 exists to fix: this line used to render in the same
    // size and colour as the idle-fleet count above it.
    if let Some(starving) = board.0.iter().min_by_key(|d| d.since_tick) {
        let name = buildings
            .get(starving.building)
            .map(|b| format!("{:?} #{}", b.kind, b.id.0))
            .unwrap_or_else(|_| "?".into());
        let hours = game_hours(tick.0.saturating_sub(starving.since_tick));
        lines.push(Line::critical(format!(
            "STARVING: {name} short {:.1} t {:?} for {hours:.1} h",
            starving.deficit, starving.resource,
        )));
        // One toast when it *crosses* an hour of starvation. Pushing every
        // frame let the fold counter run to (×55) in the first capture — the
        // fold is a safety net, not a substitute for firing once.
        if hours >= 1.0 && !notified.contains(&starving.building) {
            notified.insert(starving.building);
            notices.push(Notice::critical(format!(
                "{name} is starving \u{2014} short {:.1} t {:?}",
                starving.deficit, starving.resource,
            )));
        }
    }
    write_readout(readout, &theme, &lines, &mut spans);
}

/// The tool's name and, separately, how to drive it — hierarchy needs the two
/// apart, since the name is what changes and the instructions are not news.
fn tool_label(mode: ToolMode) -> (String, String) {
    match mode {
        ToolMode::Inspect => ("INSPECT".into(), "click a building".into()),
        ToolMode::Road(class) => (
            format!("ROAD \u{2014} {class:?}"),
            "click-chain, right-click ends, X cuts, R rebuilds last cut".into(),
        ),
        ToolMode::Building(kind) => (
            format!("BUILD \u{2014} {kind:?}"),
            "click places, 3 cycles kind".into(),
        ),
        ToolMode::Wire => (
            "NETWORK".into(),
            "click-chain hops, right-click ends, X cuts, 4 cycles utility".into(),
        ),
        ToolMode::Shuttle => (
            "HAUL POLICY".into(),
            "click source (export all), then destination (import to 90%)".into(),
        ),
        ToolMode::TransitLine => (
            "BUS LINE".into(),
            "click stops in order, right-click closes the loop".into(),
        ),
        ToolMode::Zone(kind) => (
            format!("ZONE \u{2014} {kind:?}"),
            "two clicks span a district, 7 cycles use, X erases".into(),
        ),
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
    theme: Res<Theme>,
    panel: Query<&Children, With<ToolPanel>>,
    readouts: Query<&Readout>,
    mut spans: Query<(&mut TextSpan, &mut TextColor, &mut TextFont)>,
) {
    if !mode.is_changed() && !speed.is_changed() && !feedback.is_changed() {
        return;
    }
    let Some(readout) = panel
        .iter()
        .flat_map(|c| c.iter())
        .find_map(|c| readouts.get(c).ok())
    else {
        return;
    };
    let (tool, hint) = tool_label(*mode);
    let mut lines = vec![
        Line::routine(tool),
        Line::dim(hint),
        Line::dim(format!("speed {}", speed_label(*speed))),
    ];
    // A refusal has to be felt, not read: the toast fires from
    // `notice_refusals`, the placement shakes in `game::juice`, and this line
    // is the one that stays put so the player can re-read the number.
    if let Some(shortfall) = feedback.0 {
        lines.push(Line::critical(format!(
            "REFUSED: paving needs {:.1} t gravel, {:.1} t in nearby yards",
            shortfall.needed,
            shortfall.available.max(0.0),
        )));
    }
    write_readout(readout, &theme, &lines, &mut spans);
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
#[allow(clippy::too_many_arguments)]
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
    sites: Query<&crate::sim::construction::ConstructionSite>,
    theme: Res<Theme>,
    mut panel: Query<(&mut Node, &Children), With<InspectPanel>>,
    readouts: Query<&Readout>,
    mut spans: Query<(&mut TextSpan, &mut TextColor, &mut TextFont)>,
) {
    let Ok((mut panel, children)) = panel.single_mut() else {
        return;
    };
    let Some(readout) = children.iter().find_map(|c| readouts.get(c).ok()) else {
        return;
    };
    let Some((building, inventory, powered, output, staffing, policies)) =
        selected.0.and_then(|e| buildings.get(e).ok())
    else {
        if panel.display != Display::None {
            panel.display = Display::None;
            write_readout(readout, &theme, &[], &mut spans);
        }
        return;
    };
    if panel.display != Display::Flex {
        panel.display = Display::Flex;
    }

    let mut lines = vec![
        Line::routine(format!("{:?} #{}", building.kind, building.id.0)),
        Line::dim(format!(
            "yard {:.1} / {:.0} t",
            inventory.total(),
            inventory.capacity
        )),
    ];
    // A site names its phase, its bill, and — the whole point — its stall.
    if let Some(site) = selected.0.and_then(|e| sites.get(e).ok()) {
        if let Some(phase) = site.phase() {
            let mut text = format!(
                "under construction {}/{} {:?}: work {:.0}%",
                site.current + 1,
                site.phases.len(),
                phase.kind,
                100.0 * phase.done / phase.work.max(1e-3),
            );
            if let Some((resource, need)) = phase.material {
                text.push_str(&format!(
                    "   {resource:?} {:.1}/{need:.1} t",
                    phase.consumed
                ));
            }
            lines.push(Line::routine(text));
        }
        match site.bottleneck {
            Some(crate::sim::construction::Bottleneck::NoMaterial) => {
                lines.push(Line::critical("STALLED: no material"))
            }
            Some(crate::sim::construction::Bottleneck::NoMachine) => {
                lines.push(Line::critical("STALLED: no machine"))
            }
            None => {}
        }
    }
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
                let below = amount < band.min_pct * inventory.capacity - 1e-3;
                let role = if below {
                    "DEMANDING"
                } else if amount > band.max_pct * inventory.capacity + 1e-3 {
                    "supplying"
                } else {
                    "in band"
                };
                let text = format!(
                    "{marker} {kind:?} {} {amount:.1} t  {:.0}\u{2013}{:.0}  {role}",
                    band_bar(amount, inventory.capacity, band.min_pct),
                    band.min_pct * inventory.capacity,
                    band.max_pct * inventory.capacity,
                );
                // Below the min line is a bucket actively pulling on the
                // dispatcher — attention, not noise.
                lines.push(if below {
                    Line::attention(text)
                } else {
                    Line::routine(text)
                });
            }
            None if amount > 0.05 || (inventory.capacity > 0.0 && i == focus.0) => {
                lines.push(Line::dim(format!(
                    "{marker} {kind:?}: {amount:.1} t  (no band)"
                )));
            }
            None => {}
        }
    }
    if any_band || inventory.capacity > 0.0 {
        lines.push(Line::dim(
            "B next resource   , . min -/+   Shift+, . max -/+",
        ));
    }
    if building.kind == BuildingKind::Depot {
        let mut parked = 0;
        let mut trucks = Vec::new();
        for (asset, on_road, job) in &fleet {
            if Some(asset.home_depot) != selected.0 {
                continue;
            }
            if !on_road {
                parked += 1;
            }
            let state = match job.map(|j| j.phase) {
                None => "parked",
                Some(FreightPhase::ToPickup) => "\u{2192} pickup",
                Some(FreightPhase::Loading) => "loading",
                Some(FreightPhase::ToDropoff) => "\u{2192} dropoff",
                Some(FreightPhase::Unloading) => "unloading",
                Some(FreightPhase::ReturnToDepot) => "\u{2192} home",
            };
            trucks.push(Line::dim(format!(
                "  truck #{} {:?}: {state}",
                asset.id.0, asset.cargo_class
            )));
        }
        lines.push(Line::routine(format!(
            "slots {parked}/{DEPOT_SLOTS} parked"
        )));
        lines.extend(trucks);
        lines.push(Line::dim("T buy bulk truck   Y buy covered truck"));
    }
    if let Some(staffing) = staffing {
        let needed = building.kind.workers_needed();
        let text = format!(
            "staff {} present / {} assigned of {needed}",
            staffing.present,
            staffing.assigned.len(),
        );
        // A works with nobody in it produces nothing, and looks identical
        // from the outside to one that is running.
        lines.push(if needed > 0 && staffing.present == 0 {
            Line::critical(text)
        } else {
            Line::dim(text)
        });
    }
    if let Some(powered) = powered {
        lines.push(if powered.0 {
            Line::dim("powered")
        } else {
            Line::critical("NO POWER")
        });
    }
    if let Some(output) = output {
        lines.push(Line::dim(format!("output {:.0} MW", output.0)));
    }
    write_readout(readout, &theme, &lines, &mut spans);
}

/// One horizontal bar per yard and per truck: dark backing line, colored fill
/// proportional to inventory. Cheap, zoom-independent-enough legibility.
/// Yard and cargo fill, drawn in the world above each building and truck.
///
/// These read as stray coloured lines in the R0 capture — a bright green that
/// was in no palette, a near-black track that looked like a rendering fault.
/// They are now on the palette and, more importantly, they *say* something:
/// an empty yard is the same alarm colour as the starving line that names it.
fn draw_fill_bars(
    buildings: Query<(&Building, &Inventory)>,
    trucks: Query<&ActiveVehicle>,
    mut gizmos: Gizmos,
) {
    // Empty is critical, comfortable is quiet, full is the signal-ok gold the
    // power lamps already use — the same three-step reading as `Severity`.
    let fill_color = |fill: f32| {
        if fill < 0.08 {
            Severity::Critical.color()
        } else if fill < 0.30 {
            Severity::Attention.color()
        } else if fill > 0.97 {
            super::palette::Role::SignalOk.color()
        } else {
            Color::srgb(0.55, 0.62, 0.45)
        }
    };
    let mut bar = |center: Vec3, width: f32, fill: f32, color: Color| {
        let half = Vec3::X * (width * 0.5);
        // The track is the panel's sunk colour, so a bar reads as the same
        // object as the meters in the HUD rather than as a black wire.
        gizmos.line(center - half, center + half, Theme::PANEL_SUNK);
        if fill > 0.01 {
            let tip = center - half + Vec3::X * (width * fill.min(1.0));
            gizmos.line(center - half + Vec3::Y * 0.22, tip + Vec3::Y * 0.22, color);
        }
    };
    for (building, inventory) in &buildings {
        if inventory.capacity <= 0.0 {
            continue;
        }
        let fill = inventory.total() / inventory.capacity;
        let height = super::buildings::kind_height(building.kind) + 2.4;
        bar(
            building.pos + Vec3::Y * height,
            building.kind.footprint().x * 0.8,
            fill,
            fill_color(fill),
        );
    }
    for truck in &trucks {
        let fill = truck.cargo.total() / truck.cargo.capacity;
        bar(truck.pos + Vec3::Y * 3.2, 4.0, fill, fill_color(fill));
    }
}
