//! sov-dispatch-wedge-ab4: retail (store -> consumer) purchases are settled
//! at eat-time, not match-time, and never create a Dispatch/truck — the
//! buyer's own walk to the seller's building is the physical movement (see
//! `souls::desire::buyfood`). Covers the kornai-economist ruling's three
//! mandatory reservation-release paths (eat / despawn / TTL expiry), the
//! `recipe_should_produce` capital-reserved fix, and cancellation-with-return
//! for the factory->store leg that still uses real dispatches.

use super::*;

use super::hoarding::{build_company_at, remove_soul, setup_seller_buyer};
use crate::economy::{DispatchState, Market};
use crate::map_dynamic::BuildingInfos;
use crate::souls::goods_company::recipe_should_produce;
use crate::souls::human::spawn_human;
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::world::{HumanEnt, HumanID};
use crate::{ParCommandBuffer, SoulID};
use prototypes::{GameTime, GoodsCompanyID, ItemID, Money, TICKS_PER_HOUR};

fn mk_human(id: u64) -> SoulID {
    SoulID::Human(HumanID::from(slotmapd::KeyData::from_ffi(id)))
}

fn bread() -> ItemID {
    ItemID::new("bread")
}

/// Matches one bread unit between a real bakery seller and a fabricated
/// human buyer, driving `make_trades` directly (mirrors `ledger.rs`'s style:
/// exercise the market mechanics without fighting human pathfinding).
fn match_one_bread(ctx: &mut TestCtx, seller: SoulID, seller_pos: geom::Vec3, buyer: SoulID) {
    let mut m = ctx.g.write::<Market>();
    m.produce(seller, bread(), 1);
    m.sell(seller, seller_pos.xy(), bread(), 1, 0);
    m.buy(buyer, seller_pos.xy(), bread(), 1);
    let trades: Vec<_> = m
        .make_trades(|_| None)
        .iter()
        .filter(|t| t.kind == bread() && t.buyer.0 == buyer)
        .copied()
        .collect();
    assert_eq!(trades.len(), 1, "the bread match must happen");
    assert_eq!(trades[0].money_delta, Money::ZERO);
}

/// (i) A human bread purchase creates no Dispatch; the seller's stock is
/// reserved (not debited) at match time, and settles only when the buyer
/// eats — at which point the seller is debited, the reservation released,
/// and the buyer credited NOTHING (the loaf is destroyed at consumption, not
/// added to the buyer's own capital). Money::ZERO end to end.
#[test]
fn scenario_retail_no_dispatch_settles_at_eat_time() {
    let mut ctx = TestCtx::new();
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    ctx.tick();
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    let buyer = mk_human((1 << 32) | 42);
    let gvt_before = ctx.g.read::<crate::economy::Government>().money;

    match_one_bread(&mut ctx, seller, seller_pos, buyer);

    {
        let m = ctx.g.read::<Market>();
        assert!(
            m.dispatches().is_empty(),
            "a human bread purchase must never create a Dispatch"
        );
        assert_eq!(
            m.capital(seller, bread()),
            1,
            "match must not move seller stock (reserved, not debited)"
        );
        let claim = m
            .retail_claim(buyer)
            .expect("a matched retail purchase must record a claim");
        assert_eq!(claim.seller, seller);
        assert_eq!(claim.kind, bread());
        assert_eq!(claim.qty, 1);
    }

    // Eat-time settlement.
    let settled = ctx.g.write::<Market>().settle_retail(buyer);
    assert!(settled, "settle_retail must find and consume the claim");

    let m = ctx.g.read::<Market>();
    assert_eq!(m.capital(seller, bread()), 0, "seller must end up debited");
    assert_eq!(
        m.capital(buyer, bread()),
        0,
        "buyer must be credited NOTHING: the loaf is destroyed at consumption"
    );
    assert!(
        m.retail_claim(buyer).is_none(),
        "the claim must be consumed by settlement"
    );
    let gvt_after = ctx.g.read::<crate::economy::Government>().money;
    assert_eq!(
        gvt_before, gvt_after,
        "no money may move on the retail path"
    );
}

/// (ii) A buyer despawning (`Market::remove`) while holding an outstanding
/// retail claim must release the SELLER's reservation, not just the buyer's
/// own rows.
#[test]
fn scenario_retail_buyer_despawn_releases_reservation() {
    let mut ctx = TestCtx::new();
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    ctx.tick();
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    let buyer = mk_human((1 << 32) | 43);
    match_one_bread(&mut ctx, seller, seller_pos, buyer);

    assert_eq!(ctx.g.read::<Market>().reserved(seller, bread()), 1);

    remove_soul(&mut ctx, buyer); // buyer "died" mid-wait

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.reserved(seller, bread()),
        0,
        "a dead buyer's outstanding retail claim must release the seller's reservation"
    );
    assert!(m.retail_claim(buyer).is_none());
    assert_eq!(
        m.capital(seller, bread()),
        1,
        "the loaf was never delivered, seller keeps it"
    );
}

/// (iii) TTL expiry releases the reservation and resets `BuyFood` back to
/// `Empty` so the human re-queues; `last_ate` must NOT advance (hunger keeps
/// rising -- going without, never game over).
#[test]
fn scenario_retail_ttl_releases_and_resets_buyfood() {
    let mut ctx = TestCtx::new();
    ctx.build_roads(&[
        geom::Vec3::new(0.0, 0.0, 0.0),
        geom::Vec3::new(200.0, 0.0, 0.0),
    ]);
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    let house = ctx.build_house_at(geom::Vec2::new(150.0, 20.0));
    ctx.tick();

    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    let human_id = spawn_human(&mut ctx.g, house).expect("human must spawn");
    let buyer = SoulID::Human(human_id);

    // Drive `BuyFood::apply` directly for deterministic control instead of
    // fighting `update_decision_system`'s desire-selection and randomized
    // per-human wait. First call: `Empty` -> issues its own buy order ->
    // `WaitingForTrade`.
    apply_buyfood(&mut ctx.g, human_id);
    ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);
    assert!(
        ctx.g.read::<Market>().inner()[&bread()]
            .buy_order(buyer)
            .is_some(),
        "BuyFood::apply from Empty must issue a buy order"
    );

    // Match that exact buy order (mirrors a real domestic match).
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, bread(), 1);
        m.sell(seller, seller_pos.xy(), bread(), 1, 0);
        let trades: Vec<_> = m
            .make_trades(|_| None)
            .iter()
            .filter(|t| t.kind == bread() && t.buyer.0 == buyer)
            .copied()
            .collect();
        assert_eq!(trades.len(), 1, "the bread match must happen");
    }
    assert!(ctx.g.read::<Market>().retail_claim(buyer).is_some());

    let last_ate_before = ctx.g.world().humans.get(human_id).unwrap().food.last_ate;

    // Drive ticks past the retail TTL (an hour of ticks); `advance_dispatches`
    // (called every tick by `market_update`) sweeps expired claims.
    ctx.advance_ticks(TICKS_PER_HOUR as u32 + 2);

    let m = ctx.g.read::<Market>();
    assert!(
        m.retail_claim(buyer).is_none(),
        "the claim must expire within its TTL"
    );
    assert_eq!(
        m.reserved(seller, bread()),
        0,
        "TTL expiry must release the seller's reservation"
    );
    drop(m);

    let last_ate_after = ctx.g.world().humans.get(human_id).unwrap().food.last_ate;
    assert_eq!(
        last_ate_before, last_ate_after,
        "TTL expiry must NOT advance last_ate: going without, never fed for nothing"
    );

    // Two more `apply()` calls: the first observes the claim is gone and
    // resets `WaitingForTrade` -> `Empty` (see `BuyFood::apply`'s
    // `WaitingForTrade` arm); the second, now in `Empty`, re-issues a fresh
    // buy order. `BuyFoodState` is private to its module, so the fresh buy
    // order is the externally observable proof the human didn't get stuck
    // waiting on a reservation that no longer exists.
    apply_buyfood(&mut ctx.g, human_id);
    ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);
    apply_buyfood(&mut ctx.g, human_id);
    ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);

    let m = ctx.g.read::<Market>();
    assert!(
        m.inner()[&bread()].buy_order(buyer).is_some(),
        "the human must re-queue for bread after its claim expired, not wait forever"
    );
}

/// (BLOCKER 2, second gate pass) A human whose buy order is posted but never
/// matched at all must NOT be reset back to `Empty` -- that would silently
/// score it as if it had never eaten (see `score`'s
/// `last_ate.elapsed/DAY - 1.0` fallback), sending it to work/home instead of
/// staying parked, every decision cycle, for the whole shortage. Contrasts
/// with the TTL-expired case above, which correctly DOES reset.
#[test]
fn scenario_never_matched_waiting_is_not_reset() {
    let mut ctx = TestCtx::new();
    ctx.build_roads(&[
        geom::Vec3::new(0.0, 0.0, 0.0),
        geom::Vec3::new(200.0, 0.0, 0.0),
    ]);
    let house = ctx.build_house_at(geom::Vec2::new(150.0, 20.0));
    ctx.tick();

    let human_id = spawn_human(&mut ctx.g, house).expect("human must spawn");

    // First call: `Empty` -> issues a buy order, no seller exists to match
    // it -> `WaitingForTrade`.
    apply_buyfood(&mut ctx.g, human_id);
    ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);
    assert!(
        ctx.g.read::<Market>().inner()[&bread()]
            .buy_order(SoulID::Human(human_id))
            .is_some(),
        "BuyFood::apply from Empty must issue a buy order"
    );

    // Several more calls, still nothing to match it: the order must survive
    // and the human must stay parked at score 0.0, never re-queuing with a
    // fresh `Empty -> WaitingForTrade` cycle (which would be silently
    // observable as harmless here, but is the exact bug shape: the guard
    // below is what actually catches it).
    for _ in 0..5 {
        apply_buyfood(&mut ctx.g, human_id);
        ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);
    }

    assert!(
        ctx.g.read::<Market>().inner()[&bread()]
            .buy_order(SoulID::Human(human_id))
            .is_some(),
        "a never-matched order must not be dropped by a spurious reset"
    );

    let (world, res) = ctx.g.world_res();
    let time = res.read::<GameTime>();
    let h = world.humans.get(human_id).unwrap();
    assert_eq!(
        h.food.score(&time, &h.location, &h.bought),
        0.0,
        "a never-matched wait must stay parked at score 0.0, not fall through to \
         last_ate.elapsed(), which would send the human to work/home instead of waiting"
    );
}

/// Calls `BuyFood::apply` directly for one human, using a single split
/// borrow (`world_res`) so world mutation and resource reads don't conflict
/// the way two separate `Simulation::read`/`world_mut_unchecked` calls would.
fn apply_buyfood(
    g: &mut crate::Simulation,
    human_id: HumanID,
) -> crate::souls::human::HumanDecisionKind {
    let (world, res) = g.world_res();
    let cbuf = res.read::<ParCommandBuffer<HumanEnt>>();
    let binfos = res.read::<BuildingInfos>();
    let map = res.read::<crate::map::Map>();
    let market = res.read::<Market>();
    let time = res.read::<GameTime>();
    let h = world.humans.get_mut(human_id).unwrap();
    h.food.apply(
        &cbuf,
        &binfos,
        &map,
        &market,
        &time,
        human_id,
        &h.trans,
        &h.location,
        &mut h.bought,
    )
}

/// (iv) `recipe_should_produce` reads capital MINUS reserved: a seller
/// sitting on several matched-but-uncollected sales must keep producing
/// instead of reading as full and halting (the wedge this ticket exists to
/// close: without the fix, a store with a handful of unclaimed retail
/// reservations halts production while unable to sell what it's holding).
#[test]
fn scenario_bakery_keeps_producing_with_uncollected_loaves() {
    let mut ctx = TestCtx::new();
    let bakery_proto = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery_proto, geom::Vec2::new(30.0, 20.0));
    ctx.tick();
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    let recipe = bakery_proto.recipe.as_ref().unwrap();
    // storage_multiplier is 5 with production amount 1 => halt threshold is
    // (raw) capital >= 6. Reserve 6 loaves via matched-but-uncollected retail
    // claims, leaving raw capital at 6 but unreserved capital at 0.
    for i in 0..6u64 {
        let buyer = mk_human((1 << 40) | i);
        match_one_bread(&mut ctx, seller, seller_pos, buyer);
    }

    // The bakery also needs its consumption input (flour) on hand, otherwise
    // the unrelated consumption-side check masks what this test is about.
    ctx.g
        .write::<Market>()
        .produce(seller, ItemID::new("flour"), 1);

    let m = ctx.g.read::<Market>();
    assert_eq!(m.capital(seller, bread()), 6);
    assert_eq!(m.reserved(seller, bread()), 6);
    assert!(
        recipe_should_produce(recipe, seller, &m),
        "raw capital reads as full (6 >= threshold) but every unit is already \
         spoken for by an uncollected sale; production must not halt"
    );
}

/// (v)(b) ToSource with a truck reserved but the vehicle entity gone: nothing
/// was ever debited, so cancellation is just freeing the reservations -- no
/// leaked dispatcher slot, no phantom seller debit. A second, still-alive
/// truck proves the dispatcher doesn't wedge: if the dead truck's
/// `reserved_by` entry were never freed, that alone wouldn't block a fresh
/// query (a new candidate is a new id) -- what it WOULD do is drive a second
/// dispatch to the same dead entity again, since `query` prefers the nearest
/// unreserved candidate and a leaked reservation is invisible until reused.
/// Driving a second buy through end to end after the first truck dies is the
/// concrete proof the cancellation path is clean.
#[test]
fn scenario_dead_truck_tosource_cancels_without_leak() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    let dead_truck =
        spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }

    // Drive until the dispatch exists, then a few more ticks so the
    // dispatcher has time to assign the (only) truck to it -- state stays
    // `ToSource` throughout, whether or not a truck is attached yet.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        if !ctx.g.read::<Market>().dispatches().is_empty() {
            break;
        }
        assert!(ticks < 4000, "dispatch was never created");
    }
    for _ in 0..5 {
        ctx.tick();
    }
    assert_eq!(
        ctx.g.read::<Market>().dispatches()[0].state,
        DispatchState::ToSource
    );

    // The vehicle entity vanishes before arriving (e.g. despawned).
    ctx.g.world_mut_unchecked().vehicles.remove(dead_truck);

    ctx.tick();

    let m = ctx.g.read::<Market>();
    assert!(
        m.dispatches().is_empty(),
        "the dispatch must be cancelled once its truck is gone"
    );
    assert_eq!(
        m.reserved(seller, cereal),
        0,
        "the reservation must be released, not frozen forever"
    );
    assert_eq!(
        m.capital(seller, cereal),
        10,
        "nothing was ever debited (truck never arrived), goods stay with the seller"
    );
    drop(m);

    // A fresh truck and a fresh buy order must be able to complete normally:
    // proves the dead truck's dispatcher reservation isn't still blocking or
    // otherwise wedging future dispatches.
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");
    ctx.g
        .write::<Market>()
        .buy(buyer, buyer_pos.xy(), cereal, 5);

    assert!(
        drain_or_return(&mut ctx, 6000),
        "a fresh dispatch must complete normally after the dead-truck cancellation"
    );
    assert_eq!(
        ctx.g.read::<Market>().capital(buyer, cereal),
        5,
        "the second delivery must complete cleanly"
    );
}

/// (v)(a) Loading with the buyer's building demolished after the truck has
/// already loaded: goods are debited and physically on the truck, so they
/// must be driven back to the seller and re-credited on arrival -- never an
/// instant teleport refund.
#[test]
fn scenario_demolished_buyer_returns_goods_physically() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 5);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }

    // Drive until the dispatch has debited the seller and entered Loading.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if !m.dispatches().is_empty() && m.dispatches()[0].state == DispatchState::Loading {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Loading");
    }
    assert_eq!(
        ctx.g.read::<Market>().capital(seller, cereal),
        0,
        "seller must already be debited by the time the truck is loading"
    );

    // Demolish the buyer's building (soul stays registered as owner, but the
    // building itself is gone -- door_pos(buyer) now returns None).
    let buyer_building = ctx
        .g
        .read::<BuildingInfos>()
        .building_owned_by(buyer)
        .unwrap();
    ctx.g.map_mut().remove_building(buyer_building);

    // Drive until the dispatch resolves (Returning -> removed).
    assert!(
        drain_or_return(&mut ctx, 6000),
        "dispatch never resolved after the buyer's building was demolished"
    );

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal),
        5,
        "goods must be physically driven back and re-credited to the seller, \
         never instant-refunded"
    );
    assert_eq!(
        m.capital(buyer, cereal),
        0,
        "the demolished buyer must never be credited for goods it can't receive"
    );
}

/// (gate defect 1) A human bread buy order with no domestic seller must NOT
/// be filled by the external market: bread has no `optout_exttrade` flag
/// (base_mod/items.lua), so without a human-specific carve-out the ext-trade
/// buy block would credit the human directly and attach a money_delta —
/// both forbidden on the retail path (money never gates food; the loaf is
/// only ever destroyed at eat-time, never conjured by paying for it). The
/// unmatched order must survive untouched so the human keeps re-polling
/// without unbounded state growth (no capital, no claim, no dispatch).
#[test]
fn scenario_human_order_never_fills_via_external_market() {
    let mut ctx = TestCtx::new();
    let buyer = mk_human((1 << 32) | 77);
    let ext = mk_soul_stub(); // stands in for `find_external`'s pick

    let mut m = ctx.g.write::<Market>();
    m.buy(buyer, geom::Vec2::ZERO, bread(), 1);

    // find_external always returns Some: if the human path is not carved
    // out, this is exactly the condition that fills it externally.
    for _ in 0..3 {
        let trades: Vec<_> = m
            .make_trades(|_| Some(ext))
            .iter()
            .filter(|t| t.kind == bread() && t.buyer.0 == buyer)
            .copied()
            .collect();
        assert!(
            trades.is_empty(),
            "a human bread order must never be filled by the external market"
        );
    }

    assert_eq!(
        m.capital(buyer, bread()),
        0,
        "no external fill means no capital credit"
    );
    assert!(
        m.retail_claim(buyer).is_none(),
        "no domestic match happened, so there must be no claim"
    );
    assert!(
        m.inner()[&bread()].buy_order(buyer).is_some(),
        "the unmatched order must survive so the human keeps re-polling, \
         not vanish (which would silently starve it with no re-buy signal)"
    );
}

fn mk_soul_stub() -> SoulID {
    mk_human((1 << 32) | 999)
}

/// (gate defect 2) A second domestic match for a buyer that still holds an
/// outstanding claim must not orphan the first claim's reservation: the old
/// claim's reservation is released before the new one overwrites it, so
/// total reserved stock never exceeds what's actually outstanding.
#[test]
fn scenario_double_match_does_not_orphan_reservation() {
    let mut ctx = TestCtx::new();
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    ctx.tick();
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    let buyer = mk_human((1 << 32) | 88);

    // First match: claim #1, reserved = 1.
    match_one_bread(&mut ctx, seller, seller_pos, buyer);
    assert_eq!(ctx.g.read::<Market>().reserved(seller, bread()), 1);

    // Second match for the SAME buyer while claim #1 is still outstanding
    // (buyfood would never do this itself; this simulates the path the gate
    // flagged as reachable via defect 1's re-buy loop).
    match_one_bread(&mut ctx, seller, seller_pos, buyer);

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.reserved(seller, bread()),
        1,
        "overwriting claim #1 with claim #2 must release #1's reservation, \
         not stack on top of it"
    );
    let claim = m.retail_claim(buyer).unwrap();
    assert_eq!(claim.qty, 1);
    drop(m);

    // Settling the surviving claim must bring reserved back to exactly 0,
    // never leaving an orphaned unit behind.
    ctx.g.write::<Market>().settle_retail(buyer);
    assert_eq!(ctx.g.read::<Market>().reserved(seller, bread()), 0);
}

/// (gate defect 3) TTL expiry mid-walk must not be a free meal: if the claim
/// expires before the human physically arrives, `BoughtAt` must NOT settle
/// or credit a meal -- `last_ate` stays put and the state resets to `Empty`
/// so the human goes without and re-queues.
#[test]
fn scenario_ttl_expired_arrival_is_not_a_free_meal() {
    let mut ctx = TestCtx::new();
    ctx.build_roads(&[
        geom::Vec3::new(0.0, 0.0, 0.0),
        geom::Vec3::new(200.0, 0.0, 0.0),
    ]);
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    let house = ctx.build_house_at(geom::Vec2::new(150.0, 20.0));
    ctx.tick();
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    let human_id = spawn_human(&mut ctx.g, house).expect("human must spawn");
    let buyer = SoulID::Human(human_id);

    match_one_bread(&mut ctx, seller, seller_pos, buyer);
    assert!(ctx.g.read::<Market>().retail_claim(buyer).is_some());

    // Force BuyFood directly into BoughtAt at the seller's building (as if
    // it had already walked there and matched), so this test isolates the
    // eat-vs-expired branch instead of re-deriving the earlier transitions.
    {
        let (world, _res) = ctx.g.world_res();
        world
            .humans
            .get_mut(human_id)
            .unwrap()
            .food
            .set_state_bought_at_for_test(seller_b);
    }

    let last_ate_before = ctx.g.world().humans.get(human_id).unwrap().food.last_ate;

    // Expire the claim (TTL sweep) WITHOUT the human having arrived yet.
    ctx.advance_ticks(TICKS_PER_HOUR as u32 + 2);
    assert!(ctx.g.read::<Market>().retail_claim(buyer).is_none());

    // Now "arrive": location must equal the seller building for `BoughtAt`
    // to attempt settlement.
    {
        let (world, _res) = ctx.g.world_res();
        let h = world.humans.get_mut(human_id).unwrap();
        h.location = crate::transportation::Location::Building(seller_b);
    }
    apply_buyfood(&mut ctx.g, human_id);
    ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);

    let last_ate_after = ctx.g.world().humans.get(human_id).unwrap().food.last_ate;
    assert_eq!(
        last_ate_before, last_ate_after,
        "arriving after the claim expired must NOT be treated as a meal"
    );
    assert_eq!(
        ctx.g.read::<Market>().capital(seller, bread()),
        1,
        "nothing must be settled/debited from the seller for an expired claim"
    );
}

/// (gate defect 4) `Returning` with no route home must terminate within a
/// bounded number of retries rather than wedge in `Loading` forever: sever
/// every road (the seller becomes physically unreachable) after the buyer's
/// building is demolished, so `Itinerary::route` back to the seller can
/// never succeed. The dispatch must still resolve (goods honestly lost, not
/// stuck), and the truck must be freed back to the Dispatcher.
#[test]
fn scenario_returning_with_severed_road_terminates() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 5);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }

    // Drive until the dispatch has debited the seller and entered Loading.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if !m.dispatches().is_empty() && m.dispatches()[0].state == DispatchState::Loading {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Loading");
    }

    // Demolish the buyer's building, then sever every road so the truck
    // (wherever it physically is) can never route back to the seller.
    let buyer_building = ctx
        .g
        .read::<BuildingInfos>()
        .building_owned_by(buyer)
        .unwrap();
    ctx.g.map_mut().remove_building(buyer_building);
    let road_ids: Vec<_> = ctx.g.map().roads().keys().collect();
    for r in road_ids {
        ctx.g.map_mut().remove_road(r);
    }

    // Drive well past MAX_RETURN_ROUTE_RETRIES worth of tick-retries; the
    // dispatch must resolve (get removed) rather than sit wedged forever.
    assert!(
        drain_or_return(&mut ctx, 6000),
        "a Returning dispatch with no route home must still terminate, not wedge forever"
    );

    // The goods are an honest physical loss (already debited from the
    // seller when the truck loaded, never returned since there was no road
    // to return them on) -- re-crediting here would be exactly the instant
    // teleport-refund the ruling forbids.
    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal),
        0,
        "no route home means no re-credit -- an honest loss, not a teleport"
    );
    assert_eq!(m.capital(buyer, cereal), 0);
}

/// (FIX 3) A truck that vanishes mid-`Returning` (e.g. despawned) must be
/// caught the same way every sibling state (`ToSource`/`Loading`/
/// `ToDestination`) already catches a vanished truck: dispatch removed,
/// truck freed, loss logged -- not left wedged forever waiting on an
/// itinerary that will never end because the vehicle it belongs to no
/// longer exists.
#[test]
fn scenario_dead_truck_while_returning_terminates() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    let truck =
        spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 5);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }

    // Demolish the buyer's building BEFORE the truck loads, so the dispatch
    // routes straight into `Returning` (route to the seller still exists;
    // only the buyer's door is gone) instead of `Loading`'s door_pos-None
    // wait.
    let buyer_building = ctx
        .g
        .read::<BuildingInfos>()
        .building_owned_by(buyer)
        .unwrap();
    ctx.g.map_mut().remove_building(buyer_building);

    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if !m.dispatches().is_empty() && m.dispatches()[0].state == DispatchState::Returning {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Returning");
    }
    assert_eq!(
        ctx.g.read::<Market>().capital(seller, cereal),
        0,
        "seller must already be debited by the time the truck is loading/returning"
    );

    // The truck vanishes mid-return.
    ctx.g.world_mut_unchecked().vehicles.remove(truck);

    ctx.tick();

    let m = ctx.g.read::<Market>();
    assert!(
        m.dispatches().is_empty(),
        "a dispatch whose truck vanished while Returning must not wedge forever"
    );
    assert_eq!(
        m.capital(seller, cereal),
        0,
        "goods already debited and lost with the truck are not re-credited -- an \
         honest physical loss, not a teleport"
    );
}

fn drain_or_return(ctx: &mut TestCtx, max_ticks: u32) -> bool {
    let mut spent = 0;
    while spent < max_ticks {
        let chunk = (max_ticks - spent).min(50);
        ctx.advance_ticks(chunk);
        spent += chunk;
        if ctx.g.read::<Market>().dispatches().is_empty() {
            return true;
        }
    }
    false
}

/// sov-xyx: `BoughtAt(b)` is an inescapable sink when `b` is demolished
/// mid-walk. Every exit from that arm needs `loc == Building(b)`, but
/// `routing_changed_system` marks the destination reached on a dead building
/// WITHOUT pushing `GetInBuilding` (`map_dynamic/router.rs`), so `loc` never
/// advances: the human re-emits `GoTo` forever, hunger ramps, and the bread
/// demand it represents silently vanishes from what the Planner observes.
/// Demolishing the store must return the customer to `Empty` so it re-queues,
/// WITHOUT advancing `last_ate` (went without, same as the expired-claim
/// branch).
#[test]
fn scenario_demolished_store_releases_bought_at_customer() {
    let mut ctx = TestCtx::new();
    ctx.build_roads(&[
        geom::Vec3::new(0.0, 0.0, 0.0),
        geom::Vec3::new(200.0, 0.0, 0.0),
    ]);
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    let house = ctx.build_house_at(geom::Vec2::new(150.0, 20.0));
    ctx.tick();
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    let human_id = spawn_human(&mut ctx.g, house).expect("human must spawn");
    let buyer = SoulID::Human(human_id);

    match_one_bread(&mut ctx, seller, seller_pos, buyer);

    // Force `BoughtAt(seller_b)` while the human is still at home: this is
    // the in-transit customer the ticket is about.
    {
        let (world, _res) = ctx.g.world_res();
        world
            .humans
            .get_mut(human_id)
            .unwrap()
            .food
            .set_state_bought_at_for_test(seller_b);
    }
    assert_ne!(
        ctx.g.world().humans.get(human_id).unwrap().location,
        crate::transportation::Location::Building(seller_b),
        "the customer must still be in transit for this test to mean anything"
    );

    let last_ate_before = ctx.g.world().humans.get(human_id).unwrap().food.last_ate;

    // Real demolition, through the same command the player's bulldozer uses.
    ctx.apply(&[WorldCommand::MapRemoveBuilding(seller_b)]);
    assert!(
        !ctx.g.map().buildings().contains_key(seller_b),
        "the store must actually be gone"
    );
    drop(ctx.g.map());
    // Why the guard needs `&Map` and not the `binfos` the ticket named:
    // nothing removes a demolished building from `BuildingInfos`, and
    // slotmapd's `SecondaryMap::get` only version-checks its own slot, so the
    // entry (and its `owner`) outlive the bulldozer. `binfos` is not a
    // liveness oracle for a BuildingID.
    assert!(
        ctx.g.read::<BuildingInfos>().get(seller_b).is_some(),
        "BuildingInfos still holds the demolished building -- so it cannot be \
         the existence check"
    );

    let decision = apply_buyfood(&mut ctx.g, human_id);
    ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);
    assert!(
        matches!(decision, crate::souls::human::HumanDecisionKind::Yield),
        "a demolished store must not keep emitting GoTo, got {:?}",
        decision
    );
    assert_eq!(
        last_ate_before,
        ctx.g.world().humans.get(human_id).unwrap().food.last_ate,
        "going without must NOT advance last_ate (never game over)"
    );

    // Back in `Empty`, the next decision re-queues a real buy order — the
    // observable proof the demand did not silently disappear.
    apply_buyfood(&mut ctx.g, human_id);
    ParCommandBuffer::<HumanEnt>::apply(&mut ctx.g);
    assert!(
        ctx.g
            .read::<Market>()
            .inner()
            .get(&bread())
            .and_then(|sm| sm.buy_order(buyer))
            .is_some(),
        "the customer must re-queue a bread buy order after the store died"
    );
}
