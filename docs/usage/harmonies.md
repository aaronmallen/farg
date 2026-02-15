# Color Harmonies

How to generate harmonious color palettes using hue rotation and luminance scaling on the `ColorSpace` trait.

## Quick Start

Hue-based harmonies require any cylindrical or perceptual color space feature. Oklch is recommended.

```toml
[dependencies]
farg = { version = "0.4", features = ["space-oklch"] }
```

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let coral = Rgb::<Srgb>::new(255, 87, 51);

// Complementary — opposite on the color wheel
let complement = coral.complementary();

// Analogous — two neighbors at ±30°
let [left, right] = coral.analogous();

// Monochromatic — two darker and two lighter variations
let [dark2, dark1, light1, light2] = coral.monochromatic();
```

## Hue-Based Harmonies

All hue-based methods rotate through the hue channel of the highest-priority cylindrical color
space enabled. The priority chain is: Oklch > LCh > Okhsl > Okhsv > Okhwb > HSL > HSV > HWB.

The original color is never included in the returned array.

### Complementary

Returns the color directly opposite on the color wheel (+180°). Provides maximum hue contrast.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let blue = Rgb::<Srgb>::new(0, 100, 200);
let complement = blue.complementary();
```

### Analogous

Returns two colors adjacent on the color wheel (-30° and +30°). Creates harmonious, low-contrast
palettes.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let green = Rgb::<Srgb>::new(34, 139, 34);
let [left, right] = green.analogous();
```

### Split-Complementary

Returns two colors flanking the complement (+150° and +210°). Offers strong contrast with more
variety than a straight complementary pair.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let purple = Rgb::<Srgb>::new(128, 0, 128);
let [a, b] = purple.split_complementary();
```

### Triadic

Returns two colors at equal 120° intervals (+120° and +240°). Together with the original, the
three colors form an equilateral triangle on the color wheel.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let red = Rgb::<Srgb>::new(220, 20, 60);
let [b, c] = red.triadic();
```

### Tetradic

Returns three colors at 90° intervals (+90°, +180°, +270°). Together with the original, the four
colors form a square on the color wheel with two complementary pairs.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let orange = Rgb::<Srgb>::new(255, 165, 0);
let [b, c, d] = orange.tetradic();
```

## Luminance-Based Harmony

### Monochromatic

Returns four luminance variations — two darker and two lighter — while preserving chromaticity.
Always available with no extra feature flags.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let teal = Rgb::<Srgb>::new(0, 128, 128);
let [dark2, dark1, light1, light2] = teal.monochromatic();

// Luminance scale factors: 0.30, 0.60, 1.40, 1.70
```

## Any Color Type Works

All harmony methods are available on every type that implements `ColorSpace`, so you can call them
on any color type directly.

```rust
use farg::space::{ColorSpace, Xyz};

let white = Xyz::new(0.95047, 1.0, 1.08883);
let [darker2, darker1, lighter1, lighter2] = white.monochromatic();
let complement = white.complementary();
```

## Feature Flags

Hue-based harmonies (`analogous`, `complementary`, `split_complementary`, `triadic`, `tetradic`)
require at least one cylindrical or perceptual color space:

| Feature        | Color Space | Default |
|----------------|-------------|---------|
| `space-oklch`  | Oklch       | No      |
| `space-lch`    | CIE LCh     | No      |
| `space-okhsl`  | Okhsl       | No      |
| `space-okhsv`  | Okhsv       | No      |
| `space-okhwb`  | Okhwb       | No      |
| `space-hsl`    | HSL         | No      |
| `space-hsv`    | HSV         | No      |
| `space-hwb`    | HWB         | No      |

`monochromatic()` has no extra dependencies and is always available.

```toml
# Oklch (recommended for perceptual uniformity)
[dependencies]
farg = { version = "0.4", features = ["space-oklch"] }

# Or any cylindrical space works
[dependencies]
farg = { version = "0.4", features = ["space-hsl"] }
```
