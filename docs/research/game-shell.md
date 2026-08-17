# Research: the game shell in Bevy 0.19 (issue #84)

**Compiled:** 2026-08-17 · Part of #81 · Feeds "'Polished' defined"

**Question:** What does a complete game shell cost in Bevy 0.19? Per brief §4c item: existing crates, what must be hand-rolled, effort ballpark.

## Ground truth about this repo (verified in-tree, 2026-08-17)

- **The shipping UI stack is native `bevy_ui`**, not egui. `src/game/hud.rs` (1,076 lines), `src/game/toolbar.rs` (347 lines) and the plan ledger are all `Node`/`Text`/flexbox with `BackgroundColor`/`BorderColor`. `bevy_egui 0.40` and `bevy-inspector-egui 0.37` are in `Cargo.toml` but **unused anywhere in `src/`** — dev-tool remnants. **Recommendation: build the entire shell in native `bevy_ui` for consistency with the state-document aesthetic; consider dropping the egui deps or gating them behind a dev feature.**
- **No app states exist.** `src/lib.rs` boots `DefaultPlugins` straight into ~20 sim plugins + `GamePlugin`; world spawn happens at `Startup`. There is no `States` usage at all.
- Saves: custom postcard column format, `SAVE_VERSION = 5`, hard version gate (mismatch = error, no migration), single `QUICKSAVE_PATH`, F5/F9 only (`src/game/saveload.rs`, `src/sim/save.rs`).
- Speed/pause already modelled: `SimSpeed { Paused, Normal, Double, Quad }` (`src/sim/clock.rs`), keyboard-driven, HUD readout exists. `TickIndex` exists; no calendar date.
- Camera: hand-rolled `CameraRig` (focus/yaw/pitch/dist, transform derived each frame) with `DistanceFog`, `Bloom`, tonemapping (`src/game/camera.rs`, 124 lines).
- `bevy_kira_audio 0.26` is already a dependency.

## Crate compatibility (checked against index.crates.io, 2026-08-17)

All latest **stable** versions, with their declared Bevy requirement:

| Crate | Version | Bevy req | Relevance |
|---|---|---|---|
| leafwing-input-manager | 0.21.0 | ^0.19 | keybinding/action maps |
| bevy_enhanced_input | 0.26.0 | ^0.19 | keybinding alternative |
| bevy-persistent | 0.11.0 | ^0.19 | settings persistence |
| bevy_simple_prefs | 0.9.0 | ^0.19 | settings persistence (smaller) |
| bevy_pkv | 0.16.0 | ^0.19 | key-value settings store |
| bevy_fluent | 0.15.0 | ^0.19 | Fluent localisation |
| bevy_framepace | 0.22.0 | ^0.19 | frame cap for perf options |
| bevy_dev_tools | 0.19.1 | (in-engine) | `FpsOverlayPlugin` |
| bevy_asset_loader | 0.27.0 | ^0.19 | loading states |
| bevy_embedded_assets | 0.16.0 | ^0.19 | single-binary shipping |
| bevy_kira_audio | 0.26.0 | ^0.19 | audio (already a dep) |
| bevy_seedling | 0.8.0 | ^0.19 | audio alternative |
| bevy-steamworks | 0.17.0 | ^0.19 | Steamworks API (note hyphen) |
| steamworks | 0.13.1 | agnostic | raw Steamworks bindings |
| sentry | 0.49.1 | agnostic | crash telemetry |
| bevy_tweening | 0.16.0 | ^0.19 | UI/camera easing |
| bevy_easings | 0.19.0 | ^0.19 | easing alternative |
| iyes_perf_ui | 0.5.0 | **^0.16 — avoid** | stuck 3 versions behind |
| bevy_toast | 0.1.1 | **^0.6.1 — avoid** | dead; hand-roll toasts |

Bevy 0.19 in-engine features that matter for the shell (per the [Bevy 0.19 release notes](https://bevy.org/news/bevy-0-19/)): **Feathers** widget set is out of `experimental_` and "stable enough for broad use" (BSN-based sliders/checkboxes/buttons — candidate for settings rows); **`EditableText`** gives native text input (save-slot naming); **AccessKit** integration is built in ([Bevy is the first general-purpose engine with built-in accessibility](https://accesskit.dev/accesskit-integration-makes-bevy-the-first-general-purpose-game-engine-with-built-in-accessibility-support/)), with 0.19 adding the `AccessibleLabel` mixin component; native `States`/`SubStates` + state-scoped despawn have been in-engine since 0.14.

---

## Per-item breakdown

### 0. The keystone: app-state retrofit (prerequisite, not in the brief list)

The single structural cost everything else hangs off. Today every sim/game plugin registers unconditionally and spawns the world at `Startup`. A shell needs `AppState { MainMenu, Loading, InGame }` (+ `SubStates` for pause/ledger overlays), `run_if(in_state(InGame))` on the sim schedule (cheap — the sim already runs through one substep driver in `clock.rs`, so gating is nearly a single choke point), world spawn moved to `OnEnter(InGame)`, and teardown (state-scoped entities) on exit-to-menu. The capture/bench binaries (16 of them in `src/bin/`) construct their own apps and must keep working — the gate must default open for piecemeal apps, the same trick `hud.rs` already uses with `Option<Res>`.

- **Crates:** none — native `States` covers it.
- **Hand-roll:** all of it; mostly mechanical, risk is in the 115-test/capture surface.
- **Effort: 2–3 days** including re-verifying gates.

### 1. Main menu / new game / load screen

- **Crates:** none needed. Native `bevy_ui` screens (the toolbar's button/flyout pattern in `toolbar.rs` is directly reusable). Assets are procedural meshes, so `bevy_asset_loader` loading states are optional — a load screen here is really "deserialize the save + rebuild render state", which is fast; a simple spinner state suffices.
- **Hand-roll:** menu layout (Continue / New Plan / Load / Settings / Quit), new-game flow (initially trivial: one authored scenario — the First Plan; later a scenario picker), hooking Load into the existing `SaveLoadRequests`.
- **Effort: 1–2 days** on top of the state retrofit.

### 2. Settings (video / audio / keybindings)

- **Video:** all native — `Window` mode (windowed/borderless/fullscreen), resolution, `PresentMode` (vsync), `UiScale`. Hand-roll the rows; Feathers widgets can supply sliders/toggles if their look can be reskinned to the state-document theme, else hand-rolled buttons (pattern exists). **~1 day.**
- **Audio:** `bevy_kira_audio` is already a dep (currently unused in `src/`). Channels (master/SFX/ambience) + volume sliders. **~0.5–1 day** once any audio exists at all (audio content itself is a separate ticket).
- **Keybindings:** the expensive one. Input today is raw `ButtonInput<KeyCode>` scattered through `tools.rs`, `hud.rs`, `camera.rs`, `saveload.rs`, `toolbar.rs`. Real rebinding requires migrating to an action layer — `leafwing-input-manager 0.21` or `bevy_enhanced_input 0.26` (both on 0.19; leafwing is the incumbent, enhanced-input the newer contextual design). Migration ~2–3 days + rebind UI ~1–2 days. **Recommendation for 1.0: ship fixed keys** (the HUD legend already documents them) and defer rebinding; the toolbar means every action is mouse-reachable already, which blunts the accessibility argument for rebinding at 1.0.
- **Persistence:** `bevy-persistent 0.11` or `bevy_simple_prefs 0.9` — settings file in the platform config dir. **Hours.**
- **Effort: 2–3 days minimal (fixed keys); +3–5 days for full rebinding.**

### 3. Saves UX beyond F5/F9

- **Crates:** none apply — the postcard column format is custom (correctly so, per `research/bevy-ecosystem.md` §6).
- **Hand-roll:** save-slot model (directory of files + a small header readable *without* full deserialization: name, timestamp, tick, population, plan period, `SAVE_VERSION` — currently the version is inside the postcard blob; add a tiny fixed-layout preamble), load/save/delete browser screen, save-name input (`EditableText`), confirm-overwrite, autosave on a tick timer + rotating slots, and surfacing the version-gate error as UI instead of a string. Migration shims across `SAVE_VERSION` bumps are a policy decision — currently hard-gated, which is fine pre-1.0 but a 1.0 promise ("your saves survive patches") makes every future column change cost a shim.
- **Effort: 2–3 days.**

### 4. Pause + date + speed UI

Cheapest item — the model already exists (`SimSpeed` with 4 states, keys, HUD readout). Missing: a calendar date derived from `TickIndex` (plan periods already imply a calendar; pick ticks-per-day and render "Year 2, Month 3" Soviet-style), clickable pause/1×/2×/4× buttons (toolbar button pattern), and a pause dim/vignette (plan-ledger overlay pattern in `hud.rs` is exactly this).

- **Crates:** none. **Hand-roll:** trivial. **Effort: 0.5–1 day.**

### 5. Notifications / event log

- **Crates:** nothing viable — `bevy_toast` is Bevy-0.6-era; egui toast crates conflict with the native-UI decision. Hand-roll.
- **Hand-roll:** a `Notification` event/queue resource, a toast stack in the existing right-side HUD flex column (that column was built in G1.4 precisely so panels stack), timed fade-out, severity styling, and a scrollback log panel (native `bevy_ui` overflow/scrolling handles it). Sources already exist and just need wiring: treasury NO FUNDS refusals, `DeficitBoard`, `StallBoard`, plan-period results, construction completions. Click-to-jump pairs with camera item 6.
- **Effort: 1–2 days** for the framework + first ~6 event types; then an ongoing small tax per new event.

### 6. Camera polish

- **Crates:** keep the hand-rolled `CameraRig` — `bevy_panorbit_camera` would be a regression against a rig already fitted to the game (fog/bloom/tonemapping tuned, ground-focus model). The ecosystem doc already called RTS-camera polish "a thin custom layer" (§7 gaps).
- **Hand-roll:** edge-of-screen pan, movement smoothing/inertia (lerp the rig, or `bevy_tweening 0.16` for one-shot moves), zoom-toward-cursor, focus bounds clamped to the buildable world (`GROUND_HALF` is already imported), rotation snap, and jump-to-location (from notifications/ledger).
- **Effort: 1–2 days.**

### 7. Performance scaling options

- **Native levers:** `Msaa` level, shadow map resolution / `ShadowFilteringMethod`, `Bloom` toggle, `DistanceFog` range (doubles as draw distance), `VisibilityRange` LOD tier distances (relevant once citizen/vehicle counts grow), `PresentMode`. Frame cap via `bevy_framepace 0.22` (0.19 ✅). FPS readout via in-engine `bevy_dev_tools::FpsOverlayPlugin` — **avoid `iyes_perf_ui`** (stuck on 0.16).
- **Hand-roll:** a `QualityPreset { Low, Medium, High }` resource applying the levers + custom rows in settings. The presets are cheap; making Low *matter* is future perf work, not shell work.
- **Effort: 1–2 days.**

### 8. EN localisation baseline

- **Crates:** `bevy_fluent 0.15` is current on 0.19 (Fluent FTL assets, locale fallback). But for an **EN-only 1.0** full Fluent is over-engineering.
- **Recommended baseline:** centralise UI strings — today every label is an inline literal across ~1,400 lines of `hud.rs`/`toolbar.rs` — into one strings module (or one FTL file if you want the format future-proof from day one). That makes a later locale a data swap and, more immediately, makes tone/voice editing (the state-document register is a feature) one-file work. Caveats to record: ASCII progress-bar art and column-aligned monospace layouts embed EN string widths; Cyrillic font coverage becomes an asset question only when a RU locale is attempted.
- **Effort: 1–2 days** for centralisation; **3–4 days** if adopting `bevy_fluent` outright. Recommend the former.

### 9. Accessibility basics

- **In-engine:** AccessKit ships inside Bevy (`bevy_a11y`); native `bevy_ui` widgets expose accessibility nodes, and 0.19's `AccessibleLabel` component lets custom widgets (our hand-rolled buttons/panels) advertise labels cheaply.
- **Cheap wins to define as the 1.0 bar:** UI scale slider (item 2 gives it), a font-size bump option, contrast check on the rust-accent-on-dark palette (verify ≥4.5:1 for text), `AccessibleLabel` on every interactive widget, pause-anytime + zero time-critical input (already true — a planned-economy sim is turn-tolerant by nature), toolbar mouse path for every hotkey action (already true since G1.3).
- **Not the 1.0 bar:** full screen-reader traversal of the ledger/inspect panels, colorblind palettes per data channel.
- **Effort: 1–2 days.**

### 10. Linux + Windows builds, itch/Steam packaging

- **Builds:** add a tuned `[profile.release]` (`lto`, `codegen-units = 1`, `strip`) — currently absent from `Cargo.toml`. Linux: build against oldest-supported glibc (CI on an older distro image) — the standard Bevy pitfall; list runtime deps (ALSA/udev/Vulkan). Windows: `x86_64-pc-windows-gnu` cross-compiles Bevy cleanly from Linux, or MSVC in CI. `bevy_embedded_assets 0.16` if shipping a single binary (fonts are the main external asset).
- **itch.io:** `butler` push per channel; a GitHub Actions matrix that builds both targets and pushes to itch is the standard pattern. **1–2 days total including CI.**
- **Steam:** a plain binary runs fine on Steam with *zero* Steamworks integration — `bevy-steamworks 0.17` (0.19 ✅) only needed for achievements/overlay niceties, defer it. Real costs: $100 app fee, steampipe depot setup + Linux runtime (sniper) testing (**1–2 days**), and the store page (capsule art, screenshots, trailer — days of asset work; `/asset-gen` + the capture binaries help). **+2–4 days engineering, art extra.**
- **Effort: itch 1–2 days; Steam +2–4 days + store assets.**

### 11. Crash / telemetry posture

- **Options:** (a) nothing; (b) **local-only crash log** — `std::panic::set_hook` writing panic message + backtrace + version + OS to a file beside the saves, with a "please attach this file" line, plus routine autosave (item 3) bounding lost progress; (c) opt-in remote via `sentry 0.49` (pure Rust, panic integration works under Bevy) — requires a consent dialog for a defensible privacy posture, plus service setup.
- **Recommendation:** (b) for 1.0 — it's a solo native game; a crash file + autosave covers the actual player need with zero privacy surface. Log-to-file for non-crash diagnostics via a `tracing-appender` layer on Bevy's `LogPlugin`. Sentry can be added later without UX change beyond the consent toggle.
- **Effort: 0.5–1 day local; +1–2 days if opt-in Sentry.**

---

## Totals and the shape of "polished"

| Item | Minimal 1.0 | Full |
|---|---|---|
| 0. App-state retrofit | 2–3 d | 2–3 d |
| 1. Main menu / new / load | 1–2 d | 1–2 d |
| 2. Settings | 2–3 d (fixed keys) | 5–8 d (rebinding) |
| 3. Saves UX | 2–3 d | 2–3 d |
| 4. Pause/date/speed | 0.5–1 d | 0.5–1 d |
| 5. Notifications/log | 1–2 d | 1–2 d |
| 6. Camera polish | 1–2 d | 1–2 d |
| 7. Perf scaling | 1–2 d | 1–2 d |
| 8. EN localisation | 1–2 d (strings module) | 3–4 d (bevy_fluent) |
| 9. Accessibility | 1–2 d | 1–2 d |
| 10. Packaging | 1–2 d (itch) | 3–6 d (+Steam, ex-art) |
| 11. Crash posture | 0.5–1 d | 1.5–3 d |
| **Total** | **≈ 14–25 days** | **≈ 22–38 days** |

**Reading:** a credible minimal shell is ~3–5 working weeks, and roughly half of it is pure hand-rolled `bevy_ui` work where the repo already has strong patterns (toolbar buttons, overlay sheets, HUD flex stacking). The only genuinely structural item is the app-state retrofit (item 0) — do it first and alone; everything else is additive and independently shippable. The two biggest optional costs — key rebinding and Steam — are cleanly deferrable past 1.0. Crate-wise the shell needs at most four new small deps (`bevy-persistent` or `bevy_simple_prefs`, `bevy_framepace`, optionally `leafwing-input-manager`, optionally `bevy_tweening`), all current on 0.19; everything else is in-engine or already in `Cargo.toml`.

## Sources

- Repo (verified 2026-08-17): `src/lib.rs`, `src/game/{hud,toolbar,camera,saveload}.rs`, `src/sim/{clock,save}.rs`, `Cargo.toml`, `research/bevy-ecosystem.md`
- [Bevy 0.19 release notes](https://bevy.org/news/bevy-0-19/) — Feathers stabilisation, `EditableText`, `AccessibleLabel`
- [Bevy 0.18→0.19 migration guide](https://bevy.org/learn/migration-guides/0-18-to-0-19/)
- [AccessKit × Bevy announcement](https://accesskit.dev/accesskit-integration-makes-bevy-the-first-general-purpose-game-engine-with-built-in-accessibility-support/)
- index.crates.io sparse-index queries for every crate in the table above (versions + declared Bevy requirement), 2026-08-17
- [leafwing-input-manager](https://github.com/Leafwing-Studios/leafwing-input-manager) · [bevy_fluent](https://github.com/kgv/bevy_fluent)
