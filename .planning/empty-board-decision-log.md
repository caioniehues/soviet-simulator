# Empty-board run — decision log (2026-09-04, for post-conclusion review)

58 tickets (54 open + 4 in progress) → 171/171 closed. Every close carries
sha + gates in its reason; detail lives in per-ticket `bd` comments.
Decisions below are the ones made by judgment rather than by ticket text.

## Supersede / obsolescence (closed without code)
- sov-tg2 → superseded by sov-am1 + sov-ary. The golden gate shells the real
  binary headless and byte-compares (stronger than tg2's asked sha256-ignored
  test); ary proved it red. Headless run verified by orchestrator.
- sov-7it → resolved by obsolescence. `~/.claude/rules/delegation.md` no
  longer exists; nothing references the claimed roster. Session transcripts
  matched the grep but are not rules.
- sov-l1e → already fixed in tree (04efbe2); verified non-vacuous by mutation
  today, no edit.

## Adopt-or-remove
- sov-ip7 → REMOVE. Spike claims re-derived true, but swap cost exceeds gain
  while the hand-rolled module has measured numbers. Rationale on ticket.
- sov-oiu → rank/seed ONLY behind authoritative tick-seeded A*. fast_paths
  cannot reproduce the tick jitter; cache never served stale (None on
  revision mismatch). No stock/dispatch/position moves on lookup (pillar).
- sov-d3a → DELETE the native dead block (kept for wasm32 under cfg with
  reason comment). A windowed/offscreen comparison was never reproducible
  (surface negotiates its own format); no --windowed flag.

## Behavior changes to existing tests (all cited, none silent)
- sov-jcl: bound 20 → 300 ticks (sov-13h intent); test waits 600 ticks now.
- scenario_0151: freight-station isolation (implements sov-bqm) + budget 8000;
  honest/dishonest signal holds.
- query_same_lane_works: past-target trucks now offered as fallback tier
  (sov-2uv); test reordered + re-registers (reserve removes positions; free
  does not restore — needs update cycle).
- unpark refusal test setup: default city now parks 2 station trucks
  (sov-2uv); setup removes the default station to restore composition.
- sov-qi8 setup: exact-10 spawn (auto-spawn overshot) + GoTo commute
  (idle humans never enter the grid).

## Reopened mid-run
- sov-ejz: the claimed barrier fix was ABSENT from the tree (only
  try_window/present_mode/gate work had landed). True site: water samples
  the live depth texture in the main pass → depth→depth_sample copy at head
  of before-main encoder. Game survived 30s, validation clean, golden
  byte-identical. distrustSecond-hand "verified" claims on GPU work.

## Partial visual proof (honest gap)
- sov-8lu trade row: EYEBALLED live (3 frames, `target/renderer-evidence/
  ui-trade-status-row.png`, md5 29af5442). Accessors public, always shown.
- sov-hoard-panel-mko + sov-dda.4: code compiled and bound to the real
  public signal through the proven yakui path, but building SELECTION was
  not achievable in this headless environment (companies are not
  world-clickable; camera hunt + Economy-window navigation + xlib clicks
  exhausted: wilderness views, no settlement reached). If the panel layout
  is wrong, no test will catch it — re-eyeball on a machine with a display.
- Ancillary find: trade_status first rendered off-screen (bare minrow below
  the full-height toolbox box); anchored under the menu bar. Sibling UI
  work should be screenshotted, not assumed visible.

## Environment notes
- cargo-deny binary absent: buw verified via agent's exit-0 run (covered the
  fast_paths addition) + lock-level proof today (13/13 git sources pinned).
- lavapipe recipe (no root): see engine_demo/golden/lavapipe.note.txt.
  MSSIM 0.9948 vs RADV golden, threshold >= 0.99.
- Xvfb + X11 backend + ffmpeg x11grab screenshots the game; Wayland and
  ImageMagick import do not. xlib via `uv pip --target /tmp/pylibs`
  python-xlib drives keys/clicks. Arrow keys pan only after a focus click.
