---
name: bind-operator-supplied-proof
description: When fixing a gate that passes on nothing, bind the new proof-of-provenance argument to the run in the SAME commit — an unbound argument is just a new place to be wrong
metadata:
  type: feedback
---

When a gate or check is fixed by adding an argument that carries proof (a record, a manifest,
a baseline), **bind that argument to the run in the same commit**. An unbound path is an
assertion the operator supplies and the tool believes.

**Why:** measured on `tools/run_validation_gate.py`, 2026-08-28. B2 was "the gate passes on
zero input"; the fix added `--capture-record` so a run could prove validation ran. The
reviewer then found N1 — the record was a free-floating path with no tie to the child's
output. Moving `--out` to a fresh directory and forgetting to move `--capture-record` made a
run with `validation_requested: false` and zero validation markers report PASS. Same defect,
new shape, one round trip wasted. Worse: the test I wrote for B2 hand-wrote the record before
the run and asserted PASS, so it *enshrined* the hole as expected behaviour.

**How to apply:**
- Ask "who supplies this, and what stops them supplying the wrong one?" before shipping the
  argument.
- Cheapest general binding is freshness: stamp a start time before spawning the child, refuse
  any artifact whose mtime predates it. Floor the stamp to the second so coarse filesystem
  timestamp granularity cannot fail an artifact written *during* the run.
- Prefer freshness over deriving the path from the child's CLI when the tool wraps an
  arbitrary command — parsing the child's flags couples a general tool to one binary and
  silently stops binding when those flags change. This trade-off was accepted by the review
  gate; record it in a docstring so it is not re-litigated.
- **In a test for a provenance check, have the child write the artifact.** A test that
  pre-writes it is testing the hole, not the guard. See [[renderer-proof-obligations]].
