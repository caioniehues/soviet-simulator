#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![warn(clippy::iter_over_hash_type)]

use crate::init::{gsystems, init_funcs, saveload_funcs};
use crate::map::{BuildingKind, Map};
use crate::map_dynamic::{Itinerary, ItineraryLeader};
use crate::souls::add_souls_to_empty_buildings;
use crate::transportation::{transport_grid_equal, TransportGrid};
use crate::utils::resources::{Ref, RefMut, Resources};
use crate::utils::scheduler::RunnableSystem;
use crate::world_command::WorldCommand;
use crate::world_command::WorldCommand::Init;
use common::saveload::Encoder;
use common::FastMap;
use derive_more::{From, TryInto};
use geom::Vec3;
use prototypes::{prototype, ColorsPrototype, ColorsPrototypeID, GameTime, Tick};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::time::{Duration, Instant};
use utils::rand_provider::RandProvider;
use utils::scheduler::SeqSchedule;

#[macro_use]
extern crate common;

#[allow(unused_imports)]
#[macro_use]
extern crate inline_tweak;

#[macro_use]
extern crate egui_inspect;

#[macro_use]
extern crate log as extern_log;

pub mod economy;
pub mod init;
pub mod map;
pub mod map_dynamic;
pub mod multiplayer;
mod rerun;
pub mod souls;
#[cfg(test)]
mod tests;
pub mod transportation;
pub mod utils;
mod world;
pub mod world_command;

pub use world::*;

pub use utils::par_command_buffer::ParCommandBuffer;
pub use utils::replay::*;

pub fn colors() -> &'static ColorsPrototype {
    prototype::<ColorsPrototypeID>(ColorsPrototypeID::new("colors"))
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, From, TryInto,
)]
pub enum SoulID {
    Human(HumanID),
    GoodsCompany(CompanyID),
    FreightStation(FreightStationID),
}

impl Display for SoulID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SoulID::Human(id) => write!(f, "{:?}", id),
            SoulID::GoodsCompany(id) => write!(f, "{:?}", id),
            SoulID::FreightStation(id) => write!(f, "{:?}", id),
        }
    }
}

impl From<SoulID> for AnyEntity {
    fn from(value: SoulID) -> Self {
        match value {
            SoulID::Human(id) => AnyEntity::HumanID(id),
            SoulID::GoodsCompany(id) => AnyEntity::CompanyID(id),
            SoulID::FreightStation(id) => AnyEntity::FreightStationID(id),
        }
    }
}

impl TryFrom<AnyEntity> for SoulID {
    type Error = ();

    fn try_from(value: AnyEntity) -> Result<Self, Self::Error> {
        match value {
            AnyEntity::HumanID(id) => Ok(SoulID::Human(id)),
            AnyEntity::CompanyID(id) => Ok(SoulID::GoodsCompany(id)),
            AnyEntity::FreightStationID(id) => Ok(SoulID::FreightStation(id)),
            _ => Err(()),
        }
    }
}

debug_inspect_impl!(SoulID);

pub struct Simulation {
    pub(crate) world: World,
    resources: Resources,
}

const RNG_SEED: u64 = 123;
const VERSION: &str = include_str!("../../VERSION");

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SimulationOptions {
    pub terrain_size: u16,
    pub save_replay: bool,
    #[serde(default = "default_seed")]
    pub seed: u64,
}

fn default_seed() -> u64 {
    RNG_SEED
}

impl Default for SimulationOptions {
    fn default() -> Self {
        SimulationOptions {
            terrain_size: 50,
            save_replay: true,
            seed: RNG_SEED,
        }
    }
}

/// Writes an `is_equal` mismatch dump into the gitignored `simulation/world/`
/// directory (the `world` basename rule in `.gitignore` covers it), so a
/// deliberate red run leaves `git status --porcelain` clean of new files.
fn dump_debug_json(file_name: &str, bytes: &[u8]) {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("world");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(file_name), &*String::from_utf8_lossy(bytes)).unwrap();
}

impl Simulation {
    pub fn schedule() -> SeqSchedule {
        let mut schedule = SeqSchedule::default();
        for s in gsystems() {
            let s = (s.s)();
            schedule.add_system(s);
        }
        schedule
    }

    pub fn new(gen_terrain: bool) -> Simulation {
        Self::new_with_options(SimulationOptions {
            terrain_size: if gen_terrain { 50 } else { 0 },
            ..Default::default()
        })
    }

    pub fn from_replay(replay: Replay) -> (Simulation, SimulationReplayLoader) {
        let mut sim = Simulation {
            world: Default::default(),
            resources: Default::default(),
        };

        info!("Seed is {}", RNG_SEED);

        for s in init_funcs() {
            (s.f)(&mut sim);
        }

        (
            sim,
            SimulationReplayLoader {
                replay,
                pastt: Tick::default(),
                idx: 0,
                speed: 1,
                advance_n_ticks: 0,
            },
        )
    }

    /// The committed replay that defines the fixture world (ADR-0002).
    ///
    /// It is the only replay in the repository: the determinism gate
    /// (`tests::test_iso::test_world_survives_serde`, plus the census guard
    /// `tests::determinism_gate::sov_rvu_fixture_world_census`) and the
    /// `native_app` startup fallback both materialise this exact byte string.
    pub const FIXTURE_REPLAY: &'static str = include_str!("tests/world_replay.json");

    /// Materialises a replay through the real schedule up to its last recorded
    /// tick, returning the populated simulation (ADR-0002).
    ///
    /// This is the only sanctioned way to derive a world from a replay: it uses
    /// `Simulation::schedule()`, never `SeqSchedule::default()`, so souls,
    /// economy and transportation actually run.
    pub fn materialise_replay(replay: Replay) -> Simulation {
        let (mut sim, mut loader) = Self::from_replay(replay);
        // The loader defaults to one tick per `advance_tick` call; a large batch
        // keeps ~35k ticks to a few hundred calls instead of 35k.
        loader.speed = 512;
        let mut schedule = Self::schedule();
        while !loader.advance_tick(&mut sim, &mut schedule) {}
        sim
    }

    pub fn new_with_options(opts: SimulationOptions) -> Simulation {
        let mut sim = Simulation {
            world: Default::default(),
            resources: Default::default(),
        };

        info!("Seed is {}", opts.seed);
        info!("{:?}", opts);

        for s in init_funcs() {
            (s.f)(&mut sim);
        }

        Init(Box::new(opts)).apply(&mut sim);

        let start_commands: Vec<(u32, WorldCommand)> =
            common::saveload::JSON::decode(START_COMMANDS.as_bytes()).unwrap();

        for (_, command) in start_commands {
            command.apply(&mut sim);
        }

        sim
    }

    pub fn world_res(&mut self) -> (&mut World, &mut Resources) {
        (&mut self.world, &mut self.resources)
    }

    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn world_mut_unchecked(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        if self.resources.iter().count() != other.resources.iter().count() {
            return false;
        }

        for l in saveload_funcs() {
            let a = (l.save)(self);
            let b = (l.save)(other);

            if a != b {
                // The transport grid keeps its cells in a hashmap, so a decode permutes their
                // iteration order and the bytes differ even when the contents are identical.
                if l.name == "transport_grid"
                    && transport_grid_equal(
                        &self.read::<TransportGrid>(),
                        &other.read::<TransportGrid>(),
                    )
                {
                    continue;
                }
                dump_debug_json(&format!("{}_a.json", l.name), &a);
                dump_debug_json(&format!("{}_b.json", l.name), &b);
                return false;
            }
        }

        let world_a = common::saveload::Bincode::encode(&self.world).unwrap();
        let world_b = common::saveload::Bincode::encode(&other.world).unwrap();
        if world_a != world_b {
            dump_debug_json("world_a.json", &world_a);
            dump_debug_json("world_b.json", &world_b);
            return false;
        }

        true
    }

    pub fn tick<'a>(
        &mut self,
        game_schedule: &mut SeqSchedule,
        commands: impl IntoIterator<Item = &'a WorldCommand>,
    ) -> Duration {
        profiling::scope!("simulation::tick");
        let t = Instant::now();
        // It is very important that the first thing being done is applying commands
        // so that instant commands work on single player but the game is still deterministic
        {
            profiling::scope!("applying commands");
            for command in commands {
                command.apply(self);
            }
        }

        {
            let mut time = self.write::<GameTime>();
            *time = GameTime::new(Tick(time.tick.0 + 1));
        }

        game_schedule.execute(self);

        self.resources.write::<Replay>().last_tick_recorded =
            self.resources.read::<GameTime>().tick;

        t.elapsed()
    }

    pub fn get_tick(&self) -> u64 {
        self.resources.read::<GameTime>().tick.0
    }

    pub fn hashes(&self) -> BTreeMap<String, u64> {
        let mut hashes = BTreeMap::new();
        let ser = common::saveload::Bincode::encode(&self.world).unwrap();
        hashes.insert("world".to_string(), common::hash_u64(&*ser));

        for l in saveload_funcs() {
            let v = (l.save)(self);
            hashes.insert(l.name.to_string(), common::hash_u64(&*v));
        }

        hashes
    }

    pub fn load_replay_from_disk(save_name: &str) -> Option<Replay> {
        let path = format!("{save_name}_replay");
        let replay: Replay = common::saveload::JSON::load(&path).ok()?;
        Some(replay)
    }

    pub fn load_from_disk(save_name: &str) -> Option<Self> {
        let sim: Simulation = common::saveload::CompressedBincode::load(save_name).ok()?;
        if sim.resources.try_read::<Map>().ok()?.environment.size().0 == 0 {
            return None;
        }
        Some(sim)
    }

    pub fn save_to_disk(&self, save_name: &str) {
        common::saveload::CompressedBincode::save(&self, save_name);
        let rep = self.resources.read::<Replay>();
        if rep.enabled {
            common::saveload::JSONPretty::save(&*rep, &format!("{save_name}_replay"));
        }
    }

    pub fn pos<E: WorldTransform>(&self, id: E) -> Option<Vec3> {
        self.world.pos(id)
    }

    pub fn pos_any(&self, id: AnyEntity) -> Option<Vec3> {
        self.world.pos_any(id)
    }

    pub fn get<E: EntityID>(&self, id: E) -> Option<&E::Entity> {
        self.world.get(id)
    }

    pub fn contains(&self, id: AnyEntity) -> bool {
        self.world.contains(id)
    }

    pub fn write_or_default<T: Any + Send + Sync + Default>(&mut self) -> RefMut<'_, T> {
        self.resources.write_or_default::<T>()
    }

    pub fn try_write<T: Any + Send + Sync>(&self) -> Option<RefMut<'_, T>> {
        self.resources.try_write().ok()
    }

    pub fn write<T: Any + Send + Sync>(&self) -> RefMut<'_, T> {
        self.resources.write()
    }

    pub fn read<T: Any + Send + Sync>(&self) -> Ref<'_, T> {
        self.resources.read()
    }

    pub fn map(&self) -> Ref<'_, Map> {
        self.resources.read()
    }
    /// Planner-visible external-trade availability (sov-8lu): true when at
    /// least one freight station's door sits within `DISPATCH_LANE_CUTOFF`
    /// of a driving lane — the same reachability filter `market_update`
    /// applies before offering a station as an import partner. A fresh map
    /// reads false here (the default station has no lane nearby), which is
    /// exactly the silent failure the HUD readout exists to surface. Pure
    /// getter; changes nothing.
    pub fn external_trade_available(&self) -> bool {
        let map = self.resources.read::<Map>();
        self.world.freight_stations.iter().any(|(_, f)| {
            map.buildings.get(f.f.building).is_some_and(|b| {
                map.nearest_lane(
                    b.door_pos,
                    crate::map::LaneKind::Driving,
                    Some(crate::map_dynamic::DISPATCH_LANE_CUTOFF),
                )
                .is_some()
            })
        })
    }

    /// Planner-visible stalled-dispatch count (sov-8lu): see
    /// `Market::stalled_dispatch_count`. Pure getter; changes nothing.
    pub fn stalled_dispatch_count(&self) -> usize {
        self.resources
            .read::<crate::economy::Market>()
            .stalled_dispatch_count()
    }

    pub(crate) fn map_mut(&self) -> RefMut<'_, Map> {
        self.resources.write()
    }

    pub fn insert<T: Any + Send + Sync>(&mut self, res: T) {
        self.resources.insert(res);
    }
}

impl Serialize for Simulation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        log::info!("serializing sim state");
        let t = Instant::now();
        let mut m: FastMap<String, Vec<u8>> = FastMap::default();

        for l in saveload_funcs() {
            let v: Vec<u8> = (l.save)(self);
            m.insert(l.name.to_string(), v);
        }

        log::info!("took {}s to serialize resources", t.elapsed().as_secs_f32());

        let v = SimulationSer {
            world: &self.world,
            version: VERSION.to_string(),
            res: m,
        }
        .serialize(serializer);
        log::info!("took {}s to serialize in total", t.elapsed().as_secs_f32());
        v
    }
}

#[derive(Serialize)]
struct SimulationSer<'a> {
    world: &'a World,
    version: String,
    res: FastMap<String, Vec<u8>>,
}

#[derive(Deserialize)]
struct SimulationDeser {
    world: World,
    version: String,
    res: FastMap<String, Vec<u8>>,
}

impl<'de> Deserialize<'de> for Simulation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        log::info!("deserializing sim state");
        let t = Instant::now();

        let mut simdeser = <SimulationDeser as Deserialize>::deserialize(deserializer)?;

        log::info!(
            "took {}s to deserialize base deser",
            t.elapsed().as_secs_f32()
        );

        let cur_version_parts = VERSION.split('.').collect::<Vec<_>>();
        let deser_parts = simdeser.version.split('.').collect::<Vec<_>>();

        if cur_version_parts[0] != deser_parts[0]
            || (cur_version_parts[0] == "0" && cur_version_parts[1] != deser_parts[1])
        {
            log::warn!(
                "incompatible version, save might be corrupted! save is: {} - game is: {}",
                simdeser.version,
                VERSION
            );
        }

        let mut sim = Self {
            world: World::default(),
            resources: Resources::default(),
        };

        for s in init_funcs() {
            (s.f)(&mut sim);
        }

        sim.world = simdeser.world;

        for l in saveload_funcs() {
            if let Some(data) = simdeser.res.remove(l.name) {
                (l.load)(&mut sim, data);
            }
        }

        log::info!(
            "took {}s to deserialize in total",
            t.elapsed().as_secs_f32()
        );

        Ok(sim)
    }
}

const START_COMMANDS: &str = r#"
[
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4343.2334,
            6262.846,
            0.0
          ],
          "kind": "Ground"
        },
        "to": {
          "pos": [
            4222.163,
            6318.7007,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": null,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": []
        }
      }
    }
  ],
  [
    0,
    {
      "MapBuildSpecialBuilding": {
        "pos": {
          "corners": [
            5010057980197058898,
            5010739497017993874,
            5009490559183158337,
            5008809038067256064
          ]
        },
        "kind": {"RailFreightStation": 9010703082962909221},
        "gen": {"NoWalkway": {
          "door_pos": 0
        }},
        "zone": null
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            2024.0668,
            147.33333,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 2,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            2831.1282,
            2172.9182,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": 4972985476040057325,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": [
            [
              "Rail",
              9.0
            ]
          ]
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            2831.1282,
            2172.9182,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 5,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4098.609,
            4876.7466,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": 5005986054842354977,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": [
            [
              "Rail",
              9.0
            ]
          ]
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4098.609,
            4876.7466,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 6,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4180.8335,
            5765.297,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": 5008381821963615703,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": [
            [
              "Rail",
              9.0
            ]
          ]
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4180.8335,
            5765.297,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 7,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4312.9233,
            5982.129,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": 5009042048632353377,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": []
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4312.9233,
            5982.129,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 8,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4418.4004,
            6150.2427,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": 5010742073997638941,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": []
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4418.4004,
            6150.2427,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 9,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4343.2334,
            6262.846,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 3,
              "version": 1
            }
          }
        },
        "inter": 5010861869225921041,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": []
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4222.163,
            6318.7007,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 4,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4080.5332,
            6242.0093,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": 5008055507530249597,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": []
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4080.5332,
            6242.0093,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 10,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4134.9575,
            5988.7896,
            0.0
          ],
          "kind": "Ground"
        },
        "inter": 5007338222221478024,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": []
        }
      }
    }
  ],
  [
    0,
    {
      "MapMakeConnection": {
        "from": {
          "pos": [
            4134.9575,
            5988.7896,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 11,
              "version": 1
            }
          }
        },
        "to": {
          "pos": [
            4180.8335,
            5765.297,
            0.0
          ],
          "kind": {
            "Intersection": {
              "idx": 7,
              "version": 1
            }
          }
        },
        "inter": 5008608536108168175,
        "pat": {
          "lanes_forward": [
            [
              "Rail",
              9.0
            ]
          ],
          "lanes_backward": []
        }
      }
    }
  ],
  [
    0,
    {
      "AddTrain": {
        "dist": 150.0,
        "n_wagons": 7,
        "lane": {
          "idx": 3,
          "version": 1
        }
      }
    }
  ]
]
"#;
