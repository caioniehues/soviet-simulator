# Wave 3 requirements generator

**Kind:** requirements-index
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

The five requirement documents and the migration ledger are deterministic generated artifacts.
The archived legacy corpus at `docs/archive/iterations/legacy/corpus/requirements/` supplies only
STORY identity/title coverage; the requirement catalogue in the generator derives its live
contracts from the charter and stable specification anchors.

Canonical generation command:

```bash
python3 docs/plan/iterations/requirements/build_requirements.py
```

Canonical reproducibility and validation command:

```bash
python3 docs/plan/iterations/requirements/build_requirements.py --check
```

`--check` writes a complete replacement corpus to a temporary directory, byte-compares every
generated requirement file and `story-migration.md` with the declared outputs, then validates
coverage, metadata, anchors, cut dispositions, duplicate identities, and legacy-authority bans.
Use `--output-dir` and `--migration-output` together to generate or validate another complete
output location.
