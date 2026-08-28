---
name: capture-on-this-machine
description: How to launch the game and capture a real screenshot/video on caio's CachyOS+KDE Wayland box, and the three capture methods that DO NOT work
metadata:
  type: project
---

Verified working recipe, 2026-08-28 (sov-odw). The "wrong monitor" failure this
role was warned about is almost certainly the **x11grab black capture** below —
there is only ONE monitor on this machine.

**Display:** single output `DP-2`, 5120x1440 @ 0,0, Wayland (`XDG_SESSION_TYPE=wayland`).
There is no second monitor. `kscreen-doctor -o` confirms.

## What works

- **Stills: `spectacle -b -n -f -o FILE.png`** — 0.29s per call, captures real
  Wayland content. Add `-p` to include the mouse pointer (use it to verify where
  a synthetic cursor actually landed).
- **Video: burst of spectacle stills, then `ffmpeg -framerate N -i f%03d.png`.**
  ~3.3 fps is achievable, so ~62 frames gives an 18.8s video — inside the 15–20s bar.
- **Input: `dotool`** (`printf "mouseto <xfrac> <yfrac>\nclick left\n" | dotool`).
  Coordinates are **fractions of the screen**, not pixels. User is in the `input`
  group and `/dev/uinput` is group-writable, so it works with no setup.
  `wtype` is also installed. `xdotool` is NOT (and would be useless on Wayland).

## What does NOT work — do not retry

- **`ffmpeg -f x11grab -i :0`** — produces a ~10KB black/static file for a 3s
  capture. The game is a native Wayland window, invisible to XWayland. **This is
  the likely origin of the "captured the wrong monitor" incident.** Always sanity-check
  output file size and actually look at a frame.
- **`spectacle -R screen -b -n -o out.mp4`** — the process runs but writes the file
  only on a graceful stop. Killing it with SIGTERM *or* SIGINT produces **no file at
  all**. Tried twice; abandoned.
- `wf-recorder`, `grim`, `slurp`, `kooha`, `obs`, `wl-screenrec` are all **not installed**.

## Launching the game

```
cd <worktree> && setsid nohup ./target/debug/native_app > log 2>&1 < /dev/null & disown
```

**`setsid` is load-bearing.** Without it the game is killed when the spawning Bash
tool call ends. It dies *gracefully* — it serializes and logs "successfully saved
world" — so the log looks clean and you waste a capture run before noticing. Always
`pgrep -x native_app` immediately before capturing.

Side effect to be aware of: the game saves the world on exit, so running it
overwrites the default save.

Related: [[yakui-synthetic-click-trap]], [[ui-proof-ceiling]]
