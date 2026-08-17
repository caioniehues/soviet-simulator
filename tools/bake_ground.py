#!/usr/bin/env python3
"""Bake docs/art-direction.md's ground palette into the CC0 ground textures.

The ambientCG source photos are saturated lawn-green (mean sRGB #446628,
saturation 0.56).  A StandardMaterial tint cannot correct that: reaching the
doc's `#6b7050` desaturated olive needs linear multipliers of (2.5, 1.5, 3.6)
and `Color::srgb` clamps at 1.0.  So the correction happens once, offline, and
the shipped texture is already on-palette; the palette factory then only has to
avoid re-tinting it.

Transform, in LINEAR light: pull each pixel toward its own luminance by
`1 - KEEP_CHROMA`, then rescale the channels so the image's linear mean lands
exactly on the role colour.  Run from the repo root:

    python3 tools/bake_ground.py

Originals are kept alongside as `*_src.png` (ambientCG, CC0 1.0 — unchanged
provenance).
"""

import shutil
from pathlib import Path

import numpy as np
from PIL import Image

# (output, source stem, target hex from docs/art-direction.md, chroma retained)
#
# `dirt` is baked twice because one ambientCG photo serves two palette roles:
# building yards are worn earth, dirt roads are the lighter dry mud beside them.
# A material tint cannot separate them — the road role is *lighter* than the
# baked yard, so the multiplier would need to exceed 1.
JOBS = [
    ("field", "field", "6b7050", 0.45),
    ("dirt", "dirt", "7a6a52", 0.70),
    ("road_dirt", "dirt", "8a7355", 0.60),
]
TEXTURES = Path("assets/textures")
LUMA = np.array([0.2126, 0.7152, 0.0722])


def s2l(c):
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def l2s(c):
    return np.where(c <= 0.0031308, c * 12.92, 1.055 * np.clip(c, 0, None) ** (1 / 2.4) - 0.055)


def hex_linear(h):
    return s2l(np.array([int(h[i : i + 2], 16) for i in (0, 2, 4)], dtype=np.float64) / 255)


for name, source, target_hex, keep_chroma in JOBS:
    dst = TEXTURES / f"{name}.png"
    src = TEXTURES / f"{source}_src.png"
    if not src.exists():
        shutil.copy2(TEXTURES / f"{source}.png", src)

    lin = s2l(np.asarray(Image.open(src).convert("RGB"), dtype=np.float64) / 255)
    grey = (lin * LUMA).sum(2, keepdims=True)
    lin = grey + (lin - grey) * keep_chroma
    lin *= hex_linear(target_hex) / lin.reshape(-1, 3).mean(0)

    out = np.clip(lin, 0, 1)
    mean = l2s(out.reshape(-1, 3).mean(0))
    mx, mn = out.max(2), out.min(2)
    print(
        f"{name}.png -> mean #{''.join('%02x' % round(c * 255) for c in mean)}"
        f" (target #{target_hex}), saturation {((mx - mn) / np.maximum(mx, 1e-9)).mean():.3f}"
    )
    Image.fromarray((l2s(out) * 255).round().astype(np.uint8)).save(dst)
