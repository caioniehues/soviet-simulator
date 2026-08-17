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

/// Below this many outstanding tonnes a phase's bill counts as satisfied —
/// float dribble from partial truckloads must never wedge a site.
pub const MATERIAL_EPS: f32 = 0.01;

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

/// Service state on a construction-machine asset: which site it works.
/// Mirrors `dispatch::FreightJob` / `transit::BusDuty`.
#[derive(Component, Debug)]
pub struct MachineDuty {
    pub site: Entity,
}

pub struct ConstructionSimPlugin;

impl Plugin for ConstructionSimPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<super::roads::RoadSimPlugin>() {
            app.add_plugins(super::roads::RoadSimPlugin);
        }
        if !app.is_plugin_added::<super::pathfinding::PathfindingSimPlugin>() {
            app.add_plugins(super::pathfinding::PathfindingSimPlugin);
        }
        if !app.is_plugin_added::<super::traffic::TrafficSimPlugin>() {
            app.add_plugins(super::traffic::TrafficSimPlugin);
        }
        app.add_observer(attach_sites)
            .add_systems(
                SimTick,
                assign_machines.in_set(SimStage::AllocationAndDispatch),
            )
            .add_systems(
                SimTick,
                run_machines
                    .in_set(SimStage::MovementAndTransfers)
                    .after(super::traffic::LanePrep),
            )
            .add_systems(
                SimTick,
                (advance_sites, update_site_policies)
                    .chain()
                    .in_set(SimStage::ProductionAndUtilities),
            );
    }
}

/// Put idle machines onto the nearest site whose *current* phase wants their
/// skill. Several machines may serve one site — throughputs add (the W&R
/// duration law); a phase with no matching machine anywhere stays stalled.
#[allow(clippy::type_complexity)]
fn assign_machines(
    mut commands: Commands,
    fleet: Query<
        (
            Entity,
            &super::vehicles::VehicleAsset,
            Has<super::vehicles::ActivePawn>,
            Has<MachineDuty>,
        ),
        super::customs::Arrived,
    >,
    sites: Query<(Entity, &Building, &ConstructionSite)>,
    buildings: Query<&Building>,
    nodes: Query<(Entity, &super::roads::RoadNode)>,
) {
    use super::vehicles::{ActiveVehicle, PawnOf, nearest_node};
    for (asset, machine, on_road, on_duty) in &fleet {
        if on_road || on_duty {
            continue;
        }
        let Some((skill, _)) = machine.kind.construction_skill() else {
            continue;
        };
        let Ok(office) = buildings.get(machine.home_depot) else {
            continue;
        };
        let target = sites
            .iter()
            .filter(|(.., site)| site.phase().is_some_and(|p| p.skill == skill))
            .map(|(e, b, _)| (e, b.pos.distance_squared(office.pos)))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| e);
        let Some(site) = target else { continue };
        // Roll out from the office's road node, like any depot trip.
        let Some(start) = nearest_node(office.pos, &nodes) else {
            continue;
        };
        let pos = nodes.get(start).unwrap().1.pos;
        commands.entity(asset).insert(MachineDuty { site });
        commands.spawn((ActiveVehicle::at(pos), PawnOf(asset)));
    }
}

/// Drive to the duty site over the B4 stack and, once parked, pour the
/// machine's skill into the site's throughput each tick. Duty ends when the
/// site completes, vanishes, or its current phase stops wanting this skill —
/// the machine dead-heads home and parks.
#[allow(clippy::too_many_arguments)]
fn run_machines(
    mut commands: Commands,
    mut pawns: Query<(Entity, &mut super::vehicles::ActiveVehicle, &super::vehicles::PawnOf)>,
    duties: Query<&MachineDuty>,
    assets: Query<&super::vehicles::VehicleAsset>,
    mut sites: Query<(&Building, &mut ConstructionSite)>,
    buildings: Query<&Building, Without<ConstructionSite>>,
    mut svc: ResMut<super::pathfinding::PathService>,
    occupancy: Res<super::traffic::LaneOccupancy>,
    nodes: Query<(Entity, &super::roads::RoadNode)>,
    segments: Query<&super::roads::RoadSegment>,
) {
    use super::dispatch::{Progress, drive_toward};
    use super::vehicles::nearest_node;
    let dt = super::clock::SECS_PER_PASS as f32;
    for (pawn, mut vehicle, pawn_of) in &mut pawns {
        let Ok(duty) = duties.get(pawn_of.0) else {
            continue;
        };
        let Ok(machine) = assets.get(pawn_of.0) else {
            continue;
        };
        let Some((skill, rate)) = machine.kind.construction_skill() else {
            continue;
        };
        let wanted = sites
            .get(duty.site)
            .ok()
            .and_then(|(b, site)| site.phase().map(|p| (b.pos, p.skill)))
            .filter(|(_, s)| *s == skill);
        if let Some((site_pos, _)) = wanted {
            let goal = || nearest_node(site_pos, &nodes);
            match drive_toward(
                pawn, &mut vehicle, goal, dt, &mut svc, &occupancy, &nodes, &segments,
            ) {
                Progress::Arrived => {
                    if let Ok((_, mut site)) = sites.get_mut(duty.site) {
                        site.throughput += rate;
                    }
                }
                Progress::Moving | Progress::NoPath => {}
            }
        } else {
            // Done here (or wrong phase): head home and park.
            let asset = pawn_of.0;
            let home = || {
                assets
                    .get(asset)
                    .ok()
                    .and_then(|a| buildings.get(a.home_depot).ok())
                    .and_then(|b| nearest_node(b.pos, &nodes))
            };
            let done = match drive_toward(
                pawn, &mut vehicle, home, dt, &mut svc, &occupancy, &nodes, &segments,
            ) {
                Progress::Arrived => true,
                Progress::NoPath => home().is_none(),
                Progress::Moving => false,
            };
            if done {
                commands.entity(pawn).despawn();
                commands.entity(pawn_of.0).remove::<MachineDuty>();
            }
        }
    }
}

/// Every building placed while phased construction is on starts as a site.
/// The site's yard is re-sized to hold the whole material bill (a depot or
/// bus stop stores nothing when finished, but its *site* receives gravel),
/// and its storage policy demands the current phase's material — the
/// ordinary dispatcher does the delivering (B6.2).
fn attach_sites(add: On<Add, Building>, mut commands: Commands, buildings: Query<&Building>) {
    let entity = add.entity;
    let Ok(building) = buildings.get(entity) else {
        return;
    };
    let site = ConstructionSite::for_kind(building.kind);
    let bill_total: f32 = site
        .phases
        .iter()
        .filter_map(|p| p.material.map(|(_, n)| n))
        .sum();
    // A truckload of headroom so one phase's rounding overshoot can never
    // block the next phase's material out of the shared yard.
    commands.entity(entity).insert((
        site,
        Inventory::new(bill_total.max(1.0) + super::vehicles::TRUCK_CARGO_CAPACITY),
    ));
}

/// Keep the site's storage policy pointed at the current phase's material,
/// demanding only what the phase still needs — stock and the outstanding
/// bill drain in lockstep as the works consume, so total deliveries converge
/// on the bill instead of filling the yard. Idempotent per tick.
fn update_site_policies(
    mut sites: Query<(&ConstructionSite, &Inventory, &mut super::storage::StoragePolicies)>,
) {
    for (site, yard, mut policies) in &mut sites {
        let wanted = site.phase().and_then(|p| {
            p.material
                .map(|(r, _)| (r, p.material_outstanding()))
        });
        for resource in ResourceKind::ALL {
            let band = match wanted {
                Some((r, outstanding)) if r == resource && outstanding > MATERIAL_EPS => {
                    let frac = (outstanding / yard.capacity.max(1e-3)).min(1.0);
                    Some(super::storage::StorageBand::new(frac, frac))
                }
                _ => None,
            };
            if policies.band(resource).map(|b| (b.min_pct, b.max_pct))
                != band.map(|b| (b.min_pct, b.max_pct))
            {
                policies.set(resource, band);
            }
        }
    }
}

/// The site tick: fold yard material into the current phase, then apply the
/// machine throughput B6.3's fleet parked here this pass. A missing factor
/// stalls the phase with its name — never a silent wait.
fn advance_sites(
    mut commands: Commands,
    mut sites: Query<(Entity, &Building, &mut ConstructionSite, &mut Inventory)>,
) {
    for (entity, building, mut site, mut yard) in &mut sites {
        // The construction yard becomes the working yard on activation:
        // fresh capacity for the kind, default policy bands restored.
        let activate = |commands: &mut Commands| {
            commands.entity(entity).remove::<ConstructionSite>();
            commands.entity(entity).insert((
                Inventory::new(building.kind.inventory_capacity()),
                super::storage::default_policies(building.kind),
            ));
        };
        let throughput = site.throughput;
        site.throughput = 0.0;
        let current = site.current;
        let Some(phase) = site.phases.get_mut(current) else {
            activate(&mut commands);
            continue;
        };
        // Materials first: the works absorb what the yard holds.
        let mut stall = None;
        if let Some((resource, need)) = phase.material
            && need - phase.consumed > MATERIAL_EPS
        {
            let taken = yard.take(resource, CONSUME_RATE.min(need - phase.consumed));
            phase.consumed += taken;
            if need - phase.consumed > MATERIAL_EPS {
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
        if finished && let Some((resource, _)) = phase.material {
            // Whatever excess landed on site is graded into the works — a
            // finished phase never squats the shared yard against the next
            // phase's deliveries.
            yard.take(resource, f32::INFINITY);
        }
        site.bottleneck = stall;
        if finished {
            site.current += 1;
            if site.complete() {
                // Activation: the component's removal is the gate opening.
                activate(&mut commands);
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
        assert!(site.phase().unwrap().material_outstanding() <= MATERIAL_EPS);
        assert_eq!(
            site.bottleneck,
            Some(Bottleneck::NoMachine),
            "materials done, no fleet yet — the machine gap is the named stall"
        );
        assert_eq!(site.progress(), 0.0);
    }

    #[test]
    fn dispatcher_hauls_the_phase_bill_to_the_site() {
        use super::super::dispatch::{DeficitBoard, DispatchSimPlugin};
        use super::super::resources::TransportClass;
        use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadSimPlugin};
        use super::super::storage::StorageSimPlugin;
        use super::super::vehicles::{VehicleEdit, VehicleEditQueue, VehicleSimPlugin};
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        app.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            StorageSimPlugin,
            VehicleSimPlugin,
            DispatchSimPlugin,
            ConstructionSimPlugin,
        ));
        // Quarry (gravel supplier) west, the new factory site east, depot
        // with one bulk truck. The site itself orders its earthworks bill.
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            for (kind, pos) in [
                (BuildingKind::Quarry, Vec3::new(-10.0, 0.0, 0.0)),
                (BuildingKind::Factory, Vec3::new(110.0, 0.0, 0.0)),
                (BuildingKind::Depot, Vec3::new(0.0, 0.0, 15.0)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut found = (None, None, None);
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::Quarry => found.0 = Some(e),
                BuildingKind::Factory => found.1 = Some(e),
                BuildingKind::Depot => found.2 = Some(e),
                _ => {}
            }
        }
        let (quarry, factory, depot) = (found.0.unwrap(), found.1.unwrap(), found.2.unwrap());
        // The quarry site completes by fiat so it can supply: strip its site
        // and stock it (quarries under construction don't extract).
        // Quarry and depot complete by fiat (empty quarry: no supplier yet).
        world.entity_mut(quarry).remove::<ConstructionSite>();
        world.entity_mut(quarry).insert((
            Inventory::new(60.0),
            super::super::storage::default_policies(BuildingKind::Quarry),
        ));
        world.entity_mut(depot).remove::<ConstructionSite>();
        world
            .entity_mut(depot)
            .insert(super::super::storage::default_policies(BuildingKind::Depot));
        world
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyTruck {
                depot,
                class: TransportClass::Bulk,
            });
        // No gravel anywhere: the site's demand starves visibly.
        ticks(&mut app, 100);
        let starving = app
            .world()
            .resource::<DeficitBoard>()
            .0
            .iter()
            .any(|d| d.building == factory && d.resource == ResourceKind::Gravel);
        assert!(starving, "an unsupplied site sits on the deficit board");
        // Stock the quarry: the ordinary dispatcher takes it from here.
        app.world_mut()
            .get_mut::<Inventory>(quarry)
            .unwrap()
            .add(ResourceKind::Gravel, 50.0);
        // Two round trips on the compressed freight timescale: the quarry's
        // own extraction trickle costs the truck a first small haul.
        ticks(&mut app, 5200);
        let site = app.world().get::<ConstructionSite>(factory).unwrap();
        assert!(
            site.phase().unwrap().material_outstanding() <= MATERIAL_EPS,
            "the dispatcher must land the earthworks bill, outstanding = {}",
            site.phase().unwrap().material_outstanding()
        );
        assert_eq!(
            site.bottleneck,
            Some(Bottleneck::NoMachine),
            "material satisfied — the stall names the missing machine"
        );
    }

    /// Road 0→100, office west, factory site east with its yard pre-stocked
    /// (delivery is B6.2's test); returns (factory, office).
    fn machine_world(app: &mut App) -> (Entity, Entity) {
        use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue};
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        {
            let mut buildings = app.world_mut().resource_mut::<BuildingEditQueue>();
            for (kind, pos) in [
                (BuildingKind::ConstructionOffice, Vec3::new(0.0, 0.0, 20.0)),
                (BuildingKind::Factory, Vec3::new(110.0, 0.0, 0.0)),
            ] {
                buildings.0.push(BuildingEdit::Place { kind, pos });
            }
        }
        ticks(app, 2);
        let world = app.world_mut();
        let mut office = None;
        let mut factory = None;
        let mut q = world.query::<(Entity, &Building)>();
        for (e, b) in q.iter(world) {
            match b.kind {
                BuildingKind::ConstructionOffice => office = Some(e),
                BuildingKind::Factory => factory = Some(e),
                _ => {}
            }
        }
        let (office, factory) = (office.unwrap(), factory.unwrap());
        // The office completes by fiat; the factory's yard holds its bill.
        world.entity_mut(office).remove::<ConstructionSite>();
        {
            let mut yard = world.get_mut::<Inventory>(factory).unwrap();
            yard.add(ResourceKind::Gravel, 10.0);
            yard.add(ResourceKind::Goods, 12.0);
        }
        (factory, office)
    }

    fn machine_app() -> App {
        use super::super::roads::RoadSimPlugin;
        use super::super::vehicles::VehicleSimPlugin;
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            BuildingSimPlugin,
            VehicleSimPlugin,
            ConstructionSimPlugin,
        ));
        a
    }

    #[test]
    fn demolition_cancels_a_site_and_releases_its_machine() {
        use super::super::buildings::BuildingEdit;
        use super::super::vehicles::{VehicleEdit, VehicleEditQueue, VehicleKind};
        let mut app = machine_app();
        let (factory, office) = machine_world(&mut app);
        app.world_mut()
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyMachine {
                office,
                kind: VehicleKind::Excavator,
            });
        ticks(&mut app, 400); // excavator en route to the site
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Demolish { building: factory });
        ticks(&mut app, 2500); // machine notices, dead-heads home, parks
        let world = app.world_mut();
        assert!(world.get_entity(factory).is_err(), "site physically gone");
        assert_eq!(
            world
                .query::<&super::super::vehicles::ActiveVehicle>()
                .iter(world)
                .count(),
            0,
            "the machine returns to its office slot"
        );
        assert_eq!(world.query::<&MachineDuty>().iter(world).count(), 0);
    }

    #[test]
    fn machines_work_the_phases_and_the_building_activates() {
        use super::super::vehicles::{VehicleEdit, VehicleEditQueue, VehicleKind};
        let mut app = machine_app();
        let (factory, office) = machine_world(&mut app);
        {
            let mut edits = app.world_mut().resource_mut::<VehicleEditQueue>();
            edits.0.push(VehicleEdit::BuyMachine {
                office,
                kind: VehicleKind::Excavator,
            });
            edits.0.push(VehicleEdit::BuyMachine {
                office,
                kind: VehicleKind::Crane,
            });
        }
        ticks(&mut app, 6500);
        assert!(
            app.world().get::<ConstructionSite>(factory).is_none(),
            "excavator then crane must carry the site to activation"
        );
        let world = app.world_mut();
        assert_eq!(
            world
                .query::<&super::super::vehicles::ActiveVehicle>()
                .iter(world)
                .count(),
            0,
            "machines dead-head home and park after the job"
        );
        assert_eq!(
            world
                .query::<&super::super::vehicles::VehicleAsset>()
                .iter(world)
                .count(),
            2,
            "the fleet survives, parked at the office"
        );
    }

    #[test]
    fn missing_crane_stalls_the_structure_phase_by_name() {
        use super::super::vehicles::{VehicleEdit, VehicleEditQueue, VehicleKind};
        let mut app = machine_app();
        let (factory, office) = machine_world(&mut app);
        app.world_mut()
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyMachine {
                office,
                kind: VehicleKind::Excavator,
            });
        ticks(&mut app, 3000); // drive ~900 + earthworks 400 + margin
        let site = app.world().get::<ConstructionSite>(factory).unwrap();
        assert_eq!(site.phase().unwrap().kind, PhaseKind::Structure);
        assert_eq!(
            site.bottleneck,
            Some(Bottleneck::NoMachine),
            "no crane anywhere — the structure phase stalls with its name"
        );
    }

    #[test]
    fn throughput_completes_phases_and_removal_activates_the_building() {
        let mut app = app();
        let factory = place_factory(&mut app);
        app.world_mut().get_mut::<Powered>(factory).unwrap().0 = true;
        // Fake B6.3's parked fleet: pump throughput every tick until done,
        // topping the yard up with whatever the current phase bills.
        let mut budget = 3000;
        loop {
            {
                let world = app.world_mut();
                let refill = match world.get::<ConstructionSite>(factory) {
                    Some(site) => site.phase().and_then(|p| p.material.map(|(r, _)| r)),
                    None => break,
                };
                if let Some(resource) = refill {
                    world
                        .get_mut::<Inventory>(factory)
                        .unwrap()
                        .add(resource, 1.0);
                }
                let mut site = world.get_mut::<ConstructionSite>(factory).unwrap();
                site.throughput = 8.0;
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
