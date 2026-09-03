# Renderer gates

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03

Both gates are **manual**: a human (or a future agent, on a machine with the
matching GPU) runs them before landing a renderer change. The repository has no
test CI, so nothing runs them automatically.

## What this is

Two copy-paste gates guard the fixed `baseline` capture
(`engine_demo/src/capture.rs`): the GPU-timing gate rejects performance
regressions per render pass, and the validation gate rejects wgpu/Vulkan
validation messages outside the allow-list. The same-machine exact-hash golden
test (`engine_demo/tests/golden.rs`) is the third, related guard: it re-runs
the capture and requires the fresh PNG to be byte-identical to
`engine_demo/golden/baseline.png`.

## 1.0 requirement

Renderer changes must not silently regress frame cost or introduce validation
errors. Each gate fails loudly (non-zero exit) rather than passing vacuously:
the timing gate requires timing data to be present, the validation gate
requires proof validation actually ran.

## Target design

Both gates stay manual scripts with committed baselines/allow-lists per
adapter class, runnable with one copy-paste block each. No CI workflow owns
them (there is no test CI in this repo).

## Current substrate

### Gate 1 — GPU timing (manual)

Compares one timing-enabled capture against the committed baseline for the
adapter class. Tool: `tools/check_gpu_timing.py`
(`usage: check_gpu_timing.py <baseline.json> <capture.json>`).
Baseline: `engine_demo/gpu_timing_baselines/radv-navi3x/baseline.json`.

```sh
cargo run -p engine_demo -- capture --scene baseline --out target/renderer-evidence --gpu-timings
python3 tools/check_gpu_timing.py \
  engine_demo/gpu_timing_baselines/radv-navi3x/baseline.json \
  target/renderer-evidence/baseline.json
```

Pass prints `PASS <adapter-class>/<scene>`; any regression prints
`FAIL …` with per-pass reasons. The capture must be timing-enabled
(`--gpu-timings`); a record without timing data fails the gate.

Run by: whoever changes renderer code, on the matching adapter
(radv-navi3x today), before landing. Not CI-run: no workflow invokes it.

### Gate 2 — validation messages (manual)

Wraps a validation-enabled capture, stores combined output in an artifact,
and fails on any validation message outside the allow-list. Tool:
`tools/run_validation_gate.py` (`--allowlist`, `--artifact`,
`--capture-record`, then the command after `--`).
Allow-list: `engine_demo/validation_allowlists/radv-navi3x.txt`.

```sh
python3 tools/run_validation_gate.py \
  --allowlist engine_demo/validation_allowlists/radv-navi3x.txt \
  --artifact target/renderer-evidence/validation-messages.txt \
  --capture-record target/renderer-evidence/baseline.json \
  -- cargo run -p engine_demo -- capture --scene baseline --out target/renderer-evidence --validation
```

Pass is silent (exit 0); failures print `FAIL …` to stderr and keep the
artifact. `--capture-record` proves validation ran
(`device.validation_requested` in the fresh record); a run with zero
validation messages and no record is a failure, not a pass.

Run by: whoever changes renderer code, on the matching adapter, before
landing. Not CI-run: no workflow invokes it.

### Related: golden test (manual, `cargo test`)

```sh
cargo test -p engine_demo --test golden
```

Re-runs the `baseline` capture through the real binary and requires the
fresh PNG to be byte-identical to `engine_demo/golden/baseline.png`
(same-machine scope only). To regenerate after an intentional visual change:
`cargo run -p engine_demo -- capture --scene baseline --out <dir>`, then
copy `<dir>/baseline.png` over the committed golden.

## Research basis

Empty: both gates are implemented tools with committed baselines, not
research proposals.

## Future direction

If test CI is ever added, wire both gates plus the golden test into it with
per-adapter-class runners. Until then they stay manual.

## Open questions

- Which adapter classes beyond radv-navi3x need committed baselines and
  allow-lists?
- What per-pass regression tolerance should the timing baseline enforce?

## Related

- `engine_demo/src/capture.rs` — the fixed capture contract
- `engine_demo/tests/golden.rs` — same-machine exact-hash golden gate
- `engine_demo/tests/capture_contract.rs` — CPU-side capture contract guards
