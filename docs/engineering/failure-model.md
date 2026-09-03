# Failure model standard

**Kind:** standard
**Authority:** operational (the never-game-over pillar is binding)
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

Failure is simulated, persistent and recoverable. It is never a bare boolean and never a game over.

## Rules

1. **Must:** a failed or blocked transition leaves state that answers:

   ```text
   what is waiting?
   why?
   since when?
   which physical or institutional object owns the problem?
   what can recover it?
   ```

2. **Must not:** use a generic `failed: bool` where a recoverable state with a reason exists.
   `DispatchState` with a named waiting arm and a bounded wait is the pattern; an unbounded wait
   with no reason was the `sov-ahw` defect.
3. **Must not:** delete demand, goods, citizens, vehicles, queues or sites because a transaction
   failed. Unmatched demand persists as a queue with age; cargo in custody stays in custody;
   `ECO-SUB-001` (unmatched orders removed with `mem::take`) is the live violation to retire.
4. **Must:** every waiting state is bounded or explicitly infinite by design, and the bound is a
   named constant with a comment saying what it protects (`MAX_RETURN_ROUTE_RETRIES` and the
   `sov-ahw` `MAX_SOURCE_WAIT_TICKS` are the pattern).
5. **Must:** recovery is a physical path — a truck returns, a reservation is released, an order is
   re-posted — never a teleport back to the source.
6. **Must:** failure is observable: the reason reaches the inspector and, where the player should
   act, a notification derived from causal state ([observability](observability.md)).
7. **Must not:** end the game. Degradation is queues, shortages, stalls, substitution, colder homes,
   going without, underutilisation, surplus and delay.
8. **Should:** failure types are enums with a reason payload, so tests can assert the reason and
   the inspector can show it.

## Related

- [Simulation transitions standard](simulation-transitions.md)
- [Scarcity (concept)](../simulation/concepts/scarcity.md)
- [Queues (concept)](../simulation/concepts/queues.md)
- [Logistics (design) — recovery](../simulation/physical-economy/logistics.md)
