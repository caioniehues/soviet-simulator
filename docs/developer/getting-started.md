# Getting started

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Prerequisites

- Rust toolchain (edition 2021 workspace; `rustc` current stable).
- Git LFS for assets.
- Linux packages (Debian/Ubuntu): `libasound2-dev libudev-dev pkg-config libx11-dev`. On Arch-family
  systems the equivalents are `alsa-lib`, `systemd-libs`, `pkgconf`, `libx11`.
- `mdbook`, `mdbook-pagetoc`, and `mdbook-mermaid` if you want to render the docs.

## Build and run the game

```bash
git lfs pull
cargo run --release          # default member is native_app
```

`--release` is not optional — a debug build is unplayably slow. Never build inside `/tmp`: it is
a 16 GB tmpfs and a build there has filled it and killed a session. Use a worktree under `~/`.

## Run the simulation tests

```bash
cargo test -p simulation     # parallel-safe since 2026-08-26
```

43 scenario tests plus the serialisation round-trip test. Confirm a filter runs at least one test:

```bash
cargo test -p simulation scenario_0151 -- --nocapture
```

## Run headless

```bash
cargo run -p headless --release
```

`headless` ticks a `Simulation` without a renderer and is the multiplayer server binary; it is also
the seed for any future benchmark harness.

## Check dependencies and docs

```bash
cargo install cargo-deny --version 0.20.2 --locked
cargo-deny check

cargo install mdbook --version 0.5.4 --locked
cargo install mdbook-pagetoc --version 0.2.3 --locked
cargo install mdbook-mermaid --version 0.17.1 --locked

python3 scripts/check_docs.py
mdbook build
```

## Task state

```bash
bd ready          # unblocked work, ranked
bd show <id>      # goal and traps — read the description
```

`bd` is the only task-state authority; `CLAUDE.md` §Task tracking has the worker protocol. Do not
commit, push or `bd dolt push` without explicit authority from the user.

## First things to read

The [charter](../plan/charter-1.0.md), the [glossary](../reference/glossary.md), the
[current substrate](../architecture/current-substrate.md), then the
[development cycle](../process/development-cycle.md).

## Related

- [Repository tour](repository-tour.md)
- [How to read the docs](how-to-read-the-docs.md)
- [Testing standard](../engineering/testing.md)
