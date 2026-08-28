---
name: sweep-practice-blocks-2026-08-28
description: Sweep of the ~3094-line engineering-practice blocks on branch docs/agent-practice (054a0cf, 60c5131) in worktree /home/caio/sov-agents-wt — 12 confirmed-wrong claims, the verified-exact list, and two tooling traps discovered during the sweep.
metadata:
  type: project
---

Tree: worktree `/home/caio/sov-agents-wt`, branch `docs/agent-practice` @ `60c5131`,
`main` @ `073b123`. 22 agent files + 2 docs. Ran without LSP, without Grep/Glob.

## Two tooling traps found DURING this sweep — both new, both load-bearing

1. **`ct search --base .` does not descend into dot-directories.** Proven twice:
   `ct search --grep "spectacle" --base .` → exit 1 (absence) while the string is in
   `.claude/agents/ui-implementer.md`; `ct search --grep "sov-bo3" --base .` misses
   `.beads/issues.jsonl`. Pointing `--base` at the dot-dir finds them. So a `ct search`
   exit 1 from the repo root is **not** proof of absence for `.claude/`, `.beads/`.
   The shared block tells 22 agents that ct's exit 1 IS trustworthy. It is only
   trustworthy for non-dot paths.
2. **`.claude/workflows/gate-review.js` is gitignored** (`.gitignore:2` = `.claude/*`).
   It exists only in `/home/caio/soviet-simulator`, never in a worktree or clone. The
   shared judge block cites `gate-review.js:31,161` to every gate agent. Both cited lines
   are exactly right in the main checkout — and unreachable everywhere else.

## Confirmed wrong (this sweep)

1. "six `get_lua(..).unwrap_or(d)` sites" → **five**. `base.rs:17, item.rs:24,
   recipe.rs:63, zone.rs:20, zone.rs:21`. Never was six: 4 before `0caee71`, 5 after.
   Same wrong 6 sits in `.planning/architecture-review-2026-08-27.md:87`.
2. "`cargo test --lib` is about half a second" → true for geom (0.22 s incremental),
   **12.4 s runtime / 13.5 s cycle for `-p simulation`**, where most mutations happen.
3. "taking the only record of the leak with it" → the record survives in sov-2c4's
   description and `.claude/agent-memory/debugger/idle-truck-blocks-lane.md`.
4. `ToSource`/`Some(v)` "its only exit is the vehicle entity vanishing" → two exits:
   entity gone AND arrival (`it.has_ended`), market.rs:876-900. Faithful quote from
   sov-6qx, but the quote was conditional on removing the market.rs:783-786 guard.
5. soviet-authenticity: "the phrase … exists in no file under `docs/`" → the substance IS
   at `docs/reference/art-direction.md:10-14`. Only the definition's quotation marks are
   false (paraphrase presented as a quote). "Gritty, weathered, materially honest" is the
   part that appears nowhere.
6. `RenderParams` is `engine/src/gfx.rs:204-223`, not `183-202`.
7. `TransparentHasherU64::write` is `common/src/hash.rs:67`, panic at `:68`, not `:70`.
8. geom OrderedFloat "11 sites" → **12** usage sites (imports excluded); the block's own
   list names 9, missing `polygon.rs:357`, `skeleton.rs:1052`, `skeleton.rs:1123`.
9. goryak "22 files" → **21** `.rs` files.
10. ui-implementer "34 resource types registered in init.rs:35-73" → **37** calls, one
    `#[cfg(feature = "multiplayer")]`, so **36** by default.
11. "market.rs:1127 divides by qty" → the `/ qty` is `market.rs:1189`; the `dbg!` is
    `:1185`. Test fixtures are `market.rs:1229` and `:1268`; `:1168` is wrong (`:1207` is
    the `use prototypes::test_prototypes;` line, so that half holds).
12. `settlement-modeller.md:50` `human.rs:267-269` → real is **`human.rs:272`**;
    `market.rs:216` is `dispatches: Default::default(),`. (Same defect my 2026-08-27
    sweep already reported — line numbers have drifted again since.)

## Live tracker/reality contradictions found while sweeping (pre-existing, outside the diff)

- **`sov-1ae` is CLOSED — cancelled by the user 2026-08-27, the 250k bench lane is
  DROPPED.** Six documents still call it OPEN: `perf-engineer.md:41`,
  `settlement-modeller.md:91`, `common-implementer.md:40`,
  `development-cycle.md:265`, `.planning/agent-roster-review-2026-08-27.md:97-98`,
  `HANDOFF-2026-08-27-tooling-wave.md:92`.
- `perf-engineer.md:3` **frontmatter still says "the charter's five benchmarks"** — the
  exact claim roster-review B3 told us to drop. Its body is corrected; the description,
  which is what the router reads, is not.
- **The roster is 22 agents.** `CLAUDE.md:25`, `doc-reality-auditor.md:63` and
  `development-cycle.md:59` all still say 16. All 22 pin `model: opus` (no effort field).
- `test_world_survives_serde` still has **no assert on main** (`test_iso.rs:242-306`);
  the fix lives only on unmerged `fix/sov-myg-determinism-guard` (`7fa08e8`) while
  sov-myg is CLOSED.

## Verified exact — do not re-check without cause

- Shared block is byte-identical in 13 non-writer copies and byte-identical in 9 writer
  copies; the single difference is the intended "Restraint fires once" paragraph. No drift.
- `prototypes/src/prototypes/goods_company.rs:41-42` correct `get_lua_opt(..)?` form.
- `simulation/src/init.rs:233-240` log-and-default; `lib.rs:389` + `:439` `Ok(sim)` regardless.
- `networking/src/catchup.rs:39` logs, `:41` pushes anyway.
- sov-bo3: 17.6 GB RSS, OOM kill, reach from `MapBuildSpecialBuilding`. Fixed (`2cc7331`);
  bound `vs.len()+1` present at `skeleton.rs:744`.
- sov-abc: "Three unit tests asserted the illegal values 16/48/128" — verbatim close reason.
- `engine_demo/tests/capture_contract.rs:60` = the literal-constructing test.
- `cargo test -p simulation sentinel` → `running 0 tests`, `49 filtered out`, EXIT=0.
  Documented at `simulation/src/tests/scenarios/mod.rs:8-10`.
- Graph `callers_of unpark` = **0** (reproduced live, head_matches_build false) vs three
  production callers: `market.rs:853`, `router.rs:218`, `world_command.rs:359`.
- `e27a068` = the park fix; market.rs 77+/29- = 106 changed lines; commit total 88/31;
  cites sov-2c4 + sov-7pg. "38,000+ ticks" is in
  `.claude/agent-memory/debugger/idle-truck-blocks-lane.md:9`.
- `git log -100` → zero commits reverting an abstraction for complexity.
- Zero `ponytail:` markers in any `.rs`/`.lua`; the truck one was deleted by `e27a068`.
  Plugin absent from `installed_plugins.json` and from `enabledPlugins`; the name survives
  only as an `extraKnownMarketplaces` entry at `~/.claude/settings.json:212`.
- `gate-review.js:31` = severity enum, `:161` = CONFIRMED/PLAUSIBLE/REFUTED. Exact.
- `Market::set_requested` now has a production caller at `goods_company.rs:24`.
- perf numbers: 31.6% / 29% / ~1% ssao spread / 2.3x→0.26x / `validation_requested: false`
  stale-record PASS — all verbatim in `.beads/issues.jsonl` (sov-91s, sov-uuo, sov-dda.3).
- release: "3428 identical lines" and "exit code 8" in
  `.claude/agent-memory/release-engineer/verification-procedures.md:17,34` and
  `docs/process/dependency-policy.md:251,277,285`.
- validation messages: 15 → 10 on RX 7800 XT / RADV NAVI32; 5 are `vkGetDeviceProcAddr`.
- bincode-1.3.3 `config/mod.rs:15-18` table, `lib.rs:106-114` fixint+allow_trailing; one
  direct `bincode::` use, `common/src/saveload.rs:94`.
- networking: `server_playout.rs:85`, `connections.rs:129,147,164,173`,
  `connection_client.rs:90,108,117,125`, `network.rs:118-119`, 2055 lines. All exact.
- goryak panic sites `roundrect.rs:102`, `scroll.rs:152,388`, `progress_bar.rs:65`,
  `imagebutton.rs:163`, `interact_box.rs:112`, `sized_canvas.rs:86`. All exact.
  egui-inspect-derive panics `68,115,190,210`, unwraps `11,121,135,138`. All exact.
  goryak has 0 tests. yakui rev `6c6982f` (`Cargo.lock:3889`).
- sim: BTreeMap sites `dispatch.rs:10`, `map_dynamic/electricity.rs:8`, `ecostats.rs:1`,
  `spatial_map.rs:10`, `economy/mod.rs:19`; `resources.rs:15` FastMap; `lib.rs:268-276`
  hashes bytes; `par_command_buffer.rs:50-83`; `lib.rs:404-415` warn-only version gate;
  `headless/src/main.rs` 80 lines; rustc-hash 1.1.0 AND 2.0.0 both in Cargo.lock.
- The connected_road lesson IS real and I nearly mis-refuted it: `connected_road: None`
  gives a building its OWN single-node network with zero producers → blackout → 0.0.
  Documented at `simulation/src/tests/scenarios/inflation.rs:115-131`. Cross-check saved it.
- `prototypes/src/validation.rs` checks n_trucks, referenced item ids, power sign only.
- ui: `native_app/src/init.rs:85-86` static muts; sov-odw "running 3 tests", setsid,
  yakui pointer-MOTION — all verbatim in the tracker.
- sov-abs is IN_PROGRESS (not closed) → "still live" holds.

## Unverifiable at primary source (no repo/tracker artifact)

"seven of nine code lanes" (proxies give 5/9 or 9/9, never 7/9); the complexity-section
evidence (one-file test fix → nothing; renderer branch → six nits, −174 lines); "four
separate agents hit false zeros in one day"; the `spectacle` / `ffmpeg x11grab` capture
recipe; "four tickets" for the ToSource shape (a defensible four is sov-2c4/7pg/jcl/ahw,
a broader read gives six); Bosu et al.; Porter 1995; the LLM-judge anchoring study.

Related: [[sweep-agent-bodies-2026-08-27-b]], [[sweep-agent-roster-2026-08-27]],
[[generated-artifacts-and-generators]].
