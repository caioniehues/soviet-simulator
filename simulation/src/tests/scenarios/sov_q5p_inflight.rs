//! sov-q5p: `buy_until` covers in-flight inbound quantity, so imports do
//! not stack and hoards do not amplify.
//!
//! `buy_until` used to read raw capital only: a company that re-posted while
//! an import was en route drew a SECOND import, and with an unbounded border
//! a k=4 hoarder stacked 1+2+3… units in flight against a target of 4. Now
//! the order is only the CURRENT shortfall after stock on hand AND goods
//! already inbound (`Market::inbound_to`: matched but undelivered
//! `ToSource`/`Loading`/`ToDestination` dispatches).
//!
//! The honest-vs-dishonest signal survives: an honest k=1 enterprise consumes
//! to 0 and stalls with nothing inbound, while the hoarder's surplus is
//! capital on hand — asserted below alongside the bound.

use super::*;

use super::hoarding::mk_soul;
use crate::economy::Market;
use prototypes::ItemID;

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

fn order_qty(m: &Market, soul: crate::SoulID) -> u32 {
    m.inner()[&cereal()]
        .buy_order(soul)
        .map(|o| o.qty)
        .unwrap_or(0)
}

/// sov-q5p: a k=4 hoarder with a reachable border holds at most its target
/// across many cycles (no stacking), while still claiming strictly more than
/// an honest k=1 enterprise (signal nonzero).
#[test]
fn sov_q5p_hoarder_holds_target_dishonest_signal_survives() {
    let _ctx = TestCtx::new();
    let mut m = Market::default();
    let ext = mk_soul((1 << 32) | 99); // stands in for `find_external`'s pick
    let hoarder = mk_soul((1 << 32) | 11);
    let honest = mk_soul((1 << 32) | 12);
    m.set_requested(hoarder, cereal(), 4);
    m.set_requested(honest, cereal(), 1);

    let mut peak_held = 0u32;
    for _ in 0..6 {
        for (soul, consumption) in [(hoarder, 1), (honest, 1)] {
            // Both consume exactly what their recipe needs, same as
            // `recipe_act`, then re-request up to their reported quantity.
            let have = m.capital(soul, cereal());
            m.produce(soul, cereal(), -have.min(consumption));
            let want = m.requested(soul, cereal()).unwrap();
            m.buy_until(soul, Vec2::ZERO, cereal(), want);
        }
        let _ = m.make_trades(|_| Some(ext)).len();
        let held = (m.capital(hoarder, cereal()).max(0) as u32)
            + m.inbound_to(hoarder, cereal())
            + order_qty(&m, hoarder);
        peak_held = peak_held.max(held);
    }

    assert!(
        peak_held <= 4,
        "a k=4 hoarder must hold at most its target across cycles, not stack imports: {peak_held}"
    );
    assert_eq!(
        m.inbound_to(hoarder, cereal()),
        4,
        "exactly one target worth of goods may be in flight"
    );

    let hoarder_total = (m.capital(hoarder, cereal()).max(0) as u32)
        + m.inbound_to(hoarder, cereal())
        + order_qty(&m, hoarder);
    let honest_total = (m.capital(honest, cereal()).max(0) as u32)
        + m.inbound_to(honest, cereal())
        + order_qty(&m, honest);
    assert!(
        hoarder_total > honest_total,
        "the honest-vs-dishonest detection signal must stay nonzero: hoarder {hoarder_total} vs honest {honest_total}"
    );
}
