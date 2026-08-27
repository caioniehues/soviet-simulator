# Memory Index

- [Idle truck blocks lane](idle-truck-blocks-lane.md) — a "stuck dispatch" can be a lower-layer physics/lane obstruction, not an itinerary/state-machine bug; instrument road.rs directly when position itself is frozen
- [unpark orphans collider if not parked](unpark-orphans-collider-if-not-parked.md) — unpark() has no precondition guard; grabbing a mid-RoadToPark vehicle orphans a phantom collider. Also: verify your harness can catch a failure class before trusting a negative mutation result
