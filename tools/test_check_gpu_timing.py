import json
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GATE = ROOT / "tools" / "check_gpu_timing.py"


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
        self.capture = {
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
                    {"pass": "main", "samples": 30, "median_us": 98.0},
                    {"pass": "ssao", "samples": 30, "median_us": 33.8},
                    {"pass": "fog", "samples": 30, "median_us": 28.4},
                ],
            },
        }

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
        self.capture["gpu_timing"]["passes"][0]["median_us"] = 120.0

        result = self.run_gate()

        self.assertEqual(result.returncode, 1)
        self.assertIn("main median 120.000us exceeds 119.600us", result.stderr)

    def test_rejects_capture_from_another_adapter_class(self):
        self.capture["adapter"]["device_id"] = 999

        result = self.run_gate()

        self.assertEqual(result.returncode, 1)
        self.assertIn("requires device_id=29822, got 999", result.stderr)


if __name__ == "__main__":
    unittest.main()
