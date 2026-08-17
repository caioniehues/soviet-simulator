//! Citizens stage 1 (spec/citizens.md, ticket #23): persistent per-citizen
//! identity as a flat value component. The rendered pawn is a separate
//! transient entity allocated only while travelling (M2.4); nothing here
//! renders. Labour allocation (M2.3) writes `work`; the housing office
//! (M2.2) writes `home` via the household.

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CitizenId(pub u64);

/// One education tier for M2 (schema allows more later; spec holds at two
/// tiers, second arrives with professor-slots).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct EducationTier(pub u8);

/// Where the citizen physically is (the spec's 2-bit location flag). Written
/// by the commute systems; production reads presence through `Staffing`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CitizenLocation {
    #[default]
    AtHome,
    ToWork,
    AtWork,
    ToHome,
}

/// Flat per-citizen state (CS1-shaped substrate). `home` mirrors the
/// household's dwelling for O(1) reads on hot paths; the household is the
/// housing actor and the only writer of `home`.
#[derive(Component, Clone, Debug)]
pub struct Citizen {
    pub id: CitizenId,
    pub household: Entity,
    pub home: Option<Entity>,
    /// Fixed workplace binding; persists until reassignment or building loss.
    pub work: Option<Entity>,
    pub health: u8,    // 0..=100
    pub wellbeing: u8, // 0..=100
    pub education: EducationTier,
    pub location: CitizenLocation,
}

impl Citizen {
    pub fn new(id: CitizenId, household: Entity) -> Self {
        Self {
            id,
            household,
            home: None,
            work: None,
            health: 100,
            wellbeing: 70,
            education: EducationTier::default(),
            location: CitizenLocation::default(),
        }
    }
}

#[derive(Resource, Default)]
pub struct CitizenIds {
    /// Last allocated id; public so the save loader can restore the counter
    /// (ADR 0004: ids are never reused, so max-seen is not enough).
    pub next: u64,
}

impl CitizenIds {
    pub fn allocate(&mut self) -> CitizenId {
        self.next += 1;
        CitizenId(self.next)
    }
}
