# Memory Index

- [Vehicle substrate: unpark/park](vehicle-substrate-unpark.md) — the four VehicleState variants, which ones move, the three unpark callers, why Parked+Itinerary never moves
- [Phantom collider and congestion](phantom-collider-congestion.md) — an orphaned grid entry is a permanent non-escalating stall, and it would poison BPR/EMA lane cost
- [ToSource wedge surface](dispatch-tosource-wedge-surface.md) — ToSource with truck=Some has no timeout; the market.rs Parked guard must never be removed
- [Dispatcher pool and reachability](dispatcher-pool-and-reachability.md) — 50 ticks/s, 1 truck per factory, query's backward BFS is the real reachability test, and the export half still teleports
