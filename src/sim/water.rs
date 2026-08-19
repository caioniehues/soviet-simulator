//! Water ⇄ sewage, one cycle (B8.2, spec/water.md + spec/sewage.md): water
//! flows only where a pipe web physically connects a consumer to a pump
//! with spare supply **and** a treatment works with spare drainage — a
//! backed-up drain shuts consumers exactly like a dry main. Both sides run
//! on the shared pool solver over `NetKind::Water` spans. Quality grading
//! and pipe-or-tanker arrive with the wider resource tree.
//!
//! Opt-in like construction: without this plugin no `Watered` component
//! exists, and production treats water as not-yet-required.

use bevy::prelude::*;

use super::buildings::Building;
use super::catalogue::{WaterDemand, spec};
use super::network::{Components, PriorityClass, allocate};
use super::stages::{SimStage, SimTick};
use super::wires::{NetKind, WireSpan};

/// Units a pump pours into its component per tick.
pub const PUMP_SUPPLY: f32 = 20.0;
/// Drainage units a treatment works absorbs per tick.
pub const SEWAGE_CAPACITY: f32 = 20.0;
pub const DWELLING_WATER: f32 = 1.0;
pub const FACTORY_WATER: f32 = 2.0;

/// Whether the building's water gate holds this tick: supplied *and*
/// drained. Presence of the component is what makes water a requirement.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Watered(pub bool);

pub struct WaterSimPlugin;

impl Plugin for WaterSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(attach_watered).add_systems(
            SimTick,
            solve_water
                .in_set(SimStage::ProductionAndUtilities)
                .before(super::production::produce_goods),
        );
    }
}

/// Water consumers gain their gate the moment they exist (Staffing pattern).
fn attach_watered(
    add: On<Add, Building>,
    buildings: Query<(&Building, Has<Watered>)>,
    mut commands: Commands,
) {
    if let Ok((building, has)) = buildings.get(add.entity)
        && matches!(spec(building.kind).water, Some(WaterDemand::Draws(_)))
        && !has
    {
        commands.entity(add.entity).insert(Watered::default());
    }
}

/// The cycle solve: per water component, pumps pool supply and treatment
/// works pool drainage; a consumer is watered only when *both* pools cover
/// its draw — housing before industry on each side, same as the grid.
#[allow(clippy::type_complexity)]
fn solve_water(
    spans: Query<&WireSpan>,
    // `Without<Watered>` selects the non-consumers; `Without<ConstructionSite>`
    // is the inertness contract — a pump still on the scaffolding pours
    // nothing into the net. The production pass carries this filter
    // structurally, but a supplier has no recipe, so it never passes through
    // there and needs the filter stated here.
    plants: Query<
        (Entity, &Building),
        (
            Without<Watered>,
            Without<super::construction::ConstructionSite>,
        ),
    >,
    mut consumers: Query<(Entity, &Building, &mut Watered)>,
) {
    let mut components = Components::from_spans(
        spans
            .iter()
            .filter(|s| s.kind == NetKind::Water)
            .map(|s| (s.a, s.b)),
    );
    let demands: Vec<(Entity, u64, PriorityClass, f32)> = consumers
        .iter()
        .filter_map(|(e, b, _)| match spec(b.kind).water {
            Some(WaterDemand::Draws(demand)) => Some((e, b.id.0, demand.priority, demand.rate)),
            _ => None,
        })
        .collect();
    let supplied = allocate(
        &mut components,
        plants.iter().filter_map(|(e, b)| match spec(b.kind).water {
            Some(WaterDemand::Supplies(rate)) => Some((e, rate)),
            _ => None,
        }),
        &demands,
    );
    let drained = allocate(
        &mut components,
        plants.iter().filter_map(|(e, b)| match spec(b.kind).water {
            Some(WaterDemand::Drains(rate)) => Some((e, rate)),
            _ => None,
        }),
        &demands,
    );
    for (entity, _, mut watered) in &mut consumers {
        let ok = supplied.get(&entity).copied().unwrap_or(false)
            && drained.get(&entity).copied().unwrap_or(false);
        if watered.0 != ok {
            watered.0 = ok;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin, Powered,
    };
    use super::super::resources::{Inventory, ResourceKind};
    use super::super::wires::{WireEdit, WireEditQueue, WireSimPlugin};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, WireSimPlugin, WaterSimPlugin));
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

    fn place(app: &mut App, kind: BuildingKind, pos: Vec3) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place { kind, pos });
    }

    fn pipe(app: &mut App, from: Vec3, to: Vec3) {
        app.world_mut()
            .resource_mut::<WireEditQueue>()
            .0
            .push(WireEdit::Place {
                from,
                to,
                kind: NetKind::Water,
            });
    }

    fn entity_of(app: &mut App, kind: BuildingKind) -> Entity {
        let world = app.world_mut();
        world
            .query::<(Entity, &Building)>()
            .iter(world)
            .find(|(_, b)| b.kind == kind)
            .unwrap()
            .0
    }

    /// Every entity of `kind`, oldest first. `allocate`'s tie-break is by
    /// `BuildingId`, not query order, so the boundary test below needs this
    /// rather than `entity_of` once more than one instance exists.
    fn entities_of(app: &mut App, kind: BuildingKind) -> Vec<Entity> {
        let world = app.world_mut();
        let mut found: Vec<(u64, Entity)> = world
            .query::<(Entity, &Building)>()
            .iter(world)
            .filter(|(_, b)| b.kind == kind)
            .map(|(e, b)| (b.id.0, e))
            .collect();
        found.sort_by_key(|(id, _)| *id);
        found.into_iter().map(|(_, e)| e).collect()
    }

    #[test]
    fn water_needs_both_the_pump_and_the_drain() {
        let mut app = app();
        place(&mut app, BuildingKind::WaterPump, Vec3::ZERO);
        place(&mut app, BuildingKind::Factory, Vec3::new(60.0, 0.0, 0.0));
        place(
            &mut app,
            BuildingKind::SewagePlant,
            Vec3::new(120.0, 0.0, 0.0),
        );
        ticks(&mut app, 2);
        // pump → factory only: supplied but nowhere to drain
        pipe(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        ticks(&mut app, 2);
        let factory = entity_of(&mut app, BuildingKind::Factory);
        assert_eq!(
            app.world().get::<Watered>(factory).unwrap().0,
            false,
            "a backed-up drain shuts the consumer"
        );
        // connect the treatment works: the cycle closes
        pipe(
            &mut app,
            Vec3::new(60.0, 0.0, 0.0),
            Vec3::new(120.0, 0.0, 0.0),
        );
        ticks(&mut app, 2);
        assert!(app.world().get::<Watered>(factory).unwrap().0);
    }

    #[test]
    fn dry_factory_produces_nothing_until_the_cycle_closes() {
        use super::super::wires::{PoleId, SpanId, WirePole};
        // No WireSimPlugin here: the grid solve would keep zeroing our
        // hand-set power gate. Spans are spawned directly.
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, WaterSimPlugin));
        let mut app = a;
        place(&mut app, BuildingKind::Factory, Vec3::ZERO);
        place(
            &mut app,
            BuildingKind::WaterPump,
            Vec3::new(-60.0, 0.0, 0.0),
        );
        place(
            &mut app,
            BuildingKind::SewagePlant,
            Vec3::new(60.0, 0.0, 0.0),
        );
        ticks(&mut app, 2);
        let factory = entity_of(&mut app, BuildingKind::Factory);
        app.world_mut().get_mut::<Powered>(factory).unwrap().0 = true;
        ticks(&mut app, 30);
        assert_eq!(
            app.world()
                .get::<Inventory>(factory)
                .unwrap()
                .amount(ResourceKind::Goods),
            0.0,
            "power alone is not enough once water is a factor"
        );
        let pump = entity_of(&mut app, BuildingKind::WaterPump);
        let sewage = entity_of(&mut app, BuildingKind::SewagePlant);
        let world = app.world_mut();
        let _hub = world
            .spawn(WirePole {
                id: PoleId(1),
                pos: Vec3::new(0.0, 0.0, 20.0),
                kind: NetKind::Water,
            })
            .id();
        world.spawn(WireSpan {
            id: SpanId(1),
            a: pump,
            b: factory,
            kind: NetKind::Water,
        });
        world.spawn(WireSpan {
            id: SpanId(2),
            a: factory,
            b: sewage,
            kind: NetKind::Water,
        });
        ticks(&mut app, 10);
        let goods = app
            .world()
            .get::<Inventory>(factory)
            .unwrap()
            .amount(ResourceKind::Goods);
        assert!(goods > 0.0, "the closed cycle opens the water gate");
    }

    /// The other half of the inertness gap `run_heat_plants` had: `solve_water`
    /// gathers pumps with `Without<Watered>` and nothing else, so a pump still
    /// on the scaffolding poured its full `PUMP_SUPPLY` into the net. Phase 4
    /// of the catalogue refactor makes the `ConstructionSite` filter structural.
    #[test]
    fn a_pump_under_construction_supplies_nothing() {
        let mut app = app();
        place(&mut app, BuildingKind::WaterPump, Vec3::ZERO);
        place(&mut app, BuildingKind::Factory, Vec3::new(60.0, 0.0, 0.0));
        place(
            &mut app,
            BuildingKind::SewagePlant,
            Vec3::new(120.0, 0.0, 0.0),
        );
        ticks(&mut app, 2);
        pipe(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        pipe(
            &mut app,
            Vec3::new(60.0, 0.0, 0.0),
            Vec3::new(120.0, 0.0, 0.0),
        );
        ticks(&mut app, 2);
        let factory = entity_of(&mut app, BuildingKind::Factory);
        assert!(
            app.world().get::<Watered>(factory).unwrap().0,
            "baseline: the closed cycle waters the factory"
        );
        // put the pump back on the scaffolding — the cycle loses its source
        let pump = entity_of(&mut app, BuildingKind::WaterPump);
        app.world_mut().entity_mut(pump).insert(
            super::super::construction::ConstructionSite::for_kind(BuildingKind::WaterPump),
        );
        ticks(&mut app, 2);
        assert!(
            !app.world().get::<Watered>(factory).unwrap().0,
            "a site is not a pump: it must not supply the net while it is still being built"
        );
    }

    /// The water analogue of `wires.rs`'s power witness. Power has a free
    /// per-test knob (a bare `PowerOutput` entity) because `solve_power`
    /// never kind-matches its producer; water's pump/sewage rates are
    /// catalogue constants (`PUMP_SUPPLY`/`SEWAGE_CAPACITY`) fixed at 20.0,
    /// so the same "exact rate, then 1% short" trick has no lever to pull on
    /// the supply side. The lever that does exist: place exactly
    /// `pool / rate` consumers of `kind` on one pump+sewage cycle — the pool
    /// covers all of them with nothing to spare — then one more. If the
    /// solve pass is reading anything but `kind`'s claimed rate off the
    /// catalogue, either the first batch comes up short or the extra one
    /// slips through served.
    #[test]
    fn the_cycle_serves_exactly_the_draw_the_water_column_claims() {
        for kind in BuildingKind::ALL {
            let Some(WaterDemand::Draws(demand)) = spec(kind).water else {
                let mut app = app();
                place(&mut app, kind, Vec3::ZERO);
                ticks(&mut app, 2);
                let building = entity_of(&mut app, kind);
                assert!(
                    app.world().get::<Watered>(building).is_none(),
                    "{kind:?} claims no water draw, so it must carry no gate"
                );
                continue;
            };
            let pool = PUMP_SUPPLY.min(SEWAGE_CAPACITY);
            let served_count = (pool / demand.rate).floor() as usize;

            let mut app = app();
            let pump_pos = Vec3::new(-1_000.0, 0.0, 0.0);
            let sewage_pos = Vec3::new(1_000.0, 0.0, 0.0);
            place(&mut app, BuildingKind::WaterPump, pump_pos);
            place(&mut app, BuildingKind::SewagePlant, sewage_pos);
            for i in 0..served_count + 1 {
                let pos = Vec3::new(60.0 * i as f32, 0.0, 0.0);
                place(&mut app, kind, pos);
            }
            ticks(&mut app, 2);
            for i in 0..served_count + 1 {
                let pos = Vec3::new(60.0 * i as f32, 0.0, 0.0);
                pipe(&mut app, pump_pos, pos);
                pipe(&mut app, pos, sewage_pos);
            }
            ticks(&mut app, 2);

            let consumers = entities_of(&mut app, kind);
            assert_eq!(consumers.len(), served_count + 1);
            for &c in &consumers[..served_count] {
                assert!(
                    app.world().get::<Watered>(c).unwrap().0,
                    "{kind:?}: a pool of exactly {served_count} × its claimed {} must serve all {served_count}",
                    demand.rate
                );
            }
            assert!(
                !app.world().get::<Watered>(consumers[served_count]).unwrap().0,
                "{kind:?}: the pool has nothing left over the first {served_count}, \
                 so one more at its claimed {} must go unserved",
                demand.rate
            );
        }
    }
}
