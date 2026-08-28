---
name: ui-proof-ceiling
description: The "sim harness cannot drive the UI" ceiling is lower than it looks - extracting pure state logic out of a widget closure makes it unit-testable inside native_app
metadata:
  type: project
---

**Before accepting the "UI cannot be tested" ceiling, check whether the logic
under test is actually pure.** If it reduces to plain values, lift it out of the
widget closure into a free function and unit-test it directly in `native_app`.

**Why:** the standing rule is that `TestCtx` is `pub(crate)` under `#![cfg(test)]`
and cannot drive the UI, so UI acceptance = public sim accessor + eyeballed frame.
True — but it says nothing about *pure* logic that merely happens to live inside a
UI file. On sov-odw the pause/resume state machine was two `&mut u32`
(`Settings::time_warp`, `GuiState::depause_warp`) and no ECS access at all.
Extracting `fn toggle_pause(warp: &mut u32, depause_warp: &mut u32)` made it
directly testable, with **no sim accessor needed**.

Facts confirmed 2026-08-28:
- `native_app` is a **bin**, not a lib. `cargo test -p native_app` still compiles
  and runs `#[cfg(test)]` unit tests inside it — verified, 3 tests ran and were
  named in the output.
- Before that commit there were **zero** `cfg(test)` blocks anywhere in
  `native_app/src/`. This is a new pattern in the crate, not an existing one.
- Incremental rebuild of one `native_app` file is ~2–5s, so instrument-rebuild-rerun
  is a cheap debugging loop despite the crate's size.

**How to apply:** when a UI ticket's logic is a state machine over scalars, the
proof is *both* halves plus a real guard — unit test the extracted function,
mutate it to confirm the guard fails, and still capture the frame/video. Report
which halves you achieved. Do not claim a rendered behaviour is proven by a unit
test on extracted logic; the test proves the state machine, the video proves the
wiring. On sov-odw the video was what revealed the click was reaching the widget
at all.

Also: `cargo test -p native_app` inherits the 6 pre-existing `static mut` warnings
from `native_app/src/init.rs:85-101`. They are noise, not yours — but do not copy
that pattern.

Related: [[capture-on-this-machine]], [[yakui-synthetic-click-trap]]
