#!/usr/bin/env python3
"""Block command shapes that violate checked-in Soviet Simulator rules."""

import json
import shlex
import sys


SIMULATION_SERIAL = (
    "Simulation tests must run serially: "
    "cargo test -p simulation -- --test-threads=1"
)
BROAD_STAGE = "Stage explicit paths only; git add -A, --all, and . are forbidden here."
BEADS_STAGE = "Stage only the four versioned .beads files named in CLAUDE.md."


def deny(reason: str) -> None:
    print(
        json.dumps(
            {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": reason,
                }
            }
        )
    )


def shell_segments(command: str) -> list[list[str]]:
    lexer = shlex.shlex(
        command.replace("\n", " ; "), posix=True, punctuation_chars=";&|"
    )
    lexer.whitespace_split = True
    lexer.commenters = ""
    segments: list[list[str]] = [[]]
    for token in lexer:
        if token and set(token) <= set(";&|"):
            if segments[-1]:
                segments.append([])
            continue
        segments[-1].append(token)
    return [segment for segment in segments if segment]


def has_simulation_package(tokens: list[str]) -> bool:
    return any(
        token == "--package=simulation"
        or token in {"-p", "--package"}
        and index + 1 < len(tokens)
        and tokens[index + 1] == "simulation"
        for index, token in enumerate(tokens)
    )


def has_serial_flag(tokens: list[str]) -> bool:
    return any(
        token == "--test-threads=1"
        or token == "--test-threads"
        and index + 1 < len(tokens)
        and tokens[index + 1] == "1"
        for index, token in enumerate(tokens)
    )


def decision_for(command: str) -> str | None:
    for tokens in shell_segments(command):
        for index, token in enumerate(tokens):
            if token == "cargo":
                cargo = tokens[index + 1 :]
                if "test" in cargo and has_simulation_package(cargo) and not has_serial_flag(cargo):
                    return SIMULATION_SERIAL

            if token == "git" and "add" in tokens[index + 1 :]:
                add_index = tokens.index("add", index + 1)
                paths = tokens[add_index + 1 :]
                if any(path in {"-A", "--all", "."} for path in paths):
                    return BROAD_STAGE
                if any(path.rstrip("/") in {".beads", "./.beads"} for path in paths):
                    return BEADS_STAGE
    return None


def main() -> int:
    payload = json.load(sys.stdin)
    tool_input = payload.get("tool_input") or {}
    command = tool_input.get("command") or tool_input.get("cmd") or ""
    reason = decision_for(command)
    if reason:
        deny(reason)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
