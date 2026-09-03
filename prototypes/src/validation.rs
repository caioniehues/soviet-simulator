#![allow(clippy::iter_over_hash_type)]

use thiserror::Error;

use common::error::MultiError;

use crate::{CompanyKind, Prototypes};

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{0}: only factories can have trucks")]
    WrongTrucks(String),
    #[error("{0}: factories must have trucks if it produces things")]
    ZeroTrucks(String),
    #[error("{0}.{1}: referenced prototype not found")]
    ReferencedProtoNotFound(String, &'static str),

    #[error("{0}.{1}: {2}")]
    InvalidField(String, &'static str, String),
}

pub(crate) fn validate(proto: &Prototypes) -> Result<(), MultiError<ValidationError>> {
    let mut errors = vec![];

    for comp in proto.goods_company.values() {
        if comp.n_trucks > 0 && comp.kind != CompanyKind::Factory {
            errors.push(ValidationError::WrongTrucks(comp.name.clone()));
        }

        if comp.n_trucks == 0
            && comp.kind == CompanyKind::Factory
            && comp
                .recipe
                .as_ref()
                .map(|r| !r.production.is_empty())
                .unwrap_or(false)
        {
            errors.push(ValidationError::ZeroTrucks(comp.name.clone()));
        }

        if let Some(ref r) = comp.recipe {
            for item in &r.consumption {
                if !proto.item.contains_key(&item.id) {
                    errors.push(ValidationError::ReferencedProtoNotFound(
                        comp.name.clone(),
                        "consumption",
                    ));
                }
            }

            for item in &r.production {
                if !proto.item.contains_key(&item.id) {
                    errors.push(ValidationError::ReferencedProtoNotFound(
                        comp.name.clone(),
                        "production",
                    ));
                }
            }

            // A recipe's numbers are consumed as unsigned quantities and as a
            // divisor, so a value below 1 does not merely look wrong -- it
            // inverts the meaning of the data. Refuse it here, at load, where
            // the company name is still available to say WHICH entry is bad.
            //
            // `request_multiplier` is read by `souls/goods_company.rs`
            // `recipe_init` as `item.amount as u32 * request_multiplier as u32`:
            // -3 wraps to 4_294_967_293, a standing request larger than the
            // whole economy, and 0 requests nothing at all, stalling the
            // enterprise forever. Both are silent.
            if r.request_multiplier < 1 {
                errors.push(ValidationError::InvalidField(
                    comp.name.clone(),
                    "recipe.request_multiplier",
                    format!("must be at least 1, got {}", r.request_multiplier),
                ));
            }

            // `storage_multiplier` is consumed as an unsigned quantity in
            // `souls/goods_company.rs` (`item.amount * storage_multiplier` as
            // u32, handed to the stock cap): -1 wraps to 4_294_967_295. And
            // with -1 the `recipe_should_produce` gate (`capital < amount *
            // (storage_multiplier + 1)`) compares against 0, so the
            // enterprise halts silently. Refuse it here like its sibling.
            if r.storage_multiplier < 1 {
                errors.push(ValidationError::InvalidField(
                    comp.name.clone(),
                    "recipe.storage_multiplier",
                    format!("must be at least 1, got {}", r.storage_multiplier),
                ));
            }

            // `duration` is the divisor in `souls/goods_company.rs:205`
            // (float division), so 0 does not panic -- it yields infinite
            // production, silently wrong. Refuse it here, at load, like its
            // siblings above.
            if r.duration.0.0 == 0 {
                errors.push(ValidationError::InvalidField(
                    comp.name.clone(),
                    "recipe.duration",
                    format!("must be positive, got {}", r.duration),
                ));
            }

            // A production `amount` is the divisor in `economy/market.rs`
            // `calculate_price_inner`, so 0 is an integer divide by zero -- a
            // panic on a live path. Consumption amounts are compared against
            // capital and multiplied into requests; below 1 is meaningless
            // there too.
            for (field, items) in [
                ("recipe.consumption", &r.consumption),
                ("recipe.production", &r.production),
            ] {
                for item in items {
                    if item.amount < 1 {
                        // Name the item, not its hashed id: this message is
                        // read by whoever is editing the Lua, and
                        // `ItemID(11802632242151335080)` tells them nothing.
                        // The id may be dangling, which the loop above already
                        // reports separately, so fall back to the raw id.
                        let item_name = proto
                            .item
                            .get(&item.id)
                            .map(|i| i.base.name.clone())
                            .unwrap_or_else(|| format!("{:?}", item.id));
                        errors.push(ValidationError::InvalidField(
                            comp.name.clone(),
                            field,
                            format!(
                                "amount for {} must be at least 1, got {}",
                                item_name, item.amount
                            ),
                        ));
                    }
                }
            }
        }

        if comp.power_consumption.map_or(false, |v| v.0 < 0) {
            errors.push(ValidationError::InvalidField(
                comp.name.clone(),
                "power_consumption",
                "must not be negative".to_string(),
            ));
        }

        if comp.power_production.map_or(false, |v| v.0 < 0) {
            errors.push(ValidationError::InvalidField(
                comp.name.clone(),
                "power_production",
                "must not be negative".to_string(),
            ));
        }
    }

    if !errors.is_empty() {
        return Err(MultiError(errors));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::try_parse_prototypes;

    /// A minimal but base_mod-shaped mill: consumes cereal, produces flour.
    /// `duration` and `storage_multiplier` are the knobs tests turn to bad
    /// values; every other number stays in range so a refusal names the knob.
    fn mill_fixture(duration: &str, storage_multiplier: i64) -> String {
        r#"
        data:extend {
          { type = "item", name = "cereal", label = "Cereal" },
          { type = "item", name = "flour",  label = "Flour"  }
        }

        data:extend {{
            type = "goods-company",
            name = "test-mill",
            label = "Test mill",
            kind = "factory",
            bgen = "farm",
            recipe = {
                production = {{"flour", 1}},
                consumption = {{"cereal", 1}},
                duration = "{DUR}",
                storage_multiplier = {SM},
                request_multiplier = 1,
            },
            n_trucks = 1,
            n_workers = 5,
            size = 0.0,
            asset = "no.jpg",
            price = 0,
        }}
        "#
        .replace("{DUR}", duration)
        .replace("{SM}", &storage_multiplier.to_string())
    }

    #[test]
    fn sov_snw_negative_storage_multiplier_is_refused() {
        let err = try_parse_prototypes(&mill_fixture("10m", -1))
            .expect_err("storage_multiplier = -1 must be REFUSED at load, not wrapped to 4294967295");
        let msg = err.to_string();
        assert!(
            msg.contains("test-mill") && msg.contains("storage_multiplier") && msg.contains("-1"),
            "the error must name the company, the field and the offending value, got: {msg}"
        );
    }

    #[test]
    fn sov_0o0_zero_duration_is_refused() {
        let err = try_parse_prototypes(&mill_fixture("0s", 1))
            .expect_err("duration = \"0s\" must be REFUSED at load, not divide by zero downstream");
        let msg = err.to_string();
        assert!(
            msg.contains("test-mill") && msg.contains("duration") && msg.contains("0s"),
            "the error must name the company, the field and the offending value, got: {msg}"
        );
    }

    #[test]
    fn sov_0vz_wrong_typed_request_multiplier_is_refused() {
        let lua =
            mill_fixture("10m", 1).replace("request_multiplier = 1", "request_multiplier = \"four\"");
        let err = try_parse_prototypes(&lua)
            .expect_err("request_multiplier = \"four\" must be REFUSED at load, not defaulted to honest-1");
        let msg = err.to_string();
        assert!(
            msg.contains("request_multiplier"),
            "the error must name the offending field, got: {msg}"
        );
    }
}
