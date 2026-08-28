---
name: rederive-recorded-spike-numbers
description: A benchmark number recorded in a bd close reason or a git tag message may be measured in an asymmetric timed region — re-derive before citing or acting on it
metadata:
  type: feedback
---

Before citing a performance number from a closed ticket, a close reason, or a
git tag message, **re-run it and read the timed region**. Check three things:
are the two timed loops symmetric in scaffolding; is it more than one sample;
does the stated *cause* survive reading the source.

**Why:** sov-dda.3 (2026-08-27) was closed with "1.48 us/tick queued vs 0.65
baseline, about 2.3x, because it re-sorts each lane every tick". That number
was copied into the `archive/sov-dda-3-lane-queues` tag message and became the
epic's stated integration risk ("that sort must be designed out before any
adoption"). All three checks failed. The queued timed loop carried a per-tick
`first_overlap()` sweep the baseline loop did not. It was one un-repeated
debug-build sample totalling under 5 ms — re-running the unchanged test four
times gave 1.33x, 1.56x, 8.31x and 0.63x, the last with the queue model
*faster*. And the stated cause was contradicted by the module itself:
`tick_proximity()` ends with the same `rebuild_order()` call `tick_queued()`
begins with, so the sort was common to both models and could not be the
marginal cost. A fair symmetric median gave 1.48x at 5 trucks and **inverted**
to 0.26x at 500.

**How to apply:** a micro-benchmark whose total runtime is single-digit
milliseconds in a debug build is noise; demand a median over repetitions. When
an A/B ratio is the deliverable, diff the two timed regions line by line — the
asymmetry is usually an assertion or a clone, not the model. And always sweep
the scaling parameter: a ratio measured at one N (here 5) can reverse sign at
another, and a spike's own null model is often a strawman for the real engine
(here the spike baseline was O(n^2); live `road.rs` uses a bounded
TransportGrid radius query and is not).

Related: [[feedback-stale-brief-check]].
