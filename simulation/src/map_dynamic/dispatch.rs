use crate::map::{LaneID, LaneKind, TraverseDirection};
use crate::transportation::VehicleKind;
use crate::utils::resources::Resources;
use crate::world::{TrainID, VehicleID};
use crate::{Map, World};
use derive_more::From;
use geom::Vec3;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::cmp::Reverse;
use ordered_float::OrderedFloat;

/// How precise the dispatcher is. Caches dispatchable entities's positions and relation to map but only in precision circle.
/// So if a dispatchable entity moves less than the precision, nothing will be updated.
const PRECISION_RADIUS: f32 = 5.0;
const PRECISION_RADIUS_2: f32 = PRECISION_RADIUS * PRECISION_RADIUS;

/// Dispatcher is used to query for the closest networked entity matching a condition
/// For example:
/// - A rail freight station will query for the closest train to it that is not already used by another station
/// - A factory will query for a truck to deliver goods
/// - A hospital will query for the closest injured person
#[derive(Default, Serialize, Deserialize)]
pub struct Dispatcher {
    dispatches: BTreeMap<DispatchKind, DispatchOne>,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Serialize, Deserialize, From)]
pub enum DispatchID {
    FreightTrain(TrainID),
    SmallTruck(VehicleID),
}

impl From<DispatchID> for DispatchKind {
    fn from(id: DispatchID) -> Self {
        match id {
            DispatchID::FreightTrain(_) => DispatchKind::FreightTrain,
            DispatchID::SmallTruck(_) => DispatchKind::SmallTruck,
        }
    }
}

/// Dispatcher specialized to one kind
#[derive(Serialize, Deserialize)]
struct DispatchOne {
    positions: BTreeMap<DispatchID, DispatchPosition>,
    lanes: BTreeMap<LaneID, Vec<DispatchID>>,
    reserved_by: BTreeSet<DispatchID>,
    lanekind: LaneKind,
}

#[derive(Serialize, Deserialize)]
struct DispatchPosition {
    lane: LaneID,
    pos: Vec3,
    dist_along: f32,
}

/// DispatchKind is a component that is added to entities that can be dispatched
/// Usually constant.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Inspect)]
pub enum DispatchKind {
    FreightTrain,
    SmallTruck,
}

impl DispatchKind {
    pub fn lane_kind(self) -> LaneKind {
        match self {
            DispatchKind::FreightTrain => LaneKind::Rail,
            DispatchKind::SmallTruck => LaneKind::Driving,
        }
    }
}

/// DispatchQueryTarget is the target that can be queried to the dispatcher
#[derive(Copy, Clone)]
pub enum DispatchQueryTarget {
    Pos(Vec3),
    Lane(LaneID),
}

/// How far from a lane of the right kind a `DispatchQueryTarget::Pos` may sit
/// and still be served. A building whose door is further away than this can
/// never be offered a vehicle, so anything that wants to be *delivered to*
/// must check the same distance before promising a delivery.
pub const DISPATCH_LANE_CUTOFF: f32 = 50.0;

impl Dispatcher {
    /// Updates the dispatcher cache about the dispatachable entities to know where they are relative
    /// to the map, so that queries can be answered quickly
    pub fn update(&mut self, map: &Map, world: &World) {
        let disp_trains = self
            .dispatches
            .entry(DispatchKind::FreightTrain)
            .or_insert_with(|| DispatchOne::new(DispatchKind::FreightTrain.lane_kind()));

        world.trains.iter().for_each(|(ent, train)| {
            disp_trains.register(DispatchID::FreightTrain(ent), map, train.trans.pos);
        });

        let disp_trucks = self
            .dispatches
            .entry(DispatchKind::SmallTruck)
            .or_insert_with(|| DispatchOne::new(DispatchKind::SmallTruck.lane_kind()));

        world.vehicles.iter().for_each(|(ent, truck)| {
            if matches!(truck.vehicle.kind, VehicleKind::Truck) {
                disp_trucks.register(DispatchID::SmallTruck(ent), map, truck.trans.pos);
            }
        })
    }

    /// Frees the entity as it is no longer used
    /// For example if a train is no longer used by a station, it should be freed so that other stations can use it
    /// It should be re-added to the cache at the next update iteration
    pub fn free(&mut self, ent: impl Into<DispatchID>) {
        let ent: DispatchID = ent.into();
        let kind: DispatchKind = ent.into();
        let Some(disp) = self.dispatches.get_mut(&kind) else {
            return;
        };
        disp.reserved_by.remove(&ent);
    }

    /// Whether `ent` is still held by a reservation.
    ///
    /// This is the ONLY way to observe a leaked reservation for an entity that
    /// no longer exists in the world. `DispatchOne::reserve` removes the entity
    /// from `positions`, and `free` puts it back only via the next `update`,
    /// which iterates LIVE vehicles — so a despawned entity is invisible to
    /// `query` whether or not it was freed, and any assertion phrased over
    /// queryable entities is blind to the leak.
    ///
    /// Shared beyond tests: the sov-aam abandoned-truck recovery
    /// (`transportation::testing_vehicles`) and its sketch assertion
    /// (`tests::determinism_gate`) both need to tell a truck owned by a live
    /// dispatch from an abandoned one without touching the market.
    pub(crate) fn is_reserved(&self, ent: impl Into<DispatchID>) -> bool {
        let ent: DispatchID = ent.into();
        let kind: DispatchKind = ent.into();
        self.dispatches
            .get(&kind)
            .is_some_and(|disp| disp.reserved_by.contains(&ent))
    }

    pub fn unregister(&mut self, id: DispatchID) {
        let kind = id.into();
        let Some(disp) = self.dispatches.get_mut(&kind) else {
            return;
        };
        disp.unregister(id);
    }

    /// Reserves an entity that is closest to the target (if it is found) and returns it
    /// it takes `me` as an argument so that if `me` is killed, the reservation is cancelled
    /// If no entity is found, returns None
    pub fn query(
        &mut self,
        map: &Map,
        kind: DispatchKind,
        target: DispatchQueryTarget,
    ) -> Option<DispatchID> {
        let disp = self.dispatches.get_mut(&kind)?;
        let best_ent = disp.query(map, kind, target)?;
        disp.reserve(best_ent);
        Some(best_ent)
    }
}

impl DispatchOne {
    fn new(lanekind: LaneKind) -> Self {
        Self {
            positions: Default::default(),
            lanes: Default::default(),
            reserved_by: Default::default(),
            lanekind,
        }
    }

    fn register(&mut self, id: DispatchID, map: &Map, pos: Vec3) {
        let ent = self.positions.entry(id);

        let lanekind = self.lanekind;
        let find_lane = move || map.nearest_lane(pos, lanekind, Some(DISPATCH_LANE_CUTOFF));

        match ent {
            Entry::Vacant(v) => {
                let Some(n) = find_lane() else { return };
                let newl = &map.lanes[n];
                let proj = newl.points.project(pos);

                self.lanes.entry(n).or_default().push(id);
                v.insert(DispatchPosition {
                    lane: n,
                    pos,
                    dist_along: newl.points.length_at_proj(proj),
                });
            }
            Entry::Occupied(mut o) => {
                let dp = o.get_mut();

                if dp.pos.distance2(pos) < PRECISION_RADIUS_2 {
                    return;
                }

                if let Some(l) = map.lanes().get(dp.lane) {
                    let projected = l.points.project(pos);
                    if projected.distance2(pos) < PRECISION_RADIUS_2 {
                        dp.dist_along = l.points.length_at_proj(projected);
                        return;
                    }
                }

                let Some(n) = find_lane() else { return };
                self.lanes.get_mut(&dp.lane).unwrap().retain(|e| *e != id);
                self.lanes.entry(n).or_default().push(id);

                let newl = &map.lanes[n];

                let projected = newl.points.project(pos);
                *dp = DispatchPosition {
                    lane: n,
                    pos,
                    dist_along: newl.points.length_at_proj(projected),
                };
            }
        }
    }

    fn reserve(&mut self, id: DispatchID) {
        self.reserved_by.insert(id);
        let Some(pos) = self.positions.remove(&id) else {
            log::error!("Dispatcher: trying to reserve an entity that is not in the cache");
            return;
        };
        self.lanes.get_mut(&pos.lane).unwrap().retain(|e| *e != id);
    }

    pub fn unregister(&mut self, id: DispatchID) {
        // sov-w03: `reserve` removes the entity from `positions`, so a truck
        // destroyed while reserved has no position entry. The reservation must
        // be cleared FIRST: the old early-return below kept the dead id in
        // `reserved_by` forever (no later `update` ever sees it again).
        self.reserved_by.remove(&id);
        let Some(pos) = self.positions.remove(&id) else {
            return;
        };
        self.lanes.get_mut(&pos.lane).unwrap().retain(|e| *e != id);
    }

    /// Finds an entity that is closest to the target and returns it
    /// If no entity is found, returns None
    ///
    /// Ranked assignment (sov-2uv, SPEC-LOGISTICS-010): candidates compete by
    /// meaningful route distance — network meters from the vehicle to the
    /// target along the lane graph, not hop count — with the vehicle identity
    /// as the stable tie-break. Money and price never participate: this
    /// function never sees them. Demand-side deficit ordering lives above
    /// this layer (the target policy); here every queried demand gets the
    /// vehicle that reaches it cheapest, so a station's own parked trucks
    /// serve the border leg while factory trucks keep serving short hops.
    /// A vehicle past the target on its own lane would have to loop the
    /// block to come back, so it ranks in a fallback tier below every
    /// direct candidate — but it is still offered when nothing else is
    /// left, rather than starving the demand outright.
    pub fn query(
        &mut self,
        map: &Map,
        kind: DispatchKind,
        target: DispatchQueryTarget,
    ) -> Option<DispatchID> {
        if self.positions.is_empty() {
            return None;
        }

        // `start_cost[lane]`: network meters from that lane's START to the
        // target. The target lane's own start sits `base_along` meters out;
        // every predecessor adds its successor's full length on top.
        let mut base_along = 0.0;
        let target_lane = match target {
            DispatchQueryTarget::Pos(pos) => {
                let lid = map.nearest_lane(pos, kind.lane_kind(), Some(DISPATCH_LANE_CUTOFF))?;
                let lane = map.lanes().get(lid)?;
                let proj = lane.points.project(pos);
                base_along = lane.points.length_at_proj(proj);
                lid
            }
            DispatchQueryTarget::Lane(lane) => {
                #[allow(clippy::question_mark)]
                if map.lanes().get(lane).is_none() {
                    return None;
                }
                lane
            }
        };

        // Dijkstra upstream from the target over predecessor lanes (the same
        // `turns_to` expansion the old BFS used). A candidate at `dist_along`
        // in lane L costs `start_cost[L] + (len(L) - dist_along)` — its
        // remaining lane plus the network behind it. Lanes pop in cost
        // order, so once the heap top is strictly worse than the best
        // candidate no later lane can beat it.
        let mut start_cost: BTreeMap<LaneID, f32> = BTreeMap::new();
        let mut heap: BinaryHeap<(Reverse<OrderedFloat<f32>>, LaneID)> = BinaryHeap::new();
        start_cost.insert(target_lane, base_along);
        heap.push((Reverse(OrderedFloat(base_along)), target_lane));

        let mut best: Option<(f32, DispatchID)> = None;
        // Fallback tier (sov-2uv): vehicles past the target on its own
        // lane. They cannot reverse into it; they must drive to the lane
        // end and loop the block back — categorically costlier and less
        // predictable than any truck approaching direct, so they compete
        // only against each other and are offered only when no direct
        // candidate exists anywhere. Skipping them outright starved a
        // domestic hop whose own factory truck had parked centimetres
        // past its door while the rest of the fleet was out on long
        // imports (the query returned None forever); ranking them first
        // by raw meters would hand out 1200 m block-loops for 18 m hops.
        let mut best_loop: Option<(f32, DispatchID)> = None;

        while let Some((Reverse(cost), lid)) = heap.pop() {
            let cost = cost.into_inner();
            if start_cost.get(&lid).is_some_and(|&c| c < cost) {
                continue;
            }
            if best.is_some_and(|(c, _)| cost > c) {
                break;
            }

            let lane_len = map.lanes[lid].points.length();
            if let Some(ents) = self.lanes.get(&lid) {
                for ent in ents {
                    if self.reserved_by.contains(ent) {
                        continue;
                    }
                    let Some(pos) = self.positions.get(ent) else {
                        continue;
                    };
                    let on_target_lane = lid == target_lane
                        && matches!(target, DispatchQueryTarget::Pos(_));
                    if on_target_lane && pos.dist_along > base_along {
                        // Fallback tier: loop the block (lane end, then lane
                        // start to target as the ordering cost). Never
                        // competes with direct candidates and never prunes
                        // the search for one (see `best` below).
                        let cand = (lane_len - pos.dist_along) + base_along;
                        let replace = match best_loop {
                            None => true,
                            Some((c, id)) => cand < c || (cand == c && *ent < id),
                        };
                        if replace {
                            best_loop = Some((cand, *ent));
                        }
                        continue;
                    }
                    let cand = if on_target_lane {
                        base_along - pos.dist_along
                    } else {
                        cost + (lane_len - pos.dist_along)
                    };
                    let replace = match best {
                        None => true,
                        Some((c, id)) => cand < c || (cand == c && *ent < id),
                    };
                    if replace {
                        best = Some((cand, *ent));
                    }
                }
            }

            let l = &map.lanes[lid];
            let int = &map.intersections[l.src];
            for (tid, dir) in int.turns_to(lid) {
                let pred = match dir {
                    TraverseDirection::Forward => tid.src,
                    TraverseDirection::Backward => tid.dst,
                };
                let next = cost + lane_len;
                if next < start_cost.get(&pred).copied().unwrap_or(f32::MAX) {
                    start_cost.insert(pred, next);
                    heap.push((Reverse(OrderedFloat(next)), pred));
                }
            }
        }
        // Direct approach wins over any block-loop, but a looper still
        // beats nothing: it is offered when it is the only truck left.
        best.or(best_loop).map(|(_, ent)| ent)
    }
}

pub fn dispatch_system(world: &mut World, resources: &mut Resources) {
    profiling::scope!("map_dynamic::dispatch");

    let mut dispatcher = resources.write::<Dispatcher>();
    let map = resources.read::<Map>();
    dispatcher.update(&map, world);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{LanePatternBuilder, MapProject, ProjectKind};
    use common::rand::rand2;

    fn mk_ent(id: u64) -> DispatchID {
        DispatchID::FreightTrain(TrainID::from(KeyData::from_ffi(id)))
    }

    #[test]
    fn dispatch_one_register_one_works() {
        let mut disp = DispatchOne::new(LaneKind::Rail);
        let mut map = Map::default();

        let (_, r) = map
            .make_connection(
                MapProject::ground(Vec3::ZERO),
                MapProject::ground(Vec3::x(100.0)),
                None,
                &LanePatternBuilder::new().rail(true).build(),
            )
            .unwrap();

        let lanes: Vec<LaneID> = map.roads[r].lanes_iter().map(|(id, _)| id).collect();

        // first insert
        let ent = mk_ent(1 << 32);
        disp.register(ent, &map, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(disp.positions.len(), 1);
        assert_eq!(disp.lanes.len(), 1);
        assert_eq!(disp.lanes.values().next().unwrap()[0], ent);
        assert!(lanes.contains(disp.lanes.keys().next().unwrap()));

        // second insert in same lane
        let ent2 = mk_ent((1 << 32) + 1);
        disp.register(ent2, &map, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(disp.positions.len(), 2);
        assert_eq!(disp.lanes.len(), 1);
        assert_eq!(disp.lanes.values().next().unwrap(), &vec![ent, ent2]);

        // insert in another lane
        let ent3 = mk_ent((1 << 32) + 2);
        disp.register(ent3, &map, Vec3::new(100.0, 10.0, 0.0));
        assert_eq!(disp.positions.len(), 3);
        assert_eq!(disp.lanes.len(), 2);
        let mut v = disp.lanes.values();
        assert_eq!(v.next().unwrap(), &vec![ent, ent2]);
        assert_eq!(v.next().unwrap(), &vec![ent3]);

        // unregister
        disp.unregister(ent);
        assert_eq!(disp.positions.len(), 2);
        assert_eq!(disp.lanes.len(), 2);
        let mut v = disp.lanes.values();
        assert_eq!(v.next().unwrap(), &vec![ent2]);
        assert_eq!(v.next().unwrap(), &vec![ent3]);

        // unregister again
        disp.unregister(ent2);
        assert_eq!(disp.positions.len(), 1);
        assert_eq!(disp.lanes.len(), 2);
        let mut v = disp.lanes.values();
        assert_eq!(v.next().unwrap(), &vec![]);
        assert_eq!(v.next().unwrap(), &vec![ent3]);

        // ent3 moves from a lane to another
        disp.register(ent3, &map, Vec3::new(100.0, -1.0, 0.0));
        let mut v = disp.lanes.values();
        assert_eq!(v.next().unwrap(), &vec![ent3]);
        assert_eq!(v.next().unwrap(), &vec![]);

        // ent3 doesn't change lane because it's close to the old one
        disp.register(ent3, &map, Vec3::new(100.0, 1.0, 0.0));
        let mut v = disp.lanes.values();
        assert_eq!(v.next().unwrap(), &vec![ent3]);
        assert_eq!(v.next().unwrap(), &vec![]);
    }

    /// sov-w03: a truck destroyed while reserved must not keep its id in
    /// `reserved_by` forever. `reserve` removes the entity from `positions`,
    /// so the old `unregister` early-returned before clearing the
    /// reservation. Written at `Dispatcher` level through the test-only
    /// `is_reserved` probe: a despawned truck never reappears in `query`
    /// (only live vehicles are re-registered by `update`), so `reserved_by`
    /// is the only place the leak is observable. Fails if the
    /// `reserved_by.remove` moves back below the early return.
    #[test]
    fn unregister_reserved_truck_clears_reservation() {
        let mut d = Dispatcher::default();
        let mut map = Map::default();

        map.make_connection(
            MapProject::ground(Vec3::ZERO),
            MapProject::ground(Vec3::x(100.0)),
            None,
            &LanePatternBuilder::default().build(),
        )
        .unwrap();

        let truck = DispatchID::SmallTruck(VehicleID::from(KeyData::from_ffi(1 << 32)));
        d.dispatches
            .entry(DispatchKind::SmallTruck)
            .or_insert(DispatchOne::new(DispatchKind::SmallTruck.lane_kind()))
            .register(truck, &map, Vec3::ZERO);

        // Reserve it the way `query` does, then destroy it the way
        // `VehicleEnt::sim_drop` does.
        d.dispatches
            .get_mut(&DispatchKind::SmallTruck)
            .unwrap()
            .reserve(truck);
        assert!(
            d.is_reserved(truck),
            "precondition: the truck must be reserved before it is destroyed"
        );
        d.unregister(truck);
        assert!(
            !d.is_reserved(truck),
            "unregister must clear reserved_by even when positions has no entry"
        );
    }

    #[test]
    fn query_same_lane_works() {
        let mut d = Dispatcher::default();
        let mut map = Map::default();

        let (_, r) = map
            .make_connection(
                MapProject::ground(Vec3::ZERO),
                MapProject::ground(Vec3::x(100.0)),
                None,
                &LanePatternBuilder::new().one_way(true).rail(true).build(),
            )
            .unwrap();

        let (lid, _) = map.roads[r].lanes_iter().next().unwrap();

        let mut register = |id: DispatchID, pos: f32| {
            d.dispatches
                .entry(DispatchKind::FreightTrain)
                .or_insert(DispatchOne::new(DispatchKind::FreightTrain.lane_kind()))
                .register(id, &map, Vec3::x(pos))
        };

        let ent0 = mk_ent(1 << 32);
        let ent1 = mk_ent((1 << 32) + 1);
        let ent2 = mk_ent((1 << 32) + 2);

        register(ent0, 0.0);
        register(ent1, 10.0);
        register(ent2, 100.0);

        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Pos(Vec3::x(70.0)),
            ),
            Some(ent1)
        );
        d.dispatches[&DispatchKind::FreightTrain]
            .reserved_by
            .contains(&mk_ent((1 << 32) + 1));

        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Pos(Vec3::x(50.0)),
            ),
            Some(ent0)
        );
        d.dispatches[&DispatchKind::FreightTrain]
            .reserved_by
            .contains(&mk_ent(1 << 32));

        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Lane(lid),
            ),
            Some(ent2)
        );
        d.free(ent2);
        // `free` clears the reservation but only the next `update` (live
        // vehicles) would restore the position entry `reserve` removed, so
        // re-register the freed truck for the fallback probe below.
        d.dispatches
            .get_mut(&DispatchKind::FreightTrain)
            .unwrap()
            .register(ent2, &map, Vec3::x(100.0));
        // sov-2uv fallback tier: with direct candidates exhausted, the
        // past-target same-lane truck is offered via the block loop rather
        // than skipped outright.
        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Pos(Vec3::x(50.0)),
            ),
            Some(ent2)
        );

    }

    #[test]
    fn query_two_lanes_bfs() {
        let mut d = Dispatcher::default();
        let mut map = Map::default();

        let (i, _) = map
            .make_connection(
                MapProject::ground(Vec3::ZERO),
                MapProject::ground(Vec3::x(100.0)),
                None,
                &LanePatternBuilder::new().one_way(true).rail(true).build(),
            )
            .unwrap();

        let (_, r2) = map
            .make_connection(
                MapProject {
                    kind: ProjectKind::Intersection(i),
                    pos: Vec3::x(100.0),
                },
                MapProject::ground(Vec3::new(200.0, 50.0, 0.0)),
                None,
                &LanePatternBuilder::new().one_way(true).rail(true).build(),
            )
            .unwrap();

        // unrelated
        map.make_connection(
            MapProject::ground(Vec3::new(0.0, 10.0, 0.0)),
            MapProject::ground(Vec3::new(100.0, 10.0, 0.0)),
            None,
            &LanePatternBuilder::new().one_way(true).rail(true).build(),
        )
        .unwrap();

        let (lid, _) = map.roads[r2].lanes_iter().next().unwrap();

        let mut register = |id: DispatchID, pos: f32| {
            d.dispatches
                .entry(DispatchKind::FreightTrain)
                .or_insert(DispatchOne::new(DispatchKind::FreightTrain.lane_kind()))
                .register(id, &map, Vec3::x(pos))
        };

        let ent0 = mk_ent(1 << 32);
        let ent1 = mk_ent((1 << 32) + 1);
        let ent2 = mk_ent((1 << 32) + 2);

        register(ent0, 0.0);
        register(ent1, 10.0);
        register(ent2, 200.0);

        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Pos(Vec3::x(70.0)),
            ),
            Some(ent1)
        );
        d.dispatches[&DispatchKind::FreightTrain]
            .reserved_by
            .contains(&mk_ent((1 << 32) + 1));

        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Pos(Vec3::x(50.0)),
            ),
            Some(ent0)
        );
        d.dispatches[&DispatchKind::FreightTrain]
            .reserved_by
            .contains(&mk_ent(1 << 32));

        assert!(d
            .query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Pos(Vec3::x(50.0)),
            )
            .is_none());

        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Lane(lid),
            ),
            Some(ent2)
        );
    }

    /// sov-2uv: ranked assignment ends in a stable identity tie-break. Two
    /// vehicles at the same network distance from the target must resolve to
    /// the smaller identity, deterministically, regardless of registration
    /// order — never to whichever the lane's vec happens to list first.
    #[test]
    fn query_equal_distance_prefers_smaller_identity() {
        let mut d = Dispatcher::default();
        let mut map = Map::default();

        map.make_connection(
            MapProject::ground(Vec3::ZERO),
            MapProject::ground(Vec3::x(100.0)),
            None,
            &LanePatternBuilder::new().one_way(true).rail(true).build(),
        )
        .unwrap();

        let mut register = |id: DispatchID, pos: Vec3| {
            d.dispatches
                .entry(DispatchKind::FreightTrain)
                .or_insert(DispatchOne::new(DispatchKind::FreightTrain.lane_kind()))
                .register(id, &map, pos)
        };

        // Same spot, so same lane and same dist_along: a pure tie. The
        // LARGER identity registers first so insertion order disagrees with
        // the expected answer.
        let lo = mk_ent(1 << 32);
        let hi = mk_ent((1 << 32) + 1);
        register(hi, Vec3::x(10.0));
        register(lo, Vec3::x(10.0));

        assert_eq!(
            d.query(
                &map,
                DispatchKind::FreightTrain,
                DispatchQueryTarget::Pos(Vec3::x(70.0)),
            ),
            Some(lo)
        );
    }

    use crate::map::procgen::load_parismap;
    use easybench::bench;
    use slotmapd::KeyData;

    #[test]
    fn bench_query() {
        /* if 1 == 1 {
            return;
        }*/

        let mut m = Map::default();
        load_parismap(&mut m);

        let mut minx = f32::MAX;
        let mut maxx = f32::MIN;
        let mut miny = f32::MAX;
        let mut maxy = f32::MIN;
        for pos in m.intersections.iter().map(|i| i.1.pos) {
            minx = minx.min(pos.x);
            maxx = maxx.max(pos.x);
            miny = miny.min(pos.y);
            maxy = maxy.max(pos.y);
        }
        let w = maxx - minx;
        let h = maxy - miny;

        let mut start = DispatchOne::new(LaneKind::Driving);
        let mut i = 0;
        println!(
            "query empty: {}",
            bench(|| {
                i += 1;
                start.query(
                    &m,
                    DispatchKind::SmallTruck,
                    DispatchQueryTarget::Pos(Vec3::new(
                        minx + w * rand2(i as f32, 12.0),
                        miny + h * rand2(i as f32, 11.0),
                        0.0,
                    )),
                )
            })
        );

        for i in 0..100 {
            start.register(
                mk_ent((1 << 32) + i),
                &m,
                Vec3::new(
                    minx + w * rand2(i as f32, 2.0),
                    miny + h * rand2(i as f32, 1.0),
                    0.0,
                ),
            );
        }

        let mut i = 0;
        println!(
            "query 100: {}",
            bench(|| {
                i += 1;
                start.query(
                    &m,
                    DispatchKind::SmallTruck,
                    DispatchQueryTarget::Pos(Vec3::new(
                        minx + w * rand2(i as f32, 12.0),
                        miny + h * rand2(i as f32, 11.0),
                        0.0,
                    )),
                )
            })
        );

        for i in 100..1000 {
            start.register(
                mk_ent((1 << 32) + i),
                &m,
                Vec3::new(
                    minx + w * rand2(i as f32, 2.0),
                    miny + h * rand2(i as f32, 1.0),
                    0.0,
                ),
            );
        }

        let mut i = 0;
        println!(
            "query 1000: {}",
            bench(|| {
                i += 1;
                start.query(
                    &m,
                    DispatchKind::SmallTruck,
                    DispatchQueryTarget::Pos(Vec3::new(
                        minx + w * rand2(i as f32, 12.0),
                        miny + h * rand2(i as f32, 11.0),
                        0.0,
                    )),
                )
            })
        );

        for i in 1000..10000 {
            start.register(
                mk_ent((1 << 32) + i),
                &m,
                Vec3::new(
                    minx + w * rand2(i as f32, 2.0),
                    miny + h * rand2(i as f32, 1.0),
                    0.0,
                ),
            );
        }

        let mut i = 0;
        println!(
            "query 10000: {}",
            bench(|| {
                i += 1;
                start.query(
                    &m,
                    DispatchKind::SmallTruck,
                    DispatchQueryTarget::Pos(Vec3::new(
                        minx + w * rand2(i as f32, 12.0),
                        miny + h * rand2(i as f32, 11.0),
                        0.0,
                    )),
                )
            })
        );
    }
}
