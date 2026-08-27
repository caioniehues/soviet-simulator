---
name: licence-inventory
description: The 2026-08-27 third-party licence inventory (339 crates, 14 identifiers) and the two licence facts that needed source verification
metadata:
  type: project
---

Inventory command: `cargo-deny list -f json` from repo root, pinned 0.20.2.
Recorded 2026-08-27 against an unmodified `Cargo.lock` (no `criterion` yet).

**339 third-party crates, exactly 14 distinct licence identifiers** = the 13 in
the `deny.toml` `allow` list plus `LicenseRef-UFL-1.0`, which is covered by one
narrow `epaint` exception. Counts by identifier: MIT 344, Apache-2.0 272,
Zlib 14, Unlicense 7, BSD-3-Clause 6, CC0-1.0 4, Apache-2.0 WITH LLVM-exception 3,
BSD-2-Clause 3, ISC 2, 0BSD 1, BSL-1.0 1, OFL-1.1 1, Unicode-DFS-2016 1,
LicenseRef-UFL-1.0 1. They sum above 339 because `MIT OR Apache-2.0` counts twice.

**Why:** the repo is GPL-3.0 by inheritance and permanently so; nothing may ship
without the dependency licences recorded. No copyleft-incompatible identifier is
present in the current graph.

**How to apply:**

- **`epaint` is the only awkward one.** It declares
  `(MIT OR Apache-2.0) AND OFL-1.1 AND LicenseRef-UFL-1.0` -
  `~/.cargo/git/checkouts/egui-*/<rev>/crates/epaint/Cargo.toml:9`. The OFL/UFL
  branch covers its bundled `default_fonts`. Upstream's own `deny.toml:84`
  allows the same non-SPDX identifier. Do not re-litigate this; verify from
  those two lines if challenged.
- **The 13 workspace members are not in the inventory except two.**
  `licenses.private.ignore = true` excludes them. `egui-inspect` 0.4.0 and
  `egui-inspect-derive` 0.4.1 still surface, because they declare
  `MIT or Apache-2.0` inherited from the crates they were forked from. The other
  11 declare no `license` at all. Per-crate manifest metadata is NOT the
  repository's licence statement - `LICENSE` (GPL-3.0) is. That mismatch is a
  known cosmetic inconsistency, not a compliance finding.
- `cargo-deny` itself is `MIT OR Apache-2.0`, verified at
  `~/.cargo/registry/src/index.crates.io-*/cargo-deny-0.20.2/Cargo.toml:45`.
- **Still open for distribution:** the git dependencies `emilk/egui` (no branch,
  no rev) and `Uriopass/yakui` (branch `dev`) are unpinned in the manifests.
  `Cargo.lock` pins the resolved commits, which is a mitigation not a fix. Asset
  provenance under `assets/` is also unestablished. Neither is in
  `docs/process/dependency-policy.md` scope.

See [[dependency-policy-baseline]].
