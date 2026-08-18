//! The building catalogue (`.planning/2026-08-17-data-driven-buildings`): one
//! row per `BuildingKind`, holding today's exact per-kind numbers — footprint,
//! yard capacity, vacancy count, default storage bands, material recipe and
//! utility roles — the way W&R's `buildings_types/*.ini` holds one building
//! per file. `footprint()`, `inventory_capacity()`, `workers_needed()` and
//! `default_policies()` are now one-liners over this table and no longer carry
//! a match of their own; the production pass and the utility solves still do,
//! and this module's tests prove the table already agrees with each of them
//! before it is switched over.

use bevy::prelude::*;

use super::buildings::{
    BuildingKind, DWELLING_DEMAND_MW, FACTORY_DEMAND_MW, FACTORY_GOODS_RATE, MINE_COAL_RATE,
    PLANT_COAL_BURN, QUARRY_GRAVEL_RATE,
};
use super::heat::HEAT_PLANT_COAL_BURN;
use super::network::PriorityClass;
use super::resources::ResourceKind;
use super::water::{DWELLING_WATER, FACTORY_WATER, PUMP_SUPPLY, SEWAGE_CAPACITY};

impl BuildingKind {
    pub const COUNT: usize = 13;
    pub const ALL: [BuildingKind; Self::COUNT] = [
        BuildingKind::Mine,
        BuildingKind::Quarry,
        BuildingKind::PowerPlant,
        BuildingKind::Factory,
        BuildingKind::Dwelling,
        BuildingKind::Warehouse,
        BuildingKind::Depot,
        BuildingKind::BusStop,
        BuildingKind::ConstructionOffice,
        BuildingKind::WaterPump,
        BuildingKind::SewagePlant,
        BuildingKind::HeatPlant,
        BuildingKind::CustomsOffice,
    ];
}

/// Material bill a kind consumes and produces every production frame,
/// mirroring W&R's `$CONSUMPTION`/`$PRODUCTION` lines. Electricity and heat
/// are flows, not stored commodities (`resources.rs`), so a plant's fuel burn
/// is an input here but its MW/heat output is not — that lives in `demands`.
#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub inputs: &'static [(ResourceKind, f32)],
    pub outputs: &'static [(ResourceKind, f32)],
}

const NO_RECIPE: Recipe = Recipe {
    inputs: &[],
    outputs: &[],
};

/// A fixed draw against a shared-pool utility net (`network::allocate`),
/// ranked by `priority` when the pool runs short — what `solve_power`'s and
/// `solve_water`'s per-kind demand match builds today.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UtilityDemand {
    pub rate: f32,
    pub priority: PriorityClass,
}

/// A kind's role on the water net (`solve_water`'s per-kind match): a
/// consumer draws `rate` against both the intake and the drainage pool (the
/// cycle needs both closed, `water.rs`'s doc comment), a pump pours `rate`
/// into the intake pool, a treatment works pours `rate` into the drainage
/// pool.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WaterDemand {
    Draws(UtilityDemand),
    Supplies(f32),
    Drains(f32),
}

/// A kind's role on the heat net (`attach_heat_components`'s per-kind
/// match). The consumer draw itself is climate-driven
/// (`heat::dwelling_heat_demand`), not a per-kind constant, so only
/// membership — which gate a kind carries — is representable here.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeatDemand {
    Consumer,
    Producer,
}

/// One row of the catalogue: everything a building kind needs to exist today,
/// gathered from the four modules that currently hand-match on `BuildingKind`.
pub struct BuildingSpec {
    pub kind: BuildingKind,
    pub footprint: Vec2,
    pub inventory_capacity: f32,
    pub workers_needed: u32,
    /// Storage bands a newly placed building gets (`storage::default_policies`),
    /// as (resource, min_pct, max_pct) triples — the same shape `StoragePolicies::with`
    /// takes, kept as data rather than a built `StoragePolicies` because that
    /// type's construction is not `const`.
    pub default_policies: &'static [(ResourceKind, f32, f32)],
    pub recipe: Recipe,
    /// `solve_power`'s per-kind demand match; `None` outside {Factory, Dwelling}.
    pub power: Option<UtilityDemand>,
    /// `attach_watered` + `solve_water`'s per-kind match; `None` outside
    /// {Dwelling, Factory, WaterPump, SewagePlant}.
    pub water: Option<WaterDemand>,
    /// `attach_heat_components`'s per-kind match; `None` outside
    /// {Dwelling, HeatPlant}.
    pub heat: Option<HeatDemand>,
}

pub const BUILDINGS: [BuildingSpec; BuildingKind::COUNT] = [
    // Mine
    BuildingSpec {
        kind: BuildingKind::Mine,
        footprint: Vec2::new(14.0, 14.0),
        inventory_capacity: 60.0,
        workers_needed: 6,
        default_policies: &[(ResourceKind::Coal, 0.0, 0.05)],
        recipe: Recipe {
            inputs: &[],
            outputs: &[(ResourceKind::Coal, MINE_COAL_RATE)],
        },
        power: None,
        water: None,
        heat: None,
    },
    // Quarry
    BuildingSpec {
        kind: BuildingKind::Quarry,
        footprint: Vec2::new(16.0, 12.0),
        inventory_capacity: 60.0,
        workers_needed: 4,
        default_policies: &[(ResourceKind::Gravel, 0.0, 0.05)],
        recipe: Recipe {
            inputs: &[],
            outputs: &[(ResourceKind::Gravel, QUARRY_GRAVEL_RATE)],
        },
        power: None,
        water: None,
        heat: None,
    },
    // PowerPlant
    BuildingSpec {
        kind: BuildingKind::PowerPlant,
        footprint: Vec2::new(18.0, 14.0),
        inventory_capacity: 40.0,
        workers_needed: 8,
        default_policies: &[(ResourceKind::Coal, 0.6, 1.0)],
        recipe: Recipe {
            inputs: &[(ResourceKind::Coal, PLANT_COAL_BURN)],
            outputs: &[],
        },
        power: None,
        water: None,
        heat: None,
    },
    // Factory — inputs stay empty: Goods materialise from nothing today
    // (the doctrine violation `findings.md` names), fixed only once Steel
    // and Boards exist to feed it (R4). The table represents today's
    // behaviour, not the corrected one.
    BuildingSpec {
        kind: BuildingKind::Factory,
        footprint: Vec2::new(20.0, 16.0),
        inventory_capacity: 40.0,
        workers_needed: 10,
        default_policies: &[(ResourceKind::Goods, 0.0, 0.1)],
        recipe: Recipe {
            inputs: &[],
            outputs: &[(ResourceKind::Goods, FACTORY_GOODS_RATE)],
        },
        power: Some(UtilityDemand {
            rate: FACTORY_DEMAND_MW,
            priority: PriorityClass::Industry,
        }),
        water: Some(WaterDemand::Draws(UtilityDemand {
            rate: FACTORY_WATER,
            priority: PriorityClass::Industry,
        })),
        heat: None,
    },
    // Dwelling — the yard is the doorstep: goods delivered to residents land
    // here before pantry pickup.
    BuildingSpec {
        kind: BuildingKind::Dwelling,
        footprint: Vec2::new(12.0, 10.0),
        inventory_capacity: 10.0,
        workers_needed: 0,
        default_policies: &[(ResourceKind::Goods, 0.5, 1.0)],
        recipe: NO_RECIPE,
        power: Some(UtilityDemand {
            rate: DWELLING_DEMAND_MW,
            priority: PriorityClass::Housing,
        }),
        water: Some(WaterDemand::Draws(UtilityDemand {
            rate: DWELLING_WATER,
            priority: PriorityClass::Housing,
        })),
        heat: Some(HeatDemand::Consumer),
    },
    // Warehouse — the neutral all-resource band; ResourceKind::ALL's fold
    // in `storage::default_policies` unrolled here since the table wants
    // one flat literal per kind.
    BuildingSpec {
        kind: BuildingKind::Warehouse,
        footprint: Vec2::new(20.0, 12.0),
        inventory_capacity: 120.0,
        workers_needed: 0,
        default_policies: &[
            (ResourceKind::Coal, 0.2, 0.6),
            (ResourceKind::Gravel, 0.2, 0.6),
            (ResourceKind::Goods, 0.2, 0.6),
        ],
        recipe: NO_RECIPE,
        power: None,
        water: None,
        heat: None,
    },
    // Depot — shed plus the two-row parking apron south of it. Stores no
    // cargo; the fuel tank arrives with B8.
    BuildingSpec {
        kind: BuildingKind::Depot,
        footprint: Vec2::new(22.0, 26.0),
        inventory_capacity: 0.0,
        workers_needed: 0,
        default_policies: &[],
        recipe: NO_RECIPE,
        power: None,
        water: None,
        heat: None,
    },
    // BusStop
    BuildingSpec {
        kind: BuildingKind::BusStop,
        footprint: Vec2::new(5.0, 3.0),
        inventory_capacity: 0.0,
        workers_needed: 0,
        default_policies: &[],
        recipe: NO_RECIPE,
        power: None,
        water: None,
        heat: None,
    },
    // ConstructionOffice — office hut plus the machine apron.
    BuildingSpec {
        kind: BuildingKind::ConstructionOffice,
        footprint: Vec2::new(20.0, 22.0),
        inventory_capacity: 0.0,
        workers_needed: 0,
        default_policies: &[],
        recipe: NO_RECIPE,
        power: None,
        water: None,
        heat: None,
    },
    // WaterPump
    BuildingSpec {
        kind: BuildingKind::WaterPump,
        footprint: Vec2::new(10.0, 8.0),
        inventory_capacity: 0.0,
        workers_needed: 0,
        default_policies: &[],
        recipe: NO_RECIPE,
        power: None,
        water: Some(WaterDemand::Supplies(PUMP_SUPPLY)),
        heat: None,
    },
    // SewagePlant
    BuildingSpec {
        kind: BuildingKind::SewagePlant,
        footprint: Vec2::new(16.0, 12.0),
        inventory_capacity: 0.0,
        workers_needed: 0,
        default_policies: &[],
        recipe: NO_RECIPE,
        power: None,
        water: Some(WaterDemand::Drains(SEWAGE_CAPACITY)),
        heat: None,
    },
    // HeatPlant
    BuildingSpec {
        kind: BuildingKind::HeatPlant,
        footprint: Vec2::new(18.0, 12.0),
        inventory_capacity: 40.0,
        workers_needed: 0,
        default_policies: &[(ResourceKind::Coal, 0.6, 1.0)],
        recipe: Recipe {
            inputs: &[(ResourceKind::Coal, HEAT_PLANT_COAL_BURN)],
            outputs: &[],
        },
        power: None,
        water: None,
        heat: Some(HeatDemand::Producer),
    },
    // CustomsOffice — gatehouse plus inspection yard, and that yard is the
    // export dock: goods wait there for the border sale. No bands, though —
    // exports arrive by player-set haul policy and the sale drains whatever
    // lands here.
    BuildingSpec {
        kind: BuildingKind::CustomsOffice,
        footprint: Vec2::new(22.0, 14.0),
        inventory_capacity: 120.0,
        workers_needed: 0,
        default_policies: &[],
        recipe: NO_RECIPE,
        power: None,
        water: None,
        heat: None,
    },
];

pub fn spec(kind: BuildingKind) -> &'static BuildingSpec {
    &BUILDINGS[kind as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::SimPlugin;
    use crate::sim::buildings::{
        Building, BuildingEdit, BuildingEditQueue, BuildingSimPlugin, Powered,
    };
    use crate::sim::construction::ConstructionSimPlugin;
    use crate::sim::heat::HeatSimPlugin;
    use crate::sim::resources::Inventory;
    use crate::sim::storage::default_policies;
    use std::time::Duration;

    #[test]
    fn every_row_sits_at_its_own_kind_s_position() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            assert_eq!(BUILDINGS[i].kind, kind, "row {i} drifted out of position");
            assert_eq!(spec(kind).kind, kind);
        }
    }

    /// (footprint, yard capacity, vacancies, storage bands) per kind, in `ALL`'s
    /// order: a golden record of what the four `match` blocks in `buildings.rs`,
    /// `labour.rs` and `storage.rs` held before the catalogue replaced them. It
    /// duplicates four table columns on purpose. `footprint()`,
    /// `inventory_capacity()`, `workers_needed()` and `default_policies()` now
    /// *read* the table, so asserting the table against them would compare a
    /// value with itself — these literals are the only thing left between a
    /// mistyped row and a silently different game.
    ///
    /// It says nothing about whether the numbers are *right*, only that they
    /// have not drifted since the day they were transcribed. A deliberate
    /// rebalance is expected to edit both sides: that friction is the point on
    /// a structural constant, which should change rarely and never by accident.
    ///
    /// Rates and utility demands are deliberately absent. They move every time
    /// R4 tunes something, so pinning them would be friction without a
    /// guarantee; `one_frame_moves_exactly_what_the_recipe_claims` guards those
    /// by ticking the building instead, which is the stronger assertion anyway
    /// — it also catches a row being right while the system ignores it.
    type PinnedRow = (
        BuildingKind,
        Vec2,
        f32,
        u32,
        &'static [(ResourceKind, f32, f32)],
    );

    const PINNED_COLUMNS: [PinnedRow; BuildingKind::COUNT] = [
        (
            BuildingKind::Mine,
            Vec2::new(14.0, 14.0),
            60.0,
            6,
            &[(ResourceKind::Coal, 0.0, 0.05)],
        ),
        (
            BuildingKind::Quarry,
            Vec2::new(16.0, 12.0),
            60.0,
            4,
            &[(ResourceKind::Gravel, 0.0, 0.05)],
        ),
        (
            BuildingKind::PowerPlant,
            Vec2::new(18.0, 14.0),
            40.0,
            8,
            &[(ResourceKind::Coal, 0.6, 1.0)],
        ),
        (
            BuildingKind::Factory,
            Vec2::new(20.0, 16.0),
            40.0,
            10,
            &[(ResourceKind::Goods, 0.0, 0.1)],
        ),
        (
            BuildingKind::Dwelling,
            Vec2::new(12.0, 10.0),
            10.0,
            0,
            &[(ResourceKind::Goods, 0.5, 1.0)],
        ),
        (
            BuildingKind::Warehouse,
            Vec2::new(20.0, 12.0),
            120.0,
            0,
            &[
                (ResourceKind::Coal, 0.2, 0.6),
                (ResourceKind::Gravel, 0.2, 0.6),
                (ResourceKind::Goods, 0.2, 0.6),
            ],
        ),
        (BuildingKind::Depot, Vec2::new(22.0, 26.0), 0.0, 0, &[]),
        (BuildingKind::BusStop, Vec2::new(5.0, 3.0), 0.0, 0, &[]),
        (
            BuildingKind::ConstructionOffice,
            Vec2::new(20.0, 22.0),
            0.0,
            0,
            &[],
        ),
        (BuildingKind::WaterPump, Vec2::new(10.0, 8.0), 0.0, 0, &[]),
        (
            BuildingKind::SewagePlant,
            Vec2::new(16.0, 12.0),
            0.0,
            0,
            &[],
        ),
        (
            BuildingKind::HeatPlant,
            Vec2::new(18.0, 12.0),
            40.0,
            0,
            &[(ResourceKind::Coal, 0.6, 1.0)],
        ),
        (
            BuildingKind::CustomsOffice,
            Vec2::new(22.0, 14.0),
            120.0,
            0,
            &[],
        ),
    ];

    #[test]
    fn footprint_column_holds_its_pinned_values() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (pinned_kind, footprint, ..) = PINNED_COLUMNS[i];
            assert_eq!(
                pinned_kind, kind,
                "pinned row {i} drifted out of ALL's order"
            );
            assert_eq!(spec(kind).footprint, footprint, "{kind:?}");
        }
    }

    #[test]
    fn inventory_capacity_column_holds_its_pinned_values() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (pinned_kind, _, capacity, ..) = PINNED_COLUMNS[i];
            assert_eq!(
                pinned_kind, kind,
                "pinned row {i} drifted out of ALL's order"
            );
            assert_eq!(spec(kind).inventory_capacity, capacity, "{kind:?}");
        }
    }

    #[test]
    fn workers_needed_column_holds_its_pinned_values() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (pinned_kind, _, _, workers, _) = PINNED_COLUMNS[i];
            assert_eq!(
                pinned_kind, kind,
                "pinned row {i} drifted out of ALL's order"
            );
            assert_eq!(spec(kind).workers_needed, workers, "{kind:?}");
        }
    }

    fn band_of(
        bands: &'static [(ResourceKind, f32, f32)],
        resource: ResourceKind,
    ) -> Option<(f32, f32)> {
        bands
            .iter()
            .find(|(r, ..)| *r == resource)
            .map(|&(_, min, max)| (min, max))
    }

    /// Membership as much as magnitude: a band a row *grew* is drift just as
    /// much as a band whose numbers moved — the Depot has never banded anything,
    /// and giving it one would post a demand the dispatcher then hauls against.
    /// Compared per resource rather than slice against slice, because the fold
    /// is order-insensitive: a row listing its bands in another order behaves
    /// identically, and a golden record shouldn't fail on a difference the game
    /// cannot see.
    #[test]
    fn default_policies_column_holds_its_pinned_bands() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (pinned_kind, .., bands) = PINNED_COLUMNS[i];
            assert_eq!(
                pinned_kind, kind,
                "pinned row {i} drifted out of ALL's order"
            );
            for resource in ResourceKind::ALL {
                let pinned = band_of(bands, resource);
                let listed = band_of(spec(kind).default_policies, resource);
                assert_eq!(listed, pinned, "{kind:?} / {resource:?}");
            }
        }
    }

    /// The other half of the chain of custody: the pin above proves the row
    /// still lists the right triples, this proves `storage::default_policies`
    /// turns them into the right `StoragePolicies` — every band arrives, none is
    /// dropped, and `StorageBand::new`'s clamp leaves the values alone.
    /// Band-by-band rather than a whole-struct `==`, because `StoragePolicies`
    /// has no `PartialEq` (nothing but this test would ever need one).
    #[test]
    fn default_policies_folds_every_band_the_row_lists() {
        for kind in BuildingKind::ALL {
            let live = default_policies(kind);
            for resource in ResourceKind::ALL {
                let expected = band_of(spec(kind).default_policies, resource);
                let actual = live.band(resource).map(|b| (b.min_pct, b.max_pct));
                assert_eq!(actual, expected, "{kind:?} / {resource:?}");
            }
        }
    }

    /// Yard headroom for the one-frame fixture. Five kinds store nothing when
    /// finished, so their real yard refuses the seed and "consumes nothing"
    /// would hold vacuously; widening it first makes a rogue draw — or a rogue
    /// output — observable on every kind alike.
    const FIXTURE_CAPACITY: f32 = 1000.0;
    /// Seeded stock per resource: comfortably more than any one frame's input
    /// draw, small enough that a 0.02 t delta doesn't drown in f32 cancellation.
    const FIXTURE_STOCK: f32 = 1.0;
    /// One frame moves 0.02–0.05 t off a 1.0 t base; the float noise there is
    /// around 1e-7.
    const RATE_EPSILON: f32 = 1e-5;

    /// The plugins that own a production pass today — `extract_resources`,
    /// `run_power_plants`, `run_factories` (buildings) and `run_heat_plants`
    /// (heat) — plus construction, so that "placed pre-built" means a building
    /// that genuinely is not a site. Deliberately no customs plugin: the border
    /// sale drains the export yard by its own rules, which the recipe column
    /// does not model.
    fn production_app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            BuildingSimPlugin,
            HeatSimPlugin,
            ConstructionSimPlugin,
        ));
        a
    }

    fn tick(app: &mut App) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
        app.update();
    }

    fn yard(world: &World, entity: Entity) -> [f32; ResourceKind::COUNT] {
        let inventory = world.get::<Inventory>(entity).unwrap();
        ResourceKind::ALL.map(|resource| inventory.amount(resource))
    }

    /// What one production frame of `kind` actually moves through its yard,
    /// with every gate the production systems read satisfied: fuel is the
    /// seeded stock, the electricity gate is set by hand, and staffing and
    /// water gate on component *absence* — no labour or water plugin here, the
    /// "runs free" fixture path `extract_resources` documents — so the observed
    /// delta is the unscaled base rate the table holds.
    fn one_frame_yard_delta(kind: BuildingKind) -> [f32; ResourceKind::COUNT] {
        let mut app = production_app();
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::PlacePrebuilt {
                kind,
                pos: Vec3::ZERO,
            });
        // The spawn flushes at the post-Commit barrier, so this tick only
        // places; the next one is the building's first production frame.
        tick(&mut app);
        let world = app.world_mut();
        let entity = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .find(|(_, b)| b.kind == kind)
            .unwrap()
            .0;
        {
            let mut inventory = world.get_mut::<Inventory>(entity).unwrap();
            inventory.capacity = FIXTURE_CAPACITY;
            for resource in ResourceKind::ALL {
                inventory.add(resource, FIXTURE_STOCK);
            }
        }
        if let Some(mut powered) = world.get_mut::<Powered>(entity) {
            powered.0 = true;
        }
        let before = yard(app.world(), entity);
        tick(&mut app);
        let after = yard(app.world(), entity);
        std::array::from_fn(|i| after[i] - before[i])
    }

    /// What the row claims one frame moves: outputs credit the yard, inputs
    /// debit it.
    fn recipe_yard_delta(recipe: &Recipe) -> [f32; ResourceKind::COUNT] {
        let mut delta = [0.0; ResourceKind::COUNT];
        for &(resource, rate) in recipe.outputs {
            delta[resource as usize] += rate;
        }
        for &(resource, rate) in recipe.inputs {
            delta[resource as usize] -= rate;
        }
        delta
    }

    /// The recipe column against observed behaviour, all 13 kinds. Asserting a
    /// rate against the constant the row itself names proves nothing:
    /// `MINE_COAL_RATE == QUARRY_GRAVEL_RATE` and
    /// `PLANT_COAL_BURN == HEAT_PLANT_COAL_BURN` today, so a row naming the
    /// wrong one stays invisible until R4 rebalances one of them. This ticks
    /// each kind and reads its yard instead, which also makes the eight
    /// recipe-less kinds assert something. It is the contract Phase 4's generic
    /// production pass has to satisfy.
    #[test]
    fn one_frame_moves_exactly_what_the_recipe_claims() {
        for kind in BuildingKind::ALL {
            let observed = one_frame_yard_delta(kind);
            let claimed = recipe_yard_delta(&spec(kind).recipe);
            for resource in ResourceKind::ALL {
                let i = resource as usize;
                assert!(
                    (observed[i] - claimed[i]).abs() < RATE_EPSILON,
                    "{kind:?} / {resource:?}: one frame moved {}, the catalogue claims {}",
                    observed[i],
                    claimed[i]
                );
            }
        }
    }

    #[test]
    fn power_demand_matches_solve_power_s_per_kind_match() {
        for kind in BuildingKind::ALL {
            let expected = match kind {
                BuildingKind::Factory => Some(UtilityDemand {
                    rate: FACTORY_DEMAND_MW,
                    priority: PriorityClass::Industry,
                }),
                BuildingKind::Dwelling => Some(UtilityDemand {
                    rate: DWELLING_DEMAND_MW,
                    priority: PriorityClass::Housing,
                }),
                _ => None,
            };
            assert_eq!(spec(kind).power, expected, "{kind:?}");
        }
    }

    #[test]
    fn water_demand_matches_attach_watered_and_solve_water_s_per_kind_match() {
        for kind in BuildingKind::ALL {
            let expected = match kind {
                BuildingKind::Dwelling => Some(WaterDemand::Draws(UtilityDemand {
                    rate: DWELLING_WATER,
                    priority: PriorityClass::Housing,
                })),
                BuildingKind::Factory => Some(WaterDemand::Draws(UtilityDemand {
                    rate: FACTORY_WATER,
                    priority: PriorityClass::Industry,
                })),
                BuildingKind::WaterPump => Some(WaterDemand::Supplies(PUMP_SUPPLY)),
                BuildingKind::SewagePlant => Some(WaterDemand::Drains(SEWAGE_CAPACITY)),
                _ => None,
            };
            assert_eq!(spec(kind).water, expected, "{kind:?}");
        }
    }

    #[test]
    fn heat_demand_matches_attach_heat_components_s_per_kind_match() {
        for kind in BuildingKind::ALL {
            let expected = match kind {
                BuildingKind::Dwelling => Some(HeatDemand::Consumer),
                BuildingKind::HeatPlant => Some(HeatDemand::Producer),
                _ => None,
            };
            assert_eq!(spec(kind).heat, expected, "{kind:?}");
        }
    }
}
