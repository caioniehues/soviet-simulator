//! Phased construction (B6, spec/construction.md): a placed blueprint
//! becomes a working building only through the phase ladder — earthworks →
//! structure → finishing in v1 — each phase consuming delivered materials
//! from the site's own yard and machine-work from a construction fleet
//! (W&R's confirmed model; duration is emergent from supply, never a timer).
//!
//! The plugin is opt-in (the `Staffing` pattern): fixtures and older capture
//! bins that never add it keep instant fiat placement. With it added, every
//! placed building spawns carrying a `ConstructionSite`, and production /
//! power / staffing treat it as inert until the site completes.
//!
//! Material bill v1 is billed on the existing resource set (gravel for the
//! pad, goods as the boxed structure stand-in) — the real steel/concrete
//! tree widens with B9/B10 industry.

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::resources::{Inventory, ResourceKind};
use super::stages::{SimStage, SimTick};

/// Tonnes a site crew moves from the yard into the works per tick while the
/// current phase still needs material.
pub const CONSUME_RATE: f32 = 0.2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PhaseKind {
    Earthworks,
    Structure,
    Finishing,
}

/// Machine skill classes (W&R `$SKILL_CONSTRUCTION_*`), matched against the
/// construction fleet in B6.3.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Skill {
    Groundworks,
    Crane,
}

#[derive(Clone, Copy, Debug)]
pub struct SitePhase {
    pub kind: PhaseKind,
    /// Material bill: what the phase consumes from the site yard, if any.
    pub material: Option<(ResourceKind, f32)>,
    /// Tonnes already folded into the works.
    pub consumed: f32,
    /// Work units required / done. Progress rate = Σ matching machine skill.
    pub work: f32,
    pub done: f32,
    pub skill: Skill,
}

impl SitePhase {
    pub fn material_outstanding(&self) -> f32 {
        self.material
            .map(|(_, need)| (need - self.consumed).max(0.0))
            .unwrap_or(0.0)
    }
}

/// Why the current phase is not progressing this tick — the named signal the
/// whole milestone exists for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bottleneck {
    NoMaterial,
    NoMachine,
}

/// A building mid-construction. Present ⇒ the building is inert (production,
/// power output, staffing and dispatch treat it as not-yet-existing).
/// Removal of this component *is* activation.
#[derive(Component, Debug)]
pub struct ConstructionSite {
    pub phases: Vec<SitePhase>,
    pub current: usize,
    pub bottleneck: Option<Bottleneck>,
    /// Work applied this pass by parked machines (B6.3 writes, advance reads).
    pub throughput: f32,
}

impl ConstructionSite {
    /// Bill of quantities from the blueprint's footprint area — bigger
    /// buildings genuinely cost more (W&R derives this from mesh geometry;
    /// our stand-in is the pad area).
    pub fn for_kind(kind: BuildingKind) -> Self {
        let fp = kind.footprint();
        let area = fp.x * fp.y;
        let phases = vec![
            SitePhase {
                kind: PhaseKind::Earthworks,
                material: Some((ResourceKind::Gravel, (area * 0.02).max(1.0))),
                consumed: 0.0,
                work: area * 0.5,
                done: 0.0,
                skill: Skill::Groundworks,
            },
            SitePhase {
                kind: PhaseKind::Structure,
                material: Some((ResourceKind::Goods, (area * 0.03).max(1.0))),
                consumed: 0.0,
                work: area,
                done: 0.0,
                skill: Skill::Crane,
            },
            SitePhase {
                kind: PhaseKind::Finishing,
                material: None,
                consumed: 0.0,
                work: area * 0.5,
                done: 0.0,
                skill: Skill::Crane,
            },
        ];
        Self {
            phases,
            current: 0,
            bottleneck: None,
            throughput: 0.0,
        }
    }

    pub fn phase(&self) -> Option<&SitePhase> {
        self.phases.get(self.current)
    }

    pub fn complete(&self) -> bool {
        self.current >= self.phases.len()
    }

    /// 0..1 across all phases, for the rising-structure render (B6.4).
    pub fn progress(&self) -> f32 {
        let total: f32 = self.phases.iter().map(|p| p.work).sum();
        let done: f32 = self.phases.iter().map(|p| p.done).sum();
        if total > 0.0 { done / total } else { 1.0 }
    }
}

pub struct ConstructionSimPlugin;

impl Plugin for ConstructionSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attach_sites).add_systems(
            SimTick,
            advance_sites.in_set(SimStage::ProductionAndUtilities),
        );
    }
}

/// Every building placed while phased construction is on starts as a site.
fn attach_sites(add: On<Add, Building>, mut commands: Commands, buildings: Query<&Building>) {
    let entity = add.entity;
    let Ok(building) = buildings.get(entity) else {
        return;
    };
    commands
        .entity(entity)
        .insert(ConstructionSite::for_kind(building.kind));
}

/// The site tick: fold yard material into the current phase, then apply the
/// machine throughput B6.3's fleet parked here this pass. A missing factor
/// stalls the phase with its name — never a silent wait.
fn advance_sites(
    mut commands: Commands,
    mut sites: Query<(Entity, &mut ConstructionSite, &mut Inventory)>,
) {
    for (entity, mut site, mut yard) in &mut sites {
        let throughput = site.throughput;
        site.throughput = 0.0;
        let current = site.current;
        let Some(phase) = site.phases.get_mut(current) else {
            commands.entity(entity).remove::<ConstructionSite>();
            continue;
        };
        // Materials first: the works absorb what the yard holds.
        let mut stall = None;
        if let Some((resource, need)) = phase.material
            && phase.consumed < need
        {
            let taken = yard.take(resource, CONSUME_RATE.min(need - phase.consumed));
            phase.consumed += taken;
            if phase.consumed < need {
                stall = Some(Bottleneck::NoMaterial);
            }
        }
        // Then machine-work: rate is whatever skill parked here this pass.
        if stall.is_none() {
            if throughput <= 0.0 {
                stall = Some(Bottleneck::NoMachine);
            } else {
                phase.done = (phase.done + throughput).min(phase.work);
            }
        }
        let finished = phase.done >= phase.work;
        site.bottleneck = stall;
        if finished {
            site.current += 1;
            if site.complete() {
                // Activation: the component's removal is the gate opening.
                commands.entity(entity).remove::<ConstructionSite>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingSimPlugin, FACTORY_GOODS_RATE, Powered,
    };
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, ConstructionSimPlugin));
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

    fn place_factory(app: &mut App) -> Entity {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Factory,
                pos: Vec3::ZERO,
            });
        ticks(app, 2);
        let world = app.world_mut();
        world
            .query_filtered::<Entity, With<Building>>()
            .single(world)
            .unwrap()
    }

    #[test]
    fn placed_building_is_a_stalled_site_not_a_factory() {
        let mut app = app();
        let factory = place_factory(&mut app);
        // powered by hand — production must STILL not run while building
        app.world_mut().get_mut::<Powered>(factory).unwrap().0 = true;
        ticks(&mut app, 60);
        let world = app.world();
        let site = world.get::<ConstructionSite>(factory).expect("site attached");
        assert_eq!(site.phase().unwrap().kind, PhaseKind::Earthworks);
        assert_eq!(
            site.bottleneck,
            Some(Bottleneck::NoMaterial),
            "empty yard names the stall"
        );
        assert_eq!(
            world.get::<Inventory>(factory).unwrap().amount(ResourceKind::Goods),
            0.0,
            "an unfinished factory produces nothing"
        );
    }

    #[test]
    fn material_feeds_the_phase_then_the_machine_gap_names_itself() {
        let mut app = app();
        let factory = place_factory(&mut app);
        let need = {
            let site = app.world().get::<ConstructionSite>(factory).unwrap();
            site.phase().unwrap().material.unwrap().1
        };
        app.world_mut()
            .get_mut::<Inventory>(factory)
            .unwrap()
            .add(ResourceKind::Gravel, need + 1.0);
        ticks(&mut app, (need / CONSUME_RATE) as u32 + 5);
        let site = app.world().get::<ConstructionSite>(factory).unwrap();
        assert!(site.phase().unwrap().material_outstanding() < 1e-3);
        assert_eq!(
            site.bottleneck,
            Some(Bottleneck::NoMachine),
            "materials done, no fleet yet — the machine gap is the named stall"
        );
        assert_eq!(site.progress(), 0.0);
    }

    #[test]
    fn throughput_completes_phases_and_removal_activates_the_building() {
        let mut app = app();
        let factory = place_factory(&mut app);
        app.world_mut().get_mut::<Powered>(factory).unwrap().0 = true;
        {
            let mut yard = app.world_mut().get_mut::<Inventory>(factory).unwrap();
            yard.add(ResourceKind::Gravel, 30.0);
            yard.add(ResourceKind::Goods, 30.0);
        }
        // Fake B6.3's parked fleet: pump throughput every tick until done.
        let mut budget = 3000;
        loop {
            {
                let world = app.world_mut();
                if let Some(mut site) = world.get_mut::<ConstructionSite>(factory) {
                    site.throughput = 8.0;
                } else {
                    break;
                }
            }
            ticks(&mut app, 1);
            budget -= 1;
            assert!(budget > 0, "site must complete under steady throughput");
        }
        // Activated: the factory now produces like any M1 building.
        let before = app
            .world()
            .get::<Inventory>(factory)
            .unwrap()
            .amount(ResourceKind::Goods);
        ticks(&mut app, 10);
        let after = app
            .world()
            .get::<Inventory>(factory)
            .unwrap()
            .amount(ResourceKind::Goods);
        assert!(
            (after - before - 10.0 * FACTORY_GOODS_RATE).abs() < 1e-3,
            "activation opens the production gate"
        );
    }
}
