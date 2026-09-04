//! sov-b70: the border is the RESIDUAL supplier, not the default one. An
//! unmatched non-human buy order ages one market pass (`BuyOrder.age`) before
//! it becomes border-eligible, so a domestic producer that gains stock the
//! next tick wins the order instead of the border. Domestic matching ignores
//! age entirely — it already runs first every pass.

use super::*;

use super::hoarding::mk_soul;
use crate::economy::Market;
use prototypes::ItemID;

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

/// sov-b70: a fresh buy order queues one pass instead of going abroad at
/// once, then imports once eligible — the delay is exactly one pass, not a
/// permanent block.
#[test]
fn sov_b70_fresh_order_queues_one_pass_then_imports() {
    let _ctx = TestCtx::new();
    let mut m = Market::default();
    let ext = mk_soul((1 << 32) | 99); // stands in for `find_external`'s pick
    let buyer = mk_soul((1 << 32) | 2);
    m.buy(buyer, Vec2::ZERO, cereal(), 5);

    // Pass 1: too young for the border — the order survives, nothing trades.
    let pass1 = m.make_trades(|_| Some(ext)).len();
    assert_eq!(
        pass1, 0,
        "a fresh order must queue for a domestic pass before the border may take it"
    );
    assert_eq!(
        m.inner()[&cereal()]
            .buy_order(buyer)
            .map(|o| o.qty),
        Some(5),
        "the queued order must survive the pass untouched"
    );

    // Pass 2: still no domestic seller — now eligible, so it imports.
    let pass2: Vec<_> = m
        .make_trades(|_| Some(ext))
        .iter()
        .filter(|t| t.kind == cereal())
        .copied()
        .collect();
    assert_eq!(pass2.len(), 1, "the aged order must import: {pass2:?}");
    assert_eq!(pass2[0].buyer.0, buyer);
    assert_eq!(pass2[0].qty, 5);
}

/// sov-b70: a domestic producer that gains stock one tick after the order
/// was posted fills it DOMESTICALLY, not by import.
#[test]
fn sov_b70_domestic_next_tick_wins_over_border() {
    let _ctx = TestCtx::new();
    let mut m = Market::default();
    let ext = mk_soul((1 << 32) | 99); // stands in for `find_external`'s pick
    let seller = mk_soul((1 << 32) | 1);
    let buyer = mk_soul((1 << 32) | 2);
    let seller_pos = Vec2::new(30.0, 20.0);
    m.buy(buyer, Vec2::ZERO, cereal(), 5);

    // Pass 1: no domestic seller yet — the order queues, no import.
    assert_eq!(m.make_trades(|_| Some(ext)).len(), 0);

    // Pass 2 ("next tick"): the domestic producer gains stock and offers.
    m.produce(seller, cereal(), 5);
    m.sell(seller, seller_pos, cereal(), 5, 0);
    let pass2: Vec<_> = m
        .make_trades(|_| Some(ext))
        .iter()
        .filter(|t| t.kind == cereal())
        .copied()
        .collect();
    assert_eq!(
        pass2.len(),
        1,
        "the order must fill domestically once stock exists: {pass2:?}"
    );
    assert_eq!(pass2[0].seller.0, seller);
    assert_eq!(pass2[0].buyer.0, buyer);
    assert!(
        m.dispatches()
            .iter()
            .any(|d| d.kind == cereal() && d.seller == seller && d.buyer == buyer),
        "the fill must ride a domestic dispatch, not an import: {:?}",
        m.dispatches()
    );
}
