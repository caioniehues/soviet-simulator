---
name: yakui-synthetic-click-trap
description: A synthetic click at coordinates the cursor already occupies is never delivered to a yakui widget - it fakes a UI bug reproduction on already-fixed code
metadata:
  type: project
---

**Always move the cursor away and back before each synthetic click on a yakui
widget.** Never click twice at the same coordinates without an intervening move.

**Why:** yakui refreshes hover state only on pointer **motion**, and
`dotool mouseto <coords the cursor is already at>` emits no motion event. The
widget under the cursor is therefore not hovered, and `b.show().clicked` never
becomes true. Measured 2026-08-28 on sov-odw.

This cost a full debugging cycle and nearly produced a false report. The capture
run **appeared to reproduce the pause-button bug on the fixed binary**: the first
pause click worked, and every subsequent same-position click did nothing — which
is exactly the reported symptom. I got as far as suspecting a second defect in
`goryak::button_primary` before instrumenting.

The instrumented ground truth (temporary `eprintln!` on `resp.clicked`):

```
click 1 (after a move):  DBG click text=pause b_warp=0 warp_before=1
                         DBG after  text=pause warp=0 depause=1
clicks 2 and 3 (no move): NOTHING AT ALL — clicked never fired
after jiggling away+back: DBG click text=pause b_warp=0 warp_before=0
                          DBG after  text=pause warp=1 depause=1
```

Note the tell: clicks that "fail" this way produce **no event whatsoever**, not a
wrong result. If a widget seems dead but a *different* widget responds in the same
state, suspect the harness before the code — a click involving real cursor travel
(moving to a different button) works fine, which makes the failure look
state-dependent and therefore very convincing as a game bug.

**How to apply:** in any capture script driving the HUD, make the click helper
unconditionally jiggle:

```bash
click() {
  printf "mouseto 0.90 0.30\n" | dotool; sleep 0.20
  printf "mouseto %s %s\n" "$1" "$2" | dotool; sleep 0.30
  printf "click left\n" | dotool
}
```

When a UI experiment contradicts a passing unit test, instrument the widget
response with a temporary `eprintln!` and rebuild — it is ~5s incremental for
`native_app` and it settles the question immediately. Do not theorize twice.

Related: [[capture-on-this-machine]], [[ui-proof-ceiling]]
