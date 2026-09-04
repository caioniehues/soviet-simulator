import copy
import json
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GATE = ROOT / "tools" / "check_gpu_timing.py"


def timed_capture(medians):
    return {
        "adapter": {
            "backend": "Vulkan",
            "vendor_id": 4098,
            "device_id": 29822,
            "driver": "radv",
        },
        "scene": "baseline",
        "gpu_timing": {
            "status": "enabled",
            "passes": [
                {"pass": name, "samples": 30, "median_us": median}
                for name, median in medians.items()
            ],
        },
    }


class GpuTimingGateTests(unittest.TestCase):
    def setUp(self):
        self.baseline = {
            "schema_version": 1,
            "adapter_class": "radv-navi3x",
            "adapter_match": {
                "backend": "Vulkan",
                "vendor_id": 4098,
                "device_id": 29822,
                "driver": "radv",
            },
            "scene": "baseline",
            "max_regression_fraction": 0.30,
            "medians_us": {"main": 92.0, "ssao": 34.0, "fog": 28.0},
            "rank_order": [["main", "ssao"], ["ssao", "fog"]],
        }
        self.capture = timed_capture({"main": 98.0, "ssao": 33.8, "fog": 28.4})

    def run_gate(self, baseline=None, capture=None):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            baseline_path = tmp_path / "baseline.json"
            capture_path = tmp_path / "capture.json"
            baseline_path.write_text(json.dumps(baseline or self.baseline))
            capture_path.write_text(json.dumps(capture or self.capture))
            return subprocess.run(
                ["python3", str(GATE), str(baseline_path), str(capture_path)],
                text=True,
                capture_output=True,
                check=False,
            )

    def test_accepts_matching_adapter_with_medians_inside_tolerance(self):
        result = self.run_gate()

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("PASS radv-navi3x/baseline", result.stdout)

    def test_rejects_median_above_regression_tolerance(self):
        capture = timed_capture({"main": 120.0, "ssao": 33.8, "fog": 28.4})

        result = self.run_gate(capture=capture)

        self.assertEqual(result.returncode, 1)
        self.assertIn("main median 120.000us exceeds 119.600us", result.stderr)

    def test_rejects_capture_from_another_adapter_class(self):
        capture = copy.deepcopy(self.capture)
        capture["adapter"]["device_id"] = 999

        result = self.run_gate(capture=capture)

        self.assertEqual(result.returncode, 1)
        self.assertIn("requires device_id=29822, got 999", result.stderr)


class PerPassToleranceTests(unittest.TestCase):
    """sov-70h: each pass answers for its own measured spread."""

    def setUp(self):
        self.baseline = {
            "schema_version": 2,
            "adapter_class": "radv-navi3x",
            "adapter_match": {
                "backend": "Vulkan",
                "vendor_id": 4098,
                "device_id": 29822,
                "driver": "radv",
            },
            "scene": "baseline",
            "medians_us": {"main": 69.16, "ssao": 25.08, "fog": 21.08},
            "tolerances": {"main": 0.15, "ssao": 0.15, "fog": 0.15},
            "rank_order": [["main", "ssao"], ["ssao", "fog"]],
        }

    def run_gate(self, baseline=None, capture=None):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            baseline_path = tmp_path / "baseline.json"
            capture_path = tmp_path / "capture.json"
            baseline_path.write_text(json.dumps(baseline or self.baseline))
            capture_path.write_text(json.dumps(capture))
            return subprocess.run(
                ["python3", str(GATE), str(baseline_path), str(capture_path)],
                text=True,
                capture_output=True,
                check=False,
            )

    def test_ssao_29pct_regression_fails(self):
        capture = timed_capture({"main": 69.2, "ssao": 25.08 * 1.29, "fog": 21.1})

        result = self.run_gate(capture=capture)

        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("ssao median 32.353us exceeds 28.842us", result.stderr)

    def test_main_noise_inside_own_tolerance_passes(self):
        capture = timed_capture({"main": 69.16 * 1.08, "ssao": 25.1, "fog": 21.1})

        result = self.run_gate(capture=capture)

        self.assertEqual(result.returncode, 0, result.stderr)

    def test_tight_pass_uses_own_limit_not_main_limit(self):
        baseline = copy.deepcopy(self.baseline)
        baseline["tolerances"] = {"main": 0.30, "ssao": 0.05, "fog": 0.05}
        capture = timed_capture({"main": 69.2, "ssao": 25.08 * 1.10, "fog": 21.1})

        result = self.run_gate(baseline=baseline, capture=capture)

        self.assertEqual(result.returncode, 1)
        self.assertIn("(5% over 25.080us baseline)", result.stderr)

    def test_missing_tolerance_for_gated_pass_is_an_error(self):
        baseline = copy.deepcopy(self.baseline)
        del baseline["tolerances"]["ssao"]
        capture = timed_capture({"main": 69.2, "ssao": 25.1, "fog": 21.1})

        result = self.run_gate(baseline=baseline, capture=capture)

        self.assertEqual(result.returncode, 1)
        self.assertIn("has no tolerance in the baseline", result.stderr)


if __name__ == "__main__":
    unittest.main()
