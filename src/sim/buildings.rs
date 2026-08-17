//! Buildings & production stage 1 (spec/buildings.md, spec/production.md):
//! placed physical assets with typed inventories and per-frame recipes.
//! Bootstrap stubs per the M1 charter: placement is instant (no phased
//! construction until B6) and nothing requires staff (labour arrives in B2).

use bevy::prelude::*;

use super::citizens::Citizen;
use super::labour::{Staffing, labour_factor};
use super::resources::{Inventory, ResourceKind};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BuildingId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BuildingKind {
    Mine,
    Quarry,
    PowerPlant,
    Factory,
    /// Residential block; carries a flat table (`households::Dwelling`).
    Dwelling,
    /// Player-placeable storage: a big shared yard whose per-resource bands
    /// (`storage::StoragePolicies`) drive the dispatcher.
    Warehouse,
    /// Fleet home: physical parking slots bound the owned truck fleet
    /// (`vehicles::DEPOT_SLOTS`); the only place vehicles are acquired.
    Depot,
    /// Transit stop (B5 bootstrap stub): a shelter that docks a road node;
    /// bus lines are ordered loops of these (`transit::TransitLine`).
    BusStop,
}

impl BuildingKind {
    pub fn footprint(self) -> Vec2 {
        match self {
            BuildingKind::Mine => Vec2::new(14.0, 14.0),
            BuildingKind::Quarry => Vec2::new(16.0, 12.0),
            BuildingKind::PowerPlant => Vec2::new(18.0, 14.0),
            BuildingKind::Factory => Vec2::new(20.0, 16.0),
            BuildingKind::Dwelling => Vec2::new(12.0, 10.0),
            BuildingKind::Warehouse => Vec2::new(20.0, 12.0),
            // Shed plus the two-row parking apron south of it.
            BuildingKind::Depot => Vec2::new(22.0, 26.0),
            BuildingKind::BusStop => Vec2::new(5.0, 3.0),
        }
    }
    pub fn inventory_capacity(self) -> f32 {
        match self {
            BuildingKind::Mine | BuildingKind::Quarry => 60.0,
            BuildingKind::PowerPlant => 40.0,
            BuildingKind::Factory => 40.0,
            // Goods delivered to residents land here before pantry pickup.
            BuildingKind::Dwelling => 10.0,
            BuildingKind::Warehouse => 120.0,
            // Stores no cargo; the fuel tank arrives with B8.
            BuildingKind::Depot => 0.0,
            BuildingKind::BusStop => 0.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct Building {
    pub id: BuildingId,
    pub kind: BuildingKind,
    pub pos: Vec3,
}

/// Whether the building's electricity gate is satisfied this tick. Written by
/// the power module (M1.6); production only reads it. Buildings without an
/// electricity requirement ignore it.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Powered(pub bool);

/// Plant output state, read by the wire/power module: how much generation
/// capacity is live this tick (fuel-gated).
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct PowerOutput(pub f32);

// Stage-1 recipe rates, tonnes (or MW) per sim frame.
pub const MINE_COAL_RATE: f32 = 0.05;
pub const QUARRY_GRAVEL_RATE: f32 = 0.05;
pub const PLANT_COAL_BURN: f32 = 0.02;
pub const PLANT_OUTPUT_MW: f32 = 10.0;
pub const FACTORY_GOODS_RATE: f32 = 0.03;
pub const FACTORY_DEMAND_MW: f32 = 4.0;

#[derive(Resource, Default)]
pub struct BuildingEditQueue(pub Vec<BuildingEdit>);

#[derive(Clone, Copy, Debug)]
pub enum BuildingEdit {
    Place { kind: BuildingKind, pos: Vec3 },
}

#[derive(Resource, Default)]
pub struct BuildingIds {
    pub next: u64,
}

pub struct BuildingSimPlugin;

impl Plugin for BuildingSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingEditQueue>()
            .init_resource::<BuildingIds>()
            .add_systems(
                SimTick,
                apply_building_edits
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            )
            .add_systems(
                SimTick,
                (extract_resources, run_power_plants, run_factories)
                    .chain()
                    .in_set(SimStage::ProductionAndUtilities),
            );
    }
}

fn apply_building_edits(
    mut commands: Commands,
    mut queue: ResMut<BuildingEditQueue>,
    mut ids: ResMut<BuildingIds>,
) {
    for edit in queue.0.drain(..) {
        let BuildingEdit::Place { kind, pos } = edit;
        ids.next += 1;
        let mut entity = commands.spawn((
            Building {
                id: BuildingId(ids.next),
                kind,
                pos,
            },
            Inventory::new(kind.inventory_capacity()),
        ));
        match kind {
            BuildingKind::PowerPlant => {
                entity.insert(PowerOutput::default());
            }
            BuildingKind::Factory => {
                entity.insert(Powered::default());
            }
            _ => {}
        }
    }
}

/// Mines and quarries emit their commodity into their own yard inventory.
/// A full yard halts extraction — physical stock, not a counter.
/// Staffed buildings scale with the labour factor; a building with no
/// `Staffing` ledger (headless fixtures without the labour plugin) runs free.
fn extract_resources(
    mut buildings: Query<
        (&Building, &mut Inventory, Option<&Staffing>),
        Without<super::construction::ConstructionSite>,
    >,
    citizens: Query<&Citizen>,
) {
    for (building, mut inventory, staffing) in &mut buildings {
        let f = staffing.map_or(1.0, |s| labour_factor(s, building.kind, &citizens));
        if f <= 0.0 {
            continue;
        }
        match building.kind {
            BuildingKind::Mine => {
                inventory.add(ResourceKind::Coal, MINE_COAL_RATE * f);
            }
            BuildingKind::Quarry => {
                inventory.add(ResourceKind::Gravel, QUARRY_GRAVEL_RATE * f);
            }
            _ => {}
        }
    }
}

/// The plant is an ordinary recipe building: no coal ⇒ no output; a skeleton
/// crew burns and generates proportionally less (Liebig with the fuel gate).
pub(crate) fn run_power_plants(
    mut plants: Query<
        (
            &Building,
            &mut Inventory,
            &mut PowerOutput,
            Option<&Staffing>,
        ),
        Without<super::construction::ConstructionSite>,
    >,
    citizens: Query<&Citizen>,
) {
    for (building, mut inventory, mut output, staffing) in &mut plants {
        if building.kind != BuildingKind::PowerPlant {
            continue;
        }
        let f = staffing.map_or(1.0, |s| labour_factor(s, building.kind, &citizens));
        if f <= 0.0 {
            output.0 = 0.0;
            continue;
        }
        let demand = PLANT_COAL_BURN * f;
        let burned = inventory.take(ResourceKind::Coal, demand);
        output.0 = if burned >= demand * 0.999 {
            PLANT_OUTPUT_MW * f
        } else {
            0.0
        };
    }
}

/// The factory produces only while its electricity gate holds and staff are
/// present — the scarcest factor wins.
pub(crate) fn run_factories(
    mut factories: Query<
        (&Building, &mut Inventory, &Powered, Option<&Staffing>),
        Without<super::construction::ConstructionSite>,
    >,
    citizens: Query<&Citizen>,
) {
    for (building, mut inventory, powered, staffing) in &mut factories {
        if building.kind != BuildingKind::Factory {
            continue;
        }
        let f = staffing.map_or(1.0, |s| labour_factor(s, building.kind, &citizens));
        if powered.0 && f > 0.0 {
            inventory.add(ResourceKind::Goods, FACTORY_GOODS_RATE * f);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin));
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
    fn mine_accumulates_coal_until_yard_is_full() {
        let mut app = app();
        place(&mut app, BuildingKind::Mine);
        ticks(&mut app, 101); // placement tick + 100 production frames
        let world = app.world_mut();
        let inventory = world.query::<&Inventory>().single(world).unwrap();
        let coal = inventory.amount(ResourceKind::Coal);
        assert!(
            (coal - 100.0 * MINE_COAL_RATE).abs() < 1e-3,
            "coal = {coal}"
        );
    }

    #[test]
    fn plant_output_is_fuel_gated() {
        let mut app = app();
        place(&mut app, BuildingKind::PowerPlant);
        ticks(&mut app, 2);
        let world = app.world_mut();
        // empty yard: no output
        assert_eq!(world.query::<&PowerOutput>().single(world).unwrap().0, 0.0);
        // fuel it
        let mut q = world.query::<&mut Inventory>();
        q.single_mut(world).unwrap().add(ResourceKind::Coal, 1.0);
        ticks(&mut app, 1);
        let world = app.world_mut();
        assert_eq!(
            world.query::<&PowerOutput>().single(world).unwrap().0,
            PLANT_OUTPUT_MW
        );
    }

    #[test]
    fn factory_produces_only_while_powered() {
        let mut app = app();
        place(&mut app, BuildingKind::Factory);
        ticks(&mut app, 50);
        let world = app.world_mut();
        assert_eq!(
            world
                .query::<&Inventory>()
                .single(world)
                .unwrap()
                .amount(ResourceKind::Goods),
            0.0
        );
        world.query::<&mut Powered>().single_mut(world).unwrap().0 = true;
        ticks(&mut app, 10);
        let world = app.world_mut();
        let goods = world
            .query::<&Inventory>()
            .single(world)
            .unwrap()
            .amount(ResourceKind::Goods);
        assert!((goods - 10.0 * FACTORY_GOODS_RATE).abs() < 1e-3);
    }
}
