---
name: determinism-test-cannot-fail
description: test_world_survives_serde is structurally incapable of failing on divergence; proven by mutation 2026-08-27
metadata:
  type: project
---

`test_world_survives_serde` (simulation/src/tests/test_iso.rs:241-306) runs a real two-run
comparison over 10k ticks, but **no divergence can ever turn it red**. Each mismatch branch
(:276, :284, :292) does `check_size /= 2; continue 'main`; there is no `assert!`/`panic!`
anywhere in the loop body; `check_size == 0` (:253) exits green.

**Proven empirically 2026-08-27**, not just read: perturbing `sim2` inside the loop at tick 5000
(`sim2.write::<Government>().money += Money::new_bucks(1)`) produced 11 `not equal sim+sim2`
lines and still `test result: ok. 1 passed; 0 failed`. Unmutated baseline: 0 divergences, also
green in 7.74s. Green means the same thing either way.

`TestCtx::check_determinism` (tests/mod.rs:106-120) is **misnamed** — one sim, encode/decode,
compare hashes. Round-trip stability only, never a second run. See [[sim-test-harness-quirks]].

**Why:** substrate.md:37 rates repeat-run determinism "Absent". That row is correct AS WRITTEN —
a mechanism that cannot fail proves nothing, so "no cited check *proves* it" is literally true.
Do not let anyone downgrade the row to "letter-wrong because a two-run mechanism exists".

**How to apply:** never cite this test as determinism evidence. If a ticket claims to fix or rely
on repeat-run determinism, the acceptance criterion must be a check that goes RED on divergence —
demand a mutation demo. Mutating the test file temporarily is the cheap proof; revert and show
`git diff --stat` empty.

Corollary trap: an **early-onset** divergence makes the cascade walk `check_start = tick - check_size`
back below tick 3, where `SimulationOptions` does not exist yet, and the run dies on an unrelated
unwrap at `utils/resources.rs:80` (via the noinit save closure, init.rs:232). Red for the wrong
reason, with no usable diagnosis. Perturb LATE (tick ~5000) when demonstrating the green path.
See [[silent-decode-default-seam]].
