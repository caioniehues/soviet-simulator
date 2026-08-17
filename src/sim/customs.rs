//! Customs stage 1 (G1.2 #80, W&R's border pulled forward from B10): the
//! outside world exists at the CustomsOffice. Outbound, goods hauled to the
//! office by the ordinary dispatcher are sold abroad at a real dock rate —
//! roubles trickle in as tonnage crosses the counter, never on arrival.
//! Inbound, every purchased vehicle enters the republic here: it spends
//! road-time driving from the border to its depot (`InTransitFromBorder`)
//! before any dispatcher may seize it. No customs office in the world means
//! bootstrap fiat delivery — the authored First Plan (G1.5) starts with one
//! placed. Dual currency, era catalogues, and resource imports stay B10.

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::clock::{FrameIndex, SECS_PER_PASS};
use super::plan::Treasury;
use super::resources::{Inventory, ResourceKind};
use super::stages::{SimStage, SimTick};
use super::vehicles::TRUCK_SPEED;

/// Export price, roubles per tonne sold at the border.
pub fn export_price(kind: ResourceKind) -> f32 {
    match kind {
        ResourceKind::Coal => 0.8,
        ResourceKind::Gravel => 0.4,
        ResourceKind::Goods => 2.5,
    }
}

/// Tonnes over the border counter per tick (the customs dock rate — selling
/// is a throughput, not a teleport, exactly like truck docking).
pub const SALE_RATE: f32 = 0.25;

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
            (sell_exports, release_border_arrivals).in_set(SimStage::ProductionAndUtilities),
        );
    }
}

/// Drain every customs yard at the dock rate, crediting the treasury.
fn sell_exports(
    mut offices: Query<(&Building, &mut Inventory)>,
    mut treasury: ResMut<Treasury>,
) {
    for (building, mut inventory) in &mut offices {
        if building.kind != BuildingKind::CustomsOffice {
            continue;
        }
        let mut budget = SALE_RATE;
        for kind in ResourceKind::ALL {
            if budget <= 0.0 {
                break;
            }
            let sold = inventory.take(kind, budget);
            if sold > 0.0 {
                budget -= sold;
                treasury.roubles += sold * export_price(kind);
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
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, PlanSimPlugin, CustomsSimPlugin));
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
        let mut q = world.query_filtered::<
            Has<super::super::construction::ConstructionSite>,
            With<Building>,
        >();
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
        ticks(&mut app, border_transit_frames(Vec3::new(120.0, 0.0, 0.0), Vec3::ZERO) + 2);
        let world = app.world_mut();
        let mut q = world.query::<(&VehicleAsset, Has<InTransitFromBorder>)>();
        let (_, in_transit) = q.single(world).unwrap();
        assert!(!in_transit, "the border gate lifted");
    }

    #[test]
    fn goods_in_the_customs_yard_become_roubles_at_the_dock_rate() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::CustomsOffice,
                pos: Vec3::ZERO,
            });
        ticks(&mut app, 2);
        let customs = {
            let world = app.world_mut();
            let mut q = world.query::<(Entity, &Building)>();
            q.iter(world)
                .find(|(_, b)| b.kind == BuildingKind::CustomsOffice)
                .map(|(e, _)| e)
                .expect("customs placed")
        };
        app.world_mut()
            .get_mut::<Inventory>(customs)
            .unwrap()
            .add(ResourceKind::Goods, 10.0);
        let before = app.world().resource::<Treasury>().roubles;
        ticks(&mut app, 8);
        let world = app.world();
        let sold = 10.0 - world.get::<Inventory>(customs).unwrap().amount(ResourceKind::Goods);
        assert!(sold > 0.0, "the yard drains");
        let earned = world.resource::<Treasury>().roubles - before;
        assert!(
            (earned - sold * export_price(ResourceKind::Goods)).abs() < 1e-3,
            "sold {sold} t, earned {earned} rbl"
        );
    }
}
