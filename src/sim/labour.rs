//! Labour planning pass (spec/citizens.md § Labour allocation, ticket #25):
//! workplaces declare vacancies; a planning pass binds housed, unemployed
//! citizens to a *fixed* workplace, subject to commute feasibility — a real
//! travel-time check over the compiled lane graph (the check CS1 lacks).
//! Tenure persists until the workplace is lost; cutting a road never severs
//! an existing assignment, it only stops *new* ones (and, in M2.4, stops the
//! commute itself).

use std::collections::{BinaryHeap, HashMap};

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::citizens::Citizen;
use super::roads::{RoadNode, RoadSegment};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};
use super::vehicles::nearest_node;

/// Commute effective speed, m/s, before the road-class modifier (walk/bus
/// stub; real passenger vehicles arrive with B3/B5).
pub const COMMUTE_SPEED: f32 = 8.0;
/// A workplace is commute-feasible from a home when the door-to-door travel
/// time over the lane graph stays within this budget (sim seconds).
pub const MAX_COMMUTE_SECS: f32 = 120.0;

impl BuildingKind {
    /// Tiered vacancies, single tier for M2 (professor slots arrive later).
    pub fn workers_needed(self) -> u32 {
        match self {
            BuildingKind::Mine => 6,
            BuildingKind::Quarry => 4,
            BuildingKind::PowerPlant => 8,
            BuildingKind::Factory => 10,
            BuildingKind::Dwelling => 0,
        }
    }
}

/// Workplace-side labour ledger. `assigned` is the fixed-binding roster the
/// planning pass fills; `present` counts workers physically at the building
/// this tick (written by the commute systems in M2.4, read by production).
#[derive(Component, Debug, Default)]
pub struct Staffing {
    pub assigned: Vec<Entity>,
    pub present: u32,
}

impl Staffing {
    pub fn vacancies(&self, kind: BuildingKind) -> u32 {
        kind.workers_needed()
            .saturating_sub(self.assigned.len() as u32)
    }
}

pub struct LabourSimPlugin;

impl Plugin for LabourSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimTick,
            attach_staffing
                .in_set(SimStage::ApplyCommands)
                .after(ApplyCommandsFlush),
        )
        .add_systems(
            SimTick,
            (sever_lost_workplaces, plan_labour)
                .chain()
                // Newly housed citizens become employable the same tick.
                .after(super::households::assign_housing)
                .in_set(SimStage::AllocationAndDispatch),
        );
    }
}

/// Newly placed workplaces get their labour ledger.
fn attach_staffing(mut commands: Commands, added: Query<(Entity, &Building), Added<Building>>) {
    for (entity, building) in &added {
        if building.kind.workers_needed() > 0 {
            commands.entity(entity).insert(Staffing::default());
        }
    }
}

/// Building loss is the only thing that breaks tenure: clear bindings whose
/// workplace no longer exists, and drop stale entries from rosters.
fn sever_lost_workplaces(
    mut citizens: Query<&mut Citizen>,
    mut staffing: Query<(Entity, &mut Staffing)>,
    buildings: Query<(), With<Building>>,
) {
    for mut citizen in &mut citizens {
        if let Some(work) = citizen.work
            && buildings.get(work).is_err()
        {
            citizen.work = None;
        }
    }
    for (workplace, mut ledger) in &mut staffing {
        ledger.assigned.retain(|&worker| {
            citizens
                .get(worker)
                .is_ok_and(|c| c.work == Some(workplace))
        });
    }
}

/// The planning pass: for every workplace with vacancies, build its
/// travel-time reach map once (Dijkstra from its dock node), then bind
/// unemployed housed citizens whose home docks inside the commute budget.
fn plan_labour(
    mut citizens: Query<(Entity, &mut Citizen)>,
    mut workplaces: Query<(Entity, &Building, &mut Staffing)>,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &RoadNode)>,
    segments: Query<&RoadSegment>,
) {
    // (citizen, home dwelling) pool of housed unemployed this tick.
    let mut unemployed: Vec<(Entity, Entity)> = Vec::new();
    for (entity, citizen) in citizens.iter() {
        if citizen.work.is_none()
            && let Some(home) = citizen.home
        {
            unemployed.push((entity, home));
        }
    }
    if unemployed.is_empty() {
        return;
    }
    // Homes dock onto the road like yards do; dock lookups are cached across
    // workplaces within the pass.
    let mut dock_cache: HashMap<Entity, Option<Entity>> = HashMap::new();
    for (workplace, building, mut ledger) in &mut workplaces {
        let mut open = ledger.vacancies(building.kind);
        if open == 0 {
            continue;
        }
        let Some(work_node) = nearest_node(building.pos, &nodes) else {
            continue;
        };
        let reach = travel_times(work_node, &nodes, &segments);
        unemployed.retain(|&(citizen_entity, home)| {
            if open == 0 {
                return true;
            }
            let dock = *dock_cache.entry(home).or_insert_with(|| {
                buildings
                    .get(home)
                    .ok()
                    .and_then(|b| nearest_node(b.pos, &nodes))
            });
            let feasible = dock
                .and_then(|node| reach.get(&node).copied())
                .is_some_and(|t| t <= MAX_COMMUTE_SECS);
            if !feasible {
                return true;
            }
            let Ok((_, mut citizen)) = citizens.get_mut(citizen_entity) else {
                return true;
            };
            citizen.work = Some(workplace);
            ledger.assigned.push(citizen_entity);
            open -= 1;
            false
        });
        if unemployed.is_empty() {
            return;
        }
    }
}

/// Single-source travel times over the lane graph, sim seconds.
fn travel_times(
    start: Entity,
    nodes: &Query<(Entity, &RoadNode)>,
    segments: &Query<&RoadSegment>,
) -> HashMap<Entity, f32> {
    let mut best: HashMap<Entity, f32> = HashMap::from([(start, 0.0)]);
    let mut heap = BinaryHeap::from([HeapEntry {
        time: 0.0,
        node: start,
    }]);
    while let Some(HeapEntry { time, node }) = heap.pop() {
        if time > best.get(&node).copied().unwrap_or(f32::INFINITY) {
            continue;
        }
        let Ok((_, n)) = nodes.get(node) else {
            continue;
        };
        for &seg in &n.segments {
            let Ok(segment) = segments.get(seg) else {
                continue;
            };
            let next = if segment.a == node {
                segment.b
            } else {
                segment.a
            };
            let t = time + segment.length / (COMMUTE_SPEED * segment.class.speed_modifier());
            if t < best.get(&next).copied().unwrap_or(f32::INFINITY) {
                best.insert(next, t);
                heap.push(HeapEntry {
                    time: t,
                    node: next,
                });
            }
        }
    }
    best
}

/// Min-heap entry ordered by ascending time.
struct HeapEntry {
    time: f32,
    node: Entity,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
impl Eq for HeapEntry {}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.total_cmp(&self.time)
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingSimPlugin};
    use super::super::households::{HouseholdSimPlugin, RecruitmentPlan};
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

    fn place(app: &mut App, kind: BuildingKind, pos: Vec3) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place { kind, pos });
    }

    fn road(app: &mut App, from: Vec3, to: Vec3) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from,
                to,
                class: RoadClass::Dirt,
            });
    }

    fn recruit(app: &mut App, households: u32) {
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = households;
    }

    fn employed_count(app: &mut App) -> usize {
        let world = app.world_mut();
        world
            .query::<&Citizen>()
            .iter(world)
            .filter(|c| c.work.is_some())
            .count()
    }

    /// Dwelling docked at x=0, mine docked at x=100, joined by a dirt road.
    fn commuter_town(app: &mut App) {
        road(app, Vec3::ZERO, Vec3::new(100.0, 0.0, 0.0));
        place(app, BuildingKind::Dwelling, Vec3::new(0.0, 0.0, 15.0));
        place(app, BuildingKind::Mine, Vec3::new(100.0, 0.0, 15.0));
    }

    #[test]
    fn housed_citizens_fill_vacancies_over_a_feasible_commute() {
        let mut app = app();
        commuter_town(&mut app);
        recruit(&mut app, 2); // sizes 3 + 2 = 5 citizens, mine has 6 slots
        ticks(&mut app, 5);
        assert_eq!(employed_count(&mut app), 5);
        let world = app.world_mut();
        let staffing = world.query::<&Staffing>().single(world).unwrap();
        assert_eq!(staffing.assigned.len(), 5);
        assert_eq!(staffing.present, 0, "presence arrives with the commute");
    }

    #[test]
    fn no_road_path_means_no_assignment() {
        let mut app = app();
        // Same layout, but the buildings sit far beyond DOCK_RADIUS of any road.
        road(&mut app, Vec3::ZERO, Vec3::new(100.0, 0.0, 0.0));
        place(&mut app, BuildingKind::Dwelling, Vec3::new(0.0, 0.0, 300.0));
        place(&mut app, BuildingKind::Mine, Vec3::new(100.0, 0.0, 300.0));
        recruit(&mut app, 2);
        ticks(&mut app, 5);
        assert_eq!(employed_count(&mut app), 0);
    }

    #[test]
    fn over_budget_commute_is_infeasible() {
        let mut app = app();
        // 2000 m of dirt road: 2000 / (8 × 0.55) ≈ 455 s > 120 s budget.
        road(&mut app, Vec3::ZERO, Vec3::new(2000.0, 0.0, 0.0));
        place(&mut app, BuildingKind::Dwelling, Vec3::new(0.0, 0.0, 15.0));
        place(&mut app, BuildingKind::Mine, Vec3::new(2000.0, 0.0, 15.0));
        recruit(&mut app, 1);
        ticks(&mut app, 5);
        assert_eq!(employed_count(&mut app), 0);
    }

    #[test]
    fn cutting_the_road_keeps_tenure_but_blocks_new_assignments() {
        let mut app = app();
        commuter_town(&mut app);
        recruit(&mut app, 2);
        ticks(&mut app, 5);
        assert_eq!(employed_count(&mut app), 5);
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::RemoveNear {
                pos: Vec3::new(50.0, 0.0, 0.0),
            });
        recruit(&mut app, 3); // one more household (size 4) after the cut
        ticks(&mut app, 5);
        // existing bindings persist; the new household stays unassigned
        assert_eq!(employed_count(&mut app), 5);
        let world = app.world_mut();
        let staffing = world.query::<&Staffing>().single(world).unwrap();
        assert_eq!(staffing.assigned.len(), 5);
    }
}
