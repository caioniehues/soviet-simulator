//! Same-machine exact-hash golden gate for the fixed capture (sov-am1).
//!
//! Re-runs the pinned `baseline` capture and requires the fresh PNG to be
//! byte-identical to the committed golden at `engine_demo/golden/baseline.png`.
//! Byte equality implies hash equality, so no hashing dependency is needed.
//! The `image` crate (via `engine::image`, already a dependency) decodes both
//! sides to check dimensions and to describe a mismatch.
//!
//! Same-machine scope only: GPU model, driver, and build must match the
//! machine that produced the golden. There is no perceptual comparison.
//!
//! To regenerate the golden after an intentional visual change:
//! `cargo run -p engine_demo -- capture --scene baseline --out <dir>`
//! then copy `<dir>/baseline.png` over the committed golden.

use std::path::PathBuf;
use std::process::Command;

/// The committed golden. Embedded so the gate also works from a clean checkout
/// with no prior capture output on disk.
static GOLDEN_PNG: &[u8] = include_bytes!("../golden/baseline.png");

/// Scene the gate pins. Must stay `baseline`: it is the only scene whose
/// contract the capture code fixes.
const SCENE: &str = "baseline";
const EXPECTED_WIDTH: u32 = 1280;
const EXPECTED_HEIGHT: u32 = 720;
/// Observed md5 of the golden on RADV NAVI32, recorded 2026-09-03 from two
/// consecutive live runs. Informational only; the gate compares bytes.
const GOLDEN_MD5: &str = "1820c766ea67d03d0f2054aecbd4ac3e";

/// Offset of the first byte where `a` and `b` differ, or `None` if one is a
/// prefix of the other (reported via the length assert instead).
fn first_diff_offset(a: &[u8], b: &[u8]) -> Option<usize> {
    a.iter()
        .zip(b.iter())
        .position(|(x, y)| x != y)
}

#[test]
fn baseline_capture_matches_committed_golden_byte_for_byte() {
    // The golden itself must be a valid 1280x720 PNG; otherwise the gate
    // compares against garbage and a failure message blames the wrong side.
    let golden = engine::image::load_from_memory(GOLDEN_PNG).expect("committed golden must decode");
    assert_eq!(golden.width(), EXPECTED_WIDTH, "golden width");
    assert_eq!(golden.height(), EXPECTED_HEIGHT, "golden height");
    assert_eq!(
        GOLDEN_MD5.len(),
        32,
        "recorded md5 documents the golden; the gate itself compares bytes"
    );

    // Fresh directory per run: parallel `cargo test` invocations must not share it.
    let out_dir: PathBuf = std::env::temp_dir().join(format!(
        "sov-am1-golden-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("wall clock must be readable for a temp name")
            .as_nanos()
    ));
    std::fs::create_dir_all(&out_dir).expect("temp out dir must be creatable");

    // Re-run the fixed capture through the real binary, not a library
    // shortcut, so the gate covers argument parsing and file writing too.
    // `CARGO_BIN_EXE_<name>` is set for integration tests of binary targets.
    let bin = env!("CARGO_BIN_EXE_engine_demo");
    // Cargo runs integration tests with CWD set to the package dir, but the
    // binary resolves assets/ relative to the workspace root (as `cargo run`
    // does). Without this the capture dies on assets/shaders/mipmap.wgsl.
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("engine_demo must live one level below the workspace root");
    let run = Command::new(bin)
        .current_dir(workspace_root)
        .args(["capture", "--scene", SCENE, "--out"])
        .arg(&out_dir)
        .output()
        .expect("capture binary must run");
    assert!(
        run.status.success(),
        "capture run failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let fresh_path = out_dir.join(format!("{SCENE}.png"));
    let fresh = std::fs::read(&fresh_path).expect("fresh capture PNG must exist");

    let mismatch = fresh.len() != GOLDEN_PNG.len() || first_diff_offset(&fresh, GOLDEN_PNG).is_some();
    if mismatch {
        let detail = match engine::image::load_from_memory(&fresh) {
            Ok(img) => format!(
                "fresh PNG decodes as {}x{} (golden is {EXPECTED_WIDTH}x{EXPECTED_HEIGHT})",
                img.width(),
                img.height()
            ),
            Err(e) => format!("fresh PNG does not even decode: {e}"),
        };
        let _ = std::fs::remove_dir_all(&out_dir);
        panic!(
            "golden mismatch: fresh capture ({} bytes) != golden ({} bytes, md5 {GOLDEN_MD5}); \
             first differing byte: {:?}; {detail}; fresh file was at {}",
            fresh.len(),
            GOLDEN_PNG.len(),
            first_diff_offset(&fresh, GOLDEN_PNG),
            fresh_path.display(),
        );
    }

    let _ = std::fs::remove_dir_all(&out_dir);
}
