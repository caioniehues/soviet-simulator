//! The shared capacitated-network solver (B8.1, ROADMAP "The Web"): every
//! utility — electricity, water, heat — reduces to the same shape at this
//! stage: undirected spans union endpoints into components, producers pour
//! supply into their component's pool, and consumers drain it in **priority
//! order** — the planner's deficit-allocation rule (spec/electricity.md
//! OURS: brownout before blackout, hospitals-last shape). Per-edge capacity
//! and distance loss arrive when the networks earn them; the pool is the
//! confirmed W&R-scale abstraction for district grids.

use bevy::platform::collections::HashMap;
use bevy::prelude::*;

/// Deficit-allocation rank: lower serves first. A starved component browns
/// out its highest ranks first — industry goes dark before homes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PriorityClass {
    /// Homes (and later hospitals) — the last thing the planner unplugs.
    Housing = 0,
    Industry = 1,
}

/// Union-find over span endpoints. Entities never touched by a span have no
/// component — disconnected consumers get nothing, connected-but-starved
/// consumers brown out by class.
pub struct Components {
    parent: HashMap<Entity, Entity>,
}

impl Components {
    pub fn from_spans(spans: impl Iterator<Item = (Entity, Entity)>) -> Self {
        let mut parent: HashMap<Entity, Entity> = HashMap::new();
        for (a, b) in spans {
            let (ra, rb) = (Self::root_of(&mut parent, a), Self::root_of(&mut parent, b));
            if ra != rb {
                parent.insert(ra, rb);
            }
        }
        Self { parent }
    }

    fn root_of(parent: &mut HashMap<Entity, Entity>, e: Entity) -> Entity {
        let p = *parent.entry(e).or_insert(e);
        if p == e {
            return e;
        }
        let r = Self::root_of(parent, p);
        parent.insert(e, r);
        r
    }

    /// The component root of a *connected* entity; None when no span ever
    /// touched it.
    pub fn root(&mut self, e: Entity) -> Option<Entity> {
        if !self.parent.contains_key(&e) {
            return None;
        }
        Some(Self::root_of(&mut self.parent, e))
    }
}

/// Pool the producers' output per component, then serve consumers in
/// (priority, stable-id) order. Returns which consumers were served.
/// `demands` entries are (consumer entity, stable id, class, demand).
pub fn allocate(
    components: &mut Components,
    supplies: impl Iterator<Item = (Entity, f32)>,
    demands: &[(Entity, u64, PriorityClass, f32)],
) -> HashMap<Entity, bool> {
    let mut pools: HashMap<Entity, f32> = HashMap::new();
    for (entity, output) in supplies {
        if let Some(root) = components.root(entity) {
            *pools.entry(root).or_default() += output;
        }
    }
    let mut order: Vec<&(Entity, u64, PriorityClass, f32)> = demands.iter().collect();
    order.sort_by_key(|(_, id, class, _)| (*class, *id));
    let mut served = HashMap::new();
    for (entity, _, _, demand) in order {
        let ok = components.root(*entity).is_some_and(|root| {
            let pool = pools.entry(root).or_default();
            if *pool >= *demand - 1e-6 {
                *pool -= demand;
                true
            } else {
                false
            }
        });
        served.insert(*entity, ok);
    }
    served
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starved_pool_browns_out_industry_before_housing() {
        let mut world = World::new();
        let hub = world.spawn_empty().id();
        let plant = world.spawn_empty().id();
        let home = world.spawn_empty().id();
        let factory = world.spawn_empty().id();
        let mut components =
            Components::from_spans([(plant, hub), (home, hub), (factory, hub)].into_iter());
        // 5 units of supply against 1 + 5 of demand: the home (Housing)
        // must be served even though the factory's id sorts first.
        let served = allocate(
            &mut components,
            [(plant, 5.0)].into_iter(),
            &[
                (factory, 1, PriorityClass::Industry, 5.0),
                (home, 2, PriorityClass::Housing, 1.0),
            ],
        );
        assert_eq!(served[&home], true);
        assert_eq!(served[&factory], false, "industry browns out first");
    }

    #[test]
    fn disconnected_consumers_get_nothing_even_with_surplus() {
        let mut world = World::new();
        let hub = world.spawn_empty().id();
        let plant = world.spawn_empty().id();
        let island = world.spawn_empty().id();
        let mut components = Components::from_spans([(plant, hub)].into_iter());
        let served = allocate(
            &mut components,
            [(plant, 100.0)].into_iter(),
            &[(island, 1, PriorityClass::Housing, 1.0)],
        );
        assert_eq!(served[&island], false);
    }
}
