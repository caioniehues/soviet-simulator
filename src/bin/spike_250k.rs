//! PROTOTYPE — ticket #4 "Quarter-Million Spike". Throwaway benchmark, not game code.
//!
//! Question: does Bevy 0.19's ECS hold the scale floor — 250k citizen-shaped
//! entities ticking under the six-band frequency layout (architecture/simulation-clock.md
//! carried from the Unity track)? Measures ms/tick per band, two sweep strategies:
//!
//!   A. naive modulo-scan  — every band system iterates all 250k entities and
//!      skips those whose `index % period != frame % period` (CS1-shaped, but
//!      CS1 walks flat arrays by index range; Bevy pays full query iteration).
//!   B. phase-bucketed     — entities pre-bucketed by `index % period` into
//!      Vec<Entity> lists; band system random-accesses its bucket via get_mut.
//!
//! Run: cargo run --release --bin spike_250k

use bevy::prelude::*;
use bevy::tasks::{ComputeTaskPool, TaskPool};
use std::time::Instant;

const CITIZENS: u32 = 250_000;
const FRAMES: u32 = 8_192; // two full Very-low sweeps
const FRAMES_PER_DAY: u32 = 600; // Calendar band edge

const BAND_NAMES: [&str; 6] = ["high(1)", "medium(16)", "low(256)", "verylow(4096)", "calendar(600)", "housekeep(1024)"];

// ---------- citizen-shaped data (~40B of live state) ----------

#[derive(Component)]
struct CitizenIndex(u32);

#[derive(Component)]
struct Pos(Vec3);

#[derive(Component)]
struct Vel(Vec3);

#[derive(Component)]
struct Needs([f32; 8]);

#[derive(Component)]
struct Demographics {
    age_frames: u32,
    education: u8,
    flags: u8,
    health: u8,
    wellbeing: u8,
}

#[derive(Component)]
struct Bindings {
    home: u32,
    work: u32,
    last_processed_frame: u32,
}

// ---------- bench plumbing ----------

#[derive(Resource, Default)]
struct Frame(u32);

#[derive(Resource, Default)]
struct BandTimes {
    // per band: (total_ms, invocations, max_ms, samples for p95)
    samples: [Vec<f64>; 6],
}

#[derive(Resource, Default)]
struct PlanStats(f64); // sink so calendar work isn't optimized out

/// Phase buckets for mode B: buckets[band][phase] = entities with index % period == phase.
#[derive(Resource, Default)]
struct Buckets {
    medium: Vec<Vec<Entity>>,
    low: Vec<Vec<Entity>>,
    verylow: Vec<Vec<Entity>>,
}

#[derive(Resource, Clone, Copy, PartialEq)]
enum Mode {
    NaiveScan,
    Bucketed,
}

fn spawn_citizens(world: &mut World) {
    let mut rng_state = 0x9E3779B9u32;
    let mut next = move || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 17;
        rng_state ^= rng_state << 5;
        rng_state
    };
    let batch: Vec<_> = (0..CITIZENS)
        .map(|i| {
            let r = next();
            (
                CitizenIndex(i),
                Pos(Vec3::new((r % 4096) as f32, 0.0, ((r >> 12) % 4096) as f32)),
                Vel(Vec3::new(0.1, 0.0, 0.05)),
                Needs([0.5; 8]),
                Demographics {
                    age_frames: r % 1_000_000,
                    education: (r % 4) as u8,
                    flags: 0,
                    health: 200,
                    wellbeing: 128,
                },
                Bindings { home: r % 50_000, work: (r >> 8) % 50_000, last_processed_frame: 0 },
            )
        })
        .collect();
    world.spawn_batch(batch);
}

fn build_buckets(world: &mut World) {
    let mut buckets = Buckets {
        medium: vec![Vec::new(); 16],
        low: vec![Vec::new(); 256],
        verylow: vec![Vec::new(); 4096],
    };
    let mut q = world.query::<(Entity, &CitizenIndex)>();
    for (e, idx) in q.iter(world) {
        buckets.medium[(idx.0 % 16) as usize].push(e);
        buckets.low[(idx.0 % 256) as usize].push(e);
        buckets.verylow[(idx.0 % 4096) as usize].push(e);
    }
    world.insert_resource(buckets);
}

// ---------- band workloads (same math in both modes) ----------

#[inline(always)]
fn medium_work(pos: &mut Pos, vel: &Vel, demo: &mut Demographics) {
    // pedestrian-ish: 16 frames of movement applied at once + flag churn
    pos.0 += vel.0 * 16.0;
    demo.flags = demo.flags.wrapping_add(1) & 0x0F;
}

#[inline(always)]
fn low_work(needs: &mut Needs, bindings: &Bindings, demo: &mut Demographics) {
    // service-availability-ish: touch needs against bindings
    let served = (bindings.home ^ bindings.work) & 7;
    needs.0[served as usize] = (needs.0[served as usize] + 0.01).min(1.0);
    demo.wellbeing = ((needs.0.iter().sum::<f32>() / 8.0) * 255.0) as u8;
}

#[inline(always)]
fn verylow_work(needs: &mut Needs, demo: &mut Demographics, bindings: &mut Bindings, frame: u32) {
    // needs decay integrated over elapsed frames + aging + education transition
    let elapsed = frame.wrapping_sub(bindings.last_processed_frame);
    let decay = elapsed as f32 * 0.00002;
    for n in needs.0.iter_mut() {
        *n = (*n - decay).max(0.0);
    }
    demo.age_frames = demo.age_frames.wrapping_add(elapsed);
    if demo.age_frames % 500_000 < elapsed {
        demo.education = (demo.education + 1).min(3);
    }
    demo.health = demo.health.saturating_sub((needs.0[0] < 0.1) as u8);
    bindings.last_processed_frame = frame;
}

// ---------- systems ----------

fn advance_frame(mut frame: ResMut<Frame>) {
    frame.0 += 1;
}

fn band_high(mut q: Query<(&mut Pos, &Vel)>, mut times: ResMut<BandTimes>) {
    let t = Instant::now();
    // vehicle/movement-shaped: integrate everyone, parallel
    q.par_iter_mut().for_each(|(mut pos, vel)| {
        pos.0 += vel.0;
    });
    times.samples[0].push(t.elapsed().as_secs_f64() * 1e3);
}

fn band_medium(
    mode: Res<Mode>,
    frame: Res<Frame>,
    buckets: Option<Res<Buckets>>,
    mut q: Query<(&CitizenIndex, &mut Pos, &Vel, &mut Demographics)>,
    mut times: ResMut<BandTimes>,
) {
    let t = Instant::now();
    let phase = frame.0 % 16;
    match *mode {
        Mode::NaiveScan => {
            for (idx, mut pos, vel, mut demo) in &mut q {
                if idx.0 % 16 == phase {
                    medium_work(&mut pos, vel, &mut demo);
                }
            }
        }
        Mode::Bucketed => {
            for &e in &buckets.as_ref().unwrap().medium[phase as usize] {
                if let Ok((_, mut pos, vel, mut demo)) = q.get_mut(e) {
                    medium_work(&mut pos, vel, &mut demo);
                }
            }
        }
    }
    times.samples[1].push(t.elapsed().as_secs_f64() * 1e3);
}

fn band_low(
    mode: Res<Mode>,
    frame: Res<Frame>,
    buckets: Option<Res<Buckets>>,
    mut q: Query<(&CitizenIndex, &mut Needs, &Bindings, &mut Demographics)>,
    mut times: ResMut<BandTimes>,
) {
    let t = Instant::now();
    let phase = frame.0 % 256;
    match *mode {
        Mode::NaiveScan => {
            for (idx, mut needs, bindings, mut demo) in &mut q {
                if idx.0 % 256 == phase {
                    low_work(&mut needs, bindings, &mut demo);
                }
            }
        }
        Mode::Bucketed => {
            for &e in &buckets.as_ref().unwrap().low[phase as usize] {
                if let Ok((_, mut needs, bindings, mut demo)) = q.get_mut(e) {
                    low_work(&mut needs, bindings, &mut demo);
                }
            }
        }
    }
    times.samples[2].push(t.elapsed().as_secs_f64() * 1e3);
}

fn band_verylow(
    mode: Res<Mode>,
    frame: Res<Frame>,
    buckets: Option<Res<Buckets>>,
    mut q: Query<(&CitizenIndex, &mut Needs, &mut Demographics, &mut Bindings)>,
    mut times: ResMut<BandTimes>,
) {
    let t = Instant::now();
    let phase = frame.0 % 4096;
    match *mode {
        Mode::NaiveScan => {
            for (idx, mut needs, mut demo, mut bindings) in &mut q {
                if idx.0 % 4096 == phase {
                    verylow_work(&mut needs, &mut demo, &mut bindings, frame.0);
                }
            }
        }
        Mode::Bucketed => {
            for &e in &buckets.as_ref().unwrap().verylow[phase as usize] {
                if let Ok((_, mut needs, mut demo, mut bindings)) = q.get_mut(e) {
                    verylow_work(&mut needs, &mut demo, &mut bindings, frame.0);
                }
            }
        }
    }
    times.samples[3].push(t.elapsed().as_secs_f64() * 1e3);
}

fn band_calendar(
    frame: Res<Frame>,
    q: Query<(&Needs, &Demographics)>,
    mut stats: ResMut<PlanStats>,
    mut times: ResMut<BandTimes>,
) {
    if frame.0 % FRAMES_PER_DAY != 0 {
        return;
    }
    let t = Instant::now();
    // day-edge plan-fulfilment accounting: full-population read sweep
    let mut acc = 0.0f64;
    for (needs, demo) in &q {
        acc += needs.0[0] as f64 + demo.wellbeing as f64;
    }
    stats.0 += acc;
    times.samples[4].push(t.elapsed().as_secs_f64() * 1e3);
}

fn band_housekeeping(frame: Res<Frame>, q: Query<&Demographics>, mut stats: ResMut<PlanStats>, mut times: ResMut<BandTimes>) {
    if frame.0 % 1024 != 0 {
        return;
    }
    let t = Instant::now();
    // autosave-scan-shaped: count + cheap aggregate
    let alive = q.iter().filter(|d| d.health > 0).count();
    stats.0 += alive as f64;
    times.samples[5].push(t.elapsed().as_secs_f64() * 1e3);
}

// ---------- harness ----------

fn run_mode(mode: Mode) -> (BandTimes, f64, f64) {
    let mut app = App::new();
    app.insert_resource(Frame::default())
        .insert_resource(BandTimes::default())
        .insert_resource(PlanStats::default())
        .insert_resource(mode)
        .add_systems(
            Update,
            (advance_frame, band_high, band_medium, band_low, band_verylow, band_calendar, band_housekeeping).chain(),
        );

    let t_spawn = Instant::now();
    spawn_citizens(app.world_mut());
    if mode == Mode::Bucketed {
        build_buckets(app.world_mut());
    }
    let spawn_ms = t_spawn.elapsed().as_secs_f64() * 1e3;

    let t_run = Instant::now();
    for _ in 0..FRAMES {
        app.update();
    }
    let total_s = t_run.elapsed().as_secs_f64();

    let times = std::mem::take(&mut *app.world_mut().resource_mut::<BandTimes>());
    (times, spawn_ms, total_s)
}

fn stats(samples: &[f64]) -> (f64, f64, f64) {
    if samples.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let avg = sorted.iter().sum::<f64>() / sorted.len() as f64;
    let p95 = sorted[((sorted.len() as f64 * 0.95) as usize).min(sorted.len() - 1)];
    let max = *sorted.last().unwrap();
    (avg, p95, max)
}

fn rss_mb() -> f64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmRSS"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|kb| kb.parse::<f64>().ok())
        })
        .map(|kb| kb / 1024.0)
        .unwrap_or(0.0)
}

fn main() {
    ComputeTaskPool::get_or_init(TaskPool::default);
    println!("spike_250k: {CITIZENS} citizens, {FRAMES} frames per mode\n");

    for (label, mode) in [("A: naive modulo-scan", Mode::NaiveScan), ("B: phase-bucketed", Mode::Bucketed)] {
        let (times, spawn_ms, total_s) = run_mode(mode);
        let frame_ms = total_s * 1e3 / FRAMES as f64;
        println!("== mode {label} ==");
        println!("spawn: {spawn_ms:.1} ms   run: {total_s:.2} s   avg frame: {frame_ms:.3} ms   rss: {:.0} MB", rss_mb());
        println!("{:<18} {:>10} {:>10} {:>10} {:>8}", "band", "avg ms", "p95 ms", "max ms", "calls");
        for (i, name) in BAND_NAMES.iter().enumerate() {
            let (avg, p95, max) = stats(&times.samples[i]);
            println!("{name:<18} {avg:>10.4} {p95:>10.4} {max:>10.4} {:>8}", times.samples[i].len());
        }
        println!();
    }
}
