//! Storage policy substrate (spec/logistics.md, ticket #31): the player-set
//! intent layer of the dispatcher. Every stocked building can carry per-resource
//! **bands** against its shared yard capacity — stock below the min band is a
//! demand, stock above the max band is a supply (bucket-driven pull, W&R's
//! confirmed shape: the solver reads buckets, never offer objects). The
//! matching pass that turns bands into freight orders arrives in M3.3.

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::households::RecruitmentPlan;
use super::resources::{Inventory, ResourceKind};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};

/// Per-resource intent band, as fractions of the building's shared yard
/// capacity. `min` below `max`; a resource with no band is inert (never
/// demanded, never offered).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StorageBand {
    pub min_pct: f32,
    pub max_pct: f32,
}

impl StorageBand {
    pub fn new(min_pct: f32, max_pct: f32) -> Self {
        let min_pct = min_pct.clamp(0.0, 1.0);
        Self {
            min_pct,
            max_pct: max_pct.clamp(min_pct, 1.0),
        }
    }
}

/// The building's policy table (player-set on warehouses, recipe-derived
/// defaults elsewhere). Dense per-resource array — reads sit on the
/// dispatcher's hot scan path.
#[derive(Component, Clone, Debug, Default)]
pub struct StoragePolicies {
    bands: [Option<StorageBand>; ResourceKind::COUNT],
}

impl StoragePolicies {
    pub fn with(mut self, resource: ResourceKind, min_pct: f32, max_pct: f32) -> Self {
        self.set(resource, Some(StorageBand::new(min_pct, max_pct)));
        self
    }

    pub fn band(&self, resource: ResourceKind) -> Option<StorageBand> {
        self.bands[resource as usize]
    }

    pub fn set(&mut self, resource: ResourceKind, band: Option<StorageBand>) {
        self.bands[resource as usize] = band;
    }

    /// Tonnes short of the min band: the demand this bucket posts.
    pub fn deficit(&self, resource: ResourceKind, inventory: &Inventory) -> f32 {
        self.band(resource).map_or(0.0, |band| {
            (band.min_pct * inventory.capacity - inventory.amount(resource)).max(0.0)
        })
    }

    /// Tonnes above the max band: the supply this bucket offers.
    pub fn surplus(&self, resource: ResourceKind, inventory: &Inventory) -> f32 {
        self.band(resource).map_or(0.0, |band| {
            (inventory.amount(resource) - band.max_pct * inventory.capacity).max(0.0)
        })
    }
}

/// Recipe-derived default policies: producers hold back a working reserve and
/// supply the rest; consumers demand up to a working band. Warehouses get a
/// neutral all-resource band (min sum fits the shared capacity) the player
/// tunes per store. Dwellings demand goods for pantry pickup. A kind that
/// stores no cargo has nothing to band and lists none.
///
/// The bands themselves are the kind's catalogue row, held there as raw
/// (resource, min, max) triples because `StoragePolicies::with` is not `const`
/// — folding them into the real type is this function's whole job.
pub fn default_policies(kind: BuildingKind) -> StoragePolicies {
    super::catalogue::spec(kind).default_policies.iter().fold(
        StoragePolicies::default(),
        |policies, &(resource, min_pct, max_pct)| policies.with(resource, min_pct, max_pct),
    )
}

/// Presentation's queue onto the two values ADR 0003 names as policy: a
/// player-set value a sim system reads and the save persists, so it is
/// queued and barrier-applied like any other edit rather than written
/// straight into the component or resource from a HUD system.
#[derive(Resource, Default)]
pub struct PolicyEditQueue(pub Vec<PolicyEdit>);

#[derive(Clone, Copy, Debug)]
pub enum PolicyEdit {
    /// Absolute band replacement — the HUD reads the current band and
    /// computes the shifted min/max before pushing, so the applier never
    /// needs to see the step direction.
    SetBand {
        building: Entity,
        resource: ResourceKind,
        min_pct: f32,
        max_pct: f32,
    },
    /// Adjust the recruitment target by `delta`, saturating at 0.
    AdjustRecruitmentTarget { delta: i32 },
}

pub struct StorageSimPlugin;

impl Plugin for StorageSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PolicyEditQueue>()
            .add_observer(attach_default_policies)
            .add_systems(
                SimTick,
                apply_policy_edits
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            );
    }
}

/// `RecruitmentPlan` is optional: benches and tests that add
/// `StorageSimPlugin` without `HouseholdSimPlugin` (most of them —
/// storage predates households in the plugin group) never push a
/// recruitment edit, and the queue must not force that dependency on them.
fn apply_policy_edits(
    mut queue: ResMut<PolicyEditQueue>,
    mut policies: Query<&mut StoragePolicies>,
    mut plan: Option<ResMut<RecruitmentPlan>>,
) {
    for edit in queue.0.drain(..) {
        match edit {
            PolicyEdit::SetBand {
                building,
                resource,
                min_pct,
                max_pct,
            } => {
                if let Ok(mut policies) = policies.get_mut(building) {
                    policies.set(resource, Some(StorageBand::new(min_pct, max_pct)));
                }
            }
            PolicyEdit::AdjustRecruitmentTarget { delta } => {
                if let Some(plan) = plan.as_mut() {
                    plan.target_households = plan.target_households.saturating_add_signed(delta);
                }
            }
        }
    }
}

/// Newly placed buildings get their derived defaults in the same command
/// flush as the spawn. The `Has` guard keeps loader- or player-set policies
/// intact (same pattern as `Staffing`/`Dwelling` attach).
fn attach_default_policies(
    add: On<Add, Building>,
    buildings: Query<(&Building, Has<StoragePolicies>)>,
    mut commands: Commands,
) {
    if let Ok((building, has_policies)) = buildings.get(add.entity)
        && !has_policies
    {
        commands
            .entity(add.entity)
            .insert(default_policies(building.kind));
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
        a.add_plugins((SimPlugin, BuildingSimPlugin, StorageSimPlugin));
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

    fn place(app: &mut App, kind: BuildingKind) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind,
                pos: Vec3::ZERO,
            });
    }

    #[test]
    fn band_arithmetic_splits_demand_and_supply() {
        let mut inventory = Inventory::new(100.0);
        let policies = StoragePolicies::default().with(ResourceKind::Coal, 0.4, 0.8);
        // Empty: full demand to the min line, no supply.
        assert_eq!(policies.deficit(ResourceKind::Coal, &inventory), 40.0);
        assert_eq!(policies.surplus(ResourceKind::Coal, &inventory), 0.0);
        // Inside the band: inert.
        inventory.add(ResourceKind::Coal, 60.0);
        assert_eq!(policies.deficit(ResourceKind::Coal, &inventory), 0.0);
        assert_eq!(policies.surplus(ResourceKind::Coal, &inventory), 0.0);
        // Overfull: the excess above max is supply.
        inventory.add(ResourceKind::Coal, 35.0);
        assert_eq!(policies.deficit(ResourceKind::Coal, &inventory), 0.0);
        assert_eq!(policies.surplus(ResourceKind::Coal, &inventory), 15.0);
        // A resource with no band is inert both ways.
        assert_eq!(policies.deficit(ResourceKind::Goods, &inventory), 0.0);
        assert_eq!(policies.surplus(ResourceKind::Goods, &inventory), 0.0);
    }

    #[test]
    fn band_bounds_are_clamped_and_ordered() {
        let band = StorageBand::new(1.4, 0.2);
        assert_eq!(band.min_pct, 1.0);
        assert_eq!(band.max_pct, 1.0);
        let band = StorageBand::new(-0.5, 0.7);
        assert_eq!(band.min_pct, 0.0);
        assert_eq!(band.max_pct, 0.7);
    }

    #[test]
    fn placed_buildings_get_recipe_derived_defaults() {
        let mut app = app();
        place(&mut app, BuildingKind::PowerPlant);
        place(&mut app, BuildingKind::Mine);
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut q = world.query::<(&Building, &StoragePolicies)>();
        let mut seen = 0;
        for (building, policies) in q.iter(world) {
            match building.kind {
                BuildingKind::PowerPlant => {
                    let band = policies
                        .band(ResourceKind::Coal)
                        .expect("plant demands coal");
                    assert_eq!(band.min_pct, 0.6);
                    assert_eq!(policies.band(ResourceKind::Goods), None);
                }
                BuildingKind::Mine => {
                    let band = policies
                        .band(ResourceKind::Coal)
                        .expect("mine supplies coal");
                    assert_eq!(band.max_pct, 0.05);
                }
                _ => unreachable!(),
            }
            seen += 1;
        }
        assert_eq!(seen, 2);
    }

    #[test]
    fn warehouse_places_with_a_neutral_all_resource_band() {
        let mut app = app();
        place(&mut app, BuildingKind::Warehouse);
        ticks(&mut app, 2);
        let world = app.world_mut();
        let (building, inventory, policies) = world
            .query::<(&Building, &Inventory, &StoragePolicies)>()
            .single(world)
            .expect("warehouse placed with inventory and policies");
        assert_eq!(building.kind, BuildingKind::Warehouse);
        assert_eq!(
            inventory.capacity,
            BuildingKind::Warehouse.inventory_capacity()
        );
        for resource in ResourceKind::ALL {
            let band = policies.band(resource).expect("every resource banded");
            assert_eq!((band.min_pct, band.max_pct), (0.2, 0.6));
        }
        // Empty warehouse demands each resource up to its min line.
        assert_eq!(
            policies.deficit(ResourceKind::Coal, inventory),
            0.2 * inventory.capacity
        );
    }

    /// A queue-only app: `RecruitmentPlan` deliberately absent, matching
    /// every existing `StorageSimPlugin` test app in `construction.rs`,
    /// `customs.rs`, `dispatch.rs`, `vehicles.rs`, `save.rs` — the applier
    /// must not force that dependency onto them.
    #[test]
    fn set_band_applies_at_the_next_barrier_not_immediately() {
        let mut app = app();
        place(&mut app, BuildingKind::Warehouse);
        ticks(&mut app, 2);
        let world = app.world_mut();
        let building = world
            .query::<(Entity, &Building)>()
            .single(world)
            .unwrap()
            .0;

        world
            .resource_mut::<PolicyEditQueue>()
            .0
            .push(PolicyEdit::SetBand {
                building,
                resource: ResourceKind::Coal,
                min_pct: 0.9,
                max_pct: 0.95,
            });
        // Not yet applied: same tick the edit was queued, before the barrier.
        let band = world
            .get::<StoragePolicies>(building)
            .unwrap()
            .band(ResourceKind::Coal);
        assert_eq!(band, Some(StorageBand::new(0.2, 0.6))); // warehouse default, unchanged

        ticks(&mut app, 1);
        let band = app
            .world()
            .get::<StoragePolicies>(building)
            .unwrap()
            .band(ResourceKind::Coal)
            .unwrap();
        assert_eq!((band.min_pct, band.max_pct), (0.9, 0.95));
    }

    #[test]
    fn recruitment_target_delta_applies_through_the_queue() {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            BuildingSimPlugin,
            StorageSimPlugin,
            super::super::households::HouseholdSimPlugin,
            super::super::plan::PlanSimPlugin,
        ));
        a.world_mut()
            .resource_mut::<PolicyEditQueue>()
            .0
            .push(PolicyEdit::AdjustRecruitmentTarget { delta: 3 });
        ticks(&mut a, 1);
        assert_eq!(a.world().resource::<RecruitmentPlan>().target_households, 3);
        a.world_mut()
            .resource_mut::<PolicyEditQueue>()
            .0
            .push(PolicyEdit::AdjustRecruitmentTarget { delta: -5 });
        ticks(&mut a, 1);
        // Saturates at 0 rather than underflowing, same as the old direct write.
        assert_eq!(a.world().resource::<RecruitmentPlan>().target_households, 0);
    }

    /// `snapshot`/`restore` walk the full save schema (dispatch, roads,
    /// wires, vehicles, the Plan, recruitment) regardless of which table
    /// changed, so this needs the same full plugin set `save.rs`'s own
    /// tests use — not the minimal `app()` above.
    fn full_app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            super::super::roads::RoadSimPlugin,
            BuildingSimPlugin,
            super::super::plan::PlanSimPlugin,
            super::super::households::HouseholdSimPlugin,
            super::super::labour::LabourSimPlugin,
            super::super::commute::CommuteSimPlugin,
            super::super::needs::NeedsSimPlugin,
            StorageSimPlugin,
            super::super::vehicles::VehicleSimPlugin,
            super::super::dispatch::DispatchSimPlugin,
            super::super::wires::WireSimPlugin,
        ));
        a
    }

    #[test]
    fn queued_band_survives_a_save_round_trip() {
        use super::super::save::{restore, snapshot};

        let mut app = full_app();
        place(&mut app, BuildingKind::Warehouse);
        ticks(&mut app, 2);
        let world = app.world_mut();
        let building = world
            .query::<(Entity, &Building)>()
            .single(world)
            .unwrap()
            .0;
        world
            .resource_mut::<PolicyEditQueue>()
            .0
            .push(PolicyEdit::SetBand {
                building,
                resource: ResourceKind::Coal,
                min_pct: 0.7,
                max_pct: 0.9,
            });
        ticks(&mut app, 1);

        let save = snapshot(app.world_mut());
        let mut loaded = full_app();
        restore(loaded.world_mut(), &save);

        let world = loaded.world_mut();
        let policies = world
            .query::<&StoragePolicies>()
            .single(world)
            .expect("warehouse restored");
        let band = policies.band(ResourceKind::Coal).unwrap();
        assert_eq!((band.min_pct, band.max_pct), (0.7, 0.9));
    }

    #[test]
    fn queued_recruitment_target_survives_a_save_round_trip() {
        use super::super::save::{restore, snapshot};

        let mut app = full_app();
        app.world_mut()
            .resource_mut::<PolicyEditQueue>()
            .0
            .push(PolicyEdit::AdjustRecruitmentTarget { delta: 4 });
        ticks(&mut app, 1);
        assert_eq!(
            app.world().resource::<RecruitmentPlan>().target_households,
            4
        );

        let save = snapshot(app.world_mut());
        let mut loaded = full_app();
        restore(loaded.world_mut(), &save);
        assert_eq!(
            loaded
                .world()
                .resource::<RecruitmentPlan>()
                .target_households,
            4
        );
    }
}
