//! Buildings & production stage 1 (spec/buildings.md, spec/production.md):
//! placed physical assets with typed inventories and per-frame recipes.
//! Bootstrap stubs per the M1 charter: placement is instant (no phased
//! construction until B6) and nothing requires staff (labour arrives in B2).

use bevy::prelude::*;

use super::resources::Inventory;
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BuildingId(pub u64);

// Hash so presentation can cache a material per kind (game/buildings.rs).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    /// Construction office (B6): home of the machine fleet (excavators,
    /// cranes) that works every `ConstructionSite`.
    ConstructionOffice,
    /// Water intake + pumphouse (B8.2): pours supply into its pipe web.
    WaterPump,
    /// Sewage treatment (B8.2): drainage capacity — a component without it
    /// backs up and shuts its water consumers.
    SewagePlant,
    /// District heating plant (B8.3): burns coal into heat pumped over Heat pipes.
    HeatPlant,
    /// Border customs (G1.2, W&R-style pulled forward from B10): imported
    /// vehicles enter the republic here and drive to their depot; goods
    /// hauled here are sold abroad for roubles.
    CustomsOffice,
}

impl BuildingKind {
    // Both lookups read the kind's catalogue row (`catalogue::BUILDINGS`): the
    // numbers and the reasons for them live there, one row per kind, so a new
    // kind is a row rather than an edit here.
    pub fn footprint(self) -> Vec2 {
        super::catalogue::spec(self).footprint
    }
    pub fn inventory_capacity(self) -> f32 {
        super::catalogue::spec(self).inventory_capacity
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
/// Lighting and appliances per residential block (B8.1): homes are grid
/// consumers ranked above industry — a starved grid browns factories out
/// first, and a dark home wears its residents down.
pub const DWELLING_DEMAND_MW: f32 = 1.0;

#[derive(Resource, Default)]
pub struct BuildingEditQueue(pub Vec<BuildingEdit>);

#[derive(Clone, Copy, Debug)]
pub enum BuildingEdit {
    Place {
        kind: BuildingKind,
        pos: Vec3,
    },
    /// Place already built (G1.5): spawns carrying `Prebuilt`, which the
    /// construction observer honours by not attaching a site. For state-
    /// provided starting infrastructure (the border customs); player edits
    /// always use `Place`.
    PlacePrebuilt {
        kind: BuildingKind,
        pos: Vec3,
    },
    /// Demolition first cut (B6.5): the building physically vanishes with
    /// its render children; workers, households, transit lines and freight
    /// orders self-heal through their own retention passes. Explosives and
    /// sorted rubble arrive with the demolition office (spec, later stage).
    Demolish {
        building: Entity,
    },
}

#[derive(Resource, Default)]
pub struct BuildingIds {
    pub next: u64,
}

/// Spawned finished: the construction observer skips this building.
#[derive(Component, Default)]
pub struct Prebuilt;

pub struct BuildingSimPlugin;

impl Plugin for BuildingSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingEditQueue>()
            .init_resource::<BuildingIds>()
            // The zoning feedback channel lives here so the siting gate works
            // headless even without the zoning plugin (no zones = free land).
            .init_resource::<super::zoning::ZoningFeedback>()
            .add_systems(
                SimTick,
                apply_building_edits
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            );
        super::production::register(app);
    }
}

pub(crate) fn apply_building_edits(
    mut commands: Commands,
    mut queue: ResMut<BuildingEditQueue>,
    mut ids: ResMut<BuildingIds>,
    existing: Query<(), With<Building>>,
    zones: Query<&super::zoning::Zone>,
    mut zoning_feedback: ResMut<super::zoning::ZoningFeedback>,
) {
    for edit in queue.0.drain(..) {
        match edit {
            BuildingEdit::Place { kind, pos } | BuildingEdit::PlacePrebuilt { kind, pos } => {
                let prebuilt = matches!(edit, BuildingEdit::PlacePrebuilt { .. });
                // The general plan gates siting (B7.3): a mismatched district
                // refuses the blueprint with feedback; unzoned land is free.
                if let Err(zone_kind) = super::zoning::siting_allowed(kind, pos, &zones) {
                    warn!("Place dropped: {kind:?} not admitted in a {zone_kind:?} district");
                    zoning_feedback.0 = Some((kind, zone_kind));
                    continue;
                }
                zoning_feedback.0 = None;
                ids.next += 1;
                // `Prebuilt` must land before `Building`: the construction
                // observer fires the instant `Building` is applied and reads
                // the marker in the same breath.
                let mut entity = commands.spawn_empty();
                if prebuilt {
                    entity.insert(Prebuilt);
                }
                entity.insert((
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
                    BuildingKind::Factory | BuildingKind::Dwelling => {
                        entity.insert(Powered::default());
                    }
                    _ => {}
                }
            }
            BuildingEdit::Demolish { building } => {
                if existing.get(building).is_ok() {
                    commands.entity(building).despawn();
                } else {
                    warn!("Demolish dropped: {building:?} is not a building");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::resources::ResourceKind;
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
