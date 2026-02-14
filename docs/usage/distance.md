# Color Distance

How to measure the perceptual or geometric distance between colors using Farg's six distance algorithms.

## Quick Start

CIEDE2000 is enabled by default — no extra feature flags needed.

```rust
use farg::distance::ciede2000;
use farg::space::{Rgb, Srgb};

let coral = Rgb::<Srgb>::new(255, 87, 51);
let teal = Rgb::<Srgb>::new(0, 128, 128);

// CIEDE2000 color difference
let delta_e = ciede2000::calculate(coral, teal);

// Check if two colors are perceptually indistinguishable
let is_same = delta_e < ciede2000::JND; // JND = 1.0
```

## Algorithms

### CIEDE2000 (default)

The [CIEDE2000](https://en.wikipedia.org/wiki/Color_difference#CIEDE2000) formula is the most
perceptually uniform CIE color difference metric. It includes corrections for lightness, chroma,
and hue, plus an interactive term for the blue region and a rotation term for chroma/hue
interaction. The result is always >= 0.0 and is **order-independent**.

```rust
use farg::distance::ciede2000;
use farg::space::{Rgb, Srgb};

let reference = Rgb::<Srgb>::new(255, 0, 0);
let sample = Rgb::<Srgb>::new(255, 10, 0);

let delta_e = ciede2000::calculate(reference, sample);

// Values below JND (1.0) are perceptually indistinguishable
if delta_e < ciede2000::JND {
    println!("Colors look the same");
}
```

Custom parametric factors adjust the relative weighting of lightness, chroma, and hue:

```rust
use farg::distance::ciede2000;
use farg::space::{Rgb, Srgb};

let a = Rgb::<Srgb>::new(255, 0, 0);
let b = Rgb::<Srgb>::new(200, 50, 50);

// Custom weights: kL, kC, kH
let delta_e = ciede2000::calculate_parametric(a, b, 2.0, 1.0, 1.0);
```

### CIE76

The [CIE76](https://en.wikipedia.org/wiki/Color_difference#CIE76) formula computes the Euclidean
distance in CIELAB space: `sqrt((ΔL*)² + (Δa*)² + (Δb*)²)`. It is the simplest perceptual color
difference metric and is **order-independent**.

Requires: `distance-cie76`

```rust
use farg::distance::cie76;
use farg::space::{Rgb, Srgb};

let a = Rgb::<Srgb>::new(255, 0, 0);
let b = Rgb::<Srgb>::new(0, 255, 0);

let delta_e = cie76::calculate(a, b);
```

### CIE94

The [CIE94](https://en.wikipedia.org/wiki/Color_difference#CIE94) formula extends CIE76 with
weighting functions for lightness, chroma, and hue. Unlike CIE76, **argument order matters** — the
first argument is the reference color and the second is the sample.

Requires: `distance-cie94`

```rust
use farg::distance::cie94;
use farg::space::{Rgb, Srgb};

let reference = Rgb::<Srgb>::new(255, 0, 0);
let sample = Rgb::<Srgb>::new(200, 50, 50);

// Graphic arts application (default weights)
let delta_e = cie94::calculate(reference, sample);

// Textile application (more tolerant of lightness differences)
let delta_e = cie94::calculate_textiles(reference, sample);

// Custom parametric factors: kL, K1, K2
let delta_e = cie94::calculate_parametric(reference, sample, 1.0, 0.045, 0.015);
```

Weight constants are also exported:

```rust
use farg::distance::cie94::{
    GRAPHIC_ARTS_KL, GRAPHIC_ARTS_K1, GRAPHIC_ARTS_K2,
    TEXTILES_KL, TEXTILES_K1, TEXTILES_K2,
};
```

### CMC l:c

The [CMC l:c](https://en.wikipedia.org/wiki/Color_difference#CMC_l:c_(1984)) formula was developed
by the Colour Measurement Committee of the Society of Dyers and Colourists. It uses CIE LCh
components and is **not order-independent** — the first argument is the reference color.

Requires: `distance-ciecmc`

```rust
use farg::distance::ciecmc;
use farg::space::{Rgb, Srgb};

let reference = Rgb::<Srgb>::new(255, 0, 0);
let sample = Rgb::<Srgb>::new(200, 50, 50);

// Perceptibility (l=1, c=1) — are the colors noticeably different?
let delta_e = ciecmc::calculate(reference, sample);

// Acceptability (l=2, c=1) — are the colors acceptably close?
let delta_e = ciecmc::calculate_acceptability(reference, sample);

// Custom l and c factors
let delta_e = ciecmc::calculate_parametric(reference, sample, 1.5, 1.0);
```

### Euclidean

Straight-line (L2) distance in CIE XYZ space: `sqrt((ΔX)² + (ΔY)² + (ΔZ)²)`. Fast and simple
but not perceptually uniform. The result is **order-independent**.

Requires: `distance-euclidean`

```rust
use farg::distance::euclidean;
use farg::space::Xyz;

let a = Xyz::new(0.4124, 0.2126, 0.0193);
let b = Xyz::new(0.7700, 0.9278, 0.1385);

let dist = euclidean::calculate(a, b);
```

### Manhattan

Taxicab (L1) distance in CIE XYZ space: `|ΔX| + |ΔY| + |ΔZ|`. Always >= the Euclidean distance
for the same color pair. The result is **order-independent**.

Requires: `distance-manhattan`

```rust
use farg::distance::manhattan;
use farg::space::Xyz;

let a = Xyz::new(0.4124, 0.2126, 0.0193);
let b = Xyz::new(0.7700, 0.9278, 0.1385);

let dist = manhattan::calculate(a, b);
```

## Any Color Type Works

All distance functions accept `impl Into<Xyz>`, so you can pass any color type directly — no manual
conversion needed.

```rust
use farg::distance::ciede2000;
use farg::space::{Rgb, Srgb, Xyz};

let rgb = Rgb::<Srgb>::new(255, 87, 51);
let xyz = Xyz::new(0.0, 0.0, 0.0);

// Mix and match color types freely
let delta_e = ciede2000::calculate(rgb, xyz);
```

## ColorSpace Trait Integration

CIEDE2000 powers three convenience methods on the `ColorSpace` trait, available on every color type.

### Perceptual Equivalence

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let a = Rgb::<Srgb>::new(128, 128, 128);
let b = Rgb::<Srgb>::new(128, 128, 129);

// ΔE*00 < 1.0 (JND) → perceptually identical
if a.is_perceptually_equivalent(b) {
    println!("Colors are indistinguishable");
}

// ΔE*00 >= 1.0 → perceptually different
if a.is_distinguishable_from(b) {
    println!("Colors are visibly different");
}
```

### Closest Match

Find the nearest color from a palette:

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let target = Rgb::<Srgb>::new(200, 50, 50);
let palette = [
    Rgb::<Srgb>::new(255, 0, 0),
    Rgb::<Srgb>::new(0, 255, 0),
    Rgb::<Srgb>::new(0, 0, 255),
];

if let Some(closest) = target.closest_match(&palette) {
    println!("Nearest color: {closest}");
}
```

## Choosing an Algorithm

| Algorithm | Best for                                              | Symmetric | Dependencies |
|-----------|-------------------------------------------------------|-----------|--------------|
| CIEDE2000 | General-purpose perceptual comparison (recommended)   | Yes       | `space-lab`  |
| CIE94     | Industrial color matching (graphic arts and textiles) | No        | `space-lab`  |
| CMC l:c   | Textile and dye industry quality control              | No        | `space-lch`  |
| CIE76     | Simple perceptual difference, educational use         | Yes       | `space-lab`  |
| Euclidean | Fast geometric distance in XYZ space                  | Yes       | *(none)*     |
| Manhattan | Alternative geometric distance in XYZ space           | Yes       | *(none)*     |

CIEDE2000 is the recommended default for most applications and is enabled by default.

## Feature Flags

| Feature              | Algorithm | Default |
|----------------------|-----------|---------|
| `distance-ciede2000` | CIEDE2000 | Yes     |
| `distance-cie76`     | CIE76     | No      |
| `distance-cie94`     | CIE94     | No      |
| `distance-ciecmc`    | CMC l:c   | No      |
| `distance-euclidean` | Euclidean | No      |
| `distance-manhattan` | Manhattan | No      |
| `all-distance`       | All above | No      |

```toml
# Just the default (CIEDE2000)
[dependencies]
farg = "0.4"

# Add specific algorithms
[dependencies]
farg = { version = "0.4", features = ["distance-cie76", "distance-ciecmc"] }

# All distance algorithms
[dependencies]
farg = { version = "0.4", features = ["all-distance"] }
```
