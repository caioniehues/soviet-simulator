//! Pins claimed-PROVIDED behavior of the recipe/production machinery inherited
//! from the Egregoria fork (STORY-0093..0097). These are tripwires for later
//! rewrites of `Recipe`, `recipe_should_produce`/`recipe_act`, and
//! `CompanyEnt::raw_productivity` — not corpus scenario/journey cards.

use super::*;

use crate::economy::{Government, Market};
use crate::souls::goods_company::{recipe_act, recipe_init, recipe_should_produce};
use crate::souls::human::spawn_human;
use crate::SoulID;
use prototypes::{
    GameDuration, GoodsCompanyID, GoodsCompanyPrototype, ItemID, Money, Recipe, RecipeItem, Tick,
};

/// Builds a real goods-company building from a loaded prototype (not a
/// synthetic one), mirroring `TestCtx::build_house_at`'s explicit placement.
fn build_company_at(ctx: &mut TestCtx, proto: &GoodsCompanyPrototype, p: Vec2) -> BuildingID {
    let obb = OBB::new(p, Vec2::X, proto.base.size.w, proto.base.size.h);
    let b = ctx
        .g
        .map_mut()
        .build_special_building(
            &obb,
            BuildingKind::GoodsCompany(proto.id),
            proto.base.bgen,
            None,
            None,
        )
        .unwrap();
    ctx.g.write::<BuildingInfos>().insert(b);
    b
}

fn dur(secs: i64) -> GameDuration {
    GameDuration(Tick(secs as u64 * prototypes::TICKS_PER_SECOND as u64))
}

/// STORY-0093 AC-1: a Recipe can declare 2+ inputs and 2+ outputs, each with
/// an independent quantity, transformed together under one shared duration.
/// STORY-0093 AC-2: a building type binds to exactly one recipe (a bakery is
/// never seen producing flour-factory's recipe, or vice versa).
#[test]
fn scenario_0093_recipe_multi_input_multi_output() {
    let mut ctx = TestCtx::new();
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let b = build_company_at(&mut ctx, bakery, Vec2::new(100.0, 20.0));
    ctx.tick(); // company_soul spawns on the next tick, like houses

    let soul = ctx.g.read::<BuildingInfos>().owner(b).unwrap();

    // Items untouched by the bakery's own recipe (flour/bread/job-opening),
    // so this synthetic recipe can't be polluted by the real company_system.
    let recipe = Recipe {
        consumption: vec![
            RecipeItem {
                id: ItemID::new("wool"),
                amount: 2,
            },
            RecipeItem {
                id: ItemID::new("cloth"),
                amount: 3,
            },
        ],
        production: vec![
            RecipeItem {
                id: ItemID::new("furniture"),
                amount: 1,
            },
            RecipeItem {
                id: ItemID::new("high-tech-product"),
                amount: 2,
            },
        ],
        duration: dur(50),
        storage_multiplier: 5,
        request_multiplier: 1,
    };
    assert!(recipe.consumption.len() >= 2);
    assert!(recipe.production.len() >= 2);

    let pos = Vec2::ZERO;
    {
        let mut market = ctx.g.write::<Market>();
        recipe_init(&recipe, soul, pos, &mut market);
        // simulate having exactly enough of each input, in its own amount
        market.produce(soul, ItemID::new("wool"), 2);
        market.produce(soul, ItemID::new("cloth"), 3);
        assert!(recipe_should_produce(&recipe, soul, &market));

        recipe_act(&recipe, soul, pos, &mut market);

        // both inputs consumed, in their own declared quantities
        assert_eq!(market.capital(soul, ItemID::new("wool")), 0);
        assert_eq!(market.capital(soul, ItemID::new("cloth")), 0);
        // both outputs produced, in their own declared quantities
        assert_eq!(market.capital(soul, ItemID::new("furniture")), 1);
        assert_eq!(market.capital(soul, ItemID::new("high-tech-product")), 2);
    }

    // AC-2: one building type, one bound recipe — bakery and flour-factory
    // are different building types and carry different, non-interchangeable
    // recipes (never a shared swappable "recipe category").
    let flour_factory = GoodsCompanyID::new("flour-factory").prototype();
    let bakery_recipe = bakery.recipe.as_ref().unwrap();
    let flour_recipe = flour_factory.recipe.as_ref().unwrap();
    assert_ne!(
        bakery_recipe.production[0].id, flour_recipe.production[0].id,
        "two building types must not resolve to the same recipe"
    );
}

/// STORY-0094 AC-1: a Recipe with an empty inputs list is valid and produces
/// at its declared rate with no inputs ever consumed (extraction buildings).
/// Grounded in the real "cereal-farm" prototype, not a synthetic recipe.
#[test]
fn scenario_0094_extraction_no_consumed_inputs() {
    let mut ctx = TestCtx::new();
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let b = build_company_at(&mut ctx, bakery, Vec2::new(100.0, 20.0));
    ctx.tick();
    let soul = ctx.g.read::<BuildingInfos>().owner(b).unwrap();

    let cereal_farm = GoodsCompanyID::new("cereal-farm").prototype();
    let recipe = cereal_farm.recipe.as_ref().unwrap();
    assert!(
        recipe.consumption.is_empty(),
        "cereal-farm is authored as an extraction recipe with no inputs"
    );

    let pos = Vec2::ZERO;
    let mut market = ctx.g.write::<Market>();
    recipe_init(recipe, soul, pos, &mut market);
    assert!(recipe_should_produce(recipe, soul, &market));

    recipe_act(recipe, soul, pos, &mut market);
    assert_eq!(
        market.capital(soul, ItemID::new("cereal")),
        recipe.production[0].amount,
        "output produced despite consuming nothing"
    );
}

/// STORY-0095 AC-1: full output storage halts production (rate -> 0) until
/// space frees, then production resumes automatically. This is the cascade
/// engine's core gate, `recipe_should_produce`'s storage check.
#[test]
fn scenario_0095_full_output_storage_halts_production() {
    let mut ctx = TestCtx::new();
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let b = build_company_at(&mut ctx, bakery, Vec2::new(100.0, 20.0));
    ctx.tick();
    let soul = ctx.g.read::<BuildingInfos>().owner(b).unwrap();

    // storage_multiplier 0 => halt threshold is exactly 1 unit of "oil"
    let recipe = Recipe {
        consumption: vec![],
        production: vec![RecipeItem {
            id: ItemID::new("oil"),
            amount: 1,
        }],
        duration: dur(10),
        storage_multiplier: 0,
        request_multiplier: 1,
    };

    let mut market = ctx.g.write::<Market>();
    recipe_init(&recipe, soul, Vec2::ZERO, &mut market);

    assert!(
        recipe_should_produce(&recipe, soul, &market),
        "empty output storage must allow production"
    );

    // fill output storage to the halt threshold
    market.produce(soul, ItemID::new("oil"), 1);
    assert!(
        !recipe_should_produce(&recipe, soul, &market),
        "full output storage must halt production (rate -> 0)"
    );

    // free the space back up
    market.produce(soul, ItemID::new("oil"), -1);
    assert!(
        recipe_should_produce(&recipe, soul, &market),
        "production must resume automatically once output space frees"
    );
}

/// STORY-0096 AC-1: workforce is read live from `n_workers` present at the
/// building each call, never drawn from a stored labour stock. Proven by
/// mutating `workers.0` directly (the only thing `raw_productivity` reads)
/// and observing an immediate change with no separate stock bookkeeping.
#[test]
fn scenario_0096_workforce_sourced_live_from_present_population() {
    let mut ctx = TestCtx::new();
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(200.0, 0.0, 0.0)]);
    let house = ctx.build_house_at(Vec2::new(20.0, 20.0));

    let bakery = GoodsCompanyID::new("bakery").prototype(); // n_workers = 3
    let b = build_company_at(&mut ctx, bakery, Vec2::new(100.0, 20.0));
    ctx.tick();
    let soul = ctx.g.read::<BuildingInfos>().owner(b).unwrap();
    let SoulID::GoodsCompany(company_id) = soul else {
        panic!("expected a GoodsCompany soul");
    };

    let h1 = spawn_human(&mut ctx.g, house).unwrap();
    let h2 = spawn_human(&mut ctx.g, house).unwrap();
    let h3 = spawn_human(&mut ctx.g, house).unwrap();

    let world = ctx.g.world_mut_unchecked();
    let company = world.companies.get_mut(company_id).unwrap();

    assert_eq!(company.raw_productivity(bakery, None), 0.0);

    company.workers.0 = vec![h1, h2, h3];
    assert!((company.raw_productivity(bakery, None) - 1.0).abs() < 1e-6);

    // Directly shrinking the present-population list changes productivity
    // immediately, with no separate stockpile to drain first.
    company.workers.0 = vec![h1];
    assert!((company.raw_productivity(bakery, None) - 1.0 / 3.0).abs() < 1e-6);
}

/// STORY-0097 AC-1: no internal recipe execution path reads or checks
/// Money/treasury before running; production gates are physical factors
/// only. Proven behaviorally: production proceeds with a deeply negative
/// government treasury.
#[test]
fn scenario_0097_production_never_checks_treasury() {
    let mut ctx = TestCtx::new();
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let b = build_company_at(&mut ctx, bakery, Vec2::new(100.0, 20.0));
    ctx.tick();
    let soul = ctx.g.read::<BuildingInfos>().owner(b).unwrap();

    ctx.g.write::<Government>().money = Money::new_bucks(-1_000_000_000);

    let recipe = Recipe {
        consumption: vec![],
        production: vec![RecipeItem {
            id: ItemID::new("coal"),
            amount: 1,
        }],
        duration: dur(10),
        storage_multiplier: 5,
        request_multiplier: 1,
    };

    let mut market = ctx.g.write::<Market>();
    recipe_init(&recipe, soul, Vec2::ZERO, &mut market);
    assert!(
        recipe_should_produce(&recipe, soul, &market),
        "a bankrupt treasury must not gate production"
    );
    recipe_act(&recipe, soul, Vec2::ZERO, &mut market);
    assert_eq!(market.capital(soul, ItemID::new("coal")), 1);
}
