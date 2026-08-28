---
name: engine-capture-wiring
description: Wiring points for the engine render/capture path — where engine_demo capture branches to offscreen, and the negative-control trick that proves a headless branch is live
metadata:
  type: project
---

# Engine capture / offscreen wiring (as of sov-pci, 2026-08-28)

**Entry point chain for `engine_demo capture`:**
`engine_demo/src/main.rs:389` `main()` → `:407-416` builds `FrameworkOptions { capture: Some(..) }`
(always, for any capture run) → `:418 start_with_options` → `engine/src/framework.rs:344`
`if !opts.requires_window()` → `:345 beul::execute(run_offscreen::<S>(opts))`.

There is **no DISPLAY/WAYLAND check** — the branch is on `capture.is_some()` alone. Consequence:
the in-`run()` windowed capture block (`framework.rs:217-240`) is dead on native and live only on
wasm32 (`:344` is `cfg(not(wasm32))`, `:372` spawns `run` unguarded). Duplicated capture logic;
a future edit will likely touch only one copy.

**Framework option gate:** `FrameworkOptions::requires_window()` (`framework.rs:49`) is the single
switch between windowed and offscreen. Anything new that must run headless has to be reachable from
`run_offscreen`, not from `run`.

**Headless constructors** (each has exactly one production caller, inside `run_offscreen`/`Context::new_offscreen`):
`GfxContext::new_offscreen` (gfx.rs:287), `create_offscreen_target` (gfx.rs:826),
`EguiWrapper::new_headless` (egui.rs:40), `YakuiWrapper::new_headless` (yakui.rs:23).

**Panicking accessors:** `GfxContext::window()` / `surface()` (gfx.rs:814/820) panic in offscreen
mode. Safety in `run_offscreen` is a *runtime invariant*, not type-level: `engine_demo/src/main.rs:301,310,319`
call `window()` but are gated on `ctx.input` state, which `run_offscreen` never populates
(`freeze_input`, no event loop). Any future offscreen-reachable code that calls `window()`
unconditionally will panic.

## The cargo-feature-unification trap (check this on every engine diff)

`engine`'s `yakui` feature is enabled **only** by `native_app/Cargo.toml:12`. So
`cargo run -p engine_demo` compiles engine WITHOUT yakui, and every `#[cfg(feature = "yakui")]`
line in `framework.rs` is absent from that binary. A workspace build
(`cargo build -p engine_demo -p native_app`) unifies features and the SAME binary path suddenly
includes them. **A "verified by running it" claim on a cfg-gated line is worthless unless you say
which build produced it.** For sov-pci I ran both; both wrote sha256 `e547e263…`.

## The negative control that actually proves a headless branch

Running the capture headless and seeing "capture ok" is weak on its own — a fallback could be
creating a window some other way. The proof is the *contrast*, same env, same binary:

- `env -u DISPLAY -u WAYLAND_DISPLAY ./engine_demo capture --out X` → "capture ok", PNG written
- `env -u DISPLAY -u WAYLAND_DISPLAY ./engine_demo` (interactive) → panics at `framework.rs:348`,
  "neither WAYLAND_DISPLAY nor WAYLAND_SOCKET nor DISPLAY is set"

Reuse this shape for any "works without X" claim: show the un-branched path failing on the same
input. Costs one extra command and converts a plausible claim into a proven one.

Related: [[MEMORY]] recurring shape #3 below.
