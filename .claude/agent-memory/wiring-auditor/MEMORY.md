# wiring-auditor memory — soviet-simulator

## Wiring points (where to check registration)

- **System registration**: `simulation/src/init.rs`, function that calls `register_system("name", fn)`.
  Grep `register_system(` there for the full list. Confirmed present as of b3857f5:
  `register_system("dispatch_system", dispatch_system)` and `register_system("market_update", market_update)`.
  A new per-tick system that isn't in this list never runs, no matter how correct its body is.
- **Resource registration**: same file, `register_resource_default::<T, Codec>("name")`. Check this
  when a diff adds a new `Resources`-held type (e.g. `Dispatcher`) — confirmed present for `Dispatcher`
  ("dispatcher").
- **Dispatcher (map_dynamic/dispatch.rs)**: `Dispatcher::update()` is the only place entities become
  queryable via `dispatcher.query(...)`. It is called from `dispatch_system`. A vehicle/train kind not
  registered there (or gated behind a `VehicleKind` match that excludes it) is invisible to every
  caller of `dispatcher.query`, even though the caller code is otherwise correct. This was literally
  a `/* ... */` commented-out block for trucks before commit `35ce342` (fixed in-range for this audit).
- **Economy loop**: `economy::market_update` (registered system) drives `Market::make_trades` and, as
  of b3857f5, `Market::advance_dispatches`. Any new Market mechanic must be reached from inside
  `market_update` or one of the functions it calls to be live.
- **Recipe path**: `souls/goods_company.rs` `recipe_init`/`recipe_act` are called from the goods-company
  soul desire/update logic in the same file (grep `recipe_init(\|recipe_act(` there for call sites).
  These are always production-reachable; the question for a diff touching them is usually "is the new
  *input* to their logic (a market field, a config) ever populated," not "are they called."
- **UI/observability**: `native_app/src/gui/hud/windows/economy.rs`, `native_app/src/gui/inspect/inspect_building.rs`,
  `native_app/src/debug_gui/debug_inspect.rs` are the existing panels that read `Market`/`capital`.
  As of ITER-0000, none of them read the new `Market::requested`/`Market::dispatches` — the sim-side
  feature has no in-game observation surface yet. Not necessarily a defect (iteration may be sim-only
  by design) but worth naming every time a story's own wording claims "the planner/player can observe X."

## Recurring failure shapes (seen 2+ times — expect them again)

1. **Public setter with zero production callers, only a test scenario calls it.** Pattern: function A
   writes a value, function B reads it with `.unwrap_or(default)`, B is production-reachable, A is not.
   Net effect: B's fallback branch always fires in the live game, behavior is byte-identical to before
   the diff, and every test that manually calls A passes. Seen: `Market::set_requested` (b3857f5) — same
   shape as the canonical `Market::set_requested`... er, as the `set_requested`/`recipe_act` example
   this agent's own role doc opens with (a *different* commit, same shape, same function name even).
   Grep every new `pub fn set_*` / `pub fn insert_*` style setter for callers outside `tests/` on sight.
2. **Documented test-filter command matching zero tests.** A doc comment (module-level `//!` or a
   README/story) says "run `cargo test ... <substring>` to execute the sentinel/tagged set," but no
   `#[test] fn` in the tree actually contains that substring. `cargo test <filter>` exits 0 with
   "0 passed; 0 filtered out" — a green result that looks identical to a real pass. Seen twice now
   (this agent's origin story, and again in `simulation/src/tests/scenarios/mod.rs` at b3857f5, which
   names a 6-ID sentinel set — JOURNEY-0001, SCENARIO-0009/0015/0090/0115/0118 — none of which exist
   as test fn names anywhere in the tree; the actual new tests are scenario_0082/0083/0093-0097/0151).
   Always literally run the documented command with `--no-run` skipped (i.e. actually run it) and read
   the "N filtered out" / "0 passed" line, don't just grep for the string in isolation.

## Fixed instances of the recurring setter-with-no-caller shape

- `Market::set_requested` (the exact symbol named in this agent's own origin story) got a real
  production caller in the sov-lpj diff: `recipe_init` (souls/goods_company.rs:21-24) now calls
  `market.set_requested(soul, item.id, item.amount * recipe.request_multiplier)` directly, and
  `recipe_act` (line 52-55) changed its read from `.unwrap_or(item.amount)` to a bare `.unwrap()`
  — the fallback that made the bug invisible is gone, so a missing `set_requested` call would now
  panic instead of silently no-op. `recipe_init` is called from the real `company_soul` spawn path
  (goods_company.rs:173) and `recipe_act` from `company_system`, which IS registered
  (`init.rs:65`). New `Recipe::request_multiplier` (prototypes/src/types/recipe.rs) parses with
  `.unwrap_or(1)` so all pre-existing recipes stay byte-identical (honest) by default — confirmed
  by mutation: forcing `base_mod/companies.lua`'s `request_multiplier = 4` back to `1` makes
  `sov_lpj_flour_factory_hoards_bounded_no_freight_station` fail. Lesson: the same setter symbol
  can move from TEST-ONLY to REACHABLE across commits — always re-check current callers, don't
  assume a name flagged once stays flagged forever.
- New tests in `simulation/src/tests/scenarios/inflation.rs` deliberately avoid the hand-simulated
  pattern from `hoarding.rs` SCENARIO-0151 (calling `Market::produce`/`buy_until` directly) — they
  spawn a company through `build_special_building` + a tick so `company_soul`/`company_system` run
  for real. This is the correct way to prove a recipe-path feature is live; check for it as the
  bar when a new scenario claims to test soul-level wiring.

## Hazard: never `git checkout -- <file>` on a file inside the audit scope

Mid-audit on 2026-08-26, a mutation-test cleanup used `git checkout -- base_mod/companies.lua`
meant to *restore my temporary edit*, but the file had uncommitted working-tree changes as part
of the diff under audit (not yet committed) — `git checkout` silently reset it all the way to
`HEAD`, deleting both `request_multiplier` lines from the actual diff being reviewed, not just my
mutation. Recovered by re-adding the two lines by hand and diffing against `git show HEAD:<path>`
to confirm exact restoration. **When mutation-testing an uncommitted working-tree diff, restore
by hand-editing back to the known-good state (or `git stash`/reapply), never `git checkout --`,
which resets to the last commit and destroys any uncommitted target changes along with your own.**

## Notes on scenario tests specifically

- `simulation/src/tests/scenarios/hoarding.rs` scenario_0151 does NOT exercise `recipe_init`/`recipe_act`
  end-to-end — it hand-simulates the honest/inflated consumption loop by calling `Market::produce`,
  `Market::requested`, `Market::buy_until` directly, bypassing the actual soul/desire system entirely.
  It proves the *Market* mechanics work; it does not prove any real in-game company ever inflates its
  request, because nothing populates `requested` outside that test. Check whether a "proves the feature"
  test actually drives the production entry point, or reimplements it inline.
