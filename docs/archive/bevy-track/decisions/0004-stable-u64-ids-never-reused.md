# Per-family u64 IDs, never reused; runtime Entity refs never serialized

Band-bucket hashing (ADR 0002) and the save-remap pass both key on serialized identity, so the ID
scheme could not wait for the save/load ticket. Decision: per-family monotonic `u64` counters
(`CitizenId`, `BuildingId`, …) from a serialized `IdAllocator` resource; IDs are never reused;
loading resolves stable IDs to fresh entities in a remap pass. This closes CS1's unversioned
slot-reuse hazard with the smallest possible commitment — save format, column serde and
versioning stay deferred to their own ticket.
