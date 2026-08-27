---
name: sweep-agent-bodies-2026-08-27-b
description: Second 2026-08-27 sweep — the 16 agent PROMPT BODIES (not frontmatter) at HEAD 4084518. What was wrong, what was verified clean, and the standing duplicate-block map.
metadata:
  type: project
---

Tree state: HEAD `4084518`, clean. Scope was prompt bodies only; frontmatter was another
reviewer's lane. Ran with **LSP absent from the toolset** (`ToolSearch("select:LSP")` →
"No matching deferred tools found") — fell back to `grep -n`.

## Confirmed wrong (top of the report)

1. `doc-reality-auditor.md:47` — "sonnet implements, opus reviews" contradicts the
   2026-08-27 opus/high pin. **My own definition.** Same defect class as the BEADS_ACTOR
   exemplar.
2. `perf-engineer.md:21` + `settlement-modeller.md:74` + `development-cycle.md:52,238` —
   "the charter names five bench gates". `grep -n "bench" docs/plan/charter-1.0.md` → nothing.
   Charter:56-57 says the *implementation and release plans* define them. No `[[bench]]`
   target, no `benches/` dir. `sov-1ae` is open to build the first one.
3. `evidence-auditor.md:92` — `evid-spec-bindings.json` is NOT in `docs/generated/evidence/`.
   It lives at `docs/plan/iterations/evidence/evid-spec-bindings.json` (generator INPUT,
   deliberately kept under `docs/plan/` — see [[generated-artifacts-and-generators]]).
4. `logistics-modeller.md:88-92` — lists `sov-dispatch-wedge-ab4` under "Known OPEN problems"
   and tells the advisor to answer its design question. Closed 2026-08-26, commit `7e4b82f`,
   answered "Option C". Live follow-ups are `sov-jcl` / `sov-xyx` / `sov-abs`.
5. `utilities-modeller.md:80-82` — the pasted grep for weather returns **5** hits, not zero
   (all surnames in `souls/names.txt`). Conclusion right, stated premise false.
6. `soviet-authenticity.md:56-58` — quotes art-direction as "Gritty, weathered, materially
   honest". That string is in no file under `docs/`.
7. `settlement-modeller.md:34` — `human.rs:267-269` / `market.rs:216` are both the wrong lines.
   Real: `human.rs:272` (`m.buy(... "job-opening" ...)`), `market.rs:216` is `fn m()`.
8. `settlement-modeller.md:67-70` + `utilities-modeller.md:74-77` — "numeric constants the
   requirements pin". The named requirement cards are 89-line prose contracts with **zero**
   such numbers. The numbers are from the archived legacy corpus.

## Verified clean — do not re-check without cause

Every W&R corpus grammar count in every body is **exact**: `$STORAGE` 314, `$WORKERS_NEEDED`
156, `$CONSUMPTION` 146, `$PRODUCTION` 89, `$CITIZEN_ABLE_SERVE` 53, `$TYPE_LIVING` 54,
`$VEHICLE_STATION` 558, `$VEHICLE_PARKING` 359, `$CONNECTION_ROAD` 397,
`$CONNECTION_ADVANCED_POINT` 2180, `$CONNECTION_ROAD_DEAD` 1451, `$CONNECTION_WATER_DEAD` 218,
`$STORAGE_EXPORT` 81, `$STORAGE_IMPORT` 75. Count them with
`grep -rhcE "^\$TOKEN([[:space:]]|$)"` — a bare prefix grep triple-counts `$STORAGE`.

Also exact: `road.rs:55-58`, `vehicle.rs:107` unpark, `router.rs:217` cbuf, `market.rs:441`
set_requested, `market.rs:497` make_trades, `market.rs:501` "Naive O(n²)", `market.rs:280/281`
reserved/requested, `goods_company.rs:24` set_requested call, `map.rs:719` generate_along_road
(guarded by `gen_lots`, false only for `RoadSegmentKind::Arbitrary`), `native_app/src/init.rs:85-86`,
`companies.lua:40`=4 / `:582`=3, `freight_station.rs:76/132/145-148`, `dispatch.rs:61-70`,
commit `35ce342`, `electricity_cache.rs:430/465` tests, Cargo.toml git deps, `TestCtx`
`pub(crate)` under `#![cfg(test)]`, `.codex/agents` = 15 toml + no reviewer.toml.
Line counts: simulation 17,764 / native_app 58 files 10,085 / base_mod 949 / prototypes 2,802 /
assets 97 PNG + 31 wgsl / screenshots 2,580. `base_mod/colors.lua` DOES hold `gui_*` colours.
No Bevy/Godot/pre-fork reference in any body.

## The duplicate-block map (check these for drift first next time)

| Block | Copies | Known drift |
|---|---|---|
| LSP preamble | 13 of 16 | `wiring-auditor:38`, `substrate-cartographer:99` carry an older variant with NO warm rule; `soviet-authenticity` has no LSP at all (correct) |
| `cargo test -p simulation` + static-mut + `init.rs:85-86` | 9 | none found |
| W&R path + counts | 6 | "1,472 **`.ini`** files" — real: 1,472 total files, **488** `.ini`. Only `substrate-cartographer:55` says "1,472 files" correctly |
| `optout_exttrade` 1-of-21 | 6 | none |
| `TestCtx::tick()` limits | 5 | `logistics-modeller:107-108` treats it as a determinism proof; `evidence-auditor:69` and `debugger:84` explicitly deny that |
| truck-vs-train substrate | 3 | `goods_company.rs:129` is now **:132** in 2 copies |
| sentinel-command anecdote | 3 | told in PAST tense — the defect is **still live** (see below) |

## Standing traps

- **`cargo test -p simulation sentinel` still runs 0 tests** (`45 filtered out`, exit 0).
  `simulation/src/tests/scenarios/mod.rs:8-10` still documents it. Both gate bodies cite it
  as history. It has never been fixed. `sov-journey-sentinels-rxa` is the open ticket.
- **`engine/` (12.5k), `geom/` (10.5k), `networking/` (2.1k), `common/` (1.3k) have no
  implementer lane** in Phase 2 or in any body. `bc555d9` edited engine + common anyway.
- **`sov-qi8` (open)** says the `TestCtx` determinism check is unstable on `transport_grid`
  (FnvHashMap order). Five bodies describe what that check proves; none mentions the bug.
- `code-intelligence.md:52` claims "Every agent definition in `.claude/agents/` already says
  this" about the LSP warm rule — true for 13 of 16.

Related: [[sweep-agent-roster-2026-08-27]], [[sweep-uncommitted-docs-2026-08-27]],
[[generated-artifacts-and-generators]].
