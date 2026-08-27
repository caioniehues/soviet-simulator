//! `bench_scale_citizens` - the 250,000-identity CPU simulation contract.
//!
//! Finite by construction: it builds a scenario, measures a fixed number of
//! ticks, prints its result, and exits. It is NOT the headless server
//! (`headless/src/main.rs`), which loops forever and cannot be a benchmark.
//!
//! Run:
//!   cargo bench -p simulation --bench scale_250k
//!
//! Smaller, for a smoke check (the result schema records that it was not
//! contract scale):
//!   SOV_BENCH_CITIZENS=2000 SOV_BENCH_MEASURED_TICKS=30 \
//!     cargo bench -p simulation --bench scale_250k
//!
//! Environment:
//!   SOV_BENCH_CITIZENS        default 250000
//!   SOV_BENCH_WARMUP_TICKS    default 20
//!   SOV_BENCH_MEASURED_TICKS  default 100
//!   SOV_BENCH_REPEATS         default 1; >1 rebuilds the scenario from scratch
//!                             and asserts every run yields the same digest
//!   SOV_BENCH_JSON_OUT        path to write the result schema to (also stdout)
//!
//! The number this prints is CPU simulation evidence. It is not a frame rate
//! and it says nothing about GPU cost.

mod contract;

use contract::{
    build_info, build_scenario, host_info, measure, stats, to_json, to_summary, Config,
};

fn main() {
    // Criterion and libtest both pass flags we do not understand; refuse
    // silently-wrong behaviour rather than ignoring them.
    for a in std::env::args().skip(1) {
        if a == "--bench" || a == "--test" {
            continue;
        }
        eprintln!("scale_250k: unrecognised argument {a:?}; this bench takes no flags, only SOV_BENCH_* env vars");
        std::process::exit(2);
    }

    let cfg = Config::from_env();
    let host = host_info();
    let build = build_info();

    if build.profile != "release" {
        eprintln!(
            "scale_250k: WARNING - running a {} build. Timing numbers from a non-release \
             build are meaningless for this contract. Use `cargo bench`.",
            build.profile
        );
    }

    let mut repeat_digests = Vec::new();
    let mut last = None;

    for r in 0..cfg.repeats {
        eprintln!(
            "scale_250k: building scenario ({} citizens) run {}/{} ...",
            cfg.citizens,
            r + 1,
            cfg.repeats
        );
        let mut scn = build_scenario(&cfg);
        let m = measure(&mut scn, &cfg);
        repeat_digests.push(m.digest);
        let st = stats(&m.tick_ns);
        eprint!("{}", to_summary(&cfg, &scn, &m, &st, &host, &build));
        last = Some((scn, m, st));
    }

    let (scn, m, st) = last.expect("repeats >= 1");

    let all_equal = repeat_digests.windows(2).all(|w| w[0] == w[1]);
    if cfg.repeats > 1 {
        eprintln!(
            "scale_250k: digest relationship across {} equivalent runs: {}",
            cfg.repeats,
            if all_equal {
                "EQUAL (required)"
            } else {
                "DIFFERENT (contract violated)"
            }
        );
    }

    let json = to_json(&cfg, &scn, &m, &st, &host, &build, &repeat_digests);
    if let Ok(path) = std::env::var("SOV_BENCH_JSON_OUT") {
        if let Err(e) = std::fs::write(&path, &json) {
            eprintln!("scale_250k: could not write {path}: {e}");
        } else {
            eprintln!("scale_250k: wrote {path}");
        }
    }
    println!("{json}");

    if !all_equal {
        eprintln!("scale_250k: FAIL - equivalent runs produced different state digests");
        std::process::exit(1);
    }
}
