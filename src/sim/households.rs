//! Households stage 1 (spec/households.md, ticket #23): the unit at which
//! housing is allocated and goods are shared. A household entity owns its
//! member citizens; a dwelling building carries a flat table (`Dwelling`).
//! Spawning goes through `HouseholdSpawnQueue` — the plan-recruited
//! immigration source (M2.2 wires the recruitment lever; the queue is the
//! only way citizens enter the world).

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::citizens::{Citizen, CitizenIds};
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
struct HouseholdIds {
    next: u64,
}

pub struct HouseholdSimPlugin;

impl Plugin for HouseholdSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HouseholdSpawnQueue>()
            .init_resource::<HouseholdIds>()
            .init_resource::<CitizenIds>()
            .add_systems(
                SimTick,
                (attach_flat_tables, apply_household_spawns)
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            );
    }
}

/// Newly placed dwelling buildings get their flat table.
fn attach_flat_tables(mut commands: Commands, added: Query<(Entity, &Building), Added<Building>>) {
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
