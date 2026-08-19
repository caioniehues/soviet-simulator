//! Customs stage 1 (G1.2 #80, W&R's border pulled forward from B10): the
//! outside world exists at the CustomsOffice. Outbound, goods hauled to the
//! office by the ordinary dispatcher are sold abroad at a real dock rate —
//! roubles trickle in as tonnage crosses the counter, never on arrival.
//! Inbound, every purchased vehicle enters the republic here: it spends
//! road-time driving from the border to its depot (`InTransitFromBorder`)
//! before any dispatcher may seize it. No customs office in the world means
//! bootstrap fiat delivery — the authored First Plan (G1.5) starts with one
//! placed. Resource imports pulled forward from B10 too (2026-08-19): the
//! office's yard now reads its own `StoragePolicies` bands, buying abroad
//! below the min line the same tick the ordinary dispatcher would haul the
//! surplus above the max line out. Dual currency and era catalogues stay B10.

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::clock::{FrameIndex, SECS_PER_PASS};
use super::plan::Treasury;
use super::resources::{Inventory, ResourceKind};
use super::stages::{SimStage, SimTick};
use super::storage::StoragePolicies;
use super::vehicles::TRUCK_SPEED;

/// Export price, roubles per tonne sold at the border.
pub fn export_price(kind: ResourceKind) -> f32 {
    match kind {
        ResourceKind::Coal => 0.8,
        ResourceKind::Gravel => 0.4,
        ResourceKind::Goods => 2.5,
    }
}

/// Import price, roubles per tonne bought at the border — a flat markup over
/// the export price, so a resource never round-trips for profit.
pub fn import_price(kind: ResourceKind) -> f32 {
    export_price(kind) * 2.0
}

/// Tonnes over the border counter per tick (the customs dock rate — selling
/// is a throughput, not a teleport, exactly like truck docking).
pub const SALE_RATE: f32 = 0.25;

/// Tonnes bought in from abroad per tick, same counter as `SALE_RATE`.
pub const IMPORT_RATE: f32 = SALE_RATE;

/// A purchased vehicle still on the road in from the border: no dispatcher,
/// transit duty, or construction assignment may touch it until `arrives`.
#[derive(Component, Debug)]
pub struct InTransitFromBorder {
    pub arrives: u32,
}

/// Query filter for fleet-selection systems: only vehicles that have
/// finished their drive in from the border are assignable.
pub type Arrived = Without<InTransitFromBorder>;

/// Travel frames from the customs office to `to`, as the crow flies at truck
/// speed — the road-honest version can ride the PathService once imported
/// vehicles get a rendered drive-in.
pub fn border_transit_frames(customs_pos: Vec3, to: Vec3) -> u32 {
    let metres = customs_pos.distance(to);
    (metres / (TRUCK_SPEED * SECS_PER_PASS as f32)).ceil() as u32
}

pub struct CustomsSimPlugin;

impl Plugin for CustomsSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimTick,
            // Declared order (ADR 0013): sell before buy so this tick's export
            // proceeds are already in the treasury when buy_imports spends it
            // — previously accidental, since both hold conflicting Inventory/
            // Treasury access and so were already serialized in registration
            // order. release_border_arrivals is unrelated fleet bookkeeping
            // (it never touches Inventory or Treasury); placed last so it
            // can't be mistaken for a step in the office's trade pair.
            (sell_exports, buy_imports, release_border_arrivals)
                .chain()
                .in_set(SimStage::ProductionAndUtilities),
        );
    }
}

/// Drain the surplus above each resource's max band at the dock rate,
/// crediting the treasury. With the office's default (0.0, 0.0) bands every
/// tonne in the yard is surplus, so an untouched office still sells
/// everything hauled to it — raising the max band is how a player starts
/// stockpiling instead.
fn sell_exports(
    // An office under construction is inert: its yard holds construction
    // materials, and selling those would mint roubles out of its own bill.
    mut offices: Query<
        (&Building, &mut Inventory, &StoragePolicies),
        Without<super::construction::ConstructionSite>,
    >,
    mut treasury: ResMut<Treasury>,
) {
    for (building, mut inventory, policies) in &mut offices {
        if building.kind != BuildingKind::CustomsOffice {
            continue;
        }
        let mut budget = SALE_RATE;
        for kind in ResourceKind::ALL {
            if budget <= 0.0 {
                break;
            }
            let offer = policies.surplus(kind, &inventory).min(budget);
            let sold = inventory.take(kind, offer);
            if sold > 0.0 {
                budget -= sold;
                treasury.roubles += sold * export_price(kind);
            }
        }
    }
}

/// Buy up to the deficit below each resource's min band at the dock rate,
/// debiting the treasury. An office at its default bands has no deficit and
/// buys nothing — this is the seed the First Plan needs: raise a resource's
/// min line and its yard starts filling from abroad, roubles permitting.
fn buy_imports(
    mut offices: Query<
        (&Building, &mut Inventory, &StoragePolicies),
        Without<super::construction::ConstructionSite>,
    >,
    mut treasury: ResMut<Treasury>,
) {
    for (building, mut inventory, policies) in &mut offices {
        if building.kind != BuildingKind::CustomsOffice {
            continue;
        }
        let mut budget = IMPORT_RATE;
        for kind in ResourceKind::ALL {
            if budget <= 0.0 || treasury.roubles <= 0.0 {
                break;
            }
            let price = import_price(kind);
            let affordable = treasury.roubles / price;
            let request = policies
                .deficit(kind, &inventory)
                .min(budget)
                .min(affordable);
            if request <= 0.0 {
                continue;
            }
            let bought = inventory.add(kind, request);
            if bought > 0.0 {
                budget -= bought;
                treasury.roubles -= bought * price;
            }
        }
    }
}

/// The border gate lifts: an in-transit vehicle whose travel time has
/// elapsed becomes an ordinary parked asset.
fn release_border_arrivals(
    mut commands: Commands,
    frame: Res<FrameIndex>,
    in_transit: Query<(Entity, &InTransitFromBorder)>,
) {
    for (entity, transit) in &in_transit {
        if frame.0.wrapping_sub(transit.arrives) < u32::MAX / 2 {
            commands.entity(entity).remove::<InTransitFromBorder>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingSimPlugin};
    use super::super::plan::PlanSimPlugin;
    use super::super::storage::StorageSimPlugin;
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            BuildingSimPlugin,
            PlanSimPlugin,
            StorageSimPlugin,
            CustomsSimPlugin,
        ));
        a
    }

    fn place_customs(app: &mut App) -> Entity {
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::CustomsOffice,
                pos: Vec3::ZERO,
            });
        ticks(app, 2);
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &Building)>();
        q.iter(world)
            .find(|(_, b)| b.kind == BuildingKind::CustomsOffice)
            .map(|(e, _)| e)
            .expect("customs placed")
    }

    fn ticks(app: &mut App, n: u32) {
        for _ in 0..n {
            app.world_mut()
                .resource_mut::<Time>()
                .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
            app.update();
        }
    }

    #[test]
    fn a_prebuilt_customs_needs_no_construction() {
        let mut app = app();
        app.add_plugins(super::super::construction::ConstructionSimPlugin);
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::PlacePrebuilt {
                kind: BuildingKind::CustomsOffice,
                pos: Vec3::ZERO,
            });
        ticks(&mut app, 2);
        let world = app.world_mut();
        let mut q = world
            .query_filtered::<Has<super::super::construction::ConstructionSite>, With<Building>>();
        assert!(
            !q.single(world).expect("customs placed"),
            "state infrastructure arrives finished"
        );
    }

    #[test]
    fn a_bought_truck_drives_in_from_the_border_before_it_can_work() {
        use super::super::vehicles::{
            VehicleAsset, VehicleEdit, VehicleEditQueue, VehicleSimPlugin,
        };
        let mut app = app();
        app.add_plugins(VehicleSimPlugin);
        {
            let mut q = app.world_mut().resource_mut::<BuildingEditQueue>();
            q.0.push(BuildingEdit::Place {
                kind: BuildingKind::CustomsOffice,
                pos: Vec3::new(120.0, 0.0, 0.0),
            });
            q.0.push(BuildingEdit::Place {
                kind: BuildingKind::Depot,
                pos: Vec3::ZERO,
            });
        }
        ticks(&mut app, 2);
        let depot = {
            let world = app.world_mut();
            let mut q = world.query::<(Entity, &Building)>();
            q.iter(world)
                .find(|(_, b)| b.kind == BuildingKind::Depot)
                .map(|(e, _)| e)
                .unwrap()
        };
        app.world_mut()
            .resource_mut::<VehicleEditQueue>()
            .0
            .push(VehicleEdit::BuyTruck {
                depot,
                class: super::super::resources::TransportClass::Bulk,
            });
        ticks(&mut app, 2);
        {
            let world = app.world_mut();
            let mut q = world.query::<(&VehicleAsset, Has<InTransitFromBorder>)>();
            let (_, in_transit) = q.single(world).expect("truck bought");
            assert!(in_transit, "a fresh import is still on the road in");
        }
        // 120 m at 12 m/s of 1/60 s ticks = 600 frames.
        ticks(
            &mut app,
            border_transit_frames(Vec3::new(120.0, 0.0, 0.0), Vec3::ZERO) + 2,
        );
        let world = app.world_mut();
        let mut q = world.query::<(&VehicleAsset, Has<InTransitFromBorder>)>();
        let (_, in_transit) = q.single(world).unwrap();
        assert!(!in_transit, "the border gate lifted");
    }

    #[test]
    fn goods_in_the_customs_yard_become_roubles_at_the_dock_rate() {
        let mut app = app();
        let customs = place_customs(&mut app);
        app.world_mut()
            .get_mut::<Inventory>(customs)
            .unwrap()
            .add(ResourceKind::Goods, 10.0);
        let before = app.world().resource::<Treasury>().roubles;
        ticks(&mut app, 8);
        let world = app.world();
        let sold = 10.0
            - world
                .get::<Inventory>(customs)
                .unwrap()
                .amount(ResourceKind::Goods);
        assert!(sold > 0.0, "the yard drains");
        let earned = world.resource::<Treasury>().roubles - before;
        assert!(
            (earned - sold * export_price(ResourceKind::Goods)).abs() < 1e-3,
            "sold {sold} t, earned {earned} rbl"
        );
    }

    /// Raising the min band on gravel opens the import counter: the office
    /// buys up to the dock rate a tick, spending 2x the export price, and
    /// stops filling once the yard reaches the band line.
    #[test]
    fn raising_the_min_band_buys_gravel_from_abroad() {
        let mut app = app();
        let customs = place_customs(&mut app);
        {
            let world = app.world_mut();
            let mut policies = world.get_mut::<StoragePolicies>(customs).unwrap();
            policies.set(
                ResourceKind::Gravel,
                Some(super::super::storage::StorageBand::new(0.1, 0.1)),
            );
        }
        app.insert_resource(Treasury { roubles: 1000.0 });
        ticks(&mut app, 1);
        let world = app.world();
        let bought = world
            .get::<Inventory>(customs)
            .unwrap()
            .amount(ResourceKind::Gravel);
        assert!((bought - IMPORT_RATE).abs() < 1e-3, "bought {bought} t");
        let spent = 1000.0 - world.resource::<Treasury>().roubles;
        assert!(
            (spent - bought * import_price(ResourceKind::Gravel)).abs() < 1e-3,
            "bought {bought} t for {spent} rbl"
        );

        // Fill the rest of the way to the band and confirm buying stops there.
        ticks(&mut app, 200);
        let world = app.world();
        let target = 0.1 * world.get::<Inventory>(customs).unwrap().capacity;
        let filled = world
            .get::<Inventory>(customs)
            .unwrap()
            .amount(ResourceKind::Gravel);
        assert!(
            (filled - target).abs() < 1e-3,
            "filled {filled}, want {target}"
        );
    }

    /// An empty treasury buys nothing and never goes negative; a treasury
    /// that covers only part of a tick's dock rate buys a partial tonnage.
    #[test]
    fn an_empty_treasury_buys_nothing_and_never_goes_negative() {
        let mut app = app();
        let customs = place_customs(&mut app);
        {
            let world = app.world_mut();
            let mut policies = world.get_mut::<StoragePolicies>(customs).unwrap();
            policies.set(
                ResourceKind::Coal,
                Some(super::super::storage::StorageBand::new(0.5, 0.5)),
            );
        }
        app.insert_resource(Treasury { roubles: 0.0 });
        ticks(&mut app, 1);
        {
            let world = app.world();
            let bought = world
                .get::<Inventory>(customs)
                .unwrap()
                .amount(ResourceKind::Coal);
            assert_eq!(bought, 0.0, "no treasury, no import");
            assert_eq!(world.resource::<Treasury>().roubles, 0.0);
        }

        // Half a tick's worth of roubles buys half a tick's worth of tonnage.
        let price = import_price(ResourceKind::Coal);
        let half_tick_cost = IMPORT_RATE * price / 2.0;
        app.insert_resource(Treasury {
            roubles: half_tick_cost,
        });
        ticks(&mut app, 1);
        let world = app.world();
        let bought = world
            .get::<Inventory>(customs)
            .unwrap()
            .amount(ResourceKind::Coal);
        assert!(
            (bought - IMPORT_RATE / 2.0).abs() < 1e-3,
            "bought {bought} t"
        );
        assert!(world.resource::<Treasury>().roubles >= 0.0);
    }

    /// With the min band under the max band and stock sitting between them,
    /// neither `sell_exports` nor `buy_imports` moves a single tonne — no
    /// ping-pong between the two systems.
    #[test]
    fn stock_between_the_bands_is_untouched_by_either_system() {
        let mut app = app();
        let customs = place_customs(&mut app);
        {
            let world = app.world_mut();
            let mut policies = world.get_mut::<StoragePolicies>(customs).unwrap();
            policies.set(
                ResourceKind::Coal,
                Some(super::super::storage::StorageBand::new(0.2, 0.6)),
            );
            let mut inventory = world.get_mut::<Inventory>(customs).unwrap();
            let capacity = inventory.capacity;
            inventory.add(ResourceKind::Coal, 0.4 * capacity);
        }
        app.insert_resource(Treasury { roubles: 1000.0 });
        ticks(&mut app, 10);
        let world = app.world();
        let stock = world
            .get::<Inventory>(customs)
            .unwrap()
            .amount(ResourceKind::Coal);
        let expected = 0.4 * world.get::<Inventory>(customs).unwrap().capacity;
        assert!(
            (stock - expected).abs() < 1e-6,
            "stock {stock} moved out of the band's dead zone"
        );
        assert_eq!(world.resource::<Treasury>().roubles, 1000.0);
    }
}
