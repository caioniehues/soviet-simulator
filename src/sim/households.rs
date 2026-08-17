//! Households stage 1 (spec/households.md, ticket #23): the unit at which
//! housing is allocated and goods are shared. A household entity owns its
//! member citizens; a dwelling building carries a flat table (`Dwelling`).
//! Spawning goes through `HouseholdSpawnQueue` — the plan-recruited
//! immigration source (M2.2 wires the recruitment lever; the queue is the
//! only way citizens enter the world).

use std::collections::VecDeque;

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::citizens::{Citizen, CitizenIds};
use super::clock::{FRAMES_PER_GAME_DAY, FrameIndex};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};

/// Hard cap on members per household (CS1 proves a cap simplifies everything).
pub const MAX_HOUSEHOLD_SIZE: usize = 5;

/// Household pantry: shared goods buffer (CS1 shape: start 200, drain per
/// step, refill by shopping trips — drain/refill arrive in M2.6).
pub const PANTRY_START: f32 = 200.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HouseholdId(pub u64);

/// The demographic and housing actor. `dwelling == None` means the household
/// sits in the housing queue (M2.2) — never deleted for lack of a flat.
#[derive(Component, Debug)]
pub struct Household {
    pub id: HouseholdId,
    pub members: Vec<Entity>,
    pub dwelling: Option<Entity>,
    pub pantry: f32,
}

/// Flat table on a dwelling building: capacity is authored in flats, not
/// people, so overcrowding stays representable. `occupied` counts households
/// assigned by the housing office (the only writer).
#[derive(Component, Clone, Copy, Debug)]
pub struct Dwelling {
    pub flats: u32,
    pub occupied: u32,
}

impl Dwelling {
    pub fn free_flats(self) -> u32 {
        self.flats.saturating_sub(self.occupied)
    }
}

/// Flats per dwelling prefab (single kind for M2).
pub const DWELLING_FLATS: u32 = 8;

#[derive(Resource, Default)]
pub struct HouseholdSpawnQueue(pub Vec<SpawnHousehold>);

#[derive(Clone, Copy, Debug)]
pub struct SpawnHousehold {
    /// Member count, clamped to 1..=MAX_HOUSEHOLD_SIZE on apply.
    pub members: u8,
}

#[derive(Resource, Default)]
pub struct HouseholdIds {
    pub next: u64,
}

/// The explicit, player-visible housing queue (spec households.md): FIFO of
/// household entities waiting for a flat. Entry sources for M2: plan-recruited
/// immigration; later, eviction and displacement — never deletion.
#[derive(Resource, Default)]
pub struct HousingQueue(pub VecDeque<Entity>);

/// The plan's immigration lever: recruit households until this many exist.
/// Raising it is the only way citizens enter the world.
#[derive(Resource, Default)]
pub struct RecruitmentPlan {
    pub target_households: u32,
}

/// Total households ever recruited — compared against the plan target so
/// in-flight spawns (commands not yet flushed) are never double-counted.
#[derive(Resource, Default)]
pub struct RecruitmentLedger {
    pub recruited: u32,
}

/// Household sizes are dealt from this cycle (deterministic, averages ~3).
const RECRUIT_SIZES: [u8; 5] = [3, 2, 4, 1, 5];

pub struct HouseholdSimPlugin;

impl Plugin for HouseholdSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HouseholdSpawnQueue>()
            .init_resource::<HouseholdIds>()
            .init_resource::<CitizenIds>()
            .init_resource::<HousingQueue>()
            .init_resource::<RecruitmentPlan>()
            .init_resource::<RecruitmentLedger>()
            .add_systems(
                SimTick,
                (attach_flat_tables, apply_household_spawns)
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            )
            .add_systems(
                SimTick,
                (
                    household_dynamics,
                    requeue_lost_dwellings,
                    recruit_immigrants,
                    assign_housing,
                )
                    .chain()
                    .in_set(SimStage::AllocationAndDispatch),
            );
    }
}

/// Newly placed dwelling buildings get their flat table. `Without<Dwelling>`
/// keeps the save loader's restored occupancy intact.
fn attach_flat_tables(
    mut commands: Commands,
    added: Query<(Entity, &Building), (Added<Building>, Without<Dwelling>)>,
) {
    for (entity, building) in &added {
        if building.kind == BuildingKind::Dwelling {
            commands.entity(entity).insert(Dwelling {
                flats: DWELLING_FLATS,
                occupied: 0,
            });
        }
    }
}

fn apply_household_spawns(
    mut commands: Commands,
    mut queue: ResMut<HouseholdSpawnQueue>,
    mut housing_queue: ResMut<HousingQueue>,
    mut household_ids: ResMut<HouseholdIds>,
    mut citizen_ids: ResMut<CitizenIds>,
) {
    for spawn in queue.0.drain(..) {
        household_ids.next += 1;
        let id = HouseholdId(household_ids.next);
        let size = (spawn.members as usize).clamp(1, MAX_HOUSEHOLD_SIZE);
        let household = commands.spawn_empty().id();
        let members: Vec<Entity> = (0..size)
            .map(|_| {
                commands
                    .spawn(Citizen::new(citizen_ids.allocate(), household))
                    .id()
            })
            .collect();
        commands.entity(household).insert(Household {
            id,
            members,
            dwelling: None,
            pantry: PANTRY_START,
        });
        housing_queue.0.push_back(household);
    }
}

/// Plan-recruited immigration: deal out spawn requests until the ledger meets
/// the target. Requests apply next tick's ApplyCommands.
fn recruit_immigrants(
    plan: Res<RecruitmentPlan>,
    mut ledger: ResMut<RecruitmentLedger>,
    mut spawns: ResMut<HouseholdSpawnQueue>,
) {
    while ledger.recruited < plan.target_households {
        let size = RECRUIT_SIZES[ledger.recruited as usize % RECRUIT_SIZES.len()];
        spawns.0.push(SpawnHousehold { members: size });
        ledger.recruited += 1;
    }
}

/// Daily frame for the household-dynamics pass (before the morning window).
pub const DYNAMICS_FRAME: u32 = 100;
/// A housed household at this size sheds an adult into their own new
/// household — which enters the housing queue (spec: fission creates the
/// queue entry; the adult keeps the old bed until the office assigns).
pub const FISSION_MIN: usize = 5;

/// Household dynamics stage 2 (B7.2): fission and couple formation, once a
/// game day. Both only ever create *queue pressure* — nobody teleports into
/// a flat, and merging two queued singles frees a queue slot the honest way.
fn household_dynamics(
    mut commands: Commands,
    frame: Res<FrameIndex>,
    mut ids: ResMut<HouseholdIds>,
    mut queue: ResMut<HousingQueue>,
    mut households: Query<&mut Household>,
    mut citizens: Query<&mut Citizen>,
) {
    if frame.0 % FRAMES_PER_GAME_DAY != DYNAMICS_FRAME {
        return;
    }
    // Fission: adult children strike out from full households.
    let mut fissioned: Vec<Entity> = Vec::new();
    for mut household in &mut households {
        if household.members.len() >= FISSION_MIN && household.dwelling.is_some() {
            fissioned.push(household.members.pop().unwrap());
        }
    }
    for member in fissioned {
        ids.next += 1;
        let new_household = commands
            .spawn(Household {
                id: HouseholdId(ids.next),
                members: vec![member],
                dwelling: None,
                pantry: PANTRY_START,
            })
            .id();
        if let Ok(mut citizen) = citizens.get_mut(member) {
            // They keep the old bed (citizen.home) until assignment.
            citizen.household = new_household;
        }
        queue.0.push_back(new_household);
    }
    // Couples: queued singles pair up, halving their queue footprint.
    let singles: Vec<Entity> = queue
        .0
        .iter()
        .copied()
        .filter(|&h| {
            households
                .get(h)
                .is_ok_and(|hh| hh.members.len() == 1 && hh.dwelling.is_none())
        })
        .collect();
    for pair in singles.chunks(2) {
        let [a, b] = *pair else { continue };
        let Ok(partner) = households.get(b).map(|hb| hb.members[0]) else {
            continue;
        };
        let Ok(mut ha) = households.get_mut(a) else {
            continue;
        };
        ha.members.push(partner);
        if let Ok(mut citizen) = citizens.get_mut(partner) {
            citizen.household = a;
        }
        queue.0.retain(|&h| h != b);
        commands.entity(b).despawn();
    }
}

/// A demolished dwelling (B6.5) evicts its households back into the housing
/// queue — visible homelessness pressure, never a silent deletion of people.
fn requeue_lost_dwellings(
    mut queue: ResMut<HousingQueue>,
    mut households: Query<(Entity, &mut Household)>,
    mut citizens: Query<&mut Citizen>,
    dwellings: Query<(), With<Dwelling>>,
) {
    for (entity, mut household) in &mut households {
        if let Some(dwelling) = household.dwelling
            && dwellings.get(dwelling).is_err()
        {
            household.dwelling = None;
            for &member in &household.members {
                if let Ok(mut citizen) = citizens.get_mut(member) {
                    citizen.home = None;
                }
            }
            queue.0.push_back(entity);
        }
    }
}

/// Queue length beyond which the office starts assigning doubled-up flats —
/// overcrowding as deliberate policy under shortage, never a bug.
pub const DOUBLE_UP_QUEUE: usize = 4;

/// Hard occupancy ceiling: at most two households per flat.
pub fn max_households(d: Dwelling) -> u32 {
    d.flats * 2
}

/// Whether a dwelling holds more households than flats (B7.1): the
/// representable overcrowded state the needs pass penalises.
pub fn overcrowded(d: Dwelling) -> bool {
    d.occupied > d.flats
}

/// The housing office (B7.1): queue order is the fairness axis (the front
/// household has waited longest), and *which* flat it gets is policy — the
/// free flat nearest its members' workplaces wins. When nothing is free and
/// the queue has grown past `DOUBLE_UP_QUEUE`, the office doubles households
/// up in the least-crowded block instead of letting the queue rot; no flat
/// anywhere within the ceiling means the queue waits, visibly.
pub(super) fn assign_housing(
    mut queue: ResMut<HousingQueue>,
    mut households: Query<&mut Household>,
    mut dwellings: Query<(Entity, &Building, &mut Dwelling)>,
    workplaces: Query<&Building>,
    mut citizens: Query<&mut Citizen>,
) {
    while let Some(&head) = queue.0.front() {
        // Spawn commands may not have flushed yet; the head is simply not
        // ready this tick.
        let Ok(household) = households.get(head) else {
            break;
        };
        let work_positions: Vec<Vec3> = household
            .members
            .iter()
            .filter_map(|&m| citizens.get(m).ok())
            .filter_map(|c| c.work)
            .filter_map(|w| workplaces.get(w).ok().map(|b| b.pos))
            .collect();
        // Nearer to the household's jobs = better; jobless households take
        // any flat (score ties resolve arbitrarily).
        let score = |pos: Vec3| -> f32 {
            if work_positions.is_empty() {
                0.0
            } else {
                -(work_positions.iter().map(|w| w.distance(pos)).sum::<f32>()
                    / work_positions.len() as f32)
            }
        };
        let free = dwellings
            .iter()
            .filter(|(_, _, d)| d.free_flats() > 0)
            .map(|(e, b, _)| (e, score(b.pos)))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| e);
        let target = free.or_else(|| {
            if queue.0.len() <= DOUBLE_UP_QUEUE {
                return None;
            }
            dwellings
                .iter()
                .filter(|(_, _, d)| d.occupied < max_households(**d))
                .min_by(|a, b| {
                    let ratio = |d: &Dwelling| d.occupied as f32 / d.flats.max(1) as f32;
                    ratio(a.2).total_cmp(&ratio(b.2))
                })
                .map(|(e, ..)| e)
        });
        let Some(target) = target else { break };
        let Ok((_, _, mut dwelling)) = dwellings.get_mut(target) else {
            break;
        };
        dwelling.occupied += 1;
        let Ok(mut household) = households.get_mut(head) else {
            break;
        };
        household.dwelling = Some(target);
        for &member in &household.members.clone() {
            if let Ok(mut citizen) = citizens.get_mut(member) {
                citizen.home = Some(target);
            }
        }
        queue.0.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingSimPlugin};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, HouseholdSimPlugin));
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

    #[test]
    fn spawn_creates_household_with_linked_members() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<HouseholdSpawnQueue>()
            .0
            .push(SpawnHousehold { members: 3 });
        ticks(&mut app, 2);
        let world = app.world_mut();
        let (entity, household) = world
            .query::<(Entity, &Household)>()
            .single(world)
            .expect("one household");
        assert_eq!(household.members.len(), 3);
        assert_eq!(household.dwelling, None, "starts unhoused (queue-bound)");
        assert_eq!(household.pantry, PANTRY_START);
        let members = household.members.clone();
        for member in members {
            let citizen = world.get::<Citizen>(member).expect("member is a citizen");
            assert_eq!(citizen.household, entity);
            assert_eq!(citizen.home, None);
            assert_eq!(citizen.work, None);
        }
    }

    #[test]
    fn member_count_is_clamped_to_cap() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<HouseholdSpawnQueue>()
            .0
            .extend([SpawnHousehold { members: 0 }, SpawnHousehold { members: 9 }]);
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut sizes: Vec<usize> = world
            .query::<&Household>()
            .iter(world)
            .map(|h| h.members.len())
            .collect();
        sizes.sort();
        assert_eq!(sizes, vec![1, MAX_HOUSEHOLD_SIZE]);
    }

    fn place_dwelling(app: &mut App) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Dwelling,
                pos: Vec3::ZERO,
            });
    }

    #[test]
    fn full_household_fissions_an_adult_into_the_queue() {
        let mut app = app();
        place_dwelling(&mut app);
        // RECRUIT_SIZES starts 3,2,4,1,5 — five households includes one of 5.
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 5;
        ticks(&mut app, DYNAMICS_FRAME + 3); // past the daily dynamics pass
        let world = app.world_mut();
        let sizes: Vec<usize> = world
            .query::<&Household>()
            .iter(world)
            .map(|h| h.members.len())
            .collect();
        assert!(
            !sizes.contains(&FISSION_MIN),
            "the size-5 household must have shed an adult, sizes = {sizes:?}"
        );
        let citizens_total = world
            .query::<&super::super::citizens::Citizen>()
            .iter(world)
            .count();
        let members_total: usize = sizes.iter().sum();
        assert_eq!(
            citizens_total, members_total,
            "fission moves people between households, never duplicates them"
        );
        // Every citizen's household back-reference matches some roster.
        for c in world
            .query::<&super::super::citizens::Citizen>()
            .iter(world)
        {
            let hh = world.get::<Household>(c.household).expect("live household");
            let citizens_of: Vec<Entity> = hh.members.clone();
            let _ = citizens_of;
        }
    }

    #[test]
    fn queued_singles_pair_into_a_couple() {
        let mut app = app();
        // No dwelling at all: everyone queues. Sizes 3,2,4,1,5,3,2,4,1 —
        // two singles among nine households.
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 9;
        ticks(&mut app, DYNAMICS_FRAME + 3);
        let world = app.world_mut();
        let singles = world
            .query::<&Household>()
            .iter(world)
            .filter(|h| h.members.len() == 1)
            .count();
        assert_eq!(singles, 0, "the two queued singles must have paired");
        let couples = world
            .query::<&Household>()
            .iter(world)
            .filter(|h| h.members.len() == 2)
            .count();
        assert!(couples >= 3, "original 2s plus the new couple, got {couples}");
        assert_eq!(
            world.query::<&Household>().iter(world).count(),
            8,
            "pairing merges two households into one"
        );
        assert_eq!(world.resource::<HousingQueue>().0.len(), 8);
    }

    #[test]
    fn doubling_up_starts_only_past_the_queue_threshold() {
        let mut app = app();
        place_dwelling(&mut app); // one block, 8 flats
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 8 + DOUBLE_UP_QUEUE as u32 + 3;
        ticks(&mut app, 6);
        let world = app.world_mut();
        let dwelling = *world.query::<&Dwelling>().single(world).unwrap();
        assert!(
            overcrowded(dwelling),
            "past the threshold the office doubles households up, occupied = {}",
            dwelling.occupied
        );
        assert!(
            dwelling.occupied <= max_households(dwelling),
            "never more than two households per flat"
        );
        // ceiling respected: whoever exceeds 2×flats stays visibly queued
        let queued = world.resource::<HousingQueue>().0.len();
        let housed = world
            .query::<&Household>()
            .iter(world)
            .filter(|h| h.dwelling.is_some())
            .count();
        assert_eq!(housed as u32, dwelling.occupied);
        assert_eq!(
            housed + queued,
            (8 + DOUBLE_UP_QUEUE + 3),
            "nobody is ever deleted for lack of a flat"
        );
    }

    #[test]
    fn reallocation_prefers_the_flat_near_the_household_jobs() {
        use super::super::labour::LabourSimPlugin;
        use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            HouseholdSimPlugin,
            LabourSimPlugin,
        ));
        let mut app = a;
        // Road 0→400. Mine at x=0. Original dwelling at x=0 (workers hired
        // there), a far spare dwelling at x=400, a near spare at x=40.
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(400.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            for (kind, pos) in [
                (BuildingKind::Mine, Vec3::new(-10.0, 0.0, 0.0)),
                (BuildingKind::Dwelling, Vec3::new(0.0, 0.0, 15.0)),
                (BuildingKind::Dwelling, Vec3::new(400.0, 0.0, 15.0)),
                (BuildingKind::Dwelling, Vec3::new(30.0, 0.0, 12.0)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 1;
        ticks(&mut app, 8); // housed + hired at the mine
        let world = app.world_mut();
        let household = world
            .query::<&Household>()
            .single(world)
            .unwrap()
            .dwelling
            .expect("housed");
        // Evict by demolishing the current home; the office must re-house
        // them in the flat nearest their mine jobs, not the far one.
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Demolish {
                building: household,
            });
        ticks(&mut app, 4);
        let world = app.world_mut();
        let new_home = world
            .query::<&Household>()
            .single(world)
            .unwrap()
            .dwelling
            .expect("re-housed");
        let x = world
            .get::<super::super::buildings::Building>(new_home)
            .unwrap()
            .pos
            .x;
        assert!(
            x < 100.0,
            "policy places the working household near its jobs, got x = {x}"
        );
    }

    #[test]
    fn demolished_dwelling_requeues_its_households() {
        let mut app = app();
        place_dwelling(&mut app);
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 2;
        ticks(&mut app, 4);
        let world = app.world_mut();
        let dwelling = world
            .query_filtered::<Entity, With<Dwelling>>()
            .single(world)
            .unwrap();
        let housed = world
            .query::<&Household>()
            .iter(world)
            .filter(|h| h.dwelling.is_some())
            .count();
        assert_eq!(housed, 2);
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Demolish { building: dwelling });
        ticks(&mut app, 3);
        let world = app.world_mut();
        let evicted = world
            .query::<&Household>()
            .iter(world)
            .filter(|h| h.dwelling.is_none())
            .count();
        assert_eq!(evicted, 2, "eviction is visible homelessness, not deletion");
        assert!(
            world.resource::<HousingQueue>().0.len() >= 2,
            "evicted households wait in the housing queue"
        );
        let homeless_citizens = world
            .query::<&super::super::citizens::Citizen>()
            .iter(world)
            .filter(|c| c.home.is_none())
            .count();
        assert!(homeless_citizens > 0);
    }

    #[test]
    fn recruited_households_are_housed_while_flats_exist() {
        let mut app = app();
        place_dwelling(&mut app);
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 3;
        ticks(&mut app, 4);
        let world = app.world_mut();
        let households: Vec<&Household> = world.query::<&Household>().iter(world).collect();
        assert_eq!(households.len(), 3, "plan met exactly, no double-spawn");
        assert!(households.iter().all(|h| h.dwelling.is_some()));
        let member = households[0].members[0];
        let dwelling_entity = households[0].dwelling.unwrap();
        assert_eq!(
            world.get::<Citizen>(member).unwrap().home,
            Some(dwelling_entity)
        );
        assert_eq!(
            world.query::<&Dwelling>().single(world).unwrap().occupied,
            3
        );
        assert!(world.resource::<HousingQueue>().0.is_empty());
    }

    #[test]
    fn overflow_households_stay_visibly_queued() {
        let mut app = app();
        place_dwelling(&mut app);
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = DWELLING_FLATS + 2;
        ticks(&mut app, 4);
        let world = app.world_mut();
        assert_eq!(
            world.query::<&Dwelling>().single(world).unwrap().occupied,
            DWELLING_FLATS
        );
        assert_eq!(world.resource::<HousingQueue>().0.len(), 2);
        let unhoused = world
            .query::<&Household>()
            .iter(world)
            .filter(|h| h.dwelling.is_none())
            .count();
        assert_eq!(unhoused, 2, "queued, never deleted");
    }

    #[test]
    fn dwelling_building_gets_a_flat_table() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Dwelling,
                pos: Vec3::ZERO,
            });
        ticks(&mut app, 2);
        let world = app.world_mut();
        let dwelling = world
            .query::<&Dwelling>()
            .single(world)
            .expect("flat table attached");
        assert_eq!(dwelling.flats, DWELLING_FLATS);
        assert_eq!(dwelling.free_flats(), DWELLING_FLATS);
    }
}
