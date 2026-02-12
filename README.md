# Farg

A Rust library for colorimetry, color space conversions, and color manipulation.

Farg provides context-aware color conversions with f64 precision, spectral data processing, and chromatic adaptation.
It's designed to serve web developers with sensible defaults while giving colorimetrists full control over illuminants,
observers, and adaptation transforms.

## Quick Start

```toml
[dependencies]
farg = "0.4"
```

```rust
use farg::space::{ColorSpace, Rgb, Srgb, Xyz};

// Create an sRGB color from 8-bit values or hex
let coral = Rgb::<Srgb>::new(255, 87, 51);
let coral = Rgb::<Srgb>::try_from("#FF5733").unwrap();

// Convert to CIE XYZ and read components
let xyz: Xyz = coral.to_xyz();
let [x, y, z] = xyz.components();

// Adjust luminance while preserving chromaticity
let brighter = xyz.with_luminance_scaled_by(1.2);

// Convert back to sRGB
let result: Rgb<Srgb> = brighter.to_rgb();
```

## Color Spaces

Farg supports 50+ color spaces organized by family. `Xyz`, `Lms`, and `Rgb<Srgb>` are always available; all others
are enabled through [feature flags](#feature-flags).

| Family                 | Spaces                            | Feature Flags                                                             |
|------------------------|-----------------------------------|---------------------------------------------------------------------------|
| **CIE**                | XYZ, Lab, LCh, Luv                | `space-lab`, `space-lch`, `space-luv`                                     |
| **Perceptual (Oklab)** | Oklab, Oklch, Okhsl, Okhsv, Okhwb | `space-oklab`, `space-oklch`, `space-okhsl`, `space-okhsv`, `space-okhwb` |
| **Cylindrical**        | HSL, HSV/HSB, HWB                 | `space-hsl`, `space-hsv`, `space-hwb`                                     |
| **Subtractive**        | CMY, CMYK                         | `space-cmy`, `space-cmyk`                                                 |
| **Physiological**      | LMS                               | *(always available)*                                                      |
| **RGB**                | sRGB + 37 additional spaces       | `all-rgb-spaces` or individual `rgb-*` flags                              |

RGB spaces include Display P3, Adobe RGB, Rec. 2020, Rec. 2100, ACES, ARRI Wide Gamut, and many more for display,
broadcast, cinema, and legacy workflows.

## Context-Aware Conversions

Colors exist within a viewing context: an illuminant, observer, and chromatic adaptation transform. The default
context (D65, CIE 1931 2°, Bradford) matches the standard sRGB environment.

```rust
use farg::{Cat, ColorimetricContext, Illuminant};
use farg::space::Xyz;

// Adapt a D65 color to D50 (e.g., for print workflows)
let d50 = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_cat(Cat::BRADFORD);

let color = Xyz::new(0.95047, 1.0, 1.08883);
let adapted = color.adapt_to(d50);
```

## Universal Property Access

Every color space implements the `ColorSpace` trait, providing access to any color property through automatic
conversion:

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let color = Rgb::<Srgb>::new(255, 87, 51);

// Access properties from any color model
let luminance = color.luminance();           // CIE Y
let chromaticity = color.chromaticity();     // CIE xy
let hue = color.hue();                       // perceptual hue
let chroma = color.chroma();                 // perceptual chroma

// Mutate through any property
let shifted = color.with_hue(180.0);
let desaturated = color.with_chroma_scaled_by(0.5);
```

## Alpha & Compositing

All color spaces support an alpha channel with a full mutation API:

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let overlay = Rgb::<Srgb>::new(255, 0, 0).with_alpha(0.5);
let background = Rgb::<Srgb>::new(0, 0, 255);

// Flatten against a background
let composited = overlay.flatten_alpha_against(&background);
```

## Gamut Mapping

Four strategies for mapping out-of-gamut colors back into a target RGB gamut:

```rust
use farg::space::{ColorSpace, Rgb, Srgb, Xyz};

let wide = Xyz::new(0.2, 0.5, 0.1);

let clipped: Rgb<Srgb> = wide.clip_to_gamut();           // clamp to [0, 1]
let scaled: Rgb<Srgb> = wide.scale_to_gamut();           // linear scaling
let perceptual: Rgb<Srgb> = wide.perceptually_map_to_gamut(); // LMS scaling
```

## Contrast

Six algorithms for evaluating perceptual contrast between colors:

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let white = Rgb::<Srgb>::new(255, 255, 255);
let text = Rgb::<Srgb>::new(51, 51, 51);

// WCAG 2.x contrast ratio with threshold checking
let ratio = white.contrast_ratio(&text);

// APCA lightness contrast
let lc = white.lightness_contrast(&text);
```

Additional algorithms (Michelson, Weber, RMS, AERT) are available behind `contrast-*` feature flags.

## Spectral Data

Full spectral power distribution and color matching function data for all standard illuminants and observers:

```rust
use farg::{Illuminant, Observer, SpectralTable};

let d65 = Illuminant::D65;
let spd = d65.spd();
let power_at_550nm = spd.at(550);

let observer = Observer::CIE_1931_2D;
let cmf = observer.cmf();
let xyz = cmf.spectral_power_distribution_to_xyz(&spd);
```

Build custom illuminants and observers from your own spectral data:

```rust
use farg::{IlluminantBuilder, IlluminantType, ObserverBuilder};
```

## Feature Flags

Farg uses granular feature flags so you only compile what you need. The `default` feature enables Bradford CAT,
WCAG contrast, and APCA contrast. D65, CIE 1931 2°, sRGB, XYZ, and LMS are always available.

| Feature            | Contents                                                                             |
|--------------------|--------------------------------------------------------------------------------------|
| `full`             | Everything below                                                                     |
| `all-cats`         | All 9 chromatic adaptation transforms                                                |
| `all-chromaticity` | All chromaticity coordinate systems (Rg, Uv, u'v')                                   |
| `all-contrast`     | All 6 contrast algorithms                                                            |
| `all-illuminants`  | All standard illuminants (44 total across daylight, fluorescent, LED, and more)      |
| `all-observers`    | All 7 additional observers (CIE 1964 10°, CIE 2006, Stockman-Sharpe, Judd, Judd-Vos) |
| `all-rgb-spaces`   | All 37 additional RGB color spaces                                                   |
| `all-spaces`       | All color spaces (CIE, cylindrical, perceptual, subtractive, and all RGB)            |

Individual features follow the pattern `{category}-{name}`, e.g., `space-oklab`, `cat-bradford`, `illuminant-d50`,
`rgb-display-p3`.

Enable everything:

```toml
[dependencies]
farg = { version = "0.4", features = ["full"] }
```

Or pick what you need:

```toml
[dependencies]
farg = { version = "0.4", features = ["space-oklab", "space-lab", "all-illuminants"] }
```

## Documentation

- [API Documentation](https://docs.rs/farg)
- [Project Plan](docs/PLAN.md)
- [Contributing Guide](docs/CONTRIBUTING.md)

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
