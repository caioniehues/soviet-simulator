//! Public transit stage 1 (B5, ROADMAP "The Lines"): player-drawn bus lines
//! over the one road network. A line is an ordered loop of bus-stop
//! buildings; buses (B5.2) are depot-owned assets assigned to a line that
//! route stop→stop through the B4 pathfinding/traffic stack — transit rides
//! the same congestion, car-following and stall machinery as freight.
//! Structural change happens only at the ApplyCommands barrier.

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::roads::RoadNode;
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};
use super::vehicles::nearest_node;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LineId(pub u64);

/// A bus line: an ordered loop of stops. Buses serve
/// `stops[0] → stops[1] → … → stops[0]` forever. Routing between stops is
/// live (PathService per leg), so road edits and congestion re-price the
/// line without any stored geometry.
#[derive(Component, Debug)]
pub struct TransitLine {
    pub id: LineId,
    pub stops: Vec<Entity>,
}

impl TransitLine {
    /// The stop after `index`, wrapping the loop.
    pub fn next_stop(&self, index: usize) -> usize {
        (index + 1) % self.stops.len()
    }
}

#[derive(Resource, Default)]
pub struct TransitEditQueue(pub Vec<TransitEdit>);

#[derive(Clone, Debug)]
pub enum TransitEdit {
    /// Create a line over ≥2 bus stops. Every stop must be a `BusStop`
    /// building docked to a road node — an undocked shelter is unreachable
    /// and rejects the whole line (severability is real, as with yards).
    CreateLine { stops: Vec<Entity> },
    /// Remove the line. Its buses head home (B5.2) and riders re-plan on
    /// foot (B5.4); the stops themselves stay standing.
    DeleteLine { line: Entity },
}

#[derive(Resource, Default)]
pub struct TransitIds {
    pub next_line: u64,
}

pub struct TransitSimPlugin;

impl Plugin for TransitSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TransitEditQueue>()
            .init_resource::<TransitIds>()
            .add_systems(
                SimTick,
                apply_transit_edits
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            );
    }
}

fn apply_transit_edits(
    mut commands: Commands,
    mut queue: ResMut<TransitEditQueue>,
    mut ids: ResMut<TransitIds>,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &RoadNode)>,
    lines: Query<Entity, With<TransitLine>>,
) {
    for edit in std::mem::take(&mut queue.0) {
        match edit {
            TransitEdit::CreateLine { stops } => {
                if stops.len() < 2 {
                    warn!("CreateLine dropped: needs at least 2 stops");
                    continue;
                }
                let all_docked_stops = stops.iter().all(|&stop| {
                    buildings.get(stop).is_ok_and(|b| {
                        b.kind == BuildingKind::BusStop
                            && nearest_node(b.pos, &nodes).is_some()
                    })
                });
                if !all_docked_stops {
                    warn!("CreateLine dropped: every stop must be a road-docked BusStop");
                    continue;
                }
                ids.next_line += 1;
                commands.spawn(TransitLine {
                    id: LineId(ids.next_line),
                    stops,
                });
            }
            TransitEdit::DeleteLine { line } => {
                if lines.get(line).is_ok() {
                    commands.entity(line).despawn();
                } else {
                    warn!("DeleteLine dropped: {line:?} is not a transit line");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingSimPlugin};
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, RoadSimPlugin, BuildingSimPlugin, TransitSimPlugin));
        a
    }

    fn ticks(app: &mut App, n: u32) {
        for _ in 0..n {
            app.world_mut()
                .resource_mut::<Time>()
                .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
            app.update();
        }
    }

    /// Road x∈[0,200] with bus stops near both ends; returns (west, east).
    fn stop_world(app: &mut App) -> (Entity, Entity) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(200.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            for pos in [Vec3::new(0.0, 0.0, 8.0), Vec3::new(200.0, 0.0, 8.0)] {
                buildings.0.push(BuildingEdit::Place {
                    kind: BuildingKind::BusStop,
                    pos,
                });
            }
        }
        ticks(app, 2);
        let world = app.world_mut();
        let mut found: Vec<(f32, Entity)> = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .filter(|(_, b)| b.kind == BuildingKind::BusStop)
            .map(|(e, b)| (b.pos.x, e))
            .collect();
        found.sort_by(|a, b| a.0.total_cmp(&b.0));
        (found[0].1, found[1].1)
    }

    #[test]
    fn line_over_docked_stops_is_created_and_loops() {
        let mut app = app();
        let (west, east) = stop_world(&mut app);
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::CreateLine {
                stops: vec![west, east],
            });
        ticks(&mut app, 1);
        let world = app.world_mut();
        let line = world.query::<&TransitLine>().single(world).unwrap();
        assert_eq!(line.stops, vec![west, east]);
        assert_eq!(line.id, LineId(1));
        assert_eq!(line.next_stop(1), 0, "the line is a loop");
    }

    #[test]
    fn undocked_or_wrong_kind_stops_reject_the_line() {
        let mut app = app();
        let (west, _) = stop_world(&mut app);
        // A shelter far off any road: dock fails, whole line rejected.
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::BusStop,
                pos: Vec3::new(500.0, 0.0, 500.0),
            });
        ticks(&mut app, 2);
        let world = app.world_mut();
        let stranded = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .find(|(_, b)| b.pos.x > 400.0)
            .unwrap()
            .0;
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::CreateLine {
                stops: vec![west, stranded],
            });
        // A non-stop building is no line stop either.
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::CreateLine {
                stops: vec![west],
            });
        ticks(&mut app, 1);
        let world = app.world_mut();
        assert_eq!(world.query::<&TransitLine>().iter(world).count(), 0);
    }

    #[test]
    fn delete_line_despawns_it() {
        let mut app = app();
        let (west, east) = stop_world(&mut app);
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::CreateLine {
                stops: vec![west, east],
            });
        ticks(&mut app, 1);
        let world = app.world_mut();
        let line = world
            .query_filtered::<Entity, With<TransitLine>>()
            .single(world)
            .unwrap();
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::DeleteLine { line });
        ticks(&mut app, 1);
        let world = app.world_mut();
        assert_eq!(world.query::<&TransitLine>().iter(world).count(), 0);
        // stops survive the line
        let stops = world
            .query::<&Building>()
            .iter(world)
            .filter(|b| b.kind == BuildingKind::BusStop)
            .count();
        assert_eq!(stops, 2);
    }
}
