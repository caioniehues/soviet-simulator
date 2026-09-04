//! Strategic route cache (sov-oiu).
//!
//! A separately-serialized contraction-hierarchy overlay over the driving-lane
//! graph. It ranks candidate destinations and seeds candidate lane sequences
//! ONLY. It never replaces authoritative routing and never weakens
//! deterministic tie-breaking:
//!
//! - Authoritative lane routing (`CarPath` in `pathfinding.rs`) is tick-seeded:
//!   each outgoing lane costs `length / speed_limit` PLUS a deterministic
//!   `randu` jitter seeded on `hash(start_lane, tick)`. A hierarchy over static
//!   weights cannot reproduce that, so the cache stores STATIC weights only
//!   (no tick input anywhere in this module) and ties canonicalize by stable
//!   domain IDs ([`LaneID`] ffi order), never by hierarchy output order.
//! - Every lookup that touches map state takes `&Map` and re-checks the named
//!   revision first: [`StrategicCache::seed_if_current`] and
//!   [`StrategicCache::rank_if_current`] return `None` on a stale cache instead
//!   of serving it. Rebuild with [`StrategicCache::build`] after any topology
//!   or static-weight change.
//! - A seed is not a path. [`StrategicCache::expand_seed`] turns a cached lane
//!   sequence into `Traversable` steps only when every turn is legal in the
//!   CURRENT map, and the result still needs authoritative A* validation
//!   before any vehicle follows it. Nothing here takes `&mut Map`.
//!
//! PILLAR: no stock, dispatch, vehicle, or position changes as a result of a
//! cache lookup. Lookups return owned IDs and costs; the map is untouched.

use crate::map::{LaneID, LaneKind, Map, Traversable, TraverseDirection, TraverseKind, TurnID};
use common::hash_u64;
use fast_paths::{FastGraph, InputGraph};
use serde::{Deserialize, Serialize};
use slotmapd::Key;
use std::collections::BTreeMap;

/// Scale from seconds of static travel time to integer hierarchy weights.
pub const STRATEGIC_WEIGHT_SCALE: f32 = 1_000_000.0;

/// Static travel-time weight for one lane: `length / speed_limit`.
///
/// This is the tick-independent part of the authoritative A* lane cost. The
/// tick-seeded `randu` jitter is deliberately NOT included: reproducing it
/// would weaken deterministic tie-breaking, so the hierarchy never sees it.
pub fn static_weight(length: f32, speed_limit: f32) -> u32 {
    ((length / speed_limit.max(f32::EPSILON)) * STRATEGIC_WEIGHT_SCALE).max(1.0) as u32
}

/// Named map revision the cache was built from.
///
/// The name is a fingerprint over the stable lane/turn mapping plus static
/// weights: every driving lane's ([`LaneID`], src/dst intersections, length,
/// speed limit) and every legal turn's (parent intersection, src lane, dst
/// lane). Any topology edit (add/remove road, intersection rebuild) or static
/// weight change (speed limit, geometry) changes the name, so a stale cache
/// can never compare equal to a changed map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StrategicRevision(pub u64);

impl StrategicRevision {
    /// Names the current topology + static weights of `map`.
    pub fn of(map: &Map) -> Self {
        let mut lanes: Vec<LaneID> = map
            .lanes
            .iter()
            .filter_map(|(id, lane)| (lane.kind == LaneKind::Driving).then_some(id))
            .collect();
        lanes.sort();

        let mut fingerprint: Vec<(u64, u64, u64, u32, u32)> =
            Vec::with_capacity(lanes.len());
        let mut turns: Vec<(u64, u64, u64)> = Vec::new();
        for &lane_id in &lanes {
            let lane = &map.lanes[lane_id];
            fingerprint.push((
                lane_id.data().as_ffi(),
                lane.src.as_ffi(),
                lane.dst.as_ffi(),
                lane.points.length().to_bits(),
                lane.speed_limit.to_bits(),
            ));
            if let Some(inter) = map.intersections.get(lane.dst) {
                let mut dsts: Vec<TurnID> = inter
                    .turns_from(lane_id)
                    .map(|(turn, _)| turn)
                    .collect();
                dsts.sort();
                for turn in dsts {
                    turns.push((
                        turn.parent.as_ffi(),
                        turn.src.data().as_ffi(),
                        turn.dst.data().as_ffi(),
                    ));
                }
            }
        }
        Self(hash_u64((fingerprint, turns)))
    }
}

/// Separately-serialized form of the cache: stable lane mapping plus hierarchy
/// input edges. The prepared [`FastGraph`] is deliberately NOT serialized;
/// call [`StrategicSnapshot::materialize`] to re-prepare it deterministically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategicSnapshot {
    /// Named revision this snapshot was built from (see [`StrategicRevision`]).
    pub revision: u64,
    /// Lane per hierarchy node, in node order. Position IS the node id.
    pub lanes: Vec<LaneID>,
    /// Deduplicated `(from_node, to_node, static_weight)` edges, sorted.
    pub edges: Vec<(u32, u32, u32)>,
}

impl StrategicSnapshot {
    /// Re-prepares the hierarchy from the serialized mapping + edges.
    ///
    /// Returns `None` when the snapshot is corrupt (empty lane set or an edge
    /// pointing outside the lane mapping). Topology drift is NOT checked here;
    /// compare [`StrategicCache::is_current`] against the live map instead.
    pub fn materialize(&self) -> Option<StrategicCache> {
        if self.lanes.is_empty() {
            return None;
        }
        let n = self.lanes.len();
        let mut input = InputGraph::new();
        for &(from, to, weight) in &self.edges {
            if (from as usize) >= n || (to as usize) >= n {
                return None;
            }
            input.add_edge(from as usize, to as usize, weight as usize);
        }
        input.freeze();
        let index: BTreeMap<LaneID, u32> = self
            .lanes
            .iter()
            .copied()
            .enumerate()
            .map(|(node, lane)| (lane, node as u32))
            .collect();
        if index.len() != n {
            return None;
        }
        Some(StrategicCache {
            graph: fast_paths::prepare(&input),
            lanes: self.lanes.clone(),
            index,
            revision: StrategicRevision(self.revision),
            edges: self.edges.clone(),
        })
    }
}

/// Prepared strategic cache. Rank/seed only; the authoritative A* decides.
pub struct StrategicCache {
    graph: FastGraph,
    lanes: Vec<LaneID>,
    index: BTreeMap<LaneID, u32>,
    revision: StrategicRevision,
    /// Deduplicated `(from_node, to_node, static_weight)` edges, sorted.
    /// Captured at build time so [`StrategicCache::snapshot`] serializes the
    /// exact prepared inputs without re-walking the map.
    edges: Vec<(u32, u32, u32)>,
}
impl StrategicCache {
    /// Builds the cache from the current driving lanes + legal turns.
    ///
    /// Lane iteration is sorted by [`LaneID`] and edges are deduplicated
    /// (minimum static weight wins, matching the hierarchy's duplicate-edge
    /// rule) so identical maps always prepare identical caches.
    pub fn build(map: &Map) -> Self {
        let mut lane_ids: Vec<LaneID> = map
            .lanes
            .iter()
            .filter_map(|(id, lane)| (lane.kind == LaneKind::Driving).then_some(id))
            .collect();
        lane_ids.sort();

        let index: BTreeMap<LaneID, u32> = lane_ids
            .iter()
            .copied()
            .enumerate()
            .map(|(node, lane)| (lane, node as u32))
            .collect();

        // (from_node, to_node) -> min static weight, so parallel legal turns
        // between the same lane pair cannot depend on turn iteration order.
        let mut edges: BTreeMap<(u32, u32), u32> = BTreeMap::new();
        for &lane_id in &lane_ids {
            let lane = &map.lanes[lane_id];
            let Some(&source) = index.get(&lane_id) else {
                continue;
            };
            let Some(inter) = map.intersections.get(lane.dst) else {
                continue;
            };
            let mut turns: Vec<TurnID> =
                inter.turns_from(lane_id).map(|(turn, _)| turn).collect();
            turns.sort();
            for turn in turns {
                let Some(next) = map.lanes.get(turn.dst) else {
                    continue;
                };
                if next.kind != LaneKind::Driving {
                    continue;
                }
                let Some(&target) = index.get(&next.id) else {
                    continue;
                };
                let weight = static_weight(next.points.length(), next.speed_limit);
                edges
                    .entry((source, target))
                    .and_modify(|w| *w = (*w).min(weight))
                    .or_insert(weight);
            }
        }

        let mut input = InputGraph::new();
        let mut edge_list = Vec::with_capacity(edges.len());
        for ((from, to), weight) in &edges {
            input.add_edge(*from as usize, *to as usize, *weight as usize);
            edge_list.push((*from, *to, *weight));
        }
        input.freeze();

        Self {
            graph: fast_paths::prepare(&input),
            lanes: lane_ids,
            index,
            revision: StrategicRevision::of(map),
            edges: edge_list,
        }
    }

    /// Named revision this cache was built from.
    pub fn revision(&self) -> StrategicRevision {
        self.revision
    }

    /// True when the live map still names the revision this cache was built
    /// from. Any topology or static-weight change flips this to false.
    pub fn is_current(&self, map: &Map) -> bool {
        StrategicRevision::of(map) == self.revision
    }

    /// Separately-serialized form (stable mapping + edges, no prepared graph).
    pub fn snapshot(&self) -> StrategicSnapshot {
        StrategicSnapshot {
            revision: self.revision.0,
            lanes: self.lanes.clone(),
            edges: self.edges.clone(),
        }
    }

    /// Cached static cost between two lanes, or `None` when either lane is
    /// unknown or disconnected. Never tick-seeded; ties are the caller's to
    /// canonicalize by stable domain IDs (see [`StrategicCache::rank`]).
    pub fn cached_cost(&self, start: LaneID, end: LaneID) -> Option<u64> {
        let source = *self.index.get(&start)?;
        let target = *self.index.get(&end)?;
        let path = fast_paths::calc_path(&self.graph, source as usize, target as usize)?;
        if !path.is_found() || path.get_nodes().is_empty() {
            return None;
        }
        Some(path.get_weight() as u64)
    }

    /// Ranks `candidates` from `start` by cached static cost, ascending.
    /// Ties (including mutually-unreachable pairs) break by stable [`LaneID`]
    /// ffi order, never by hierarchy output order.
    pub fn rank(&self, start: LaneID, candidates: &[LaneID]) -> Vec<LaneID> {
        let mut ordered: Vec<LaneID> = candidates.to_vec();
        ordered.sort();
        ordered.sort_by_key(|lane| self.cached_cost(start, *lane));
        ordered
    }

    /// [`StrategicCache::rank`], but refuses to serve a stale cache: returns
    /// `None` when [`StrategicCache::is_current`] fails for `map`.
    pub fn rank_if_current(
        &self,
        map: &Map,
        start: LaneID,
        candidates: &[LaneID],
    ) -> Option<Vec<LaneID>> {
        if !self.is_current(map) {
            return None;
        }
        Some(self.rank(start, candidates))
    }

    /// Raw cached lane seed from `start` to `end`. A hint only: expand it with
    /// [`StrategicCache::expand_seed`] and validate the result with the
    /// authoritative A* before any vehicle follows it.
    pub fn seed(&self, start: LaneID, end: LaneID) -> Option<Vec<LaneID>> {
        let source = *self.index.get(&start)?;
        let target = *self.index.get(&end)?;
        let path = fast_paths::calc_path(&self.graph, source as usize, target as usize)?;
        if !path.is_found() {
            return None;
        }
        Some(
            path.get_nodes()
                .iter()
                .map(|&node| self.lanes[node as usize])
                .collect(),
        )
    }

    /// [`StrategicCache::seed`], but refuses to serve a stale cache: returns
    /// `None` when [`StrategicCache::is_current`] fails for `map`.
    pub fn seed_if_current(
        &self,
        map: &Map,
        start: LaneID,
        end: LaneID,
    ) -> Option<Vec<LaneID>> {
        if !self.is_current(map) {
            return None;
        }
        self.seed(start, end)
    }

    /// Expands a cached lane seed into `Traversable` steps against the CURRENT
    /// map, mirroring the authoritative A* reconstruction shape
    /// (`start`, then alternating legal turn + lane).
    ///
    /// Returns `None` when any step is illegal now (lane gone, turn illegal,
    /// lane unauthorized): a stale seed never becomes a path. A `Some` result
    /// is physically valid but still NOT authoritative — run the A* for the
    /// final path.
    pub fn expand_seed(
        map: &Map,
        start: Traversable,
        seed: &[LaneID],
    ) -> Option<Vec<Traversable>> {
        let (first, rest) = seed.split_first()?;
        if *first != start.destination_lane() {
            return None;
        }
        let mut path = Vec::with_capacity(seed.len() * 2);
        path.push(start);
        let mut last_id = *first;
        for &lane in rest {
            let next = map.lanes.get(lane)?;
            if !matches!(next.kind, LaneKind::Driving | LaneKind::Bus) {
                return None;
            }
            let inter = map.intersections.get(next.src)?;
            let legal = inter
                .turns_from(last_id)
                .any(|(turn, _)| turn.dst == lane);
            if !legal {
                return None;
            }
            let id = TurnID::new(inter.id, last_id, lane, false);
            path.push(Traversable::new(
                TraverseKind::Turn(id),
                TraverseDirection::Forward,
            ));
            path.push(Traversable::new(
                TraverseKind::Lane(lane),
                TraverseDirection::Forward,
            ));
            last_id = lane;
        }
        Some(path)
    }
}
