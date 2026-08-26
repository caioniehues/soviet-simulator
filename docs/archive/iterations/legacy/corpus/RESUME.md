# Resume — ITER-0000 in flight

**Process: `docs/dev-cycle.md`** — the eight phases and the 15-agent roster. Read it before
dispatching anything. `br ready` is the live task queue; this file is the narrative handoff.


Read this before acting; do not re-derive state. Rewritten 2026-08-22 (the previous
version said "roadmap is next" — it predated commit `2f0cbf0`).

## Where things stand — verified against disk, not remembered

| artifact | state |
|---|---|
| `requirements/` | **36 epics, 149 stories** (an earlier note said 35/139 — wrong, counted 2026-08-22). **Authoritative for STORY-IDs and for the `**Deferred:** true` flag** |
| `extract/*.json` | the eight per-domain sources, keyed by story TITLE — no STORY-IDs. `extract/validate.py` is the schema gate |
| `behavior-scenarios.md` | 151 scenarios — 57 surface, 53 contract, 41 failure-recovery |
| `behavior-corpus.md` | 151 rows; 5 promoted to `sentinel`; every `Command` is `TBD` |
| `coverage-ledger.md` | 23 chunks: 22 covered, 1 non-normative. 0 gaps |
| `roadmap.md` | **130 scheduled + 19 deferred = 149**, 14 iterations (ITER-0000..0013). All `Status: pending`. Regenerate with `python3 docs/superpowers/iterations/build_roadmap.py` **from the repo root** |
| `build_roadmap.py` | regenerates `roadmap.md` from `requirements/EPIC-*.md` |
| `par/*.md` | 16 omission-review reports, audit trail |

**Correction to a stale claim:** the `sov-scope-cut-1p6` ticket says deferral is set in
`extract/*.json`. It is not. `build_roadmap.py:87` reads `'**Deferred:** true' in blk or
'DEFERRED to Post-1.0' in blk` from the STORY block in `requirements/EPIC-*.md`. To defer a
story: edit its block in `requirements/`, then re-run `build_roadmap.py`.

## The scope cut — APPLIED 2026-08-22

19 stories deferred (18 new + STORY-0054), 130 scheduled. `apply_cut.py` holds the exact list
with a per-story charter citation and is idempotent. Two mechanisms, deliberately worded apart:

- **Full defer** inserts `**Deferred:** true`, which `build_roadmap.py:87` detects.
- **AC-level defer** prefixes ONE AC with `(POST-1.0 AC — excluded from 1.0 per …)`. It must
  NEVER contain the string `DEFERRED to Post-1.0`, because line 87 matches that at STORY-BLOCK
  level and would silently defer the whole story. Mutation-tested: the precedent wording used on
  STORY-0082 flips it to deferred; the wording used does not.

AC-level cuts (story stays in 1.0, one AC leaves): STORY-0078 AC-2, STORY-0122 AC-2/AC-3,
STORY-0082 AC-4, STORY-0139 AC-1.

Judgment calls made by the lead, not the miners: STORY-0139/0140 were reported as full charter
hits, but "vehicles are owned entities that park rather than despawn" is the foundation ITER-0003
stands on — only the fuel AC is charter-deferred. STORY-0127/0132 water and sewage treatment are a
single step with one quality ceiling, NOT "treatment tiers" — kept, and STORY-0103's 1.0
production gate depends on them. STORY-0136's incinerator is electricity OR heat per mode, never
both, so it is not CHP — kept.

## Superseded — `sov-scope-cut-1p6` (P1)

148 stories across 14 iterations is larger than a plausible 1.0. **`iterative-development`
must not start ITER-0000 until this is decided**, or the build targets an unratified scope.

The cut is not a matter of taste. `docs/charter-1.0.md:102-112` already ratifies a Post-1.0
list, and the roadmap schedules several of its items into 1.0 anyway. Charter beats spec on
SCOPE (spec still wins on MECHANISM inside an in-scope rung). Items to reconcile:

B11 crime · vehicle manufacture · vehicle lifecycle incl. fuel-as-commodity · voltage tiers ·
grid depth (transformers, treatment tiers, CHP, electric-heating fallback) · passenger rail,
signals, electrification · containers · perishables and refrigerated transport · kindergarten,
deathcare, epidemics · dual currency (already deferred: STORY-0054).

`Never` list (never schedule): tourism/hotels/attractions; fires and disasters.

## ITER-0000 in flight — where it actually stands

Commits: `1bc80ca` scope cut · `2fcfd94` harness · `6ea4553` hoard + ledger.
Suite: `cargo test -p simulation -- --test-threads=1` → **22 passed, 0 failed**.
ALWAYS use `--test-threads=1` — see `sov-test-race-initfuncs-qt6` (P1): `init.rs` pushes into
`static mut` globals unsynchronized, so the binary segfaults ~1-in-5 under parallel threads. This
is PRE-EXISTING, reproduced on an unmodified tree. A green parallel run proves little.

Done: STORY-0105, STORY-0106, STORY-0093–0097. STORY-0149 AC-1/2/3.

**The one real gap: STORY-0149 AC-4.** The delivered "truck" was
`const DISPATCH_TRAVEL_TICKS: u32 = 3` in `economy/market.rs` — a countdown, not a vehicle. The
ledger contract is honest and mutation-proven (restoring the match-time transfer fails
`test_dispatch_gates_stock_not_match` with "match must not move seller stock, left: 0, right: 5"),
but "nothing teleports" held only as bookkeeping, not physics. User chose to wire the real truck
rather than defer it. Agent T2b is doing that now.

What T2b must know (verified in source, do not re-derive):
- `DispatchKind::SmallTruck` already exists and maps to `LaneKind::Driving`; `DispatchID::SmallTruck(VehicleID)` exists. Only the registration in `Dispatcher::update()` is commented out (`dispatch.rs:94-102`) — and that commented block has TWO bugs: it says `DispatchID::Truck(ent)` (real variant is `SmallTruck`) and `truck.trans.position` (trains use `.trans.pos`).
- `souls/freight_station.rs` is the ONLY correct prior art for driving a dispatched delivery: `resources.write::<Dispatcher>()` at :76, `dispatch.query(map, DispatchKind::FreightTrain, DispatchQueryTarget::Pos(destination), ...)` at :145-148, `dispatch.free(v)` at :132.
- Company delivery runs through `c.sold.0.pop()` + `WorkKind::Driver` + `HumanDecisionKind::DeliverAtBuilding` (`goods_company.rs:244-263`) and never touches `Market` today.

Still owed after T2b: migrate the two hoarding tests out of `market.rs`'s unit-test module into
`tests/scenarios/hoarding.rs` under real SCENARIO-IDs (user confirmed corpus traceability
matters — unit tests cannot run as a sentinel set); STORY-0107's inspection panel; JOURNEY-0001;
and the 15–20s video.

## Original next-step notes — ITER-0000, the walking skeleton

User goal set 2026-08-22: **ITER-0000 end to end** — harness first, then the 9 stories,
JOURNEY-0001 passing, opus gate, and a 15–20s video. Not "implement all 130"; that was raised and
declined as un-completable in a session.

**Harness before features.** ITER-0000's first task is the scenario runner, not a story. There is
no runner today and `TestCtx` is `pub(crate)` under `#![cfg(test)]`, so it must either live inside
the crate or the visibility changes. See `sov-harness-lots-k54` and `sov-harness-perf-km4`.

The charter-amendment question (is 130 stories a 1.0 you want?) is deliberately deferred until
ITER-0000 ships and gives real per-story cost data.

## Standing obligations

- **Visual proof is owed.** Per `CLAUDE.md`, a 15–20s video once the first Soviet-side change
  lands. A prior screenshot attempt captured the wrong monitor.
- Egregoria pins git *branches* (`egui` master, a personal `yakui` fork's `dev`); lock to
  commits before any distribution.
- **`Lot::generate_along_road` is NOT disabled.** It runs live from `Map::connect`
  (`simulation/src/map/map.rs:719`), so roads DO auto-spawn lots today. An earlier brief
  claimed the opposite and that falsehood reached ~20 dispatches. Disabling it is *pending*:
  STORY-0013, ITER-0005. Consequence: `TestCtx::build_house_near` selects from `map().lots()`
  and breaks when STORY-0013 lands — see `sov-harness-lots-k54`.
- `TestCtx` is defined in `simulation/src/tests/mod.rs` (NOT `test_iso.rs`, a consumer). Its
  `tick()` does serialize → deserialize → per-key hash compare every tick — a real determinism
  check. It is `pub(crate)` under `#![cfg(test)]`, so an external integration-test crate cannot
  reach it; a scenario runner lives inside the crate or the visibility changes.
- Open tickets: `sov-scope-cut-1p6` (P1 gate), `sov-journey-sentinels-rxa`,
  `sov-scenario-coverage-bt0`, `sov-harness-lots-k54`, `sov-harness-perf-km4`.

## Decisions that still bind

- **Single rouble for 1.0.** Charter beats `spec/trade.md:26-31`. STORY-0054 carries the
  deferral; captured, not deleted.
- **Fix-everything disposition.** All 172 PAR findings remediated, numeric constants kept as
  ACs (water quality 0.93/0.97/0.60, hospital beds 100, serve-rate 3, seats StudentCount×5/4,
  route modifiers ×7.5/×0.95). Do not "simplify" these back out.
- **No `JOURNEY-NNNN` from extraction** — the taxonomy needs a `spec/journeys/` dir this
  project lacks. JOURNEY-0001 was authored by `scoping-the-simplest-core`; sentinels are
  JOURNEY-0001 + SCENARIO-0009/0015/0090/0115/0118.
