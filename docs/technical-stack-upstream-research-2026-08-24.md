# Technical stack: live substrate and upstream research

**Research date:** 2026-08-24
**Scope:** the checked-out `main` tree at `186e081`, upstream release/source
metadata retrieved 2026-08-24. This is an upgrade and hardening assessment, not
a rewrite proposal.

## Executive finding

The game is a capable, custom Rust desktop engine rather than an off-the-shelf
engine: a forward wgpu renderer feeds both yakui and egui; a serial,
slot-map-based simulation owns gameplay state; and save/network payloads use
Serde plus bincode and a bespoke TCP/UDP transport. The most important
technical work is to **make the current stack reproducible and bounded before
attempting ecosystem upgrades**. The renderer already contains high-value
features (PBR IBL, cascaded shadows, depth prepass, SSAO, fog, MSAA, GPU
instancing); replacing it would discard working value and compound the
wgpu/winit/UI migration.

There are four material risks:

1. `egui` has no `rev` and yakui tracks a personal fork's `dev` branch. The
   lockfile happens to resolve them today, but a fresh resolution is not a
   reproducible build input.
2. The renderer is on wgpu 0.20.1 / winit 0.29.15, while current upstream
   releases are wgpu 30.0.1 and winit 0.30.13. This is a deliberately staged
   migration, not a patch-level bump: it crosses several breaking API eras and
   the UI render bridges are version-coupled.
3. Bincode 1 saves have no schema/version migration envelope. The code warns
   then tries to decode an incompatible save, and the upstream bincode 2 line
   is a format/API migration, not a compatible update.
4. Networking reads peer-controlled frame sizes into growable buffers, has no
   authenticated/encrypted transport, and uses `unsafe impl Send + Sync` for a
   phantom wrapper. Treat multiplayer as trusted-LAN/development-only until
   bounded framing and threat-model work lands.

## Lead synthesis and decision

**Decision:** retain the custom engine, renderer, and typed simulation. Improve
them through measurement, correctness hardening, visibility partitioning, and a
staged dependency migration. A broad engine, ECS, renderer, UI, networking, or
async-runtime rewrite is not justified by the evidence collected in this audit.

### Architecture verdict

- The engine is a direct Rust/wgpu/winit desktop stack, with an additional wasm
  branch, rather than a general-purpose third-party engine integration.
- Presentation is already technically capable: forward PBR/IBL, depth prepass,
  four cascaded shadows, SSAO, fog, MSAA, terrain LOD, GPU instancing, Yakui,
  and legacy/debug egui.
- The authoritative simulation is a project-owned typed slot-map world. It is
  neither Bevy nor an archetype ECS; scheduled systems run serially and apply
  typed command buffers at system boundaries.
- Simulation preparation, UI, and rendering share the main frame thread.
  Rayon assists selected CPU work and command encoding, but GPU command buffers
  still submit to one queue.
- Lua/Luau prototypes, glTF/PNG assets, compressed bincode saves, and optional
  bespoke TCP/UDP multiplayer complete the live technical substrate.

### Five findings that control the roadmap

1. **The 250,000-identity performance claim is currently unproven.** None of
   the charter's five named headless benchmark gates exists, and the renderer
   has no per-pass GPU timestamps.
2. **The renderer has confirmed candidates for avoidable work.** PBR environment
   convolution runs every frame; fog still incurs a pass when disabled; UI blur
   runs unconditionally. Measurements must rank them before implementation.
3. **Visibility scaling is the largest renderer architecture risk.** Dynamic
   entities are scanned and uploaded without per-instance culling or LOD, while
   cached road/building chunks are submitted without the chunk culling already
   demonstrated by trees.
4. **Correctness boundaries precede feature expansion.** Mutable global statics
   include the known parallel-test race; saves lack a migration envelope; TCP
   framing and save decompression need explicit allocation limits; multiplayer
   remains trusted-LAN/development-only until those boundaries are hardened.
5. **Graphics modernization is one coupled migration.** wgpu, winit, egui,
   egui-wgpu, Yakui, and Yakui-wgpu cannot be upgraded safely as unrelated
   version bumps. Pin the current Git revisions and establish baselines first.

### Recommended delivery sequence

| Order | Bounded outcome | Concrete estimate |
|---:|---|---:|
| 1 | Pin Git dependencies and the Rust toolchain; prove a locked clean build. | 3-6 hours |
| 2 | Add five headless scale gates and per-pass renderer GPU timestamps. | 2-4 engineering days |
| 3 | Remove unsafe initialization globals; bound save/network inputs; introduce a versioned save envelope. | 3-6 engineering days |
| 4 | Measure and condition PBR/fog/blur, then add presentation-only chunk/instance culling and LOD. | 5-10 engineering days |
| 5 | Migrate the graphics and UI dependency tranche on an isolated compatibility branch. | 1-3 engineering weeks |

State sorting, bindless rendering, indirect GPU-driven draws, Tokio/QUIC, and an
ECS replacement remain deferred unless the new measurements identify a named
charter gate that the simpler work cannot meet.

## What is actually live

| Area | Confirmed current substrate | Evidence in this checkout | Upstream status and implication |
|---|---|---|---|
| Application/window | Custom `engine::framework`: winit event loop, `ControlFlow::Poll`, redraw requested after every frame; desktop window plus a wasm branch. | [`engine/src/framework.rs`](../engine/src/framework.rs), [`engine/Cargo.toml`](../engine/Cargo.toml) | Lock: winit 0.29.15. [winit 0.30.13](https://github.com/rust-windowing/winit/releases/tag/v0.30.13) is current; do not update it separately from wgpu/UI bridges. |
| GPU renderer | wgpu 0.20.1, surface configured as sRGB/FIFO with maximum frame latency 2; GPU backends selected from environment; forward passes include PBR/IBL, depth, cascaded shadows, SSAO, fog, background and UI blur. Renderer can record independent command buffers in Rayon when enabled. | [`engine/src/gfx.rs`](../engine/src/gfx.rs), [`engine/src/passes`](../engine/src/passes), [`Cargo.lock`](../Cargo.lock) | [wgpu 30.0.1](https://github.com/gfx-rs/wgpu/releases/tag/v30.0.1) is current. Current [surface ownership API](https://github.com/gfx-rs/wgpu/blob/trunk/wgpu/src/api/surface.rs) explicitly ties a `Surface` to its window; the project already keeps an `Arc<Window>`, which is a useful migration starting point. |
| UI | Both render every frame: yakui drives the primary HUD/tool panels, while egui remains in legacy/debug paths. Both share the wgpu device and winit event stream. | [`engine/src/yakui.rs`](../engine/src/yakui.rs), [`engine/src/egui.rs`](../engine/src/egui.rs), [`native_app/src/gui`](../native_app/src/gui) | Lock: egui family 0.27.2 at commit `d4e8966`; yakui 0.2.0 at `6c6982f`. Current [egui 0.36.1](https://github.com/emilk/egui/releases/tag/0.36.1) contains major 0.34/0.35 API changes, including a `Context::run` to `run_ui` direction and removal of deprecated APIs ([changelog](https://github.com/emilk/egui/blob/main/CHANGELOG.md)). Yakui upstream's last push was 2024-07-13 ([repository](https://github.com/Uriopass/yakui)); it is a maintenance risk, not an immediate replacement mandate. |
| Audio/input | `engine::AudioContext` combines CPAL output, oddio mixing, and lewton Ogg/Vorbis decoding; the native app layers music, ambient, and vehicle sound controllers over it. `InputContext` consumes winit window/device events and the native app maps them into authored actions. | [`engine/src/audio.rs`](../engine/src/audio.rs), [`engine/src/input.rs`](../engine/src/input.rs), [`native_app/src/audio`](../native_app/src/audio), [`native_app/src/inputmap.rs`](../native_app/src/inputmap.rs) | Retain this owned seam and update/test it independently from graphics. CPAL carries platform-specific host dependencies; no evidence collected here supports an audio/input rewrite or claims cross-platform device coverage. |
| Simulation/ECS | This is not Bevy and not an archetype ECS. `World` owns typed `HopSlotMap` storages keyed by generational IDs; a serial schedule applies typed parallel command buffers after each system. | [`simulation/src/world.rs`](../simulation/src/world.rs), [`simulation/src/utils/scheduler.rs`](../simulation/src/utils/scheduler.rs) | `slotmapd` 1.0.11 and `flat_spatial` 0.6.1 are locked. The current model supports stable identity and Serde persistence; its scaling question is measured system cost, not an ECS rewrite. |
| Data/prototypes | Lua/Luau prototypes are loaded through mlua 0.9.9; glTF 1.4.1 and PNG-only `image` 0.25.1 import visual assets. | [`prototypes/Cargo.toml`](../prototypes/Cargo.toml), [`engine/Cargo.toml`](../engine/Cargo.toml) | Prefer individual minor updates after a build/test gate; changing the Lua runtime or asset import path together with rendering would make failures non-local. |
| Save/serialization | Serde 1.0.203, bincode 1.3.3, serde_json 1.0.118, miniz_oxide. Production saves use zlib-compressed bincode; JSON is used for replay/config-style data. | [`common/src/saveload.rs`](../common/src/saveload.rs), [`simulation/src/lib.rs`](../simulation/src/lib.rs), [`Cargo.lock`](../Cargo.lock) | [bincode 2's migration guide](https://docs.rs/bincode/latest/bincode/migration_guide/index.html) documents a different API/configuration model. Keep bincode 1 for existing saves until an explicit versioned-envelope migration is tested. |
| Networking | Optional native-app multiplayer and the headless binary use custom blocking `std::net` TCP + UDP workers, `std::sync::mpsc`, compressed bincode messages, and lockstep-like frame inputs/world transfer. No Tokio, QUIC, TLS, or external networking crate is present. | [`networking/src/connections.rs`](../networking/src/connections.rs), [`networking/src/lib.rs`](../networking/src/lib.rs), [`native_app/src/network.rs`](../native_app/src/network.rs) | This is an owned protocol, so there is no dependency upgrade that supplies safety. Add protocol limits/tests before exposing it outside a trusted LAN. |
| Build/perf | Rust 2021 workspace; native app is default member. The local compiler is 1.97.1. Development builds optimise dependencies at level 2; release has no additional project settings. Rayon is globally forced to 8 threads; optional Tracy is wired through `profiling`. No checked CI workflow or packaging matrix establishes which OS/GPU backends are supported; the wasm branch compiles conditionally in source but was not proven as a deliverable. | [`Cargo.toml`](../Cargo.toml), [`engine/src/framework.rs`](../engine/src/framework.rs), [`native_app/Cargo.toml`](../native_app/Cargo.toml) | [Rust 1.98.0](https://github.com/rust-lang/rust/releases/tag/1.98.0) is current. Reproducibility needs a `rust-toolchain.toml`/MSRV policy, pinned Git dependencies, `--locked` CI, and an explicit platform/package matrix before a release. Performance work should begin with the existing Tracy scopes and charter benchmarks, not guesses. |

### Dependency versions resolved by the lockfile

The manifest ranges are not the deployed truth. These are the resolved versions
in `Cargo.lock`: wgpu 0.20.1, winit 0.29.15, egui/egui-wgpu/egui-winit 0.27.2,
yakui family 0.2.0, Serde 1.0.203, bincode 1.3.3, mlua 0.9.9, Rayon 1.10.0,
pathfinding 4.10.0, glTF 1.4.1, image 0.25.1, CPAL 0.15.3, oddio 0.7.4,
slotmapd 1.0.11, flat_spatial 0.6.1, and profiling 1.0.15. The distinction
matters: the root manifest directly tracks Git heads for egui, and a branch for
yakui.

## Renderer and engine analysis

### Strengths to retain

- The render loop is conventional and direct: simulation/UI construct drawables,
  passes encode GPU work, then the surface texture is presented. It has no
  general-purpose engine layer to unwind.
- The renderer chooses an sRGB surface format where available, uses FIFO by
  default, has runtime vsync/MSAA/shadow/SSAO/fog controls, and makes all
  presentation state visible in `GfxContext`. Those are good seams for profiling
  and incremental feature work.
- Parallel command-buffer encoding is already optional. This is compatible with
  wgpu's command-encoder model, but it must be benchmarked per target GPU:
  CPU recording can help while GPU-bound scenes will not improve.
- Shader/pipeline invalidation exists around feature defines. That is a practical
  base for renderer experimentation without replacing the pipeline architecture.

### Findings that should change near-term engineering

**Fact — validation is effectively disabled.** In debug, `GfxContext` selects
`InstanceFlags::DEBUG` and comments out validation due to an old wgpu issue;
release uses empty flags. Modern wgpu documents enabling validation via
`InstanceFlags::VALIDATION` ([official debugging guide](https://github.com/gfx-rs/wgpu/wiki/Debugging-wgpu-Applications)). **Recommendation:** after pinning the
current lockfile, add an opt-in developer environment/config switch for current
version validation and capture one render trace before performing a renderer
migration. Do not silently force validation in release.

**Fact — surface loss recovery can panic.** `Outdated`, `Lost`, and
`OutOfMemory` are grouped; the code reconfigures and then `expect`s another
frame. **Inference:** device loss or genuine OOM becomes an avoidable process
abort instead of an exit/error path. **Recommendation:** split `OutOfMemory` to
a controlled shutdown/report path and retry only recoverable surface states
before the wgpu migration.

**Fact — the app polls continuously.** `ControlFlow::Poll` plus
`request_redraw()` after every presented frame means it intentionally renders as
fast as it can even when idle. **Inference:** this can waste laptop power and
make CPU-side perf numbers less interpretable; it may be correct for current
simulation timing. **Recommendation:** profile an `WaitUntil`/fixed-tick
experiment behind a switch, preserving simulation cadence and input latency;
make no default behaviour change without a frame-time and gameplay comparison.

**Fact — Rayon is pinned to eight worker threads globally.**
`ThreadPoolBuilder::build_global()` errors are ignored. **Recommendation:**
derive the policy from available parallelism or expose a launch setting, then
measure it on the project’s 250k scenarios. It is a performance investigation,
not a safe assumption that more workers are faster.

## Simulation, persistence, and networking analysis

### ECS/simulation

The custom typed-world architecture is coherent with this game: entity identity
is generational and stored separately by kind, systems execute in declared
order, and command buffers are applied at each system boundary. This supports
the project’s deterministic tests and authoritative-state presentation. There
is no evidence in the current tree that an ECS library migration would unlock a
near-term 1.0 requirement; it would instead jeopardize save and lockstep
behaviour. Continue to improve data locality only where a profiler identifies a
system/hot storage traversal.

### Save format

`CompressedBincode` decompresses a whole input into a `Vec` and immediately
deserializes it. Sim saves are versioned only by the application-level attempt
to decode; on a mismatch the simulation logs a warning and still tries.
Accordingly, old saves are neither safely rejected nor migrated. The smallest
sound enhancement is a new, explicit envelope for *new* saves:

1. magic bytes + save-schema version + format/compression version;
2. an uncompressed and compressed size limit before allocation/decompression;
3. an explicit migration table or a clear incompatibility refusal;
4. golden fixtures for at least the current schema and the first migrated
   schema.

This preserves the existing data model and makes later bincode 2 adoption a
separate decision. It also addresses local corrupted-save robustness without
claiming bincode itself is an untrusted-input format.

### Networking security/correctness gap

**Fact:** TCP framing accepts a `u32` advertised frame length, then allocates a
`Vec` of that length; no maximum is checked in `FramedTcpReceiver`. UDP accepts
datagrams up to a 65,536-byte stack buffer. World transfer has a distinct
262,144-byte fragment constant, but it does not bound generic TCP frames.
`PhantomSendSync<T>` declares every `T` Send and Sync unsafely.

The transport also has no documented bound for internal work queues, shutdown
and join contract for its blocking worker threads, backpressure policy, or
listener exposure policy. Those lifecycle risks are distinct from malformed
frame handling: even valid traffic needs a defined overload and termination
behavior.

**Recommendation:** before any internet-facing multiplayer claim, define the
trust boundary and add: maximum control/input/frame/world sizes; cumulative
transfer quotas/timeouts; disconnect-on-violation; fuzz/property tests for
fragmentation and oversized lengths; protocol version negotiation; and removal
or a written proof of the broad unsafe marker. Encryption/authentication is a
product/hosting decision after this baseline, not a dependency swap. This is
the highest technical hardening item because a peer controls the bytes.

## Upstream divergence

The fork point is documented as upstream Egregoria `ae65c857` on 2026-08-22.
The current upstream repository is not archived, and its `master` still resolves
to that same 2025-06-02 commit ([commit](https://github.com/Uriopass/Egregoria/commit/ae65c857948a905120474cf93b96dd51cec6d5f6), [official empty compare](https://github.com/Uriopass/Egregoria/compare/ae65c857948a905120474cf93b96dd51cec6d5f6...master)). **Fact:** there are zero upstream commits to merge after the fork baseline as of this research date; divergence is entirely the fork’s own work. The official `v0.6.1` and `v0.6.2` tags are 2023 releases and are not the imported 2025 baseline ([tags](https://github.com/Uriopass/Egregoria/tags)).

This makes the dependency situation more important: upstream Egregoria cannot
be expected to carry the renderer/UI stack forward. The practical inheritance
strategy is to preserve the mature engine substrate, pin it, and upgrade its
external dependencies in independently testable slices.

## Prioritized update matrix

| Priority | Bounded action | Why now / evidence | Completion evidence |
|---|---|---|---|
| **Now** | Pin `egui*` and `yakui*` to the lockfile commits in `Cargo.toml`; record a Rust toolchain policy and run `cargo build --locked`. | Current Git dependencies are branch/HEAD inputs, explicitly non-reproducible. | Fresh clean checkout builds with `--locked`; dependency source/commit inventory is recorded. |
| **Now** | Bound TCP frames and save decompression; add oversized/truncated-input tests; document multiplayer as trusted-LAN until done. | Peer/local file bytes can cause unbounded allocation/decompression; no library update fixes owned code. | Mutation/fuzz-like tests visibly fail before limits and pass after; rejected input leaves process alive. |
| **Now** | Add an opt-in current wgpu validation/trace path and profile baseline scenes with existing Tracy scopes. | Validation is disabled by a stale comment; migration without baseline evidence is blind. | One documented trace and reproducible CPU/GPU baseline per target scenario. |
| **Next** | Make surface OOM/loss handling non-panicking, and measure redraw/thread-pool policies. | Current recovery conflates OOM with recoverable surface errors; continuous polling/8 threads are policy choices. | Error-path test/manual probe plus before/after frame-time, power, and input-latency data. |
| **Next** | Design and test a save envelope/migration boundary while retaining bincode 1 payloads. | Existing decode warning cannot protect saves across schema changes. | Versioned golden fixtures, explicit refusal/migration result, no silent old-save decode. |
| **Later** | Perform one compatibility branch for wgpu + winit + both UI bridges, upgrading each bridge to a version that declares the same wgpu/winit support. | wgpu 30/winit 0.30/egui 0.36 are far ahead; egui itself has major API changes and yakui is stale. | Compile, shader validation, visual regression captures, and playthrough of UI/input/resizing on Linux/Windows. |
| **Later** | Update registry crates in small batches, using lockfile diff, tests, and vulnerability scanning. | Most core registry dependencies already resolve newer compatible versions than their manifest minima; broad update hides regressions. | One-category update PR/commit with test and advisory results. |
| **Avoid** | Broad engine, renderer, ECS, UI, async-runtime, or networking rewrite. | The owned renderer/simulation are functioning and the risk is concentrated at concrete boundaries. | Reconsider only if measurements show a named charter gate cannot be met. |

## Local renderer and performance audit addendum

The update order above is reinforced by a read-only audit of the current render
paths. These are code-confirmed costs or missing gates; they are not claims that
a profiler has already ranked them.

1. **The five charter-scale performance gates do not exist.** The charter names
   `bench_services`, `bench_terrain`, `bench_chains`, `bench_rail`, and
   `bench_save` at 250,000 identities, but the tree contains no corresponding
   runner, Cargo bench target, or checked threshold. The `easybench` use in
   `map_dynamic::dispatch` is an ordinary unit test, not a release gate.
   Measurement infrastructure is therefore the first performance feature.

2. **Dynamic presentation work scales with total entities, not visible
   entities.** `InstancedRender` scans vehicles, wagons, humans, and itineraries
   each frame; populated instance vectors are uploaded in full. Instanced meshes
   select `lods.first()` and draw the entire instance range, with no per-instance
   frustum culling or LOD selection. With four shadow cascades enabled, the same
   geometry can participate in depth, main, and four shadow passes. See
   [`native_app/src/rendering/entity_render.rs`](../native_app/src/rendering/entity_render.rs),
   [`engine/src/drawables/instanced_mesh.rs`](../engine/src/drawables/instanced_mesh.rs),
   and [`engine/src/gfx.rs`](../engine/src/gfx.rs).

3. **Static map caching is sound, but visibility submission is incomplete.**
   Road/building meshes rebuild only for dirty subscriber chunks, which is a
   strong retained-mode seam. Steady-state rendering nevertheless submits all
   cached road/building chunks without the chunk-level frustum filtering already
   used by trees. The next structural renderer improvement should reuse that
   presentation-side chunking rather than alter authoritative simulation state.

4. **PBR environment work is regenerated every frame.** The enabled PBR path
   records six environment faces, diffuse irradiance faces, and specular
   prefilter face/mip passes on every frame. This is the clearest controlled A/B
   candidate: add GPU timestamps, measure PBR on/off, then test angular-threshold
   or progressive updates. See [`engine/src/passes/pbr.rs`](../engine/src/passes/pbr.rs)
   and [`engine/src/gfx.rs`](../engine/src/gfx.rs).

5. **Some quality settings do not remove their pass cost.** The fog setting
   changes a shader define, but the early return in `render_fog` is commented
   out, so the fog pass still runs. UI blur is also generated unconditionally,
   including while the interface is hidden. Both should be measured and made
   demand-driven only after per-pass GPU timing exists. See
   [`engine/src/passes/fog.rs`](../engine/src/passes/fog.rs) and
   [`engine/src/passes/blur.rs`](../engine/src/passes/blur.rs).

Current debug counters cover CPU preparation/encoding, draw calls, and
triangles, but render passes have no GPU timestamp queries. A useful capture
mode should time depth, each shadow cascade, PBR, SSAO, fog, main scene, blur,
Yakui, and egui over a fixed save/camera for 300 warmed release frames. Only
after those measurements should the project choose among culling, pass
frequency, draw sorting, or GPU-driven submission.

## Verification snapshot

- `cargo check --workspace --all-targets`: passed on Rust 1.97.1; warnings remain,
  including Rust-2024 `static mut` references and one future-incompatible
  transitive `document-features 0.2.8`.
- `cargo test -p simulation -- --test-threads=1`: 26 passed, 0 failed, 0
  filtered; 21.89 seconds. The single-thread flag is mandatory because
  `simulation/src/init.rs` contains the known unsynchronized `static mut`
  initialization race; parallel runs can intermittently segfault.
- `cargo test --workspace -- --test-threads=1`: 85 passed, 0 failed. This proves
  the checked-out code's unit/behaviour suite, not runtime GPU correctness,
  platform packaging, save migration, or the missing 250k performance gates.

## Recommended order of operations

1. Make the exact current build repeatable (Git revisions, toolchain, locked
   build) and capture performance/visual baseline.
2. Harden the two untrusted-byte boundaries (save and network) with tests.
3. Improve recoverability/observability in the present renderer.
4. Create a disposable wgpu/winit/UI migration branch only after the above
   gates are green; do not merge it until visual and interaction proof exists.

## Research limits

- Release versions above are live checked on 2026-08-24. Compatibility of a
  particular future yakui revision with wgpu 30 was not established; its
  upstream maintenance gap is precisely why that must be proven in a branch.
- No dependency advisory scanner was run in this research pass, so this report
  does **not** claim the lockfile is free of CVEs. A locked `cargo audit` (or
  equivalent advisory scan) belongs in the reproducibility/security follow-up.
- Exact *local* ahead/behind counts are not asserted: the local object database
  does not contain the upstream commit. The upstream-to-baseline comparison is
  nevertheless directly verified as empty above.
- CI, release packaging, wasm execution, Windows behavior, audio-device
  coverage, and GPU-backend coverage were not exercised. Source branches and
  dependencies show intended capability, not verified support.

## Primary sources

- [wgpu releases](https://github.com/gfx-rs/wgpu/releases) and [wgpu API source](https://github.com/gfx-rs/wgpu/tree/trunk/wgpu/src/api)
- [winit releases](https://github.com/rust-windowing/winit/releases)
- [egui releases](https://github.com/emilk/egui/releases), [changelog](https://github.com/emilk/egui/blob/main/CHANGELOG.md), and [integration architecture](https://github.com/emilk/egui/blob/main/ARCHITECTURE.md)
- [yakui repository](https://github.com/Uriopass/yakui)
- [bincode migration guide](https://docs.rs/bincode/latest/bincode/migration_guide/index.html)
- [Egregoria upstream at the fork-tip commit](https://github.com/Uriopass/Egregoria/commit/ae65c857948a905120474cf93b96dd51cec6d5f6)
