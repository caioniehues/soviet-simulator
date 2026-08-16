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
//! game day and a workday commute fits the day comfortably. Freight runs on
//! its own compressed scale (see vehicles.rs); unifying them is a B5 question.

use std::collections::HashMap;

use bevy::prelude::*;

use super::buildings::Building;
use super::citizens::{Citizen, CitizenLocation};
use super::clock::{FRAMES_PER_GAME_DAY, FrameIndex};
use super::labour::{COMMUTE_SPEED, Staffing};
use super::roads::{LaneDir, RoadNode, RoadSegment};
use super::stages::{SimStage, SimTick};
use super::vehicles::{RouteLeg, find_route, nearest_node};

/// Morning departures begin at this frame of the day (06:00 of the 600-frame
/// day), evening departures at `EVENING_FRAME`. Each citizen adds a personal
/// jitter so departures stream instead of pulsing (CS1's probability curve,
/// made deterministic).
pub const MORNING_FRAME: u32 = 150;
pub const EVENING_FRAME: u32 = 450;
pub const DEPARTURE_JITTER_FRAMES: u32 = 60;

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
}

pub struct CommuteSimPlugin;

impl Plugin for CommuteSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SimTick, depart_commuters.in_set(SimStage::Routing))
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

/// Spawn pawns for citizens whose departure window is open. No route right
/// now (severed network, missing dock) means stay put and retry next frame —
/// the worker is visibly absent, never teleported.
fn depart_commuters(
    mut commands: Commands,
    frame: Res<FrameIndex>,
    mut citizens: Query<(Entity, &mut Citizen)>,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    let day = frame.0 % FRAMES_PER_GAME_DAY;
    for (entity, mut citizen) in &mut citizens {
        let (Some(home), Some(work)) = (citizen.home, citizen.work) else {
            continue;
        };
        let jitter = departure_jitter(&citizen);
        let (from, to, to_work) = match citizen.location {
            CitizenLocation::AtHome if day >= MORNING_FRAME + jitter && day < EVENING_FRAME => {
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
        let Some(route) = find_route(start, goal, &nodes, &segments) else {
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
        });
        citizen.location = if to_work {
            CitizenLocation::ToWork
        } else {
            CitizenLocation::ToHome
        };
    }
}

/// Walk the route one travel-second per frame. Arrival flips the citizen's
/// location and releases the pawn; a severed route aborts the trip — the
/// citizen ends up at home, absent from work.
fn advance_commuters(
    mut commands: Commands,
    mut pawns: Query<(Entity, &mut CommuterPawn)>,
    mut citizens: Query<&mut Citizen>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    for (pawn_entity, mut pawn) in &mut pawns {
        let mut arrived = false;
        let mut aborted = false;
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
                place_on_leg(&mut pawn, segment, leg.dir, &nodes);
                break;
            }
            budget -= remaining;
            pawn.s = segment.length;
            place_on_leg(&mut pawn, segment, leg.dir, &nodes);
            pawn.leg += 1;
            pawn.s = 0.0;
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

fn place_on_leg(
    pawn: &mut CommuterPawn,
    segment: &RoadSegment,
    dir: LaneDir,
    nodes: &Query<(Entity, &RoadNode)>,
) {
    let (Ok((_, a)), Ok((_, b))) = (nodes.get(segment.a), nodes.get(segment.b)) else {
        return;
    };
    let (start, end) = match dir {
        LaneDir::Forward => (a.pos, b.pos),
        LaneDir::Backward => (b.pos, a.pos),
    };
    let travel = (end - start).normalize_or_zero();
    // Pedestrians keep to the verge, outside the vehicle lane.
    let verge = segment.class.width() * 0.5 + 0.6;
    pawn.heading = travel;
    pawn.pos = start + travel * pawn.s.min(segment.length) + travel.cross(Vec3::Y) * verge;
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
}
