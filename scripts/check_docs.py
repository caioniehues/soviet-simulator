#!/usr/bin/env python3
"""Lightweight documentation checks for docs/.

Checks:
  1. Broken relative links in every active Markdown file (docs/ minus docs/archive/, plus the
     root entry points README.md, AGENTS.md, CLAUDE.md, CONTEXT.md).
  2. Every SUMMARY.md target exists.
  3. Active pages under the wiki sections that are not reachable from SUMMARY.md (orphans).
  4. Duplicate H1 titles among active pages.
  5. Metadata block (Kind, Authority, Status, Owner, Last verified) on every specification and
     on every page under the wiki sections.

Exit status is non-zero on any error. Warnings do not fail the run.
No third-party dependencies.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCS = ROOT / "docs"
ARCHIVE = DOCS / "archive"
ROOT_ENTRYPOINTS = ["README.md", "AGENTS.md", "CLAUDE.md", "CONTEXT.md"]

# Sections whose pages must be reachable from SUMMARY.md and must carry the metadata block.
WIKI_SECTIONS = [
    "product",
    "simulation",
    "architecture",
    "engineering",
    "developer",
    "meta",
    "reference",
    "vision",
]

LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
FENCE_RE = re.compile(r"^\s*(```|~~~)")
H1_RE = re.compile(r"^#\s+(.+?)\s*$")
META_FIELDS = ["Kind", "Authority", "Status", "Owner", "Last verified"]

errors: list[str] = []
warnings: list[str] = []


def active_markdown() -> list[Path]:
    files = [p for p in DOCS.rglob("*.md") if ARCHIVE not in p.parents]
    files += [ROOT / f for f in ROOT_ENTRYPOINTS if (ROOT / f).exists()]
    return sorted(files)


def strip_fences(text: str) -> list[str]:
    """Return lines with fenced code blocks blanked out, so links in code are ignored."""
    out: list[str] = []
    in_fence = False
    for line in text.splitlines():
        if FENCE_RE.match(line):
            in_fence = not in_fence
            out.append("")
            continue
        out.append("" if in_fence else line)
    return out


def check_links(files: list[Path]) -> None:
    for path in files:
        text = path.read_text(encoding="utf-8", errors="replace")
        for lineno, line in enumerate(strip_fences(text), start=1):
            for target in LINK_RE.findall(line):
                if target.startswith(("http://", "https://", "mailto:", "#")):
                    continue
                target_path = target.split("#", 1)[0]
                if not target_path:
                    continue
                resolved = (path.parent / target_path).resolve()
                if not resolved.exists():
                    errors.append(f"{path.relative_to(ROOT)}:{lineno}: broken link -> {target}")


def summary_targets() -> set[Path]:
    summary = DOCS / "SUMMARY.md"
    targets: set[Path] = set()
    if not summary.exists():
        errors.append("docs/SUMMARY.md is missing")
        return targets
    for lineno, line in enumerate(summary.read_text(encoding="utf-8").splitlines(), start=1):
        for target in LINK_RE.findall(line):
            resolved = (DOCS / target.split("#", 1)[0]).resolve()
            if not resolved.exists():
                errors.append(f"docs/SUMMARY.md:{lineno}: target does not exist -> {target}")
            else:
                targets.add(resolved)
    return targets


def check_orphans(targets: set[Path]) -> None:
    for section in WIKI_SECTIONS:
        base = DOCS / section
        if not base.exists():
            continue
        for page in base.rglob("*.md"):
            if page.name.startswith("_"):
                continue
            if page.resolve() not in targets:
                # Specifications and reference material are navigated through their own index;
                # only wiki-section pages are required in SUMMARY.md.
                if section == "reference" and "specifications" in page.parts:
                    continue
                warnings.append(f"{page.relative_to(ROOT)}: not reachable from docs/SUMMARY.md")


def check_titles(files: list[Path]) -> None:
    seen: dict[str, Path] = {}
    for path in files:
        if ARCHIVE in path.parents:
            continue
        for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
            m = H1_RE.match(line)
            if m:
                title = m.group(1).strip()
                if title in seen and not path.name.startswith("_"):
                    warnings.append(
                        f"duplicate H1 '{title}': {seen[title].relative_to(ROOT)} and {path.relative_to(ROOT)}"
                    )
                else:
                    seen.setdefault(title, path)
                break


def check_metadata() -> None:
    required: list[Path] = list((DOCS / "reference" / "specifications").glob("*.md"))
    for section in WIKI_SECTIONS:
        base = DOCS / section
        if base.exists():
            required += [p for p in base.rglob("*.md") if not p.name.startswith("_")]
    for path in sorted(set(required)):
        if path.name == "README.md" and "specifications" not in path.parts:
            continue
        head = "\n".join(path.read_text(encoding="utf-8", errors="replace").splitlines()[:20])
        missing = [f for f in META_FIELDS if f"**{f}:**" not in head]
        if missing:
            errors.append(f"{path.relative_to(ROOT)}: metadata block missing {', '.join(missing)}")


def main() -> int:
    files = active_markdown()
    check_links(files)
    targets = summary_targets()
    check_orphans(targets)
    check_titles(files)
    check_metadata()

    for w in warnings:
        print(f"WARN  {w}")
    for e in errors:
        print(f"ERROR {e}")
    print(f"\n{len(files)} active files checked; {len(errors)} error(s), {len(warnings)} warning(s).")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
