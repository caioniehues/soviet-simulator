#!/usr/bin/env python3
"""Build and validate Wave 3 planned target evidence and current regressions."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
DEFAULT_EXTRACT = ROOT / "docs/plan/iterations/extract/requirements.json"
DEFAULT_SPECS = ROOT / "docs/reference/specifications"
DEFAULT_BINDINGS = ROOT / "docs/plan/iterations/evidence/evid-spec-bindings.json"
EVID_ROW = re.compile(r"^\| `?(EVID-[A-Z]+-\d{3})`? \| `([^`]+)`", re.MULTILINE)
SPEC_PATTERN = re.compile(r"\bSPEC-[A-Z]+-\d{3}\b")
TEST_LINE = re.compile(r"^([^\s].+): test$")
PROMOTED = {
    "SENTINEL-SUBSTRATE-DISPATCH-GATE": {
        "test": "tests::scenarios::hoarding::scenario_0082_dispatch_gates_stock_not_match",
        "description": "Current dispatch implementation delays stock transfer until its current delivery gate.",
    },
    "SENTINEL-SUBSTRATE-NO-TRUCK": {
        "test": "tests::scenarios::hoarding::scenario_0083_zero_trucks_blocks_delivery",
        "description": "Current dispatch implementation retains a blocked delivery when no SmallTruck is available.",
    },
    "SENTINEL-SUBSTRATE-HOARDING": {
        "test": "tests::scenarios::hoarding::scenario_0151_inflated_request_hoards_honest_does_not",
        "description": "Current market implementation exhibits its named inflated-request behavior.",
    },
}


def fail(message: str) -> None:
    raise ValueError(message)


def source_display(path: Path) -> str:
    """Use a repo-relative source when possible, otherwise preserve an external input path."""
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        return path.as_posix()


def read_extract(path: Path) -> list[dict[str, object]]:
    data = json.loads(path.read_text())
    if data.get("schema") != "wave3-requirement-extract/v1":
        fail(f"{path}: unsupported extract schema")
    records = data.get("requirements")
    if not isinstance(records, list) or not records:
        fail(f"{path}: missing requirements")
    return records


def read_specs(directory: Path) -> tuple[dict[str, set[str]], dict[str, dict[str, str]]]:
    specs_by_file: dict[str, set[str]] = {}
    evidence: dict[str, dict[str, str]] = {}
    for path in sorted(directory.glob("*.md")):
        text = path.read_text()
        spec_ids = set(SPEC_PATTERN.findall(text))
        if not spec_ids:
            continue
        specs_by_file[path.name] = spec_ids
        for evidence_id, command in EVID_ROW.findall(text):
            if evidence_id in evidence:
                fail(f"duplicate evidence ID {evidence_id}")
            evidence[evidence_id] = {
                "source": source_display(path),
                "target_command": command,
                "file": path.name,
            }
    if not evidence:
        fail("no specification evidence rows found")
    return specs_by_file, evidence


def read_bindings(
    path: Path, evidence: dict[str, dict[str, str]], specs_by_file: dict[str, set[str]]
) -> dict[str, list[str]]:
    data = json.loads(path.read_text())
    bindings = data.get("bindings")
    exceptions = data.get("cross_spec_exceptions", {})
    if data.get("schema") != "wave3-evid-spec-bindings/v1" or not isinstance(bindings, dict):
        fail(f"{path}: unsupported EVID-to-SPEC binding schema")
    canonical = json.dumps(bindings, sort_keys=True, separators=(",", ":")).encode()
    if data.get("bindings_sha256") != hashlib.sha256(canonical).hexdigest():
        fail(f"{path}: binding checksum differs (a mapping was changed without revalidation)")
    if not isinstance(exceptions, dict) or set(exceptions) - set(evidence):
        fail(f"{path}: unknown cross-SPEC exception")
    if set(bindings) != set(evidence):
        missing = sorted(set(evidence) - set(bindings))
        extra = sorted(set(bindings) - set(evidence))
        fail(f"{path}: EVID coverage differs; missing={missing}, extra={extra}")
    known_specs = {spec_id for values in specs_by_file.values() for spec_id in values}
    validated: dict[str, list[str]] = {}
    for evidence_id, anchors in bindings.items():
        if not isinstance(anchors, list) or not anchors or any(not isinstance(anchor, str) for anchor in anchors):
            fail(f"{path}: {evidence_id} has no precise SPEC anchor")
        if len(anchors) != len(set(anchors)):
            fail(f"{path}: {evidence_id} repeats a SPEC anchor")
        same_source = specs_by_file[evidence[evidence_id]["file"]]
        allowed = same_source | set(exceptions.get(evidence_id, []))
        unknown = set(anchors) - known_specs
        if unknown:
            fail(f"{path}: {evidence_id} references unknown SPEC anchor {sorted(unknown)}")
        if set(anchors) - allowed:
            fail(f"{path}: {evidence_id} maps outside its source SPEC contract")
        validated[evidence_id] = sorted(anchors)
    return validated


def missing_requirement_anchor_coverage(
    requirements: list[dict[str, object]], bindings: dict[str, list[str]]
) -> dict[str, list[str]]:
    """Return every live REQ anchor absent from all precise mapped EVID guards for that REQ."""
    covered_specs = {anchor for anchors in bindings.values() for anchor in anchors}
    return {
        str(requirement["id"]): sorted(set(requirement["specification_anchors"]) - covered_specs)
        for requirement in requirements
        if set(requirement["specification_anchors"]) - covered_specs
    }


def validate_requirement_anchor_coverage(
    requirements: list[dict[str, object]], bindings: dict[str, list[str]]
) -> None:
    missing = missing_requirement_anchor_coverage(requirements, bindings)
    if missing:
        details = "; ".join(f"{requirement}: {', '.join(anchors)}" for requirement, anchors in sorted(missing.items()))
        fail(f"live REQ SPEC anchors without precise EVID coverage: {details}")


def mutation_test_removed_claim_mapping(
    requirements: list[dict[str, object]], bindings: dict[str, list[str]]
) -> str:
    """Prove reverse coverage notices a real mapped requirement anchor removed in a temp copy."""
    baseline = missing_requirement_anchor_coverage(requirements, bindings)
    candidates = sorted(
        (str(requirement["id"]), anchor)
        for requirement in requirements
        for anchor in requirement["specification_anchors"]
        if any(anchor in mapped for mapped in bindings.values())
    )
    if not candidates:
        fail("removed-claim mutation has no mapped requirement anchor")
    requirement_id, removed = candidates[0]
    mutated = {evidence_id: [anchor for anchor in anchors if anchor != removed] for evidence_id, anchors in bindings.items()}
    after = missing_requirement_anchor_coverage(requirements, mutated)
    if removed not in after.get(requirement_id, []) or removed in baseline.get(requirement_id, []):
        fail(f"removed-claim mutation did not expose {requirement_id} -> {removed}")
    return f"removed-claim mutation exposed {requirement_id} -> {removed}"


def test_names() -> list[str]:
    command = ["cargo", "test", "-p", "simulation", "--", "--list", "--test-threads=1"]
    completed = subprocess.run(command, cwd=ROOT, text=True, capture_output=True, check=False)
    if completed.returncode:
        raise ValueError(f"current regression inventory command failed: {completed.stderr.strip()}")
    names = [match.group(1) for line in completed.stdout.splitlines() if (match := TEST_LINE.match(line))]
    if not names:
        fail("current regression inventory found zero runnable tests")
    return sorted(names)


def build(extract: Path, specs: Path, bindings_path: Path) -> tuple[dict[str, object], dict[str, object], str]:
    requirements = read_extract(extract)
    specs_by_file, evidence = read_specs(specs)
    bindings = read_bindings(bindings_path, evidence, specs_by_file)
    validate_requirement_anchor_coverage(requirements, bindings)
    req_by_spec: dict[str, list[str]] = {}
    for requirement in requirements:
        for spec_id in requirement["specification_anchors"]:
            req_by_spec.setdefault(spec_id, []).append(str(requirement["id"]))

    targets: list[dict[str, object]] = []
    for evidence_id, item in sorted(evidence.items()):
        linked_specs = bindings[evidence_id]
        requirement_ids = sorted({req for spec_id in linked_specs for req in req_by_spec.get(spec_id, [])})
        if not requirement_ids or not linked_specs:
            fail(f"{evidence_id}: has no rewritten requirement/SPEC binding")
        targets.append(
            {
                "id": f"TARGET-{evidence_id}",
                "requirement_ids": requirement_ids,
                "specification_ids": linked_specs,
                "evidence_id": evidence_id,
                "status": "UNIMPLEMENTED",
                "scope": "planned target proof",
                "target_command": item["target_command"],
                "source": item["source"],
                "reason": "The specification is draft and no matching current test function exists.",
            }
        )

    names = test_names()
    current = [
        {
            "id": f"REGRESSION-{index:03}",
            "test": name,
            "command": f"cargo test -p simulation {name} -- --test-threads=1",
            "scope": "current substrate regression",
        }
        for index, name in enumerate(names, 1)
    ]
    promoted = []
    for sentinel_id, item in sorted(PROMOTED.items()):
        name = item["test"]
        if name not in names:
            fail(f"{sentinel_id}: promoted current test is absent")
        promoted.append(
            {
                "id": sentinel_id,
                "test": name,
                "command": f"cargo test -p simulation {name} -- --test-threads=1",
                "scope": "current substrate regression",
                "description": item["description"],
            }
        )
    target_data = {
        "schema": "wave3-target-evidence/v1",
        "authority": "rewritten requirement-to-SPEC bindings; draft target evidence",
        "target_scenarios": targets,
        "promoted_current_regressions": promoted,
    }
    inventory = {
        "schema": "wave3-current-regression-inventory/v1",
        "authority": "current simulation test list; not target-proof evidence",
        "regressions": current,
    }
    coverage = coverage_markdown(requirements, targets, evidence)
    validate(target_data, inventory, requirements, evidence)
    return target_data, inventory, coverage


def coverage_markdown(requirements: list[dict[str, object]], targets: list[dict[str, object]], evidence: dict[str, dict[str, str]]) -> str:
    by_requirement: dict[str, list[str]] = {str(item["id"]): [] for item in requirements}
    for target in targets:
        for requirement_id in target["requirement_ids"]:
            by_requirement[requirement_id].append(target["evidence_id"])
    implemented_total = sum(1 for target in targets if target["status"] == "IMPLEMENTED")
    lines = [
        "# Wave 3 evidence coverage",
        "",
        "**Kind:** generated evidence coverage",
        "**Authority:** reporting only",
        "**Status:** draft",
        "**Owner:** project lead",
        "**Last verified:** 2026-08-24",
        "**Generator:** `python3 docs/plan/iterations/evidence/build_evidence.py --extract docs/plan/iterations/extract/requirements.json --specifications docs/reference/specifications --bindings docs/plan/iterations/evidence/evid-spec-bindings.json --output-dir docs/plan/iterations/evidence`",
        "",
        "All rows are planned target evidence. `UNIMPLEMENTED` means no target guard has been implemented or mutation-proven; current regressions are listed separately and are not target proof.",
        "",
        "| Requirement | Planned EVID anchors | Implemented | Status |",
        "| --- | ---: | ---: | --- |",
    ]
    for requirement in sorted(requirements, key=lambda item: str(item["id"])):
        count = len(by_requirement[str(requirement["id"])])
        implemented = sum(
            1
            for target in targets
            if requirement["id"] in target["requirement_ids"] and target["status"] == "IMPLEMENTED"
        )
        status = "IMPLEMENTED" if count == implemented else "UNIMPLEMENTED"
        lines.append(f"| `{requirement['id']}` | {count} | {implemented} | {status} |")
    lines.extend(
        [
            "",
            "| Current EVID anchors | Planned target scenarios | Implemented target scenarios | Uncovered EVID anchors |",
            "| ---: | ---: | ---: | ---: |",
            f"| {len(evidence)} | {len(targets)} | {implemented_total} | {len(evidence) - len(targets)} |",
            "",
            "Every current EVID anchor is represented by exactly one `TARGET-EVID-*` scenario; every target binds one or more re-derived `REQ-*` and `SPEC-*` identifiers.",
            "",
        ]
    )
    return "\n".join(lines)


def inventory_markdown(inventory: dict[str, object]) -> str:
    lines = [
        "# Current simulation regression inventory",
        "",
        "**Kind:** generated current-regression inventory",
        "**Authority:** current substrate only",
        "**Status:** informational — not target-proof evidence",
        "**Owner:** project lead",
        "**Last verified:** 2026-08-24",
        "**Generator:** `python3 docs/plan/iterations/evidence/build_evidence.py --extract docs/plan/iterations/extract/requirements.json --specifications docs/reference/specifications --bindings docs/plan/iterations/evidence/evid-spec-bindings.json --output-dir docs/plan/iterations/evidence`",
        "",
        "These are every test currently listed by the serial `simulation` test binary. They are deliberately separate from planned `TARGET-EVID-*` scenarios; identical numeric fragments from the legacy corpus are never used as a target binding.",
        "",
        "| Regression ID | Current test | Exact command | Scope |",
        "| --- | --- | --- | --- |",
    ]
    for row in inventory["regressions"]:
        lines.append(f"| `{row['id']}` | `{row['test']}` | `{row['command']}` | {row['scope']} |")
    lines.append("")
    return "\n".join(lines)


def validate(target_data: dict[str, object], inventory: dict[str, object], requirements: list[dict[str, object]], evidence: dict[str, dict[str, str]]) -> None:
    targets = target_data["target_scenarios"]
    target_ids = [target["id"] for target in targets]
    evidence_ids = [target["evidence_id"] for target in targets]
    if len(target_ids) != len(set(target_ids)) or len(evidence_ids) != len(set(evidence_ids)):
        fail("target scenarios have duplicate IDs or evidence bindings")
    if set(evidence_ids) != set(evidence):
        fail("target scenarios do not cover every current EVID anchor")
    known_requirements = {str(requirement["id"]) for requirement in requirements}
    for target in targets:
        if target["status"] != "UNIMPLEMENTED" or target["scope"] != "planned target proof":
            fail(f"{target['id']}: invalid target status/scope")
        if not target["requirement_ids"] or not target["specification_ids"] or not target["target_command"]:
            fail(f"{target['id']}: incomplete requirement/SPEC/EVID binding")
        if not set(target["requirement_ids"]).issubset(known_requirements):
            fail(f"{target['id']}: unknown requirement binding")
        if "TBD" in json.dumps(target):
            fail(f"{target['id']}: contains TBD")
    covered_requirements = {requirement_id for target in targets for requirement_id in target["requirement_ids"]}
    if covered_requirements != known_requirements:
        fail(f"requirements without planned evidence: {', '.join(sorted(known_requirements - covered_requirements))}")
    regressions = inventory["regressions"]
    if not regressions or len({row["id"] for row in regressions}) != len(regressions):
        fail("current regression inventory is empty or has duplicate IDs")
    names = {row["test"] for row in regressions}
    for promoted in target_data["promoted_current_regressions"]:
        expected = f"cargo test -p simulation {promoted['test']} -- --test-threads=1"
        if (
            promoted["scope"] != "current substrate regression"
            or promoted["test"] not in names
            or promoted["command"] != expected
            or "TBD" in json.dumps(promoted)
            or "UNIMPLEMENTED" in json.dumps(promoted)
        ):
            fail(f"{promoted['id']}: invalid promoted current regression")
        completed = subprocess.run(
            promoted["command"].split(), cwd=ROOT, text=True, capture_output=True, check=False
        )
        combined = completed.stdout + completed.stderr
        if completed.returncode or not re.search(r"running [1-9][0-9]* test", combined) or not re.search(
            r"test result: ok\. [1-9][0-9]* passed;", combined
        ):
            fail(f"{promoted['id']}: command did not execute a passing nonzero test")


def write_or_check(path: Path, content: bytes, check: bool) -> None:
    if check:
        if not path.is_file() or path.read_bytes() != content:
            fail(f"{path}: differs from deterministic evidence output")
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(content)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--extract", type=Path, default=DEFAULT_EXTRACT)
    parser.add_argument("--specifications", type=Path, default=DEFAULT_SPECS)
    parser.add_argument("--bindings", type=Path, default=DEFAULT_BINDINGS)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--test-removed-claim-mutation", action="store_true")
    args = parser.parse_args()
    args.extract = args.extract.resolve()
    args.specifications = args.specifications.resolve()
    args.bindings = args.bindings.resolve()
    args.output_dir = args.output_dir.resolve()
    try:
        requirements = read_extract(args.extract)
        specs_by_file, evidence = read_specs(args.specifications)
        bindings = read_bindings(args.bindings, evidence, specs_by_file)
        if args.test_removed_claim_mutation:
            print(mutation_test_removed_claim_mapping(requirements, bindings))
            return 0
        target, inventory, coverage = build(args.extract, args.specifications, args.bindings)
        write_or_check(args.output_dir / "target-scenarios.json", (json.dumps(target, indent=2, sort_keys=True) + "\n").encode(), args.check)
        write_or_check(args.output_dir / "current-regression-inventory.json", (json.dumps(inventory, indent=2, sort_keys=True) + "\n").encode(), args.check)
        write_or_check(args.output_dir / "current-regression-inventory.md", inventory_markdown(inventory).encode(), args.check)
        write_or_check(args.output_dir / "coverage.md", coverage.encode(), args.check)
        print(f"validated {len(target['target_scenarios'])} planned EVID scenarios and {len(inventory['regressions'])} current regressions")
        return 0
    except ValueError as error:
        print(f"evidence validation failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
