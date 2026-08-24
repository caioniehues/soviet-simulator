---
name: gotcha-verdict-before-reading
description: 2026-08-23 — I ranked three skill frameworks from their descriptions/frontmatter, published a verdict table, then reversed it after actually reading them; never rank tools you have not opened
metadata:
  type: feedback
---

**Do not publish a comparative verdict on tools, skills or frameworks until you have read each
one's actual body.** Frontmatter and `description` fields are marketing copy, not the thing.

**Why (2026-08-23, the superpowers / iterative-development / mattpocock comparison):**

Asked "what fits best our project", I answered immediately with a three-row verdict table —
*Keep · Keep · Install for one specific gap* — built from installed-plugin listings, descriptions
and repo artifacts. I had opened none of the three skill bodies.

The user did not accept it, twice:

> "hmm lets expand into the superpowers vs iterative development vs mattpacock"

> "wait. but if we HAD to choose, wat would be? mattpocock?"

Only then did I go to the sources — *"Let me read the actual substance rather than compare
descriptions — mattpocock is the one I've only seen frontmatter for"* — and my own next message
opened: **"This changes my answer."** The corrected read found what the descriptions could not
show: all three had already been used on this repo in sequence, superpowers' planning spine had
**never** run here (`docs/superpowers/plans/` is empty), and the deciding property was a regression
corpus that only one of them has. Final verdict: *"No — iterative-development. And for this
project right now it isn't close."*

Two failures in one, both already named in standing rules I broke:

1. `rules/workflow.md`: *"read primary sources fully ('hard source') — never answer from a summary
   or memory when the repo/docs are available."* The skill bodies were on local disk the whole time.
2. `rules/workflow.md`: *"always lead with a concrete recommendation... Never present an
   open-ended option list without a pick."* My table picked nothing; the user had to demand a pick.

This is the project's signature failure — *a document asserting something its subject does not
support* — turned inward on tool evaluation. I had catalogued that exact failure mode in the very
same message, citing five instances of it, while committing it.

**How to apply:**

- A comparison question is a **reading** task before it is a judgement task. Open every candidate's
  actual content first; budget for that rather than answering fast from listings.
- If asked to compare before you can read, say so and give a provisional pick with the label:
  *"provisional, from descriptions only — I have not opened them."* An honest provisional is fine;
  a confident table from frontmatter is not.
- Always name a single winner, then the caveats. The user reliably pushes back on a table that
  ranks nothing, and the push-back costs a full round-trip.
- Watch for the user's "hmmm" / "im confused" — in this session both marked *I produced comparison
  prose where a decision was wanted*, not a genuine ambiguity in the subject.

See [[gotcha-inherited-claims]] — the same untrusted-claim discipline, applied to briefs rather
than to tools.
