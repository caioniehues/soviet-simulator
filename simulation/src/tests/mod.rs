#![allow(dead_code)]
#![cfg(test)]

use crate::map::{BuildingID, BuildingKind, LanePatternBuilder, ProjectFilter};
use crate::map_dynamic::BuildingInfos;
use crate::utils::scheduler::SeqSchedule;
use crate::world_command::{WorldCommand, WorldCommands};
use crate::{Simulation, SimulationOptions};
use common::logger::MyLog;
use common::saveload::Encoder;
use geom::{Vec2, Vec3, OBB};
use prototypes::BuildingGen;
use std::sync::Once;

mod determinism_gate;
mod scenarios;
mod test_iso;
mod vehicles;

pub(crate) struct TestCtx {
    pub g: Simulation,
    sched: SeqSchedule,
}

// `crate::init::init()` does expensive one-time work (parsing prototypes, building
// the system registry). It only needs to run once per process.
static INIT: Once = Once::new();

impl TestCtx {
    pub(crate) fn new() -> Self {
        MyLog::init();
        INIT.call_once(crate::init::init);

        let g = Simulation::new_with_options(SimulationOptions {
            terrain_size: 1,
            save_replay: false,
            ..Default::default()
        });
        let sched = Simulation::schedule();

        Self { g, sched }
    }

    pub(crate) fn build_roads(&self, v: &[Vec3]) {
        let mut m = self.g.map_mut();
        for w in v.windows(2) {
            let a = m.project(w[0], 0.0, ProjectFilter::ALL);
            let b = m.project(w[1], 0.0, ProjectFilter::ALL);
            m.make_connection(a, b, None, &LanePatternBuilder::default().build());
        }
    }

    pub(crate) fn build_house_near(&self, p: Vec2) -> BuildingID {
        let lot = self
            .g
            .map()
            .lots()
            .values()
            .min_by_key(|lot| lot.shape.center().distance2(p) as i32)
            .unwrap()
            .id;

        let b = self.g.map_mut().build_house(lot).unwrap();
        self.g.write::<BuildingInfos>().insert(b);
        b
    }

    /// Places a building at an explicit position, without consulting `map().lots()`.
    /// Unlike `build_house_near`, this survives lot auto-generation being removed.
    pub(crate) fn build_house_at(&mut self, p: Vec2) -> BuildingID {
        let obb = OBB::new(p, Vec2::X, 20.0, 20.0);
        let b = self
            .g
            .map_mut()
            .build_special_building(&obb, BuildingKind::House, BuildingGen::House, None, None)
            .unwrap();
        self.g.write::<BuildingInfos>().insert(b);
        b
    }

    pub(crate) fn apply(&mut self, commands: &[WorldCommand]) {
        for c in commands {
            c.apply(&mut self.g);
        }
    }

    pub(crate) fn tick(&mut self) {
        self.g
            .tick(&mut self.sched, WorldCommands::default().as_ref());
        self.check_determinism();
    }

    /// Advances `n` ticks, only paying for the encode/decode determinism check
    /// every `CHECK_EVERY` ticks and on the final tick. Use for long journeys
    /// where per-tick `tick()` would dominate runtime.
    pub(crate) fn advance_ticks(&mut self, n: u32) {
        const CHECK_EVERY: u32 = 25;
        for i in 0..n {
            self.g
                .tick(&mut self.sched, WorldCommands::default().as_ref());
            if i % CHECK_EVERY == CHECK_EVERY - 1 || i + 1 == n {
                self.check_determinism();
            }
        }
    }

    fn check_determinism(&self) {
        let serialized = common::saveload::Bincode::encode(&self.g).unwrap();
        let deserialized: Simulation = common::saveload::Bincode::decode(&serialized).unwrap();

        let testhashes = self.g.hashes();
        for (key, hash) in deserialized.hashes().iter() {
            assert_eq!(
                testhashes.get(key),
                Some(hash),
                "key: {:?} at tick {}",
                key,
                self.g.get_tick(),
            );
        }
    }
}
