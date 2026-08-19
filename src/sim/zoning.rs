//! Zoning stage 1 (B7.3, spec/zoning.md): the general plan as a land-use
//! overlay — painted districts constrain what the planner may site there and
//! surface mismatches, and **never** cause a building to exist. A
//! zoned-but-empty district is a plan not yet fulfilled: a visible backlog
//! on the PLAN dashboard, not latent demand. No RCI bar, no spawner — the
//! housing queue is residential demand (spec zoning.md §G5).

use bevy::prelude::*;

use super::buildings::{Building, BuildingKind};
use super::stages::{ApplyCommandsFlush, SimStage, SimTick};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ZoneKind {
    Residential,
    Industrial,
}

impl ZoneKind {
    /// Which building kinds a district of this use accepts. Anything goes on
    /// unzoned land — zoning constrains, it never demands.
    pub fn admits(self, kind: BuildingKind) -> bool {
        match self {
            ZoneKind::Residential => matches!(kind, BuildingKind::Dwelling | BuildingKind::BusStop),
            ZoneKind::Industrial => !matches!(kind, BuildingKind::Dwelling),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ZoneId(pub u64);

/// An axis-aligned district on the ground plane.
#[derive(Component, Debug)]
pub struct Zone {
    pub id: ZoneId,
    pub kind: ZoneKind,
    pub min: Vec2,
    pub max: Vec2,
}

impl Zone {
    pub fn contains(&self, pos: Vec3) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.z >= self.min.y && pos.z <= self.max.y
    }
}

#[derive(Resource, Default)]
pub struct ZoneEditQueue(pub Vec<ZoneEdit>);

#[derive(Clone, Copy, Debug)]
pub enum ZoneEdit {
    Paint {
        kind: ZoneKind,
        min: Vec2,
        max: Vec2,
    },
    /// Remove the zone under `pos` (un-planning is always allowed; the
    /// buildings already standing there are grandfathered).
    Erase { pos: Vec3 },
}

/// Why the last placement was rejected by the plan, for the HUD. Cleared by
/// the next successful placement.
#[derive(Resource, Default, Clone, Copy)]
pub struct ZoningFeedback(pub Option<(BuildingKind, ZoneKind)>);

#[derive(Resource, Default)]
pub struct ZoneIds {
    pub next: u64,
}

pub struct ZoningSimPlugin;

impl Plugin for ZoningSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZoneEditQueue>()
            .init_resource::<ZoneIds>()
            .add_systems(
                SimTick,
                apply_zone_edits
                    .in_set(SimStage::ApplyCommands)
                    .after(ApplyCommandsFlush)
                    // The siting gate reads zones during building placement.
                    .before(super::buildings::apply_building_edits),
            );
    }
}

fn apply_zone_edits(
    mut commands: Commands,
    mut queue: ResMut<ZoneEditQueue>,
    mut ids: ResMut<ZoneIds>,
    zones: Query<(Entity, &Zone)>,
) {
    for edit in queue.0.drain(..) {
        match edit {
            ZoneEdit::Paint { kind, min, max } => {
                let (min, max) = (min.min(max), min.max(max));
                if (max - min).min_element() < 4.0 {
                    warn!("Paint dropped: degenerate zone rect");
                    continue;
                }
                ids.next += 1;
                commands.spawn(Zone {
                    id: ZoneId(ids.next),
                    kind,
                    min,
                    max,
                });
            }
            ZoneEdit::Erase { pos } => {
                if let Some((entity, _)) = zones.iter().find(|(_, z)| z.contains(pos)) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

/// The siting gate, called by `apply_building_edits`: a placement inside a
/// district whose use does not admit the kind is rejected with feedback.
/// Unzoned land admits anything.
pub(crate) fn siting_allowed(
    kind: BuildingKind,
    pos: Vec3,
    zones: &Query<&Zone>,
) -> Result<(), ZoneKind> {
    for zone in zones.iter() {
        if zone.contains(pos) && !zone.kind.admits(kind) {
            return Err(zone.kind);
        }
    }
    Ok(())
}

/// A zone with no admitted building inside is an unfulfilled plan — the
/// dashboard's backlog number.
pub fn unfulfilled_zones(zones: &Query<&Zone>, buildings: &Query<&Building>) -> (usize, usize) {
    let total = zones.iter().count();
    let unfulfilled = zones
        .iter()
        .filter(|z| {
            !buildings
                .iter()
                .any(|b| z.contains(b.pos) && z.kind.admits(b.kind))
        })
        .count();
    (total, unfulfilled)
}

#[cfg(test)]
mod tests {
    use super::super::SimPlugin;
    use super::super::buildings::{BuildingEdit, BuildingEditQueue, BuildingSimPlugin};
    use super::*;
    use std::time::Duration;

    fn app() -> App {
        let mut a = App::new();
        a.insert_resource(Time::<()>::default());
        a.add_plugins((SimPlugin, BuildingSimPlugin, ZoningSimPlugin));
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

    #[test]
    fn zoning_constrains_siting_and_never_spawns() {
        let mut app = app();
        app.world_mut()
            .resource_mut::<ZoneEditQueue>()
            .0
            .push(ZoneEdit::Paint {
                kind: ZoneKind::Industrial,
                min: Vec2::new(0.0, 0.0),
                max: Vec2::new(100.0, 100.0),
            });
        ticks(&mut app, 2);
        {
            let world = app.world_mut();
            assert_eq!(world.query::<&Zone>().iter(world).count(), 1);
            assert_eq!(
                world.query::<&Building>().iter(world).count(),
                0,
                "a zone never causes a building to exist"
            );
        }
        // A dwelling inside the industrial district is refused with feedback…
        let place = |app: &mut App, kind, pos| {
            app.world_mut()
                .resource_mut::<BuildingEditQueue>()
                .0
                .push(BuildingEdit::Place { kind, pos });
            ticks(app, 2);
        };
        place(&mut app, BuildingKind::Dwelling, Vec3::new(50.0, 0.0, 50.0));
        {
            let world = app.world_mut();
            assert_eq!(world.query::<&Building>().iter(world).count(), 0);
            let feedback = world.resource::<ZoningFeedback>().0;
            assert_eq!(
                feedback,
                Some((BuildingKind::Dwelling, ZoneKind::Industrial))
            );
        }
        // …a factory inside is admitted, a dwelling outside is free land.
        place(&mut app, BuildingKind::Factory, Vec3::new(50.0, 0.0, 50.0));
        place(&mut app, BuildingKind::Dwelling, Vec3::new(200.0, 0.0, 0.0));
        let world = app.world_mut();
        assert_eq!(world.query::<&Building>().iter(world).count(), 2);
        assert!(world.resource::<ZoningFeedback>().0.is_none());
    }

    #[test]
    fn backlog_counts_unfulfilled_districts_and_erase_unplans() {
        let mut app = app();
        {
            let mut edits = app.world_mut().resource_mut::<ZoneEditQueue>();
            edits.0.push(ZoneEdit::Paint {
                kind: ZoneKind::Residential,
                min: Vec2::new(0.0, 0.0),
                max: Vec2::new(50.0, 50.0),
            });
            edits.0.push(ZoneEdit::Paint {
                kind: ZoneKind::Industrial,
                min: Vec2::new(100.0, 0.0),
                max: Vec2::new(150.0, 50.0),
            });
        }
        app.world_mut()
            .resource_mut::<BuildingEditQueue>()
            .0
            .push(BuildingEdit::Place {
                kind: BuildingKind::Factory,
                pos: Vec3::new(120.0, 0.0, 20.0),
            });
        ticks(&mut app, 3);
        let world = app.world_mut();
        let zones: Vec<(ZoneKind, Vec2, Vec2)> = world
            .query::<&Zone>()
            .iter(world)
            .map(|z| (z.kind, z.min, z.max))
            .collect();
        let placed: Vec<(BuildingKind, Vec3)> = world
            .query::<&Building>()
            .iter(world)
            .map(|b| (b.kind, b.pos))
            .collect();
        let unfulfilled = zones
            .iter()
            .filter(|(k, min, max)| {
                !placed.iter().any(|(bk, bp)| {
                    bp.x >= min.x
                        && bp.x <= max.x
                        && bp.z >= min.y
                        && bp.z <= max.y
                        && k.admits(*bk)
                })
            })
            .count();
        assert_eq!(
            (zones.len(), unfulfilled),
            (2, 1),
            "the empty residential district is a plan not yet fulfilled"
        );
        app.world_mut()
            .resource_mut::<ZoneEditQueue>()
            .0
            .push(ZoneEdit::Erase {
                pos: Vec3::new(20.0, 0.0, 20.0),
            });
        ticks(&mut app, 2);
        let world = app.world_mut();
        assert_eq!(world.query::<&Zone>().iter(world).count(), 1);
    }
}
