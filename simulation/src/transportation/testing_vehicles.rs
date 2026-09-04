use crate::map::{Map, PathKind};
use crate::map_dynamic::{Dispatcher, DispatchID, Itinerary};
use crate::transportation::{VehicleKind, VehicleState};
use crate::utils::resources::Resources;
use crate::{VehicleID, World};
use common::scroll::BTreeSetScroller;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Default, Serialize, Deserialize)]
pub struct RandomVehicles {
    pub vehicles: BTreeSet<VehicleID>,
    pub vehicle_scroller: BTreeSetScroller<VehicleID>,
}

pub fn random_vehicles_update(world: &mut World, res: &mut Resources) {
    profiling::scope!("transportation::random_vehicles_update");

    let rv = &mut *res.write::<RandomVehicles>();
    let map = res.read::<Map>();

    let mut to_kill = Vec::new();

    let tick = res.tick();

    for &v_id in rv.vehicle_scroller.iter_looped(&rv.vehicles).take(100) {
        let v = match world.vehicles.get_mut(v_id) {
            Some(x) => x,
            None => {
                to_kill.push(v_id);
                continue;
            }
        };

        if !(v.it.has_ended(0.0) || v.it.is_wait_for_reroute().is_some()) {
            continue;
        }
        let rng = common::hash_u64((tick.0, v_id));

        if let Some(it) = Itinerary::random_route(rng, v.trans.pos, tick, &map, PathKind::Vehicle) {
            v.it = it;
        }
    }

    // sov-aam: recover abandoned dispatch trucks. `advance_dispatches`
    // (economy/market.rs) parks every truck it is done with, but both park
    // sites (`Unloading`, `Returning`) silently skip the truck when
    // `reserve_near` finds no free spot: the dispatch is removed and the
    // dispatcher freed, leaving a `Driving` truck with an ended itinerary in
    // a live lane. Nothing ever assigns it a new route -- dispatch trucks are
    // not random-vehicle members -- so `calc_decision` keeps returning
    // `(0, dir)` (`get_point()` is `None`) and it sits forever, and every
    // dispatch truck queued behind it in the corridor freezes with it while
    // rail-served border exports keep flowing (the "matches decay to
    // exports-only" symptom). The goods are already settled before either
    // park site (buyer credited on the `ToDestination` arrival, seller
    // re-credited on the `Returning` arrival), so the abandoned truck carries
    // nothing and cruising it is pure recovery, never a teleport: adopt it
    // into the random-vehicle set and the loop above hands it a route.
    // Trucks owned by a live dispatch stay `reserved_by` and are untouched,
    // as are `Parked`/`RoadToPark` trucks and non-trucks.
    {
        let disp = res.read::<Dispatcher>();
        for (id, ve) in world.vehicles.iter() {
            if !matches!(ve.vehicle.kind, VehicleKind::Truck) {
                continue;
            }
            if !matches!(
                ve.vehicle.state,
                VehicleState::Driving | VehicleState::Panicking(_)
            ) {
                continue;
            }
            if !ve.it.has_ended(0.0) {
                continue;
            }
            if disp.is_reserved(DispatchID::SmallTruck(id)) {
                continue;
            }
            rv.vehicles.insert(id);
        }
    }

    for v in to_kill {
        rv.vehicles.remove(&v);
    }
}
