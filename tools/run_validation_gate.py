#!/usr/bin/env python3
"""Run a validation command, persist its output, and reject new message signatures."""

import argparse
import json
import math
import subprocess
import sys
import time
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


def require_record_written_by_this_run(path: Path, not_before: float) -> None:
    """Reject a capture record that predates the run the gate just wrapped.

    Without this the record is a free-floating path the operator supplies and the gate
    believes. Moving `--out` to a fresh directory while leaving `--capture-record` pointed at
    an older validated record -- an ordinary slip, not an attack -- let a run with
    validation_requested=false and zero validation markers report PASS.

    Freshness is checked rather than deriving the path from the child's output directory,
    because this gate wraps an arbitrary command: parsing `--out` and `--scene` back out of
    engine_demo's argv would couple a general-purpose gate to one binary's CLI and would
    silently stop binding anything the moment those flags change or a second consumer appears.
    An mtime is mechanism-independent.
    """
    try:
        mtime = path.stat().st_mtime
    except OSError as error:
        raise ValueError(f"cannot stat capture record {path}: {error}") from error
    if mtime < not_before:
        raise ValueError(
            f"capture record {path} was last written at "
            f"{time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(mtime))}, before this run "
            f"started at {time.strftime('%Y-%m-%d %H:%M:%S', time.localtime(not_before))}; "
            f"it describes a different run and proves nothing about this one"
        )


def validation_was_requested(path: Path) -> bool:
    """Read `device.validation_requested` out of a capture record.

    Raises `ValueError` when the record is missing, unparseable, or does not carry the flag --
    the gate must fail loudly rather than assume validation ran.
    """
    try:
        record = json.loads(path.read_text())
    except OSError as error:
        raise ValueError(f"cannot read capture record {path}: {error}") from error
    except json.JSONDecodeError as error:
        raise ValueError(f"capture record {path} is not valid JSON: {error}") from error
    device = record.get("device")
    if not isinstance(device, dict) or "validation_requested" not in device:
        raise ValueError(f"capture record {path} has no device.validation_requested field")
    return bool(device["validation_requested"])


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="capture combined output and fail on validation messages outside an allow-list"
    )
    parser.add_argument("--allowlist", required=True, type=Path)
    parser.add_argument("--artifact", required=True, type=Path)
    parser.add_argument(
        "--capture-record",
        type=Path,
        help="capture record JSON; its device.validation_requested proves validation ran",
    )
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)
    if args.command[:1] == ["--"]:
        args.command = args.command[1:]
    if not args.command:
        parser.error("a command is required after --")
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    # Floored to the second so a filesystem with coarse timestamp granularity cannot make a
    # record written *during* the run look older than the run.
    started_at = math.floor(time.time())
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

    # F12: list the new messages BEFORE returning on a nonzero child exit, so a run that both
    # fails and emits a new hazard still reports the hazard.
    if new_messages:
        print(
            f"FAIL validation messages: {allowed_count} allowed, {len(new_messages)} new; "
            f"artifact: {args.artifact}",
            file=sys.stderr,
        )
        for message in new_messages:
            print(f"  - {message}", file=sys.stderr)

    if result.returncode != 0:
        print(
            f"FAIL validation command exited {result.returncode}; artifact: {args.artifact}",
            file=sys.stderr,
        )
        return result.returncode
    if new_messages:
        return 1

    # B2: a gate that passes on zero input proves nothing. Forgetting `--validation`, or running
    # where VK_LAYER_KHRONOS_validation is not installed, produces a silent, empty, "clean" run.
    # Confirm validation actually ran before reporting success. Either confirmation suffices:
    # the capture record's own flag, or at least one observed validation message.
    if args.capture_record is not None:
        try:
            require_record_written_by_this_run(args.capture_record, started_at)
            requested = validation_was_requested(args.capture_record)
        except ValueError as error:
            print(f"FAIL validation gate: {error}", file=sys.stderr)
            return 2
        if not requested:
            print(
                f"FAIL validation was not requested: {args.capture_record} reports "
                f"device.validation_requested=false; the run proves nothing",
                file=sys.stderr,
            )
            return 2
    elif not observed:
        print(
            "FAIL validation cannot be confirmed: the command produced 0 validation messages "
            "and no --capture-record was given. Either the validation layer did not run, or "
            "pass --capture-record so the run can prove it did. "
            f"Artifact: {args.artifact}",
            file=sys.stderr,
        )
        return 2

    print(
        f"PASS validation messages: {allowed_count} allowed, 0 new; artifact: {args.artifact}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
