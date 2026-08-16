//! Legibility pass: HUD overlay (active tool, key legend, sim speed),
//! inspect-click panel, yard/cargo fill bars, and time controls. Presentation
//! only — reads sim state, writes nothing but `SimSpeed` (a player control,
//! not sim state).

use bevy::prelude::*;

use super::tools::{GroundCursor, ToolMode};
use crate::sim::SimSpeed;
use crate::sim::buildings::{Building, PowerOutput, Powered};
use crate::sim::resources::{Inventory, ResourceKind};
use crate::sim::vehicles::ActiveVehicle;

#[derive(Component)]
struct ToolReadout;

#[derive(Component)]
struct InspectReadout;

/// Building picked with the Inspect tool.
#[derive(Resource, Default)]
struct Selected(Option<Entity>);

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                (
                    drive_time_controls,
                    drive_inspect_tool,
                    update_tool_readout,
                    update_inspect_readout,
                    draw_selection_ring,
                    draw_fill_bars,
                ),
            );
    }
}

fn spawn_hud(mut commands: Commands) {
    let font = TextFont {
        font_size: bevy::text::FontSize::Px(15.0),
        ..default()
    };
    commands.spawn((
        ToolReadout,
        Text::new(""),
        font.clone(),
        TextColor(Color::srgb(0.92, 0.90, 0.82)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            top: Val::Px(10.0),
            ..default()
        },
        Name::new("HudTool"),
    ));
    commands.spawn((
        InspectReadout,
        Text::new(""),
        font,
        TextColor(Color::srgb(0.95, 0.93, 0.85)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(12.0),
            bottom: Val::Px(12.0),
            ..default()
        },
        Name::new("HudInspect"),
    ));
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

fn tool_label(mode: ToolMode) -> String {
    match mode {
        ToolMode::Inspect => "INSPECT - click a building".into(),
        ToolMode::Road(class) => {
            format!("ROAD ({class:?}) - click-chain, right-click ends, X cuts")
        }
        ToolMode::Building(kind) => format!("BUILD ({kind:?}) - click places, 3 cycles kind"),
        ToolMode::Wire => "WIRE - click-chain hops, right-click ends, X cuts".into(),
        ToolMode::Shuttle => "SHUTTLE - click source building, then destination".into(),
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
    mut readout: Query<&mut Text, With<ToolReadout>>,
) {
    if !mode.is_changed() && !speed.is_changed() {
        return;
    }
    let Ok(mut text) = readout.single_mut() else {
        return;
    };
    text.0 = format!(
        "{}   |   speed {}\n\
         1 dirt road   2 paved road   3 building   4 wire   5 shuttle   Esc inspect\n\
         WASD/arrows pan   Q/E rotate   wheel zoom   Space pause   [ ] speed",
        tool_label(*mode),
        speed_label(*speed),
    );
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

fn update_inspect_readout(
    selected: Res<Selected>,
    buildings: Query<(
        &Building,
        &Inventory,
        Option<&Powered>,
        Option<&PowerOutput>,
    )>,
    mut readout: Query<&mut Text, With<InspectReadout>>,
) {
    let Ok(mut text) = readout.single_mut() else {
        return;
    };
    let Some((building, inventory, powered, output)) =
        selected.0.and_then(|e| buildings.get(e).ok())
    else {
        if !text.0.is_empty() {
            text.0.clear();
        }
        return;
    };
    let mut lines = format!(
        "{:?} #{}\nyard {:.1} / {:.0} t",
        building.kind,
        building.id.0, // struct display: kind + stable id
        inventory.total(),
        inventory.capacity,
    );
    for kind in ResourceKind::ALL {
        let amount = inventory.amount(kind);
        if amount > 0.05 {
            lines.push_str(&format!("\n  {kind:?}: {amount:.1} t"));
        }
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
