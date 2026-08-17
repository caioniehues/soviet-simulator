//! Citizen needs stage 1 (spec/needs.md, ticket #28): food and rest only —
//! per-citizen 0–1 satisfactions with low-frequency decay. Food refills by
//! eating from the household pantry (drain is real; the pantry itself refills
//! by fiat until B3 puts shops in the chain). Rest drains away from home and
//! recovers at home. Wellbeing is recomputed from the two satisfactions and
//! feeds work probability — the CS1 consequence coupling (`GetWorkProbability`)
//! that gives needs teeth: a miserable citizen skips workdays, staffing falls,
//! production falls.

use bevy::prelude::*;

use super::citizens::{Citizen, CitizenId, CitizenLocation};
use super::clock::{FRAMES_PER_GAME_DAY, FrameIndex};
use super::households::{Household, PANTRY_START};
use super::stages::{SimStage, SimTick};

/// Needs update cadence, sim frames (24 updates per 600-frame game day —
/// the "low frequency" band of architecture/simulation-clock.md).
pub const NEEDS_INTERVAL_FRAMES: u32 = 25;
/// Food satisfaction lost per needs update (~0.36/day uneaten).
pub const FOOD_DECAY: f32 = 0.015;
/// Rest lost per update while away from home (a 12-update workday ≈ 0.48).
pub const REST_DRAIN: f32 = 0.04;
/// Rest regained per update at home (a night at home restores the day).
pub const REST_RECOVERY: f32 = 0.06;
/// Below this satisfaction a citizen at home eats a meal.
pub const EAT_THRESHOLD: f32 = 0.6;
/// Pantry units one meal drains (CS1 shape: fixed cost per consumption step).
pub const MEAL_PANTRY_COST: f32 = 1.0;
/// Citizens attend fully at or above this wellbeing; below it the daily work
/// probability scales down linearly.
pub const ATTENDANCE_FULL_WELLBEING: f32 = 60.0;
/// Someone always shows up — the floor on daily work probability.
pub const ATTENDANCE_FLOOR: f32 = 0.15;

/// Per-citizen need satisfactions, 0..=1. Attached by observer the moment a
/// `Citizen` spawns, so no citizen ever ticks needless.
#[derive(Component, Clone, Copy, Debug)]
pub struct CitizenNeeds {
    pub food: f32,
    pub rest: f32,
    /// Today's attendance roll, decided once at the morning boundary from
    /// that moment's wellbeing — sticky for the day, so a citizen who calls
    /// in absent stays absent even as resting at home lifts wellbeing back.
    pub attends: bool,
}

impl Default for CitizenNeeds {
    fn default() -> Self {
        Self {
            food: 1.0,
            rest: 1.0,
            attends: true,
        }
    }
}

pub struct NeedsSimPlugin;

impl Plugin for NeedsSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attach_needs).add_systems(
            SimTick,
            update_needs.in_set(SimStage::NeedsAndServiceDemand),
        );
    }
}

/// Same pattern as `labour::attach_staffing`: needs land in the spawn flush.
/// The `has_needs` guard keeps the save loader's restored values intact.
fn attach_needs(
    add: On<Add, Citizen>,
    citizens: Query<Has<CitizenNeeds>, With<Citizen>>,
    mut commands: Commands,
) {
    if citizens.get(add.entity) == Ok(false) {
        commands.entity(add.entity).insert(CitizenNeeds::default());
    }
}

/// Daily work probability from wellbeing (CS1's coupling, our curve): full
/// attendance at comfortable wellbeing, linear decline below, floored.
pub fn work_probability(wellbeing: u8) -> f32 {
    (wellbeing as f32 / ATTENDANCE_FULL_WELLBEING).clamp(ATTENDANCE_FLOOR, 1.0)
}

/// Deterministic per-(citizen, day) attendance draw — a hash, not an RNG, so
/// replays and save/load round-trips agree (splitmix64 finalizer).
pub fn attends_today(id: CitizenId, day: u32, probability: f32) -> bool {
    let mut x = id.0 ^ (day as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    let fraction = (x >> 11) as f64 / (1u64 << 53) as f64;
    fraction < probability as f64
}

/// The low-frequency needs pass: decay, meals, rest, wellbeing — one walk
/// over the citizen table every `NEEDS_INTERVAL_FRAMES`.
/// Rest ceiling in a doubled-up flat (B7.1): overcrowding is livable but it
/// wears — a shared kitchen and a corridor bed never fully restore anyone.
pub const OVERCROWDED_REST_CAP: f32 = 0.6;
/// Rest ceiling in an unpowered home (B8.1): no light, no stove — a dark
/// flat rests worse than a crowded lit one is the wrong ordering, so this
/// cap sits above the overcrowding one; both can apply (min wins).
pub const DARK_HOME_REST_CAP: f32 = 0.75;
/// Rest ceiling in an unheated home in the cold months (B8.3): a freezing
/// flat is worse than a dark one — you can sleep without light, not without
/// warmth — so this cap sits below the dark-home one; all caps min-stack.
pub const COLD_HOME_REST_CAP: f32 = 0.65;

fn update_needs(
    frame: Res<FrameIndex>,
    mut citizens: Query<(&mut Citizen, &mut CitizenNeeds)>,
    mut households: Query<&mut Household>,
    dwellings: Query<(
        &super::households::Dwelling,
        Option<&super::buildings::Powered>,
        Option<&super::heat::Heated>,
    )>,
) {
    if !frame.0.is_multiple_of(NEEDS_INTERVAL_FRAMES) {
        return;
    }
    // The morning boundary is an interval frame (150 % 25 == 0), so the
    // attendance roll rides the same pass, right after the day's first
    // at-home recovery lands.
    let roll_attendance = frame.0 % FRAMES_PER_GAME_DAY == super::commute::MORNING_FRAME;
    for (mut citizen, mut needs) in &mut citizens {
        needs.food = (needs.food - FOOD_DECAY).max(0.0);
        if citizen.location == CitizenLocation::AtHome {
            let mut rest_cap: f32 = 1.0;
            if let Some((dwelling, powered, heated)) =
                citizen.home.and_then(|d| dwellings.get(d).ok())
            {
                if super::households::overcrowded(*dwelling) {
                    rest_cap = rest_cap.min(OVERCROWDED_REST_CAP);
                }
                if powered.is_some_and(|p| !p.0) {
                    rest_cap = rest_cap.min(DARK_HOME_REST_CAP);
                }
                if heated.is_some_and(|h| !h.0) {
                    rest_cap = rest_cap.min(COLD_HOME_REST_CAP);
                }
            }
            needs.rest = (needs.rest + REST_RECOVERY).min(rest_cap.max(needs.rest));
        } else {
            needs.rest = (needs.rest - REST_DRAIN).max(0.0);
        }
        // Eating happens at home, from the shared pantry. The pantry refills
        // by fiat when it runs dry — the B3 shop trip replaces this line.
        if needs.food < EAT_THRESHOLD
            && citizen.location == CitizenLocation::AtHome
            && let Ok(mut household) = households.get_mut(citizen.household)
        {
            if household.pantry < MEAL_PANTRY_COST {
                household.pantry = PANTRY_START;
            }
            household.pantry -= MEAL_PANTRY_COST;
            needs.food = 1.0;
        }
        let wellbeing = 100.0 * (0.55 * needs.food + 0.45 * needs.rest);
        citizen.wellbeing = wellbeing.round().clamp(0.0, 100.0) as u8;
        if roll_attendance {
            needs.attends = attends_today(
                citizen.id,
                day_index(frame.0),
                work_probability(citizen.wellbeing),
            );
        }
    }
}

/// Convenience for HUD/commute callers: the day index the attendance draw is
/// keyed on.
pub fn day_index(frame: u32) -> u32 {
    frame / FRAMES_PER_GAME_DAY
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin,
    };
    use super::super::commute::CommuteSimPlugin;
    use super::super::households::{HouseholdSimPlugin, RecruitmentPlan};
    use super::super::labour::{LabourSimPlugin, Staffing};
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
            NeedsSimPlugin,
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

    fn spawn_household(app: &mut App, members: u8) {
        use super::super::households::{HouseholdSpawnQueue, SpawnHousehold};
        app.world_mut()
            .resource_mut::<HouseholdSpawnQueue>()
            .0
            .push(SpawnHousehold { members });
    }

    #[test]
    fn needs_attach_on_spawn_and_decay_low_frequency() {
        let mut app = app();
        spawn_household(&mut app, 1);
        ticks(&mut app, 2);
        {
            let world = app.world_mut();
            let needs = world.query::<&CitizenNeeds>().single(world).unwrap();
            assert_eq!(needs.food, 1.0);
        }
        // 3 updates elapse (frames 25, 50, 75); rest stays capped at home.
        ticks(&mut app, 80);
        let world = app.world_mut();
        let needs = world.query::<&CitizenNeeds>().single(world).unwrap();
        assert!((needs.food - (1.0 - 3.0 * FOOD_DECAY)).abs() < 1e-5);
        assert_eq!(needs.rest, 1.0);
    }

    #[test]
    fn meals_drain_the_pantry_and_refill_food() {
        let mut app = app();
        spawn_household(&mut app, 1);
        ticks(&mut app, 2);
        {
            let world = app.world_mut();
            let mut q = world.query::<&mut CitizenNeeds>();
            q.single_mut(world).unwrap().food = 0.2; // hungry, at home
        }
        ticks(&mut app, NEEDS_INTERVAL_FRAMES);
        let world = app.world_mut();
        let needs = world.query::<&CitizenNeeds>().single(world).unwrap();
        assert_eq!(needs.food, 1.0, "meal refills satisfaction");
        let household = world.query::<&Household>().single(world).unwrap();
        assert_eq!(household.pantry, PANTRY_START - MEAL_PANTRY_COST);
    }

    #[test]
    fn empty_pantry_refills_by_fiat_until_shops_exist() {
        let mut app = app();
        spawn_household(&mut app, 1);
        ticks(&mut app, 2);
        {
            let world = app.world_mut();
            let mut q = world.query::<&mut Household>();
            q.single_mut(world).unwrap().pantry = 0.2;
            let mut n = world.query::<&mut CitizenNeeds>();
            n.single_mut(world).unwrap().food = 0.1;
        }
        ticks(&mut app, NEEDS_INTERVAL_FRAMES);
        let world = app.world_mut();
        let household = world.query::<&Household>().single(world).unwrap();
        assert_eq!(household.pantry, PANTRY_START - MEAL_PANTRY_COST);
    }

    #[test]
    fn wellbeing_is_recomputed_from_needs() {
        let mut app = app();
        spawn_household(&mut app, 1);
        ticks(&mut app, 2);
        {
            let world = app.world_mut();
            let mut q = world.query::<&mut CitizenNeeds>();
            let mut needs = q.single_mut(world).unwrap();
            needs.food = 0.8; // above EAT_THRESHOLD: no meal interferes
            needs.rest = 0.8;
        }
        ticks(&mut app, NEEDS_INTERVAL_FRAMES);
        let world = app.world_mut();
        // decay applies first: food 0.785, rest capped up to 0.86
        let (citizen, needs) = world
            .query::<(&Citizen, &CitizenNeeds)>()
            .single(world)
            .unwrap();
        let expected = (100.0 * (0.55 * needs.food + 0.45 * needs.rest)).round() as u8;
        assert_eq!(citizen.wellbeing, expected);
        assert!(citizen.wellbeing < 100);
    }

    /// Dwelling at x=0, mine at x=100, dirt road, two households (5 citizens).
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
            .target_households = 2;
    }

    #[test]
    fn low_wellbeing_cuts_attendance() {
        let mut app = app();
        commuter_town(&mut app);
        ticks(&mut app, 140); // assigned, still before the morning roll
        // Exhausted citizens: food just above the meal threshold (so no meal
        // masks it), rest gone. The frame-150 roll sees wellbeing ≈ 40
        // → work probability ≈ 0.67.
        {
            let world = app.world_mut();
            let mut q = world.query::<&mut CitizenNeeds>();
            for mut needs in q.iter_mut(world) {
                needs.food = 0.7;
                needs.rest = 0.0;
            }
        }
        // frame 300: mid-morning of day 0, all commutes complete
        ticks(&mut app, 160);
        let world = app.world_mut();
        let present = world.query::<&Staffing>().single(world).unwrap().present;
        assert!(
            present < 5,
            "wellbeing ≈ 40 must keep some of the 5 workers home, present = {present}"
        );
    }

    #[test]
    fn full_wellbeing_keeps_full_attendance() {
        let mut app = app();
        commuter_town(&mut app);
        ticks(&mut app, 300);
        let world = app.world_mut();
        let present = world.query::<&Staffing>().single(world).unwrap().present;
        assert_eq!(present, 5, "satisfied citizens all attend");
    }

    #[test]
    fn attendance_draw_is_deterministic_and_tracks_probability() {
        let id = CitizenId(42);
        assert_eq!(attends_today(id, 3, 0.5), attends_today(id, 3, 0.5));
        assert!(attends_today(id, 3, 1.0));
        assert!(!attends_today(id, 3, 0.0));
        // over many days the empirical rate approaches the probability
        let hits = (0..1000).filter(|&day| attends_today(id, day, 0.3)).count();
        assert!((250..=350).contains(&hits), "hits = {hits}");
    }

    #[test]
    fn overcrowded_flat_caps_rest_recovery() {
        use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingKind};
        use super::super::households::{Dwelling, RecruitmentPlan};
        let mut app = app();
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Dwelling,
                pos: Vec3::ZERO,
            });
        app.world_mut()
            .resource_mut::<RecruitmentPlan>()
            .target_households = 1;
        ticks(&mut app, 4);
        // Drain rest, then mark the block overcrowded by hand.
        {
            let world = app.world_mut();
            let mut q = world.query::<&mut CitizenNeeds>();
            for mut n in q.iter_mut(world) {
                n.rest = 0.3;
            }
            let mut d = world.query::<&mut Dwelling>();
            let mut dwelling = d.single_mut(world).unwrap();
            dwelling.occupied = dwelling.flats + 3;
        }
        ticks(&mut app, 200); // many at-home recovery passes
        let world = app.world_mut();
        for needs in world.query::<&CitizenNeeds>().iter(world) {
            assert!(
                needs.rest <= OVERCROWDED_REST_CAP + 1e-3,
                "a corridor bed never fully restores anyone, rest = {}",
                needs.rest
            );
            assert!(needs.rest > 0.3, "recovery still happens, just capped");
        }
    }
}
