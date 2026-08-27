---
name: silent-decode-default-seam
description: 16 resources cross the save/load seam; decode errors are swallowed into defaults, only map has a guard
metadata:
  type: project
---

The resource save/load seam swallows failures. `register_resource_noinit`'s load closure
(simulation/src/init.rs:233-240) logs `log::error!` on decode `Err` and returns — leaving the
init_func's **default** in place. `Deserialize for Simulation` (lib.rs:428-432) ignores every
load outcome and returns `Ok(sim)` (:439). The save side **unwraps** (init.rs:232): loud where
nothing is lost, silent where everything is.

Exactly **16** resources cross the seam (re-counted 2026-08-27): 12 `register_resource_default::<`
+ 3 `register_resource::<` call sites + 1 direct `register_resource_noinit::<` (SimulationOptions,
init.rs:122). The 6 `register_resource_noserialize` (:116-121) do not cross. A grep for
`register_resource::<` also matches the fn definition at :215 — subtract it.

Only **one** post-load guard exists: `environment.size().0 == 0` (lib.rs:289). It lives in
`load_from_disk`, so it does **not** protect the `Deserialize` path used by netcode
(networking/src/worldsend.rs) or by tests.

**SimulationOptions is the one noinit resource** — no init_func inserts it, `from_replay`
(lib.rs:154-176) runs init_funcs only, and it arrives via a replay command around tick 3.
Anything that serializes a sim before tick 3 panics at `utils/resources.rs:80`. This is why
test_iso.rs sets `check_start = 3`; see [[determinism-test-cannot-fail]].

**Why:** a corrupt or version-skewed save loads "successfully" with silently defaulted state —
the player sees a world that is subtly wrong rather than an error.

**How to apply:** when reviewing anything touching persistence, check whether a new resource gets
a post-load sanity guard. Treat "it loads without error" as no evidence at all.
