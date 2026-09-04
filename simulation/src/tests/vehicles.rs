use super::*;
use crate::transportation::{
    spawn_parked_vehicle, unpark, TransportGrid, VehicleKind, VehicleState,
};

/// sov-6qx: `unpark` used to warn "wasn't parked" and then carry on anyway --
/// inserting a SECOND transport-grid entry and overwriting `v.collider` with
/// its handle. The first entry was never removed (only `Transporter::destroy`
/// does that), leaving a phantom obstacle with speed 0 parked in a lane
/// forever. `unpark` on a vehicle that is already `Driving` must leave the
/// grid exactly as it found it.
#[test]
fn unpark_on_driving_vehicle_leaks_no_phantom_collider() {
    let mut ctx = TestCtx::new();
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 0.0)]);

    let car = spawn_parked_vehicle(&mut ctx.g, VehicleKind::Car, Vec3::ZERO)
        .expect("a parked car must spawn");
    assert!(
        unpark(&mut ctx.g, car),
        "setup: unparking a genuinely Parked car must succeed"
    );

    let v = ctx.g.world().vehicles.get(car).expect("car entity");
    assert!(
        matches!(v.vehicle.state, VehicleState::Driving),
        "setup: the first unpark must leave the car Driving"
    );
    let coll_before = v.collider.expect("a driving car holds a Transporter").0;
    let len_before = ctx.g.read::<TransportGrid>().len();

    // The bug: a second unpark, on a car that is emphatically not Parked.
    assert!(
        !unpark(&mut ctx.g, car),
        "unpark must refuse a non-Parked vehicle and say so to its caller"
    );

    let coll_after = ctx
        .g
        .world()
        .vehicles
        .get(car)
        .expect("car entity")
        .collider
        .expect("collider")
        .0;
    assert_eq!(
        len_before,
        ctx.g.read::<TransportGrid>().len(),
        "unpark on a non-Parked vehicle must not add a transport-grid entry"
    );
    assert_eq!(
        coll_before, coll_after,
        "the vehicle must keep its existing collider, not orphan it"
    );
    assert!(
        ctx.g.read::<TransportGrid>().get(coll_before).is_some(),
        "the original grid entry must still be the live one"
    );
}

/// sov-8p6: a refused Unpark must degrade, never strand the rider.
///
/// If a dispatch leaves the rider's car mid-parking (`RoadToPark`) when the
/// router's Unpark step runs, `unpark` refuses (sov-6qx: forcing it would
/// orphan a grid entry as a permanent phantom blocker). The old router logged
/// and proceeded to DriveTo, whose `has_ended` a parking-then-parked car never
/// reaches -- the human sat in `Location::Vehicle` forever.
///
/// The fix re-queues the Unpark behind the plan: parking finishes in
/// `TIME_TO_PARK` and the retry then succeeds, so the trip completes. This
/// test drives the real route (walk to car, board, refused unpark, retry,
/// drive, arrive). Against the pre-fix router it fails: the rider never
/// reaches the destination building.
#[test]
fn refused_unpark_mid_parking_retries_to_completion() {
    use crate::map_dynamic::router::park;
    use crate::map_dynamic::{Destination, ParkingManagement};
    use crate::souls::human::{spawn_human, HumanDecisionKind};
    use crate::transportation::Location;
    use crate::Map;

    let mut ctx = TestCtx::new();
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 0.0)]);

    let home = ctx.build_house_near(Vec2::new(0.0, 0.0));
    let dest = ctx.build_house_near(Vec2::new(90.0, 0.0));
    let human = spawn_human(&mut ctx.g, home).expect("human must spawn");
    let car = ctx
        .g
        .world()
        .humans
        .get(human)
        .expect("human")
        .router
        .personal_car
        .expect("spawned human owns a personal car");

    // Drive the human through its normal decision path: GoTo keeps calling
    // `go_to` until arrival, so no desire re-evaluation fights the trip.
    ctx.g
        .world_mut_unchecked()
        .humans
        .get_mut(human)
        .expect("human")
        .decision
        .kind = HumanDecisionKind::GoTo(Destination::Building(dest));

    // Tick until the human boards the car. Per-tick, not chunked: the Unpark
    // step pops on the first routing update after boarding, so the sabotage
    // below must land in between.
    let mut boarded = false;
    for _ in 0..2000 {
        ctx.tick();
        if ctx.g.world().humans.get(human).expect("human").location == Location::Vehicle(car) {
            boarded = true;
            break;
        }
    }
    assert!(boarded, "human never boarded the car");

    // A dispatch left the car mid-parking just as the Unpark step runs.
    {
        let (world, res) = ctx.g.world_res();
        let resa = {
            let map = res.read::<Map>();
            let pos = world.vehicles.get(car).expect("car").trans.pos;
            res.write::<ParkingManagement>()
                .reserve_near(pos, &map)
                .expect("a spot near the car")
        };
        {
            let map = res.read::<Map>();
            let v = world.vehicles.get_mut(car).expect("car");
            park(&map, v, resa);
        }
        assert!(
            matches!(
                world.vehicles.get(car).expect("car").vehicle.state,
                VehicleState::RoadToPark(..)
            ),
            "precondition: the car must be mid-parking when Unpark runs"
        );
    }

    // The trip must complete: the retry unparks once parking finishes, then
    // the human drives over and walks into the destination building.
    let mut arrived = false;
    for _ in 0..160 {
        ctx.advance_ticks(25);
        if ctx.g.world().humans.get(human).expect("human").location == Location::Building(dest) {
            arrived = true;
            break;
        }
    }
    assert!(
        arrived,
        "rider stalled after refused unpark: never reached destination"
    );
}

/*
use crate::map_dynamic::{Destination, Itinerary, ParkingManagement, Router};
use prototypes::GameTime;
use crate::vehicles::{spawn_parked_vehicle, unpark, VehicleKind};
use geom::{vec2, vec3, Vec3};
use crate::map::{Map, PathKind};

use super::*;
use crate::pedestrians::Location;
use crate::souls::desire::{BuyFood, Home};
use crate::souls::human::spawn_human;
use crate::ParCommandBuffer;

#[test]
fn test_car_simple() {
    let mut ctx = TestCtx::init();

    ctx.build_roads(&[vec2(0.0, 0.0), vec2(100.0, 0.0), vec2(100.0, 50.0)]);

    let g = &mut ctx.g;

    let car = spawn_parked_vehicle(g, VehicleKind::Car, Vec3::ZERO).unwrap();
    unpark(g, car);

    let pos = g.pos(car.0).unwrap();

    let spot_id = g
        .write::<ParkingManagement>()
        .reserve_near(vec3(100.0, 50.0, 0.0), &*g.map())
        .unwrap();
    let end_pos = spot_id.park_pos(&*g.map()).unwrap();

    let itin = Itinerary::route(pos, end_pos, &*g.read::<Map>(), PathKind::Vehicle).unwrap();
    *g.comp_mut::<Itinerary>(car.0).unwrap() = itin;

    for _ in 0..1000 {
        ctx.tick();
        if ctx
            .g
            .comp::<Itinerary>(car.0)
            .unwrap()
            .has_ended(ctx.g.read::<GameTime>().timestamp)
        {
            return;
        }
    }

    panic!("car has not arrived after 1000 ticks.")
}

#[test]
fn test_router_and_back() {
    let mut ctx = TestCtx::init();

    ctx.build_roads(&[vec2(0.0, 0.0), vec2(100.0, 0.0), vec2(100.0, 50.0)]);

    let b1 = ctx.build_house_near(vec2(0.0, 0.0));
    let human = spawn_human(&mut ctx.g, b1).unwrap();

    ctx.g
        .write::<ParCommandBuffer>()
        .remove_component::<Desire<Home>>(human.0);
    ctx.g
        .write::<ParCommandBuffer>()
        .remove_component::<Desire<BuyFood>>(human.0);

    let b2 = ctx.build_house_near(vec2(100.0, 5.0));

    for _ in 0..3 {
        ctx.g
            .comp_mut::<Router>(human.0)
            .unwrap()
            .go_to(Destination::Building(b2));

        for i in 0..1000 {
            ctx.tick();
            //log::info!("{}: {:?}", i, ctx.g.comp::<Location>(human.0).unwrap());
            if ctx.g.comp::<Location>(human.0).unwrap() == &Location::Building(b2) {
                break;
            }
            if i == 999 {
                panic!("ped has not arrived after 1000 ticks")
            }
        }

        ctx.g
            .comp_mut::<Router>(human.0)
            .unwrap()
            .go_to(Destination::Building(b1));

        for i in 0..2000 {
            ctx.tick();
            //log::info!("{}: {:?}", i, ctx.g.comp::<Location>(human.0).unwrap());
            if ctx.g.comp::<Location>(human.0).unwrap() == &Location::Building(b1) {
                break;
            }
            if i == 1999 {
                panic!("ped has not arrived after 1000 ticks")
            }
        }
    }
}

#[test]
fn test_router_and_back_change_middle() {
    let mut ctx = TestCtx::init();

    ctx.build_roads(&[vec2(0.0, 0.0), vec2(100.0, 0.0), vec2(100.0, 50.0)]);

    let b1 = ctx.build_house_near(vec2(0.0, 0.0));
    let human = spawn_human(&mut ctx.g, b1).unwrap();

    ctx.g
        .write::<ParCommandBuffer>()
        .remove_component::<Desire<Home>>(human.0);
    ctx.g
        .write::<ParCommandBuffer>()
        .remove_component::<Desire<BuyFood>>(human.0);

    let b2 = ctx.build_house_near(vec2(100.0, 5.0));

    for _ in 0..3 {
        ctx.g
            .comp_mut::<Router>(human.0)
            .unwrap()
            .go_to(Destination::Building(b2));
        for i in 0..1000 {
            ctx.tick();
            //log::info!("{}: {:?}", i, ctx.g.comp::<Location>(human.0).unwrap());

            if matches!(
                ctx.g.comp::<Location>(human.0).unwrap(),
                &Location::Vehicle(_)
            ) && i > 300
            {
                ctx.g
                    .comp_mut::<Router>(human.0)
                    .unwrap()
                    .go_to(Destination::Building(b1));
                break;
            }
            if i == 999 {
                panic!("not arrived")
            }
        }

        for i in 0..1000 {
            ctx.tick();
            //log::info!("{}: {:?}", i, ctx.g.comp::<Location>(human.0).unwrap());
            if ctx.g.comp::<Location>(human.0).unwrap() == &Location::Building(b1) {
                break;
            }
            if i == 999 {
                panic!("not arrived")
            }
        }
    }
}*/
