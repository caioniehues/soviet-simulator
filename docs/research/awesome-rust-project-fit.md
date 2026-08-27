# Awesome Rust, Are We Game Yet, and Awesome WGPU project-fit review

**Kind:** explanation  
**Authority:** explanatory  
**Status:** research snapshot  
**Owner:** research  
**Last verified:** 2026-08-27

## Question

Which resources indexed by [Awesome Rust][awesome], [Are We Game Yet][awgy], and [Awesome WGPU][awesome-wgpu] fit the current Soviet Simulator Rust/Egregoria hard fork?

## Executive recommendation

Adopt **cargo-deny** first as a CI-only dependency-policy gate. Evaluate **Criterion.rs** only after a headless 250,000-identity benchmark contract exists. Use **cargo-mutants** as a scoped, periodic evidence gate for high-risk simulation changes.

Do not add an engine, ECS, renderer, UI framework, async runtime, serialization format, pathfinding library, scripting runtime, logging framework, or test runner now. These changes do not close a named 1.0 gap. They add migration risk to working seams. This is an **inference**, not a ratified decision.

## Method and source boundary

Awesome Rust, Are We Game Yet, and Awesome WGPU were only discovery indexes. Awesome Rust describes itself as a curated list of Rust code and resources [^awesome]. Are We Game Yet describes its ecosystem as game-development libraries and tools, and says the ecosystem is still young [^awgy]. Awesome WGPU calls itself a curated resource list but has only 21 commits and explicitly labels some links as old, including an engine based on wgpu 0.2 [^awesome-wgpu]. Candidate claims use their primary repositories or documentation. Local claims refer to tree `ba7e8e7ed806a070647609d2e40224d18ece651b`, observed on 2026-08-27.

I screened profiling and benchmarking, deterministic-test evidence, serialization, crash/log reporting, spatial/pathfinding, UI/graphics, Lua data, dependency/release policy, and CI. This is a decision review, not a catalogue inventory.

## Current constraints

| Constraint | Evidence | Consequence |
|---|---|---|
| Custom Rust/Egregoria hard fork; not Bevy. | [CLAUDE.md](../../CLAUDE.md), [charter](../plan/charter-1.0.md) | Reject engine and ECS replacements. |
| Authoritative world is serial and command-first. Repeat-run determinism is not proven. | [substrate map](../reference/architecture/substrate.md), [scheduler](../../simulation/src/utils/scheduler.rs) | Tests need stable identity and authoritative-state oracles. |
| Charter target is 250,000 identities at 60 fps. Headless gates do not exist. | [charter](../plan/charter-1.0.md), [technical-stack audit](../explanation/research/technical-stack-upstream-2026-08-24.md) | Benchmark tools need a representative scenario and host policy. |
| `profiling`/Tracy, QuickCheck, Rayon, `pathfinding`, `flat_spatial`, Serde, bincode, `log`, and mlua Luau already exist. | [simulation manifest](../../simulation/Cargo.toml), [native-app manifest](../../native_app/Cargo.toml), [root manifest](../../Cargo.toml) | Prefer narrow tools. Do not duplicate capability. |
| Saves are zlib-compressed bincode without a format envelope or migration boundary. | [save codec](../../common/src/saveload.rs), [technical-stack audit](../explanation/research/technical-stack-upstream-2026-08-24.md) | Add owned bounds, fixtures, and migration tests before a format replacement. |
| Charter requires Linux/Windows binaries, no telemetry, panic log, and autosave-on-crash. No checked CI workflow exists. | [charter](../plan/charter-1.0.md), local `.github/` inventory | Prioritize reproducible builds and dependency policy before packaging. |

## Shortlist

| Candidate | Fit and benefit | Integration seam | Cost and risk | Maturity/license | Decision and evidence |
|---|---|---|---|---|---|
| **cargo-deny** | High. Checks advisories, licenses, sources, and duplicate/banned crates. This supports a GPL-3.0 release and makes Git dependency inputs explicit policy. | Versioned `deny.toml`; `cargo deny check` CI job. Start report-only, then make exceptions explicit. | No production dependency. Initial work is Git and transitive-license review. It is not legal advice. | Rust 1.88 minimum; MIT/Apache-2.0. Upstream documents advisory, license, bans, and source checks. | **Adopt first.** It closes a release gap without runtime change. [^cargo-deny] |
| **Criterion.rs** | Medium-high. Statistical comparison and named baselines fit future CPU scale gates. | Add a workspace benchmark only after a spec sets seed, ticks, digest, warm-up, sample policy, and stored result schema. Use a headless scenario, not only a microbenchmark. | Host noise can look like regression. HTML reports require Gnuplot. It cannot measure GPU passes or itself prove 60 fps. | Maintained repository supports stable Rust and is MIT/Apache-2.0. The old Awesome Rust link directs new work to this organisation. | **Evaluate after benchmark contract.** The missing scenario is the blocker. [^awesome] [^criterion-old] [^criterion] [^criterion-ctx7] |
| **cargo-mutants** | Medium. Finds mutations that current tests do not kill. This fits economy, custody, and determinism guards where a green test can be vacuous. | Run on one changed simulation file after focused tests pass. Record survivors in issue evidence. Scan wider only periodically. | Can be expensive. It needs non-flaky `cargo test` or nextest tests and is semi-actively maintained. It cannot replace domain review. | Upstream documents CI integration and its August 2026 maintenance status. License was not stated in the consulted README. | **Adopt as scoped evidence gate.** First trial: one ledger-sensitive module. Confirm license before CI pinning. [^mutants] |

## Reject or defer

| Resource or category | Decision | Reason |
|---|---|---|
| `profiling` / Tracy and other profilers | **No change.** | The code already has `profiling::scope!` and the native Tracy feature. Instrument and measure charter gates first. |
| QuickCheck and Proptest | **No new framework.** | QuickCheck is already a dev-dependency. Add deterministic seeds, replay/digest assertions, and shrinkable domain generators first. [^awesome-testing] |
| cargo-nextest | **Defer.** | Required command is `cargo test -p simulation`; parallel simulation tests are trusted. A different runner does not create determinism evidence. Reconsider only if CI timing proves a bottleneck. [^nextest] |
| `insta`, `rstest`, mocks, golden files | **Defer.** | The next test gap is an authoritative determinism oracle, not test syntax. Add only for a defined test brief. |
| `rkyv`, RON, MessagePack, bincode 2 | **Reject in this tranche.** | Save risk is versioning and bounds, not encoding speed or syntax. Replacement risks saves and lockstep messages. [^awesome-encoding] |
| `tracing` or crash-report replacement | **Defer.** | `tracing` offers structured diagnostics without Tokio, but replacing `log` does not create a local panic log or autosave. Implement those owned behaviours first. [^tracing] |
| New pathfinding, spatial, UI, graphics, scripting, engine, ECS, or async runtime | **Reject.** | Existing pathfinding, spatial, wgpu, egui, Yakui, and mlua seams exist. The charter and substrate favour measured local improvements. |
| cargo-audit and cargo-auditable | **Defer behind cargo-deny and release CI.** | cargo-deny includes advisory checks and adds license/source/bans. cargo-auditable is useful after release binaries exist, not as packaging. [^awesome-security] [^cargo-audit] |

## Game-specific screen

Are We Game Yet confirms that the live stack already occupies the useful game-facing lanes. It lists the same asset loader, audio I/O, game UI, and input components that this workspace already uses. The following decisions are inferences from that overlap and the charter scope.

| Area and index candidates | Current seam | Decision |
|---|---|---|
| Asset loading: `gltf` | `engine` already resolves glTF 1.4.1. | **No change.** Its listed current version is the same resolved version. Add a new loader only for a specified asset source. [^awgy-assets] |
| Audio: CPAL, oddio, lewton; alternatives such as Kira | The engine uses CPAL output, oddio mixing, and lewton decoding. | **No change.** The charter needs bounded ambience and music, not spatial-audio or mixer replacement. [^awgy-audio] |
| Input: winit helpers, gilrs, SDL | The engine has a winit input layer; the charter fixes keybindings. | **Reject.** Gamepad and platform-layer changes are outside the current product need. [^awgy-input] |
| UI: egui, Yakui, Iced, imgui | The project already renders egui and Yakui. | **No change.** Yakui 0.3.0 is now listed upstream, but this is part of the coupled wgpu/winit/UI migration already deferred by the stack audit. Do not update it alone. [^awgy-ui] |
| Physics: collision and physics engines | The charter does not require a general physics system. | **Reject.** Roads, terrain, placement, and vehicles need domain-specific geometry and routing, not rigid-body physics. [^awgy] |
| Networking: GGRS, Quinn, Renet, Laminar | The project owns a TCP/UDP, lockstep-like development transport. | **Defer.** These frameworks would rewrite protocol and determinism boundaries. First add the documented framing limits, quotas, timeouts, and tests. [^awgy-networking] |
| Game engines and ECS | Custom engine and typed slot-map world. | **Reject.** A new engine or ECS conflicts with the hard-fork boundary and risks saves, identities, and physical-goods authority. [^awgy] |

## WGPU-specific screen

Awesome WGPU did not yield a compatible reusable renderer component. The index is useful as historical examples and learning material, but it predates the workspace's wgpu 0.20.1 API era. This is a sourced catalogue fact plus a local compatibility inference.

| Area | Current evidence | Decision |
|---|---|---|
| GPU profiling and timing | Existing CPU scope instrumentation covers the renderer, but every inspected render/compute pass sets `timestamp_writes: None`. | **Do owned work, not a new framework.** Add per-pass GPU timestamp queries behind an opt-in developer gate. This is already the stack-audit recommendation and directly measures the renderer's named hot paths. |
| Validation and capture/debugging | Debug builds use `InstanceFlags::DEBUG`; the source comments out validation due to an old wgpu issue. | **Defer a tool decision.** First pin the graphics dependency tranche, re-test current-version validation, and capture one reproducible trace. No Awesome WGPU tool was verified as compatible with wgpu 0.20.1. |
| WGSL and shader tooling | The engine loads WGSL from `assets/shaders`, processes imports/defines, watches dependencies, and pushes validation error scopes around pipeline compilation. | **No new shader pipeline.** Use the existing compiler and error path. A Naga or external shader-tool insertion would be part of the coupled wgpu/UI migration. |
| Render graphs, asset pipelines, engines, and examples | The renderer already has forward PBR, depth, shadow, SSAO, fog, blur, and UI passes. The index largely points to engines, old examples, or learning material. | **Reject.** A render-graph or renderer replacement gives no charter evidence benefit and would hide the existing passes that must first be measured. [^awesome-wgpu] |

The actionable renderer result is therefore narrow: create GPU timestamp evidence and re-enable a validation/capture experiment after dependency pins. Do not add a rendering dependency from this review.

## Adoption sequence

1. Create a release-hygiene issue for cargo-deny. Baseline the lockfile, state source and license policy, then add a non-mutating CI check.
2. Define one headless 250,000-identity benchmark contract: seed, ticks, digest, warm-up, measurement method, host policy, and stored results.
3. Prototype Criterion against that contract. Keep it only if it separates expected host variance from a regression.
4. Trial cargo-mutants on one changed high-risk simulation module. Keep only survivors that expose a missing behaviour assertion.
5. After CI builds Linux and Windows artifacts from pinned inputs, evaluate cargo-auditable and implement the owned panic-log/autosave contract.

## Explicit no-change recommendation

Do not replace the custom engine, serial typed simulation, wgpu renderer, egui/Yakui UI, Lua/Luau loader, bincode payload, or existing profiler here. The valuable next changes are reproducibility, benchmark evidence, save boundaries, and test strength. These preserve physical-goods causality and persistent identities.

## Uncertainties

- This review did not run a candidate, create CI, or add dependencies.
- The worktree has uncommitted work. This report cites observed files and does not claim a clean baseline.
- The cargo-mutants README consulted here does not state its license. Confirm it before adding a required CI tool.
- GPU timestamps and renderer-scale measurements are separate work. Criterion measures only CPU work in its benchmark target.

## Sources

Accessed 2026-08-27. Primary sources support candidate claims.

[^awesome]: [rust-unofficial/awesome-rust README](https://github.com/rust-unofficial/awesome-rust/blob/main/README.md).
[^awgy]: [Are We Game Yet ecosystem](https://arewegameyet.rs/#ecosystem).
[^awesome-wgpu]: [rofrol/awesome-wgpu README](https://github.com/rofrol/awesome-wgpu/blob/master/README.md).
[^awgy-assets]: [Are We Game Yet 3D format loaders](https://arewegameyet.rs/ecosystem/3dformatloaders/).
[^awgy-audio]: [Are We Game Yet audio](https://arewegameyet.rs/ecosystem/audio/).
[^awgy-input]: [Are We Game Yet input](https://arewegameyet.rs/ecosystem/input/).
[^awgy-networking]: [Are We Game Yet networking](https://arewegameyet.rs/ecosystem/networking/).
[^awgy-ui]: [Are We Game Yet UI](https://arewegameyet.rs/ecosystem/ui/).
[^awesome-testing]: [Awesome Rust testing entries](https://github.com/rust-unofficial/awesome-rust/blob/main/README.md#testing).
[^awesome-encoding]: [Awesome Rust encoding entries](https://github.com/rust-unofficial/awesome-rust/blob/main/README.md#encoding).
[^awesome-security]: [Awesome Rust security-tool entries](https://github.com/rust-unofficial/awesome-rust/blob/main/README.md#security-tools).
[^cargo-deny]: [EmbarkStudios/cargo-deny README](https://github.com/EmbarkStudios/cargo-deny/blob/main/README.md).
[^criterion-old]: [Original Criterion.rs maintenance notice](https://github.com/bheisler/criterion.rs#criterionrs).
[^criterion]: [criterion-rs/criterion.rs README](https://github.com/criterion-rs/criterion.rs/blob/master/README.md).
[^criterion-ctx7]: Context7 documentation for `/bheisler/criterion.rs`, queried 2026-08-27: custom harnesses, named baselines, confidence settings, and CSV output. The maintained repository above is the source for current direction.
[^mutants]: [sourcefrog/cargo-mutants README](https://github.com/sourcefrog/cargo-mutants/blob/main/README.md).
[^nextest]: [nextest-rs/nextest cargo-nextest README](https://github.com/nextest-rs/nextest/blob/main/cargo-nextest/README.md).
[^tracing]: [tokio-rs/tracing README](https://github.com/tokio-rs/tracing/blob/main/README.md).
[^cargo-audit]: [RustSec cargo-audit README](https://github.com/RustSec/rustsec/blob/main/cargo-audit/README.md).

## Related documents

- [1.0 charter](../plan/charter-1.0.md)
- [Substrate architecture](../reference/architecture/substrate.md)
- [Technical stack and upstream audit](../explanation/research/technical-stack-upstream-2026-08-24.md)
