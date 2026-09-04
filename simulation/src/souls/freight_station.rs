use serde::{Deserialize, Serialize};

use geom::Transform;
use prototypes::{FreightStationPrototypeID, GameTime};

use crate::map::{BuildingID, Map, PathKind};
use crate::map_dynamic::{
    BuildingInfos, DispatchID, DispatchKind, DispatchQueryTarget, Dispatcher, Itinerary,
};
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::utils::resources::Resources;
use crate::world::{FreightStationEnt, FreightStationID, TrainID};
use crate::World;
use crate::{ParCommandBuffer, Simulation, SoulID};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Inspect)]
pub enum FreightTrainState {
    /// The train is coming to the station
    Arriving,
    /// The train is waiting for the station to load goods
    Loading,
    /// The train is going to the destination
    Moving,
}

const MAX_TRAINS_PER_STATION: usize = 2;

/// Units of Border custody one train arrival unloads at the station.
/// Matches the `waiting_cargo`/`wanted_cargo` counter consumption below, so
/// today's counter event becomes a real restock of the same size.
pub const TRAIN_RESTOCK_QTY: u32 = 100;

/// Upper bound of the Border custody ledger. The ledger is an accountable
/// buffer for the outside world, not a full economy: it caps instead of
/// growing without limit, and stations start full so pre-ledger import paths
/// (which never saw a train) keep working until draws bring them down.
pub const MAX_BORDER_STOCK: u32 = 10_000;

/// A freight train station
/// A component that identifies freight station souls, managing freight station logic
/// and the freight trains that are associated with them.
#[derive(Serialize, Deserialize, Inspect)]
pub struct FreightStation {
    pub proto: FreightStationPrototypeID,
    pub building: BuildingID,
    pub trains: Vec<(TrainID, FreightTrainState)>,
    pub waiting_cargo: u32,
    pub wanted_cargo: u32,
    /// Border custody (ADR-0003 §3): bounded stock of external goods held at
    /// the border. Import dispatches draw from it when their truck loads
    /// (`Market::advance_dispatches`); train arrivals replenish it
    /// (`on_train_arrived`). Empty means going-without at the border.
    pub border_stock: u32,
}

impl FreightStation {
    /// Settles one train arrival: consumes the waiting/wanted counters (the
    /// pre-ledger restock event) and replenishes the Border custody ledger,
    /// saturating at `MAX_BORDER_STOCK`.
    pub fn on_train_arrived(&mut self) {
        self.waiting_cargo = self.waiting_cargo.saturating_sub(100);
        self.wanted_cargo = self.wanted_cargo.saturating_sub(100);
        self.border_stock = self
            .border_stock
            .saturating_add(TRAIN_RESTOCK_QTY)
            .min(MAX_BORDER_STOCK);
    }

    /// Draws `qty` units of Border custody for one import shipment. Returns
    /// false — drawing nothing — when the ledger holds less than `qty`.
    pub fn try_draw_border_stock(&mut self, qty: u32) -> bool {
        if self.border_stock < qty {
            return false;
        }
        self.border_stock -= qty;
        true
    }
}

pub fn freight_station_soul(
    sim: &mut Simulation,
    building: BuildingID,
    proto: FreightStationPrototypeID,
) -> Option<FreightStationID> {
    let map = sim.map();

    let n_trucks = proto.prototype().n_trucks;
    let b = map.buildings.get(building)?;

    let _height = b.height;
    let obb = b.obb;
    let door = b.door_pos;
    let _pos = obb.center();
    let _axis = obb.axis();

    drop(map);

    // Station-owned road fleet (sov-2uv, $VEHICLE_STATION precedent):
    // imports draw from the city-wide pool today, so every long border haul
    // borrows a factory truck and crowds out domestic short hops. Spawn the
    // base_mod-declared trucks parked at the station door; `dispatch_system`
    // picks them up like any other truck. Degrade, never refuse: without
    // nearby parking the station still opens for trains.
    for _ in 0..n_trucks {
        if spawn_parked_vehicle(sim, VehicleKind::Truck, door).is_none() {
            log::warn!(
                "freight station at {:?} could not park a declared truck; \
                 station opens train-only for this truck",
                door
            );
        }
    }

    let map = sim.map();
    let b = map.buildings.get(building)?;

    let height = b.height;
    let obb = b.obb;
    let pos = obb.center();
    let axis = obb.axis();

    drop(map);

    let f = FreightStation {
        proto,
        building,
        trains: Vec::with_capacity(MAX_TRAINS_PER_STATION),
        waiting_cargo: 0,
        wanted_cargo: 0,
        border_stock: MAX_BORDER_STOCK,
    };
    let id = sim.world.insert(FreightStationEnt {
        f,
        trans: Transform::new_dir(pos.z(height), axis[1].z(0.0).normalize()),
    });

    sim.write::<BuildingInfos>()
        .set_owner(building, SoulID::FreightStation(id));

    Some(id)
}

pub fn freight_station_system(world: &mut World, resources: &mut Resources) {
    profiling::scope!("souls::freight_station_system");
    let cbuf = resources.read::<ParCommandBuffer<FreightStationEnt>>();
    let mut dispatch = resources.write::<Dispatcher>();
    let map = resources.read::<Map>();
    let time = resources.read::<GameTime>();
    let tick = time.tick;

    for (me, f) in world.freight_stations.iter_mut() {
        let pos = f.trans;
        let station = &mut f.f;
        if !map.buildings.contains_key(station.building) {
            cbuf.kill(me);
            continue;
        }

        // update our trains, and remove the ones that are done
        let mut to_clean = vec![];
        let mut arrivals = 0u32;
        for (trainid, state) in &mut station.trains {
            let Some(train) = world.trains.get_mut(*trainid) else {
                to_clean.push(*trainid);
                continue;
            };
            let itin = &mut train.it;

            match state {
                FreightTrainState::Arriving => {
                    if itin.has_ended(0.0) {
                        *state = FreightTrainState::Loading;
                        arrivals += 1;
                        *itin = Itinerary::wait_until(time.timestamp + 10.0);
                    }
                }
                FreightTrainState::Loading => {
                    if itin.has_ended(time.timestamp) {
                        let Some(&ext) = map.external_train_stations.first() else {
                            // sov-33c: the external trading station was demolished
                            // while this train was Loading. Never game over: keep
                            // waiting for the border to come back instead of
                            // panicking on the now-empty vec (same degrade shape
                            // as the unroutable case below).
                            *itin = Itinerary::wait_until(time.timestamp + 10.0);
                            continue;
                        };
                        let bpos = map.buildings[ext].obb.center().z(0.0);

                        *itin = if let Some(r) =
                            Itinerary::route(tick, train.trans.pos, bpos, &map, PathKind::Rail)
                        {
                            r
                        } else {
                            Itinerary::wait_until(time.timestamp + 10.0);
                            continue;
                        };
                        *state = FreightTrainState::Moving;
                    }
                }
                FreightTrainState::Moving => {
                    if itin.has_ended(time.timestamp) {
                        to_clean.push(*trainid);
                    }
                }
            }
        }
        // sov-uo5: each arrival's counter consumption becomes a real Border
        // custody restock event, settled here (same tick) once the `trains`
        // borrow above has ended.
        for _ in 0..arrivals {
            station.on_train_arrived();
        }
        for v in to_clean {
            station.trains.retain(|x| x.0 != v);
            dispatch.free(v)
        }

        // If enough goods are waiting, query for a train to take them to the external trading station
        if station.trains.len() >= MAX_TRAINS_PER_STATION {
            continue;
        }
        if station.waiting_cargo + station.wanted_cargo < 10 {
            continue;
        }

        let destination = pos.pos + pos.dir * 75.0 - pos.dir.perp_up() * 40.0;

        let Some(DispatchID::FreightTrain(trainid)) = dispatch.query(
            &map,
            DispatchKind::FreightTrain,
            DispatchQueryTarget::Pos(destination),
        ) else {
            continue;
        };

        let train = world.trains.get_mut(trainid).unwrap();

        train.it = unwrap_or!(
            Itinerary::route(tick, train.trans.pos, destination, &map, PathKind::Rail,),
            continue
        );

        station.trains.push((trainid, FreightTrainState::Arriving));
    }
}

#[cfg(test)]
mod tests {
    use geom::{vec2, vec3, OBB};
    use prototypes::{BuildingGen, FreightStationPrototypeID};

    use crate::map_dynamic::BuildingInfos;
    use crate::souls::human::{spawn_human, HumanDecisionKind};
    use crate::tests::TestCtx;
    use crate::{BuildingKind, SoulID, WorldCommand};

    #[test]
    fn test_deliver_to_freight_station_incrs_station() {
        let mut test = TestCtx::new();

        test.build_roads(&[vec3(0., 0., 0.), vec3(100., 0., 0.)]);
        let house = test.build_house_near(vec2(50.0, 50.0));
        let human = spawn_human(&mut test.g, house).unwrap();

        test.apply(&[WorldCommand::MapBuildSpecialBuilding {
            pos: OBB::new(vec2(50.0, 50.0), vec2(1.0, 0.0), 5.0, 5.0),
            kind: BuildingKind::RailFreightStation(FreightStationPrototypeID::new(
                "freight-station",
            )),
            gen: BuildingGen::NoWalkway {
                door_pos: vec2(50.0, 50.0),
            },
            zone: None,
            connected_road: None,
        }]);
        test.tick();

        let station = test
            .g
            .map()
            .buildings()
            .iter()
            .find(|(_, b)| matches!(b.kind, BuildingKind::RailFreightStation(_)))
            .unwrap()
            .0;

        test.g
            .world_mut_unchecked()
            .humans
            .get_mut(human)
            .unwrap()
            .decision
            .kind = HumanDecisionKind::DeliverAtBuilding(station);

        let binfos = test.g.read::<BuildingInfos>();
        let SoulID::FreightStation(stationsoul) = binfos.owner(station).unwrap() else {
            panic!()
        };
        drop(binfos);

        for _ in 0..100 {
            test.tick();

            if test.g.get(stationsoul).unwrap().f.waiting_cargo == 1 {
                return;
            }
        }

        panic!("should have delivered to freight station")
    }

    /// sov-33c: demolishing the external trading station while a train is
    /// Loading must degrade to waiting (never-game-over), never panic on the
    /// now-empty `external_train_stations` vec. The START train is unattached
    /// (no cargo demand yet), so it is claimed here as a Loading train with
    /// an already-expired wait: the very next system pass takes the Loading
    /// branch with the border gone. Pre-fix this tick panics at
    /// `first().unwrap()`.
    #[test]
    fn sov_33c_demolished_external_station_loading_train_waits() {
        let mut test = TestCtx::new();
        test.tick(); // souls (freight station, train) spawn on the first tick

        let station = test
            .g
            .world()
            .freight_stations
            .iter()
            .next()
            .map(|(id, _)| id)
            .expect("START_COMMANDS must seed a freight station soul");
        // Spawn a real train on a short rail spur: the Loading branch needs a
        // live `world.trains` entry, and no system spawns freight trains on
        // its own (`AddTrain` is a no-op in `WorldCommand::apply`).
        let lane = {
            let mut map = test.g.map_mut();
            let (_, road) = map
                .make_connection(
                    crate::map::MapProject::ground(vec3(0.0, 0.0, 0.0)),
                    crate::map::MapProject::ground(vec3(100.0, 0.0, 0.0)),
                    None,
                    &crate::map::LanePatternBuilder::new().rail(true).build(),
                )
                .expect("rail spur must build on test terrain");
            let lanes: Vec<_> = map.roads[road]
                .lanes_iter()
                .map(|(id, _)| id)
                .collect();
            lanes
                .into_iter()
                .next()
                .expect("rail spur must have a lane")
        };
        test.apply(&[WorldCommand::SpawnTrain {
            wagons: vec![
                prototypes::RollingStockID::new("locomotive"),
                prototypes::RollingStockID::new("freight-wagon"),
            ],
            lane,
            dist: 10.0,
        }]);
        let train = test
            .g
            .world()
            .trains
            .iter()
            .next()
            .map(|(id, _)| id)
            .expect("SpawnTrain must insert a train");

        // Expired wait: the Loading branch below sees it as ended.
        let now = test.g.read::<prototypes::GameTime>().timestamp;
        test.g.world_mut_unchecked().trains.get_mut(train).unwrap().it =
            crate::Itinerary::wait_until(now - 1.0);
        test.g
            .world_mut_unchecked()
            .freight_stations
            .get_mut(station)
            .unwrap()
            .f
            .trains
            .push((train, super::FreightTrainState::Loading));

        // Demolish the border out from under the Loading train.
        let ext = test
            .g
            .map()
            .buildings()
            .iter()
            .find(|(_, b)| b.kind == BuildingKind::ExternalTrading)
            .map(|(id, _)| id)
            .expect("terrain gen must seed an external trading station");
        test.g.map_mut().remove_building(ext);
        assert!(
            test.g.map().external_train_stations.is_empty(),
            "the demolish must leave no external train station behind"
        );

        test.tick();

        let st = test.g.world().freight_stations.get(station).unwrap();
        assert!(
            st.f.trains
                .iter()
                .any(|(id, s)| *id == train && matches!(s, super::FreightTrainState::Loading)),
            "a Loading train whose border station was demolished must degrade to \
             waiting (still Loading), not panic or vanish"
        );
    }
}
