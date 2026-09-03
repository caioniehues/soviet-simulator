//! ADR-0002: the fixture world is a materialised replay, never an authored
//! save. This module is the *canonical definition of the minimum city* and the
//! only sanctioned way to regenerate `world_replay.json`.
//!
//! It issues `WorldCommand`s through `Simulation::tick` (never `map_mut`
//! directly, never mouse input) so every placement lands in the `Replay`
//! resource at the tick it was applied, exactly as an in-game session would
//! record it. Houses are `MapBuildSpecialBuilding { kind: House }` with
//! explicit footprints: `MapBuildHouse(LotID)` is forbidden here because it
//! depends on auto-generated lots, which STORY-0013 removes (ADR-0002,
//! decision driver 4).
//!
//! The rail loop, the `RailFreightStation` and the `AddTrain` are NOT issued
//! here: `Simulation::new_with_options` unconditionally applies `START_COMMANDS`
//! (lib.rs), and because `save_replay` is on, those 12 commands are recorded
//! into the replay at tick 1 just like the ones below. This file adds what
//! `START_COMMANDS` lacks: vehicle roads, houses and companies.
//!
//! Regenerate with:
//!
//!   cargo test -p simulation regenerate_fixture_replay -- --ignored --nocapture
//!
//! The test is `#[ignore]`d so CI never rewrites the committed baseline; moving
//! the determinism baseline is a deliberate, stated act.

use super::*;

use crate::map::terrain::Tree;
use crate::map::{Environment, LaneKind, LanePattern, Map, ProjectKind, RoadID, Zone};
use crate::world_command::WorldCommand;
use crate::Replay;
use geom::{vec2, Intersect, Polygon, AABB};
use prototypes::{GoodsCompanyID, GoodsCompanyPrototype};

/// The level-0 `EcoStats` history needs `128 * LEVEL_FREQS[0] = 32_000` ticks to
/// fill (economy/ecostats.rs); the tail overshoots it so the Economy panel has a
/// full graph the moment the fixture loads. It also overshoots a lot further
/// than that: the station spur is ~1.3 km and import trucks average well under
/// 1 m/s through the commuting grid, so the first import deliveries land
/// ~15-20k ticks in and the domestic chains behind them need tens of thousands
/// more. Level 0 only shows the last 32k ticks -- the tail must be long enough
/// that steady trade, not the bootstrap, fills it. It must NOT be much
/// longer: past ~300k the corridor convoys lock into a permanent 0.0 m
/// deadlock and deliveries stop entirely, so the tail ends inside the fluid
/// phase, not the deadlocked one.
const TAIL_TICK: u64 = 200_000;

/// Vehicle-road grid: 3 columns x 4 rows of intersections, i.e. 2x3 blocks, in
/// the same coordinate neighbourhood the previous replay used (the terrain there
/// is known-buildable land, clear of the `START_COMMANDS` rail line to the west).
const XS: [f32; 3] = [4200.0, 4600.0, 5000.0];
const YS: [f32; 4] = [3800.0, 4200.0, 4600.0, 5000.0];

/// The companies of the minimum city, by base_mod prototype name.
/// `coal-power-plant` is first and deliberate: every other entry declares a
/// `power_consumption`, and a company in a blacked-out electricity network has
/// `productivity == 0` (souls/goods_company.rs), which would make the whole
/// fixture economy idle.
const COMPANIES: [&str; 12] = [
    "coal-power-plant",
    "cereal-farm",
    "flour-factory",
    "bakery",
    "supermarket",
    "wool-farm",
    "cloth-factory",
    "clothes-store",
    "coal-mine",
    "oil-pump",
    "florist",
    "horticulturalist",
];

const N_HOUSES: usize = 30;

/// One buildable spot beside a vehicle road, in the same terms the in-game
/// special-building tool uses (`gui/tools/specialbuilding.rs`): a projection
/// onto the road centreline plus the side to build on.
struct Slot {
    road: RoadID,
    road_width: f32,
    proj: Vec2,
    side: Vec2,
}

/// One `MapMakeConnection`, with both ends resolved against the *current* map
/// exactly as the road tool does (`Map::project`), then applied as its own tick.
///
/// One command per tick is not decoration. `Map::make_connection` merges a
/// degree-2 intersection whose two roads are collinear (map.rs:196-208), which
/// invalidates the `RoadID` it just returned -- `MapMakeMultipleConnections`
/// then panics on `map.roads[r]` (world_command.rs:268). Building line by line
/// and re-projecting in between is what a player does, and it lets the
/// perpendicular crossings split the long roads instead of stitching collinear
/// stubs together.
fn connect(sim: &mut Simulation, sched: &mut SeqSchedule, a: Vec2, b: Vec2, pat: &LanePattern) {
    let (from, to) = {
        let map = sim.map();
        let filter = ProjectFilter::ROAD | ProjectFilter::INTER;
        (
            map.project(a.z(0.0), 5.0, filter),
            map.project(b.z(0.0), 5.0, filter),
        )
    };
    sim.tick(
        sched,
        &[WorldCommand::MapMakeConnection {
            from,
            to,
            inter: None,
            pat: pat.clone(),
        }],
    );
}

/// The vehicle-road grid: three full-length avenues, then the cross streets,
/// which split the avenues into blocks.
fn lay_grid(sim: &mut Simulation, sched: &mut SeqSchedule) {
    let pat = LanePatternBuilder::default().build();

    let (ymin, ymax) = (YS[0], YS[YS.len() - 1]);
    for &x in XS.iter() {
        connect(sim, sched, vec2(x, ymin), vec2(x, ymax), &pat);
    }
    for &y in YS.iter() {
        for ix in 0..XS.len() - 1 {
            connect(sim, sched, vec2(XS[ix], y), vec2(XS[ix + 1], y), &pat);
        }
    }
}

/// sov-rvu: a spur from the grid to the `START_COMMANDS` freight station door.
///
/// A default city cannot trade across the border, deliberately (sov-ie6):
/// `market_update` only offers a freight station whose door is within
/// `DISPATCH_LANE_CUTOFF` (50 m) of a `Driving` lane, and the hardcoded
/// station sits ~1.3 km north of the grid with rail only. The consequence
/// reaches further than the border: with no external trader,
/// `Market::make_trades`' buy leg drops every unmatched non-human buy order,
/// so the companies' `recipe_init` orders vanish on the first ticks, late
/// production (40-200 s recipes on partial staffing) never finds a buyer, and
/// the fixture records zero imports/exports and zero goods trades. Joining
/// the grid to the door summons the first import dispatch and bootstraps the
/// domestic chains (bread, flower) behind it -- the train is the reward for
/// connecting the station by road, exactly as the game intends.
fn lay_station_road(sim: &mut Simulation, sched: &mut SeqSchedule) {
    let (door3, centre) = {
        let map = sim.map();
        let (_, b) = map
            .buildings()
            .iter()
            .find(|(_, b)| matches!(b.kind, BuildingKind::RailFreightStation(_)))
            .expect("START_COMMANDS must have seeded a RailFreightStation");
        (b.door_pos, b.obb.center())
    };
    // Stop 30 m past the door away from the building centre: inside the 50 m
    // dispatch cutoff, but clear of the footprint (an endpoint projecting
    // onto a building hits `unreachable!()` in `make_connection`).
    let away = (door3.xy() - centre).try_normalize().unwrap_or(Vec2::X);
    let end = door3.xy() + away * 30.0;
    let pat = LanePatternBuilder::default().build();
    connect(sim, sched, vec2(XS[0], YS[YS.len() - 1]), end, &pat);
    assert!(
        sim.map()
            .nearest_lane(
                door3,
                LaneKind::Driving,
                Some(crate::map_dynamic::DISPATCH_LANE_CUTOFF),
            )
            .is_some(),
        "the spur must put a driving lane within reach of the station door \
         {door3:?}, or the border stays closed and the fixture stays dead"
    );
}

/// Walks every non-rail road at a fixed spacing and emits a slot on each side.
/// Iteration order is the road slotmap's, which is insertion order, so the list
/// is identical on every machine.
fn slots(map: &Map) -> Vec<Slot> {
    const SPACING: f32 = 55.0;

    let mut out = Vec::new();
    for (id, road) in map.roads() {
        if road.lanes_iter().all(|(_, k)| k == LaneKind::Rail) {
            continue;
        }
        let pts = road.points();
        let len = pts.length();
        let n = (len / SPACING) as usize;
        for i in 1..n {
            let (p, dir) = pts.point_dir_along(i as f32 * SPACING);
            let side = dir.xy().perpendicular();
            out.push(Slot {
                road: id,
                road_width: road.width,
                proj: p.xy(),
                side,
            });
            out.push(Slot {
                road: id,
                road_width: road.width,
                proj: p.xy(),
                side: -side,
            });
        }
    }
    out
}

/// Footprint for `slot` sized `w` x `h`, snapped clear of its road exactly like
/// the in-game tool, or `None` if it would hit a road, intersection or an
/// already-claimed footprint. `taken` covers the current command batch, which
/// the spatial map has not seen yet.
fn footprint(map: &Map, slot: &Slot, w: f32, h: f32, taken: &[OBB]) -> Option<OBB> {
    let obb = OBB::new(
        slot.proj + slot.side * (h + slot.road_width + 0.5) * 0.5,
        slot.side,
        w,
        h,
    );

    if map
        .spatial_map()
        .query(
            obb,
            ProjectFilter::ROAD | ProjectFilter::INTER | ProjectFilter::BUILDING,
        )
        .any(|x| x != ProjectKind::Road(slot.road))
    {
        return None;
    }
    if taken.iter().any(|t| t.intersects(&obb)) {
        return None;
    }
    Some(obb)
}

/// One `MapBuildSpecialBuilding` per company prototype, each snapped to a road
/// and electrically connected to it so they share one network with the power
/// plant. Zoned prototypes get their footprint as their zone, like the toolbox.
fn company_commands(map: &Map, slots: &[Slot], taken: &mut Vec<OBB>) -> Vec<WorldCommand> {
    let mut cmds = Vec::with_capacity(COMPANIES.len());
    let mut next = 0;

    for name in COMPANIES {
        let proto: &GoodsCompanyPrototype = GoodsCompanyID::new(name).prototype();
        let (w, h) = (proto.base.size.w, proto.base.size.h);

        let mut placed = false;
        // Stride the slot list so companies spread over the whole grid instead
        // of crowding the first road.
        while next < slots.len() {
            let slot = &slots[next];
            next += 4;
            let Some(obb) = footprint(map, slot, w, h, taken) else {
                continue;
            };
            taken.push(obb);
            cmds.push(WorldCommand::MapBuildSpecialBuilding {
                pos: obb,
                kind: BuildingKind::GoodsCompany(proto.id),
                gen: proto.base.bgen,
                zone: proto
                    .zone
                    .as_ref()
                    .map(|_| Zone::new(Polygon::from(obb.corners.as_slice()), Vec2::X)),
                connected_road: Some(slot.road),
            });
            placed = true;
            break;
        }
        assert!(placed, "no room left on the grid for company {name}");
    }

    cmds
}

/// `N_HOUSES` lot-free houses in the gaps the companies left.
fn house_commands(map: &Map, slots: &[Slot], taken: &mut Vec<OBB>) -> Vec<WorldCommand> {
    let mut cmds = Vec::with_capacity(N_HOUSES);
    for slot in slots {
        if cmds.len() == N_HOUSES {
            break;
        }
        let Some(obb) = footprint(map, slot, 20.0, 20.0, taken) else {
            continue;
        };
        taken.push(obb);
        cmds.push(WorldCommand::MapBuildSpecialBuilding {
            pos: obb,
            kind: BuildingKind::House,
            gen: BuildingGen::House,
            zone: None,
            connected_road: Some(slot.road),
        });
    }
    assert_eq!(
        cmds.len(),
        N_HOUSES,
        "not enough room on the grid for houses"
    );
    cmds
}

fn n_kind(sim: &Simulation, f: impl Fn(&BuildingKind) -> bool) -> usize {
    sim.map()
        .buildings()
        .iter()
        .filter(|(_, b)| f(&b.kind))
        .count()
}

/// sov-rvu: the fixture's first 1200 m avenue clears every tree out of one
/// 256 m tree cell, which is what exposed this. `remove_trees_near` uses
/// `remove_maintain`, so the emptied cell stays allocated in the grid;
/// `SerializedEnvironment::from` used to write it as `(cell_id, [])` while
/// `From<SerializedEnvironment>` can only create a cell by inserting a tree
/// into it. The map therefore stopped surviving its own save/load round trip
/// and `test_world_survives_serde` went red at the tick the road was laid.
#[test]
fn sov_rvu_environment_roundtrip_drops_emptied_tree_cells() {
    let mut env = Environment::default();
    // Two trees, in two different 256 m tree-grid cells.
    for p in [vec2(10.0, 10.0), vec2(300.0, 10.0)] {
        env.trees.insert(p, Tree::new(p));
    }
    env.remove_trees_near(AABB::new_ll_ur(vec2(0.0, 0.0), vec2(100.0, 100.0)), |_| {});

    let cells = env.trees.storage().cells.clone();
    assert_eq!(cells.len(), 2, "both cells must still be allocated");
    assert_eq!(
        cells.values().filter(|c| c.objs.is_empty()).count(),
        1,
        "exactly one cell must be emptied but still allocated"
    );

    let bytes = common::saveload::Bincode::encode(&env).unwrap();
    let reloaded: Environment = common::saveload::Bincode::decode(&bytes).unwrap();
    assert_eq!(
        bytes,
        common::saveload::Bincode::encode(&reloaded).unwrap(),
        "an emptied tree cell must not be serialized: it cannot be reconstructed"
    );
}

#[test]
#[ignore = "rewrites the committed determinism baseline; run deliberately"]
fn regenerate_fixture_replay() {
    MyLog::init();
    INIT.call_once(crate::init::init);

    let t0 = std::time::Instant::now();
    let mut sched = Simulation::schedule();
    let mut sim = Simulation::new_with_options(SimulationOptions {
        terrain_size: 50,
        save_replay: true,
        ..Default::default()
    });
    assert!(
        sim.read::<Replay>().enabled,
        "save_replay must be on or nothing gets recorded"
    );
    // 1. the vehicle road grid.
    lay_grid(&mut sim, &mut sched);
    sim.tick(&mut sched, &[]);
    // 1b. the spur to the START_COMMANDS freight station: without it the
    // border stays closed (sov-ie6) and the fixture city never trades.
    lay_station_road(&mut sim, &mut sched);
    sim.tick(&mut sched, &[]);
    let n_roads = sim.map().roads().len();
    println!("grid laid: {} roads total", n_roads);

    // 2. companies, then 3. houses -- separate batches so the second sees the
    // first in the spatial map.
    let (companies, houses) = {
        let map = sim.map();
        let slots = slots(&map);
        println!("{} candidate slots along the grid", slots.len());
        let mut taken = Vec::new();
        let companies = company_commands(&map, &slots, &mut taken);
        let houses = house_commands(&map, &slots, &mut taken);
        (companies, houses)
    };
    sim.tick(&mut sched, &companies);
    sim.tick(&mut sched, &[]);
    sim.tick(&mut sched, &houses);
    sim.tick(&mut sched, &[]);

    println!(
        "built {} companies, {} houses at tick {}",
        n_kind(&sim, |k| matches!(k, BuildingKind::GoodsCompany(_))),
        n_kind(&sim, |k| matches!(k, BuildingKind::House)),
        sim.get_tick()
    );

    // 4. the tail: let the city actually live, and fill the EcoStats history.
    let t_tail = std::time::Instant::now();
    let mut next_log = 5_000;
    while sim.get_tick() < TAIL_TICK {
        sim.tick(&mut sched, &[]);
        if sim.get_tick() >= next_log {
            println!(
                "tick {} ({:.0}s elapsed, {} humans, {} vehicles)",
                sim.get_tick(),
                t_tail.elapsed().as_secs_f32(),
                sim.world().humans.len(),
                sim.world().vehicles.len()
            );
            next_log += 5_000;
        }
    }

    let humans = sim.world().humans.len();
    let vehicles = sim.world().vehicles.len();
    let companies = sim.world().companies.len();
    let stations = sim.world().freight_stations.len();
    let non_rail_roads = sim
        .map()
        .roads()
        .iter()
        .filter(|(_, r)| r.lanes_iter().any(|(_, k)| k != LaneKind::Rail))
        .count();

    println!(
        "CENSUS at tick {}: {} humans, {} vehicles, {} companies, {} freight stations, \
         {} non-rail roads of {} roads, money {}",
        sim.get_tick(),
        humans,
        vehicles,
        companies,
        stations,
        non_rail_roads,
        sim.map().roads().len(),
        sim.read::<crate::economy::Government>().money,
    );

    assert!(humans > 0, "hollow fixture: no humans");
    assert!(vehicles > 0, "hollow fixture: no vehicles");
    assert!(companies >= 10, "hollow fixture: {companies} companies");
    assert!(non_rail_roads > 0, "no vehicle road: trucks unexercised");

    let mut replay = sim.read::<Replay>().clone();
    replay.last_tick_recorded = replay
        .last_tick_recorded
        .max(prototypes::Tick(sim.get_tick()));

    let bytes = common::saveload::JSONPretty::encode(&replay).unwrap();
    let json = std::str::from_utf8(&bytes).unwrap();
    assert!(
        !json.contains("MapBuildHouse"),
        "ADR-0002: the fixture replay must not depend on auto-generated lots"
    );

    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests/world_replay.json");
    println!(
        "wrote {} ({} bytes, last_tick_recorded {}) in {:.0}s",
        out.display(),
        bytes.len(),
        replay.last_tick_recorded.0,
        t0.elapsed().as_secs_f32(),
    );
    std::fs::write(&out, bytes).unwrap();
}
