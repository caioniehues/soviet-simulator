#!/usr/bin/env python3
"""Apply the charter-grounded 1.0 scope cut to requirements/EPIC-*.md.

Run from anywhere; paths resolve relative to this file. Re-run build_roadmap.py
FROM THE REPO ROOT afterwards.

THREE mechanisms, deliberately distinct. Picking the wrong one is how this script
did damage on its first pass (opus gate, 2026-08-22, 5 CONFIRMED findings):

  FULL  -> inserts '**Deferred:** true'. build_roadmap.py:87 detects it and drops
           the story from the roadmap.
  AC_X  -> prefixes ONE AC line, excluding just that AC. The marker must NEVER
           contain 'DEFERRED to Post-1.0' — line 87 matches that substring at
           STORY-BLOCK level and would defer the whole story. Use ONLY when the
           entire AC is out of scope.
  AC_ED -> rewrites AC text in place. Use when a single sentence mixes deferred
           and in-scope content. Excluding such an AC deletes content the charter
           ships: STORY-0078 AC-2 caps kindergarten AND school AND university in
           one sentence, and charter:92 ships two of those three.

Idempotent. Verifies every declared target was found; exits 1 otherwise.
"""
import re, sys, pathlib

REQ = pathlib.Path(__file__).parent / 'requirements'

# ---- whole-story deferrals -------------------------------------------------
FULL = {
    # charter:107 "B11 crime" — the entire justice chain.
    'STORY-0088': 'charter:107 "B11 crime"',
    'STORY-0089': 'charter:107 "B11 crime"',
    'STORY-0090': 'charter:107 "B11 crime"',
    'STORY-0091': 'charter:107 "B11 crime"',
    'STORY-0092': 'charter:107 "B11 crime"',
    # charter:108 "voltage tiers" + charter:110 "grid depth (transformers, ...)"
    'STORY-0019': 'charter:108 "voltage tiers"; charter:110 "grid depth (transformers, ...)"',
    'STORY-0020': 'charter:110 "grid depth (transformers, ...)" — HIGH/LOW topology needs STORY-0019',
    'STORY-0031': 'charter:110 "grid depth (... electric-heating fallback)"',
    # Added by the opus gate: its only AC is premised on STORY-0139 AC-1's
    # empty-tank halt, which is itself deferred. Left scheduled it is unprovable.
    'STORY-0026': 'charter:106 "vehicle lifecycle including fuel-as-commodity" — premised on the deferred empty-tank halt (STORY-0139 AC-1)',
    # charter:108 "dual currency"; charter:95 "single rouble"
    'STORY-0053': 'charter:108 "dual currency"; charter:95 "single rouble"',
    # Pre-existing: its AC-1 already carried the block-level trigger phrase, so
    # build_roadmap dropped it while this script did not declare it — 18 declared
    # vs 19 actual. Declared now so the roadmap is reproducible from this file.
    'STORY-0054': 'charter:108 "dual currency"; charter:95 "single rouble" — no loan mechanic appears anywhere in the charter',
    'STORY-0056': 'charter:108 "era calendar from 1917"; charter:95 "one fixed 1950s–60s era, flat catalogue"',
    'STORY-0057': 'charter:106 "vehicle lifecycle including fuel-as-commodity" — resale needs wear tracking',
    'STORY-0110': 'charter:113 "perishables and refrigerated transport"',
    'STORY-0120': 'charter:106 "vehicle lifecycle"; charter:108 "era calendar from 1917"',
    'STORY-0142': 'charter:108 "vehicle manufacture"',
    'STORY-0144': 'charter:106 "vehicle lifecycle including fuel-as-commodity"',
    'STORY-0045': 'charter:95 "All 16 resources trade both ways at fixed per-kind prices (no market)"',
    'STORY-0038': 'charter:110 "passenger rail, signals, electrification" — and no charter row ships passenger transport at all',
    # NOT deferred (reverted by the opus gate): STORY-0039. Its single AC is a
    # rail<->road transship and never invokes a ship or aircraft; charter:94
    # ships "Rail | Minimal freight — 3 buildings" and R12 names the cargo
    # station explicitly. Deferring it left rail freight with nowhere to go.
}

# ---- whole-AC exclusions (AC is entirely out of scope) ---------------------
AC_X = {
    ('STORY-0082', 'AC-4'): 'charter:104-105 "Loyalty / legitimacy / broadcast / monuments (the crown jewel, gets its own design effort)"',
    ('STORY-0139', 'AC-1'): 'charter:106 "vehicle lifecycle including fuel-as-commodity" — the fuel field and its empty-tank halt; vehicle-as-owned-asset remains in 1.0 via the other ACs',
}

# ---- in-place AC text edits (sentence mixes deferred + in-scope content) ---
# (story, ac, exact-old, new). Exact-old must appear verbatim exactly once.
AC_ED = [
    # charter:92 ships "education at two tiers (School + Technical Institute)".
    # Drop only the kindergarten value; keep the school/university ceilings.
    ('STORY-0078', 'AC-2', '(kindergarten 10, school 12, university 3)', '(school 12, university 3)'),
    ('STORY-0122', 'AC-2', '(kindergarten < school < university)', '(school < university)'),
    ('STORY-0122', 'AC-3', 'kindergarten 10 per cycle, school 12 per cycle, university 3 per cycle',
                           'school 12 per cycle, university 3 per cycle'),
    # charter:117 Never — "Tourism, hotels and attractions". Data-only enum, but
    # the Never list is absolute.
    ('STORY-0112', 'AC-3', '(basic | advanced | hotel | prison)', '(basic | advanced | prison)'),
    ('STORY-0112', 'AC-3', '$STORAGE_DEMAND_BASIC/_ADVANCED/_HOTEL/_PRISON', '$STORAGE_DEMAND_BASIC/_ADVANCED/_PRISON'),
    # charter:119 Never — "Fires and disasters". Ambulance is in scope (charter:92).
    ('STORY-0080', 'AC-2', 'key-service vehicles (fire/ambulance/personal)', 'key-service vehicles (ambulance/personal)'),
    # STORY-0039 stays scheduled; narrow the narrative so the ship/air legs that
    # triggered the bad deferral are not implied as 1.0 work.
    ('STORY-0039', None, 'transship goods between transport media (rail to road, road to ship, ship to air)',
                         'transship goods between transport media (rail to road)'),
    ('STORY-0039', 'AC-1', 'of a different medium (e.g. road)', 'of a different medium (road)'),
]

FULL_MARK = '**Deferred:** true'
AC_MARK = '(POST-1.0 AC — excluded from 1.0 per '
# Normalise the one pre-existing AC that used the block-level trigger phrase.
LEGACY = '(DEFERRED to Post-1.0 per docs/charter-1.0.md:108 — captured, not scheduled for 1.0) '

seen_full, seen_x, seen_ed = set(), set(), set()

for path in sorted(REQ.glob('EPIC-*.md')):
    txt = orig = path.read_text()
    out = []
    for blk in re.split(r'\n(?=## STORY-)', txt):
        m = re.match(r'## (STORY-\d+)', blk)
        if not m:
            out.append(blk); continue
        sid = m.group(1)

        # 1. in-place text edits first (they must not be masked by a prefix)
        for s, ac, old, new in AC_ED:
            if s != sid:
                continue
            if old in blk:
                blk = blk.replace(old, new, 1); seen_ed.add((s, ac, old))
            elif new in blk:
                seen_ed.add((s, ac, old))  # already applied

        # 2. whole-AC exclusions
        for (s, ac), reason in AC_X.items():
            if s != sid:
                continue
            if re.search(rf'\n- {re.escape(ac)}: \(POST-1\.0 AC', blk):
                seen_x.add((s, ac))
            else:
                blk, n = re.subn(rf'(\n- {re.escape(ac)}: )', rf'\1{AC_MARK}{reason}) ', blk, count=1)
                if n:
                    seen_x.add((s, ac))

        # 3. whole-story deferral
        if sid in FULL:
            blk = blk.replace(LEGACY, '')  # drop the legacy in-AC marker if present
            if FULL_MARK not in blk:
                blk = re.sub(r'(\n\*\*Title:\*\* .*\n)',
                             rf'\1\n{FULL_MARK}\n**Deferred reason:** {FULL[sid]}\n', blk, count=1)
            seen_full.add(sid)

        out.append(blk)

    txt = '\n'.join(out)
    if txt != orig:
        path.write_text(txt); print(f'wrote {path.name}')

missing = []
if set(FULL) - seen_full:   missing.append(f'FULL={sorted(set(FULL)-seen_full)}')
if set(AC_X) - seen_x:      missing.append(f'AC_X={sorted(set(AC_X)-seen_x)}')
ed_keys = {(s, a, o) for s, a, o, _ in AC_ED}
if ed_keys - seen_ed:       missing.append(f'AC_ED={sorted(ed_keys-seen_ed)}')
if missing:
    print('FAIL missing ' + ' '.join(missing), file=sys.stderr); sys.exit(1)
print(f'OK {len(seen_full)} full defers, {len(seen_x)} AC exclusions, {len(seen_ed)} AC text edits')
