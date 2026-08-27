//! The 250k-identity benchmark contract.
//!
//! This module defines *what is measured*, independent of any benchmark framework.
//! `benches/scale_250k.rs` drives it with a plain `main()`. A framework trial
//! (see `sov-1jt`) must call into this module and must not define its own scenario.
//!
//! # What this measures
//!
//! CPU simulation cost of `Simulation::tick` with N persistent citizen identities
//! alive, on a road grid, created through the production `WorldCommand` seam.
//!
//! # What this does NOT measure
//!
//! Frame rate, GPU cost, rendering, or anything in `engine/` or `native_app/`.
//! CPU simulation time and frame rate are different quantities. See
//! `docs/reference/benchmark-contract.md`.

#![allow(clippy::needless_range_loop)]

use std::fmt::Write as _;
use std::time::{Duration, Instant};

use geom::{vec2, Vec2, OBB};
use prototypes::BuildingGen;
use simulation::map::{BuildingKind, RoadID};
use simulation::utils::scheduler::SeqSchedule;
use simulation::world_command::{WorldCommand, WorldCommands};
use simulation::{Simulation, SimulationOptions};

// ---------------------------------------------------------------------------
// Contract constants. Changing any of these changes the contract; bump
// SCHEMA_VERSION and say so in docs/reference/benchmark-contract.md.
// ---------------------------------------------------------------------------

pub const SCHEMA: &str = "sov.bench.scale/v1";
pub const BENCH_NAME: &str = "bench_scale_citizens";

/// The label that must travel with every number this runner produces.
pub const EVIDENCE_KIND: &str = "CPU simulation evidence";

/// Charter target scale: 250,000 citizen identities.
pub const DEFAULT_CITIZENS: usize = 250_000;
/// Ticks run and discarded before measurement starts.
pub const DEFAULT_WARMUP_TICKS: u32 = 20;
/// Ticks measured, one sample each.
pub const DEFAULT_MEASURED_TICKS: u32 = 100;
/// Seed handed to `SimulationOptions`. Fixed: the contract is reproducible or it is nothing.
pub const SEED: u64 = 123;
/// Terrain chunks per side. 50 is the production default (`Simulation::new(true)`).
pub const TERRAIN_SIZE: u16 = 50;
/// Metres between road-grid intersections.
pub const ROAD_SPACING: f32 = 300.0;
/// Metres between house centres along a road.
pub const HOUSE_PITCH: f32 = 26.0;
/// House footprint, metres.
pub const HOUSE_SIZE: f32 = 20.0;
/// Metres of each road left empty at either end, so that houses on
/// perpendicular roads do not overlap near an intersection.
pub const ROAD_END_INSET: f32 = 25.0;
/// Site list is this multiple of the target, because overlapping footprints are
/// refused and must be replaced.
pub const SITE_SLACK: f64 = 1.5;
/// Build commands applied per tick during population.
pub const BUILD_BATCH: usize = 20_000;

/// Which building mesh generator the houses use.
///
/// `BuildingGen` selects the *render* exterior only: `building.mesh` is read in
/// zero places under `simulation/src` and in exactly one place in the whole
/// repo, `native_app/src/rendering/map_rendering/map_mesh.rs:521`. The
/// simulation state machine is driven by `BuildingKind::House`, which is
/// identical in both modes.
///
/// `House` runs the procedural exterior generator, which reaches a defect that
/// aborts the process (see `docs/reference/benchmark-contract.md`), so it cannot
/// currently complete at contract scale. `CenteredDoor` skips it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshMode {
    /// Production `BuildingGen::House`: full procedural exterior.
    ProceduralHouse,
    /// `BuildingGen::CenteredDoor`: no procedural exterior mesh.
    NoExteriorMesh,
}

impl MeshMode {
    pub fn as_str(self) -> &'static str {
        match self {
            MeshMode::ProceduralHouse => "procedural_house",
            MeshMode::NoExteriorMesh => "no_exterior_mesh",
        }
    }

    fn gen(self) -> BuildingGen {
        match self {
            MeshMode::ProceduralHouse => BuildingGen::House,
            MeshMode::NoExteriorMesh => BuildingGen::CenteredDoor {
                vertical_factor: 1.0,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub citizens: usize,
    pub warmup_ticks: u32,
    pub measured_ticks: u32,
    pub repeats: u32,
    pub mesh: MeshMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            citizens: DEFAULT_CITIZENS,
            warmup_ticks: DEFAULT_WARMUP_TICKS,
            measured_ticks: DEFAULT_MEASURED_TICKS,
            repeats: 1,
            // Default is the mode that can actually complete at contract scale.
            // Switching to procedural meshes is opt-in precisely because it
            // currently aborts; see the defect note in the contract document.
            mesh: MeshMode::NoExteriorMesh,
        }
    }
}

impl Config {
    /// Reads overrides from the environment. Every override is reported in the
    /// result schema, so a non-default run can never be mistaken for a contract run.
    pub fn from_env() -> Self {
        let mut c = Self::default();
        if let Some(v) = env_usize("SOV_BENCH_CITIZENS") {
            c.citizens = v;
        }
        if let Some(v) = env_usize("SOV_BENCH_WARMUP_TICKS") {
            c.warmup_ticks = v as u32;
        }
        if let Some(v) = env_usize("SOV_BENCH_MEASURED_TICKS") {
            c.measured_ticks = v as u32;
        }
        if let Some(v) = env_usize("SOV_BENCH_REPEATS") {
            c.repeats = v.max(1) as u32;
        }
        if std::env::var("SOV_BENCH_PROCEDURAL_MESH").is_ok() {
            c.mesh = MeshMode::ProceduralHouse;
        }
        c
    }

    pub fn is_contract_scale(&self) -> bool {
        self.citizens == DEFAULT_CITIZENS
            && self.warmup_ticks == DEFAULT_WARMUP_TICKS
            && self.measured_ticks == DEFAULT_MEASURED_TICKS
    }
}

fn env_usize(key: &str) -> Option<usize> {
    std::env::var(key).ok()?.trim().parse().ok()
}

// ---------------------------------------------------------------------------
// One-time process setup
// ---------------------------------------------------------------------------

/// `simulation::init::init()` loads Lua prototypes from `<cwd>/base_mod/`.
/// Cargo runs bench binaries with the *package* directory as cwd, so the
/// workspace root has to be selected explicitly. Without this the runner panics
/// in `load_prototypes` rather than measuring anything.
pub fn enter_workspace_root() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("simulation/ must have a parent")
        .to_path_buf();
    std::env::set_current_dir(&root)
        .unwrap_or_else(|e| panic!("could not enter workspace root {}: {e}", root.display()));
    assert!(
        root.join("base_mod/data.lua").is_file(),
        "base_mod/data.lua not found under {} - the prototype seam moved",
        root.display()
    );
}

static INIT: std::sync::Once = std::sync::Once::new();

pub fn init_once() {
    INIT.call_once(|| {
        enter_workspace_root();
        simulation::init::init();
    });
}

// ---------------------------------------------------------------------------
// Scenario construction - production seams only
// ---------------------------------------------------------------------------

/// Everything the runner produced for one scenario instance.
pub struct Scenario {
    pub sim: Simulation,
    pub sched: SeqSchedule,
    pub citizens: usize,
    pub houses: usize,
    pub buildings: usize,
    pub roads: usize,
    pub setup_terrain: Duration,
    pub setup_roads: Duration,
    pub setup_buildings: Duration,
    pub setup_populate: Duration,
    pub populate_ticks: u32,
}

/// Side length of the road grid needed to seat `citizens` houses, with slack.
/// Slack matters: `build_special_building` refuses a house that overlaps an
/// existing one, so the site list must be longer than the target.
fn grid_side_for(citizens: usize) -> u32 {
    let usable = ROAD_SPACING - 2.0 * ROAD_END_INSET;
    let per_road = 2 * (usable / HOUSE_PITCH).max(1.0) as usize;
    let needed_roads = (citizens as f64 * SITE_SLACK) as usize / per_road.max(1) + 1;
    let side = ((needed_roads as f64 / 2.0).sqrt().ceil() as u32) + 2;
    side.max(3)
}

/// Builds the scenario: terrain, road grid, `citizens` houses, and the citizens
/// themselves. Every mutation goes through `WorldCommand` applied by
/// `Simulation::tick`, which is the same path the running game and the
/// multiplayer server use. Nothing here reaches into private map state.
pub fn build_scenario(cfg: &Config) -> Scenario {
    init_once();

    let t_terrain = Instant::now();
    let mut sim = Simulation::new_with_options(SimulationOptions {
        terrain_size: TERRAIN_SIZE,
        // Replay recording clones every command into a Vec. At 250k build
        // commands that is the dominant cost and is not part of the contract.
        save_replay: false,
        seed: SEED,
    });
    let mut sched = Simulation::schedule();
    let setup_terrain = t_terrain.elapsed();
    progress("terrain", setup_terrain);

    // Centre the road grid inside the generated terrain.
    let terrain_span = TERRAIN_SIZE as f32 * 512.0;
    let centre = vec2(terrain_span * 0.5, terrain_span * 0.5);

    let side = grid_side_for(cfg.citizens);

    let t_roads = Instant::now();
    tick_with(
        &mut sim,
        &mut sched,
        vec![WorldCommand::MapLoadTestField {
            pos: centre,
            size: side,
            spacing: ROAD_SPACING,
        }],
    );
    let setup_roads = t_roads.elapsed();
    progress("roads", t_roads.elapsed());

    let sites = plan_house_sites(&sim, (cfg.citizens as f64 * SITE_SLACK) as usize);
    assert!(
        sites.len() > cfg.citizens,
        "road grid offers {} sites for {} citizens; grid_side_for is wrong",
        sites.len(),
        cfg.citizens
    );

    // `build_special_building` refuses overlapping footprints, so the number of
    // houses that actually appear is less than the number of commands sent.
    // Keep feeding sites until the house count is exactly the target: the
    // contract is 250000 identities, not "about 250000".
    let t_build = Instant::now();
    let mut next = 0usize;
    let mut rejected = 0usize;
    loop {
        let have = count_houses(&sim);
        if have >= cfg.citizens {
            break;
        }
        let want = cfg.citizens - have;
        assert!(
            next < sites.len(),
            "ran out of house sites: {have}/{} placed, {rejected} refused as overlapping",
            cfg.citizens
        );
        let end = (next + want.min(BUILD_BATCH)).min(sites.len());
        let cmds: Vec<WorldCommand> = sites[next..end]
            .iter()
            .map(|&(pos, dir, road)| WorldCommand::MapBuildSpecialBuilding {
                pos: OBB::new(pos, dir, HOUSE_SIZE, HOUSE_SIZE),
                kind: BuildingKind::House,
                gen: cfg.mesh.gen(),
                zone: None,
                connected_road: Some(road),
            })
            .collect();
        let sent = cmds.len();
        tick_with(&mut sim, &mut sched, cmds);
        rejected += sent - (count_houses(&sim) - have);
        next = end;
    }
    let setup_buildings = t_build.elapsed();
    progress("buildings", setup_buildings);
    let houses = count_houses(&sim);
    assert_eq!(
        houses, cfg.citizens,
        "house count must match the contract exactly"
    );

    // `add_souls_to_empty_buildings` is a registered production system: it runs
    // once per tick and gives every ownerless house a human. Tick until the
    // population stops growing.
    let t_pop = Instant::now();
    let mut populate_ticks = 0;
    loop {
        let before = sim.world().humans.len();
        tick_with(&mut sim, &mut sched, Vec::new());
        populate_ticks += 1;
        let after = sim.world().humans.len();
        if after == before {
            break;
        }
        assert!(
            populate_ticks < 200,
            "population still growing after 200 ticks ({after} humans)"
        );
    }
    let setup_populate = t_pop.elapsed();
    progress("populate", setup_populate);

    let citizens = sim.world().humans.len();
    let buildings = sim.map().buildings().len();
    let roads_n = sim.map().roads().len();

    Scenario {
        sim,
        sched,
        citizens,
        houses,
        buildings,
        roads: roads_n,
        setup_terrain,
        setup_roads,
        setup_buildings,
        setup_populate,
        populate_ticks,
    }
}

/// Live phase marker with current RSS. A 250k run is long and memory-hungry;
/// a silent runner that gets OOM-killed tells you nothing about which phase did it.
fn progress(phase: &str, took: Duration) {
    eprintln!(
        "scale_250k: [{phase}] {:.1} s, rss {} MiB",
        took.as_secs_f64(),
        current_rss_kb() / 1024
    );
}

fn tick_with(sim: &mut Simulation, sched: &mut SeqSchedule, cmds: Vec<WorldCommand>) {
    let mut wc = WorldCommands::default();
    wc.extend(cmds);
    sim.tick(sched, wc.as_ref());
}

/// Deterministic house sites: walk every road in slotmap order, step along its
/// centreline, offset left and right of the carriageway.
fn plan_house_sites(sim: &Simulation, want: usize) -> Vec<(Vec2, Vec2, RoadID)> {
    let map = sim.map();
    let mut out = Vec::with_capacity(want.min(1 << 21));
    let mut roads: Vec<RoadID> = map.roads().keys().collect();
    roads.sort_unstable(); // slotmap order is already stable; be explicit about it

    for rid in roads {
        let Some(road) = map.roads().get(rid) else {
            continue;
        };
        let pts = road.points();
        let a = pts.first().xy();
        let b = pts.last().xy();
        let along = b - a;
        let len = along.mag();
        // Sites near an intersection collide with the perpendicular road's
        // sites, so the ends are left empty.
        let usable = len - 2.0 * ROAD_END_INSET;
        if usable < HOUSE_PITCH {
            continue;
        }
        let dir = along / len;
        let perp = dir.perpendicular();
        let offset = road.width * 0.5 + HOUSE_SIZE * 0.5 + 2.0;

        let n = (usable / HOUSE_PITCH) as usize;
        for i in 0..n {
            let base = a + dir * (ROAD_END_INSET + HOUSE_PITCH * (i as f32 + 0.5));
            for side in [1.0f32, -1.0] {
                out.push((base + perp * offset * side, dir, rid));
                if out.len() >= want {
                    return out;
                }
            }
        }
    }
    out
}

fn count_houses(sim: &Simulation) -> usize {
    sim.map()
        .buildings()
        .values()
        .filter(|b| b.kind == BuildingKind::House)
        .count()
}

// ---------------------------------------------------------------------------
// Measurement
// ---------------------------------------------------------------------------

pub struct Measurement {
    pub warmup: Duration,
    pub measured_total: Duration,
    pub tick_ns: Vec<u64>,
    pub first_measured_tick: u64,
    pub last_measured_tick: u64,
    /// `Simulation::hashes()` folded into one value. Two equivalent runs must
    /// produce the same digest.
    pub digest: u64,
    pub digest_components: Vec<(String, u64)>,
    /// Per-system average milliseconds over the scheduler's rolling 100-tick
    /// history, taken straight from `SeqSchedule::times()` - the same instrument
    /// the game's own timings readout uses. With `measured_ticks == 100` this
    /// window is exactly the measured window. This is the profile, not a guess.
    pub system_ms: Vec<(String, f32)>,
}

/// Runs warm-up then the measured window. `advance` is the exact unit of work
/// under test: one `Simulation::tick` with no commands.
pub fn measure(scn: &mut Scenario, cfg: &Config) -> Measurement {
    let t_warm = Instant::now();
    for _ in 0..cfg.warmup_ticks {
        tick_with(&mut scn.sim, &mut scn.sched, Vec::new());
    }
    let warmup = t_warm.elapsed();

    let first_measured_tick = scn.sim.get_tick() + 1;
    let mut tick_ns = Vec::with_capacity(cfg.measured_ticks as usize);
    let t_all = Instant::now();
    for _ in 0..cfg.measured_ticks {
        let t = Instant::now();
        tick_with(&mut scn.sim, &mut scn.sched, Vec::new());
        tick_ns.push(t.elapsed().as_nanos() as u64);
    }
    let measured_total = t_all.elapsed();
    let last_measured_tick = scn.sim.get_tick();

    let system_ms = scn.sched.times();
    let components: Vec<(String, u64)> = scn.sim.hashes().into_iter().collect();
    let digest = fold_digest(&components);

    Measurement {
        warmup,
        measured_total,
        tick_ns,
        first_measured_tick,
        last_measured_tick,
        digest,
        digest_components: components,
        system_ms,
    }
}

/// FNV-1a over the sorted (name, hash) pairs `Simulation::hashes()` returns.
/// `hashes()` yields a BTreeMap, so the order is already stable.
fn fold_digest(components: &[(String, u64)]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (name, v) in components {
        for b in name.as_bytes().iter().chain(v.to_le_bytes().iter()) {
            h ^= *b as u64;
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    }
    h
}

// ---------------------------------------------------------------------------
// Statistics
// ---------------------------------------------------------------------------

pub struct Stats {
    pub count: usize,
    pub min: u64,
    pub p50: u64,
    pub p90: u64,
    pub p99: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    /// (p90 - p10) / p50, as a percentage. The spread that decides whether a
    /// difference between two runs is a finding or noise.
    pub spread_pct: f64,
}

pub fn stats(samples: &[u64]) -> Stats {
    assert!(!samples.is_empty(), "no samples");
    let mut s = samples.to_vec();
    s.sort_unstable();
    let pick = |q: f64| s[(((s.len() - 1) as f64) * q).round() as usize];
    let mean = s.iter().map(|&x| x as f64).sum::<f64>() / s.len() as f64;
    let var = s.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / s.len() as f64;
    let p50 = pick(0.50);
    let p10 = pick(0.10);
    let p90 = pick(0.90);
    Stats {
        count: s.len(),
        min: s[0],
        p50,
        p90,
        p99: pick(0.99),
        max: s[s.len() - 1],
        mean,
        stddev: var.sqrt(),
        spread_pct: if p50 == 0 {
            0.0
        } else {
            (p90 as f64 - p10 as f64) / p50 as f64 * 100.0
        },
    }
}

// ---------------------------------------------------------------------------
// Host, build and provenance
// ---------------------------------------------------------------------------

pub struct HostInfo {
    pub hostname: String,
    pub kernel: String,
    pub cpu_model: String,
    pub logical_cpus: usize,
    pub mem_total_kb: u64,
    pub mem_available_kb: u64,
    pub load_average: String,
    pub cpu_governor: String,
    pub rayon_threads: usize,
}

pub fn host_info() -> HostInfo {
    HostInfo {
        hostname: read_trim("/proc/sys/kernel/hostname"),
        kernel: read_trim("/proc/sys/kernel/osrelease"),
        cpu_model: proc_cpuinfo_field("model name"),
        logical_cpus: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(0),
        mem_total_kb: meminfo_kb("MemTotal"),
        mem_available_kb: meminfo_kb("MemAvailable"),
        load_average: read_trim("/proc/loadavg"),
        cpu_governor: read_trim("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"),
        rayon_threads: rayon::current_num_threads(),
    }
}

pub struct BuildInfo {
    /// Derived from the bench executable's own path, so it cannot disagree with
    /// the binary that produced the numbers.
    pub profile: String,
    pub debug_assertions: bool,
    pub rustc: String,
    pub git_sha: String,
    pub git_dirty: bool,
    pub target_arch: String,
    pub target_os: String,
}

pub fn build_info() -> BuildInfo {
    let exe = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let profile = if exe.contains("/release/") {
        "release"
    } else if exe.contains("/debug/") {
        "debug"
    } else {
        "unknown"
    };
    BuildInfo {
        profile: profile.to_string(),
        debug_assertions: cfg!(debug_assertions),
        rustc: cmd("rustc", &["--version"]),
        git_sha: cmd("git", &["rev-parse", "HEAD"]),
        git_dirty: !cmd("git", &["status", "--porcelain"]).is_empty(),
        target_arch: std::env::consts::ARCH.to_string(),
        target_os: std::env::consts::OS.to_string(),
    }
}

fn read_trim(path: &str) -> String {
    std::fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| String::from("unknown"))
}

fn proc_cpuinfo_field(field: &str) -> String {
    std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with(field))
                .and_then(|l| l.split_once(':'))
                .map(|(_, v)| v.trim().to_string())
        })
        .unwrap_or_else(|| String::from("unknown"))
}

fn meminfo_kb(field: &str) -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with(field))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(0)
}

fn cmd(prog: &str, args: &[&str]) -> String {
    std::process::Command::new(prog)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Current resident set of this process, in kB.
pub fn current_rss_kb() -> u64 {
    proc_status_kb("VmRSS")
}

fn proc_status_kb(field: &str) -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with(field))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(0)
}

/// Peak resident set of this process, in kB. Reported because a 250k-identity
/// run is as much a memory result as a time result.
pub fn peak_rss_kb() -> u64 {
    proc_status_kb("VmHWM")
}

// ---------------------------------------------------------------------------
// Result schema
// ---------------------------------------------------------------------------

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', " ")
}

/// Emits the stable machine-readable result. Hand-rolled JSON: adding a
/// serialiser dependency to measure the simulation would change what is being
/// measured's dependency graph for no gain.
#[allow(clippy::too_many_arguments)]
pub fn to_json(
    cfg: &Config,
    scn: &Scenario,
    m: &Measurement,
    st: &Stats,
    host: &HostInfo,
    build: &BuildInfo,
    repeat_digests: &[u64],
) -> String {
    let mut o = String::with_capacity(8192);
    let _ = write!(o, "{{");
    let _ = write!(o, "\"schema\":\"{SCHEMA}\",");
    let _ = write!(o, "\"bench\":\"{BENCH_NAME}\",");
    let _ = write!(o, "\"evidence_kind\":\"{EVIDENCE_KIND}\",");
    let _ = write!(
        o,
        "\"not_evidence_of\":[\"frame rate\",\"fps\",\"GPU cost\",\"render cost\",\"end-to-end game loop\"],"
    );
    let _ = write!(o, "\"contract_scale\":{},", cfg.is_contract_scale());

    let _ = write!(o, "\"run\":{{");
    let _ = write!(o, "\"seed\":{SEED},");
    let _ = write!(o, "\"requested_citizens\":{},", cfg.citizens);
    let _ = write!(o, "\"actual_citizens\":{},", scn.citizens);
    let _ = write!(o, "\"buildings\":{},", scn.buildings);
    let _ = write!(o, "\"houses\":{},", scn.houses);
    let _ = write!(o, "\"roads\":{},", scn.roads);
    let _ = write!(o, "\"warmup_ticks\":{},", cfg.warmup_ticks);
    let _ = write!(o, "\"measured_ticks\":{},", cfg.measured_ticks);
    let _ = write!(o, "\"populate_ticks\":{},", scn.populate_ticks);
    let _ = write!(o, "\"building_mesh_mode\":\"{}\",", cfg.mesh.as_str());
    let _ = write!(o, "\"repeats\":{},", cfg.repeats);
    let _ = write!(
        o,
        "\"tick_range\":{{\"first_measured\":{},\"last_measured\":{}}}",
        m.first_measured_tick, m.last_measured_tick
    );
    let _ = write!(o, "}},");

    let _ = write!(o, "\"build\":{{");
    let _ = write!(o, "\"profile\":\"{}\",", esc(&build.profile));
    let _ = write!(o, "\"debug_assertions\":{},", build.debug_assertions);
    let _ = write!(o, "\"rustc\":\"{}\",", esc(&build.rustc));
    let _ = write!(o, "\"git_sha\":\"{}\",", esc(&build.git_sha));
    let _ = write!(o, "\"git_dirty\":{},", build.git_dirty);
    let _ = write!(o, "\"target_arch\":\"{}\",", esc(&build.target_arch));
    let _ = write!(o, "\"target_os\":\"{}\"", esc(&build.target_os));
    let _ = write!(o, "}},");

    let _ = write!(o, "\"host\":{{");
    let _ = write!(o, "\"hostname\":\"{}\",", esc(&host.hostname));
    let _ = write!(o, "\"kernel\":\"{}\",", esc(&host.kernel));
    let _ = write!(o, "\"cpu_model\":\"{}\",", esc(&host.cpu_model));
    let _ = write!(o, "\"logical_cpus\":{},", host.logical_cpus);
    let _ = write!(o, "\"rayon_threads\":{},", host.rayon_threads);
    let _ = write!(o, "\"cpu_governor\":\"{}\",", esc(&host.cpu_governor));
    let _ = write!(o, "\"mem_total_kb\":{},", host.mem_total_kb);
    let _ = write!(
        o,
        "\"mem_available_kb_at_start\":{},",
        host.mem_available_kb
    );
    let _ = write!(o, "\"peak_rss_kb\":{},", peak_rss_kb());
    let _ = write!(o, "\"loadavg_at_start\":\"{}\"", esc(&host.load_average));
    let _ = write!(o, "}},");

    let _ = write!(o, "\"digest\":{{");
    let _ = write!(
        o,
        "\"algorithm\":\"fnv1a64 over Simulation::hashes() (bincode/JSON encode + common::hash_u64 per resource)\","
    );
    let _ = write!(o, "\"combined\":\"{:#018x}\",", m.digest);
    let _ = write!(o, "\"components\":{{");
    for (i, (k, v)) in m.digest_components.iter().enumerate() {
        if i > 0 {
            let _ = write!(o, ",");
        }
        let _ = write!(o, "\"{}\":\"{:#018x}\"", esc(k), v);
    }
    let _ = write!(o, "}},");
    let _ = write!(o, "\"repeat_combined\":[");
    for (i, d) in repeat_digests.iter().enumerate() {
        if i > 0 {
            let _ = write!(o, ",");
        }
        let _ = write!(o, "\"{d:#018x}\"");
    }
    let _ = write!(o, "],");
    let _ = write!(
        o,
        "\"repeats_all_equal\":{}",
        repeat_digests.windows(2).all(|w| w[0] == w[1])
    );
    let _ = write!(o, "}},");

    let _ = write!(o, "\"durations_ns\":{{");
    let _ = write!(o, "\"setup_terrain\":{},", scn.setup_terrain.as_nanos());
    let _ = write!(o, "\"setup_roads\":{},", scn.setup_roads.as_nanos());
    let _ = write!(o, "\"setup_buildings\":{},", scn.setup_buildings.as_nanos());
    let _ = write!(o, "\"setup_populate\":{},", scn.setup_populate.as_nanos());
    let _ = write!(o, "\"warmup\":{},", m.warmup.as_nanos());
    let _ = write!(o, "\"measured_total\":{}", m.measured_total.as_nanos());
    let _ = write!(o, "}},");

    let _ = write!(o, "\"system_ms\":{{");
    for (i, (k, v)) in m.system_ms.iter().enumerate() {
        if i > 0 {
            let _ = write!(o, ",");
        }
        let _ = write!(o, "\"{}\":{:.4}", esc(k), v);
    }
    let _ = write!(o, "}},");

    let _ = write!(o, "\"tick_ns\":{{");
    let _ = write!(o, "\"count\":{},", st.count);
    let _ = write!(o, "\"min\":{},", st.min);
    let _ = write!(o, "\"p50\":{},", st.p50);
    let _ = write!(o, "\"p90\":{},", st.p90);
    let _ = write!(o, "\"p99\":{},", st.p99);
    let _ = write!(o, "\"max\":{},", st.max);
    let _ = write!(o, "\"mean\":{:.1},", st.mean);
    let _ = write!(o, "\"stddev\":{:.1},", st.stddev);
    let _ = write!(o, "\"spread_p10_p90_pct\":{:.2},", st.spread_pct);
    let _ = write!(o, "\"samples\":[");
    for (i, v) in m.tick_ns.iter().enumerate() {
        if i > 0 {
            let _ = write!(o, ",");
        }
        let _ = write!(o, "{v}");
    }
    let _ = write!(o, "]}}");

    let _ = write!(o, "}}");
    o
}

/// The human-readable summary. Leads with the evidence label on purpose.
pub fn to_summary(
    cfg: &Config,
    scn: &Scenario,
    m: &Measurement,
    st: &Stats,
    host: &HostInfo,
    build: &BuildInfo,
) -> String {
    let mut o = String::new();
    let ms = |ns: u64| ns as f64 / 1e6;
    let _ = writeln!(o, "=== {BENCH_NAME} :: {EVIDENCE_KIND} ===");
    let _ = writeln!(
        o,
        "NOT frame-rate evidence. NOT GPU evidence. Simulation::tick CPU time only."
    );
    let _ = writeln!(
        o,
        "scale      : {} citizens, {} buildings, {} roads (requested {})",
        scn.citizens, scn.buildings, scn.roads, cfg.citizens
    );
    let _ = writeln!(
        o,
        "mesh mode  : {} (render-only; does not affect simulation state)",
        cfg.mesh.as_str()
    );
    let _ = writeln!(
        o,
        "build      : {} (debug_assertions={}) {}{}",
        build.profile,
        build.debug_assertions,
        &build.git_sha.chars().take(12).collect::<String>(),
        if build.git_dirty { " DIRTY" } else { "" }
    );
    let _ = writeln!(
        o,
        "host       : {} / {} x{} threads / governor {} / loadavg {}",
        host.cpu_model, host.hostname, host.rayon_threads, host.cpu_governor, host.load_average
    );
    let _ = writeln!(
        o,
        "setup      : terrain {:.0} ms, roads {:.0} ms, buildings {:.0} ms, populate {:.0} ms ({} ticks)",
        scn.setup_terrain.as_secs_f64() * 1e3,
        scn.setup_roads.as_secs_f64() * 1e3,
        scn.setup_buildings.as_secs_f64() * 1e3,
        scn.setup_populate.as_secs_f64() * 1e3,
        scn.populate_ticks
    );
    let _ = writeln!(
        o,
        "window     : warmup {} ticks ({:.0} ms), measured ticks {}..={} (n={})",
        cfg.warmup_ticks,
        m.warmup.as_secs_f64() * 1e3,
        m.first_measured_tick,
        m.last_measured_tick,
        st.count
    );
    let _ = writeln!(
        o,
        "tick ms    : p50 {:.3}  p90 {:.3}  p99 {:.3}  min {:.3}  max {:.3}  mean {:.3}  sd {:.3}",
        ms(st.p50),
        ms(st.p90),
        ms(st.p99),
        ms(st.min),
        ms(st.max),
        st.mean / 1e6,
        st.stddev / 1e6
    );
    let _ = writeln!(
        o,
        "spread     : p10-p90 = {:.1}% of p50  (a difference smaller than this is noise)",
        st.spread_pct
    );
    let _ = writeln!(o, "peak rss   : {} MiB", peak_rss_kb() / 1024);
    let _ = writeln!(o, "digest     : {:#018x}", m.digest);
    let total: f32 = m.system_ms.iter().map(|(_, v)| *v).sum();
    let _ = writeln!(
        o,
        "per-system average ms over the last 100 ticks (SeqSchedule::times):"
    );
    for (name, v) in m.system_ms.iter().take(8) {
        let _ = writeln!(
            o,
            "  {:<28} {:>9.3} ms  {:>5.1}%",
            name,
            v,
            if total > 0.0 { v / total * 100.0 } else { 0.0 }
        );
    }
    let _ = writeln!(o, "  {:<28} {:>9.3} ms  (all systems)", "TOTAL", total);
    o
}
