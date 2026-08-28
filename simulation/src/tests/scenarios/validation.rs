//! sov-k3w: proves that a bad numeric value in a base_mod-shaped recipe is
//! REFUSED at load, rather than being parsed into a value whose meaning is the
//! opposite of what the data says.
//!
//! Why these tests live in `simulation` and not in `prototypes`: the failure
//! this ticket exists for is a *data-layer* failure, and `cargo test -p
//! prototypes` is not evidence about it. A cereal production `amount` of 0
//! passes `cargo test -p prototypes` (5 passed) while killing `cargo test -p
//! simulation` with 20+ failures, because `market.rs`'s `calculate_prices`
//! divides by that amount. The two consumers that give these numbers their
//! meaning both live in this crate:
//!
//!   * `souls/goods_company.rs` `recipe_init`:
//!     `item.amount as u32 * recipe.request_multiplier as u32`. A
//!     `request_multiplier` of -3 wraps to 4_294_967_293 -- a standing market
//!     request larger than the whole economy, with no panic and no log.
//!     A `request_multiplier` of 0 is the sibling case: a silent permanent
//!     stall, requesting nothing and so never producing.
//!   * `economy/market.rs` `calculate_price_inner`: `... / qty`, where `qty`
//!     is the production amount. An amount of 0 is an integer divide by zero
//!     -> panic, i.e. game over, which this project's pillars forbid outright.
//!
//! `try_parse_prototypes` runs the real `parse` + `validate` path used by
//! `load_prototypes`, but returns the error instead of installing the result,
//! so a test can observe the refusal without panicking and without touching
//! the thread-local prototype slot the rest of this binary relies on.

use prototypes::try_parse_prototypes;

/// A minimal but base_mod-shaped mill: consumes cereal, produces flour.
/// `production_amount`, `consumption_amount` and `recipe_extra` are the knobs
/// each test below turns to exactly one bad value.
fn mill_fixture(production_amount: i64, consumption_amount: i64, recipe_extra: &str) -> String {
    format!(
        r#"
        data:extend {{
          {{ type = "item", name = "cereal", label = "Cereal" }},
          {{ type = "item", name = "flour",  label = "Flour"  }}
        }}

        data:extend {{{{
            type = "goods-company",
            name = "test-mill",
            label = "Test mill",
            kind = "factory",
            bgen = "farm",
            recipe = {{
                production = {{{{"flour", {production_amount}}}}},
                consumption = {{{{"cereal", {consumption_amount}}}}},
                duration = "10m",
                storage_multiplier = 5,
                {recipe_extra}
            }},
            n_trucks = 1,
            n_workers = 5,
            size = 0.0,
            asset = "no.jpg",
            price = 0,
        }}}}
        "#
    )
}

/// Control: the same fixture with every number in range must LOAD. Without
/// this, every assertion below could pass for the wrong reason -- a fixture
/// that is simply malformed is refused too, and would look identical.
#[test]
fn sov_k3w_good_recipe_is_accepted() {
    try_parse_prototypes(&mill_fixture(1, 1, "request_multiplier = 4,"))
        .expect("a recipe with production 1, consumption 1, request_multiplier 4 must load");

    // request_multiplier is optional; omitting it must still load (24 of the
    // 26 base_mod recipes do exactly this and take the default of 1).
    try_parse_prototypes(&mill_fixture(1, 1, ""))
        .expect("an omitted request_multiplier must still load, defaulting to 1");
}

#[test]
fn sov_k3w_negative_request_multiplier_is_refused() {
    let err = try_parse_prototypes(&mill_fixture(1, 1, "request_multiplier = -3,"))
        .expect_err("request_multiplier = -3 must be REFUSED at load, not wrapped to 4294967293");
    let msg = err.to_string();
    assert!(
        msg.contains("test-mill") && msg.contains("request_multiplier") && msg.contains("-3"),
        "the error must name the company, the field and the offending value, got: {msg}"
    );
}

#[test]
fn sov_k3w_zero_request_multiplier_is_refused() {
    let err = try_parse_prototypes(&mill_fixture(1, 1, "request_multiplier = 0,"))
        .expect_err("request_multiplier = 0 must be REFUSED at load, not become a silent stall");
    let msg = err.to_string();
    assert!(
        msg.contains("test-mill") && msg.contains("request_multiplier"),
        "the error must name the company and the field, got: {msg}"
    );
}

/// The divide-by-zero case: `market.rs`'s `calculate_price_inner` divides the
/// recipe's cost by its production amount.
#[test]
fn sov_k3w_zero_production_amount_is_refused() {
    let err = try_parse_prototypes(&mill_fixture(0, 1, ""))
        .expect_err("a production amount of 0 must be REFUSED at load");
    let msg = err.to_string();
    assert!(
        msg.contains("test-mill") && msg.contains("production"),
        "the error must name the company and the field, got: {msg}"
    );
}

#[test]
fn sov_k3w_negative_recipe_amounts_are_refused() {
    let err = try_parse_prototypes(&mill_fixture(-2, 1, ""))
        .expect_err("a negative production amount must be REFUSED at load");
    assert!(
        err.to_string().contains("production"),
        "got: {}",
        err.to_string()
    );

    let err = try_parse_prototypes(&mill_fixture(1, 0, ""))
        .expect_err("a consumption amount of 0 must be REFUSED at load");
    assert!(
        err.to_string().contains("consumption"),
        "got: {}",
        err.to_string()
    );

    let err = try_parse_prototypes(&mill_fixture(1, -5, ""))
        .expect_err("a negative consumption amount must be REFUSED at load");
    assert!(
        err.to_string().contains("consumption"),
        "got: {}",
        err.to_string()
    );
}
