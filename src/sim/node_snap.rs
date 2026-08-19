//! Shared node-snapping arithmetic (ADR 0016). `roads.rs` and `wires.rs`
//! each resolve a click to "the nearest existing node within a radius, or
//! none" and "the nearest edge to a point within a radius, or none" — the
//! same two comparisons run over two different graphs. Only the comparison
//! is shared; who counts as a node, how a miss creates one, what an edit
//! costs and what removal orphans stay in each caller (ADR 0016 rejects a
//! generic edit queue for exactly that reason: the bodies genuinely differ).

use bevy::prelude::*;

/// Nearest of `candidates` to `pos` within `radius`, or `None` if every
/// candidate is out of range (or there are none).
pub fn nearest_node_within(
    candidates: impl Iterator<Item = (Entity, Vec3)>,
    pos: Vec3,
    radius: f32,
) -> Option<Entity> {
    let r2 = radius * radius;
    candidates
        .map(|(e, p)| (e, p.distance_squared(pos)))
        .filter(|(_, d2)| *d2 <= r2)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

/// Perpendicular distance from `p` to the segment `a`–`b`, clamped to the
/// segment's ends.
pub fn point_to_segment_distance(p: Vec3, a: Vec3, b: Vec3) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    p.distance(a + ab * t)
}

/// Nearest of `candidates` (entity, endpoint a, endpoint b) to `pos` by
/// [`point_to_segment_distance`], within `radius`, or `None`.
pub fn nearest_edge_within(
    candidates: impl Iterator<Item = (Entity, Vec3, Vec3)>,
    pos: Vec3,
    radius: f32,
) -> Option<Entity> {
    candidates
        .map(|(e, a, b)| (e, point_to_segment_distance(pos, a, b)))
        .filter(|(_, d)| *d <= radius)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::roads::{RoadClass, RoadEdit, RoadEditQueue, RoadNode, RoadSimPlugin};
    use super::super::wires::{NetKind, WireEdit, WireEditQueue, WirePole, WireSimPlugin};
    use bevy::prelude::*;
    use std::time::Duration;

    /// The drift this extraction exists to prevent: a road and a wire click
    /// at the same offset from an existing node must reach the same
    /// snap-or-create verdict, because both now run `nearest_node_within`
    /// rather than two hand-rolled copies of it that could diverge.
    #[test]
    fn roads_and_wires_resolve_the_same_offset_to_the_same_snap_verdict() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        app.add_plugins((SimPlugin, RoadSimPlugin, WireSimPlugin));

        let origin = Vec3::ZERO;
        // 3.0 sits inside both SNAP_RADIUS (6.0) and POLE_SNAP_RADIUS (4.0):
        // a genuine within-both-radii offset, not a radius-difference case.
        let near = Vec3::new(3.0, 0.0, 0.0);

        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: origin,
                to: Vec3::new(100.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        app.world_mut()
            .resource_mut::<WireEditQueue>()
            .0
            .push(WireEdit::Place {
                from: origin,
                to: Vec3::new(100.0, 0.0, 0.0),
                kind: NetKind::Power,
            });
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
        app.update();

        // Second click near (not on) the origin node/pole for each graph.
        app.world_mut()
            .resource_mut::<RoadEditQueue>()
            .0
            .push(RoadEdit::Place {
                from: near,
                to: Vec3::new(200.0, 0.0, 0.0),
                class: RoadClass::Dirt,
            });
        app.world_mut()
            .resource_mut::<WireEditQueue>()
            .0
            .push(WireEdit::Place {
                from: near,
                to: Vec3::new(200.0, 0.0, 0.0),
                kind: NetKind::Power,
            });
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f64(1.0 / 60.0 + 1e-9));
        app.update();

        let world = app.world_mut();
        let road_nodes_at_origin = world
            .query::<&RoadNode>()
            .iter(world)
            .filter(|n| n.pos.distance(origin) < 1e-3)
            .count();
        let poles_at_origin = world
            .query::<&WirePole>()
            .iter(world)
            .filter(|p| p.pos.distance(origin) < 1e-3)
            .count();
        // Both graphs joined the existing node/pole rather than minting a
        // second one at `near` — exactly one origin node, exactly one pole.
        assert_eq!(
            road_nodes_at_origin, 1,
            "the second road click must snap, not stub"
        );
        assert_eq!(
            poles_at_origin, 1,
            "the second wire click must snap, not stub"
        );
    }
}
