#!/usr/bin/env python3
"""Generate the Wave 3 roadmap from re-derived requirements and evidence metadata."""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
REQ_PATTERN = re.compile(r"^## (REQ-[A-Z]+-\d{3}) —", re.MULTILINE)


def fail(message: str) -> None:
    raise ValueError(message)


def read_requirement_ids(directory: Path) -> set[str]:
    values = {
        requirement_id
        for path in directory.glob("*.md")
        for requirement_id in REQ_PATTERN.findall(path.read_text())
    }
    if not values:
        fail(f"{directory}: no re-derived requirement IDs")
    return values


def build(requirements_dir: Path, extract_path: Path, evidence_path: Path) -> str:
    source_ids = read_requirement_ids(requirements_dir)
    extract = json.loads(extract_path.read_text())
    evidence = json.loads(evidence_path.read_text())
    requirements = extract.get("requirements", [])
    if extract.get("schema") != "wave3-requirement-extract/v1":
        fail("unsupported requirement extract")
    records = {item["id"]: item for item in requirements}
    if set(records) != source_ids:
        fail("extract requirement IDs differ from re-derived requirement files")
    if evidence.get("schema") != "wave3-target-evidence/v1":
        fail("unsupported evidence metadata")
    targets = evidence.get("target_scenarios", [])
    by_requirement = {requirement_id: [] for requirement_id in source_ids}
    for target in targets:
        if target.get("status") not in {"UNIMPLEMENTED", "IMPLEMENTED"}:
            fail(f"{target.get('id')}: invalid evidence status")
        for requirement_id in target.get("requirement_ids", []):
            if requirement_id not in by_requirement:
                fail(f"{target.get('id')}: unknown requirement binding")
            by_requirement[requirement_id].append(target)
    uncovered = [requirement_id for requirement_id, linked in by_requirement.items() if not linked]
    if uncovered:
        fail(f"requirements without evidence: {', '.join(sorted(uncovered))}")
    planned = sum(1 for target in targets if target["status"] == "UNIMPLEMENTED")
    implemented = sum(1 for target in targets if target["status"] == "IMPLEMENTED")
    lines = [
        "# Wave 3 controlled-documentation roadmap",
        "",
        "**Kind:** generated roadmap",
        "**Authority:** reporting only; requirements and specifications remain authoritative",
        "**Status:** draft — no target implementation is claimed",
        "**Owner:** project lead",
        "**Last verified:** 2026-08-24",
        "**Generator:** `python3 docs/plan/iterations/build_roadmap.py --requirements-dir docs/plan/iterations/requirements --extract docs/plan/iterations/extract/requirements.json --evidence docs/generated/evidence/target-scenarios.json --output docs/generated/roadmap.md`",
        "",
        "This roadmap reports the current re-derived Wave 3 contract. It does not import legacy completion, scenario IDs, or status claims. Current substrate regressions are intentionally reported outside the target-evidence totals.",
        "",
        "| Re-derived requirements | Planned target scenarios | Implemented target scenarios | Current status |",
        "| ---: | ---: | ---: | --- |",
        f"| {len(records)} | {planned} | {implemented} | draft / target evidence unimplemented |",
        "",
        "## Requirement schedule",
        "",
        "| Requirement | Contract | Planned EVID scenarios | Implemented | Status |",
        "| --- | --- | ---: | ---: | --- |",
    ]
    for requirement_id in sorted(records):
        record = records[requirement_id]
        linked = by_requirement[requirement_id]
        linked_implemented = sum(1 for target in linked if target["status"] == "IMPLEMENTED")
        status = "UNIMPLEMENTED" if linked_implemented != len(linked) else "IMPLEMENTED"
        lines.append(f"| `{requirement_id}` | {record['title']} | {len(linked)} | {linked_implemented} | {status} |")
    lines.extend(
        [
            "",
            "## Evidence boundary",
            "",
            "Each planned row binds a rewritten `REQ-*` identifier, one or more stable `SPEC-*` anchors, and one current `EVID-*` anchor. A scenario can become implemented only when its exact guard exists, executes at least one test, and has mutation evidence. The separately generated current-regression inventory is not target proof.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--requirements-dir", type=Path, required=True)
    parser.add_argument("--extract", type=Path, required=True)
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    args.requirements_dir = args.requirements_dir.resolve()
    args.extract = args.extract.resolve()
    args.evidence = args.evidence.resolve()
    args.output = args.output.resolve()
    try:
        expected = build(args.requirements_dir, args.extract, args.evidence).encode()
        if args.check:
            if not args.output.is_file() or args.output.read_bytes() != expected:
                fail(f"{args.output}: differs from deterministic roadmap output")
        else:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_bytes(expected)
        print(f"validated roadmap from {len(read_requirement_ids(args.requirements_dir))} requirements")
        return 0
    except ValueError as error:
        print(f"roadmap generation failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
