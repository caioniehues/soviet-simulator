//! Storage policy substrate (spec/logistics.md, ticket #31): the player-set
//! intent layer of the dispatcher. Every stocked building can carry per-resource
//! **bands** against its shared yard capacity — stock below the min band is a
//! demand, stock above the max band is a supply (bucket-driven pull, W&R's
//! confirmed shape: the solver reads buckets, never offer objects). The
//! matching pass that turns bands into freight orders arrives in M3.3.

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::resources::{Inventory, ResourceKind};

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
/// tunes per store. Dwellings demand goods for pantry pickup.
pub fn default_policies(kind: BuildingKind) -> StoragePolicies {
    let p = StoragePolicies::default();
    match kind {
        BuildingKind::Mine => p.with(ResourceKind::Coal, 0.0, 0.05),
        BuildingKind::Quarry => p.with(ResourceKind::Gravel, 0.0, 0.05),
        BuildingKind::PowerPlant => p.with(ResourceKind::Coal, 0.6, 1.0),
        BuildingKind::Factory => p.with(ResourceKind::Goods, 0.0, 0.1),
        BuildingKind::Dwelling => p.with(ResourceKind::Goods, 0.5, 1.0),
        BuildingKind::Warehouse => ResourceKind::ALL
            .into_iter()
            .fold(p, |acc, r| acc.with(r, 0.2, 0.6)),
        // Store no cargo — nothing to band.
        BuildingKind::Depot
        | BuildingKind::BusStop
        | BuildingKind::ConstructionOffice
        | BuildingKind::WaterPump
        | BuildingKind::SewagePlant => p,
        BuildingKind::HeatPlant => p.with(ResourceKind::Coal, 0.6, 1.0),
        // No bands: exports arrive by player-set haul policy; the border
        // sale drains whatever lands here.
        BuildingKind::CustomsOffice => p,
    }
}

pub struct StorageSimPlugin;

impl Plugin for StorageSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attach_default_policies);
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
}
