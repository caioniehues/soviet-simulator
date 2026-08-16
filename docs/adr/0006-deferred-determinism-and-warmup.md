# Deliberate deferrals: bit-determinism and the warm-up settle pass

Two doors left consciously open, not assumptions:

**Determinism** — the sim commits to single-run stability only (stable bucket assignment +
explicit system ordering). Cross-platform bit-determinism (replay/multiplayer) is deferred:
adopting it now would forbid the `par_iter` accumulation patterns and float math the
quarter-million spike relies on. Revisit only if replay/multiplayer becomes a goal.

**Warm-up** — the carried 16,384-frame headless settle pass (pipes settled before the player sees
frame one) is a documented pattern, not built until a system needs settling; earliest forcing
function is production chains. Systems will opt out via a sentinel substep per the CS1 pattern.
