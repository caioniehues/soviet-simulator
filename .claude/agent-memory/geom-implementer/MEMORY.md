# Memory Index

- [Skeleton LAV traps](project_skeleton-lav-traps.md) — bound LAV walks by vs.len()+1 never LAV::len; absolute EPSILON makes small footprints fail more
- [Procgen house init trap](project_procgen-house-init-trap.md) — gen_exterior_house SIGSEGVs in release without crate::init::init(); test cwd must be simulation/
- [Memory-capped runs](feedback_memory-capped-runs.md) — never run a memory-growth repro uncapped; the cgroup ceiling IS the evidence
