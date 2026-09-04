//! sov-eix: the border clears on freight-station THROUGHPUT — a declared
//! per-tick per-station capacity in units — never on price. Demand beyond the
//! capacity is served partially and the remainder QUEUES for the next pass:
//! it is never refused (the order/offer survives) and never priced out
//! (no money gate anywhere on this path).

use super::*;

use super::hoarding::mk_soul;
use crate::economy::Market;
use prototypes::ItemID;

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

/// sov-eix, export half: a 250-unit surplus against one station serves 100
/// per pass and keeps the remainder offered — queueing, not refusal.
#[test]
fn sov_eix_export_excess_queues_across_passes() {
    let _ctx = TestCtx::new();
    let mut m = Market::default();
    let ext = mk_soul((1 << 32) | 99); // stands in for `find_external`'s pick
    let seller = mk_soul((1 << 32) | 1);
    m.produce(seller, cereal(), 250);
    m.sell(seller, Vec2::ZERO, cereal(), 250, 0);

    // Pass 1: exactly one capacity worth exports, the rest stays offered.
    let pass1: Vec<_> = m
        .make_trades(|_| Some(ext))
        .iter()
        .filter(|t| t.kind == cereal())
        .copied()
        .collect();
    assert_eq!(pass1.len(), 1);
    assert_eq!(pass1[0].qty, 100, "one pass moves one capacity");
    assert_eq!(
        m.inner()[&cereal()].sell_order(seller).unwrap().qty,
        150,
        "the unserved surplus must stay offered, not refused"
    );
    // Pass 2: the queue drains further — never stuck, never dropped. Only
    // the unreserved remainder is free to export (pass 1's 100 is spoken
    // for by its dispatch), so this pass serves 50, not a second 100.
    let pass2: Vec<_> = m
        .make_trades(|_| Some(ext))
        .iter()
        .filter(|t| t.kind == cereal())
        .copied()
        .collect();
    assert_eq!(pass2.len(), 1);
    assert_eq!(pass2[0].qty, 50, "the free remainder exports next");
    assert_eq!(
        m.inner()[&cereal()].sell_order(seller).unwrap().qty,
        100,
        "the remainder keeps queueing until served"
    );
    assert_eq!(m.reserved(seller, cereal()), 150);
}

/// sov-eix, import half: a 250-unit buy order against one station imports
/// 100 per pass and the remainder stays ordered.
#[test]
fn sov_eix_import_excess_queues_across_passes() {
    let _ctx = TestCtx::new();
    let mut m = Market::default();
    let ext = mk_soul((1 << 32) | 99); // stands in for `find_external`'s pick
    let buyer = mk_soul((1 << 32) | 2);
    m.buy(buyer, Vec2::ZERO, cereal(), 250);

    // Pass 1: the fresh order queues for a domestic pass (sov-b70).
    assert_eq!(m.make_trades(|_| Some(ext)).len(), 0);

    // Pass 2: eligible — one capacity worth imports, the rest stays ordered.
    let pass2: Vec<_> = m
        .make_trades(|_| Some(ext))
        .iter()
        .filter(|t| t.kind == cereal())
        .copied()
        .collect();
    assert_eq!(pass2.len(), 1);
    assert_eq!(pass2[0].qty, 100, "one pass moves one capacity");
    assert_eq!(
        pass2[0].buyer.0, buyer,
        "a partial fill still names the queuing buyer"
    );
    assert_eq!(
        m.inner()[&cereal()]
            .buy_order(buyer)
            .map(|o| o.qty),
        Some(150),
        "the unserved demand must stay ordered, not refused"
    );
}
