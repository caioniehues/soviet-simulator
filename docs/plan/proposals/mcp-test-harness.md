# Proposal — an MCP harness so the agent can test the game

**Kind:** proposal
**Authority:** advisory — nothing here is ratified
**Status:** draft for decision
**Owner:** project lead
**Date:** 2026-08-27
**Feeds:** the standing gap that every UI and renderer change is proven by a human at a keyboard

---

## Verdict

**Build it, in two tiers, and do not build the fourth capability you asked for.**

Three of the four capabilities requested are buildable and worth building. The fourth —
driving the real UI with synthetic input — is not merely expensive, it *cannot do the job it
was requested for*, because the single interaction it was meant to automate is impossible
by construction in the current code. That is a finding, not an excuse, and it is evidenced
below.

The approved shape ("a new crate that owns a `Simulation` directly") is correct for the
simulation and the save/load lanes, and **structurally cannot reach the renderer or the UI**.
Those need a second, smaller mechanism inside `native_app`. Hence two tiers.

| Capability requested | Verdict | Tier |
|---|---|---|
| Drive the sim headlessly | **Yes** — clean, low risk | A |
| Save and load worlds | **Yes** — needs a path parameter | A |
| Render frames the agent can see | **Yes** — but not headless on this machine | B |
| Drive the real UI | **No** — build state writes instead | B (reduced) |

---

## Evidence status

Every claim below is tagged. This matters because two claims that reached me from workers
were wrong in the file path or the line, and I only found that by re-deriving them.

- `[lead]` — I read the source myself in this session and quote it.
- `[worker]` — a worker reported it and I did **not** re-derive it. Treat as plausible, not proven.

---

## The findings that change the shape

### 1. `WorldCommand::apply` tells you nothing `[lead]`

`simulation/src/world_command.rs:223`:

```rust
pub fn apply(&self, sim: &mut Simulation) {
```

It returns `()`. Not `Result`, not the id of anything it created. An MCP tool
`build_company(...)` therefore cannot report whether it built anything, where, or what the
entity id is. Note also lines 224-225: **the money is deducted before the match arm runs**,
so a command that does nothing at all still charges the player.

**Consequence for the design.** Every write tool must be a *diff*: snapshot the relevant
component storage, apply, snapshot again, and report the delta. That is not a workaround to
be embarrassed about — it is strictly more honest than a return value would be, because it
observes the world rather than trusting the mutation. But it is real work, and it is per-tool.

**Recommendation.** Do not change `WorldCommand::apply`'s signature to fix this. That
signature is load-bearing for the replay system (`world_command.rs:227-231` pushes every
command into `Replay`). Diff in the harness instead.

### 2. The repo's only determinism check cannot fail `[lead]`

`simulation/src/tests/test_iso.rs:241-306`. Reported to me as `simulation/tests/test_iso.rs`
— that path does not exist; the real one is under `src/`. The line numbers were right.

The test builds two simulations from one replay, advances both, and compares them at
checkpoints. Every mismatch branch looks like this (lines 276-283):

```rust
if !sim.is_equal(&sim2) {
    println!("not equal sim+sim2");
    sim.save_to_disk("world");
    sim2.save_to_disk("world2");
    check_start = tick - check_size;
    check_size = check_size / 2;
    continue 'main;
}
```

There is **no `assert!` and no `panic!` anywhere in the function.** On divergence it halves
`check_size`, restarts, and narrows toward the divergence point to dump two save files for a
human to diff. When `check_size` reaches 0 the loop breaks (lines 252-255) and the test
**returns normally — green**.

This is a *bisection debugger* wearing a `#[test]` attribute. As a debugger it is good. As a
guard it is vacuous: determinism could break tomorrow and CI would stay green.

**Consequence.** Reproducibility is the property the entire MCP harness would rest on — an
agent that cannot re-run a scenario and get the same world cannot debug anything. That
property is currently **structurally likely and formally unproven**. This is the single
highest-value thing to fix, and it is small: turn the divergence branches into failures once
the narrowing has done its job.

**Recommendation.** Fix this *first*, before any MCP work. It is a few lines, it is
independent of everything else in this proposal, and it converts the harness's foundation
from assumed to verified.

### 3. Companies are unclickable, so synthetic UI input solves nothing `[lead]`

`native_app/src/gui/tools/selectable.rs:8-17`:

```rust
pub fn select_radius(id: AnyEntity) -> f32 {
    match id {
        AnyEntity::VehicleID(_) => 5.0,
        AnyEntity::TrainID(_) => 10.0,
        AnyEntity::WagonID(_) => 10.0,
        AnyEntity::FreightStationID(_) => 0.0,
        AnyEntity::CompanyID(_) => 0.0,
        AnyEntity::HumanID(_) => 3.0,
    }
}
```

A radius of `0.0` with a `dist2 >= rad * rad` rejection means `0.0 >= 0.0`, which is always
true. **No click at any position can ever select a Company or a FreightStation.**

The whole point of the synthetic-input capability was to automate the inspector sequence:
select a company, open the debug inspector, read its state. Step one of that sequence is not
slow or fiddly — it is *impossible*. Synthetic mouse input would faithfully reproduce a dead
end.

**This also corrects instructions I gave the user earlier in this session.** I said "left-click
the company you built". That could never have worked. The real human path is: click the
*building*, which opens the yakui building panel, then click the owner hyperlink inside it,
then F3, then tick the debug-inspector checkbox `[worker]`.

**Recommendation.** Do not build synthetic input. Write the three UI resources directly. The
entire five-act manual sequence collapses into one tool call, and it is robust against the UI
being immediate-mode with no stable widget ids.

### 4. Frame capture is a modification, not new work — but not where I was told `[lead]`

A working GPU→CPU→PNG readback already runs in production. `engine/src/texture.rs:236`:

```rust
pub fn save_to_file(&self, device: &Device, queue: &wgpu::Queue, path: PathBuf, mip_level: u32)
```

and its one live caller, `native_app/src/gui/hud/toolbox/building.rs:313`, uses it to cache
building thumbnails. So the round trip is exercised every session.

But the report that "capture reads the swapchain" does not hold. I found **no capture call in
the render path at all** — `engine/src/framework.rs` and `engine/src/gfx.rs` contain no
`save_to_file` and no screenshot code. More decisively, the surface is configured at
`gfx.rs:316-318` with:

```rust
usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
```

**No `COPY_SRC`.** The swapchain texture cannot be copied to a buffer as configured. Offscreen
render targets *can* — `gfx.rs:983-985` creates them with `COPY_SRC`.

**Consequence, and it is good news.** The capture should read an offscreen target, not the
swapchain — which is exactly the path `save_to_file` already serves, and avoids touching the
surface configuration. `save_to_file` bails on non-RGBA8 formats (`texture.rs:243-246`), so the
target's format must be checked.

The headless constraint is separate and still binds: adapter selection passes
`compatible_surface: Some(&surface)` at `gfx.rs:285`, and only a Radeon ICD is installed with
no software rasteriser `[worker]`. So frames need a real window on this machine today.

---

## Architecture

### Tier A — `sov-mcp`: a new crate that owns a `Simulation`

This is the shape you approved, and it holds.

```
sov-mcp (new crate, bin)
  ├── owns: Simulation, SeqSchedule          — no window, no GPU, no event loop
  ├── transport: stdio (JSON-RPC)
  └── tools: tick, apply_command, query_*, save, load, world_hash
```

**SDK: `rmcp = "=3.1.4"`** `[worker, with live probe]`, Apache-2.0, which is one-way
compatible into this repo's GPL-3.0. Features: `server`, `transport-io`, `macros`, `schemars`,
`local`, with `default-features = false`. 57 crates resolved, all permissive licences.

The decisive feature is **`local`**, which drops the `Send + Sync` bound from rmcp's `Service`
trait entirely. The researcher compiled and ran a server owning a deliberately `!Send + !Sync`
struct on a `current_thread` runtime inside a `LocalSet`, and demonstrated state persisting
across tool calls. That means the harness can hold a `Simulation` in a plain `RefCell` with no
`Arc<Mutex<…>>` and no threading discipline — which matters, because a fixed-timestep
deterministic sim should never be touched from two threads.

**Pin it with `=`.** This SDK went 2.x → 3.x in under a month and shipped five releases in
August 2026 alone `[worker]`.

Two constraints, both benign:
- Handlers take `&self`. Interior mutability is mandatory. Never hold a `borrow()` across an
  `.await` — a `RefCell` double-borrow panics.
- One thread means a long `tick(100000)` blocks the server; there is no concurrent query
  mid-tick. For a deterministic sim, serialised execution is a feature.

**Two ergonomic problems to fix in Tier A** `[worker]`: the save path `world/{name}.zip` is
hardcoded relative to CWD, and `init()` requires CWD to be the repo root. Both need to become
parameters, or the harness is not runnable from anywhere but one directory.

**Third-party pattern check** `[worker]`: mature MCP-in-a-game work (Godot, Unity, Unreal) all
solves a *main-loop mismatch* — the engine runs free and the MCP server must queue commands
into it, typically by polling from inside the loop. **Tier A has no such mismatch, because it
owns the clock.** Tools drive time. That sidesteps the hardest problem the field has.

### Tier B — in-process inside `native_app`: frames and UI state

`native_app` is a binary crate with no `[lib]` section `[lead]` and private modules, and
`engine::framework::start` never returns `[worker]`. An external crate therefore **cannot link
it and cannot reach the UI at all.** Tier B must live inside `native_app`, behind a feature
flag.

```
native_app --features mcp
  ├── an MCP listener on its own thread
  ├── a command queue drained ON THE MAIN THREAD, inside the existing frame loop
  └── tools: capture_frame, set_ui_state, apply_command, tick
```

This is the "poll from inside the loop" pattern the Godot bridges use, and it is the only
shape that respects `UiWorld` being `!Sync` via `RefCell` `[worker]`.

`set_ui_state` writes the three resources directly — `GuiState.debug_window`,
`DebugState.debug_inspector`, `InspectedEntity.e` `[worker]` — instead of simulating five
clicks, one of which is impossible.

`capture_frame` renders to an offscreen `COPY_SRC` target and reuses `Texture::save_to_file`'s
readback, then encodes for transport.

### The image ceiling — the sharp edge `[worker, measured]`

Claude Code gates MCP tool results on `MAX_MCP_OUTPUT_TOKENS` (default 25,000), **measured
against the base64 payload as text**, while the real context cost of an image is its visual
tokens. A 1920×1080 frame costs ~2,691 tokens of actual context but is scored at roughly
90,000 by the gate — over-counted about 30×.

Measured on a real 1080p frame of this game (`assets/screen2.jpg`):

| Encoding | base64 chars | Fits default 25k? |
|---|---|---|
| 1920×1080 **PNG** | 2,662,300 | No — ~30× over |
| 1920×1080 JPEG q70 | 353,332 | No |
| 1280×720 WebP q70 | 65,476 | Yes |
| 800×450 JPEG q70 | 54,192 | Yes |

**Never send PNG of a rendered frame** — and PNG is the naive choice, because the `image`
crate is already wired for it and `save_to_file` writes PNG.

The failure mode is quiet: the tool returns successfully, the payload is spilled to a file, and
the agent simply never sees the frame. **Mitigations:** encode WebP, downscale before
encoding, cap the payload inside the tool, set `MAX_MCP_OUTPUT_TOKENS` in the server's
`.mcp.json` env block, and take a `width`/`height` argument so a cheap thumbnail is available
when the agent only needs to confirm something rendered. Verify visually on the very first
frame; do not trust the tool's own success.

One more transport trap: **stdout is the protocol channel.** Any `println!` reaching stdout
corrupts JSON-RPC. This crate must redirect every sim log away from stdout.

---

## What NOT to build

1. **Synthetic mouse and keyboard input.** Finding 3. It automates a dead end.
2. **A changed `WorldCommand::apply` signature.** Finding 1. Diff in the harness instead.
3. **Headless rendering, for now.** Only a Radeon ICD is installed and adapter selection wants
   a surface `[worker]`. Installing lavapipe is a separate, cheap experiment — not a
   precondition for Tier A, and not something to bundle into this work.

---

## Recommended sequence

| Step | What | Gate |
|---|---|---|
| **0** | Make `test_world_survives_serde` able to fail | The mutation: force a divergence, see it go **red** |
| **1** | Tier A skeleton — `sov-mcp`, stdio, `tick`, `world_hash` | Two runs of the same script produce the same hash |
| **2** | Tier A writes — `apply_command` with before/after diffs | A build command reports the entity it created |
| **3** | Tier A persistence — `save`/`load` with a path parameter | Save, load, hash — hash matches |
| **4** | Tier B `capture_frame` behind `--features mcp` | A real frame arrives and is *visually confirmed* |
| **5** | Tier B `set_ui_state` | The debug inspector opens on a named company |

Step 0 is independent and should happen regardless of whether the rest is approved.
Steps 1-3 are the bulk of the value and carry the least risk. Steps 4-5 are the ones that
retire the "human at the keyboard" cost, and they are the riskier half.

---

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| The image gate silently eats frames | **High** — reads as success | WebP, downscale, cap in-tool, raise env var, verify the first frame by eye |
| `rmcp` release velocity | Medium | Pin with `=`, expect one migration |
| Determinism is unproven | **High** — the foundation | Step 0, before anything else |
| Every write tool needs a bespoke diff | Medium — it is per-tool cost | Accept it; it is more honest than a return value |
| `!Send` sim + real wgpu/winit untested together | Medium | Tier A has no GPU, so this only bites at step 4 |
| No public prior art for `rmcp` driving a game | Low | The pattern is proven in C#/TS; only the exact combination is new |

---

## Open decisions

1. **Do step 0 now, separately?** I recommend yes. It is small, independent, and it is the
   only thing here that makes an existing guarantee real rather than assumed.
2. **Tier A only, or both tiers?** Tier A is the safe majority of the value. Tier B is what
   removes you from the loop on visual work.
3. **Try lavapipe?** A software Vulkan rasteriser would make step 4 headless. It is a package
   install and one experiment, and it either works or it does not.

---

## Provenance

Four research lanes, 2026-08-27: `researcher` (MCP SDK, transport, images, prior art),
`substrate-cartographer` (the simulation seam), `engine-implementer` (headless and capture),
`ui-implementer` (the UI seam). Their reports are the `[worker]` claims. The `[lead]` claims
were re-derived from source in the main session, and two of them corrected what a report said.
