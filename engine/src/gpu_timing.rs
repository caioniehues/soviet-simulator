//! Opt-in per-pass GPU timestamps (sov-sqs).
//!
//! This measures **GPU** time: the timestamps are written by the GPU itself at the start and end
//! of a render pass and resolved from a query set. It is not the same quantity as
//! [`crate::framework::EngineTimes`] or the `profiling` spans, which measure CPU time spent
//! *recording* commands and can read near zero while the GPU is saturated.
//!
//! The whole module is inert unless the caller opts in **and** the adapter reports
//! [`wgpu::Features::TIMESTAMP_QUERY`]. When either is missing, `GfxContext::gpu_timings` stays
//! `None`, every render pass keeps `timestamp_writes: None` exactly as before, and the run
//! continues with a recorded reason.

use std::ops::Range;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Device, QuerySet,
    QuerySetDescriptor, QueryType, Queue, RenderPassTimestampWrites,
};

pub const N_GPU_PASSES: usize = 9;
const N_QUERIES: u32 = (N_GPU_PASSES * 2) as u32;

/// The render passes this module can time, in query-slot order.
///
/// Each pass owns two slots: `2*i` for the start timestamp and `2*i + 1` for the end.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum GpuPass {
    DepthPrepass = 0,
    ShadowCascade0 = 1,
    ShadowCascade1 = 2,
    ShadowCascade2 = 3,
    ShadowCascade3 = 4,
    Ssao = 5,
    Fog = 6,
    Main = 7,
    Background = 8,
}

/// Names as they appear in a capture record. Index matches the [`GpuPass`] discriminant.
pub const GPU_PASS_NAMES: [&str; N_GPU_PASSES] = [
    "depth_prepass",
    "shadow_cascade_0",
    "shadow_cascade_1",
    "shadow_cascade_2",
    "shadow_cascade_3",
    "ssao",
    "fog",
    "main",
    "background",
];

/// One contiguous run of timestamp queries and its fixed resolve-buffer destination.
#[derive(Debug, PartialEq, Eq)]
struct QueryResolveRun {
    range: Range<u32>,
    destination_offset: u64,
}

/// Return query-slot runs and their fixed resolve-buffer destinations.
///
/// Each pass occupies two fixed slots in the resolve and readback buffers.
fn written_query_runs(written: &[bool; N_GPU_PASSES]) -> Vec<QueryResolveRun> {
    let mut runs = Vec::new();
    let mut start = None;

    for (pass, was_written) in written.iter().copied().enumerate() {
        match (start, was_written) {
            (None, true) => start = Some(pass),
            (Some(first), false) => {
                let range = (first as u32 * 2)..(pass as u32 * 2);
                runs.push(QueryResolveRun {
                    destination_offset: range.start as u64 * 8,
                    range,
                });
                start = None;
            }
            _ => {}
        }
    }
    if let Some(first) = start {
        let range = (first as u32 * 2)..N_QUERIES;
        runs.push(QueryResolveRun {
            destination_offset: range.start as u64 * 8,
            range,
        });
    }

    runs
}

impl GpuPass {
    /// The shadow cascade at `i`, or `None` when `i` is outside the instrumented cascades.
    pub fn shadow_cascade(i: usize) -> Option<GpuPass> {
        match i {
            0 => Some(GpuPass::ShadowCascade0),
            1 => Some(GpuPass::ShadowCascade1),
            2 => Some(GpuPass::ShadowCascade2),
            3 => Some(GpuPass::ShadowCascade3),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        GPU_PASS_NAMES[self as usize]
    }
}

/// One pass's timings across every sampled frame, in microseconds of GPU time.
#[derive(Clone, Debug)]
pub struct PassTimingSummary {
    pub pass: &'static str,
    pub samples: usize,
    pub min_us: f64,
    pub median_us: f64,
    pub max_us: f64,
}

/// A timestamp query set plus the buffers needed to read it back.
pub struct GpuTimings {
    set: QuerySet,
    resolve: Buffer,
    readback: Buffer,
    /// Nanoseconds per timestamp tick, as the queue reports it.
    period_ns: f32,
    /// Passes only write timestamps while armed, so warm-up frames stay untouched.
    armed: AtomicBool,
    /// Which slots were written this frame. A pass that did not run must not be reported.
    written: Mutex<[bool; N_GPU_PASSES]>,
    /// Per-pass microsecond samples, one entry per armed frame that pass ran in.
    samples: Mutex<Vec<Vec<f64>>>,
}

impl GpuTimings {
    /// The caller must already have checked that the device was created with
    /// [`wgpu::Features::TIMESTAMP_QUERY`].
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let size = (N_QUERIES as u64) * 8;
        Self {
            set: device.create_query_set(&QuerySetDescriptor {
                label: Some("per-pass gpu timestamps"),
                ty: QueryType::Timestamp,
                count: N_QUERIES,
            }),
            resolve: device.create_buffer(&BufferDescriptor {
                label: Some("gpu timestamp resolve"),
                size,
                usage: BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }),
            readback: device.create_buffer(&BufferDescriptor {
                label: Some("gpu timestamp readback"),
                size,
                usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            period_ns: queue.get_timestamp_period(),
            armed: AtomicBool::new(false),
            written: Mutex::new([false; N_GPU_PASSES]),
            samples: Mutex::new(vec![Vec::new(); N_GPU_PASSES]),
        }
    }

    /// Start or stop writing timestamps. Warm-up frames run unarmed, so their cost is excluded.
    pub fn set_armed(&self, armed: bool) {
        self.armed.store(armed, Ordering::Relaxed);
    }

    pub fn is_armed(&self) -> bool {
        self.armed.load(Ordering::Relaxed)
    }

    /// The `timestamp_writes` a render pass descriptor should carry for `pass`.
    ///
    /// Returns `None` while unarmed, which is exactly the descriptor the pass carried before this
    /// module existed.
    pub fn writes(&self, pass: GpuPass) -> Option<RenderPassTimestampWrites<'_>> {
        if !self.is_armed() {
            return None;
        }
        if let Ok(mut w) = self.written.lock() {
            w[pass as usize] = true;
        }
        let base = pass as u32 * 2;
        Some(RenderPassTimestampWrites {
            query_set: &self.set,
            beginning_of_pass_write_index: Some(base),
            end_of_pass_write_index: Some(base + 1),
        })
    }

    /// Resolve this frame's timestamps and fold them into the running samples.
    ///
    /// Blocks on the GPU, so it only ever runs during a capture. Every failure path returns
    /// quietly: a missing timing sample must never stop the frame loop.
    pub fn collect_frame(&self, device: &Device, queue: &Queue) {
        if !self.is_armed() {
            return;
        }
        let written = match self.written.lock() {
            Ok(mut w) => {
                let snapshot = *w;
                *w = [false; N_GPU_PASSES];
                snapshot
            }
            Err(_) => return,
        };
        if !written.iter().any(|v| *v) {
            return;
        }

        let mut enc = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("gpu timestamp resolve encoder"),
        });
        for run in written_query_runs(&written) {
            enc.resolve_query_set(&self.set, run.range, &self.resolve, run.destination_offset);
        }
        enc.copy_buffer_to_buffer(&self.resolve, 0, &self.readback, 0, (N_QUERIES as u64) * 8);
        queue.submit(Some(enc.finish()));

        let slice = self.readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        device.poll(wgpu::Maintain::Wait);
        if !matches!(rx.recv(), Ok(Ok(()))) {
            return;
        }

        {
            let mapped = slice.get_mapped_range();
            let ticks: Vec<u64> = mapped
                .chunks_exact(8)
                .map(|c| u64::from_le_bytes(c.try_into().unwrap_or([0; 8])))
                .collect();
            if let Ok(mut samples) = self.samples.lock() {
                for (i, was_written) in written.iter().enumerate() {
                    if !was_written {
                        continue;
                    }
                    let (begin, end) = (ticks[i * 2], ticks[i * 2 + 1]);
                    // An unwritten or wrapped pair says nothing; drop it rather than report a
                    // negative duration as if it were a measurement.
                    if end <= begin {
                        continue;
                    }
                    samples[i].push((end - begin) as f64 * self.period_ns as f64 / 1000.0);
                }
            }
        }
        self.readback.unmap();
    }

    /// Summarise every pass that produced at least one sample.
    pub fn summary(&self) -> Vec<PassTimingSummary> {
        let Ok(samples) = self.samples.lock() else {
            return Vec::new();
        };
        samples
            .iter()
            .enumerate()
            .filter(|(_, s)| !s.is_empty())
            .map(|(i, s)| {
                let mut sorted = s.clone();
                sorted.sort_by(|a, b| a.total_cmp(b));
                PassTimingSummary {
                    pass: GPU_PASS_NAMES[i],
                    samples: sorted.len(),
                    min_us: sorted[0],
                    median_us: sorted[sorted.len() / 2],
                    max_us: sorted[sorted.len() - 1],
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn written_query_runs_returns_no_runs_for_an_empty_mask() {
        assert!(written_query_runs(&[false; N_GPU_PASSES]).is_empty());
    }

    #[test]
    fn written_query_runs_combines_adjacent_passes() {
        let mut written = [false; N_GPU_PASSES];
        written[1] = true;
        written[2] = true;

        assert_eq!(
            written_query_runs(&written),
            vec![QueryResolveRun {
                range: 2..6,
                destination_offset: 16,
            }]
        );
    }

    #[test]
    fn written_query_runs_keeps_separated_passes_in_fixed_slots() {
        let mut written = [false; N_GPU_PASSES];
        written[0] = true;
        written[3] = true;
        written[8] = true;

        assert_eq!(
            written_query_runs(&written),
            vec![
                QueryResolveRun {
                    range: 0..2,
                    destination_offset: 0,
                },
                QueryResolveRun {
                    range: 6..8,
                    destination_offset: 48,
                },
                QueryResolveRun {
                    range: 16..18,
                    destination_offset: 128,
                },
            ]
        );
    }
}
