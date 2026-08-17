//! The building catalogue (`.planning/2026-08-17-data-driven-buildings`): one
//! row per `BuildingKind`, holding today's exact per-kind numbers — footprint,
//! yard capacity, vacancy count, default storage bands, material recipe and
//! utility roles — the way W&R's `buildings_types/*.ini` holds one building
//! per file. Nothing reads this table yet: `footprint()`, `inventory_capacity()`,
//! `workers_needed()`, `default_policies()` and the utility solves still carry
//! their own hand-written matches, and this module's tests exist to prove the
//! table already agrees with every one of them before anything is switched
//! over.

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
    // Dwelling
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
    // Depot — stores no cargo; the fuel tank arrives with B8.
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
    // ConstructionOffice
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
    // CustomsOffice — no bands: exports arrive by player-set haul policy;
    // the border sale drains whatever lands here.
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
    use crate::sim::storage::default_policies;

    #[test]
    fn every_row_sits_at_its_own_kind_s_position() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            assert_eq!(BUILDINGS[i].kind, kind, "row {i} drifted out of position");
            assert_eq!(spec(kind).kind, kind);
        }
    }

    #[test]
    fn footprint_matches_the_hand_written_lookup() {
        for kind in BuildingKind::ALL {
            assert_eq!(spec(kind).footprint, kind.footprint(), "{kind:?}");
        }
    }

    #[test]
    fn inventory_capacity_matches_the_hand_written_lookup() {
        for kind in BuildingKind::ALL {
            assert_eq!(
                spec(kind).inventory_capacity,
                kind.inventory_capacity(),
                "{kind:?}"
            );
        }
    }

    #[test]
    fn workers_needed_matches_the_hand_written_lookup() {
        for kind in BuildingKind::ALL {
            assert_eq!(spec(kind).workers_needed, kind.workers_needed(), "{kind:?}");
        }
    }

    /// Band-by-band rather than a whole-struct `==`: `StoragePolicies` has no
    /// `PartialEq` (nothing but this test would ever need one), so the table's
    /// (resource, min, max) triples are compared against the live function's
    /// bands directly.
    #[test]
    fn default_policies_matches_the_hand_written_lookup() {
        for kind in BuildingKind::ALL {
            let live = default_policies(kind);
            for resource in ResourceKind::ALL {
                let expected = spec(kind)
                    .default_policies
                    .iter()
                    .find(|(r, ..)| *r == resource)
                    .map(|&(_, min, max)| (min, max));
                let actual = live.band(resource).map(|b| (b.min_pct, b.max_pct));
                assert_eq!(actual, expected, "{kind:?} / {resource:?}");
            }
        }
    }

    #[test]
    fn recipe_rates_match_the_named_production_constants() {
        use super::super::buildings::{
            FACTORY_GOODS_RATE, MINE_COAL_RATE, PLANT_COAL_BURN, QUARRY_GRAVEL_RATE,
        };
        use super::super::heat::HEAT_PLANT_COAL_BURN;
        assert_eq!(
            spec(BuildingKind::Mine).recipe.outputs,
            &[(ResourceKind::Coal, MINE_COAL_RATE)]
        );
        assert_eq!(
            spec(BuildingKind::Quarry).recipe.outputs,
            &[(ResourceKind::Gravel, QUARRY_GRAVEL_RATE)]
        );
        assert_eq!(
            spec(BuildingKind::PowerPlant).recipe.inputs,
            &[(ResourceKind::Coal, PLANT_COAL_BURN)]
        );
        assert_eq!(
            spec(BuildingKind::Factory).recipe.inputs,
            &[] as &[(ResourceKind, f32)],
            "the Factory's doctrine violation stays represented, not fixed"
        );
        assert_eq!(
            spec(BuildingKind::Factory).recipe.outputs,
            &[(ResourceKind::Goods, FACTORY_GOODS_RATE)]
        );
        assert_eq!(
            spec(BuildingKind::HeatPlant).recipe.inputs,
            &[(ResourceKind::Coal, HEAT_PLANT_COAL_BURN)]
        );
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
