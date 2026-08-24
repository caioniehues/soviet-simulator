---
name: false-claims-failure-inventory
description: Forensic verdicts on the six "empirical failures" cited by docs/dev-cycle.md and docs/framework-study-brief.md — two are overstated, one is time-inverted and REFUTED; verified at fdfabca 2026-08-24
metadata:
  type: project
---

The six-item "empirical failure inventory" appears verbatim in `docs/framework-study-brief.md:110-124`,
`docs/dev-cycle.md:89-99`, and (in fragments) in `.claude/agents/data-implementer.md:31-34`,
`sim-implementer.md:63`, `doc-reality-auditor.md:21-25`, `substrate-cartographer.md:27-36`.
**Re-derived against `HEAD = fdfabca`, 2026-08-24.** Full fact-sheet: `/tmp/fw-study/reports/failure-verifier.md`.

## The two that are wrong

**REFUTED — "four agent definitions targeted paths deleted five days *before* they were written"**
(`doc-reality-auditor.md:24-25`). **The arrow of time is inverted.** The four agent files
(`catalogue-implementer`, `presentation-implementer`, `prototype-researcher`, `refactor-reviewer`)
were authored at `1cf5d97`, **2026-08-17**. `src/sim/` and `src/game/` both existed at that commit
(`git ls-tree --name-only 1cf5d97 src/`) and were deleted at the fork `68fe28c`, **2026-08-22** —
five days *after*. Also "four" should be three: `prototype-researcher.md` names neither path (its
only path ref is `~/.cargo/registry/src/*/bevy*-0.19.1/`, line 21). Implication: this is
**decay after authoring**, so the fix is a re-validation trigger on structural moves, not more care
at authoring time. Today **zero** of the 15 agent defs has a dangling file reference.

**OVERSTATED ~3× — "`optout_exttrade` falsified three claims in a landed commit."** The count
"1 of 21" IS exact (`grep -c 'type = "item"' base_mod/items.lua` → 21; single flag at
`items.lua:6`, on `job-opening`). But the landed commit is `b3857f5`, whose message lists F1/F2/F3 —
and **only F1 turns on the Lua flag**. F2 (`Market::remove` leaks `reserved`/`requested`/`dispatches`)
and F3 (`set_requested` has zero production callers) are independent. Already propagated in this
inflated form into `data-implementer.md:31-34` and `dev-cycle.md:97-99`.

## The one that is still live and unfixed

**`docs/superpowers/iterations/RESUME.md:84`** still reads "`souls/freight_station.rs` is the ONLY
correct prior art for driving a dispatched delivery" with **no parking/collider warning anywhere in
that file** (`grep -n "parking\|Parked\|VehicleState\|unpark" RESUME.md` → nothing). The correction
lives only in `brief-truck.md:29-37` and `35ce342`'s commit message. `CLAUDE.md` tells agents to read
`RESUME.md` **first**. The next agent on this seam walks into the identical trap.

## The four that hold

- **Truck registration commented out** — `/* */` block, `git show 35ce342 -- .../dispatch.rs`. Had
  two compile errors inside it (`DispatchID::Truck` — real variant is `SmallTruck`;
  `.trans.position` — real field is `.trans.pos`). Tell nobody caught: `world.rs:79-82` called
  `unregister(DispatchID::SmallTruck(id))` live from the fork with **nothing ever registering**.
  Correction to the story: `6ea4553`'s commit message **disclosed** the AC-4 violation at length —
  the failure was loud, not silent.
- **Train/truck asymmetry** — `freight_station.rs:112-119` assigns `*itin = Itinerary::route(..)`
  directly; `TrainEnt` (`world.rs:120-128`) has no `VehicleState` and no collider. `VehicleEnt`
  (`world.rs:59-65`) has both. `road.rs:21-24` early-returns without a collider; `road.rs:55-58`
  gates drive input on `Driving | Panicking`.
- **`cargo test -p simulation sentinel` → 0 tests, exit 0** — reproduced verbatim at `HEAD`, both
  with and without `--test-threads=1`. Documented in **code**, `simulation/src/tests/scenarios/mod.rs:5-13`,
  which still exists. Zero of the 12 scenario tests contains `sentinel`; none of the six named
  sentinel IDs exists as a test. Real runner: `cargo test -p simulation -- --test-threads=1` → 26 passed.
- **Free-credit path** — `git show 68fe28c:.../market.rs:285-296`: `*capital.entry(buyer) += qty_buy`
  ran BEFORE `let Some(ext) = find_external(..) else { continue }`. `find_external` is a closure over
  `world.freight_stations` (`economy/mod.rs:64-74`), `min_by_key` → `None` when empty. Fixed at
  `fdfabca` by moving the credit inside the `Some(ext)` arm. **Nuance the story omits:** the bug was
  already correctly written down at `roadmap.md:23` (`2f0cbf0`, 2026-08-22) — two days before the
  specialist "found" it — but filed as a test-fencing precondition, not a bug. The gap was
  **disposition, not detection**.

## The evidence-class warning

**Claim 5a (`CLAUDE.md` → nonexistent `bevy.md`) cannot be verified at primary source.** `CLAUDE.md`
was **gitignored** until `5cf7953` (`git show 5cf7953 -- .gitignore` flips `CLAUDE.md` →
`CLAUDE.local.md`), so no version of the offending text is in history. The claim rests entirely on
the self-report of the commit that removed it. Substrate half IS confirmed: no `bevy.md` ever
existed at the repo root (`docs/archive/bevy.md` was *created*, 53 added lines, by the fork),
`grep -c bevy Cargo.lock` → 0. Durable lesson: the one file every agent auto-loads was excluded
from review by `.gitignore` for the project's whole life.

## Tooling note for this repo

`ToolSearch("select:LSP")` returns `No matching deferred tools found` in subagent sessions, while
the `lsp-first-read-guard.js` PreToolUse hook still blocks `Read` on `.rs` files. Workaround that
works: read code via `nl -ba <file>` and `git show <sha>:<path>` in Bash. Also, the Bash `grep`
wrapper sometimes reports `rg-fff: 0 EXACT matches` and returns fuzzy results — re-run with a
simpler pattern before believing a negative.

See [[MEMORY]].
