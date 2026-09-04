use geom::skeleton::{faces_from_skeleton, skeleton};
use geom::{minmax, vec2, Intersect, LinearColor, Polygon, Segment, Shape, Vec2, Vec3, AABB};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::panic::catch_unwind;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColoredMesh {
    pub faces: Vec<(Vec<Vec3>, LinearColor)>,
}

impl ColoredMesh {
    pub fn bbox(&self) -> AABB {
        let (ll, ur) = unwrap_or!(
            minmax(self.faces.iter().flat_map(|x| &x.0).map(|x| x.xy())),
            return AABB::zero()
        );
        AABB::new_ll_ur(ll, ur)
    }

    pub fn translate(&mut self, off: Vec2) {
        for (p, _) in &mut self.faces {
            for v in p {
                *v += off.z0();
            }
        }
    }
}

/// Bound on `gen_exterior_house` polygon attempts (sov-crl).
///
/// Each attempt generates a *different* polygon (the retry counter reseeds the
/// RNG), so in practice one succeeds quickly — but nothing in the skeleton
/// algorithm guarantees any given polygon is valid, so an unbounded `loop`
/// is a hang under the never-game-over pillar (a hang ends the game from the
/// player's seat just as surely as a panic). After this many failed attempts
/// the generator degrades to [`fallback_house_mesh`]: a plain rectangular
/// house with four walls and a flat roof. It is architecturally boring but
/// always usable (non-empty faces, sane bbox, door on the south edge), so map
/// generation always terminates with a placeable building.
const MAX_EXTERIOR_RETRIES: u32 = 64;

pub fn gen_exterior_house(size: f32, seed: u64) -> (ColoredMesh, Vec2) {
    gen_exterior_house_with_attempts(size, seed, MAX_EXTERIOR_RETRIES)
}

/// Placeholder mesh used when every skeleton attempt fails (sov-crl).
///
/// Deliberately trivial — a rectangle with the same footprint scale
/// (`size / 40`, matching the ~15-20 x 20-28 pre-scale rect in
/// [`gen_exterior_house`]) and wall/roof colors from the prototype set — so
/// the building pipeline (`Building::make`: rotate, walkway, insert) works
/// unchanged. Dimensions are clamped to stay positive for degenerate `size`.
fn fallback_house_mesh(size: f32, _seed: u64) -> (ColoredMesh, Vec2) {
    let s = (size / 40.0).max(0.05);
    let (w, d, h) = (17.5 * s, 24.0 * s, 5.0);
    let (hw, hd) = (w * 0.5, d * 0.5);
    let roof_col = LinearColor::from(crate::colors().roof_col);
    let wall_col = crate::colors().house_col.into();
    let c00 = Vec3::new(-hw, -hd, 0.0);
    let c10 = Vec3::new(hw, -hd, 0.0);
    let c11 = Vec3::new(hw, hd, 0.0);
    let c01 = Vec3::new(-hw, hd, 0.0);
    let r00 = Vec3::new(-hw, -hd, h);
    let r10 = Vec3::new(hw, -hd, h);
    let r11 = Vec3::new(hw, hd, h);
    let r01 = Vec3::new(-hw, hd, h);
    let mut mesh = ColoredMesh::default();
    mesh.faces.push((vec![c00, c10, r10, r00], wall_col));
    mesh.faces.push((vec![c10, c11, r11, r10], wall_col));
    mesh.faces.push((vec![c11, c01, r01, r11], wall_col));
    mesh.faces.push((vec![c01, c00, r00, r01], wall_col));
    mesh.faces.push((vec![r00, r10, r11, r01], roof_col));
    (mesh, Vec2::new(0.0, -hd))
}

fn gen_exterior_house_with_attempts(size: f32, seed: u64, max_attempts: u32) -> (ColoredMesh, Vec2) {
    'retry: for retry_cnt in 0..max_attempts {
        let mut ri = 0.0;
        let realseed = ((u64::from(retry_cnt) << 32) + seed) as f32;
        let mut gen_range = |a, b| -> f32 {
            ri += 1.0;
            common::rand::rand2(realseed, ri) * (b - a) + a
        };

        let width = gen_range(15.0, 20.0);
        let height = gen_range(20.0, 28.0);

        let mut p = Polygon::rect(width, height);

        for _ in 0..gen_range(1.0, 5.0) as usize {
            let seg = gen_range(0.0, p.len() as f32) as usize;

            let origlen = p.segment(seg).vec().mag();
            if origlen < 8.0 {
                continue;
            }

            let l = gen_range(-0.2, 0.5);
            let r = gen_range(l + 0.4, l + 1.0);
            if r <= 1.0 {
                p.split_segment(seg, r);
            }

            let newlen = p.segment(seg).vec().mag();

            if l >= 0.0 {
                p.split_segment(seg, l * origlen / newlen);
                p.extrude(seg + 1, gen_range(1.0, 8.0));
            } else {
                p.extrude(seg, gen_range(1.0, 8.0));
            }

            p.simplify();
        }

        for x in p.iter_mut() {
            *x *= size / 40.0;
        }

        let c = p.bbox().center();

        for x in p.iter_mut() {
            *x -= c;
        }

        let merge_triangles = gen_range(0.0, 1.0) < 0.5;

        // silence panics
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| ()));
        // have to catch because the algorithm for skeleton might fail and is quite complicated
        let (skeleton, (faces, contour)) = unwrap_or!(
            catch_unwind(|| {
                // `None` means the skeleton algorithm detected its own state was
                // inconsistent; treat it exactly like the panic path below and retry
                // with a different polygon.
                let skeleton = skeleton(p.as_slice(), &[])?;
                let faces = faces_from_skeleton(p.as_slice(), &skeleton, merge_triangles)?;
                Some((skeleton, faces))
            })
            .ok()
            .flatten(),
            {
                std::panic::set_hook(hook);
                continue 'retry;
            }
        );

        std::panic::set_hook(hook);

        if faces.len() < 2 {
            continue 'retry;
        }

        let segments = skeleton
            .iter()
            .flat_map(|x| x.sinks.iter().map(move |&dst| Segment::new(x.source, dst)));

        for mut x in segments.clone() {
            x.scale(0.99);
            for mut y in segments.clone() {
                y.scale(0.99);
                if x == y {
                    continue;
                }

                if x.intersects(&y) {
                    continue 'retry;
                }
            }
        }

        for s in &skeleton {
            if !p.contains(s.source) {
                continue 'retry;
            }
        }

        let lowest_segment = unwrap_or!(
            p.segments().min_by_key(|s| OrderedFloat(s.src.y + s.dst.y)),
            continue 'retry
        );

        let mut roofs = ColoredMesh::default();
        let roof_col = LinearColor::from(crate::colors().roof_col);

        let height = 4.0 + gen_range(0.0, 2.0);

        for mut face in faces {
            if face.len() < 3 {
                continue 'retry;
            }
            for v in &mut face {
                v.z += height;
            }
            roofs.faces.push((face, roof_col));
        }

        if contour.len() < 4 {
            continue 'retry;
        }

        let mut walls = Vec::with_capacity(contour.len());

        for (a, b, c) in geom::skeleton::window(&contour) {
            let ba = (a - b).normalize().xy();
            let bc = (c - b).normalize().xy();

            let mut d = (ba + bc).try_normalize().unwrap_or_default();

            if ba.perp_dot(bc) > 0.0 {
                d = -d;
            }

            if d.is_close(Vec2::ZERO, 0.1) {
                d = ba.perpendicular();
            }

            walls.push(b + d.z0() * 0.8 + Vec3::z(height));
        }

        for (&a, &b, _) in geom::skeleton::window(&walls) {
            let face = vec![a, b, b.xy().z0(), a.xy().z0()];
            roofs.faces.push((face, crate::colors().house_col.into()));
        }

        return (roofs, lowest_segment.middle());
    }
    // sov-crl: every attempt failed (corrupt skeletons, degenerate faces).
    // Degrade to a placeholder instead of retrying forever.
    fallback_house_mesh(size, seed)
}

///  XXXXX   
///  XXXXX   
///    XXX   
///     |
pub fn gen_exterior_farm(size: f32, seed: u64) -> (ColoredMesh, Vec2) {
    let h_size = 30.0;
    let (mut mesh, mut door_pos) = gen_exterior_house(h_size, seed);

    let gen_range = |a, b| -> f32 { common::rand::rand(seed as f32 + 7.0) * (b - a) + a };

    let b = mesh.bbox();
    let off = -b.ll - Vec2::splat(size * 0.5) + vec2(gen_range(0.0, size - h_size), 3.0);
    mesh.translate(off);
    door_pos += off;

    (mesh, door_pos)
}

// How to gen a house
// Idea: Make everything out of rectangles
// 1. Make exterior
//    - pick random rectangle
//    - add random rectangle along this rectangle (or not)
//    - add random rectangle along this rectangle (or not)
// 3. Merge the rectangles in one shape
// 3. Recursively split the shape horizontally and vertically
// 4. Score the resulting house based on "rectanglicity" and size of resulting regions
//    - rectanglicity: area of region divided by area of smallest surrounding bbox
// 5. Put holes in between regions for the doors
// 6. Put a outgoing door somewhere
// 7. Assign rooms somehow
//  necessary:
//    - bedroom
//    - kitchen
//    - toilets
//  optional:
//    - dining room
//    - office
//    - playroom
// 8. Score the room assignment based on some rules: kitchen next to bedrooms, small toilet and big bedroom etc

/*
const SIZE: usize = 200; // 20 meters

type Idx = (usize, usize);

struct HGrid([[u8; SIZE]; SIZE]);

struct GeneratedHouse {
    exterior: Polygon,
    //    rooms: Vec<(RoomType, Polygon)>,
    //    walls: Vec<>
}

impl HGrid {
    fn v(&self, pos: Idx) -> u8 {
        self.0[pos.1][pos.0]
    }

    fn add_rectangle(&mut self, near: Idx) {
        let w = randi_in(10, 50);
    }
}
*/
//fn gen_house

#[cfg(test)]
mod placement_stress {
    use super::gen_exterior_house;

    /// Resident set size in kilobytes, from `/proc/self/statm` (page count * page size).
    fn rss_kb() -> u64 {
        let statm = std::fs::read_to_string("/proc/self/statm").unwrap_or_default();
        let pages: u64 = statm
            .split_whitespace()
            .nth(1)
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        pages * 4
    }

    /// sov-bo3: `LAV::iter_keys` used to walk a corrupted linked list without a bound,
    /// which reached 17.6 GB RSS and got the process OOM-killed. `gen_exterior_house`
    /// is deterministic in `seed`, so this sweep is a fixed, reproducible corpus.
    ///
    /// Run it under a hard memory ceiling — it is a memory guard, not a timing test:
    ///
    ///   cargo test -p simulation --release placement_stress -- --ignored --nocapture
    ///
    /// wrapped in e.g.
    ///
    ///   systemd-run --user --scope -p MemoryMax=2G -p MemorySwapMax=0 -- <the above>
    ///
    /// Before the fix this is killed by the ceiling. After it, it completes and RSS
    /// stays flat.
    #[test]
    #[ignore = "slow memory-guard sweep; run explicitly under a MemoryMax ceiling"]
    fn gen_exterior_house_8m_100k_placements() {
        // `gen_exterior_house` reads `crate::colors()`, which reads the prototype set.
        // Without this the process dies inside an `unwrap_unchecked` — a debug-assert
        // panic in dev, a SIGSEGV in release. That is unrelated to sov-bo3.
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(crate::init::init);

        let n: u64 = std::env::var("SOV_BO3_PLACEMENTS")
            .ok()
            .and_then(|x| x.parse().ok())
            .unwrap_or(100_000);
        let start = rss_kb();
        let mut peak = start;
        for seed in 0..n {
            let (mesh, _) = gen_exterior_house(8.0, seed);
            assert!(!mesh.faces.is_empty(), "seed {seed} produced no faces");
            if seed % 10_000 == 0 {
                let now = rss_kb();
                peak = peak.max(now);
                // stderr is unbuffered, so the last line survives a hard kill by the ceiling.
                eprintln!("seed {seed}: rss {now} kB");
            }
        }
        let end = rss_kb();
        peak = peak.max(end);
        eprintln!("{n} placements: start rss {start} kB, peak rss {peak} kB, end rss {end} kB");
        assert!(
            peak < start + 512 * 1024,
            "RSS grew by more than 512 MB across the sweep: {start} kB -> {peak} kB"
        );
    }

    /// sov-crl: the retry loop in `gen_exterior_house` is bounded. Forcing
    /// zero attempts simulates total skeleton failure (every polygon
    /// rejected) and must still terminate with a usable placeholder mesh —
    /// four walls plus a roof, a non-degenerate bbox, and a door on the
    /// south edge — instead of retrying forever.
    #[test]
    fn exterior_house_total_failure_falls_back_to_usable_mesh() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(crate::init::init);

        let (mesh, door) = super::gen_exterior_house_with_attempts(8.0, 0xC41u64, 0);
        assert_eq!(mesh.faces.len(), 5, "fallback is 4 walls + 1 roof");
        let bbox = mesh.bbox();
        assert!(
            bbox.ll.x < bbox.ur.x && bbox.ll.y < bbox.ur.y,
            "fallback bbox is non-degenerate: {bbox:?}"
        );
        assert!(
            (door.y - bbox.ll.y).abs() < 1e-3,
            "fallback door sits on the south edge: door {door:?} bbox {bbox:?}"
        );
        assert!(
            door.x >= bbox.ll.x && door.x <= bbox.ur.x,
            "fallback door is within the footprint: door {door:?} bbox {bbox:?}"
        );
    }

    /// sov-crl: the bounded generator terminates on every seed in a fixed
    /// corpus with a non-empty mesh. Under the old unbounded `loop` a seed
    /// whose polygons kept failing would hang this test forever; with the
    /// bound the worst case degrades to the documented fallback above.
    #[test]
    fn exterior_house_terminates_across_seeds() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(crate::init::init);

        for seed in 0..256u64 {
            let (mesh, _) = gen_exterior_house(8.0, seed);
            assert!(!mesh.faces.is_empty(), "seed {seed} produced no faces");
        }
    }
}
