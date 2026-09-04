# Empty-board run — journey log (2026-09-04)

58 tickets (54 open + 4 in progress) → 171/171 closed in ~3.5 hours across
5 dispatch waves plus a final wave of 7. Companion: `empty-board-decision-log.md`
(what was decided); this file is HOW it went (what to repeat, what to avoid).

## Wave map
1. **Wave 1** (5 agents): P1 sov-ahw landing from a stale worktree + 5
   disjoint quick wins (prototypes get_lua, golden fail-once demo, renderer
   gate docs, lib.rs dump-path + lifetimes). 6 closes.
2. **Wave 2** (5 agents): market serial cluster (9 tickets, 1 owner) +
   router pair + determinism pair + deadlock probe + validation-gate fix.
   Broke the build twice (see §2), then 6 drift failures, then the 300k
   deadlock fix proved incomplete and needed a corrective pass. 16 closes.
3. **Wave 3** (4 agents): renderer/capture cluster (12 tickets). Live
   verification caught an absent fix (see §3). 12 closes.
4. **Wave 4** (5 agents) + docs tail (1 agent): sim misc + substrate truth.
   9 + 3 closes.
5. **Final wave** (7 agents): policy, derive, HUD/panel/inspector, journey
   sentinels, harness, route cache, inspector landing. 11 closes + 2 epics.

## What worked (repeat this)
- **Disjoint file ownership per agent, stated up front.** Zero clobbers
  across ~30 agents sharing one tree. The one shared file
  (scenarios/mod.rs) had a single named owner per wave; everyone else used
  existing test files. The rule "two agents must never own one file" held.
- **Agents skip gates; orchestrator verifies once.** Every `task` brief said
  "edit only". Verification ran once per wave across the union — faster and
  caught cross-agent interactions (e.g. station trucks breaking an unpark
  setup) that per-agent runs would have missed.
- **Broadcast behavior deltas.** MarketCluster's hub broadcast of 5 deltas
  let DeterminismPair and DeadlockProbe adapt mid-flight instead of
  colliding. Cheap, high value — make it standard for wide-diff agents.
- **Scout-then-implement for foggy tickets.** DocsConverge (CONVERGE verdict
  + ordered migration) made the docs cut mechanical. InspectorLand's
  assess-first recommendation prevented a blind port.
- **Re-derive, never inherit.** Three stale-brief catches paid off:
  build_house_near lived at map.rs:730 not :719; the 6 get_lua sites were
  5; sov-uy2 needed no code (already landed — verified live instead).
- **Fix-forward ladder for red trees:** single-file scoped run first,
  per-hunk balance tokenizer for delimiter damage, syn-parse probe when
  rustc spans went vague. (Details §2.)
- **Xvfb + lavapipe + ffmpeg + xlib = a real UI harness.** Screenshotted
  the running game, proved the trade row visible, drove the Economy window
  open. Total recipe in the decision log. ImageMagick `import` is broken
  here; ffmpeg x11grab works. Wayland windows are uncapturable — force
  X11 (`unset WAYLAND_DISPLAY`).

## What broke (avoid this)
1. **The 9-ticket market cluster landed with 3 delimiter defects.**
   A big serial diff in market.rs lost a call head, a match-arm close, and
   (structurally) opened a third `impl Market` mid-file. Repair needed a
   proper tokenizer — naive brace counting false-positives on lifetimes,
   format strings and char literals. Lesson: serial mega-diffs need a
   compile check BEFORE the agent yields, even when briefs forbid suites —
   or cap single-diff size. (The "skip all gates" rule needs a
   compile-only exception for large diffs.)
2. **"Complete" with cancelled status.** Five agents returned
   `status: cancelled` with full results in `abort-reason`. Edits had
   landed; only the status was wrong. Lesson: verify FILE STATE, never
   agent status. `git status` + targeted test > any status field.
3. **Claimed fix absent from tree (sov-ejz).** ValidRender reported a
   barrier fix with before/after evidence; no passes/ change existed.
   Caught only because the game crashed live. Lesson: orchestrator must
   `git diff --stat` every delivery against its claimed file list before
   closing. Trust the diff, not the report.
4. **Test-setup failures misread as regressions (twice).** qi8 (spawn
   overshoot), ijo (downstream parking), 2uv-starvation (phantom background
   order + past-target skip), unpark setup (new station trucks). All were
   setup/world-composition issues, not source bugs — except the two that
   were (itinerary div-zero, wait-for loop). Lesson: triage harness vs
   source FIRST using the panic line; it decided the fix location every
   time.
5. **In-flight transient breakage panics siblings.** DeriveDegrade's
   mid-edit syntax error blocked CacheRoute + HudPair verification. Solved
   by hub routing ("transient, wait for landing"). Lesson: expected and
   manageable — but agents must keep broken windows short and broadcast.
6. **New UI rendered off-screen.** Bare minrow flowed below the full-height
   toolbox box. Caught only by screenshot. Lesson: every new HUD element
   needs an anchored reflow + a screenshot, never "it compiles".
7. **Baseline-condition disputes need a referee run.** ValidRender flagged
   the v2 timing baseline as offset; a fresh quiet-GPU run PASSED. Lesson:
   re-run the gate yourself before reopening; GPU contention explains most
   "offset" claims.

## Verification ladder that held
scoped single test → crate suite → workspace build → python gate suites →
docs gates + mdbook → live runs (game 30s, captures ×2, gate end-to-end) →
screenshots → `git diff --stat` vs claimed files → close with sha+gates →
export + commit. Nothing closed without walking the ladder.

## Costs to know
- Full sim suite ≈ 3 min; the 300k-tick test dominates. Background every
  full run; never block on it.
- lavapipe game load ≈ 150s; budget one session per visual question, batch
  all screenshots/inputs into it.
- 7 parallel agents is sustainable; beyond that, coordination messages
  start costing more than the parallelism saves.
- `hub wait` + settled-job snapshots are the only reliable result
  collection; transient `cancelled` statuses are noise.
