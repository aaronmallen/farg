# Contrast Calculation

How to measure contrast between colors using Farg's six contrast algorithms.

## Quick Start

WCAG and APCA are enabled by default — no extra feature flags needed.

```rust
use farg::contrast::{apca, wcag};
use farg::space::{Rgb, Srgb};

let text = Rgb::<Srgb>::new(0, 0, 0);       // black
let background = Rgb::<Srgb>::new(255, 255, 255); // white

// WCAG 2.x contrast ratio
let ratio = wcag::contrast_ratio(text, background);
assert!(ratio.meets_aa());         // >= 4.5:1

// APCA lightness contrast
let lc = apca::calculate(text, background);
assert!(lc.meets_body_text_threshold()); // Lc >= 60
```

## Algorithms

### WCAG 2.x (default)

The [WCAG 2.2 contrast ratio](https://www.w3.org/TR/WCAG22/#dfn-contrast-ratio) is the most widely
used accessibility metric. The ratio ranges from 1:1 (no contrast) to 21:1 (black on white) and is
**order-independent** — swapping the two colors produces the same result.

```rust
use farg::contrast::wcag;
use farg::space::{Rgb, Srgb};

let text = Rgb::<Srgb>::new(51, 51, 51);
let bg = Rgb::<Srgb>::new(255, 255, 255);

let ratio = wcag::contrast_ratio(text, bg);
println!("Ratio: {:.1}:1", ratio.value()); // ~12.6:1

// Check conformance levels
ratio.meets_aa();             // normal text >= 4.5:1
ratio.meets_aa_large_text();  // large text  >= 3:1
ratio.meets_aaa();            // normal text >= 7:1
ratio.meets_aaa_large_text(); // large text  >= 4.5:1
```

The raw value is available via `.value()` or `Into<f64>`:

```rust
let value: f64 = ratio.into();
```

Threshold constants are also exported:

```rust
use farg::contrast::wcag::{AA_NORMAL_TEXT, AA_LARGE_TEXT, AAA_NORMAL_TEXT, AAA_LARGE_TEXT};
```

### APCA (default)

The [APCA (Accessible Perceptual Contrast Algorithm)](https://github.com/Myndex/SAPC-APCA) is a
next-generation contrast metric designed to replace WCAG 2.x ratios. It returns **Lc (Lightness
Contrast)** values that are polarity-aware:

- **Positive Lc**: dark text on light background (normal polarity)
- **Negative Lc**: light text on dark background (reverse polarity)

Unlike WCAG, **argument order matters** — the first argument is the text color and the second is the
background.

```rust
use farg::contrast::apca;
use farg::space::{Rgb, Srgb};

let text = Rgb::<Srgb>::new(0, 0, 0);
let bg = Rgb::<Srgb>::new(255, 255, 255);

// Dark text on light background → positive Lc
let normal = apca::calculate(text, bg);
assert!(normal.value() > 0.0);

// Light text on dark background → negative Lc
let reverse = apca::calculate(bg, text);
assert!(reverse.value() < 0.0);
```

APCA uses different thresholds for normal and reverse polarity:

| Text size  | Normal polarity (Lc) | Reverse polarity (Lc) |
|------------|----------------------|-----------------------|
| Body text  | >= 60                | >= 75                 |
| Large text | >= 45                | >= 60                 |
| Very large | >= 30                | >= 45                 |

```rust
let lc = apca::calculate(text, bg);

lc.meets_body_text_threshold();       // body text
lc.meets_large_text_threshold();      // large text
lc.meets_very_large_text_threshold(); // very large text / non-text
```

### AERT

The [W3C AERT brightness difference](https://www.w3.org/TR/AERT/#color-contrast) measures the
absolute difference in BT.601 perceived brightness between two colors. Values range from 0
(identical) to 255 (black vs white). The recommended minimum for accessible text is 125.

Requires: `contrast-aert`

```rust
use farg::contrast::aert;
use farg::space::{Rgb, Srgb};

let a = Rgb::<Srgb>::new(0, 0, 0);
let b = Rgb::<Srgb>::new(255, 255, 255);

let diff = aert::calculate(a, b);
assert!(diff >= aert::RECOMMENDED_MINIMUM); // >= 125
```

### Michelson

[Michelson contrast](https://en.wikipedia.org/wiki/Contrast_(vision)#Michelson_contrast) (also
called visibility or modulation contrast) is defined as `(L_max - L_min) / (L_max + L_min)`. Values
range from 0.0 to 1.0. Originally developed for sinusoidal gratings, it is order-independent.

Requires: `contrast-michelson`

```rust
use farg::contrast::michelson;
use farg::space::Xyz;

let dark = Xyz::new(0.0, 0.2, 0.0);
let light = Xyz::new(0.0, 0.8, 0.0);

let contrast = michelson::calculate(dark, light);
assert!(contrast > 0.0 && contrast < 1.0);
```

### RMS

RMS (Root Mean Square) contrast computes the standard deviation of two luminance values, simplifying
to `|L1 - L2| / 2` for a pair. The result is always >= 0.0 and order-independent.

Requires: `contrast-rms`

```rust
use farg::contrast::rms;
use farg::space::Xyz;

let a = Xyz::new(0.0, 0.2, 0.0);
let b = Xyz::new(0.0, 0.8, 0.0);

let contrast = rms::calculate(a, b);
// |0.2 - 0.8| / 2 = 0.3
```

### Weber

[Weber contrast](https://en.wikipedia.org/wiki/Contrast_(vision)#Weber_contrast) measures the
visibility of a target against a uniform background: `(L_target - L_background) / L_background`.
The result is **signed** — positive when the target is brighter, negative when darker. Returns
`f64::INFINITY` for a non-black target on a black background.

Requires: `contrast-weber`

```rust
use farg::contrast::weber;
use farg::space::Xyz;

let target = Xyz::new(0.0, 0.6, 0.0);
let background = Xyz::new(0.0, 0.2, 0.0);

let contrast = weber::calculate(target, background);
// (0.6 - 0.2) / 0.2 = 2.0
```

## Any Color Type Works

All contrast functions accept `impl Into<Xyz>`, so you can pass any color type directly — no manual
conversion needed.

```rust
use farg::contrast::wcag;
use farg::space::{Rgb, Srgb, Xyz};

let rgb = Rgb::<Srgb>::new(255, 87, 51);
let xyz = Xyz::new(0.0, 0.0, 0.0);

// Mix and match color types freely
let ratio = wcag::contrast_ratio(rgb, xyz);
```

## Feature Flags

| Feature              | Algorithm | Default |
|----------------------|-----------|---------|
| `contrast-wcag`      | WCAG 2.x  | Yes     |
| `contrast-apca`      | APCA      | Yes     |
| `contrast-aert`      | AERT      | No      |
| `contrast-michelson` | Michelson | No      |
| `contrast-rms`       | RMS       | No      |
| `contrast-weber`     | Weber     | No      |
| `all-contrast`       | All above | No      |

```toml
# Just the defaults (WCAG + APCA)
[dependencies]
farg = "0.4"

# Add specific algorithms
[dependencies]
farg = { version = "0.4", features = ["contrast-michelson", "contrast-weber"] }

# All contrast algorithms
[dependencies]
farg = { version = "0.4", features = ["all-contrast"] }
```
