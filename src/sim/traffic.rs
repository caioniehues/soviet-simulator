//! B4.2 congestion (ticket #40, spec/traffic.md): the entire congestion
//! machine is local, CS1's confirmed shape — vehicles stamp load onto the
//! segment they occupy, a low-pass filter turns the load into a 0–100
//! density scalar, and routing reads that scalar as a cost multiplier.
//! Nothing computes network-wide flow; a jam is information, never an event
//! to garbage-collect.

use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use super::clock::TickIndex;
use super::pathfinding::PathService;
use super::roads::{LaneDir, RoadSegment};
use super::stages::{SimStage, SimTick};
use super::vehicles::ActiveVehicle;

/// Density remeasure cadence, ticks (medium-frequency band, matching the
/// dispatcher's matching interval).
pub const DENSITY_INTERVAL: u32 = 15;

/// Road metres one vehicle effectively occupies when computing a lane's
/// capacity (vehicle length + headway).
pub const VEHICLE_FOOTPRINT_M: f32 = 8.0;

/// Low-pass step: density moves at most this much toward its target per
/// remeasure (CS1 `RoadBaseAI.SimulationStep` moves ±5 per step).
pub const DENSITY_STEP: u8 = 5;

/// Live congestion state per road segment. Absent component = never
/// trafficked (density 0). `density` is 0–100; `blocked` flags a segment
/// whose instantaneous load exceeded its physical lane capacity.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SegmentTraffic {
    pub density: u8,
    pub blocked: bool,
}

impl SegmentTraffic {
    /// Routing cost multiplier: 1.0 (empty) → 2.0 (saturated), CS1's
    /// `(1000 + density×10)/1000` curve.
    pub fn cost_multiplier(self) -> f32 {
        1.0 + self.density as f32 / 100.0
    }
}

/// Marker for the occupancy build inside `MovementAndTransfers`; movement
/// systems that respect car-following order themselves `.after(LanePrep)`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LanePrep;

/// Who is where on each directed lane, rebuilt at the top of every movement
/// pass (B4.3). Jams are lane space reservation: a follower reads the gap to
/// its leader and refuses to enter it — no queue object, no scheduler.
#[derive(Resource, Default)]
pub struct LaneOccupancy {
    map: HashMap<(Entity, LaneDir), Vec<(Entity, f32)>>,
}

impl LaneOccupancy {
    /// Metres `me` may advance along its lane before violating the leader's
    /// reserved space. `INFINITY` with no leader ahead.
    pub fn gap_ahead(&self, segment: Entity, dir: LaneDir, s: f32, me: Entity) -> f32 {
        self.map
            .get(&(segment, dir))
            .into_iter()
            .flatten()
            .filter(|(e, os)| *e != me && *os > s)
            .map(|(_, os)| (os - s - VEHICLE_FOOTPRINT_M).max(0.0))
            .fold(f32::INFINITY, f32::min)
    }

    /// Whether the mouth of the lane is clear enough to enter.
    pub fn entry_free(&self, segment: Entity, dir: LaneDir, me: Entity) -> bool {
        self.map
            .get(&(segment, dir))
            .into_iter()
            .flatten()
            .all(|(e, os)| *e == me || *os >= VEHICLE_FOOTPRINT_M)
    }
}

fn build_lane_occupancy(mut occupancy: ResMut<LaneOccupancy>, vehicles: Query<(Entity, &ActiveVehicle)>) {
    occupancy.map.clear();
    for (entity, vehicle) in &vehicles {
        if let Some(leg) = vehicle.route.get(vehicle.leg) {
            occupancy
                .map
                .entry((leg.segment, leg.dir))
                .or_default()
                .push((entity, vehicle.s));
        }
    }
}

pub struct TrafficSimPlugin;

impl Plugin for TrafficSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LaneOccupancy>()
            .add_systems(
                SimTick,
                build_lane_occupancy
                    .in_set(SimStage::MovementAndTransfers)
                    .in_set(LanePrep),
            )
            .add_systems(
                SimTick,
                measure_density.in_set(SimStage::AccountingAndCausalHistory),
            );
    }
}

/// Count vehicles per occupied segment, low-pass the density toward the
/// measured load, and bump the pathfinding snapshot version when anything
/// moved so the next routing frame prices the congestion in.
fn measure_density(
    mut commands: Commands,
    tick: Res<TickIndex>,
    mut svc: ResMut<PathService>,
    vehicles: Query<&ActiveVehicle>,
    mut segments: Query<(Entity, &RoadSegment, Option<&mut SegmentTraffic>)>,
) {
    if !tick.0.is_multiple_of(DENSITY_INTERVAL) {
        return;
    }
    let mut load: HashMap<Entity, u32> = HashMap::new();
    for vehicle in &vehicles {
        if let Some(leg) = vehicle.route.get(vehicle.leg) {
            *load.entry(leg.segment).or_default() += 1;
        }
    }
    let mut changed = false;
    for (entity, segment, traffic) in &mut segments {
        let count = load.get(&entity).copied().unwrap_or(0);
        let capacity = ((segment.length * segment.lanes.len() as f32 / VEHICLE_FOOTPRINT_M)
            .floor() as u32)
            .max(1);
        let target = ((count * 100) / capacity).min(100) as u8;
        let blocked = count > capacity;
        match traffic {
            Some(mut t) => {
                let stepped = if target > t.density {
                    t.density.saturating_add(DENSITY_STEP).min(target)
                } else {
                    t.density.saturating_sub(DENSITY_STEP).max(target)
                };
                if stepped != t.density || blocked != t.blocked {
                    t.density = stepped;
                    t.blocked = blocked;
                    changed = true;
                }
            }
            None if target > 0 || blocked => {
                commands.entity(entity).insert(SegmentTraffic {
                    density: target.min(DENSITY_STEP),
                    blocked,
                });
                changed = true;
            }
            None => {}
        }
    }
    if changed {
        svc.congestion_version += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::pathfinding::{CostProfile, PathService, PathfindingSimPlugin};
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadNode, RoadSimPlugin};
    use super::super::vehicles::RouteLeg;
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((
            SimPlugin,
            RoadSimPlugin,
            PathfindingSimPlugin,
            TrafficSimPlugin,
        ));
        a
    }

    fn ticks(app: &mut App, n: u32) {
        for _ in 0..n {
            app.world_mut()
                .resource_mut::<Time>()
                .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
            app.update();
        }
    }

    fn place(app: &mut App, from: Vec3, to: Vec3) {
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from,
                to,
                class: RoadClass::Dirt,
            });
    }

    fn segment_between(app: &mut App, ax: f32, bx: f32) -> Entity {
        let world = app.world_mut();
        let mut nodes = world.query::<&RoadNode>();
        let pos_of = |world: &mut World, e: Entity| world.get::<RoadNode>(e).unwrap().pos.x;
        let _ = &mut nodes;
        let mut q = world.query::<(Entity, &RoadSegment)>();
        let list: Vec<(Entity, Entity, Entity)> =
            q.iter(world).map(|(e, s)| (e, s.a, s.b)).collect();
        for (e, a, b) in list {
            let (xa, xb) = (pos_of(app.world_mut(), a), pos_of(app.world_mut(), b));
            if (xa.min(xb) - ax.min(bx)).abs() < 1.0 && (xa.max(xb) - ax.max(bx)).abs() < 1.0 {
                return e;
            }
        }
        panic!("no segment between x={ax} and x={bx}");
    }

    /// Park `n` fake pawns on `segment` so the density pass sees load.
    fn park_vehicles(app: &mut App, segment: Entity, n: u32) {
        use super::super::roads::LaneDir;
        for _ in 0..n {
            let mut v = ActiveVehicle::at(Vec3::ZERO);
            v.route = vec![RouteLeg {
                segment,
                dir: LaneDir::Forward,
            }];
            app.world_mut().spawn(v);
        }
    }

    #[test]
    fn density_rises_under_load_and_decays_after() {
        let mut app = app();
        place(&mut app, Vec3::ZERO, Vec3::new(100.0, 0.0, 0.0));
        ticks(&mut app, 1);
        let seg = segment_between(&mut app, 0.0, 100.0);
        // capacity = 100 m × 2 lanes / 8 m = 25; 20 vehicles → target 80
        park_vehicles(&mut app, seg, 20);
        ticks(&mut app, DENSITY_INTERVAL * 4 + 1);
        let d = app.world().get::<SegmentTraffic>(seg).unwrap().density;
        assert!(d >= 15, "low-passed density climbing, got {d}");
        assert!(d <= 80, "density must not overshoot its target, got {d}");
        // remove the load; density must decay by steps, not snap to 0
        let world = app.world_mut();
        let pawns: Vec<Entity> = world
            .query_filtered::<Entity, With<ActiveVehicle>>()
            .iter(world)
            .collect();
        for p in pawns {
            world.despawn(p);
        }
        let before = app.world().get::<SegmentTraffic>(seg).unwrap().density;
        ticks(&mut app, DENSITY_INTERVAL + 1);
        let after = app.world().get::<SegmentTraffic>(seg).unwrap().density;
        assert!(after < before && after >= before.saturating_sub(DENSITY_STEP));
    }

    #[test]
    fn overload_flags_blocked() {
        let mut app = app();
        place(&mut app, Vec3::ZERO, Vec3::new(40.0, 0.0, 0.0));
        ticks(&mut app, 1);
        let seg = segment_between(&mut app, 0.0, 40.0);
        // capacity = 40 × 2 / 8 = 10; 12 vehicles exceed it
        park_vehicles(&mut app, seg, 12);
        ticks(&mut app, DENSITY_INTERVAL + 1);
        let t = app.world().get::<SegmentTraffic>(seg).unwrap();
        assert!(t.blocked, "overloaded segment must flag Blocked");
    }

    #[test]
    fn congested_leg_prices_into_the_route() {
        let mut app = app();
        // Two equal-length parallel routes 0 → 200: via z=0 midpoint and via
        // z=60 detour of identical geometry.
        place(&mut app, Vec3::ZERO, Vec3::new(100.0, 0.0, 30.0));
        place(&mut app, Vec3::new(100.0, 0.0, 30.0), Vec3::new(200.0, 0.0, 0.0));
        place(&mut app, Vec3::ZERO, Vec3::new(100.0, 0.0, -30.0));
        place(&mut app, Vec3::new(100.0, 0.0, -30.0), Vec3::new(200.0, 0.0, 0.0));
        ticks(&mut app, 1);
        // saturate the north pair
        let world = app.world_mut();
        let mut q = world.query::<(Entity, &RoadSegment)>();
        let segs: Vec<Entity> = q.iter(world).map(|(e, _)| e).collect();
        let north: Vec<Entity> = segs
            .iter()
            .copied()
            .filter(|&e| {
                let s = world.get::<RoadSegment>(e).unwrap();
                let za = world.get::<RoadNode>(s.a).unwrap().pos.z;
                let zb = world.get::<RoadNode>(s.b).unwrap().pos.z;
                za.max(zb) > 1.0
            })
            .collect();
        assert_eq!(north.len(), 2);
        for seg in &north {
            park_vehicles(&mut app, *seg, 30);
        }
        // let the low-pass climb to full price
        ticks(&mut app, DENSITY_INTERVAL * 25);
        let world = app.world_mut();
        let mut nodes = world.query::<(Entity, &RoadNode)>();
        let (mut start, mut goal) = (None, None);
        for (e, n) in nodes.iter(world) {
            if n.pos.distance(Vec3::ZERO) < 1.0 {
                start = Some(e);
            }
            if n.pos.distance(Vec3::new(200.0, 0.0, 0.0)) < 1.0 {
                goal = Some(e);
            }
        }
        let (start, goal) = (start.unwrap(), goal.unwrap());
        let svc = app.world().resource::<PathService>();
        // With the north pair at ~2× cost, every seed must route south.
        for seed in 0..8 {
            let route = svc
                .route_now(start, goal, CostProfile::Vehicle, seed)
                .expect("route exists");
            for leg in &route {
                assert!(
                    !north.contains(&leg.segment),
                    "seed {seed} routed through the congested pair"
                );
            }
        }
    }
}
