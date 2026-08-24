#!/usr/bin/env python3
"""Regression tests for the Soviet Simulator command-policy hook."""

import importlib.util
import pathlib
import unittest


POLICY_PATH = pathlib.Path(__file__).with_name("pre_tool_use_policy.py")
SPEC = importlib.util.spec_from_file_location("pre_tool_use_policy", POLICY_PATH)
assert SPEC and SPEC.loader
POLICY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(POLICY)


class CommandPolicyTests(unittest.TestCase):
    def test_denied_commands(self) -> None:
        commands = (
            "cargo test -p simulation",
            "cargo test --package=simulation",
            "git add -A",
            "git -C /home/caio/soviet-simulator add .",
            'git add "."',
            "git add .beads/",
        )
        for command in commands:
            with self.subTest(command=command):
                self.assertIsNotNone(POLICY.decision_for(command))

    def test_allowed_commands(self) -> None:
        commands = (
            "cargo test -p simulation -- --test-threads=1",
            "cargo test --package=simulation -- --test-threads 1",
            "git add crates/simulation/src/lib.rs",
            "git add .beads/issues.jsonl .beads/interactions.jsonl .beads/metadata.json .beads/config.yaml",
        )
        for command in commands:
            with self.subTest(command=command):
                self.assertIsNone(POLICY.decision_for(command))


if __name__ == "__main__":
    unittest.main()
