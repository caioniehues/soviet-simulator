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
