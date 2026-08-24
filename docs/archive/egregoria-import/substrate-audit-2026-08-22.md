# The Egregoria substrate — what we inherited

**Status:** authoritative input to the 1.0 requirements pass. Produced 2026-08-22
by six parallel audits against upstream `ae65c857` immediately after the fork
([`NOTICE.md`](../NOTICE.md)). Every claim below carries a `file:line` a skeptic
can check; claims that could not be verified are marked UNCONFIRMED and must not
be cited as fact.

The specs in `spec/` were written against the abandoned Bevy track. They describe
**intent**, not existing code. This document is the map between them.

Classification used throughout: **PROVIDED** (exists, works) · **PARTIAL**
(machinery exists, delta needed) · **ABSENT** (greenfield) · **CONFLICTS**
(Egregoria does something incompatible with our design).

---

## 1. The five findings that change planning

1. **Determinism is proven, not hoped for.** `simulation/src/tests/test_iso.rs`
   (`test_world_survives_serde`) replays a recorded 41KB session twice
   independently and binary-searches to the exact tick of first divergence,
   dumping both sides to `<name>_a.json`/`_b.json` for diffing.
   `TestCtx::tick` (`simulation/src/tests/mod.rs:65-82`) round-trips the whole
   sim through bincode every tick and asserts hash equality.
   **Consequence: behaviour tests need no new harness.** Extend `TestCtx`.

2. **Internal trade already has no price in it.** Every internal trade is
   `money_delta: Money::ZERO` (`economy/market.rs:226`); matching ranks pairs
   purely by squared distance (`market.rs:219`). The clearing rule is
   quantity-and-proximity barter, not price discovery. "Keep the matching
   engine, freeze the prices" is already true of most of the code path.

3. **At the citizen level Egregoria is already not capitalist.** No wages
   (grep for wage/salary returns nothing; the only labour-linked money is a flat
   government upkeep debit, `economy/mod.rs:51`). Humans have no currency field
   at all. Prices are cost-plus, computed once at startup by `calculate_prices`
   (`market.rs:343-403`) and never recalculated. Citizens never compare a price.
   The capitalism lives entirely in the government's external-trade account.

4. **Electricity exists and is the model we reject.** See §5.

5. **Save format has no migration story.** Version mismatch only logs
   `"incompatible version, save might be corrupted!"` and attempts the decode
   anyway (`simulation/src/lib.rs:409-420`). Bincode will hard-fail on any field
   reshape. **Treat every save as disposable across schema changes.**

---

## 2. Map, roads, traffic, vehicles

| Requirement | Verdict | Evidence |
|---|---|---|
| Unified lane graph (node/segment/lane) | PROVIDED | `map/objects/road.rs:34`, `intersection.rs:23`, `lane.rs:64` |
| Road types as authored prefabs | PROVIDED | `LanePattern`/`LanePatternBuilder`, `lane.rs:83-228` |
| Turn-restriction-aware routing | PROVIDED | `Intersection::turns_from/turns_to`, `intersection.rs:289,304` |
| A* over lane-hop chains, per trip, no OD cache | PROVIDED | `map/pathfinding.rs:134,247` |
| Road surface class (dirt/paved/highway) | ABSENT | `LaneKind` is lane *purpose*, not surface; `lane.rs:13` |
| **Per-segment traffic density / congestion** | **ABSENT** | see §6 |
| Car-following | PARTIAL | IDM-ish lookahead raycast, `transportation/road.rs` `calc_front_dist` |
| Vehicles never despawn on gridlock | PROVIDED (better than spec feared) | `VehicleState::Panicking`, `vehicle.rs:19-20`, 200s wait then resume |
| Parking as physical reserved slots | PROVIDED | `map_dynamic/parking.rs:42,58` |
| Vehicle as economic asset (fuel/wear/capacity/owner/driver) | ABSENT | `Vehicle` (`transportation/vehicle.rs:34-44`) is a bare kinematic shell |
| Roads/buildings as construction output | CONFLICTS | `Road::make`, `Building::make` materialize instantly, uncosted |

**CONFLICT — `Lot::generate_along_road` (`map/objects/lot.rs:59-104`).** Every road
built auto-spawns a strip of randomly-sized (20/30/40m) building lots along it,
independent of planner intent. This is structurally the CS1 road-birthed zoning
swarm `spec/zoning.md` studied and rejected. **Decision 2026-08-22: disable
entirely.** The planner sites every building explicitly. Accept that early cities
look emptier until siting tools exist.

Note two unrelated types both called `Zone`: `Building.zone` (`building.rs:49,78`)
is a company's own production footprint polygon, *not* city zoning.

---

## 3. Buildings, production, resources

| Requirement | Verdict | Evidence |
|---|---|---|
| Many-to-many recipes with duration | PROVIDED | `prototypes/src/types/recipe.rs:35-47` |
| Extraction (no inputs) | PROVIDED | `base_mod/companies.lua:56` |
| Output-space backpressure | PROVIDED | `recipe_should_produce`, `souls/goods_company.rs:36-39` |
| Money never gates production | PROVIDED | internal recipes never touch `Money` |
| Declared workforce, runtime sourcing | PROVIDED | `n_workers`, `goods_company.rs:25,266-295` |
| One building, one recipe (ADR-0017) | PROVIDED | `GoodsCompanyPrototype.recipe: Option<Recipe>` |
| Liebig multiplicative bottleneck | PARTIAL | labour is linear, power is a **binary** blackout gate (`goods_company.rs:95-101`); inputs are all-or-nothing boolean, not continuous scaling |
| Bottleneck reason surfaced to player | ABSENT | `recipe_should_produce` returns bare `bool` |
| Byproducts, pollution, waste chain | ABSENT | `Recipe` has only consumption/production |
| Machinery/wear factor, skilled-labour tier | ABSENT | single `n_workers` tier only |
| **Full construction process** | **ABSENT / CONFLICTS** | `BuildingPrototype.price: Money` (`building.rs:33`) is the flat purchase price `spec/construction.md` rejects. No `ConstructionProject`, phases, bill of quantities, or construction office exists |
| **Item ontology** (mass, volume, storage/transport class, shelf life, hazard) | **ABSENT** | `ItemPrototype` is `{base, id, optout_exttrade}` — `prototypes/src/prototypes/item.rs:8-12`. That is the entire schema |

**The prototype layer** (`prototypes/`, Lua via `mlua`/luau) is Factorio's
`data:extend` idiom (`base_mod/companies.lua:1`), loaded once into a leaked
static (`load.rs:17-31,61-63`) — **no hot reload**, balance edits need a restart.
Typed single-parent chains (`goods_company ⊂ building ⊂ base`) via a required
`base:` field plus `Deref`, declared in `prototypes/src/prototypes/mod.rs`.
Adding fields is additive and cheap (`get_lua`/`get_lua_opt` helpers,
`goods_company.rs:39-43`) — this is where the Soviet catalogue goes.

`FreightStation` (`souls/freight_station.rs`) is the nearest existing
"logistics office" shape — owns a fleet, counts wanted/waiting cargo, dispatches.
Study it as a pattern for the construction office; it is not reusable as-is.

---

## 4. Economy — the Kornai merge

The design target is Kornai's shortage economy; see
`docs/charter-1.0.md` and the memory file `economy-model-kornai`.

**Where the market machinery actually stands:**

- **Clearing rule.** `make_trades` (`market.rs:193-336`) collects every viable
  (seller, buyer) pair, scores each `sorder.pos.distance2(border.pos)`
  (`market.rs:219`), sorts globally ascending (`market.rs:232-233`), drains
  greedily. No price term anywhere in the loop.
- **Price enters only at the border**, as a fixed per-item `ext_value`
  (`market.rs:35`) from `calculate_prices` (`market.rs:343-403`), computed once,
  never recalculated.
- **Money is one undifferentiated scalar.** `Money(i64)`
  (`prototypes/src/types/money.rs:14`) serves treasury, ext price and trade
  delta alike. No account types, no circuits.
- **`SingleMarket.capital` is a false friend** (`market.rs:32`): it is *goods on
  hand* (`i32`), not currency. The only real money pool is `Government.money`
  (`government.rs:10`).

**CONFLICT — the infinite external partner.** Every unmet buy order is satisfied
instantly and unconditionally by an external partner with no capacity limit and
no vehicle trip (`market.rs:285-304`, symmetric sell side `:307-331`). This is
exactly the CS1 "unlimited priority-0 offer" pattern `spec/logistics.md` rejects.
**This is where excess demand must become a queue.**

**BUG on that same seam.** Both external loops mutate `capital` *before*
checking a freight station exists (`market.rs:291` credits, `:293` then checks;
same shape at `:317`/`:320`). If `find_external` returns `None`, goods appear
with no trade record and no money debit. Harmless today, load-bearing the moment
the queue lands there. Fix as part of that change, not on top of it.

**The hoarding hook is one function.** `Market::buy_until` (`market.rs:161-167`),
called from `recipe_init`/`recipe_act` (`souls/goods_company.rs:23,47`), always
requests exactly `item.amount` from the static recipe. Plan quota and honest
requirement are currently the same literal number — nothing to see through yet.
**This is the single cleanest insertion point for the core game loop.**

**Reusability verdict:** the internal matcher is already a zero-price physical
logistics matcher. Deleting the external block leaves it fully functional. Gaps
for our use are additive fields on `BuyOrder`/`SellOrder`/`SingleMarket`
(transport class, deficit priority, queue position/timestamp), not a different
engine.

**Decision 2026-08-22 — two circuits confirmed** despite the cost. Households get
cash (*nal*) and wages; enterprises settle in accounting roubles (*beznal*) that
cannot buy consumer goods. Note this is **net-new machinery**: there is currently
no household money and no wage system to rewire.

**Decision 2026-08-22 — bootstrap: both, different roles.** A stocked starter
warehouse seeds turn one; customs imports at a markup remain the permanent paid
escape hatch whenever a chain deadlocks.

---

## 5. People

| Requirement | Verdict | Evidence |
|---|---|---|
| Persistent individual identity across save/load | PROVIDED | `PersonalInfo` in `HumanEnt`, whole sim via `CompressedBincode`; souls are not fungible |
| Fixed workplace binding | PROVIDED | `Work.workplace`, `souls/desire/work.rs:20-26` |
| Purchases resolve on arrival, not on trade match | PROVIDED (spec's target bug already fixed) | `souls/desire/buyfood.rs:50-54` |
| Per-person decision loop | PROVIDED, thin | three `score()` fns in a max-arbiter, `souls/human.rs:190-231` |
| Needs as 0..1 satisfaction, wants, aspirations | ABSENT | only food is modelled, as a hunger-clock utility score |
| Households (shared pantry, family, housing queue) | ABSENT | every human owns their own home and pantry |
| Education, healthcare, crime | ABSENT | no fields, no buildings |
| Age progression, death | ABSENT | `age` set once at spawn, never incremented |
| Labour allocation by tier/commute feasibility | CONFLICTS | a job is an `ItemID::new("job-opening")` traded by **Euclidean distance** (`souls/human.rs:267-269`, `market.rs:216`) |

**Scale:** the only throughput mechanism is a per-human randomized re-decision
interval, `30 + rand2(pos)*50` ticks, spatially staggered (`human.rs:185`).
Worth preserving and extending. All systems iterate the full human collection
sequentially each tick; no cap, no LOD, no `par_iter`. **Population ceiling is
UNCONFIRMED** — needs runtime profiling, not a code read. Upstream's own doc
flags per-citizen AI as an unresolved scale risk.

Standing decision: `souls/human.rs` stays as-is until the economy is rewired.

---

## 6. Utilities

**Electricity — CONFLICTS.** It exists, and it is the coverage-and-connectivity
model `spec/electricity.md` rejects by name. `ElectricityCache`
(`map/electricity_cache.rs:39-63`) is a union-find over road-graph adjacency
where `map_electricity_edges` (`:244-280`) makes every building auto-adjacent to
its road and every road to its intersections. Any building touching any road
touching any producer is powered. No laid wire, no capacity, no loss, no voltage
tiers. Blackout is binary per network (`map_dynamic/electricity.rs:89`), no
priority classes, no brownout. **Whoever takes this ticket is replacing a working
system**, and the change is architectural: unweighted adjacency → weighted,
typed, capacitated graph.

Generation piggybacks on ordinary recipe buildings via `power_production` /
`power_consumption` (`prototypes/src/prototypes/building.rs:34-35`,
`base_mod/companies.lua:96-116`) — a good pattern to copy for other utilities.
`Power` (`prototypes/src/types/power.rs`) is a clean unit-parsing newtype
(`"2.46MW"`), reusable as a template for `Volume`/`Mass`.

**Water, sewage, heating, waste — ABSENT, all four.** Verified by keyword sweeps
across `simulation/src`, `prototypes/`, `base_mod/`: zero footprint. The only
`water` hit is a terrain height doc-comment (`map/terrain.rs:86`).

**Hidden prerequisite:** heating's demand model needs temperature, and there is
**no weather or climate system at all** in `simulation/src`.

`ElectricityCache`'s union-find is a *template to clone*, not state to share —
the specs require separate networks per utility.

---

## 7. Traffic congestion — the missing subsystem

Route cost is `l.points.length() / l.speed_limit` plus a random jitter seeded
from tick and lane (`map/pathfinding.rs:224-225`). The jitter scatters routes
but carries **no load information**. Nothing accumulates per-lane load; nothing
feeds load back into routing. `spec/traffic.md` and `spec/pathfinding.md` were
both written assuming CS1's density byte and congestion multiplier were
adoptable. They are not present.

**Scouted 2026-08-22 — no one has this to give us:**

- **A/B Street** — Apache-2.0, **usable**, alive. But `vehicle_cost()`
  (`map_model/src/pathfind/vehicles.rs:298-388`) is static freeflow time plus
  fixed penalties, same shape as ours; its CH graph rebuilds only on map edits.
  Its `Queue { geom_len, reserved_length }`
  (`sim/src/mechanics/queue.rs:31-44`) is a real per-lane capacity accumulator —
  **portable near-verbatim under Apache-2.0** — but feeds only local insertion
  and lane-changing, never the router.
- **Most valuable finding: A/B Street tried congestion-triggered rerouting and
  removed it.** Their discrete-event design doc reports agents "repeatedly
  rerouted" into pathological trip times. This is the flapping failure mode,
  confirmed by someone who hit it.
- **Citybound** — AGPL-3.0, confirmed. **Disqualified** (would force the whole
  project to AGPL). Also abandoned since 2023-01.
- **OpenTTD** — GPL-2.0-**only**, confirmed by reading `COPYING.md` directly.
  **Off-limits for code**, ideas only.
- **SUMO** — dual EPL-2.0 **OR GPL-2.0-or-later**; the GPL arm makes it
  **usable**. **MATSim** — GPL-2.0-or-later, usable.
- **Simutrans** — Artistic License 1.0, not on the approved list; needs an
  explicit call before copying anything.

**Recommended build (deliberate, not first-thing-found):**

1. Per-lane **EMA** load counter (O(1) per lane, time-constant of a few in-game
   minutes), pattern borrowed from A/B Street's `Queue`.
2. **BPR volume-delay function** `t = t₀·(1 + 0.15·(v/c)⁴)` as the cost
   multiplier over the existing `length/speed_limit` term.
3. **Gawron damping before it re-enters A***: blend, never snap —
   `T(i+1) = 0.3·observed + 0.7·remembered`. This is SUMO's default and it is
   specifically what prevents the two-corridor ping-pong A/B Street hit.
4. Player-readable congestion derived **from the same EMA**, not measured
   separately — TM:PE issue #66 shows two competing density trackers is a real
   bug class.
5. MATSim's storage/flow-capacity queue model is the v2 stretch goal if BPR
   proves too crude.

**UNCONFIRMED, do not cite:** CS1's "2× congestion multiplier" that our specs
state as fact could not be verified from any primary or community source.
Simutrans's actual cost function was not located.

---

## 8. Presentation and shell

**Renderer** (`engine/`, wgpu forward): depth prepass, cascaded sun shadows,
SSAO, fog, PBR with full IBL (irradiance + specular prefilter + BRDF LUT),
instancing, sky pass — `engine/src/gfx.rs:712-775`. Meshes are glTF.

**The art-direction gap is the cheapest high-leverage fix in the project.**
The entire palette is `base_mod/colors.lua:1-70`, a ~15-field Lua table, still
100% stock Egregoria — including `lot_residential_col = {0.2,0.6,0.25}`, the
exact saturated lawn-green `docs/art-direction.md` forbids. One file.

But note: **the old enforcement mechanism is gone.** `src/game/palette.rs` did
not survive the fork, and `Material::new_raw` (`engine/src/material.rs:44-120`)
takes unclamped `metallic`/`roughness` from per-asset glTF and
`assets/companies.json`. There is no chokepoint where the art doc's material
rules (roughness floor, albedo clamp) can be enforced — one must be added, or
the rules enforced at authoring time.

Sun colour/direction is procedural time-of-day (`native_app/src/game_loop.rs:274-308`),
i.e. Egregoria has a full day/night cycle the art doc does not mention wanting.

**Shell:** yakui-primary (20 files) with egui surviving in 6 legacy spots. The
architecture doc's claim that `topgui.rs` holds most of the UI is **stale**.
New panels use the `goryak` idiom — `Window{...}.show(|| {...})`, see
`native_app/src/gui/hud/windows/load.rs:33-124`. Tools register as per-frame
systems in `run_ui_systems` (`native_app/src/gui/mod.rs:40-54`).

**Notifications are a real gap against the playtest verdict.** There is no
severity-tiered toast system — only a single-slot `ErrorTooltip`
(`native_app/src/gui/mod.rs:56-79`) with no colour coding. "Critical warnings
were invisible" will recur unless this is built.

**Headless** (`headless/src/main.rs`) is a real server binary with autosave. For
behaviour tests the lower-ceremony path is driving `Simulation` in-process the
way `simulation/src/tests/` already does.

---

## 9. Open questions this audit could not close

- Population ceiling — needs runtime profiling.
- CS1's congestion multiplier constant — unverified, drop the assumption.
- Simutrans's cost function and its Artistic-1.0 usability.
- Whether `assets/shaders/pbr/render.wgsl` has any weathering support.
- Egregoria pins **git branches** for `egui` (master) and a personal `yakui`
  fork (`dev`). These must be locked to commits before anything is distributed.
