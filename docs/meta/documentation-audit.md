# Documentation audit

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

This page defines the documentation validation baseline. It does not classify every document or
replace the [document authority](document-authority.md) model.

## Baseline

- Markdown under `docs/` is canonical. The mdBook output is a view.
- Each active wiki page has the required metadata and is reachable from `SUMMARY.md`.
- Relative links and `SUMMARY.md` targets resolve in the repository.
- Implementation claims are checked against source and tests. Source wins on disagreement.
- The rendered book must build with the configured preprocessors.

## Commands

```bash
python3 scripts/check_docs.py
mdbook build
```

## Related

- [Documentation model](documentation-model.md)
- [Document authority](document-authority.md)
- [Mermaid rendering](mermaid.md)
