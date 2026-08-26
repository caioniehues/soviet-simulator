#!/usr/bin/env python3
"""Build the Wave 3 structured requirement extract from rewritten requirements only."""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
DEFAULT_REQUIREMENTS = ROOT / "docs/plan/iterations/requirements"
DEFAULT_SPECS = ROOT / "docs/reference/specifications"
REQ_PATTERN = re.compile(r"^## (REQ-[A-Z]+-\d{3}) — (.+)$", re.MULTILINE)
SPEC_PATTERN = re.compile(r"\bSPEC-[A-Z]+-\d{3}\b")
LEGACY_ITERATIONS = "/".join(("docs", "superpowers", "iterations"))
LEGACY_ROOT_SPEC = "spec" + "/"
FORBIDDEN = ("[SUBSTRATE:", LEGACY_ITERATIONS, "`" + LEGACY_ROOT_SPEC, " " + LEGACY_ROOT_SPEC)


def fail(message: str) -> None:
    raise ValueError(message)


def source_display(path: Path) -> str:
    """Keep extract identity stable when the requirement corpus is generated outside the repo."""
    return f"docs/plan/iterations/requirements/{path.name}"


def parse_requirements(directory: Path) -> list[dict[str, object]]:
    records: list[dict[str, object]] = []
    for path in sorted(directory.glob("*.md")):
        text = path.read_text()
        for token in FORBIDDEN:
            if token in text:
                fail(f"{path}: forbidden legacy citation {token!r}")
        matches = list(REQ_PATTERN.finditer(text))
        if not matches:
            if re.search(r"^\*\*Kind:\*\* requirements\s*$", text, re.MULTILINE):
                fail(f"{path}: requirement contract contains no requirement heading")
            continue
        for index, match in enumerate(matches):
            body = text[match.end() : matches[index + 1].start() if index + 1 < len(matches) else len(text)]
            status = re.search(r"^\*\*Status:\*\* (.+)$", body, re.MULTILINE)
            anchors = re.search(r"^\*\*Specification anchors:\*\* (.+)$", body, re.MULTILINE)
            criteria = re.search(r"^### Acceptance criteria\n\n((?:- .+\n?)+)", body, re.MULTILINE)
            if status is None or status.group(1).strip() != "proposed":
                fail(f"{path}: {match.group(1)} must be proposed")
            if anchors is None:
                fail(f"{path}: {match.group(1)} has no specification anchors")
            if criteria is None:
                fail(f"{path}: {match.group(1)} has no acceptance criteria")
            acceptance = [line[2:].strip() for line in criteria.group(1).splitlines() if line.startswith("- ")]
            if not acceptance or any("TBD" in item for item in acceptance):
                fail(f"{path}: {match.group(1)} has empty or TBD acceptance criteria")
            records.append(
                {
                    "id": match.group(1),
                    "title": match.group(2),
                    "status": "proposed",
                    "source": source_display(path),
                    "specification_anchors": sorted(set(SPEC_PATTERN.findall(anchors.group(1)))),
                    "acceptance_criteria": acceptance,
                }
            )
    ids = [record["id"] for record in records]
    if len(ids) != len(set(ids)):
        fail("duplicate requirement ID")
    return sorted(records, key=lambda record: str(record["id"]))


def specification_ids(directory: Path) -> set[str]:
    return {
        anchor
        for path in directory.glob("*.md")
        for anchor in SPEC_PATTERN.findall(path.read_text())
    }


def build(requirements: Path, specs: Path) -> dict[str, object]:
    records = parse_requirements(requirements)
    unresolved_sources = sorted({
        str(record["source"])
        for record in records
        if not (ROOT / str(record["source"])).is_file()
    })
    if unresolved_sources:
        fail(f"canonical requirement sources missing from tracked repository: {', '.join(unresolved_sources)}")
    known = specification_ids(specs)
    missing = sorted(
        anchor
        for record in records
        for anchor in record["specification_anchors"]
        if anchor not in known
    )
    if missing:
        fail(f"unknown specification anchors: {', '.join(missing)}")
    return {
        "schema": "wave3-requirement-extract/v1",
        "authority": "rewritten requirements plus current specification anchors",
        "requirements": records,
    }


def encoded(value: dict[str, object]) -> bytes:
    return (json.dumps(value, indent=2, sort_keys=True) + "\n").encode()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--requirements", type=Path, default=DEFAULT_REQUIREMENTS)
    parser.add_argument("--specifications", type=Path, default=DEFAULT_SPECS)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    args.requirements = args.requirements.resolve()
    args.specifications = args.specifications.resolve()
    args.output = args.output.resolve()
    try:
        output = encoded(build(args.requirements, args.specifications))
        if args.check:
            if not args.output.is_file() or args.output.read_bytes() != output:
                fail(f"{args.output}: differs from deterministic extract")
        else:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_bytes(output)
        print(f"validated {len(json.loads(output)['requirements'])} rewritten requirements")
        return 0
    except ValueError as error:
        print(f"extract validation failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
