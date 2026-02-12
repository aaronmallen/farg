# Color Conversions

How to convert colors between Farg's 70+ color spaces.

## Core Concept: XYZ as Universal Hub

All conversions flow through CIE 1931 XYZ, the device-independent reference space. Every color type has a
`to_xyz()` method, and XYZ has methods to convert to every other space. This means any color space can reach
any other color space through XYZ.

```text
               ┌──────────────┐
               │     XYZ      │
               └──────┬───────┘
          ┌───────┬───┴───┬────────┐
          ▼       ▼       ▼        ▼
        Lab    Oklab   LinearRGB  LMS
         │       │       │
         ▼       ▼       ▼
        LCh   Oklch    RGB (sRGB, Display P3, …)
                │        │
                ▼        ▼
          Okhsl/Okhsv  HSL / HSV / HWB
```

## Basic Conversions

### Using `to_*` Methods

Every color type provides direct `to_*` methods for common conversions.

```rust
use farg::space::{Rgb, Srgb, Xyz};

let color = Rgb::<Srgb>::new(255, 87, 51);

// To XYZ (universal hub)
let xyz: Xyz = color.to_xyz();

// To perceptual spaces (requires feature flags)
let lab = color.to_xyz().to_lab();        // space-lab
let oklab = color.to_oklab();             // space-oklab (fast path via linear sRGB)
let oklch = color.to_oklab().to_oklch();  // space-oklab + space-oklch

// To cylindrical RGB spaces
let hsl = color.to_hsl();   // space-hsl
let hsv = color.to_hsv();   // space-hsv
let hwb = color.to_hwb();   // space-hwb

// To subtractive
let cmy = color.to_cmy();   // space-cmy
let cmyk = color.to_cmyk(); // space-cmyk
```

### Using `From` / `Into`

All color spaces implement `From<Xyz>`, so you can use `.into()` for conversions through XYZ.

```rust
use farg::space::{Lab, Oklab, Xyz};

let xyz = Xyz::new(0.4124, 0.2126, 0.0193);

// From trait
let lab = Lab::from(xyz);

// Into trait
let oklab: Oklab = xyz.into();
```

Many spaces also implement `From<Rgb<S>>` directly:

```rust
use farg::space::{Hsl, Oklch, Rgb, Srgb};

let color = Rgb::<Srgb>::new(255, 87, 51);
let hsl: Hsl<Srgb> = color.into();
let oklch: Oklch = color.into();
```

## RGB Space Conversions

### Between RGB Spaces

The `to_rgb::<S>()` method converts between any two RGB color spaces via XYZ.

```rust
use farg::space::{DisplayP3, Rgb, Srgb};

let srgb = Rgb::<Srgb>::new(255, 128, 0);
let p3: Rgb<DisplayP3> = srgb.to_rgb();
```

Requires the feature flag for the target RGB space (e.g. `rgb-display-p3`).

### Gamut Checking

When converting to a smaller-gamut RGB space, values may fall outside the 0.0-1.0 range.

```rust
use farg::space::{DisplayP3, Rgb, Srgb};

let p3 = Rgb::<DisplayP3>::new(0, 255, 0);
let srgb: Rgb<Srgb> = p3.to_rgb();

if !srgb.is_in_gamut() {
    // Option 1: Hard clip (fast, may shift hue)
    let mut clipped = srgb;
    clipped.clip_to_gamut();

    // Option 2: Chroma reduction (preserves hue, requires space-lab)
    let mut compressed = srgb;
    compressed.compress_to_gamut();
}
```

### Creating RGB Colors

```rust
use farg::space::{Rgb, Srgb};

// From 8-bit values (0-255)
let color = Rgb::<Srgb>::new(255, 100, 50);

// From normalized values (0.0-1.0)
let color = Rgb::<Srgb>::from_normalized(1.0, 0.39, 0.196);

// From hex code
let color = Rgb::<Srgb>::from_hexcode("#FF5733").unwrap();

// Constants
let black = Rgb::<Srgb>::BLACK;
let white = Rgb::<Srgb>::WHITE;
```

## Chromatic Adaptation

Colors exist within a viewing context (illuminant + observer). When converting between contexts with
different white points, chromatic adaptation is needed.

```rust
use farg::{Cat, ColorimetricContext, Illuminant, Observer};
use farg::space::Xyz;

// Create a color (default context: D65, CIE 1931 2°, Bradford CAT)
let color = Xyz::new(0.95047, 1.0, 1.08883);

// Adapt to a D50 viewing context
let d50_context = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_observer(Observer::CIE_1931_2D)
    .with_cat(Cat::BRADFORD);

let adapted = color.adapt_to(d50_context);
```

`adapt_to` is available on all CIE and physiological spaces: `Xyz`, `Lab`, `Lch`, `Luv`, and `Lms`.

### Context Without Adaptation

Use `with_context` to relabel a color's context without transforming its values.

```rust
let relabeled = color.with_context(d50_context); // No transformation
let adapted = color.adapt_to(d50_context);        // Transforms values
```

## Conversion Chain Examples

### sRGB to Oklch and Back

```rust
use farg::space::{Oklab, Oklch, Rgb, Srgb};

let original = Rgb::<Srgb>::new(70, 130, 180); // Steel blue

// Forward: sRGB → Oklab → Oklch
let oklab = original.to_oklab();
let oklch = oklab.to_oklch();
let [l, c, h] = oklch.components();

// Reverse: Oklch → Oklab → XYZ → sRGB
let back: Rgb<Srgb> = oklch.to_xyz().to_rgb();
```

### Cross-Space RGB Workflow

```rust
use farg::space::{AdobeRgb, DisplayP3, Rgb, Srgb};

let srgb = Rgb::<Srgb>::new(200, 50, 50);

// Convert to multiple output spaces
let p3: Rgb<DisplayP3> = srgb.to_rgb();
let adobe: Rgb<AdobeRgb> = srgb.to_rgb();
let xyz = srgb.to_xyz();
```

Requires `rgb-display-p3` and `rgb-adobe-rgb` features.

### Lab Round-Trip with Adaptation

```rust
use farg::{ColorimetricContext, Illuminant};
use farg::space::{Lab, Rgb, Srgb, Xyz};

let srgb = Rgb::<Srgb>::new(255, 128, 0);

// sRGB → XYZ → Lab (Lab auto-adapts to D50 internally)
let lab = srgb.to_xyz().to_lab();
let [l, a, b] = lab.components();

// Lab → XYZ → sRGB
let back: Rgb<Srgb> = lab.to_xyz().to_rgb();
```

### Reading Components

```rust
use farg::space::{Rgb, Srgb};

let color = Rgb::<Srgb>::new(255, 87, 51);

// 8-bit access
let r: u8 = color.red();    // 255
let g: u8 = color.green();  // 87
let b: u8 = color.blue();   // 51

// Normalized access (0.0-1.0)
let [r, g, b] = color.components();

// Display
println!("{color}"); // sRGB(255, 87, 51)
```

## Feature Flags for Conversions

Most conversion methods are gated behind feature flags matching the target space.

| Conversion target | Required feature  |
|-------------------|-------------------|
| Lab               | `space-lab`       |
| LCh               | `space-lch`       |
| Luv               | `space-luv`       |
| Oklab             | `space-oklab`     |
| Oklch             | `space-oklch`     |
| Okhsl             | `space-okhsl`     |
| Okhsv             | `space-okhsv`     |
| Okhwb             | `space-okhwb`     |
| HSL               | `space-hsl`       |
| HSV               | `space-hsv`       |
| HWB               | `space-hwb`       |
| CMY               | `space-cmy`       |
| CMYK              | `space-cmyk`      |
| Display P3        | `rgb-display-p3`  |
| Adobe RGB         | `rgb-adobe-rgb`   |
| Other RGB spaces  | `rgb-*`           |

XYZ, LMS, and sRGB conversions are always available with no feature flags.
