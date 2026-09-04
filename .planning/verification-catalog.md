# Repository verification catalog (2026-09-04)

Read-only inventory of every implementation-verification mechanism in this
repo: exact invocations, exit contracts, costs, owners, and the agent-hostile
edges. Source of truth for what the proposed `sov-verify` MCP server must
wrap (see `verification-mcp-design.md`). Commands verified against the tree;
costs measured where stated.

## 1. tools/*.py

### tools/run_validation_gate.py — validation-message gate
- **Command:** `python3 tools/run_validation_gate.py --allowlist <allowlist.txt> --artifact <out.txt> [--capture-record <record.json>] -- <command> [args...]`
- **Proves:** (a) the wrapped command's combined stdout+stderr contains no
  validation-marker line outside the allowlist (markers, casefolded:
  `sync-hazard`, `validation error`, `validation warning`, `wgpu error`);
  entries match by **exact whole-line equality** (sov-xa4 — bare signatures
  forbidden); (b) artifact file gets full combined output; (c) anti-vacuous
  proofs: with `--capture-record`, the record must be **rewritten during this
  run** (mtime ≥ floor(start)) and `device.validation_requested` true, else
  exit 2; without a record, at least one observed validation line is
  required, else exit 2; (d) adapter-scoped allowlists (`# adapter_match:
  {...}` headers, sov-y27): every scope field must equal the run record's
  `adapter` object or exit 1; scoped list with no `--capture-record` exits 2.
- **Exits:** `0` PASS (`PASS validation messages: N allowed, 0 new`); `1`
  new messages, scope mismatch, or child failure with new messages (new
  messages print first); `2` usage/infra; else the child's own nonzero exit.
- **Output:** child stdout passthrough + one PASS line; stderr FAIL lines +
  per-message `  - <line>`; artifact = full combined output.
- **Cost:** milliseconds of Python + child runtime (a `--validation`
  capture ≈ seconds on GPU). **CI:** none — never invoked by any workflow.
- **Self-tests:** `tools/test_run_validation_gate.py` (stdlib unittest;
  allow/block, stdout capture, exact-vs-substring, freshness,
  validation_requested, scope). `python3 -m unittest
  tools.test_run_validation_gate`.

### tools/check_gpu_timing.py — per-pass GPU regression gate
- **Command:** `python3 tools/check_gpu_timing.py <baseline.json> <capture.json>`
  (exactly 2 args else exit 2).
- **Proves:** same adapter class (every `adapter_match` field equal), same
  `scene`, `gpu_timing.status == enabled`, every `medians_us` pass numeric
  and within `tolerances[pass]` (legacy `max_regression_fraction` fallback),
  every `rank_order` pair still ordered. Current baseline
  `engine_demo/gpu_timing_baselines/radv-navi3x/baseline.json` (schema 2):
  9 passes, 15% per-pass tolerance (`max(3*spread, 0.15)`), rank pairs
  main>ssao>fog>depth_prepass; provenance 3 quiet runs 2026-09-04.
- **Exits:** `0` PASS (`PASS <adapter_class>/<scene>`); `1` FAIL lines;
  `2` usage/unreadable JSON. No JSON output. **Cost:** milliseconds + the
  two captures behind the JSONs (each ~seconds GPU + 90 warmup frames).
- **Self-tests:** `tools/test_check_gpu_timing.py` (inside/over tolerance,
  wrong adapter). **CI:** none.

### tools/bake_ground.py — NOT verification
Asset transform (bakes ground textures toward art-direction hex targets).
No gate consumes its output. Listed so nobody wraps it by mistake.

## 2. scripts/check_docs.py — docs checker
- **Command:** `python3 scripts/check_docs.py` (no deps, no flags).
- **Scope (6 checks):** broken relative links in active MD; every
  `docs/SUMMARY.md` target exists; orphans (wiki-section pages unreachable
  from SUMMARY) are ERRORS; duplicate H1 warnings only; metadata block
  (`Kind,Authority,Status,Owner,Last verified` in first 20 lines) on specs +
  wiki sections + root entrypoints; `Implementation claims: yes` requires
  non-empty `Verified-at:`.
- **Output:** `<N> active files checked; <E> error(s), <W> warning(s).`
  Exit 1 on any error. **Cost:** ~1–2 s. **CI:** docs job (path-filtered).

## 3. engine_demo/tests/
- **golden.rs** (`cargo test -p engine_demo --test golden`): respawns the
  real binary (`CARGO_BIN_EXE_engine_demo`, CWD = workspace root), runs
  `capture --scene baseline`, byte-compares fresh PNG vs committed
  `engine_demo/golden/baseline.png` (md5 `1820c766…`, 1280x720).
  Same-machine (RADV) only, by design. Cross-adapter path: lavapipe
  MSSIM ≥ 0.99 (note + `lavapipe.baseline.png`; compare script is
  working-tree-external — known gap).
- **capture_contract.rs:** 4 CPU-side guards, no GPU (unpad/swizzle,
  FrameworkOptions interactive/capture contracts, pass-name slot table,
  record JSON completeness + determinism).

## 4. cargo test layout
- **Workspace:** 13 members, resolver 2, default `native_app`, all
  `publish = false` (load-bearing for deny rules).
- **simulation** (`cargo test -p simulation`, parallel-safe): TestCtx ticks
  run round-trip determinism checks; `advance_ticks(n)` checks every 25 +
  final. Filters: `scenario_` (~31 tests), `sentinel` (promoted corpus,
  `--test-threads=1`). Standing rule: a filter matching zero tests is a
  failure — never ship a vacuous green.
  - Sentinels: hoarding substrate trio, cross-domain journey pair
    (REQ/SPEC/EVID-bound, mutation-proven), pillar guards
    (nothing-teleports / never-game-over / clearing-by-queue).
  - Determinism gates: is_equal divergence, system visibility, bincode
    order-insensitivity, fixture census, 10-humans/20k-ticks,
    two-trucks-simultaneous.
  - Ignored/long: `placement_stress` OOM sweep (`#[ignore]`, release-only,
    MUST run under `systemd-run … MemoryMax=2G`); 300k-tick convoy test
    (default suite, MINUTES-long, reads as a hang); `fixture_builder`
    regen (deliberate-only baseline rewrite).
- **Small suites:** prototypes (load + validation refusals), networking
  (hostile frame/world/ack), common (saveload bincode-confinement),
  geom, engine, egui-inspect-derive. Zero tests: native_app, headless,
  assets_gui, goryak, vectiler.

## 5. CI — what it gates and what it omits
- **dependency-policy.yml:** one job, `cargo-deny check` (pinned 0.20.2).
  Header forbids adding build/test/packaging/release steps.
- **docs.yml:** `check_docs.py` + mdbook 0.5.4/pagetoc/mermaid build.
- **oom-guard.yml:** placement_stress under a 2G ceiling (red-proofed).
- **NOT gated:** sim suite, engine_demo tests, captures,
  validation/gpu-timing gates, evidence `--check`, clippy/fmt.

## 6. deny.toml + mdbook
- `cargo-deny check` from root (canonical; absolute-path variants are not
  durable). Advisories/licenses/bans/sources; 13 SPDX allows + epaint
  exception; git sources allow-listed (egui/yakui, both rev-pinned).
  Binary usually absent on agent machines (cold install = minutes; version
  must be exactly 0.20.2).
- `mdbook build` (0.5.4 + pagetoc + mermaid); `book/` output never edited.

## 7. Evidence generator + guards
`docs/plan/iterations/evidence/build_evidence.py` — generate, `--check`
(deterministic-output guard), `--test-removed-claim-mutation`
(reverse-coverage self-test). Guards: no duplicate EVID, exact bindings
coverage, anchors in-contract, reverse REQ coverage, TARGET standards,
nonempty regression inventory from a serial `--list` (zero tests = fail),
5 sentinel promotions each executed to `test result: ok`. Committed under
`docs/generated/evidence/`. NOT in CI — stale inventory is a known drift
source.

## 8. Capture CLI contract
`engine_demo capture [--scene baseline] [--out dir] [--gpu-timings]
[--gpu-samples n] [--validation] [--list-scenes]`. Pins (not settable):
camera, 1280x720, 1/60 fixed delta, shader time 0, frozen input, drawable
set, 90 warmup frames. Per run: PNG + record JSON (adapter, resolution,
build, passes, drawcalls/tris, gpu timing + status, scene extras).

## 9. Agent-hostile edges (design the server around these)
- Golden: same-machine only; headless RADV needs `env -u DISPLAY
  -u WAYLAND_DISPLAY WGPU_BACKEND=vulkan`; GL fallback is existing behavior.
- Validation e2e: needs the layer installed + `--validation` + matching
  `--capture-record`; without it exit 2 reads as hostile.
- GPU timing: needs quiet GPU (1.16–1.35x slowdown runs had to be
  excluded); failure procedure is re-run-quiet-first.
- OOM guard: systemd user scope (container-hostile); never selected by
  plain `cargo test`; `-- --ignored` needed for discovery.
- 300k-tick test dominates suite time; evidence build executes every
  sentinel via subprocess (minutes).
- cargo-deny / mdbook binaries usually absent (minutes to install).
- Full `cargo test` output exhausts model windows — distill server-side.

## 10. Hand-verified gaps (no gate covers these)
Screenshots/visual review; live game/interactive runs (mutation policy
explicitly out-of-contract for UI-only paths); lockfile/prose inspection
and doc re-recording rules; external-trade conservation (11 market mutants
survive the suite — hand-audited); lavapipe compare script (external);
docs tone/structure/authority rules (human-only).
