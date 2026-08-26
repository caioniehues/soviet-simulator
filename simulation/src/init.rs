use serde::de::DeserializeOwned;
use serde::Serialize;

#[allow(unused_imports)]
use common::saveload::{Bincode, Encoder, JSONPretty, JSON};
use prototypes::{GameTime, Tick};

use crate::economy::{market_update, EcoStats, Government, Market};
use crate::map::Map;
use crate::map_dynamic::{
    dispatch_system, electricity_flow_system, itinerary_update, routing_changed_system,
    routing_update_system, BuildingInfos, Dispatcher, ElectricityFlow, ParkingManagement,
};
use crate::multiplayer::MultiplayerState;
use crate::souls::freight_station::freight_station_system;
use crate::souls::goods_company::company_system;
use crate::souls::human::update_decision_system;
use crate::transportation::pedestrian_decision_system;
use crate::transportation::road::{vehicle_decision_system, vehicle_state_update_system};
use crate::transportation::testing_vehicles::{random_vehicles_update, RandomVehicles};
use crate::transportation::train::{
    locomotive_system, train_reservations_update, TrainReservations,
};
use crate::transportation::{transport_grid_synchronize, TransportGrid};
use crate::utils::resources::Resources;
use crate::world::{CompanyEnt, FreightStationEnt, HumanEnt, TrainEnt, VehicleEnt, WagonEnt};
use crate::World;
use crate::{
    add_souls_to_empty_buildings, utils, ParCommandBuffer, RandProvider, Replay, RunnableSystem,
    Simulation, SimulationOptions, RNG_SEED,
};

pub fn init() {
    //crate::rerun::init_rerun();

    // # Safety
    // This function is called only once, before any other function in this crate.
    unsafe {
        #[cfg(not(test))]
        let base = "./";
        #[cfg(test)]
        let base = "../";

        match prototypes::load_prototypes(base) {
            Ok(_) => {}
            Err(e) => {
                panic!("Error loading prototypes: {}", e)
            }
        }
    }

    let mut registry = Registry::default();

    register_system(
        &mut registry,
        "electricity_flow_system",
        electricity_flow_system,
    );
    register_system(&mut registry, "dispatch_system", dispatch_system);
    register_system(
        &mut registry,
        "update_decision_system",
        update_decision_system,
    );
    register_system(&mut registry, "company_system", company_system);
    register_system(
        &mut registry,
        "pedestrian_decision_system",
        pedestrian_decision_system,
    );
    register_system(
        &mut registry,
        "transport_grid_synchronize",
        transport_grid_synchronize,
    );
    register_system(&mut registry, "locomotive_system", locomotive_system);
    register_system(
        &mut registry,
        "vehicle_decision_system",
        vehicle_decision_system,
    );
    register_system(
        &mut registry,
        "vehicle_state_update_system",
        vehicle_state_update_system,
    );
    register_system(
        &mut registry,
        "routing_changed_system",
        routing_changed_system,
    );
    register_system(
        &mut registry,
        "routing_update_system",
        routing_update_system,
    );
    register_system(&mut registry, "itinerary_update", itinerary_update);
    register_system(&mut registry, "market_update", market_update);
    register_system(
        &mut registry,
        "train_reservations_update",
        train_reservations_update,
    );
    register_system(&mut registry, "freight_station", freight_station_system);
    register_system(&mut registry, "random_vehicles", random_vehicles_update);
    register_system(&mut registry, "update_map", |_, res| {
        res.write::<Map>().update()
    });

    register_system_sim(
        &mut registry,
        "add_souls_to_empty_buildings",
        add_souls_to_empty_buildings,
    );

    register_resource_noserialize::<ParCommandBuffer<VehicleEnt>>(&mut registry);
    register_resource_noserialize::<ParCommandBuffer<TrainEnt>>(&mut registry);
    register_resource_noserialize::<ParCommandBuffer<HumanEnt>>(&mut registry);
    register_resource_noserialize::<ParCommandBuffer<WagonEnt>>(&mut registry);
    register_resource_noserialize::<ParCommandBuffer<FreightStationEnt>>(&mut registry);
    register_resource_noserialize::<ParCommandBuffer<CompanyEnt>>(&mut registry);
    register_resource_noinit::<SimulationOptions, Bincode>(&mut registry, "simoptions");

    register_resource_default::<ElectricityFlow, Bincode>(&mut registry, "electricity_flow");
    register_resource_default::<Market, Bincode>(&mut registry, "market");
    register_resource_default::<EcoStats, Bincode>(&mut registry, "ecostats");
    register_resource_default::<MultiplayerState, Bincode>(&mut registry, "multiplayer_state");
    register_resource_default::<RandomVehicles, Bincode>(&mut registry, "random_vehicles");
    register_resource_default::<Map, Bincode>(&mut registry, "map");
    register_resource_default::<TrainReservations, Bincode>(&mut registry, "train_reservations");
    register_resource_default::<Government, Bincode>(&mut registry, "government");
    register_resource_default::<ParkingManagement, Bincode>(&mut registry, "pmanagement");
    register_resource_default::<BuildingInfos, Bincode>(&mut registry, "binfos");
    register_resource::<GameTime, Bincode>(&mut registry, "game_time", || GameTime::new(Tick(1)));
    register_resource::<TransportGrid, Bincode>(&mut registry, "transport_grid", || {
        TransportGrid::new(100)
    });
    register_resource::<RandProvider, Bincode>(&mut registry, "randprovider", || {
        RandProvider::new(RNG_SEED)
    });
    register_resource_default::<Dispatcher, Bincode>(&mut registry, "dispatcher");
    register_resource_default::<Replay, JSON>(&mut registry, "replay");

    let _ = REGISTRY.set(registry);
}

pub struct InitFunc {
    pub f: Box<dyn Fn(&mut Simulation) + Send + Sync + 'static>,
}

pub(crate) struct SaveLoadFunc {
    pub name: &'static str,
    pub save: Box<dyn Fn(&Simulation) -> Vec<u8> + Send + Sync + 'static>,
    pub load: Box<dyn Fn(&mut Simulation, Vec<u8>) + Send + Sync + 'static>,
}

pub(crate) struct GSystem {
    pub(crate) s: Box<dyn Fn() -> Box<dyn RunnableSystem> + Send + Sync>,
}

#[derive(Default)]
struct Registry {
    init_funcs: Vec<InitFunc>,
    saveload_funcs: Vec<SaveLoadFunc>,
    gsystems: Vec<GSystem>,
}

static REGISTRY: std::sync::OnceLock<Registry> = std::sync::OnceLock::new();

pub(crate) fn init_funcs() -> &'static [InitFunc] {
    &REGISTRY.get().expect("init() not called").init_funcs
}

pub(crate) fn saveload_funcs() -> &'static [SaveLoadFunc] {
    &REGISTRY.get().expect("init() not called").saveload_funcs
}

pub(crate) fn gsystems() -> &'static [GSystem] {
    &REGISTRY.get().expect("init() not called").gsystems
}

fn register_system(registry: &mut Registry, name: &'static str, s: fn(&mut World, &mut Resources)) {
    registry.gsystems.push(GSystem {
        s: Box::new(move || {
            Box::new(utils::scheduler::RunnableFn {
                f: move |sim| s(&mut sim.world, &mut sim.resources),
                name,
            })
        }),
    });
}

fn register_system_sim(registry: &mut Registry, name: &'static str, s: fn(&mut Simulation)) {
    registry.gsystems.push(GSystem {
        s: Box::new(move || Box::new(utils::scheduler::RunnableFn { f: s, name })),
    });
}

fn register_resource_noserialize<T: 'static + Default + Send + Sync>(registry: &mut Registry) {
    registry.init_funcs.push(InitFunc {
        f: Box::new(|uiw| uiw.insert(T::default())),
    });
}

fn register_resource_default<
    T: 'static + Send + Sync + Serialize + DeserializeOwned + Default,
    E: Encoder,
>(
    registry: &mut Registry,
    name: &'static str,
) {
    register_resource::<T, E>(registry, name, T::default);
}

fn register_resource<T: 'static + Send + Sync + Serialize + DeserializeOwned, E: Encoder>(
    registry: &mut Registry,
    name: &'static str,
    initializer: impl Fn() -> T + Send + Sync + 'static,
) {
    registry.init_funcs.push(InitFunc {
        f: Box::new(move |uiw| uiw.insert(initializer())),
    });
    register_resource_noinit::<T, E>(registry, name);
}

fn register_resource_noinit<T: 'static + Send + Sync + Serialize + DeserializeOwned, E: Encoder>(
    registry: &mut Registry,
    name: &'static str,
) {
    registry.saveload_funcs.push(SaveLoadFunc {
        name,
        save: Box::new(move |uiworld| E::encode(&*uiworld.read::<T>()).unwrap()),
        load: Box::new(move |uiworld, data| match E::decode::<T>(&data) {
            Ok(res) => {
                uiworld.insert(res);
            }
            Err(e) => {
                log::error!("Error loading resource {}: {}", name, e);
            }
        }),
    });
}
