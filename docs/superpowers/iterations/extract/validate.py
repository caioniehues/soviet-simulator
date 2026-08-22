#!/usr/bin/env python3
"""Validate one extraction JSON. Usage: python3 validate.py <file.json>"""
import json, re, sys

IMPACT = {'none', 'local', 'cross-surface', 'journey'}
SEAM = {'unit', 'integration', 'app-level', 'process-level', 'e2e'}
KIND = {'surface', 'failure-recovery', 'contract'}

p = sys.argv[1]
d = json.load(open(p))
bad = []
if set(d) != {'stories', 'scenarios'}:
    bad.append(f"top-level keys must be exactly stories+scenarios, got {sorted(d)}")
titles = {s['title'] for s in d['stories']}
if len(titles) != len(d['stories']):
    bad.append("duplicate story titles")
acs = 0
for s in d['stories']:
    for k in ('title', 'epic_theme', 'as_a', 'i_want', 'so_that', 'acceptance_criteria', 'sources'):
        if not s.get(k):
            bad.append(f"story {s.get('title')!r}: missing/empty {k}")
    for ac in s.get('acceptance_criteria', []):
        acs += 1
        t = ac.get('text', '')
        if not re.search(r'\[SUBSTRATE:[^\]]+\]\s*$', t):
            bad.append(f"AC {s['title']}/{ac.get('id')}: text must END with [SUBSTRATE: ...]")
        if ac.get('behavioral_impact') not in IMPACT:
            bad.append(f"AC {s['title']}/{ac.get('id')}: bad behavioral_impact {ac.get('behavioral_impact')!r}")
        if ac.get('proof_seam') not in SEAM:
            bad.append(f"AC {s['title']}/{ac.get('id')}: bad proof_seam {ac.get('proof_seam')!r}")
        if re.search(r'\b(STORY|EPIC)-\d', t):
            bad.append(f"AC {s['title']}/{ac.get('id')}: must not assign STORY-/EPIC- IDs")
    ids = [a.get('id') for a in s.get('acceptance_criteria', [])]
    if len(set(ids)) != len(ids):
        bad.append(f"story {s['title']!r}: duplicate AC ids")
for sc in d['scenarios']:
    if sc.get('kind') not in KIND:
        bad.append(f"scenario {sc.get('title')!r}: bad kind {sc.get('kind')!r}")
    if sc.get('proof_seam') not in SEAM:
        bad.append(f"scenario {sc.get('title')!r}: bad proof_seam")
    for k in ('steps', 'final_observables', 'owning_story_titles', 'sources'):
        if not sc.get(k):
            bad.append(f"scenario {sc.get('title')!r}: missing/empty {k}")
    for t in sc.get('owning_story_titles', []):
        if t not in titles:
            bad.append(f"scenario {sc.get('title')!r}: owns unknown story {t!r}")
    for st in sc.get('steps', []):
        if not st.get('action') or not st.get('expected'):
            bad.append(f"scenario {sc.get('title')!r}: step missing action/expected")

print(f"{p}: stories={len(d['stories'])} acs={acs} scenarios={len(d['scenarios'])}")
if bad:
    print(f"FAIL ({len(bad)} problems)")
    for b in bad[:40]:
        print("  -", b)
    sys.exit(1)
print("OK")
