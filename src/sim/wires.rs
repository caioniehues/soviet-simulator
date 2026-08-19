//! Electricity stage 1 (ticket #14, spec/electricity.md under M1 charter Q11):
//! a single generic wire class, player-laid hops, no tiers/transformers/loss
//! (voltage tiers deferred to B10). A building has power only if physically
//! wired — through hops — to a plant with spare capacity. No coverage field.
//! Deficit allocation runs on the shared pool solver (B8.1,
//! `sim/network.rs`): housing before industry, brownout before blackout.

use bevy::prelude::*;

use super::buildings::{Building, PowerOutput, Powered};
use super::stages::{ApplierOrder, SimStage, SimTick};

/// Endpoint clicks snap to an existing pole within this radius.
pub const POLE_SNAP_RADIUS: f32 = 4.0;
/// Endpoint clicks snap to a building whose centre is within its footprint
/// half-diagonal plus this margin.
pub const BUILDING_SNAP_MARGIN: f32 = 2.0;

/// Which utility a pole/span carries (B8.2): the same laid-hop hardware
/// serves all three webs; components never mix kinds.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum NetKind {
    #[default]
    Power,
    Water,
    Heat,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PoleId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SpanId(pub u64);

/// Free-standing wire support created wherever an endpoint snaps to nothing.
#[derive(Component, Debug)]
pub struct WirePole {
    pub id: PoleId,
    pub pos: Vec3,
    pub kind: NetKind,
}

/// One laid hop. Endpoints are poles or buildings.
#[derive(Component, Debug)]
pub struct WireSpan {
    pub id: SpanId,
    pub a: Entity,
    pub b: Entity,
    pub kind: NetKind,
}

#[derive(Resource, Default)]
pub struct WireEditQueue(pub Vec<WireEdit>);

#[derive(Clone, Copy, Debug)]
pub enum WireEdit {
    /// Each endpoint snaps to a building, else an existing same-kind pole,
    /// else creates a new pole at the position.
    Place { from: Vec3, to: Vec3, kind: NetKind },
    /// Remove the span nearest to `pos` (within `POLE_SNAP_RADIUS` of its line).
    RemoveNear { pos: Vec3 },
}

#[derive(Resource, Default)]
pub struct WireIds {
    pub next_pole: u64,
    pub next_span: u64,
}

pub struct WireSimPlugin;

impl Plugin for WireSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WireEditQueue>()
            .init_resource::<WireIds>()
            .add_systems(SimTick, apply_wire_edits.in_set(ApplierOrder::Wires))
            .add_systems(
                SimTick,
                // Grid solve sits between generation and consumption inside
                // the buildings chain: plants have this tick's output, the
                // factory gate is set before it is read.
                solve_power
                    .in_set(SimStage::ProductionAndUtilities)
                    .after(super::production::produce_flows)
                    .before(super::production::produce_goods),
            );
    }
}

fn endpoint_at(world: &mut World, pos: Vec3, kind: NetKind) -> Entity {
    // building first: wiring a yard beats planting a pole inside it
    let mut buildings = world.query::<(Entity, &Building)>();
    let hit = buildings
        .iter(world)
        .map(|(e, b)| {
            let reach = b.kind.footprint().length() * 0.5 + BUILDING_SNAP_MARGIN;
            (e, b.pos.distance_squared(pos), reach)
        })
        .filter(|(_, d2, reach)| *d2 <= reach * reach)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, ..)| e);
    if let Some(building) = hit {
        return building;
    }
    let mut poles = world.query::<(Entity, &WirePole)>();
    let snapped = super::node_snap::nearest_node_within(
        poles
            .iter(world)
            .filter(|(_, p)| p.kind == kind)
            .map(|(e, p)| (e, p.pos)),
        pos,
        POLE_SNAP_RADIUS,
    );
    snapped.unwrap_or_else(|| {
        let id = {
            let mut ids = world.resource_mut::<WireIds>();
            ids.next_pole += 1;
            PoleId(ids.next_pole)
        };
        world.spawn(WirePole { id, pos, kind }).id()
    })
}

fn endpoint_pos(world: &World, e: Entity) -> Option<Vec3> {
    world
        .get::<WirePole>(e)
        .map(|p| p.pos)
        .or_else(|| world.get::<Building>(e).map(|b| b.pos))
}

fn apply_wire_edits(world: &mut World) {
    let edits = std::mem::take(&mut world.resource_mut::<WireEditQueue>().0);
    for edit in edits {
        match edit {
            WireEdit::Place { from, to, kind } => {
                let a = endpoint_at(world, from, kind);
                let b = endpoint_at(world, to, kind);
                if a == b {
                    continue;
                }
                let mut spans = world.query::<&WireSpan>();
                let duplicate = spans
                    .iter(world)
                    .any(|s| s.kind == kind && ((s.a == a && s.b == b) || (s.a == b && s.b == a)));
                if duplicate {
                    continue;
                }
                let id = {
                    let mut ids = world.resource_mut::<WireIds>();
                    ids.next_span += 1;
                    SpanId(ids.next_span)
                };
                world.spawn(WireSpan { id, a, b, kind });
            }
            WireEdit::RemoveNear { pos } => remove_span_near(world, pos),
        }
    }
}

fn remove_span_near(world: &mut World, pos: Vec3) {
    let mut spans = world.query::<(Entity, &WireSpan)>();
    let candidates: Vec<(Entity, Entity, Entity)> =
        spans.iter(world).map(|(e, s)| (e, s.a, s.b)).collect();
    let hit = super::node_snap::nearest_edge_within(
        candidates.iter().filter_map(|&(e, a, b)| {
            let (pa, pb) = (endpoint_pos(world, a)?, endpoint_pos(world, b)?);
            Some((e, pa, pb))
        }),
        pos,
        POLE_SNAP_RADIUS,
    );
    let Some(span) = hit else { return };
    let (_, a, b) = *candidates.iter().find(|(e, ..)| *e == span).unwrap();
    world.despawn(span);
    // orphaned poles go with their last span; buildings stay
    for endpoint in [a, b] {
        if world.get::<WirePole>(endpoint).is_none() {
            continue;
        }
        let mut spans = world.query::<&WireSpan>();
        let attached = spans
            .iter(world)
            .any(|s| s.a == endpoint || s.b == endpoint);
        if !attached {
            world.despawn(endpoint);
        }
    }
}

/// Connectivity + plant-capacity check over the wire graph. Union-find over
/// span endpoints; per component, plants pool this tick's output and factories
/// draw in ascending BuildingId order — a starved plant (no coal ⇒ zero
/// output) blacks out its dependents the same tick.
/// The grid solve, now on the shared pool solver (B8.1): homes rank above
/// industry, so a starved component browns factories out before dwellings —
/// deficit allocation is planner policy, never id order.
fn solve_power(
    spans: Query<&WireSpan>,
    outputs: Query<(Entity, &PowerOutput)>,
    // A site is inert on the consumer side too: scaffolding must never draw
    // demand a finished building then goes without (the B6 contract).
    mut consumers: Query<
        (Entity, &Building, &mut Powered),
        Without<super::construction::ConstructionSite>,
    >,
) {
    use super::catalogue::spec;
    use super::network::{Components, PriorityClass, allocate};
    let mut components = Components::from_spans(
        spans
            .iter()
            .filter(|s| s.kind == NetKind::Power)
            .map(|s| (s.a, s.b)),
    );
    let demands: Vec<(Entity, u64, PriorityClass, f32)> = consumers
        .iter()
        .filter_map(|(e, b, _)| {
            spec(b.kind)
                .power
                .map(|demand| (e, b.id.0, demand.priority, demand.rate))
        })
        .collect();
    let served = allocate(
        &mut components,
        outputs.iter().map(|(e, o)| (e, o.0)),
        &demands,
    );
    for (entity, _, mut powered) in &mut consumers {
        if let Some(&ok) = served.get(&entity)
            && powered.0 != ok
        {
            powered.0 = ok;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{
        BuildingEdit, BuildingEditQueue, BuildingKind, BuildingSimPlugin, FACTORY_DEMAND_MW,
        FACTORY_GOODS_RATE, PLANT_OUTPUT_MW,
    };
    use super::super::resources::{Inventory, ResourceKind};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, WireSimPlugin));
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

    fn place_building(app: &mut App, kind: BuildingKind, pos: Vec3) {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place { kind, pos });
    }

    fn wire(app: &mut App, from: Vec3, to: Vec3) {
        app.world_mut()
            .resource_mut::<WireEditQueue>()
            .0
            .push(WireEdit::Place {
                from,
                to,
                kind: NetKind::Power,
            });
    }

    fn building_entity(app: &mut App, kind: BuildingKind) -> Entity {
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        q.iter(world).find(|(_, b)| b.kind == kind).unwrap().0
    }

    fn fuel_plant(app: &mut App, tonnes: f32) {
        let plant = building_entity(app, BuildingKind::PowerPlant);
        app.world_mut()
            .get_mut::<Inventory>(plant)
            .unwrap()
            .add(ResourceKind::Coal, tonnes);
    }

    #[test]
    fn wired_factory_is_powered_and_produces() {
        let mut app = app();
        place_building(&mut app, BuildingKind::PowerPlant, Vec3::ZERO);
        place_building(&mut app, BuildingKind::Factory, Vec3::new(120.0, 0.0, 0.0));
        ticks(&mut app, 2);
        fuel_plant(&mut app, 10.0);
        // two hops through a mid-field pole
        wire(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        wire(
            &mut app,
            Vec3::new(60.0, 0.0, 0.0),
            Vec3::new(120.0, 0.0, 0.0),
        );
        // Wire edits are exclusive-world writes at the tick-opening barrier,
        // so the gate opens the same tick: all 11 frames produce.
        ticks(&mut app, 11);
        let world = app.world_mut();
        let factory = world
            .query::<(&Building, &Inventory)>()
            .iter(world)
            .find(|(b, _)| b.kind == BuildingKind::Factory)
            .unwrap()
            .1;
        let goods = factory.amount(ResourceKind::Goods);
        assert!(
            (goods - 11.0 * FACTORY_GOODS_RATE).abs() < 1e-3,
            "goods = {goods}"
        );
        // exactly one pole was created and reused by the second hop
        assert_eq!(world.query::<&WirePole>().iter(world).count(), 1);
        assert_eq!(world.query::<&WireSpan>().iter(world).count(), 2);
    }

    #[test]
    fn unwired_factory_stays_dark() {
        let mut app = app();
        place_building(&mut app, BuildingKind::PowerPlant, Vec3::ZERO);
        place_building(&mut app, BuildingKind::Factory, Vec3::new(120.0, 0.0, 0.0));
        ticks(&mut app, 2);
        fuel_plant(&mut app, 10.0);
        ticks(&mut app, 20);
        let world = app.world_mut();
        let powered = world.query::<&Powered>().single(world).unwrap();
        assert!(!powered.0);
    }

    #[test]
    fn capacity_check_powers_only_what_the_plant_can_carry() {
        assert!(PLANT_OUTPUT_MW / FACTORY_DEMAND_MW < 3.0);
        let mut app = app();
        place_building(&mut app, BuildingKind::PowerPlant, Vec3::ZERO);
        for i in 0..3 {
            place_building(
                &mut app,
                BuildingKind::Factory,
                Vec3::new(80.0, 0.0, 60.0 * i as f32),
            );
        }
        ticks(&mut app, 2);
        fuel_plant(&mut app, 10.0);
        for i in 0..3 {
            wire(&mut app, Vec3::ZERO, Vec3::new(80.0, 0.0, 60.0 * i as f32));
        }
        ticks(&mut app, 3);
        let world = app.world_mut();
        let lit = world
            .query::<&Powered>()
            .iter(world)
            .filter(|p| p.0)
            .count();
        assert_eq!(lit, 2, "10 MW carries exactly two 4 MW factories");
    }

    #[test]
    fn coal_starvation_blacks_out_the_grid() {
        let mut app = app();
        place_building(&mut app, BuildingKind::PowerPlant, Vec3::ZERO);
        place_building(&mut app, BuildingKind::Factory, Vec3::new(120.0, 0.0, 0.0));
        ticks(&mut app, 2);
        fuel_plant(&mut app, 0.1); // 5 frames of burn at 0.02 t/frame
        wire(&mut app, Vec3::ZERO, Vec3::new(120.0, 0.0, 0.0));
        ticks(&mut app, 3);
        {
            let world = app.world_mut();
            assert!(world.query::<&Powered>().single(world).unwrap().0);
        }
        ticks(&mut app, 10); // coal gone: output 0 ⇒ blackout
        let world = app.world_mut();
        assert!(!world.query::<&Powered>().single(world).unwrap().0);
    }

    #[test]
    fn removing_the_span_despawns_orphan_poles_but_not_buildings() {
        let mut app = app();
        place_building(&mut app, BuildingKind::PowerPlant, Vec3::ZERO);
        ticks(&mut app, 2);
        wire(&mut app, Vec3::ZERO, Vec3::new(60.0, 0.0, 0.0));
        ticks(&mut app, 1);
        {
            let world = app.world_mut();
            assert_eq!(world.query::<&WirePole>().iter(world).count(), 1);
        }
        app.world_mut()
            .resource_mut::<WireEditQueue>()
            .0
            .push(WireEdit::RemoveNear {
                pos: Vec3::new(30.0, 0.0, 0.0),
            });
        ticks(&mut app, 1);
        let world = app.world_mut();
        assert_eq!(world.query::<&WireSpan>().iter(world).count(), 0);
        assert_eq!(world.query::<&WirePole>().iter(world).count(), 0);
        assert_eq!(world.query::<&Building>().iter(world).count(), 1);
    }

    #[test]
    fn starved_grid_serves_homes_before_factories() {
        use super::super::buildings::DWELLING_DEMAND_MW;
        let mut app = app();
        // One plant (10 MW when fuelled), three factories (4 MW each) and two
        // dwellings (1 MW each) on one grid: 14 demanded vs 10 supplied.
        // Housing serves first; two factories fit; the third browns out.
        place_building(&mut app, BuildingKind::PowerPlant, Vec3::ZERO);
        place_building(&mut app, BuildingKind::Factory, Vec3::new(40.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Factory, Vec3::new(80.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Factory, Vec3::new(200.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Dwelling, Vec3::new(120.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Dwelling, Vec3::new(160.0, 0.0, 0.0));
        ticks(&mut app, 2);
        for x in [40.0, 80.0, 120.0, 160.0, 200.0] {
            wire(&mut app, Vec3::ZERO, Vec3::new(x, 0.0, 0.0));
        }
        fuel_plant(&mut app, 10.0);
        ticks(&mut app, 3);
        let world = app.world_mut();
        let mut powered_homes = 0;
        let mut powered_factories = 0;
        for (b, p) in world
            .query::<(&super::super::buildings::Building, &Powered)>()
            .iter(world)
        {
            match (b.kind, p.0) {
                (BuildingKind::Dwelling, true) => powered_homes += 1,
                (BuildingKind::Factory, true) => powered_factories += 1,
                _ => {}
            }
        }
        assert_eq!(powered_homes, 2, "housing is the last thing unplugged");
        assert_eq!(
            powered_factories, 2,
            "10 MW − 2×{DWELLING_DEMAND_MW} MW homes leaves room for two 4 MW factories"
        );
    }

    /// Pins the Dwelling row's `PriorityClass::Housing` itself, which the
    /// starved-grid test above cannot: there the leftover pool happens to
    /// cover both homes even under flat id-order, so a class flip changes
    /// nothing it asserts. Here the pool is sized so the two answers differ —
    /// 9 MW against two factories (8 MW, lower ids) and three homes (3 MW):
    /// Housing-first serves 3 homes + 1 factory; id-order would serve
    /// 2 factories + 1 home. The catalogue mutation this exists to catch was
    /// real (phase 6 shipped it): Housing → Industry passed all 153 tests.
    #[test]
    fn a_priority_flip_on_the_dwelling_row_cannot_hide_in_the_leftovers() {
        let mut app = app();
        // Factories first: flat id-order would hand them the pool.
        place_building(&mut app, BuildingKind::Factory, Vec3::new(40.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Factory, Vec3::new(80.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Dwelling, Vec3::new(120.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Dwelling, Vec3::new(160.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Dwelling, Vec3::new(200.0, 0.0, 0.0));
        ticks(&mut app, 2);
        // A bare 9 MW source (no `Building`, so `produce_flows` never
        // overwrites it), spanned to every consumer — the witness-test trick.
        let source = app.world_mut().spawn(PowerOutput(9.0)).id();
        let consumers: Vec<Entity> = {
            let world = app.world_mut();
            world
                .query::<(Entity, &super::super::buildings::Building)>()
                .iter(world)
                .map(|(e, _)| e)
                .collect()
        };
        for (i, consumer) in consumers.into_iter().enumerate() {
            app.world_mut().spawn(WireSpan {
                id: SpanId(9_100 + i as u64),
                a: source,
                b: consumer,
                kind: NetKind::Power,
            });
        }
        ticks(&mut app, 1);
        let world = app.world_mut();
        let mut powered_homes = 0;
        let mut powered_factories = 0;
        for (b, p) in world
            .query::<(&super::super::buildings::Building, &Powered)>()
            .iter(world)
        {
            match (b.kind, p.0) {
                (BuildingKind::Dwelling, true) => powered_homes += 1,
                (BuildingKind::Factory, true) => powered_factories += 1,
                _ => {}
            }
        }
        assert_eq!(
            powered_homes, 3,
            "every home lights before any surplus reaches industry"
        );
        assert_eq!(
            powered_factories, 1,
            "9 MW − 3 MW of homes leaves room for exactly one 4 MW factory"
        );
    }

    /// A construction site is inert on the consumer side: scaffolding with a
    /// lower `BuildingId` must never take the MW a finished building then
    /// goes without. Before the `Without<ConstructionSite>` filter this
    /// exact layout browned out the finished factory.
    #[test]
    fn a_site_never_outdraws_a_finished_building() {
        use super::super::construction::ConstructionSite;
        let mut app = app();
        place_building(&mut app, BuildingKind::Factory, Vec3::new(40.0, 0.0, 0.0));
        place_building(&mut app, BuildingKind::Factory, Vec3::new(80.0, 0.0, 0.0));
        ticks(&mut app, 2);
        // The first-placed factory (lower id, so first in line for the pool)
        // becomes a site by hand — the wires test app runs no construction
        // plugin, which is exactly why this must be explicit.
        let (site, finished) = {
            let world = app.world_mut();
            let mut found: Vec<(u64, Entity)> = world
                .query::<(Entity, &super::super::buildings::Building)>()
                .iter(world)
                .map(|(e, b)| (b.id.0, e))
                .collect();
            found.sort_by_key(|(id, _)| *id);
            (found[0].1, found[1].1)
        };
        app.world_mut()
            .entity_mut(site)
            .insert(ConstructionSite::for_kind(BuildingKind::Factory));
        // A pool of exactly one factory's demand, spanned to both.
        let source = app.world_mut().spawn(PowerOutput(4.0)).id();
        for (i, consumer) in [site, finished].into_iter().enumerate() {
            app.world_mut().spawn(WireSpan {
                id: SpanId(9_200 + i as u64),
                a: source,
                b: consumer,
                kind: NetKind::Power,
            });
        }
        ticks(&mut app, 1);
        assert!(
            app.world().get::<Powered>(finished).unwrap().0,
            "the finished factory gets the pool the site must not draw"
        );
        assert!(
            !app.world().get::<Powered>(site).unwrap().0,
            "the site's gate stays dark while it is scaffolding"
        );
    }

    /// Every kind's power role, read off the catalogue and checked against what
    /// the grid actually does — the witness that survives `solve_power` reading
    /// the column instead of matching on the kind.
    ///
    /// Two claims per kind, and neither can go tautological. Presence of the
    /// `Powered` gate is a *spawn-time* decision in `buildings.rs`, and
    /// `solve_power`'s query requires the component, so a kind that claims a
    /// draw but never earns one is skipped entirely — the row would be a lie
    /// nobody reads. The served/starved threshold is `network::allocate`'s
    /// arithmetic against a pool set to exactly the claimed rate: a row bound to
    /// the wrong constant moves the cut, which is the `MINE_COAL_RATE ==
    /// QUARRY_GRAVEL_RATE` trap the Phase 1b review named, on the utility side.
    #[test]
    fn the_grid_serves_exactly_the_draw_the_power_column_claims() {
        use super::super::catalogue::spec;
        for kind in BuildingKind::ALL {
            let mut app = app();
            place_building(&mut app, kind, Vec3::ZERO);
            ticks(&mut app, 2);
            let building = building_entity(&mut app, kind);
            let Some(demand) = spec(kind).power else {
                assert!(
                    app.world().get::<Powered>(building).is_none(),
                    "{kind:?} claims no power draw, so it must carry no gate — \
                     `solve_power`'s query requires one, and a gate here would \
                     put the kind on the grid the table says it is off"
                );
                continue;
            };
            assert!(
                app.world().get::<Powered>(building).is_some(),
                "{kind:?} claims a power draw but never earned a `Powered` gate, \
                 so `solve_power` cannot see it at all"
            );
            // A bare source: no `Building`, so `produce_flows` never overwrites
            // it and the component's pool is exactly what this test says.
            let source = app.world_mut().spawn(PowerOutput(demand.rate)).id();
            app.world_mut().spawn(WireSpan {
                id: SpanId(9_000),
                a: source,
                b: building,
                kind: NetKind::Power,
            });
            ticks(&mut app, 1);
            assert!(
                app.world().get::<Powered>(building).unwrap().0,
                "{kind:?}: a pool of exactly its claimed {} MW must serve it",
                demand.rate
            );
            app.world_mut().get_mut::<PowerOutput>(source).unwrap().0 = demand.rate * 0.99;
            ticks(&mut app, 1);
            assert!(
                !app.world().get::<Powered>(building).unwrap().0,
                "{kind:?}: one percent short of its claimed {} MW must starve it",
                demand.rate
            );
        }
    }
}
