use super::*;
use crate::souls::human::spawn_human;
use crate::transportation::{
    transport_grid_equal, TransportGrid, TransportState, TransportationGroup,
};
use crate::Replay;
use common::saveload::{Bincode, JSON};

/// A simulation built exactly like `TestCtx::new`'s, but owned bare so the test
/// can choose which schedule ticks it.
fn fresh_sim() -> Simulation {
    MyLog::init();
    INIT.call_once(crate::init::init);

    Simulation::new_with_options(SimulationOptions {
        terrain_size: 1,
        save_replay: false,
        ..Default::default()
    })
}

/// Same idiom as `TestCtx::build_house_at`: an explicit lot-free house, also
/// registered in `BuildingInfos` so `add_souls_to_empty_buildings` can see it.
fn house_at(sim: &Simulation, p: Vec2) -> BuildingID {
    let obb = OBB::new(p, Vec2::X, 20.0, 20.0);
    let b = sim
        .map_mut()
        .build_special_building(&obb, BuildingKind::House, BuildingGen::House, None, None)
        .unwrap();
    sim.write::<BuildingInfos>().insert(b);
    b
}

fn tick_n(sim: &mut Simulation, sched: &mut SeqSchedule, n: u32) {
    for _ in 0..n {
        sim.tick(sched, WorldCommands::default().as_ref());
    }
}

/// sov-y66: `is_equal` only walked `saveload_funcs()` (resources), so any
/// divergence living in the ECS `World` -- entities, components -- was invisible
/// and the isomorphism gate stayed green through it.
#[test]
fn sov_y66_is_equal_sees_ecs_world_divergence() {
    let mut a = fresh_sim();
    let mut b = fresh_sim();
    assert!(
        a.is_equal(&b),
        "setup: two identically-built simulations must start out equal"
    );

    // Spawn the *same* human in both, so every resource stays byte-identical.
    let ha = house_at(&a, Vec2::new(50.0, 50.0));
    let hb = house_at(&b, Vec2::new(50.0, 50.0));
    let _ = spawn_human(&mut a, ha).expect("a human must spawn in a's house");
    let idb = spawn_human(&mut b, hb).expect("a human must spawn in b's house");
    assert!(
        a.is_equal(&b),
        "setup: spawning the same human in both simulations must keep them equal"
    );

    // Perturb b's ECS world only. No resource is touched by this.
    b.world_mut_unchecked()
        .humans
        .get_mut(idb)
        .expect("b's human entity")
        .trans
        .pos
        .x += 1.0;

    assert!(
        !a.is_equal(&b),
        "is_equal must compare the ECS world: a moved entity is a divergence"
    );
}

/// sov-n8v: the isomorphism gate ticked with an empty `SeqSchedule::default()`,
/// so no registered system ever ran and every world-only effect of the real
/// schedule went unchecked. `is_equal` must report such a world-only difference.
#[test]
fn sov_n8v_registered_systems_are_visible_to_is_equal() {
    let mut real = fresh_sim();
    let mut empty = fresh_sim();
    let mut probe = fresh_sim();
    assert!(
        !Simulation::schedule().times().is_empty(),
        "Simulation::schedule() must actually carry the registered systems"
    );
    house_at(&real, Vec2::new(50.0, 50.0));
    house_at(&empty, Vec2::new(50.0, 50.0));
    house_at(&probe, Vec2::new(50.0, 50.0));

    tick_n(&mut real, &mut Simulation::schedule(), 50);
    // `empty` and `probe` run the *empty* schedule: no system runs, so they only
    // advance GameTime and stay byte-identical to each other.
    tick_n(&mut empty, &mut SeqSchedule::default(), 50);
    tick_n(&mut probe, &mut SeqSchedule::default(), 50);

    assert!(
        !real.world().humans.is_empty(),
        "setup: the populated schedule must put souls in the world"
    );
    assert!(
        empty.world().humans.is_empty(),
        "setup: the empty schedule must leave the world untouched"
    );
    let real_world = Bincode::encode(real.world()).unwrap();
    let empty_world = Bincode::encode(empty.world()).unwrap();
    assert_ne!(
        real_world, empty_world,
        "setup: the two schedules must produce different worlds"
    );
    assert!(
        empty.is_equal(&probe),
        "setup: two empty-schedule runs must be equal"
    );

    // Graft the system-built world onto a simulation whose *resources* are still
    // bit-for-bit those of `probe`. The only remaining difference is the world.
    std::mem::swap(empty.world_mut_unchecked(), real.world_mut_unchecked());

    assert!(
        !empty.is_equal(&probe),
        "is_equal must catch a divergence that lives only in the system-built world"
    );
}

/// sov-n8v: `TransportGrid`'s cells live in an `FnvHashMap`, so a bincode
/// roundtrip permutes them and the raw bytes differ although the content is
/// identical -- a spurious red for the isomorphism gate. `transport_grid_equal`
/// must be order-insensitive without going blind to real changes.
#[test]
fn sov_n8v_transport_grid_equal_survives_bincode_roundtrip() {
    let mut g = TransportGrid::new(10);
    let mut handles = Vec::with_capacity(40);
    for i in 0..40 {
        handles.push(g.insert(
            Vec2::new(i as f32 * 7.3, i as f32 * 3.1),
            TransportState {
                radius: 1.0 + i as f32 * 0.1,
                speed: i as f32 * 0.25,
                group: if i % 2 == 0 {
                    TransportationGroup::Vehicles
                } else {
                    TransportationGroup::Pedestrians
                },
                flag: i as u64,
                ..Default::default()
            },
        ));
    }
    g.maintain();

    let bytes = Bincode::encode(&g).unwrap();
    let mut roundtripped: TransportGrid = Bincode::decode(&bytes).unwrap();

    assert!(
        transport_grid_equal(&g, &roundtripped),
        "a decoded transport grid holds the same objects, whatever the cell order"
    );

    // A genuine content change must still be caught.
    let (_, state) = roundtripped
        .get_mut(handles[7])
        .expect("handles survive a roundtrip");
    state.flag = 0xDEAD_BEEF;
    assert!(
        !transport_grid_equal(&g, &roundtripped),
        "a changed TransportState must not compare equal"
    );

    // ...and so must a missing object.
    let mut shortened: TransportGrid = Bincode::decode(&bytes).unwrap();
    let removed = shortened.remove_maintain(handles[3]);
    assert!(removed.is_some(), "setup: the object to remove must exist");
    assert!(
        !transport_grid_equal(&g, &shortened),
        "a removed object must not compare equal"
    );
}

/// sov-n8v: the schedule the gate now ticks with must be the one that actually
/// populates the world, otherwise the gate is green over an empty simulation.
#[test]
fn sov_n8v_replay_fixture_populates_world() {
    let mut sim = fresh_sim();
    house_at(&sim, Vec2::new(50.0, 50.0));

    tick_n(&mut sim, &mut Simulation::schedule(), 200);

    assert!(
        !sim.world().humans.is_empty(),
        "200 ticks of Simulation::schedule() must spawn souls into the built house"
    );
}

/// sov-rvu / ADR-0002 §4: the determinism gate proves the replay round-trips
/// identically, not that it builds anything. A hollow replay -- every
/// `MapBuildHouse` silently no-oping once auto-lots are gone, say -- round-trips
/// deterministically *while empty*, so the gate stays green over a dead city and
/// every UI ticket that needs live state is unprovable. This census is the loud
/// failure: the committed replay is also the UI fixture world, so it must end
/// with a populated city, and with at least one non-rail road so trucks, parking
/// and the road router are exercised at all (the pre-`sov-rvu` replay was rail
/// only).
#[test]
fn sov_rvu_fixture_world_census() {
    MyLog::init();
    INIT.call_once(crate::init::init);

    let replay: Replay = JSON::decode(Simulation::FIXTURE_REPLAY.as_bytes())
        .expect("Simulation::FIXTURE_REPLAY must decode as a Replay");
    let sim = Simulation::materialise_replay(replay);

    let (humans, vehicles, companies) = (
        sim.world().humans.len(),
        sim.world().vehicles.len(),
        sim.world().companies.len(),
    );

    assert!(
        humans >= 20,
        "fixture world must be populated: {humans} humans, expected >= 20"
    );
    assert!(
        vehicles >= 10,
        "fixture world must move: {vehicles} vehicles, expected >= 10"
    );
    assert!(
        companies >= 10,
        "fixture world must trade: {companies} companies, expected >= 10"
    );

    assert!(
        sim.map()
            .roads
            .values()
            .any(|r| r.lanes_iter().any(|(_, kind)| !kind.is_rail())),
        "fixture world must contain at least one non-rail road, otherwise trucks, \
         parking and the road router are never exercised"
    );
}
