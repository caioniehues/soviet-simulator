//! Public transit stage 1 (B5, ROADMAP "The Lines"): player-drawn bus lines
//! over the one road network. A line is an ordered loop of bus-stop
//! buildings; buses (B5.2) are depot-owned assets assigned to a line that
//! route stop→stop through the B4 pathfinding/traffic stack — transit rides
//! the same congestion, car-following and stall machinery as freight.
//! Structural change happens only at the ApplyCommands barrier.

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::dispatch::{Progress, drive_toward};
use super::pathfinding::{PathService, PathfindingSimPlugin};
use super::roads::{RoadNode, RoadSegment};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};
use super::traffic::{LaneOccupancy, LanePrep, TrafficSimPlugin};
use super::vehicles::{ActivePawn, ActiveVehicle, PawnOf, VehicleAsset, VehicleKind, nearest_node};

/// Seats on a bus; boarding stops at capacity — the queue at the shelter is
/// the overcrowding signal (B5.3).
pub const BUS_CAPACITY: usize = 30;

/// Bus cruise speed, metres per commute travel-second (3× walking pace —
/// riding must genuinely beat the pavement or mode choice is a fiction).
pub const BUS_SPEED: f32 = 24.0;
/// Ticks a bus dwells at each stop for boarding/alighting.
pub const BUS_DWELL_TICKS: u32 = 40;

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
    /// Put an idle bus parked at `depot` into service on `line`.
    AssignBus { line: Entity, depot: Entity },
}

/// Service state on the bus *asset* (persistent identity), mirroring
/// `dispatch::FreightJob`: which line it serves and where it's headed.
#[derive(Component, Debug)]
pub struct BusDuty {
    pub line: Entity,
    /// Index into the line's stop list the bus is driving toward.
    pub next_stop: usize,
    /// Ticks left dwelling at a stop (0 = driving).
    pub dwell: u32,
    /// Riders aboard as (commuter pawn, alight stop). Bounded by
    /// `BUS_CAPACITY` — a full bus boards nobody and the shelter queue is
    /// the visible overcrowding signal.
    pub riders: Vec<(Entity, Entity)>,
}

/// Waiting commuters per stop building, FIFO: (pawn, alight stop). Commuters
/// join on reaching the shelter (`commute::advance_commuters`); buses drain
/// it while dwelling.
#[derive(Resource, Default)]
pub struct StopQueues(pub bevy::platform::collections::HashMap<Entity, Vec<(Entity, Entity)>>);

#[derive(Resource, Default)]
pub struct TransitIds {
    pub next_line: u64,
}

pub struct TransitSimPlugin;

impl Plugin for TransitSimPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<PathfindingSimPlugin>() {
            app.add_plugins(PathfindingSimPlugin);
        }
        if !app.is_plugin_added::<TrafficSimPlugin>() {
            app.add_plugins(TrafficSimPlugin);
        }
        app.init_resource::<TransitEditQueue>()
            .init_resource::<TransitIds>()
            .init_resource::<StopQueues>()
            .add_systems(
                SimTick,
                (apply_transit_edits, prune_dead_stops)
                    .chain()
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            )
            .add_systems(
                SimTick,
                run_buses
                    .in_set(SimStage::MovementAndTransfers)
                    .after(LanePrep),
            );
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn apply_transit_edits(
    mut commands: Commands,
    mut queue: ResMut<TransitEditQueue>,
    mut ids: ResMut<TransitIds>,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &RoadNode)>,
    lines: Query<Entity, With<TransitLine>>,
    fleet: Query<(Entity, &VehicleAsset, Has<ActivePawn>, Has<BusDuty>), super::customs::Arrived>,
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
                        b.kind == BuildingKind::BusStop && nearest_node(b.pos, &nodes).is_some()
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
            TransitEdit::AssignBus { line, depot } => {
                if lines.get(line).is_err() {
                    warn!("AssignBus dropped: {line:?} is not a transit line");
                    continue;
                }
                // An idle parked bus homed at this depot.
                let Some((asset, home)) = fleet
                    .iter()
                    .find(|(_, a, on_road, on_duty)| {
                        a.kind == VehicleKind::Bus && a.home_depot == depot && !on_road && !on_duty
                    })
                    .map(|(e, a, ..)| (e, a.home_depot))
                else {
                    warn!("AssignBus dropped: no idle bus parked at {depot:?}");
                    continue;
                };
                // Roll out from the depot's road node, like a freight trip.
                let start = buildings
                    .get(home)
                    .ok()
                    .and_then(|b| nearest_node(b.pos, &nodes));
                let Some(start) = start else {
                    warn!("AssignBus dropped: depot {depot:?} docks no road node");
                    continue;
                };
                let pos = nodes.get(start).unwrap().1.pos;
                commands.entity(asset).insert(BusDuty {
                    line,
                    next_stop: 0,
                    dwell: 0,
                    riders: Vec::new(),
                });
                commands.spawn((ActiveVehicle::at(pos), PawnOf(asset)));
            }
        }
    }
}

/// A demolished shelter (B6.5) drops out of every line; a line left with
/// fewer than two stops dissolves and its buses head home.
fn prune_dead_stops(
    mut commands: Commands,
    mut lines: Query<(Entity, &mut TransitLine)>,
    stops: Query<&Building>,
) {
    for (entity, mut line) in &mut lines {
        let before = line.stops.len();
        line.stops.retain(|&s| stops.get(s).is_ok());
        if line.stops.len() < 2 {
            commands.entity(entity).despawn();
        } else if line.stops.len() != before {
            warn!("line {:?} lost a demolished stop", line.id);
        }
    }
}

/// The bus service loop, mirroring `run_freight`'s shape: drive to the next
/// stop through the full B4 stack (async routes, car-following, congestion,
/// stalls), dwell for boarding, advance the loop. A deleted line sends the
/// bus home; a severed stop holds and retries — a line is a plan, and its
/// failure mode is visible waiting, never a despawn.
#[allow(clippy::too_many_arguments)]
fn run_buses(
    mut commands: Commands,
    mut pawns: Query<(Entity, &mut ActiveVehicle, &PawnOf)>,
    mut duties: Query<&mut BusDuty>,
    mut riders_q: Query<&mut super::commute::CommuterPawn>,
    mut queues: ResMut<StopQueues>,
    assets: Query<&VehicleAsset>,
    lines: Query<&TransitLine>,
    buildings: Query<&Building>,
    mut svc: ResMut<PathService>,
    occupancy: Res<LaneOccupancy>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    use super::commute::CommutePhase;
    // Passenger transit runs on the commute travel-second timescale (one
    // travel-second per tick, like pedestrians) — riding must genuinely beat
    // walking. Freight keeps its own compressed scale (vehicles.rs). The
    // movement engine budgets TRUCK_SPEED × dt, so the bus's cruise speed is
    // expressed through dt.
    let dt = BUS_SPEED / super::vehicles::TRUCK_SPEED;
    for (pawn, mut vehicle, pawn_of) in &mut pawns {
        let Ok(mut duty) = duties.get_mut(pawn_of.0) else {
            continue;
        };
        if let Ok(line) = lines.get(duty.line) {
            if duty.dwell > 0 {
                duty.dwell -= 1;
                continue;
            }
            let stop = line.stops.get(duty.next_stop % line.stops.len()).copied();
            let goal = || {
                stop.and_then(|s| buildings.get(s).ok())
                    .and_then(|b| nearest_node(b.pos, &nodes))
            };
            match drive_toward(
                pawn,
                &mut vehicle,
                goal,
                dt,
                &mut svc,
                &occupancy,
                &nodes,
                &segments,
            ) {
                Progress::Arrived => {
                    if let Some(stop) = stop {
                        // Alight first — their seats free up for boarding. A
                        // rider whose alight stop was demolished (B6.5) steps
                        // off at the next dwell and walks the rest.
                        duty.riders.retain(|&(rider, alight)| {
                            if alight != stop && buildings.get(alight).is_ok() {
                                return true;
                            }
                            if let Ok(mut r) = riders_q.get_mut(rider) {
                                r.phase = CommutePhase::Resume;
                            }
                            false
                        });
                        // Board FIFO while seats remain; a rider whose alight
                        // stop this line never serves stays queued for the
                        // right line.
                        if let Some(queue) = queues.0.get_mut(&stop) {
                            let mut i = 0;
                            while i < queue.len() && duty.riders.len() < BUS_CAPACITY {
                                let (rider, alight) = queue[i];
                                if line.stops.contains(&alight)
                                    && let Ok(mut r) = riders_q.get_mut(rider)
                                {
                                    r.phase = CommutePhase::Ride { bus: pawn };
                                    r.transit = None;
                                    duty.riders.push((rider, alight));
                                    queue.remove(i);
                                } else {
                                    i += 1;
                                }
                            }
                        }
                    }
                    duty.dwell = BUS_DWELL_TICKS;
                    duty.next_stop = line.next_stop(duty.next_stop);
                }
                Progress::Moving | Progress::NoPath => {}
            }
        } else {
            // Line deleted: dead-head home and park.
            let asset = pawn_of.0;
            let home = || {
                assets
                    .get(asset)
                    .ok()
                    .and_then(|a| buildings.get(a.home_depot).ok())
                    .and_then(|b| nearest_node(b.pos, &nodes))
            };
            let done = match drive_toward(
                pawn,
                &mut vehicle,
                home,
                dt,
                &mut svc,
                &occupancy,
                &nodes,
                &segments,
            ) {
                Progress::Arrived => true,
                Progress::NoPath => home().is_none(),
                Progress::Moving => false,
            };
            if done {
                // Anyone still aboard steps off at the depot and walks on.
                for (rider, _) in duty.riders.drain(..) {
                    if let Ok(mut r) = riders_q.get_mut(rider) {
                        r.phase = CommutePhase::Resume;
                    }
                }
                commands.entity(pawn).despawn();
                commands.entity(pawn_of.0).remove::<BusDuty>();
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
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            TransitSimPlugin,
        ));
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
            .push(TransitEdit::CreateLine { stops: vec![west] });
        ticks(&mut app, 1);
        let world = app.world_mut();
        assert_eq!(world.query::<&TransitLine>().iter(world).count(), 0);
    }

    fn line_with_bus(app: &mut App) -> (Entity, Entity) {
        use super::super::buildings::BuildingEdit;
        use super::super::vehicles::{VehicleEdit, VehicleEditQueue};
        let (west, east) = stop_world(app);
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Depot,
                pos: Vec3::new(0.0, 0.0, 20.0),
            });
        ticks(app, 2);
        let world = app.world_mut();
        let depot = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .find(|(_, b)| b.kind == BuildingKind::Depot)
            .unwrap()
            .0;
        world
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyBus { depot });
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::CreateLine {
                stops: vec![west, east],
            });
        ticks(app, 2);
        let world = app.world_mut();
        let line = world
            .query_filtered::<Entity, With<TransitLine>>()
            .single(world)
            .unwrap();
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::AssignBus { line, depot });
        ticks(app, 2);
        (line, depot)
    }

    fn bus_app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            super::super::plan::PlanSimPlugin,
            super::super::vehicles::VehicleSimPlugin,
            TransitSimPlugin,
        ));
        a
    }

    #[test]
    fn assigned_bus_loops_the_line_and_dwells() {
        let mut app = bus_app();
        line_with_bus(&mut app);
        // reach the east stop (200 m at truck speed on dirt ≈ 1850 ticks),
        // sampling the pawn so we can prove it actually crossed the map
        let mut max_x = f32::MIN;
        let mut looped = false;
        for _ in 0..30 {
            ticks(&mut app, 100);
            let world = app.world_mut();
            if let Ok(v) = world.query::<&ActiveVehicle>().single(world) {
                max_x = max_x.max(v.pos.x);
            }
            let duty = world.query::<&BusDuty>().single(world).unwrap();
            if duty.next_stop == 0 && max_x > 150.0 {
                looped = true;
            }
        }
        assert!(
            max_x > 150.0,
            "bus must reach the east stop, max_x = {max_x}"
        );
        assert!(looped, "arrival must advance the loop back toward stop 0");
        let world = app.world_mut();
        assert_eq!(
            world.query::<&ActiveVehicle>().iter(world).count(),
            1,
            "the bus serves the loop forever — never despawned"
        );
    }

    #[test]
    fn deleting_the_line_sends_the_bus_home_to_park() {
        let mut app = bus_app();
        let (line, _) = line_with_bus(&mut app);
        ticks(&mut app, 300); // en route eastbound
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::DeleteLine { line });
        ticks(&mut app, 3000); // dead-head home from anywhere on the map
        let world = app.world_mut();
        assert_eq!(
            world.query::<&ActiveVehicle>().iter(world).count(),
            0,
            "pawn parks (despawns) at the depot"
        );
        assert_eq!(world.query::<&BusDuty>().iter(world).count(), 0);
        let world = app.world_mut();
        let bus = world.query::<&VehicleAsset>().single(world).unwrap();
        assert_eq!(bus.kind, VehicleKind::Bus, "the asset survives parked");
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
