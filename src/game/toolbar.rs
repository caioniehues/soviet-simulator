//! Bottom toolbar (G1.3, #77): the mouse-driven face of the tool state
//! machine. Category buttons open a flyout of concrete tools; clicking one
//! writes the same `ToolMode` (and `ActiveNetKind`) the number keys write,
//! so keyboard and mouse stay two doors into one state — the highlight
//! follows `ToolMode`, never the click. Toolbar hover parks the ground
//! cursor so a UI click can never also fire the world tool underneath.

use bevy::prelude::*;

use super::tools::ToolMode;
use super::wires::ActiveNetKind;
use crate::sim::buildings::BuildingKind;
use crate::sim::roads::RoadClass;
use crate::sim::wires::NetKind;
use crate::sim::zoning::ZoneKind;

/// True while the pointer is over any toolbar control; the ground cursor
/// (and with it every world tool) is parked while set.
#[derive(Resource, Default)]
pub struct UiHover(pub bool);

/// Which category's flyout is open, by index into `CATEGORIES`.
#[derive(Resource, Default)]
struct OpenCategory(Option<usize>);

#[derive(Clone, Copy, PartialEq, Debug)]
enum ToolAction {
    Road(RoadClass),
    Building(BuildingKind),
    Wire(NetKind),
    Shuttle,
    TransitLine,
    Zone(ZoneKind),
}

struct Category {
    label: &'static str,
    tools: &'static [(&'static str, ToolAction)],
}

const CATEGORIES: &[Category] = &[
    Category {
        label: "INSPECT",
        tools: &[],
    },
    Category {
        label: "ROADS [1/2]",
        tools: &[
            ("DIRT ROAD", ToolAction::Road(RoadClass::Dirt)),
            ("PAVED ROAD", ToolAction::Road(RoadClass::Paved)),
        ],
    },
    Category {
        label: "BUILD [3]",
        tools: &[
            ("MINE", ToolAction::Building(BuildingKind::Mine)),
            ("QUARRY", ToolAction::Building(BuildingKind::Quarry)),
            ("POWER PLANT", ToolAction::Building(BuildingKind::PowerPlant)),
            ("FACTORY", ToolAction::Building(BuildingKind::Factory)),
            ("DWELLING", ToolAction::Building(BuildingKind::Dwelling)),
            ("WAREHOUSE", ToolAction::Building(BuildingKind::Warehouse)),
            ("DEPOT", ToolAction::Building(BuildingKind::Depot)),
            ("BUS STOP", ToolAction::Building(BuildingKind::BusStop)),
            (
                "CONSTR. OFFICE",
                ToolAction::Building(BuildingKind::ConstructionOffice),
            ),
            ("WATER PUMP", ToolAction::Building(BuildingKind::WaterPump)),
            ("SEWAGE WORKS", ToolAction::Building(BuildingKind::SewagePlant)),
            ("HEAT PLANT", ToolAction::Building(BuildingKind::HeatPlant)),
            ("CUSTOMS", ToolAction::Building(BuildingKind::CustomsOffice)),
        ],
    },
    Category {
        label: "NETWORKS [4]",
        tools: &[
            ("POWER LINE", ToolAction::Wire(NetKind::Power)),
            ("WATER MAIN", ToolAction::Wire(NetKind::Water)),
            ("HEAT MAIN", ToolAction::Wire(NetKind::Heat)),
        ],
    },
    Category {
        label: "HAUL [5]",
        tools: &[("TRUCK SHUTTLE", ToolAction::Shuttle)],
    },
    Category {
        label: "TRANSIT [6]",
        tools: &[("BUS LINE", ToolAction::TransitLine)],
    },
    Category {
        label: "ZONES [7]",
        tools: &[
            ("RESIDENTIAL", ToolAction::Zone(ZoneKind::Residential)),
            ("INDUSTRIAL", ToolAction::Zone(ZoneKind::Industrial)),
        ],
    },
];

#[derive(Component)]
struct CategoryButton(usize);

#[derive(Component)]
struct FlyoutRoot(usize);

#[derive(Component)]
struct ToolButton(ToolAction);

const BUTTON_BG: Color = Color::srgba(0.10, 0.11, 0.12, 0.95);
const BUTTON_HOVER: Color = Color::srgba(0.18, 0.17, 0.15, 0.98);
const BUTTON_ACTIVE: Color = Color::srgba(0.63, 0.35, 0.20, 0.95);
const TEXT: Color = Color::srgb(0.92, 0.90, 0.82);

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiHover>()
            .init_resource::<OpenCategory>()
            .add_systems(Startup, spawn_toolbar)
            .add_systems(
                Update,
                (
                    track_ui_hover,
                    handle_category_clicks,
                    handle_tool_clicks,
                    sync_flyouts,
                    paint_buttons,
                )
                    .chain(),
            );
    }
}

fn button_bundle(font: &TextFont, label: &str) -> impl Bundle {
    (
        Button,
        Node {
            padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BUTTON_BG),
        children![(
            Text::new(label),
            font.clone(),
            TextColor(TEXT),
            Pickable::IGNORE,
        )],
    )
}

fn spawn_toolbar(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
        font_size: bevy::text::FontSize::Px(13.0),
        ..default()
    };
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                column_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            Name::new("Toolbar"),
        ))
        .with_children(|bar| {
            for (index, category) in CATEGORIES.iter().enumerate() {
                bar.spawn((
                    Node {
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Stretch,
                        row_gap: Val::Px(2.0),
                        ..default()
                    },
                    Name::new(category.label),
                ))
                .with_children(|cell| {
                    cell.spawn((CategoryButton(index), button_bundle(&font, category.label)));
                    cell.spawn((
                        FlyoutRoot(index),
                        Node {
                            flex_direction: FlexDirection::ColumnReverse,
                            row_gap: Val::Px(1.0),
                            display: Display::None,
                            ..default()
                        },
                    ))
                    .with_children(|flyout| {
                        for (label, action) in category.tools {
                            flyout.spawn((ToolButton(*action), button_bundle(&font, label)));
                        }
                    });
                });
            }
        });
}

/// While the pointer touches any toolbar button, the world tools are parked
/// (`GroundCursor` reads this via `UiHover`).
fn track_ui_hover(
    interactions: Query<&Interaction, With<Button>>,
    mut hover: ResMut<UiHover>,
) {
    let over = interactions
        .iter()
        .any(|i| !matches!(i, Interaction::None));
    if hover.0 != over {
        hover.0 = over;
    }
}

fn handle_category_clicks(
    keys: Res<ButtonInput<KeyCode>>,
    interactions: Query<(&Interaction, &CategoryButton), Changed<Interaction>>,
    mut open: ResMut<OpenCategory>,
    mut mode: ResMut<ToolMode>,
) {
    if keys.just_pressed(KeyCode::Escape) && open.0.is_some() {
        open.0 = None;
    }
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if CATEGORIES[button.0].tools.is_empty() {
            // INSPECT acts directly instead of opening an empty flyout.
            *mode = ToolMode::Inspect;
            open.0 = None;
        } else {
            open.0 = if open.0 == Some(button.0) {
                None
            } else {
                Some(button.0)
            };
        }
    }
}

fn handle_tool_clicks(
    interactions: Query<(&Interaction, &ToolButton), Changed<Interaction>>,
    mut open: ResMut<OpenCategory>,
    mut mode: ResMut<ToolMode>,
    mut net_kind: ResMut<ActiveNetKind>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        *mode = match button.0 {
            ToolAction::Road(class) => ToolMode::Road(class),
            ToolAction::Building(kind) => ToolMode::Building(kind),
            ToolAction::Wire(kind) => {
                net_kind.0 = kind;
                ToolMode::Wire
            }
            ToolAction::Shuttle => ToolMode::Shuttle,
            ToolAction::TransitLine => ToolMode::TransitLine,
            ToolAction::Zone(kind) => ToolMode::Zone(kind),
        };
        open.0 = None;
    }
}

fn sync_flyouts(open: Res<OpenCategory>, mut flyouts: Query<(&FlyoutRoot, &mut Node)>) {
    if !open.is_changed() {
        return;
    }
    for (flyout, mut node) in &mut flyouts {
        let want = if open.0 == Some(flyout.0) {
            Display::Flex
        } else {
            Display::None
        };
        if node.display != want {
            node.display = want;
        }
    }
}

/// Does this action match the live tool state? (The highlight follows the
/// state machine, so key-driven switches light the toolbar too.)
fn action_is_active(action: ToolAction, mode: ToolMode, net: NetKind) -> bool {
    match (action, mode) {
        (ToolAction::Road(a), ToolMode::Road(b)) => a == b,
        (ToolAction::Building(a), ToolMode::Building(b)) => a == b,
        (ToolAction::Wire(a), ToolMode::Wire) => a == net,
        (ToolAction::Shuttle, ToolMode::Shuttle) => true,
        (ToolAction::TransitLine, ToolMode::TransitLine) => true,
        (ToolAction::Zone(a), ToolMode::Zone(b)) => a == b,
        _ => false,
    }
}

fn category_is_active(index: usize, mode: ToolMode, net: NetKind) -> bool {
    let category = &CATEGORIES[index];
    if category.tools.is_empty() {
        return mode == ToolMode::Inspect;
    }
    category
        .tools
        .iter()
        .any(|(_, action)| action_is_active(*action, mode, net))
}

fn paint_buttons(
    mode: Res<ToolMode>,
    net_kind: Res<ActiveNetKind>,
    mut categories: Query<
        (&Interaction, &CategoryButton, &mut BackgroundColor),
        Without<ToolButton>,
    >,
    mut tools: Query<(&Interaction, &ToolButton, &mut BackgroundColor), Without<CategoryButton>>,
) {
    let paint = |interaction: &Interaction, active: bool, bg: &mut BackgroundColor| {
        let want = if active {
            BUTTON_ACTIVE
        } else if !matches!(interaction, Interaction::None) {
            BUTTON_HOVER
        } else {
            BUTTON_BG
        };
        if bg.0 != want {
            bg.0 = want;
        }
    };
    for (interaction, button, mut bg) in &mut categories {
        paint(
            interaction,
            category_is_active(button.0, *mode, net_kind.0),
            &mut bg,
        );
    }
    for (interaction, button, mut bg) in &mut tools {
        paint(
            interaction,
            action_is_active(button.0, *mode, net_kind.0),
            &mut bg,
        );
    }
}
