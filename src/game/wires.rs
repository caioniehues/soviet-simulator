//! Wire tool + rendering (presentation side). Hops go through the sim's edit
//! queue; poles are primitive cylinders, spans are gizmo lines, and factories
//! show a power lamp — all reads, no sim writes (ADR 0003).

use bevy::prelude::*;

use super::buildings::kind_height;
use super::palette::{Mat, Role};
use super::tools::{GroundCursor, ToolMode};
use crate::sim::buildings::{Building, Powered};
use crate::sim::wires::{NetKind, WireEdit, WireEditQueue, WirePole, WireSpan};

const POLE_HEIGHT: f32 = 7.0;

/// Last laid point while a wire click chain is open.
#[derive(Resource, Default)]
struct WireChainStart(Option<Vec3>);

/// Which utility the wire tool lays; key 4 cycles it (B8.2).
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub struct ActiveNetKind(pub NetKind);

pub fn net_color(kind: NetKind) -> Color {
    match kind {
        // Power cable is the one that reads as a real material — bare wire
        // against the sky; water and heat are schematic overlay colours.
        NetKind::Power => Role::Coal.color(),
        NetKind::Water => Role::WaterLine.color(),
        NetKind::Heat => Role::HeatLine.color(),
    }
}

pub struct WireToolPlugin;

impl Plugin for WireToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WireChainStart>()
            .init_resource::<ActiveNetKind>()
            .add_systems(
                Update,
                (
                    drive_wire_tool,
                    preview_wire_chain,
                    sync_pole_meshes,
                    draw_spans,
                    draw_power_lamps,
                ),
            );
    }
}

fn drive_wire_tool(
    mode: Res<ToolMode>,
    cursor: Res<GroundCursor>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut chain: ResMut<WireChainStart>,
    mut net_kind: ResMut<ActiveNetKind>,
    mut edits: ResMut<WireEditQueue>,
) {
    if *mode != ToolMode::Wire {
        chain.0 = None;
        return;
    }
    // Repeat presses of the wire key cycle the utility being laid (the
    // press that *entered* wire mode doesn't count — mode just changed).
    if keys.just_pressed(KeyCode::Digit4) && !mode.is_changed() {
        net_kind.0 = match net_kind.0 {
            NetKind::Power => NetKind::Water,
            NetKind::Water => NetKind::Heat,
            NetKind::Heat => NetKind::Power,
        };
        info!("wire kind: {:?}", net_kind.0);
    }
    // X cuts the span under the cursor regardless of chain state
    if keys.just_pressed(KeyCode::KeyX)
        && let Some(pos) = cursor.0
    {
        edits.0.push(WireEdit::RemoveNear { pos });
        return;
    }
    if buttons.just_pressed(MouseButton::Right) {
        chain.0 = None;
        return;
    }
    if buttons.just_pressed(MouseButton::Left)
        && let Some(point) = cursor.0
    {
        match chain.0 {
            None => chain.0 = Some(point),
            Some(from) => {
                edits.0.push(WireEdit::Place {
                    from,
                    to: point,
                    kind: net_kind.0,
                });
                chain.0 = Some(point);
            }
        }
    }
}

fn preview_wire_chain(chain: Res<WireChainStart>, cursor: Res<GroundCursor>, mut gizmos: Gizmos) {
    let (Some(from), Some(to)) = (chain.0, cursor.0) else {
        return;
    };
    gizmos.line(
        from + Vec3::Y * POLE_HEIGHT,
        to + Vec3::Y * POLE_HEIGHT,
        Role::SignalOk.color(),
    );
}

fn sync_pole_meshes(
    mut commands: Commands,
    added: Query<(Entity, &WirePole), Added<WirePole>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, pole) in &added {
        let timber = Mat::new(Role::Timber).add_to(&mut materials);
        commands
            .entity(entity)
            .insert((
                Mesh3d(meshes.add(Cylinder::new(0.28, POLE_HEIGHT))),
                MeshMaterial3d(timber.clone()),
                Transform::from_translation(pole.pos + Vec3::Y * (POLE_HEIGHT * 0.5)),
                Name::new("WirePole"),
            ))
            .with_children(|parent| {
                // crossarm near the tip
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(2.2, 0.22, 0.22))),
                    MeshMaterial3d(timber),
                    Transform::from_xyz(0.0, POLE_HEIGHT * 0.5 - 0.7, 0.0),
                ));
            });
    }
}

/// Where a span attaches on an endpoint: pole tip or building roof edge.
fn attach_point(
    entity: Entity,
    poles: &Query<&WirePole>,
    buildings: &Query<&Building>,
) -> Option<Vec3> {
    if let Ok(pole) = poles.get(entity) {
        return Some(pole.pos + Vec3::Y * POLE_HEIGHT);
    }
    let building = buildings.get(entity).ok()?;
    Some(building.pos + Vec3::Y * kind_height(building.kind))
}

fn draw_spans(
    spans: Query<&WireSpan>,
    poles: Query<&WirePole>,
    buildings: Query<&Building>,
    mut gizmos: Gizmos,
) {
    for span in &spans {
        let (Some(a), Some(b)) = (
            attach_point(span.a, &poles, &buildings),
            attach_point(span.b, &poles, &buildings),
        ) else {
            continue;
        };
        // shallow sag so wires read as wires, not survey lines; pipes run
        // at grade in their utility colour
        let color = net_color(span.kind);
        let sag = if span.kind == NetKind::Power {
            1.0
        } else {
            0.0
        };
        let mid = (a + b) * 0.5 - Vec3::Y * (a.distance(b) * 0.04).min(2.0) * sag;
        gizmos.line(a, mid, color);
        gizmos.line(mid, b, color);
    }
}

/// Factory power lamp: yellow when the gate holds, dark red when starved.
fn draw_power_lamps(lamps: Query<(&Building, &Powered)>, mut gizmos: Gizmos) {
    for (building, powered) in &lamps {
        let color = if powered.0 {
            Role::SignalOk.color()
        } else {
            Role::SignalFail.color()
        };
        let anchor = building.pos + Vec3::Y * (kind_height(building.kind) + 2.0);
        gizmos.sphere(Isometry3d::from_translation(anchor), 0.8, color);
    }
}
