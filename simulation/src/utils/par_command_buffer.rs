use crate::utils::resources::Resources;
use crate::world::Entity;
use crate::Simulation;
use std::sync::Mutex;

pub trait SimDrop: Entity {
    fn sim_drop(self, id: Self::ID, world: &mut crate::World, res: &mut Resources);
}

type ExecType = Box<dyn for<'a> FnOnce(&'a mut Simulation) + Send>;

pub struct ParCommandBuffer<E: SimDrop> {
    to_kill: Mutex<Vec<E::ID>>,
    exec_ent: Mutex<Vec<(E::ID, ExecType)>>,
}

impl<E: SimDrop> Default for ParCommandBuffer<E> {
    fn default() -> Self {
        Self {
            to_kill: Default::default(),
            exec_ent: Default::default(),
        }
    }
}

#[allow(clippy::unwrap_used)]
impl<E: SimDrop> ParCommandBuffer<E> {
    pub fn kill(&self, e: E::ID) {
        self.to_kill.lock().unwrap().push(e);
    }

    pub fn kill_all(&self, e: &[E::ID]) {
        self.to_kill.lock().unwrap().extend_from_slice(e);
    }

    pub fn exec_ent(&self, e: E::ID, f: impl for<'a> FnOnce(&'a mut Simulation) + 'static + Send) {
        self.exec_ent.lock().unwrap().push((e, Box::new(f)));
    }

    pub fn exec_on<T: Send + Sync + 'static>(
        &self,
        e: E::ID,
        f: impl for<'a> FnOnce(&'a mut T) + 'static + Send,
    ) {
        self.exec_ent(e, move |sim| {
            f(&mut *sim.write::<T>());
        })
    }

    pub fn apply(sim: &mut Simulation) {
        profiling::scope!("par_command_buffer::apply");
        let mut deleted: Vec<E::ID> = std::mem::take(
            &mut *sim
                .write::<ParCommandBuffer<E>>()
                .to_kill
                .get_mut()
                .unwrap(),
        );

        deleted.sort_unstable();

        for entity in deleted {
            let Some(v) = E::storage_mut(&mut sim.world).remove(entity) else {
                continue;
            };

            E::sim_drop(v, entity, &mut sim.world, &mut sim.resources);
        }

        let mut exec_ent = std::mem::take(
            &mut *sim
                .write::<ParCommandBuffer<E>>()
                .exec_ent
                .get_mut()
                .unwrap(),
        );

        // sov-7l1: defined same-tick kill+exec behaviour is SKIP. Kills
        // drain above before execs run, but that ordering alone never made
        // deferred work safe: the old loop discarded the entity id
        // (`for (_, exec)`) and ran every closure, including closures queued
        // for entities killed earlier in this same drain. Never cite "kills
        // drain first" as a safety argument for an unwrap inside an exec
        // closure; the liveness check below is the actual guarantee, so
        // callers MUST NOT add their own per-closure liveness checks.
        //
        // Caller audit (why skipping is the safe outcome for each kind):
        // - `unpark` closures (economy/market.rs ToSource rollback,
        //   map_dynamic/router.rs Unpark step): `unpark` already refuses
        //   unknown entities, so a skip matches the old no-op outcome. The
        //   market rollback's Market/Dispatcher release is re-performed next
        //   tick by the designed wedge-(b) path for a gone reserved truck
        //   (economy/market.rs `DispatchState::ToSource`, `Some(v)` arm with
        //   `world.vehicles.get(v).is_none()`).
        // - `get_mut`-guarded closures (router.rs walk_outside,
        //   souls/goods_company.rs driver/worker assigns, souls/human.rs
        //   freight wait): they early-return on a missing entity, so the
        //   skip only takes their no-op branch earlier.
        // - `exec_on` Market closures (souls/desire/buyfood.rs buy and
        //   settle_retail, souls/goods_company.rs recipe_act): they write
        //   Market rows keyed by a soul whose `sim_drop` already ran
        //   `Market::remove`. Running them would resurrect ledger rows for
        //   a dead soul (recipe_act even unwraps the erased `requested`
        //   row); skipping is the fix.
        // - `Transporter::destroy` closures (router.rs walk_inside,
        //   transportation/road.rs parking-complete): these free a grid
        //   handle already detached from the component, so they never needed
        //   the entity alive -- but the skip DOES strand that handle if a
        //   kill lands in the same drain after the detach. No HumanEnt kill
        //   path exists through any buffer, so walk_inside is unaffected;
        //   the road case needs a company-teardown kill of a truck in the
        //   same drain it finishes parking. Rare, accepted, and owned by a
        //   follow-up if it ever fires, not by per-closure checks here.
        for (id, exec) in exec_ent {
            if E::storage(&sim.world).contains_key(id) {
                exec(sim);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParCommandBuffer;
    use crate::transportation::{Speed, Vehicle, VehicleKind, VehicleState};
    use crate::world::VehicleEnt;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Once};

    static INIT: Once = Once::new();

    fn test_sim() -> crate::Simulation {
        INIT.call_once(crate::init::init);
        crate::Simulation::new_with_options(crate::SimulationOptions {
            terrain_size: 1,
            save_replay: false,
            ..Default::default()
        })
    }

    /// A `VehicleEnt` whose `sim_drop` touches no resource: no collider, not
    /// parked, not a truck. Killing it needs only the world itself.
    fn test_vehicle() -> VehicleEnt {
        VehicleEnt {
            trans: geom::Transform {
                pos: geom::vec3(0.0, 0.0, 0.0),
                dir: geom::vec3(1.0, 0.0, 0.0),
            },
            speed: Speed(0.0),
            vehicle: Vehicle {
                ang_velocity: 0.0,
                wait_time: 0.0,
                max_speed_multiplier: 1.0,
                state: VehicleState::Driving,
                kind: VehicleKind::Car,
                tint: geom::Color::default(),
                flag: 0,
            },
            it: Default::default(),
            collider: None,
        }
    }

    #[test]
    fn same_tick_kill_skips_exec() {
        let mut sim = test_sim();
        let id = sim.world.vehicles.insert(test_vehicle());
        let ran = Arc::new(AtomicBool::new(false));
        let probe = Arc::clone(&ran);
        sim.write::<ParCommandBuffer<VehicleEnt>>().kill(id);
        sim.write::<ParCommandBuffer<VehicleEnt>>()
            .exec_ent(id, move |_| probe.store(true, Ordering::SeqCst));
        ParCommandBuffer::<VehicleEnt>::apply(&mut sim);
        assert!(
            sim.world.vehicles.get(id).is_none(),
            "kill must still apply in the same drain"
        );
        assert!(
            !ran.load(Ordering::SeqCst),
            "sov-7l1: exec for an entity killed in the same drain must be skipped"
        );
    }

    #[test]
    fn live_entity_exec_still_runs() {
        let mut sim = test_sim();
        let id = sim.world.vehicles.insert(test_vehicle());
        let ran = Arc::new(AtomicBool::new(false));
        let probe = Arc::clone(&ran);
        sim.write::<ParCommandBuffer<VehicleEnt>>()
            .exec_ent(id, move |_| probe.store(true, Ordering::SeqCst));
        ParCommandBuffer::<VehicleEnt>::apply(&mut sim);
        assert!(
            sim.world.vehicles.get(id).is_some(),
            "live entity must survive an exec-only drain"
        );
        assert!(
            ran.load(Ordering::SeqCst),
            "exec for a live entity must still run"
        );
    }
}
