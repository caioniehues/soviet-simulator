# App assembly is two `PluginGroup`s; binaries declare exclusions, never inclusions

**Status:** decided 2026-08-17, **not yet built** — groundwork, [#118](https://github.com/caioniehues/soviet-simulator/issues/118).

Eighteen bin targets each hand-typed the plugin list, and every one was a frozen snapshot of
`lib.rs` from the day it was written — `capture.rs` ran 9 sim plugins against `lib.rs`'s 18,
`bench_traffic` ran 4. `bevy.md` documented the gap as a human discipline ("any binary that
adds `GamePlugin` must register **every** sim plugin `lib.rs` does"), enforced by a runtime
`Res<T>` panic rather than the compiler. App assembly had no module, so nineteen call sites
implemented it.

It is now `SimPlugins` and `GamePlugins`, Bevy `PluginGroup`s composed with `add_group`. The
load-bearing property is an inversion: a binary that lists what it *includes* is correct
only until the next domain plugin lands, which is exactly how all eighteen drifted, whereas
a binary that states what it *excludes* — `SimPlugins.build().disable::<HeatSimPlugin>()` —
stays correct as the system grows. Inclusions rot; exclusions don't. Two things fall out:
`add_group` replaces the two-root arrangement where `lib.rs` composed the sim plugins and
`GamePlugin` composed the presentation ones, and the 15-element `add_plugins` tuple ceiling
disappears, which both files previously worked around with a second call and a comment.

Two consequences recorded so they aren't mistaken for oversights. **The duplicate resource
registrations are deleted** — `Treasury` was `init_resource`'d in four plugins,
`AllocationFeedback` in three, `StatePlan` and `ZoningFeedback` in two, every one commented
as a guard for partial plugin sets. That workaround outlived its cause and violated
`architecture/ecs.md`'s own constraint 3, one authoritative owner per fact; a bench that
disables an owner now inserts what it needs, as `bench_chain` already does for its
infinite treasury. **The seven historical capture binaries are deleted** (`capture_m2` …
`capture_m8`): their videos are committed under `screenshots/result/`, the charter's
acceptance discipline compares recordings rather than re-running old scripts, and keeping
them meant every future plugin change had to keep seven dead scripts compiling.
