#!/usr/bin/env python3
"""Compare one engine_demo GPU-timing capture with an adapter-class baseline."""

import json
import sys
from pathlib import Path


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise ValueError(f"cannot read {path}: {error}") from error


def compare(baseline: dict, capture: dict) -> list[str]:
    failures = []
    adapter_class = baseline.get("adapter_class", "unknown-adapter")
    expected_adapter = baseline.get("adapter_match", {})
    actual_adapter = capture.get("adapter", {})
    for field, expected in expected_adapter.items():
        actual = actual_adapter.get(field)
        if actual != expected:
            failures.append(
                f"adapter class {adapter_class} requires {field}={expected!r}, got {actual!r}"
            )

    expected_scene = baseline.get("scene")
    if capture.get("scene") != expected_scene:
        failures.append(
            f"baseline scene is {expected_scene!r}, capture scene is {capture.get('scene')!r}"
        )

    timing = capture.get("gpu_timing", {})
    if timing.get("status") != "enabled":
        failures.append(f"GPU timing is not enabled: {timing.get('status')!r}")

    actual_passes = {
        entry.get("pass"): entry
        for entry in timing.get("passes", [])
        if isinstance(entry, dict) and isinstance(entry.get("pass"), str)
    }
    per_pass = baseline.get("tolerances", {})
    if not isinstance(per_pass, dict):
        failures.append("tolerances must be a per-pass object")
        per_pass = {}
    legacy = baseline.get("max_regression_fraction")
    legacy_ok = isinstance(legacy, (int, float)) and legacy >= 0
    if not per_pass and not legacy_ok:
        failures.append("max_regression_fraction must be a non-negative number")
        legacy = 0

    def tolerance_for(pass_name: str) -> float:
        # Each pass answers for its own spread first (sov-70h); the global is only
        # the schema-1 fallback for baselines that predate per-pass tolerances.
        if isinstance(per_pass.get(pass_name), (int, float)):
            return float(per_pass[pass_name])
        return float(legacy) if legacy_ok else 0.0

    for pass_name, expected_median in baseline.get("medians_us", {}).items():
        entry = actual_passes.get(pass_name)
        if entry is None:
            failures.append(f"missing timed pass {pass_name!r}")
            continue
        actual_median = entry.get("median_us")
        if not isinstance(actual_median, (int, float)):
            failures.append(f"pass {pass_name!r} has no numeric median_us")
            continue
        if pass_name not in per_pass and not legacy_ok:
            failures.append(f"pass {pass_name!r} has no tolerance in the baseline")
            continue
        tolerance = tolerance_for(pass_name)
        limit = expected_median * (1 + tolerance)
        if actual_median > limit:
            failures.append(
                f"{pass_name} median {actual_median:.3f}us exceeds "
                f"{limit:.3f}us ({tolerance:.0%} over {expected_median:.3f}us baseline)"
            )

    for slower, faster in baseline.get("rank_order", []):
        slower_entry = actual_passes.get(slower)
        faster_entry = actual_passes.get(faster)
        if slower_entry is None or faster_entry is None:
            continue
        slower_median = slower_entry.get("median_us")
        faster_median = faster_entry.get("median_us")
        if isinstance(slower_median, (int, float)) and isinstance(
            faster_median, (int, float)
        ) and slower_median <= faster_median:
            failures.append(
                f"rank order changed: {slower}={slower_median:.3f}us must exceed "
                f"{faster}={faster_median:.3f}us"
            )

    return failures


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(
            "usage: check_gpu_timing.py <baseline.json> <capture.json>",
            file=sys.stderr,
        )
        return 2

    baseline_path = Path(argv[1])
    capture_path = Path(argv[2])
    try:
        baseline = load_json(baseline_path)
        capture = load_json(capture_path)
    except ValueError as error:
        print(f"FAIL {error}", file=sys.stderr)
        return 2

    failures = compare(baseline, capture)
    adapter_class = baseline.get("adapter_class", "unknown-adapter")
    scene = baseline.get("scene", "unknown-scene")
    if failures:
        print(f"FAIL {adapter_class}/{scene}", file=sys.stderr)
        for failure in failures:
            print(f"  - {failure}", file=sys.stderr)
        return 1

    print(f"PASS {adapter_class}/{scene}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
