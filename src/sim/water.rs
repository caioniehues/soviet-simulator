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

use super::buildings::{Building, BuildingKind};
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
                .before(super::buildings::run_factories),
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
        && matches!(
            building.kind,
            BuildingKind::Factory | BuildingKind::Dwelling
        )
        && !has
    {
        commands.entity(add.entity).insert(Watered::default());
    }
}

/// The cycle solve: per water component, pumps pool supply and treatment
/// works pool drainage; a consumer is watered only when *both* pools cover
/// its draw — housing before industry on each side, same as the grid.
fn solve_water(
    spans: Query<&WireSpan>,
    plants: Query<(Entity, &Building), Without<Watered>>,
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
        .filter_map(|(e, b, _)| match b.kind {
            BuildingKind::Dwelling => Some((e, b.id.0, PriorityClass::Housing, DWELLING_WATER)),
            BuildingKind::Factory => Some((e, b.id.0, PriorityClass::Industry, FACTORY_WATER)),
            _ => None,
        })
        .collect();
    let supplied = allocate(
        &mut components,
        plants
            .iter()
            .filter(|(_, b)| b.kind == BuildingKind::WaterPump)
            .map(|(e, _)| (e, PUMP_SUPPLY)),
        &demands,
    );
    let drained = allocate(
        &mut components,
        plants
            .iter()
            .filter(|(_, b)| b.kind == BuildingKind::SewagePlant)
            .map(|(e, _)| (e, SEWAGE_CAPACITY)),
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
        BuildingEdit, BuildingEditQueue, BuildingSimPlugin, Powered,
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

    #[test]
    fn water_needs_both_the_pump_and_the_drain() {
        let mut app = app();
        place(&mut app, BuildingKind::WaterPump, Vec3::ZERO);
        place(&mut app, BuildingKind::Factory, Vec3::new(60.0, 0.0, 0.0));
        place(&mut app, BuildingKind::SewagePlant, Vec3::new(120.0, 0.0, 0.0));
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
        place(&mut app, BuildingKind::WaterPump, Vec3::new(-60.0, 0.0, 0.0));
        place(&mut app, BuildingKind::SewagePlant, Vec3::new(60.0, 0.0, 0.0));
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
}
