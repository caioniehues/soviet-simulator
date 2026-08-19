---
name: sim-plugins-group-shape
description: SimPlugins/GamePlugins PluginGroup shape (ADR 0012, ticket #118) — full 18-plugin order, and which of the 8 bench bins disable what
metadata:
  type: project
---

`SimPlugins` (`src/lib.rs`) and `GamePlugins` (`src/game/mod.rs`) are Bevy
`PluginGroup`s built with `PluginGroupBuilder::start::<Self>().add(...)`, replacing
the old `add_sim_plugins()` free function and `GamePlugin` single-plugin wrapper. A
binary composes `app.add_plugins(SimPlugins).add_plugins(game::GamePlugins)` — or
`SimPlugins.build().disable::<T>()...` when it wants a subset — never a hand-typed
inclusion tuple. `game::zoning::ZoningToolPlugin` still has its own
`is_plugin_added::<ZoningSimPlugin>` guard (`src/game/zoning.rs:19`) — untouched,
out of scope, still correct since it's a different plugin-ordering concern (tool
plugin vs. group membership).

`SimPlugins` order (also the order `lib.rs`'s old function used, load-bearing because
`CommuteSimPlugin`/`DispatchSimPlugin` auto-add `PathfindingSimPlugin` +
`TransitSimPlugin`/`TrafficSimPlugin` via `is_plugin_added` guards — see
[[plugin-group-dedup-trap]] for the resource-ownership half of this ticket):
Sim, Road, Building, Storage, Household, Labour, Commute, Needs, Vehicle, Dispatch,
Construction, Zoning, Water, Heat, Plan, Customs, Wire, Save.

Pathfinding/Transit/Traffic are dependency-only — never their own `.add()` entry in
the group. A bin that disables both `CommuteSimPlugin` and `DispatchSimPlugin` and
still wants them (`bench_traffic`) must `.add_plugins((PathfindingSimPlugin,
TrafficSimPlugin))` itself, outside the group.

Per-bench disable list as of 2026-08-19 (all verified against their real gate,
release build, `cargo run --release --bin <name>`):

| bin | disables |
|---|---|
| `bench_chain` (0.33ms gate) | Household, Labour, Commute, Needs, Construction, Zoning, Water, Heat, Customs, Save |
| `bench_citizens` | Construction, Zoning, Water, Heat, Customs, Save |
| `bench_dispatch` | Household, Labour, Commute, Needs, Construction, Zoning, Water, Heat, Customs, Wire, Save |
| `bench_networks` | Road, Storage, Household, Labour, Commute, Needs, Vehicle, Dispatch, Construction, Zoning, Customs, Save |
| `bench_render` | none — runs the full stack + `GamePlugins`, matches `lib.rs` exactly |
| `bench_sites` | Household, Labour, Commute, Needs, Zoning, Water, Heat, Customs, Wire, Save |
| `bench_traffic` | everything except Road (16 disables) + explicit `PathfindingSimPlugin`/`TrafficSimPlugin` add |
| `bench_transit` | Storage, Needs, Dispatch, Construction, Zoning, Water, Heat, Customs, Wire, Save |
| `capture` (M1.7) | Construction, Zoning, Water, Heat, Customs — this was the actual drift bug the ticket named (9-of-18 plugins missing `PlanSimPlugin`, latent panic risk masked only by the duplicate registrations this same ticket deleted) |
| `capture_g1`, `capture_r0` | none — already ran the full stack pre-ticket |

Never enable `ConstructionSimPlugin` in a bench that places buildings via
`BuildingEditQueue` and expects them immediately functional (`bench_networks`,
`bench_chain`, etc.) — with it enabled every placed building gets a
`ConstructionSite` and produces nothing until built, which the R0.7 inertness fix
(separate slice, `Without<ConstructionSite>` on heat/water) makes bite immediately.
Only `bench_sites` wants sites.

Deleted `capture_m2`..`capture_m8` (7 bins, no `[[bin]]` entries in `Cargo.toml` —
Cargo auto-discovers `src/bin/*.rs`, so `git rm` is the whole job).
