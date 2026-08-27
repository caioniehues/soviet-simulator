//! Stamps build provenance into the binary so a capture record can say which build produced it.
//!
//! Uses no dependencies on purpose: adding one here would change `Cargo.lock`, which the
//! dependency policy checks. Every value degrades to "unknown" rather than failing the build.

use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn main() {
    // Rebuild when the checked-out commit changes, so the stamp cannot go stale silently.
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/index");

    let commit = git(&["rev-parse", "--short=10", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    // An empty status means a clean tree. A failed git call is reported as unknown-dirty=true,
    // because claiming "clean" without evidence is the one answer that could mislead a reader.
    let dirty = git(&["status", "--porcelain"]).map_or(true, |s| !s.trim().is_empty());

    let rustc = Command::new(std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into()))
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=SOV_GIT_COMMIT={commit}");
    println!("cargo:rustc-env=SOV_GIT_DIRTY={dirty}");
    println!("cargo:rustc-env=SOV_RUSTC={rustc}");
    println!(
        "cargo:rustc-env=SOV_TARGET={}",
        std::env::var("TARGET").unwrap_or_else(|_| "unknown".into())
    );
    println!(
        "cargo:rustc-env=SOV_PROFILE={}",
        std::env::var("PROFILE").unwrap_or_else(|_| "unknown".into())
    );
}
