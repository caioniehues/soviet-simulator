//! sov-2uv: freight stations own base_mod-declared road vehicles, and ranked
//! dispatch assignment keeps a domestic short hop moving while a long border
//! import is still in flight.
//!
//! Before the fix, `base_mod` gave trucks to factories only and stations
//! owned none, so every import borrowed a factory truck for the longest
//! journey on the map while `Dispatcher::query` handed out the single shared
//! pool first-come-first-served: the domestic hop starved behind the border
//! haul. After the fix the station parks its own declared trucks at its door
//! and the query ranks candidates by network meters, so each dispatch gets
//! its genuinely nearest truck — the import takes a station truck, the hop
//! takes the factory truck, and the later-posted domestic delivery finishes
//! while the earlier-posted import is still driving.

use super::*;

use super::hoarding::{build_company_at, mk_soul};
use super::inflation::remove_default_freight_station;
use crate::economy::{DispatchState, Market};
use crate::map::BuildingKind;
use crate::map_dynamic::BuildingInfos;
use crate::world_command::WorldCommand;
use crate::SoulID;
use geom::OBB;
use prototypes::{BuildingGen, FreightStationPrototypeID, GoodsCompanyID, ItemID};

/// A domestic short hop (seller door to a nearby buyer west of it, ~55 road
/// meters) completes while a long border import (station door to a far buyer,
/// ~500 road meters, posted FIRST) is still driving.
///
/// Red without the fix: the station owns no trucks, so the import holds the
/// city's only truck for its whole long haul and the hop waits in `ToSource`
/// until long after the import delivers. Green with it: the import rides a
/// station truck, the hop rides the factory truck, on overlapping ticks.
#[test]
fn sov_2uv_short_hop_completes_while_long_import_in_flight() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(640.0, 0.0, 0.0)]);

    // Reachable freight station, same proven geometry as
    // `ledger::sov_abs_ext_trade_import_is_physical`: the door lands within
    // the dispatcher's 50-unit lane cutoff of the spur, so trucks can serve
    // it; the route out to the far buyer below is the long leg.
    let station_centre = Vec2::new(150.0, 200.0);
    ctx.apply(&[WorldCommand::MapBuildSpecialBuilding {
        pos: OBB::new(station_centre, Vec2::X, 160.0, 200.0),
        kind: BuildingKind::RailFreightStation(FreightStationPrototypeID::new("freight-station")),
        gen: BuildingGen::NoWalkway {
            door_pos: Vec2::new(110.0, 0.0),
        },
        zone: None,
        connected_road: None,
    }]);
    // The spur goes in BEFORE the soul-spawning tick below: the station's
    // declared trucks park near its door, and with no lane there yet
    // `spawn_parked_vehicle` finds no parking and the station opens
    // train-only.
    ctx.build_roads(&[
        Vec3::new(250.0, 0.0, 0.0),
        Vec3::new(station_centre.x, station_centre.y - 120.0, 0.0),
    ]);
    ctx.tick();
    assert_eq!(
        ctx.g.world().freight_stations.len(),
        1,
        "exactly one reachable freight station must exist"
    );

    // The station soul spawns on the tick above and parks its declared
    // trucks at its door. This is the ownership assertion: base_mod declares
    // `n_trucks = 2` on the freight-station entry.
    assert_eq!(
        ctx.g.world().vehicles.len(),
        2,
        "the freight station must own its two base_mod-declared trucks"
    );

    // Domestic seller shell: a real factory company (its one truck is the
    // short-hop fleet) with an isolated power network, so its recipe never
    // fires and the only orders in the city are the ones posted below.
    // An oil-pump — not a flour-factory: `recipe_init` posts input buy
    // orders for whatever the recipe consumes at spawn, and a flour-factory
    // would post its own cereal buy matching a second background import.
    // That import holds the fleet's spare station truck, leaving the hop
    // only its own factory truck — parked centimetres past its door, i.e.
    // a block-looping fallback that loses the race. The pump consumes
    // nothing, so no phantom demand exists. Owners are set on houses
    // immediately, so no humans ever spawn and no company driver can take
    // the factory truck out from under the market.
    let factory_b = build_company_at(
        &mut ctx,
        GoodsCompanyID::new("oil-pump").prototype(),
        // x = 125: the pump's truck parks in the fixed roadside spot just
        // east of its door — BEHIND the door's lane projection in travel
        // direction — so the dispatcher sees a direct approach instead of
        // a past-target block-loop fallback (sov-2uv query tiers). A few
        // meters either way can flip that sign (spots are road-fixed while
        // the door slides with the building), so do not "tidy" this number.
        Vec2::new(125.0, 20.0),
    );
    ctx.tick();
    assert_eq!(
        ctx.g.world().vehicles.len(),
        3,
        "city fleet must be exactly the station's two trucks plus the factory's one"
    );

    let seller = ctx.g.read::<BuildingInfos>().owner(factory_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(factory_b).unwrap().door_pos;

    // West of the seller (not east): the seller's driveway feeds the
    // westbound lane, so an eastbound delivery must first loop west to the
    // lane end, U-turn, and come back past its own door — a ~230 m
    // block-loop that eats the whole concurrency margin. Westbound the
    // same lane carries the truck straight to the buyer's door.
    let domestic_buyer_b = ctx.build_house_at(Vec2::new(60.0, 20.0));
    let domestic_buyer = mk_soul((1 << 32) | 21);
    ctx.g
        .write::<BuildingInfos>()
        .set_owner(domestic_buyer_b, domestic_buyer);
    let domestic_buyer_pos = ctx
        .g
        .map()
        .buildings
        .get(domestic_buyer_b)
        .unwrap()
        .door_pos;

    let import_buyer_b = ctx.build_house_at(Vec2::new(600.0, 20.0));
    let import_buyer = mk_soul((1 << 32) | 22);
    ctx.g
        .write::<BuildingInfos>()
        .set_owner(import_buyer_b, import_buyer);
    let import_buyer_pos = ctx
        .g
        .map()
        .buildings
        .get(import_buyer_b)
        .unwrap()
        .door_pos;

    let cereal = ItemID::new("cereal");
    let vegetable = ItemID::new("vegetable");

    // Phase 1: the long import, posted first. No domestic cereal exists, so
    // after one border-eligibility pass the freight station matches it.
    ctx.g
        .write::<Market>()
        .buy(import_buyer, import_buyer_pos.xy(), cereal, 4);
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if m
            .dispatches()
            .iter()
            .any(|d| d.kind == cereal && d.truck().is_some())
        {
            break;
        }
        assert!(
            ticks < 1500,
            "the import never got a truck: station-owned fleet is missing"
        );
    }
    let import_truck = {
        let m = ctx.g.read::<Market>();
        let d = m
            .dispatches()
            .iter()
            .find(|d| d.kind == cereal && d.buyer == import_buyer)
            .expect("import dispatch must exist once its truck is assigned");
        assert!(
            matches!(d.seller, SoulID::FreightStation(_)),
            "the import must haul out of the freight station, like any physical delivery"
        );
        d.truck().expect("polled until the import holds a truck")
    };

    // Phase 2: the domestic short hop, posted while the import is driving.
    // `stock == qty` keeps the surplus-export leg from touching it.
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, vegetable, 4);
        m.sell(seller, seller_pos.xy(), vegetable, 4, 4);
        m.buy(domestic_buyer, domestic_buyer_pos.xy(), vegetable, 4);
    }
    // The hop must be picked up by a DIFFERENT truck than the import's: that
    // is the ranked assignment doing its job instead of FCFS on one pool.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if let Some(d) = m
            .dispatches()
            .iter()
            .find(|d| d.kind == vegetable && d.truck().is_some())
        {
            assert_ne!(
                d.truck().unwrap(),
                import_truck,
                "the short hop must ride its own truck, not the import's"
            );
            break;
        }
        assert!(
            ticks < 1500,
            "the domestic hop never got a truck: it is starved behind the import"
        );
    }

    // Phase 3: the later-posted hop completes while the earlier-posted
    // import is still in flight — the anti-FCFS outcome.
    let mut ticks = 0;
    loop {
        ctx.advance_ticks(100);
        ticks += 100;
        if ctx.g.read::<Market>().capital(domestic_buyer, vegetable) == 4 {
            break;
        }
        assert!(
            ticks < 20000,
            "the domestic short hop never completed while the import was driving"
        );
    }
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(
            m.capital(import_buyer, cereal),
            0,
            "the import must still be in flight when the short hop completes"
        );
        let d = m
            .dispatches()
            .iter()
            .find(|d| d.kind == cereal && d.buyer == import_buyer)
            .expect("the import dispatch must still be driving, not delivered");
        assert!(
            d.truck().is_some(),
            "the import must hold its truck (driving), not wait truckless in ToSource"
        );
        assert!(
            !matches!(d.state, DispatchState::ToSource) || d.truck().is_some(),
            "the import must be past the unassigned wait: {:?}",
            d.state
        );
    }
}
