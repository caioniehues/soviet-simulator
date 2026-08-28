import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GATE = ROOT / "tools" / "run_validation_gate.py"


class ValidationGateTests(unittest.TestCase):
    def test_writes_stderr_artifact_and_allows_known_signature(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            allowlist = tmp_path / "allowlist.txt"
            artifact = tmp_path / "validation-messages.txt"
            allowlist.write_text("SYNC-HAZARD-WRITE-AFTER-WRITE\n")
            result = subprocess.run(
                [
                    "python3",
                    str(GATE),
                    "--allowlist",
                    str(allowlist),
                    "--artifact",
                    str(artifact),
                    "--",
                    "python3",
                    "-c",
                    "import sys; print('SYNC-HAZARD-WRITE-AFTER-WRITE known', file=sys.stderr)",
                ],
                text=True,
                capture_output=True,
                check=False,
            )
            written = artifact.read_text() if artifact.exists() else ""

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(written, "SYNC-HAZARD-WRITE-AFTER-WRITE known\n")
        self.assertIn("PASS validation messages: 1 allowed, 0 new", result.stdout)

    def test_rejects_new_validation_signature_and_keeps_artifact(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            allowlist = tmp_path / "allowlist.txt"
            artifact = tmp_path / "validation-messages.txt"
            allowlist.write_text("SYNC-HAZARD-WRITE-AFTER-WRITE\n")
            result = subprocess.run(
                [
                    "python3",
                    str(GATE),
                    "--allowlist",
                    str(allowlist),
                    "--artifact",
                    str(artifact),
                    "--",
                    "python3",
                    "-c",
                    "import sys; print('SYNC-HAZARD-READ-AFTER-WRITE new', file=sys.stderr)",
                ],
                text=True,
                capture_output=True,
                check=False,
            )
            written = artifact.read_text()

        self.assertEqual(result.returncode, 1)
        self.assertIn("FAIL validation messages: 0 allowed, 1 new", result.stderr)
        self.assertEqual(written, "SYNC-HAZARD-READ-AFTER-WRITE new\n")

    def test_captures_validation_signature_written_to_stdout(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            allowlist = tmp_path / "allowlist.txt"
            artifact = tmp_path / "validation-messages.txt"
            allowlist.write_text("")
            result = subprocess.run(
                [
                    "python3",
                    str(GATE),
                    "--allowlist",
                    str(allowlist),
                    "--artifact",
                    str(artifact),
                    "--",
                    "python3",
                    "-c",
                    "print('Validation Warning: NEW-VALIDATION-WARNING')",
                ],
                text=True,
                capture_output=True,
                check=False,
            )
            written = artifact.read_text()

        self.assertEqual(result.returncode, 1)
        self.assertIn("FAIL validation messages: 0 allowed, 1 new", result.stderr)
        self.assertEqual(
            written, "Validation Warning: NEW-VALIDATION-WARNING\n"
        )


if __name__ == "__main__":
    unittest.main()
