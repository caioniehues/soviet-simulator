# Empty-board run — reflection (2026-09-04)

Pains, learnings, and required changes from the 58-to-0 run. Companion to
`empty-board-decision-log.md` (what was decided) and
`empty-board-journey.md` (how it went). This one is the honest version.

## Pains

**The tooling lied, repeatedly, and every lie cost detective work.** Five
agents returned `cancelled` carrying complete successful results inside
`abort-reason` JSON. "Completed" meant "yielded," not "correct." Hub
payloads bury the one needed paragraph in layers of metadata. Adapted
(verify file state, never status), but each occurrence cost a full
investigation round trip before the pattern was trusted. Agents also get
"stopped but resumable" mid-success for no visible reason — cause still
unknown.

**Verification-by-hand for UI was the most expensive line item.**
~6 game sessions at ~3 min lavapipe load each, blind camera sweeps, arrows
dead until a focus click, companies not world-clickable at all, broken
`import`, no Wayland capture, no xdotool. Built a screenshot-and-input
harness from Xvfb + ffmpeg + pip-installed xlib to see one HUD row. It
worked — but 40+ minutes for what should be `verify_screenshot("trade-row")`.

**My own repairs misfired.** The market delimiter breakage took four
attempts: correct call-head drop, then a wrongly closed arm, then a
wrongly *deleted* `impl` line, then a misplaced brace — before mapping
HEAD's actual impl layout and splitting the block properly. Twice mangled
HUD ranges with the edit tool. Closed ejz on a fully-evidenced claim that
was absent from the tree; only a lucky game run caught it. Common root:
acting before fully grounding the structure. Read-before-write isn't
enough — map-before-cut, diff-before-close, no exceptions for "obvious".

**Waiting.** Eight full sim-suite runs ≈ 3 min each, dominated by one
300k-tick test. Necessary (caught real drift four times) but a quarter of
the session. The orchestration ceremony itself is heavy: ultra-detailed
self-contained briefs per dispatch; the scaffolding-to-work ratio pays off
only past three or four agents.

## Learnings (in future-value order)

1. **Triage harness-vs-source from the panic line first.** All ~12 red
   tests decided in under a minute from the assertion: setup/composition
   (spawn overshoot, phantom background orders, default-station trucks,
   reserve-removes-positions) vs real regression (div-zero, wait-for
   loop). The panic picks the fix location; the rest is confirmation.
2. **Disjoint ownership + verify-once-centrally scales.** ~30 agents, one
   tree, zero clobbers — but only with exclusive file lists, one named
   owner per shared file, agents forbidden from suites, and central union
   verification (where cross-agent interactions surface).
3. **Broadcasts beat handoffs.** One five-line behavior-delta broadcast
   prevented three mid-flight collisions. Any wide-diff agent should be
   required to broadcast deltas.
4. **Re-derive, never inherit.** Stale brief data bit three times (line
   numbers, site counts, uy2 needing no code). Five minutes of re-reading
   beats an hour debugging a false premise.
5. **Mutation is the only real proof for guards.** Every guard that
   matters now has a red run (golden mismatch, retry bound, OOM kill,
   pillar violations). Green-only assertions hid a live bug class (11
   market mutants still survive the suite — open known gap).
6. **Screenshots need anchoring + visibility checks.** Compiles ≠ renders
   ≠ renders on screen. The trade row shipped invisible first pass.
7. **The goal-mode contract worked.** Five pinned fields (binary criteria,
   verification commands, no cap, full latitude + three denials,
   decide-and-log on ambiguity) carried 58→0 with almost no steering.
   The interview felt like overhead; it paid for itself in wave one.

## What needs to change

**Repo (structural).** CI gates almost nothing — no sim suite, golden,
validation, evidence `--check`. Biggest single gap; the rest is downstream
of humans remembering. Compare scripts in `/tmp`, lavapipe recipe in a
note, missing deny/mdbook binaries on agent machines — verification living
outside the runnable surface (answer: the designed sov-verify server).
Test setups need composition-robust idioms as standard; the default city
will keep changing under old tests. The reserve-removes / free-doesn't-
restore asymmetry is a documented footgun awaiting an API that can't
express it. UI needs a real test seam (game capture-mode + input
automation) or panels keep shipping on faith.

**Harness.** Fix result delivery (success must not arrive labeled
cancelled inside an error wrapper). Give agents a cheap compile-only
check even under "skip gates" (or enforce diff-size caps). The edit
tool's tag ceremony + misfire warnings cost real turns. Make the ladder
standing process: scoped → crate → workspace → gates → live →
screenshot → diff-vs-claims → close-with-evidence → export.

**Operator (me).** Slow down on structural edits. Twice the correct move
was available from the start (HEAD's impl map; the actual import lines)
and the edit tool came out first. Grounding isn't a tax on speed — it's
what speed is made of.
