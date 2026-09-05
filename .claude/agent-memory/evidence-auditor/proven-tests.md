---
name: proven-tests
description: Tests in this repo I have personally mutation-proven — watched fail, then revert — with date and the mutation used
metadata:
  type: project
---

A test listed here does not need re-proving unless the test or the code it guards changes.

## 2026-08-27 — sov-bo3, branch `fix/sov-bo3-lav-unbounded`, commit `4d1d18b`

`geom/src/skeleton.rs` module `cycle_tests`:

- **`iter_keys_terminates_on_cycle_not_through_head`** — PROVEN, twice.
  - Mutation A: remove `.take(limit)` and the `keys.len() == limit` branch → test never
    returns; SIGKILL by a 1G cgroup ceiling (`journalctl --user`: "The kernel OOM killer
    killed some processes in this unit", "1G memory peak").
  - Mutation B (the ticket's named trap): keep `.take(limit)`, drop the corrupt branch so
    the truncated prefix is returned → `iter_keys walked 9 vertices over an 8-vertex arena`.
    This is what proves the `keys.len() <= vs.len()` assertion is *not* vacuous.
- **`iter_keys_walks_a_full_arena_ring`** — PROVEN. Mutation: `limit = vs.len()` (drop the
  `+ 1`) → this test fails *and* all four pre-existing `skeleton::tests` fail on `.unwrap()`.
- **`simulation ... placement_stress::gen_exterior_house_8m_100k_placements`** (`#[ignore]`) —
  PROVEN. Mutation A above → SIGKILL between seed 30000 and 40000 under a 2G ceiling.
  Unmutated: ok, RSS 7488 → 8572 kB, 1.38 s.

**Proven NOT guarded** (same commit): neutering `any_corrupt()` to `false` — which deletes the
entire "refuse rather than truncate" half — leaves `cargo test -p geom` at 23 passed and the
100k sweep green. See [[weak-evidence-shapes]].

Useful side-fact: the six `.unwrap()`s added to `skeleton::tests` are live guards, not
None-hiders. Mutation C makes every one of them fire.

## 2026-08-28 — sov-mwy audit, `simulation/src/economy/market.rs` at main `f6725f1`

**Proven NOT guarded — the ext-trade branch of `Market::make_trades`.** Eleven mutations,
each applied alone, full `cargo test -p simulation` each, reverted each. **All eleven printed
`test result: ok. 52 passed; 0 failed`.** Line numbers at `f6725f1`:

- `651` delete `-` and `*`→`/` — the import `money_delta` sign and magnitude
- `718` `-`→`+` — exportable surplus quantity
- `723` `<`→`<=` and `-`→`+` — the oversell guard and its reserved-aware subtraction
- `732` `-=`→`+=` / `/=` — the seller's capital debit on export (units leave the city here)
- `733` `-=`→`+=` / `/=` — the sell-order quantity debit on export
- `740` `*`→`/` — the export `money_delta` magnitude
- `569` `-`→`+` — the domestic reserved-aware affordability guard

Filed as `sov-sp6` (blocked by `sov-20g`). Reachability: `base_mod/items.lua` has 21 items and
sets `optout_exttrade` on exactly one, so 20 of 21 goods take this path.

**Control that makes those eleven believable** — same harness, same command, one function over:
`market.rs:485` `settle_retail` `-=`→`+=` gives
`test result: FAILED. 51 passed; 1 failed`, `scenario_retail_no_dispatch_settles_at_eat_time`,
`seller must end up debited`. Always run this control before reporting a wall of survivors;
without it "nothing failed" and "the harness is broken" look identical.

**PROVEN guards (from the trial's own `mutants.out/caught.txt`, 96 kills, base `345a79a`):**
`Market::remove` `+=`→`-=` and `<`→`>`; `Market::settle_retail` `-=`→`+=`. These are real.

## 2026-09-02 — sov-ahw, worktree `/home/caio/sov-ahw-wt` at `4e9e930` + uncommitted diff

`ledger::sov_ahw_stranded_tosource_import_reposts_and_resumes_production` — PROVEN for
the re-post (drop `buy_until` → red :1279), the refund (drop `gvt.money -= money_delta` →
red :1307), the bound (`MAX_SOURCE_WAIT_TICKS = u32::MAX` → red :1279) and the removal
(drop `remove = true` → red :1292). **NOT guarded for the seller's reserved release**: the
test asserts `reserved(bakery, flour)`, the buyer, which is always 0. Only
`hoarding::scenario_0083` (:229) catches that mutation. `scenario_0083`'s rewritten
`.all(|d| ...)` state assertion is VACUOUS — its negation passes, zero dispatches at tick 1000.
Gotcha: `Money` Debug prints negative cents as `0$` (`-0.50$` shows as `0$`), so a
`money_delta: 0$` in a Dispatch dump is not proof the delta is zero.
Collision lesson: another agent doing a stale `Write` of market.rs re-introduced my in-flight
mutation after I had restored it. Keep a pristine copy and `diff` against it at the very end,
not just md5 right after each restore.
