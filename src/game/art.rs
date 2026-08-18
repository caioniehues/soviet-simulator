//! The presentation half of the building catalogue: one row of art facts per
//! kind, the shape `sim::catalogue` uses for simulation facts. Wall, roof,
//! attach height, shipped product and toolbar label live here rather than in
//! the sim table because no sim system reads them and no save carries them
//! (ADR 0003); the reference implementation keeps meshes and materials out of
//! its building files for the same reason (ADR 0017).
//!
//! The silhouette is deliberately not a column. `parts()` in `buildings.rs` is
//! authored geometry, not data, and a shape grammar able to express it would
//! be speculative machinery.

use crate::sim::buildings::BuildingKind;
use crate::sim::resources::ResourceKind;

use super::palette::{Mat, Role};

/// A surface as data. `Mat` is a builder holding asset handles, so it cannot
/// be `const`; these three numbers are everything the wall and roof materials
/// vary, and the row rebuilds the request on demand.
pub struct Surface {
    role: Role,
    shade: f32,
    metallic: f32,
}

impl Surface {
    const fn new(role: Role, shade: f32) -> Self {
        Self {
            role,
            shade,
            metallic: 0.0,
        }
    }

    /// Only corrugated steel reads as metal; every other surface here is
    /// dielectric, which is `Mat`'s own default.
    const fn metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic;
        self
    }

    pub fn mat(&self) -> Mat {
        Mat::new(self.role)
            .shade(self.shade)
            .metallic(self.metallic)
    }
}

/// One row of the art catalogue: everything the presentation layer needs to
/// dress a kind, gathered from the four modules that hand-matched on
/// `BuildingKind`.
pub struct BuildingArt {
    pub kind: BuildingKind,
    /// The BUILD flyout's caption. Kept beside the kind so the toolbar reads
    /// one list rather than maintaining a parallel one that can drift out of
    /// order against the number-key cycle.
    pub label: &'static str,
    /// Primary wall material, as a palette role plus a shade — the families
    /// read apart at RTS zoom (brick mine, concrete plant, timber depot)
    /// without any kind inventing a colour of its own. The old table held
    /// thirteen hand-picked greys, two of which (the blue pump, the pink heat
    /// plant) were saturation the art doc reserves for signals.
    pub wall: Surface,
    /// Rusted steel over anything industrial, tarred concrete over anything
    /// civic. The old single rust roof was the other half of why every
    /// building read the same.
    pub roof: Surface,
    /// Where the presentation hangs overlays: wire spans, the power lamp, the
    /// yard fill bar. Not the mesh's tallest extent — a mine's headframe and a
    /// power plant's chimneys both overshoot it — but the height an attachment
    /// reads as belonging to the building.
    pub height: f32,
    /// What the haul-policy tool ships from this kind once its yard is empty.
    /// Not derivable from the sim recipe: a warehouse, a dwelling and a depot
    /// produce nothing and still default to Goods.
    pub shipped: ResourceKind,
}

pub const BUILDING_ART: [BuildingArt; BuildingKind::COUNT] = [
    // Mine
    BuildingArt {
        kind: BuildingKind::Mine,
        label: "MINE",
        wall: Surface::new(Role::SootBrick, 1.0),
        roof: RUST_ROOF,
        height: 6.0,
        shipped: ResourceKind::Coal,
    },
    // Quarry
    BuildingArt {
        kind: BuildingKind::Quarry,
        label: "QUARRY",
        wall: Surface::new(Role::Concrete, 0.8),
        roof: CIVIC_ROOF,
        height: 3.0,
        shipped: ResourceKind::Gravel,
    },
    // PowerPlant
    BuildingArt {
        kind: BuildingKind::PowerPlant,
        label: "POWER PLANT",
        wall: Surface::new(Role::Concrete, 0.72),
        roof: RUST_ROOF,
        height: 12.0,
        shipped: ResourceKind::Coal,
    },
    // Factory
    BuildingArt {
        kind: BuildingKind::Factory,
        label: "FACTORY",
        wall: Surface::new(Role::Concrete, 0.75),
        roof: RUST_ROOF,
        height: 9.0,
        shipped: ResourceKind::Goods,
    },
    // Dwelling
    BuildingArt {
        kind: BuildingKind::Dwelling,
        label: "DWELLING",
        wall: Surface::new(Role::Concrete, 0.82),
        roof: TARRED_ROOF,
        height: 11.0,
        shipped: ResourceKind::Goods,
    },
    // Warehouse
    BuildingArt {
        kind: BuildingKind::Warehouse,
        label: "WAREHOUSE",
        wall: Surface::new(Role::WornEarth, 0.9),
        roof: RUST_ROOF,
        height: 7.0,
        shipped: ResourceKind::Goods,
    },
    // Depot
    BuildingArt {
        kind: BuildingKind::Depot,
        label: "DEPOT",
        wall: Surface::new(Role::Timber, 1.15),
        roof: CIVIC_ROOF,
        height: 6.0,
        shipped: ResourceKind::Goods,
    },
    // BusStop
    BuildingArt {
        kind: BuildingKind::BusStop,
        label: "BUS STOP",
        wall: Surface::new(Role::Concrete, 0.85),
        roof: TARRED_ROOF,
        height: 3.0,
        shipped: ResourceKind::Goods,
    },
    // ConstructionOffice
    BuildingArt {
        kind: BuildingKind::ConstructionOffice,
        label: "CONSTR. OFFICE",
        wall: Surface::new(Role::MachineOchre, 0.85),
        roof: CIVIC_ROOF,
        height: 5.0,
        shipped: ResourceKind::Goods,
    },
    // WaterPump
    BuildingArt {
        kind: BuildingKind::WaterPump,
        label: "WATER PUMP",
        wall: Surface::new(Role::Concrete, 0.95),
        roof: RUST_ROOF,
        height: 4.0,
        shipped: ResourceKind::Goods,
    },
    // SewagePlant
    BuildingArt {
        kind: BuildingKind::SewagePlant,
        label: "SEWAGE WORKS",
        wall: Surface::new(Role::Concrete, 0.65),
        roof: RUST_ROOF,
        height: 4.0,
        shipped: ResourceKind::Goods,
    },
    // HeatPlant
    BuildingArt {
        kind: BuildingKind::HeatPlant,
        label: "HEAT PLANT",
        wall: Surface::new(Role::SootBrick, 1.2),
        roof: RUST_ROOF,
        height: 11.0,
        shipped: ResourceKind::Coal,
    },
    // CustomsOffice
    BuildingArt {
        kind: BuildingKind::CustomsOffice,
        label: "CUSTOMS",
        wall: Surface::new(Role::Concrete, 0.7),
        roof: TARRED_ROOF,
        height: 5.0,
        shipped: ResourceKind::Goods,
    },
];

/// The three roofs, named because the split is a rule about what a building
/// *is* — industrial, civic, or domestic — not thirteen independent choices.
const RUST_ROOF: Surface = Surface::new(Role::RustedSteel, 0.75).metallic(0.3);
const CIVIC_ROOF: Surface = Surface::new(Role::Concrete, 0.55);
const TARRED_ROOF: Surface = Surface::new(Role::Asphalt, 1.25);

pub fn art(kind: BuildingKind) -> &'static BuildingArt {
    &BUILDING_ART[kind as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_row_sits_at_its_own_kind_s_position() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            assert_eq!(
                BUILDING_ART[i].kind, kind,
                "row {i} drifted out of position"
            );
            assert_eq!(art(kind).kind, kind);
        }
    }

    /// (label, wall, roof, attach height, shipped product) per kind, in `ALL`'s
    /// order: a golden record of what the `match` blocks in `buildings.rs` and
    /// `vehicles.rs` and the BUILD category's hand-written list held before the
    /// art catalogue replaced them. It duplicates five table columns on
    /// purpose. `kind_material()`, `roof_material()`, `kind_height()` and
    /// `shipped_resource()` now *read* the table, so asserting the table
    /// against them would compare a value with itself — these literals are the
    /// only thing left between a mistyped row and a silently different game.
    ///
    /// It says nothing about whether the values are *right*, only that they
    /// have not drifted since the day they were transcribed. Every column here
    /// is a structural constant: no rebalance touches a roof, and a capture is
    /// the only thing that can judge them, so the friction of editing both
    /// sides is the point.
    ///
    /// Surfaces are pinned as (role, shade, metallic) rather than as a built
    /// `Mat`, so swapping *which* material a kind wears fails as loudly as
    /// mistyping a shade.
    type PinnedRow = (
        BuildingKind,
        &'static str,
        (Role, f32, f32),
        (Role, f32, f32),
        f32,
        ResourceKind,
    );

    const PINNED_ART: [PinnedRow; BuildingKind::COUNT] = [
        (
            BuildingKind::Mine,
            "MINE",
            (Role::SootBrick, 1.0, 0.0),
            (Role::RustedSteel, 0.75, 0.3),
            6.0,
            ResourceKind::Coal,
        ),
        (
            BuildingKind::Quarry,
            "QUARRY",
            (Role::Concrete, 0.8, 0.0),
            (Role::Concrete, 0.55, 0.0),
            3.0,
            ResourceKind::Gravel,
        ),
        (
            BuildingKind::PowerPlant,
            "POWER PLANT",
            (Role::Concrete, 0.72, 0.0),
            (Role::RustedSteel, 0.75, 0.3),
            12.0,
            ResourceKind::Coal,
        ),
        (
            BuildingKind::Factory,
            "FACTORY",
            (Role::Concrete, 0.75, 0.0),
            (Role::RustedSteel, 0.75, 0.3),
            9.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::Dwelling,
            "DWELLING",
            (Role::Concrete, 0.82, 0.0),
            (Role::Asphalt, 1.25, 0.0),
            11.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::Warehouse,
            "WAREHOUSE",
            (Role::WornEarth, 0.9, 0.0),
            (Role::RustedSteel, 0.75, 0.3),
            7.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::Depot,
            "DEPOT",
            (Role::Timber, 1.15, 0.0),
            (Role::Concrete, 0.55, 0.0),
            6.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::BusStop,
            "BUS STOP",
            (Role::Concrete, 0.85, 0.0),
            (Role::Asphalt, 1.25, 0.0),
            3.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::ConstructionOffice,
            "CONSTR. OFFICE",
            (Role::MachineOchre, 0.85, 0.0),
            (Role::Concrete, 0.55, 0.0),
            5.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::WaterPump,
            "WATER PUMP",
            (Role::Concrete, 0.95, 0.0),
            (Role::RustedSteel, 0.75, 0.3),
            4.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::SewagePlant,
            "SEWAGE WORKS",
            (Role::Concrete, 0.65, 0.0),
            (Role::RustedSteel, 0.75, 0.3),
            4.0,
            ResourceKind::Goods,
        ),
        (
            BuildingKind::HeatPlant,
            "HEAT PLANT",
            (Role::SootBrick, 1.2, 0.0),
            (Role::RustedSteel, 0.75, 0.3),
            11.0,
            ResourceKind::Coal,
        ),
        (
            BuildingKind::CustomsOffice,
            "CUSTOMS",
            (Role::Concrete, 0.7, 0.0),
            (Role::Asphalt, 1.25, 0.0),
            5.0,
            ResourceKind::Goods,
        ),
    ];

    /// Every pinned test opens with this, so a reordered table fails as a
    /// reorder rather than as thirteen unrelated value mismatches.
    fn pinned(i: usize, kind: BuildingKind) -> PinnedRow {
        let row = PINNED_ART[i];
        assert_eq!(row.0, kind, "pinned row {i} drifted out of ALL's order");
        row
    }

    fn surface_of(surface: &Surface) -> (Role, f32, f32) {
        (surface.role, surface.shade, surface.metallic)
    }

    #[test]
    fn wall_column_holds_its_pinned_surfaces() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (_, _, wall, ..) = pinned(i, kind);
            assert_eq!(surface_of(&art(kind).wall), wall, "{kind:?} wall");
        }
    }

    #[test]
    fn roof_column_holds_its_pinned_surfaces() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (_, _, _, roof, ..) = pinned(i, kind);
            assert_eq!(surface_of(&art(kind).roof), roof, "{kind:?} roof");
        }
    }

    #[test]
    fn height_column_holds_its_pinned_values() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (.., height, _) = pinned(i, kind);
            assert_eq!(art(kind).height, height, "{kind:?} attach height");
        }
    }

    #[test]
    fn shipped_column_holds_its_pinned_products() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (.., shipped) = pinned(i, kind);
            assert_eq!(art(kind).shipped, shipped, "{kind:?} shipped product");
        }
    }

    #[test]
    fn label_column_holds_its_pinned_captions() {
        for (i, kind) in BuildingKind::ALL.into_iter().enumerate() {
            let (_, label, ..) = pinned(i, kind);
            assert_eq!(art(kind).label, label, "{kind:?} toolbar label");
        }
    }
}
