//! Commute trips (spec/citizens.md § Trips, ticket #26): assigned citizens
//! physically travel home ⇄ work over the lane graph as transient pawns —
//! the flat `Citizen` struct never moves, a `CommuterPawn` entity exists only
//! while travelling (both research labs agree on the split). Presence at the
//! workplace, tallied here into `Staffing::present`, is what production gates
//! on (M2.5). A blocked or severed route means the worker is simply absent —
//! the same legible cascade as a cut freight road.
//!
//! Timescale: one commute "travel second" elapses per sim frame, so the
//! feasibility budget (`MAX_COMMUTE_SECS` = 120) is a fifth of the 600-frame
//! game day and a workday commute fits the day comfortably. B5's answer to
//! the timescale question: passenger buses run on this same travel-second
//! scale (transit must genuinely outpace walking), while freight keeps its
//! compressed scale (see vehicles.rs).
//!
//! B5.3 multi-leg trips: a commuter whose plan includes a transit ride walks
//! to the board stop, queues (`transit::StopQueues`), rides as a passenger
//! of the bus pawn, and resumes on foot after alighting. A vanished line or
//! full buses degrade to waiting — and past a give-up threshold, to walking.

use std::collections::HashMap;

use bevy::prelude::*;

use super::buildings::Building;
use super::citizens::{Citizen, CitizenLocation};
use super::clock::{FRAMES_PER_GAME_DAY, FrameIndex};
use super::labour::{COMMUTE_SPEED, Staffing};
use super::needs::CitizenNeeds;
use super::pathfinding::{CostProfile, PathService, PathSystems, PathfindingSimPlugin};
use super::roads::{LaneDir, RoadNode, RoadSegment};
use super::stages::{SimStage, SimTick};
use super::transit::{StopQueues, TransitLine, TransitSimPlugin};
use super::vehicles::{RouteLeg, nearest_node, nearest_node_unbounded};

/// Morning departures begin at this frame of the day (06:00 of the 600-frame
/// day), evening departures at `EVENING_FRAME`. Each citizen adds a personal
/// jitter so departures stream instead of pulsing (CS1's probability curve,
/// made deterministic).
pub const MORNING_FRAME: u32 = 150;
pub const EVENING_FRAME: u32 = 450;
pub const DEPARTURE_JITTER_FRAMES: u32 = 60;

/// Ticks a commuter waits at a stop before giving up on the bus and walking
/// the rest (a deleted line must degrade to absence, never trap a citizen).
pub const WAIT_GIVE_UP_TICKS: u32 = 900;

/// Where a trip currently is in its multi-leg plan (B5.3).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommutePhase {
    /// Walking the current `route`.
    Walk,
    /// Standing at the board stop, queued for a bus.
    Wait { ticks: u32 },
    /// Aboard this bus pawn; position follows it.
    Ride { bus: Entity },
    /// Between legs (alighted, gave up, or lost the bus): re-plan the walk
    /// to the destination from wherever the pawn stands.
    Resume,
}

/// Transient trip pawn; exists only door-to-door. Presentation eases a
/// rendered figure toward `pos` (ADR 0003).
#[derive(Component, Debug)]
pub struct CommuterPawn {
    pub citizen: Entity,
    pub pos: Vec3,
    pub heading: Vec3,
    pub route: Vec<RouteLeg>,
    pub leg: usize,
    /// Metres travelled along the current leg.
    pub s: f32,
    pub to_work: bool,
    pub phase: CommutePhase,
    /// Planned transit leg after the current walk: (board stop, alight stop).
    pub transit: Option<(Entity, Entity)>,
}

pub struct CommuteSimPlugin;

impl Plugin for CommuteSimPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<PathfindingSimPlugin>() {
            app.add_plugins(PathfindingSimPlugin);
        }
        if !app.is_plugin_added::<TransitSimPlugin>() {
            app.add_plugins(TransitSimPlugin);
        }
        app.add_systems(
            SimTick,
            depart_commuters
                .in_set(SimStage::Routing)
                .after(PathSystems),
        )
        .add_systems(
            SimTick,
            (advance_commuters, tally_presence)
                .chain()
                .in_set(SimStage::MovementAndTransfers),
        );
    }
}

fn departure_jitter(citizen: &Citizen) -> u32 {
    (citizen.id.0 as u32).wrapping_mul(7) % DEPARTURE_JITTER_FRAMES
}

/// Expected stop wait folded into the transit cost estimate, travel-seconds
/// (simutrans prices waiting ~8× a plain hop — findings.md; ours is a flat
/// half-headway stand-in until schedules land).
pub const WAIT_PENALTY_SECS: f32 = 15.0;

/// A candidate single-ride transit plan with its estimated door-to-door cost.
pub struct Itinerary {
    pub board: Entity,
    pub alight: Entity,
    pub cost: f32,
}

/// Cheapest walk→ride→walk itinerary over all lines (v1: one ride, no
/// transfers). Straight-line distances estimate each leg; `None` when no
/// line offers two distinct stops.
pub fn best_itinerary(
    from: Vec3,
    to: Vec3,
    lines: &Query<&TransitLine>,
    buildings: &Query<&Building>,
) -> Option<Itinerary> {
    let mut best: Option<Itinerary> = None;
    for line in lines.iter() {
        let stop_pos = |e: Entity| buildings.get(e).ok().map(|b| b.pos);
        let nearest = |target: Vec3| {
            line.stops
                .iter()
                .filter_map(|&s| stop_pos(s).map(|p| (s, p.distance(target))))
                .min_by(|a, b| a.1.total_cmp(&b.1))
        };
        let (Some((board, d_walk1)), Some((alight, d_walk2))) = (nearest(from), nearest(to)) else {
            continue;
        };
        if board == alight {
            continue;
        }
        let (Some(bp), Some(ap)) = (stop_pos(board), stop_pos(alight)) else {
            continue;
        };
        let cost = (d_walk1 + d_walk2) / COMMUTE_SPEED
            + WAIT_PENALTY_SECS
            + bp.distance(ap) / super::transit::BUS_SPEED;
        if best.as_ref().is_none_or(|b| cost < b.cost) {
            best = Some(Itinerary {
                board,
                alight,
                cost,
            });
        }
    }
    best
}

/// Spawn pawns for citizens whose departure window is open. No route right
/// now (severed network, missing dock) means stay put and retry next frame —
/// the worker is visibly absent, never teleported.
#[allow(clippy::too_many_arguments)]
fn depart_commuters(
    mut commands: Commands,
    frame: Res<FrameIndex>,
    svc: Res<PathService>,
    mut citizens: Query<(Entity, &mut Citizen, Option<&CitizenNeeds>)>,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &RoadNode)>,
    lines: Query<&TransitLine>,
) {
    let day = frame.0 % FRAMES_PER_GAME_DAY;
    for (entity, mut citizen, needs) in &mut citizens {
        let (Some(home), Some(work)) = (citizen.home, citizen.work) else {
            continue;
        };
        let jitter = departure_jitter(&citizen);
        let (from, to, to_work) = match citizen.location {
            CitizenLocation::AtHome if day >= MORNING_FRAME + jitter && day < EVENING_FRAME => {
                // Wellbeing gates the workday itself (M2.6): the per-day
                // attendance roll lives in the needs pass; a citizen rolled
                // out stays home the whole day.
                if needs.is_some_and(|n| !n.attends) {
                    continue;
                }
                (home, work, true)
            }
            CitizenLocation::AtWork if day >= EVENING_FRAME + jitter || day < MORNING_FRAME => {
                (work, home, false)
            }
            _ => continue,
        };
        let (Ok(from_b), Ok(to_b)) = (buildings.get(from), buildings.get(to)) else {
            continue;
        };
        let (Some(start), Some(goal)) = (
            nearest_node(from_b.pos, &nodes),
            nearest_node(to_b.pos, &nodes),
        ) else {
            continue;
        };
        // Mode choice (B5.3/5.4): a transit itinerary that beats the walk on
        // time-plus-wait cost turns the trip multi-leg — walk to the board
        // stop, ride, walk on. Straight-line estimates keep this a cheap
        // per-departure scan; the actual legs are properly routed.
        let walk_est = from_b.pos.distance(to_b.pos) / COMMUTE_SPEED;
        let transit = best_itinerary(from_b.pos, to_b.pos, &lines, &buildings)
            .filter(|it| it.cost < walk_est);
        // Commute tolerance is behavioural, not just a planner gate (B5.4):
        // a workday trip whose best mode blows the budget is refused — the
        // citizen stays home, visibly absent. Deleting the only viable line
        // degrades far workplaces exactly this way. The trip *home* is never
        // refused — nobody sleeps at the factory over a schedule change.
        let best_cost = transit.as_ref().map_or(walk_est, |it| it.cost);
        if to_work && best_cost > super::labour::MAX_COMMUTE_SECS {
            continue;
        }
        let (walk_goal, transit_legs) = match &transit {
            Some(it) => {
                let board_dock = buildings
                    .get(it.board)
                    .ok()
                    .and_then(|b| nearest_node(b.pos, &nodes));
                match board_dock {
                    Some(dock) => (dock, Some((it.board, it.alight))),
                    None => (goal, None),
                }
            }
            None => (goal, None),
        };
        // Walkers route synchronously on the shared snapshot (stable
        // distance cost, no congestion jitter — spec/pathfinding.md); the
        // request/poll round-trip would only delay every departure a tick.
        let Some(route) = svc.route_now(start, walk_goal, CostProfile::Pedestrian, citizen.id.0)
        else {
            continue;
        };
        let pos = nodes.get(start).unwrap().1.pos;
        commands.spawn(CommuterPawn {
            citizen: entity,
            pos,
            heading: Vec3::X,
            route,
            leg: 0,
            s: 0.0,
            to_work,
            phase: CommutePhase::Walk,
            transit: transit_legs,
        });
        citizen.location = if to_work {
            CitizenLocation::ToWork
        } else {
            CitizenLocation::ToHome
        };
    }
}

/// Walk the route one travel-second per frame; queue, ride and resume for
/// transit legs (B5.3). Final arrival flips the citizen's location and
/// releases the pawn; a severed route aborts the trip — the citizen ends up
/// at home, absent from work.
#[allow(clippy::too_many_arguments)]
fn advance_commuters(
    mut commands: Commands,
    svc: Res<PathService>,
    mut queues: ResMut<StopQueues>,
    mut pawns: Query<(Entity, &mut CommuterPawn)>,
    mut citizens: Query<&mut Citizen>,
    buildings: Query<&Building>,
    buses: Query<&super::vehicles::ActiveVehicle>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    for (pawn_entity, mut pawn) in &mut pawns {
        let mut arrived = false;
        let mut aborted = false;
        match pawn.phase {
            CommutePhase::Wait { ticks } => {
                if ticks >= WAIT_GIVE_UP_TICKS {
                    // The bus is not coming (deleted line, endless overflow):
                    // leave the queue and walk the rest.
                    if let Some((board, _)) = pawn.transit.take()
                        && let Some(queue) = queues.0.get_mut(&board)
                    {
                        queue.retain(|(p, _)| *p != pawn_entity);
                    }
                    pawn.phase = CommutePhase::Resume;
                } else {
                    pawn.phase = CommutePhase::Wait { ticks: ticks + 1 };
                }
                continue;
            }
            CommutePhase::Ride { bus } => {
                // The rider is wherever the bus is; alighting is the bus's
                // dwell logic (transit::run_buses). A vanished bus strands
                // the rider mid-road: resume on foot.
                match buses.get(bus) {
                    Ok(vehicle) => {
                        pawn.pos = vehicle.pos;
                        pawn.heading = vehicle.heading;
                    }
                    Err(_) => pawn.phase = CommutePhase::Resume,
                }
                continue;
            }
            CommutePhase::Resume => {
                // Re-plan the walk to the destination from where we stand.
                let dest = citizens
                    .get(pawn.citizen)
                    .ok()
                    .and_then(|c| if pawn.to_work { c.work } else { c.home });
                let goal = dest
                    .and_then(|d| buildings.get(d).ok())
                    .and_then(|b| nearest_node(b.pos, &nodes));
                let start = nearest_node_unbounded(pawn.pos, &nodes);
                let route = match (start, goal) {
                    (Some(start), Some(goal)) => {
                        let seed = citizens.get(pawn.citizen).map(|c| c.id.0).unwrap_or(0);
                        svc.route_now(start, goal, CostProfile::Pedestrian, seed)
                    }
                    _ => None,
                };
                match route {
                    Some(route) => {
                        if let Some(start) = start
                            && let Ok((_, n)) = nodes.get(start)
                        {
                            pawn.pos = n.pos;
                        }
                        pawn.route = route;
                        pawn.leg = 0;
                        pawn.s = 0.0;
                        pawn.transit = None;
                        pawn.phase = CommutePhase::Walk;
                    }
                    None => aborted = true,
                }
                if !aborted {
                    continue;
                }
            }
            CommutePhase::Walk => {
                let mut budget = f32::MAX;
                loop {
                    let Some(leg) = pawn.route.get(pawn.leg).copied() else {
                        arrived = true;
                        break;
                    };
                    let Ok(segment) = segments.get(leg.segment) else {
                        aborted = true;
                        break;
                    };
                    let Some(lane) = segment.lanes.iter().find(|l| l.dir == leg.dir) else {
                        aborted = true;
                        break;
                    };
                    if budget == f32::MAX {
                        budget = COMMUTE_SPEED * lane.speed_modifier;
                    }
                    let remaining = segment.length - pawn.s;
                    if budget < remaining {
                        pawn.s += budget;
                        place_on_leg(&mut pawn, segment, leg.dir);
                        break;
                    }
                    budget -= remaining;
                    pawn.s = segment.length;
                    place_on_leg(&mut pawn, segment, leg.dir);
                    pawn.leg += 1;
                    pawn.s = 0.0;
                }
                // Walk leg done with a transit ride still ahead: join the
                // stop queue instead of finishing the trip.
                if arrived && let Some((board, alight)) = pawn.transit {
                    queues
                        .0
                        .entry(board)
                        .or_default()
                        .push((pawn_entity, alight));
                    pawn.phase = CommutePhase::Wait { ticks: 0 };
                    continue;
                }
            }
        }
        if !(arrived || aborted) {
            continue;
        }
        if let Ok(mut citizen) = citizens.get_mut(pawn.citizen) {
            citizen.location = if arrived && pawn.to_work {
                CitizenLocation::AtWork
            } else {
                // Arrived home, or stranded: they make their own way back.
                CitizenLocation::AtHome
            };
        }
        commands.entity(pawn_entity).despawn();
    }
}

fn place_on_leg(pawn: &mut CommuterPawn, segment: &RoadSegment, dir: LaneDir) {
    let (pos, travel) = segment.point_at(pawn.s, dir);
    // Pedestrians keep to the verge, outside the vehicle lane.
    let verge = segment.class.width() * 0.5 + 0.6;
    pawn.heading = travel;
    pawn.pos = pos + travel.cross(Vec3::Y) * verge;
}

/// Presence is recomputed from citizen locations every tick — production
/// reads `Staffing::present` in the very next stage.
fn tally_presence(citizens: Query<&Citizen>, mut staffing: Query<(Entity, &mut Staffing)>) {
    let mut present: HashMap<Entity, u32> = HashMap::new();
    for citizen in &citizens {
        if citizen.location == CitizenLocation::AtWork
            && let Some(work) = citizen.work
        {
            *present.entry(work).or_default() += 1;
        }
    }
    for (workplace, mut ledger) in &mut staffing {
        let count = present.get(&workplace).copied().unwrap_or(0);
        if ledger.present != count {
            ledger.present = count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
    };
    use super::super::households::{HouseholdSimPlugin, RecruitmentPlan};
    use super::super::labour::LabourSimPlugin;
    use super::super::plan::PlanSimPlugin;
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
            PlanSimPlugin,
            HouseholdSimPlugin,
            LabourSimPlugin,
            CommuteSimPlugin,
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

    /// Dwelling at x=0, mine at x=100, dirt road between, two households.
    fn commuter_town(app: &mut App) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Dwelling,
            pos: Vec3::new(0.0, 0.0, 15.0),
        });
        buildings.0.push(BuildingEdit::Place {
            kind: BuildingKind::Mine,
            pos: Vec3::new(100.0, 0.0, 15.0),
        });
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 2; // 5 citizens, 6 mine slots
    }

    fn mine_presence(app: &mut App) -> u32 {
        let world = app.world_mut();
        world.query::<&Staffing>().single(world).unwrap().present
    }

    #[test]
    fn workers_commute_in_and_presence_tracks_arrivals() {
        let mut app = app();
        commuter_town(&mut app);
        // Morning window opens at frame 150 (+ ≤60 jitter); the 100 m dirt
        // walk takes ~23 frames. By frame 300 everyone assigned is at work.
        ticks(&mut app, 300);
        assert_eq!(mine_presence(&mut app), 5);
        let world = app.world_mut();
        assert_eq!(world.query::<&CommuterPawn>().iter(world).count(), 0);
    }

    #[test]
    fn workers_go_home_in_the_evening() {
        let mut app = app();
        commuter_town(&mut app);
        ticks(&mut app, 600); // full day: in by ~230, out from 450
        assert_eq!(mine_presence(&mut app), 0);
        let world = app.world_mut();
        let at_home = world
            .query::<&Citizen>()
            .iter(world)
            .filter(|c| c.location == CitizenLocation::AtHome)
            .count();
        assert_eq!(at_home, 5);
    }

    #[test]
    fn cutting_the_road_keeps_workers_absent() {
        let mut app = app();
        commuter_town(&mut app);
        ticks(&mut app, 140); // assigned, still before the morning window
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::RemoveNear {
                pos: Vec3::new(50.0, 0.0, 0.0),
            });
        ticks(&mut app, 200); // window opens; no route ⇒ nobody departs
        assert_eq!(mine_presence(&mut app), 0);
        let world = app.world_mut();
        assert_eq!(world.query::<&CommuterPawn>().iter(world).count(), 0);
        let at_home = world
            .query::<&Citizen>()
            .iter(world)
            .filter(|c| c.location == CitizenLocation::AtHome)
            .count();
        assert_eq!(at_home, 5);
    }

    #[test]
    fn production_follows_the_workday() {
        use super::super::resources::{Inventory, ResourceKind};
        let mut app = app();
        commuter_town(&mut app);
        ticks(&mut app, 145); // assigned, but the morning window hasn't opened
        {
            let world = app.world_mut();
            let coal = world
                .query::<(&super::super::buildings::Building, &Inventory)>()
                .iter(world)
                .map(|(_, i)| i.amount(ResourceKind::Coal))
                .sum::<f32>();
            assert_eq!(coal, 0.0, "nobody at work yet ⇒ mine idle");
        }
        ticks(&mut app, 155); // workers walked in; the mine runs part-staffed
        let world = app.world_mut();
        let coal = world
            .query::<(&super::super::buildings::Building, &Inventory)>()
            .iter(world)
            .map(|(_, i)| i.amount(ResourceKind::Coal))
            .sum::<f32>();
        assert!(coal > 0.0, "presence gates production on");
    }

    #[test]
    fn severing_the_route_mid_walk_aborts_the_trip() {
        let mut app = app();
        commuter_town(&mut app);
        ticks(&mut app, 165); // first departures under way (window opens ~157)
        {
            let world = app.world_mut();
            assert!(world.query::<&CommuterPawn>().iter(world).count() > 0);
        }
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::RemoveNear {
                pos: Vec3::new(50.0, 0.0, 0.0),
            });
        ticks(&mut app, 10);
        let world = app.world_mut();
        assert_eq!(world.query::<&CommuterPawn>().iter(world).count(), 0);
        assert_eq!(mine_presence(&mut app), 0);
    }

    // ---- B5.3 riders --------------------------------------------------

    use super::super::transit::{BusDuty, StopQueues, TransitEdit, TransitEditQueue, TransitLine};
    use super::super::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};

    fn transit_app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            super::super::plan::PlanSimPlugin,
            HouseholdSimPlugin,
            LabourSimPlugin,
            VehicleSimPlugin,
            CommuteSimPlugin,
        ));
        a
    }

    /// Dwelling at x=0, mine at x=450 (walk-feasible but transit-cheaper),
    /// bus stops docking both end nodes, depot with one bus on the line.
    fn transit_town(app: &mut App) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(450.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            for (kind, pos) in [
                (BuildingKind::Dwelling, Vec3::new(0.0, 0.0, 15.0)),
                (BuildingKind::Mine, Vec3::new(450.0, 0.0, 15.0)),
                (BuildingKind::BusStop, Vec3::new(5.0, 0.0, -8.0)),
                (BuildingKind::BusStop, Vec3::new(445.0, 0.0, -8.0)),
                (BuildingKind::Depot, Vec3::new(0.0, 0.0, 35.0)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 2;
        ticks(app, 2);
        let world = app.world_mut();
        let mut stops: Vec<(f32, Entity)> = Vec::new();
        let mut depot = None;
        let mut q = world.query::<(Entity, &super::super::buildings::Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::BusStop => stops.push((b.pos.x, e)),
                BuildingKind::Depot => depot = Some(e),
                _ => {}
            }
        }
        stops.sort_by(|a, b| a.0.total_cmp(&b.0));
        let depot = depot.unwrap();
        world
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyBus { depot });
        world
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::CreateLine {
                stops: vec![stops[0].1, stops[1].1],
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
    }

    #[test]
    fn commuters_ride_the_bus_to_work() {
        let mut app = transit_app();
        transit_town(&mut app);
        // Sample through the morning: someone must actually be aboard.
        let mut rode = false;
        for _ in 0..30 {
            ticks(&mut app, 15);
            let world = app.world_mut();
            if world
                .query::<&BusDuty>()
                .iter(world)
                .any(|d| !d.riders.is_empty())
            {
                rode = true;
            }
        }
        assert!(rode, "at least one commuter must ride the bus");
        // Everyone assigned makes it to the mine before the evening window.
        ticks(&mut app, 440_u32.saturating_sub(6 + 30 * 15));
        assert_eq!(
            mine_presence(&mut app),
            5,
            "riders arrive and staff the mine"
        );
    }

    #[test]
    fn far_workplace_staffs_only_while_the_line_runs() {
        let mut app = transit_app();
        // 1200 m: walking blows the 120 s budget (dirt ≈ 273 s) — only the
        // bus line makes the mine reachable at all.
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(1200.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            for (kind, pos) in [
                (BuildingKind::Dwelling, Vec3::new(0.0, 0.0, 15.0)),
                (BuildingKind::Mine, Vec3::new(1200.0, 0.0, 15.0)),
                (BuildingKind::BusStop, Vec3::new(5.0, 0.0, -8.0)),
                (BuildingKind::BusStop, Vec3::new(1195.0, 0.0, -8.0)),
                (BuildingKind::Depot, Vec3::new(0.0, 0.0, 35.0)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 2;
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut stops: Vec<(f32, Entity)> = Vec::new();
        let mut depot = None;
        let mut q = world.query::<(Entity, &super::super::buildings::Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::BusStop => stops.push((b.pos.x, e)),
                BuildingKind::Depot => depot = Some(e),
                _ => {}
            }
        }
        stops.sort_by(|a, b| a.0.total_cmp(&b.0));
        let depot = depot.unwrap();
        world
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyBus { depot });
        world
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::CreateLine {
                stops: vec![stops[0].1, stops[1].1],
            });
        ticks(&mut app, 2);
        let world = app.world_mut();
        let line = world
            .query_filtered::<Entity, With<TransitLine>>()
            .single(world)
            .unwrap();
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::AssignBus { line, depot });
        // Day 1: the line makes the far mine feasible — workers are assigned
        // and, at some point in the workday, present.
        let mut peak = 0;
        for _ in 0..30 {
            ticks(&mut app, 15);
            peak = peak.max(mine_presence(&mut app));
        }
        assert!(
            peak > 0,
            "transit-extended catchment must staff the far mine"
        );
        {
            let world = app.world_mut();
            let assigned = world
                .query::<&Citizen>()
                .iter(world)
                .filter(|c| c.work.is_some())
                .count();
            assert_eq!(assigned, 5, "all five bind through the transit itinerary");
        }
        // Kill the line. Tenure holds, but the next workday's trip is beyond
        // anyone's tolerance on foot — the mine stands unstaffed.
        app.world_mut()
            .resource_mut::<TransitEditQueue>()
            .0
            .push(TransitEdit::DeleteLine { line });
        // run to deep into day 2's work window (frames ~750–1050)
        let so_far = 4 + 30 * 15;
        ticks(&mut app, 1040_u32.saturating_sub(so_far));
        assert_eq!(
            mine_presence(&mut app),
            0,
            "no line, no feasible commute — the far mine degrades to empty"
        );
        let world = app.world_mut();
        let still_assigned = world
            .query::<&Citizen>()
            .iter(world)
            .filter(|c| c.work.is_some())
            .count();
        assert_eq!(still_assigned, 5, "tenure survives the deleted line");
    }

    #[test]
    fn full_bus_leaves_the_overflow_standing_at_the_stop() {
        let mut app = transit_app();
        transit_town(&mut app);
        // Stuff the bus with phantom riders whose alight target is a real
        // building the line never serves (the depot) — no seat ever frees
        // up, at either shelter.
        let world = app.world_mut();
        let depot = world
            .query::<(Entity, &super::super::buildings::Building)>()
            .iter(world)
            .find(|(_, b)| b.kind == BuildingKind::Depot)
            .unwrap()
            .0;
        let phantoms: Vec<(Entity, Entity)> = (0..super::super::transit::BUS_CAPACITY)
            .map(|_| (world.spawn_empty().id(), depot))
            .collect();
        world
            .query::<&mut BusDuty>()
            .single_mut(world)
            .unwrap()
            .riders = phantoms;
        // Morning: commuters walk to the west shelter and queue, but the bus
        // is full — the queue persists (until the give-up threshold, which
        // this window stays below).
        ticks(&mut app, 320);
        let world = app.world_mut();
        let queued: usize = world
            .resource::<StopQueues>()
            .0
            .values()
            .map(|q| q.len())
            .sum();
        assert!(
            queued > 0,
            "a full bus must leave the overflow visibly waiting"
        );
        let waiting = world
            .query::<&CommuterPawn>()
            .iter(world)
            .filter(|p| matches!(p.phase, CommutePhase::Wait { .. }))
            .count();
        assert!(waiting > 0, "queued commuters stand at the shelter");
    }
}
