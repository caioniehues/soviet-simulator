# Serialization standard

**Kind:** standard
**Authority:** operational; rules marked *(target)* depend on the persistence decision
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Rules

1. **Must:** every serialised resource is registered through `init.rs` (`register_resource`) so
   `SimulationSer` and `hashes()` see it; a resource that must not persist uses
   `register_resource_noserialize`.
2. **Must:** entity identity survives a save cycle — `slotmapd` exists for this; never replace it
   with upstream `slotmap` without proving the round-trip invariant another way.
3. **Must:** a change that alters a serialised layout says so in its `bd` issue and its commit,
   and either ships a migration *(target)* or declares an explicit version-gated hard break (the
   charter allows hard breaks during development; none after the 1.0 release candidate).
4. **Should** *(target)*: released saves carry an envelope — magic, format version, schema/game
   version, codec, sizes, checksum, payload — and load through a `SaveMigration` sequence
   ([persistence](../architecture/persistence.md)).
5. **Should** *(target)*: a major-version mismatch is rejected, not warned past. Today it warns and a
   resource that fails to decode silently becomes its default, producing a hybrid world.
6. **Must:** the round-trip determinism check in `TestCtx` keeps passing after any serialisation
   change; a new resource adds itself to that check by registration.
7. **Should:** keep internal snapshots (`ArcSwap`-published views, caches) on a separate, faster
   path from the released save contract; `rkyv` is a candidate there only.

## Related

- [Persistence (architecture)](../architecture/persistence.md)
- [Determinism standard](determinism.md)
- [Current substrate — persistence](../architecture/current-substrate.md#persistence-and-determinism)
