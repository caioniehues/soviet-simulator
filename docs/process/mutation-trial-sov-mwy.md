# Mutation trial — sov-mwy (market.rs)

**Kind:** trial result
**Authority:** operational
**Status:** complete — **decision: ADOPT** (file-scoped, Phase 3, non-blocking)
**Owner:** project lead
**Depends on:** `docs/process/mutation-policy.md` (sov-4f7)
**Ran:** 2026-08-28, worktree `/home/caio/sov-mwy-wt` (branch `work/sov-mwy`, off main `345a79a`)
**Machine:** host `hal`, 12 threads, ~30 GB RAM

This is the first mutation-testing trial the policy calls for. It runs command **B** (file-scoped)
from the policy against the highest-risk economy file and records a per-mutant disposition. It
changes no production code.

---

## Tool

| | |
|---|---|
| Binary | `/tmp/sov-tools/bin/cargo-mutants` |
| Version | `cargo-mutants 27.1.0` (from `--version`) — matches the policy pin |
| Licence | **MIT** — `license = "MIT"` in `~/.cargo/registry/src/index.crates.io-*/cargo-mutants-27.1.0/Cargo.toml`, and its shipped `LICENSE` reads `MIT License / Copyright (c) 2021 Martin Pool` |
| Obligation on this repo | None. MIT build-time analysis binary, never linked into a shipped artifact, never vendored, not in any `Cargo.toml`. |

---

## Scope and command

Bounded to the single file `simulation/src/economy/market.rs`, package `simulation`.

```sh
export TMPDIR=/home/caio/sov-mwy-scratch          # see "Deviation" below
unset CARGO_TARGET_DIR
/tmp/sov-tools/bin/cargo-mutants mutants \
    -p simulation \
    -f 'simulation/src/economy/market.rs' \
    -t 90 --build-timeout 300 \
    -j 4 \
    -o /home/caio/sov-mwy-mutants-out \
    -v -V
```

Mutant count confirmed before the run (`--list`, no build): **163**, matching the policy's measured count.

### Deviation from the brief (recorded)

The brief's trap #1 said to set `TMPDIR=/home/caio/sov-mwy-wt/.mutants-tmp` — **inside** the
worktree. That does not work: `cargo-mutants` copies the whole source tree into `TMPDIR`, so a
`TMPDIR` inside the copied tree recurses into itself and aborts within 6 s:

```
Error: Failed to copy .../.mutants-tmp/cargo-mutants-...tmp/.mutants-tmp/cargo-mutants-...tmp/... (repeated)
Caused by: File name too long (os error 36)
```

Fix, honouring the brief's actual intent (keep scratch off the `/tmp` tmpfs): `TMPDIR` was moved to
`/home/caio/sov-mwy-scratch`, a sibling directory **outside** the copied worktree but still under
`/home`. `CARGO_TARGET_DIR` was left unset so `cargo-mutants` manages its own per-mutant build dirs
under that scratch (it used reflink copy-on-write; the 1.8 GB tree copied in ~0.3 s). The `/tmp`
tmpfs was never touched by the run.

---

## Baseline (must be green before the run — it was)

`cargo test -p simulation` in the worktree, before starting:

```
test result: ok. 45 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 12.69s
   Doc-tests simulation
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`cargo-mutants` re-ran the same baseline internally and agreed: `ok  Unmutated baseline in 41s build + 12s test`.

---

## Result — the run's own summary

```
163 mutants tested in 38m: 22 missed, 96 caught, 21 unviable, 24 timeouts
```

| Outcome | Count | Meaning |
|---|---|---|
| **caught** (killed) | **96** | A test failed when the mutant was applied — the guard exists and fired. |
| **missed** (survived) | **22** | No test noticed the mutation — a test gap or an equivalent/out-of-contract mutant. |
| **timeout** | **24** | Mutated test run exceeded `-t 90 s`. **INCONCLUSIVE** — never a survivor, never a kill. |
| **unviable** | **21** | Mutant did not compile (mostly `Box::leak(Default::default())` for `&T` returns). Neither killed nor survived. |
| Total | 163 | |

**Runtime cost:** wall-clock **2262 s = 37.7 min** (`WALLCLOCK_SECONDS=2262`), within the policy's
2 h limit for a file-scoped run. 4 concurrent jobs; each mutant ≈ 5–30 s incremental build + up to
90 s test.

### At least one killed mutant (proof the run detects real guards)

96 mutants were killed. Three concrete examples, each an economic guard that fired:

```
caught  market.rs:301:19: replace += with -= in Market::remove            in 16s build + 32s test
caught  market.rs:471:53: replace -= with += in Market::settle_retail      in 17s build + 35s test
caught  market.rs:298:17: replace < with > in Market::remove               in 27s build + 29s test
```

The first two are conservation guards on the retail/reserved ledger — flipping the sign of a
capital or reservation update makes an existing scenario test go red. This is the run working.

---

## Survivor disposition — all 22

Every survivor gets exactly one label per the policy (REAL GAP / EQUIVALENT / OUT OF CONTRACT /
ACCEPTED). No survivor is left silent.

### Cluster A — `make_trades` external-trade branch (11 survivors) → **REAL GAP**

This is the finding of the run. Every survivor below is in the `if !*optout_exttrade { … }` branch
of `make_trades` (lines 655–718) or the reserved-aware capital guards feeding it. This branch is
**highly reachable**: `base_mod/items.lua` sets `optout_exttrade = true` on exactly one item of
twenty-one, so twenty of twenty-one goods traverse this path — yet no `cargo test` scenario
asserts the money and quantity it moves.

| Mutant | Line | Untested behaviour |
|---|---|---|
| `replace - with +` | 555:35 | `cap_seller - already_reserved < trade.qty` — the reserved-aware domestic-match affordability guard; no test constructs a seller with a live reservation that would flip it. |
| `delete -` | 680:38 | sign of `money_delta = -(*ext_value * qty_buy)` on an external **buy** — govt should lose money; no test asserts the sign. |
| `replace * with /` | 680:51 | magnitude of that same ext-buy `money_delta`. |
| `replace - with +` | 693:52 | `free_qty as i32 - order.stock` — the exportable-surplus quantity; no test asserts how much stock is exported. |
| `replace < with <=` | 698:57 | `*cap - already_reserved < qty_sell` boundary — the "selling more than it has" export guard at exact equality. |
| `replace - with +` | 698:29 | the reserved-aware subtraction inside that same export guard. |
| `replace -= with +=` | 707:26 | `*cap -= qty_sell` — the seller's capital debit on an external sell (a units-leave-the-city point). |
| `replace -= with /=` | 707:26 | same debit, different corruption. |
| `replace -= with +=` | 708:31 | `order.qty -= qty_sell` — the sell-order quantity debit on external sell. |
| `replace -= with /=` | 708:31 | same debit, different corruption. |
| `replace * with /` | 715:49 | magnitude of `money_delta = *ext_value * qty_sell` on an external **sell**. |

**Missing assertion to close it:** a scenario (or unit test on `make_trades` with an injected
`find_external`) that drives one full external buy and one external sell, then asserts (a) the
seller's `capital` and `sell_order.qty` decreased by exactly `qty_sell`, and (b) each resulting
`Trade.money_delta` has the correct sign and magnitude (`±ext_value * qty`). One test kills most of
this cluster. Recommend a `bd` follow-up ticket; see decision below.

### Cluster B — dispatch state-machine dwell/TTL/retry timing (5 survivors) → **REAL GAP** (boundary)

These guards have scenario tests that exercise the *path* but do not *pin the exact tick*, so a
one-tick or inverted-branch mutation slips through.

| Mutant | Line | Untested behaviour |
|---|---|---|
| `replace == with !=` | 847:35 | `if ticks_left == 0` gating the end of `Loading` dwell — exact dwell duration not asserted. |
| `replace == with !=` | 1007:35 | same, for `Unloading` dwell. |
| `replace == with !=` | 1062:33 | `if claim.ticks_left == 0` — retail-claim TTL expiry tick; `scenario_retail_ttl_*` tests exist but don't pin the expiry tick. |
| `replace >= with <` | 897:72 | `return_route_retries + 1 >= MAX_RETURN_ROUTE_RETRIES` — the severed-road give-up boundary; `scenario_returning_with_severed_road_terminates` covers the path but not the exact retry count. |
| `replace + with *` | 897:68 | the `+ 1` in that same retry comparison (`x*1` vs `x+1` diverges only above the boundary). |

**Missing assertion:** assert the exact number of ticks a dispatch spends in `Loading`/`Unloading`
(= `DISPATCH_DWELL_TICKS`), the exact tick a retail claim expires (= `RETAIL_CLAIM_TTL_TICKS`), and
the exact retry count before a severed-road dispatch is dropped (= `MAX_RETURN_ROUTE_RETRIES`).
Lower value than Cluster A: these are timing constants, not conservation.

### Cluster C — `Market::remove` edge cases (2 survivors) → **REAL GAP**

| Mutant | Line | Untested behaviour |
|---|---|---|
| `replace != with ==` | 274:51 | `retail_claims.retain(\|_, c\| c.seller != soul)` — dropping claims that a *removed seller* still owes to live buyers; no test removes a seller that holds outstanding retail claims. |
| `replace \|\| with &&` | 300:32 | `if d.buyer != soul \|\| d.seller == soul` — the guard selecting which dangling dispatches get the dead-buyer routing vs. are left alone; no test constructs an unrelated dispatch coexisting with the removed soul. |

**Missing assertion:** a `remove` scenario where the removed soul is a seller with a live retail
claim from a surviving buyer, plus an unrelated third-party dispatch, asserting conservation across
the removal. These are exactly the `sov-dispatch-wedge` class of bug the surrounding comments cite.

### Cluster D — `calculate_prices` (2 survivors) → 1 REAL GAP, 1 EQUIVALENT

Both are the `*` in `price_workers.inner() as f32 * price_multiplier` (line 1128:66). The sole unit
test `calculate_prices()` calls `calculate_prices(1.0)`.

| Mutant | Disposition | Reason |
|---|---|---|
| `replace * with /` | **EQUIVALENT under the test's only input** | `x / 1.0 == x * 1.0`; with `price_multiplier` fixed at `1.0` no assertion can distinguish `*` from `/`. Killing it requires a second test input (`multiplier != 1.0`). |
| `replace * with +` | **REAL GAP** | `x + 1.0` differs from `x * 1.0` by 1 inner unit, but the later integer `/ qty` division (qty = 3 for cereal) absorbs a ±1 perturbation, so the assertion passes anyway. |

**Missing assertion:** add a `calculate_prices(m)` case with `m != 1.0` (e.g. `2.0`) and assert the
worker-cost term scales with `m`. Note price is **not** a clearing gate in the Kornai model
(clearing is by queue, never by price), so this cluster is low priority — a candidate for ACCEPTED
if not scheduled.

### Accessors (2 survivors)

| Mutant | Line | Disposition | Reason |
|---|---|---|---|
| `replace Market::iter -> … with iter::empty()` | 221:9 | **OUT OF CONTRACT** | `Market::iter()`'s only production callers are UI panels (`native_app/src/gui/hud/windows/economy.rs`, `inspect_building.rs`, `inspect_human.rs`). The sim test harness cannot drive the UI (fork rule: UI is proven by frames, not `cargo test`), so no sim test is owed. Not a sim-side gap. |
| `replace Market::register with ()` | 480:9 | **EQUIVALENT** | `register` only pre-creates a zero `capital` row (`self.m(kind).capital.entry(soul).or_default()`). Every consumer treats absent and zero identically — `make_trades` uses `capital.entry().or_default()`, `Market::capital` uses `.unwrap_or(0)`, and a seller with capital 0 is skipped either way. Making it a no-op changes no observable behaviour. |

---

## Timeout disposition — all 24 → **INCONCLUSIVE**

Per the policy, a timeout is neither a survivor nor a kill. `-t` was **not** raised to convert any
of these; the disposition for every one is `inconclusive — re-run at higher -t if it becomes
load-bearing`. The common cause here is benign: the mutation removes a termination/accumulation
condition, so a scenario test that ticks until a threshold runs past the 90 s budget. None is
grounds for a production bug ticket.

| Function | Timed-out mutants | Likely mechanism |
|---|---|---|
| `SingleMarket::capital` (69) | `Some(0)`, `Some(1)` | a scenario ticks until capital converges; a constant capital never lets it settle. |
| `find_trade_place` (126) | `None`, `Some(Default)` | routing loop never resolves a trade place. |
| `Market::sell` (228) | `with ()` | orders never posted; a scenario waits forever for a trade. |
| `Market::remove` (301, 390) | `+= → *=` (×2) | reservation/capital values explode instead of incrementing, no convergence. |
| `Market::produce` (486, 489) | `→ 0/1/-1` (×3), `+= → -=`, `+= → *=` | production never reaches the threshold a scenario ticks toward. |
| `Market::make_trades` (498, 507, 511, 520, 555, 575) | `Vec::new`, `> → ==/</>=`, `== → !=`, `> → ==/>=`, `< → ==` (×9) | matching-loop comparisons flip, so trades never clear and the scenario never terminates. |
| `Market::advance_dispatches` (746, 747, 1049) | `with ()`, `< → >`, `+= → *=` | dispatch lifecycle never advances / never terminates. |

24 total. Every timeout is accounted for above.

## Unviable — 21 → not evidence either way

All 21 failed to compile (counted, not analysed): mostly `cargo-mutants` synthesising
`Box::leak(Box::new(Default::default()))` or `Vec::leak(...)` to fake a `&T` / `&[T]` return, plus
a few arithmetic swaps that don't typecheck (`+ → *` where types differ). A non-compiling mutant is
never tested; correctly excluded from both the kill and survivor counts.

---

## Restoration proof (tree clean of source after the run)

```
$ git -C /home/caio/sov-mwy-wt status --porcelain | grep -v '^??'
        (empty — no tracked source changes)
$ git -C /home/caio/sov-mwy-wt status --short
?? target-mutants/
```

The only untracked entry is the worktree's own `target-mutants/` build directory (removed after the
trial); no `market.rs` mutation survived. `cargo-mutants` mutated copies under `TMPDIR`, never the
worktree source — the restoration check confirms it.

---

## Decision: **ADOPT** (file-scoped, Phase 3, non-blocking)

Reasoning:

1. **It produced a real finding at acceptable cost.** 37.7 min bounded to one file surfaced one
   substantive test gap — the entire external-trade branch of `make_trades` (Cluster A, 11
   survivors) moves money and units for 20 of 21 goods with no `cargo test` asserting either. That
   is exactly the conservation-adjacent defect class the policy exists to name, and no other Phase
   gate would have found it by test-sensitivity.
2. **The signal-to-noise held.** Of 22 survivors, 14 are actionable REAL GAPs outside the boundary
   cluster, 3 are honest EQUIVALENT/OUT-OF-CONTRACT (two accessors and price-under-single-input),
   and 5 are boundary timing constants. Not a wall of noise to scroll past.
3. **The timeout discipline worked as written.** 24 timeouts stayed in their own column and were
   dispositioned INCONCLUSIVE without inflating the survivor count or manufacturing kills.

Adopt scope, unchanged from the policy: file-scoped runs (command B) at `evidence-auditor`'s
discretion on an eligible economy change, **inside Phase 3, never blocking, never a substitute for a
Phase 4 gate**. This trial does not make the tool mandatory and does not add it to CI or any
`Cargo.toml`.

### Recommended follow-up (not part of this trial)

File one `bd` ticket for **Cluster A**: a `make_trades` external-trade conservation test asserting
seller `capital`/`sell_order.qty` decrements and `Trade.money_delta` sign+magnitude on both an
external buy and an external sell. Clusters B–D are lower priority (timing constants; price is not a
clearing gate).

---

## Audit addendum (evidence-auditor, 2026-08-28; appended by lead)

The trial above was audited before it was committed. The auditor re-derived every claim from
source rather than reading the report, and hand-re-ran the headline cluster. Its findings are
recorded here so the evidence lives with the trial rather than only in the tracker.

**All 8 acceptance criteria MET.** `cargo-mutants 27.1.0` matches the pin at
`mutation-policy.md:119`; licence re-derived from the shipped `LICENSE` (MIT, Copyright (c) 2021
Martin Pool), not from the manifest field alone; all 22 survivors matched `mutants.out/missed.txt`
one-for-one with no silent omission; all 24 timeouts accounted for by the function/line table
(2+2+1+2+5+9+3); the 2262 s runtime matches `debug.log`'s final mutant timestamp of 2261.85 s; and
the worktree was confirmed clean of leftover mutations independently (`git diff HEAD -- simulation/`
empty, `target-mutants/` and the scratch TMPDIR both gone).

**Cluster A is still live on `main` at f6725f1 — re-proved, not assumed.** `main` was 6 `market.rs`
commits ahead of the trial base, including `sov-abs` and `sov-dii`, which added external-trade
tests. Reading those tests cannot settle whether the gap closed, so the auditor re-ran all 11
Cluster A mutations by hand, one at a time, with a full `cargo test -p simulation` each. **All 11
still survive**, every one printing `test result: ok. 52 passed; 0 failed; 1 ignored`.

The instrument was proven rather than trusted. A control mutation at `market.rs:485`
(`settle_retail`, `-=` -> `+=`, a line the trial recorded as caught) on the same harness:

```
test tests::scenarios::retail::scenario_retail_no_dispatch_settles_at_eat_time ... FAILED
panicked at 'assertion `left == right` failed: seller must end up debited
test result: FAILED. 51 passed; 1 failed; 1 ignored
```

So "all 11 survived" is a measurement, not a harness that cannot fail.

Sharper than the trial put it: `sov_abs_ext_trade_import_is_physical` proves the goods move
physically and asserts **nothing** about `Trade.money_delta`. That is this repo's recurring shape —
the detect half tested, the react half only announced.

Follow-up filed as **sov-sp6** (P2), blocked by **sov-20g**, since sov-20g moves the debit and the
test should be written against the final shape.
