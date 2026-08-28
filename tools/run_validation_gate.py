#!/usr/bin/env python3
"""Run a validation command, persist its output, and reject new message signatures."""

import argparse
import subprocess
import sys
from pathlib import Path


VALIDATION_MARKERS = (
    "sync-hazard",
    "validation error",
    "validation warning",
    "wgpu error",
)


def read_allowlist(path: Path) -> list[str]:
    try:
        lines = path.read_text().splitlines()
    except OSError as error:
        raise ValueError(f"cannot read allow-list {path}: {error}") from error
    return [line.strip() for line in lines if line.strip() and not line.lstrip().startswith("#")]


def validation_lines(stderr: str) -> list[str]:
    return [
        line
        for line in stderr.splitlines()
        if any(marker in line.casefold() for marker in VALIDATION_MARKERS)
    ]


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="capture combined output and fail on validation messages outside an allow-list"
    )
    parser.add_argument("--allowlist", required=True, type=Path)
    parser.add_argument("--artifact", required=True, type=Path)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)
    if args.command[:1] == ["--"]:
        args.command = args.command[1:]
    if not args.command:
        parser.error("a command is required after --")
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    try:
        allowlist = read_allowlist(args.allowlist)
        result = subprocess.run(
            args.command,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            check=False,
        )
        args.artifact.parent.mkdir(parents=True, exist_ok=True)
        args.artifact.write_text(result.stdout)
    except (OSError, ValueError) as error:
        print(f"FAIL validation gate: {error}", file=sys.stderr)
        return 2

    if result.stdout:
        sys.stdout.write(result.stdout)

    observed = validation_lines(result.stdout)
    new_messages = [
        line for line in observed if not any(allowed in line for allowed in allowlist)
    ]
    allowed_count = len(observed) - len(new_messages)

    if result.returncode != 0:
        print(
            f"FAIL validation command exited {result.returncode}; artifact: {args.artifact}",
            file=sys.stderr,
        )
        return result.returncode
    if new_messages:
        print(
            f"FAIL validation messages: {allowed_count} allowed, {len(new_messages)} new; "
            f"artifact: {args.artifact}",
            file=sys.stderr,
        )
        for message in new_messages:
            print(f"  - {message}", file=sys.stderr)
        return 1

    print(
        f"PASS validation messages: {allowed_count} allowed, 0 new; artifact: {args.artifact}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
