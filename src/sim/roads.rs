//! Authoritative road graph (spec/roads.md, M1 charter Q10): player-drawn
//! centerlines compile into the one unified lane graph — node/segment/per-lane
//! buffer. The lane network *is* the routing graph; there is no separate
//! abstract routing layer. M1 segments are straight; curves arrive in B4.

use bevy::prelude::*;

use super::buildings::Building;
use super::resources::{Inventory, ResourceKind};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};

pub const SNAP_RADIUS: f32 = 6.0;

/// Paving is a physical build: gravel per metre of centreline, drawn from
/// building yards whose centre lies within `GRAVEL_SUPPLY_RADIUS` of the new
/// segment. Dirt roads stay free (graded earth). Full phased construction
/// arrives in B6; this is the M1 "single-quantum" stub the charter names.
pub const PAVED_GRAVEL_PER_METRE: f32 = 0.05;
pub const GRAVEL_SUPPLY_RADIUS: f32 = 80.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoadClass {
    Dirt,
    Paved,
}

impl RoadClass {
    pub fn width(self) -> f32 {
        match self {
            RoadClass::Dirt => 6.0,
            RoadClass::Paved => 8.0,
        }
    }
    /// Road-class factor in `effective speed = vehicle × road-class × terrain`.
    pub fn speed_modifier(self) -> f32 {
        match self {
            RoadClass::Dirt => 0.55,
            RoadClass::Paved => 1.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NodeId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SegmentId(pub u64);

/// Intersection / endpoint. Adjacency lives here (≤8 attached segments).
#[derive(Component, Debug)]
pub struct RoadNode {
    pub id: NodeId,
    pub pos: Vec3,
    pub segments: Vec<Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LaneDir {
    /// travels a → b
    Forward,
    /// travels b → a
    Backward,
}

#[derive(Clone, Copy, Debug)]
pub struct Lane {
    pub dir: LaneDir,
    /// lateral offset from the centerline, metres (+ = right of travel dir)
    pub offset: f32,
    pub speed_modifier: f32,
}

/// Edge between two nodes with its compiled lane buffer.
#[derive(Component, Debug)]
pub struct RoadSegment {
    pub id: SegmentId,
    pub a: Entity,
    pub b: Entity,
    pub class: RoadClass,
    pub length: f32,
    /// Bumped on every recompile; cached routes compare it (spec/roads.md).
    pub modification_index: u32,
    pub lanes: Vec<Lane>,
}

impl RoadSegment {
    /// Rebuild the derived geometry (length + lane buffer) from endpoint
    /// positions. Shared by the dirty-segment pass and the save loader, so
    /// lanes are never serialized — they are always derived. Does not bump
    /// `modification_index`; that is the compile pass's business.
    pub fn recompile(&mut self, a_pos: Vec3, b_pos: Vec3) {
        self.length = a_pos.distance(b_pos);
        let half = self.class.width() * 0.25;
        let speed = self.class.speed_modifier();
        self.lanes = vec![
            Lane {
                dir: LaneDir::Forward,
                offset: half,
                speed_modifier: speed,
            },
            Lane {
                dir: LaneDir::Backward,
                offset: -half,
                speed_modifier: speed,
            },
        ];
    }
}

/// Marks a segment whose lane buffer must be recompiled this tick.
#[derive(Component)]
pub struct DirtySegment;

/// Edits from the tool side; drained inside the ApplyCommands stage so
/// structural change happens only at the named barrier.
#[derive(Resource, Default)]
pub struct RoadEditQueue(pub Vec<RoadEdit>);

#[derive(Clone, Copy, Debug)]
pub enum RoadEdit {
    /// Both endpoints snap to an existing node within `SNAP_RADIUS`, else a
    /// new node is created at the position.
    Place {
        from: Vec3,
        to: Vec3,
        class: RoadClass,
    },
    /// Remove the segment nearest to `pos` (within `SNAP_RADIUS` of its line).
    RemoveNear { pos: Vec3 },
    /// Re-place the most recently cut segment (paved pays gravel again).
    RebuildLast,
}

/// The most recently cut segment, so the rebuild hotkey can restore it.
#[derive(Resource, Default, Clone, Copy)]
pub struct LastCut(pub Option<(Vec3, Vec3, RoadClass)>);

/// Why the last paved placement failed, for the HUD. Cleared by the next
/// successful placement.
#[derive(Resource, Default, Clone, Copy)]
pub struct RoadBuildFeedback(pub Option<GravelShortfall>);

#[derive(Clone, Copy, Debug)]
pub struct GravelShortfall {
    pub needed: f32,
    pub available: f32,
}

#[derive(Resource, Default)]
pub struct RoadIds {
    pub next_node: u64,
    pub next_segment: u64,
}

pub struct RoadSimPlugin;

impl Plugin for RoadSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadEditQueue>()
            .init_resource::<RoadIds>()
            .init_resource::<LastCut>()
            .init_resource::<RoadBuildFeedback>()
            .add_systems(
                SimTick,
                (apply_road_edits, compile_dirty_segments)
                    .chain()
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush),
            );
    }
}

fn apply_road_edits(world: &mut World) {
    let edits = std::mem::take(&mut world.resource_mut::<RoadEditQueue>().0);
    for edit in edits {
        match edit {
            RoadEdit::Place { from, to, class } => place_segment(world, from, to, class),
            RoadEdit::RemoveNear { pos } => remove_segment_near(world, pos),
            RoadEdit::RebuildLast => {
                if let Some((from, to, class)) = world.resource::<LastCut>().0 {
                    place_segment(world, from, to, class);
                }
            }
        }
    }
}

fn snap_or_create_node(world: &mut World, pos: Vec3) -> Entity {
    let mut nodes = world.query::<(Entity, &RoadNode)>();
    let snapped = nodes
        .iter(world)
        .map(|(e, n)| (e, n.pos.distance_squared(pos)))
        .filter(|(_, d2)| *d2 <= SNAP_RADIUS * SNAP_RADIUS)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e);
    snapped.unwrap_or_else(|| {
        let id = {
            let mut ids = world.resource_mut::<RoadIds>();
            ids.next_node += 1;
            NodeId(ids.next_node)
        };
        world
            .spawn(RoadNode {
                id,
                pos,
                segments: Vec::new(),
            })
            .id()
    })
}

/// Where `pos` would land after node snapping, without mutating the graph —
/// the cost and duplicate checks must run before any node is created.
fn snapped_node(world: &mut World, pos: Vec3) -> (Option<Entity>, Vec3) {
    let mut nodes = world.query::<(Entity, &RoadNode)>();
    nodes
        .iter(world)
        .map(|(e, n)| (e, n.pos, n.pos.distance_squared(pos)))
        .filter(|(.., d2)| *d2 <= SNAP_RADIUS * SNAP_RADIUS)
        .min_by(|a, b| a.2.total_cmp(&b.2))
        .map_or((None, pos), |(e, p, _)| (Some(e), p))
}

/// Drain `cost` tonnes of gravel from yards within `GRAVEL_SUPPLY_RADIUS` of
/// the segment, nearest yard first; all-or-nothing. Returns false (and records
/// the shortfall) when the delivered stock nearby cannot cover the build.
fn pay_gravel(world: &mut World, a: Vec3, b: Vec3, cost: f32) -> bool {
    let mut yards = world.query::<(Entity, &Building, &Inventory)>();
    let mut candidates: Vec<(f32, u64, Entity, f32)> = yards
        .iter(world)
        .map(|(e, building, inventory)| {
            let d = point_to_segment_distance(building.pos, a, b);
            (d, building.id.0, e, inventory.amount(ResourceKind::Gravel))
        })
        .filter(|(d, .., gravel)| *d <= GRAVEL_SUPPLY_RADIUS && *gravel > 0.0)
        .collect();
    candidates.sort_by(|x, y| x.0.total_cmp(&y.0).then(x.1.cmp(&y.1)));
    let available: f32 = candidates.iter().map(|(.., g)| g).sum();
    if available < cost {
        world.resource_mut::<RoadBuildFeedback>().0 = Some(GravelShortfall {
            needed: cost,
            available,
        });
        return false;
    }
    let mut owed = cost;
    for (_, _, entity, _) in candidates {
        if owed <= 0.0 {
            break;
        }
        owed -= world
            .get_mut::<Inventory>(entity)
            .unwrap()
            .take(ResourceKind::Gravel, owed);
    }
    true
}

fn place_segment(world: &mut World, from: Vec3, to: Vec3, class: RoadClass) {
    // Read-only pre-checks: both would-snap endpoints resolving to the same
    // node, or to two nodes already joined by a segment, reject the placement
    // before any node is created or gravel is paid.
    let (node_a, pa) = snapped_node(world, from);
    let (node_b, pb) = snapped_node(world, to);
    if let (Some(na), Some(nb)) = (node_a, node_b) {
        if na == nb {
            return;
        }
        let a_segments = world.get::<RoadNode>(na).unwrap().segments.clone();
        for &seg in &a_segments {
            let s = world.get::<RoadSegment>(seg).unwrap();
            if (s.a == na && s.b == nb) || (s.a == nb && s.b == na) {
                return;
            }
        }
    }
    if class == RoadClass::Paved {
        let cost = pa.distance(pb) * PAVED_GRAVEL_PER_METRE;
        if !pay_gravel(world, pa, pb, cost) {
            return;
        }
    }
    world.resource_mut::<RoadBuildFeedback>().0 = None;
    let a = node_a.unwrap_or_else(|| snap_or_create_node(world, from));
    let b = node_b.unwrap_or_else(|| snap_or_create_node(world, to));
    if a == b {
        return;
    }
    let id = {
        let mut ids = world.resource_mut::<RoadIds>();
        ids.next_segment += 1;
        SegmentId(ids.next_segment)
    };
    let segment = world
        .spawn((
            RoadSegment {
                id,
                a,
                b,
                class,
                length: 0.0,
                modification_index: 0,
                lanes: Vec::new(),
            },
            DirtySegment,
        ))
        .id();
    for node in [a, b] {
        world
            .get_mut::<RoadNode>(node)
            .unwrap()
            .segments
            .push(segment);
    }
    mark_ring_dirty(world, &[a, b]);
}

fn remove_segment_near(world: &mut World, pos: Vec3) {
    let mut segments = world.query::<(Entity, &RoadSegment)>();
    let hit = segments
        .iter(world)
        .filter_map(|(e, s)| {
            let (pa, pb) = endpoint_positions(world, s)?;
            Some((e, point_to_segment_distance(pos, pa, pb)))
        })
        .filter(|(_, d)| *d <= SNAP_RADIUS)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e);
    let Some(segment) = hit else { return };
    let (a, b, class) = {
        let s = world.get::<RoadSegment>(segment).unwrap();
        (s.a, s.b, s.class)
    };
    if let (Some(na), Some(nb)) = (world.get::<RoadNode>(a), world.get::<RoadNode>(b)) {
        let cut = Some((na.pos, nb.pos, class));
        world.resource_mut::<LastCut>().0 = cut;
    }
    world.despawn(segment);
    for node in [a, b] {
        let orphaned = {
            let mut n = world.get_mut::<RoadNode>(node).unwrap();
            n.segments.retain(|&s| s != segment);
            n.segments.is_empty()
        };
        if orphaned {
            world.despawn(node);
        }
    }
    mark_ring_dirty(world, &[a, b]);
}

/// 1-ring incremental recompile: every surviving segment attached to the
/// touched nodes gets recompiled (its lane fan depends on the junction).
fn mark_ring_dirty(world: &mut World, nodes: &[Entity]) {
    let mut ring = Vec::new();
    for &node in nodes {
        if let Some(n) = world.get::<RoadNode>(node) {
            ring.extend(n.segments.iter().copied());
        }
    }
    for segment in ring {
        world.entity_mut(segment).insert(DirtySegment);
    }
}

fn endpoint_positions(world: &World, s: &RoadSegment) -> Option<(Vec3, Vec3)> {
    Some((
        world.get::<RoadNode>(s.a)?.pos,
        world.get::<RoadNode>(s.b)?.pos,
    ))
}

fn point_to_segment_distance(p: Vec3, a: Vec3, b: Vec3) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    p.distance(a + ab * t)
}

fn compile_dirty_segments(
    mut commands: Commands,
    mut dirty: Query<(Entity, &mut RoadSegment), With<DirtySegment>>,
    nodes: Query<&RoadNode>,
) {
    for (entity, mut segment) in &mut dirty {
        let (Ok(a), Ok(b)) = (nodes.get(segment.a), nodes.get(segment.b)) else {
            continue;
        };
        let (a_pos, b_pos) = (a.pos, b.pos);
        segment.recompile(a_pos, b_pos);
        segment.modification_index = segment.modification_index.wrapping_add(1);
        commands.entity(entity).remove::<DirtySegment>();
    }
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, RoadSimPlugin));
        a
    }

    fn tick(app: &mut App) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
        app.update();
    }

    fn push(app: &mut App, edit: RoadEdit) {
        app.world_mut().resource_mut::<RoadEditQueue>().0.push(edit);
    }

    fn counts(app: &mut App) -> (usize, usize) {
        let nodes = app
            .world_mut()
            .query::<&RoadNode>()
            .iter(app.world())
            .count();
        let segments = app
            .world_mut()
            .query::<&RoadSegment>()
            .iter(app.world())
            .count();
        (nodes, segments)
    }

    #[test]
    fn chain_of_two_segments_snaps_shared_node() {
        let mut app = app();
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            },
        );
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::new(101.0, 0.0, 2.0), // within SNAP_RADIUS of (100,0,0)
                to: Vec3::new(200.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            },
        );
        tick(&mut app);
        assert_eq!(counts(&mut app), (3, 2));
    }

    /// A yard with gravel stock near the origin, for paved-build tests.
    fn stock_gravel(app: &mut App, pos: Vec3, tonnes: f32) -> Entity {
        use super::super::buildings::{BuildingId, BuildingKind};
        let mut inventory = Inventory::new(60.0);
        inventory.add(ResourceKind::Gravel, tonnes);
        app.world_mut()
            .spawn((
                Building {
                    id: BuildingId(999),
                    kind: BuildingKind::Quarry,
                    pos,
                },
                inventory,
            ))
            .id()
    }

    #[test]
    fn compile_fills_lanes_and_bumps_modification_index() {
        let mut app = app();
        stock_gravel(&mut app, Vec3::ZERO, 50.0);
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(60.0, 0.0, 80.0),
                class: RoadClass::Paved,
            },
        );
        tick(&mut app);
        let world = app.world_mut();
        let segment = world.query::<&RoadSegment>().single(world).unwrap();
        assert_eq!(segment.lanes.len(), 2);
        assert!((segment.length - 100.0).abs() < 1e-3);
        assert_eq!(segment.modification_index, 1);
        assert_eq!(segment.lanes[0].speed_modifier, 1.0);
        let none_dirty = world
            .query_filtered::<(), With<DirtySegment>>()
            .iter(world)
            .count();
        assert_eq!(none_dirty, 0);
    }

    #[test]
    fn removal_despawns_orphans_and_recompiles_the_ring() {
        let mut app = app();
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            },
        );
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::new(100.0, 0.0, 0.0),
                to: Vec3::new(200.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            },
        );
        tick(&mut app);
        let world = app.world_mut();
        let first_mod = world
            .query::<&RoadSegment>()
            .iter(world)
            .find(|s| s.id == SegmentId(1))
            .unwrap()
            .modification_index;
        assert_eq!(first_mod, 1);

        // cut the second segment near its middle
        push(
            &mut app,
            RoadEdit::RemoveNear {
                pos: Vec3::new(150.0, 0.0, 0.0),
            },
        );
        tick(&mut app);
        // orphaned far node despawned; shared node survives (still attached)
        assert_eq!(counts(&mut app), (2, 1));
        let world = app.world_mut();
        let survivor = world.query::<&RoadSegment>().single(world).unwrap();
        // 1-ring recompile bumped the surviving neighbor
        assert_eq!(survivor.modification_index, 2);
    }

    #[test]
    fn duplicate_edge_is_rejected() {
        let mut app = app();
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            },
        );
        tick(&mut app);
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::new(1.0, 0.0, 0.0),
                to: Vec3::new(99.0, 0.0, 0.0),
                class: RoadClass::Paved,
            },
        );
        tick(&mut app);
        assert_eq!(counts(&mut app), (2, 1));
    }

    #[test]
    fn paved_build_consumes_delivered_gravel_from_the_nearby_yard() {
        let mut app = app();
        let yard = stock_gravel(&mut app, Vec3::new(0.0, 0.0, 20.0), 50.0);
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Paved,
            },
        );
        tick(&mut app);
        assert_eq!(counts(&mut app), (2, 1));
        let left = app
            .world()
            .get::<Inventory>(yard)
            .unwrap()
            .amount(ResourceKind::Gravel);
        let cost = 100.0 * PAVED_GRAVEL_PER_METRE;
        assert!((left - (50.0 - cost)).abs() < 1e-3, "left = {left}");
    }

    #[test]
    fn paved_build_without_gravel_is_rejected_with_feedback() {
        let mut app = app();
        stock_gravel(&mut app, Vec3::new(0.0, 0.0, 20.0), 1.0); // < 5 t needed
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Paved,
            },
        );
        tick(&mut app);
        assert_eq!(counts(&mut app), (0, 0));
        let shortfall = app.world().resource::<RoadBuildFeedback>().0.unwrap();
        assert!((shortfall.needed - 5.0).abs() < 1e-3);
        assert!((shortfall.available - 1.0).abs() < 1e-3);
        // dirt on the same line still goes down free
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            },
        );
        tick(&mut app);
        assert_eq!(counts(&mut app), (2, 1));
        assert!(app.world().resource::<RoadBuildFeedback>().0.is_none());
    }

    #[test]
    fn rebuild_hotkey_restores_the_last_cut_segment() {
        let mut app = app();
        push(
            &mut app,
            RoadEdit::Place {
                from: Vec3::ZERO,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            },
        );
        tick(&mut app);
        push(
            &mut app,
            RoadEdit::RemoveNear {
                pos: Vec3::new(50.0, 0.0, 0.0),
            },
        );
        tick(&mut app);
        assert_eq!(counts(&mut app), (0, 0));
        push(&mut app, RoadEdit::RebuildLast);
        tick(&mut app);
        let (nodes, segments) = counts(&mut app);
        assert_eq!((nodes, segments), (2, 1));
        let world = app.world_mut();
        let segment = world.query::<&RoadSegment>().single(world).unwrap();
        assert_eq!(segment.class, RoadClass::Dirt);
        assert!((segment.length - 100.0).abs() < 1e-3);
    }
}
