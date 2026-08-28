import json
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

    def test_refuses_to_pass_when_the_run_produced_no_validation_messages(self):
        """B2: forgetting --validation, or running without the layer, must not read as PASS."""
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
                    "print('capture ok')",
                ],
                text=True,
                capture_output=True,
                check=False,
            )

        self.assertEqual(result.returncode, 2)
        self.assertIn("FAIL validation cannot be confirmed", result.stderr)
        self.assertNotIn("PASS", result.stdout)

    def test_refuses_a_capture_record_that_did_not_request_validation(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            allowlist = tmp_path / "allowlist.txt"
            artifact = tmp_path / "validation-messages.txt"
            record = tmp_path / "capture.json"
            allowlist.write_text("")
            record.write_text(json.dumps({"device": {"validation_requested": False}}))
            result = subprocess.run(
                [
                    "python3",
                    str(GATE),
                    "--allowlist",
                    str(allowlist),
                    "--artifact",
                    str(artifact),
                    "--capture-record",
                    str(record),
                    "--",
                    "python3",
                    "-c",
                    "print('capture ok')",
                ],
                text=True,
                capture_output=True,
                check=False,
            )

        self.assertEqual(result.returncode, 2)
        self.assertIn("FAIL validation was not requested", result.stderr)

    def test_a_clean_run_passes_when_the_capture_record_proves_validation_ran(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            allowlist = tmp_path / "allowlist.txt"
            artifact = tmp_path / "validation-messages.txt"
            record = tmp_path / "capture.json"
            allowlist.write_text("")
            record.write_text(json.dumps({"device": {"validation_requested": True}}))
            result = subprocess.run(
                [
                    "python3",
                    str(GATE),
                    "--allowlist",
                    str(allowlist),
                    "--artifact",
                    str(artifact),
                    "--capture-record",
                    str(record),
                    "--",
                    "python3",
                    "-c",
                    "print('capture ok')",
                ],
                text=True,
                capture_output=True,
                check=False,
            )

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("PASS validation messages: 0 allowed, 0 new", result.stdout)

    def test_lists_new_messages_even_when_the_command_also_exits_nonzero(self):
        """F12: a run that both fails and emits a new hazard must still name the hazard."""
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
                    "import sys; print('SYNC-HAZARD-READ-AFTER-WRITE new'); sys.exit(3)",
                ],
                text=True,
                capture_output=True,
                check=False,
            )

        self.assertEqual(result.returncode, 3)
        self.assertIn("- SYNC-HAZARD-READ-AFTER-WRITE new", result.stderr)
        self.assertIn("FAIL validation command exited 3", result.stderr)


if __name__ == "__main__":
    unittest.main()
