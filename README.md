# Farg

A Rust library for colorimetry, color space conversions, and color manipulation.

Farg provides context-aware color conversions with f64 precision, spectral data processing, and chromatic adaptation.
It's designed to serve web developers with sensible defaults while giving colorimetrists full control over illuminants,
observers, and adaptation transforms.

## Features

- **40+ RGB color spaces** — sRGB, Display P3, Adobe RGB, Rec. 2020, ACES, and many more
- **Cylindrical color spaces** — HSL, HSV/HSB, HWB
- **Subtractive color spaces** — CMY, CMYK
- **CIE XYZ and LMS** — Device-independent color spaces as conversion hubs
- **Chromatic adaptation** — Bradford, CAT02, CAT16, Von Kries, and other transforms
- **Spectral data** — SPD and color matching function support with wavelength-level access
- **Standard illuminants** — D65, D50, A, E, fluorescent, LED, and others
- **Standard observers** — CIE 1931 2°, CIE 1964 10°, CIE 2006, Stockman-Sharpe
- **Chromaticity coordinates** — CIE xy, uv, u'v', and rg systems
- **Feature-gated** — Include only what you need for minimal binary size
- **f64 precision** — All internal calculations use 64-bit floating point

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
farg = "0.2"
```

By default, farg includes the Bradford chromatic adaptation transform and the D65 illuminant with CIE 1931 2° observer.
Enable additional features as needed:

```toml
[dependencies]
farg = { version = "0.2", features = ["all-rgb-spaces", "all-illuminants"] }
```

Or enable everything:

```toml
[dependencies]
farg = { version = "0.2", features = ["full"] }
```

## Quick Start

### Create and convert colors

```rust
use farg::space::{Rgb, Srgb, Xyz};

// Create an sRGB color from 8-bit values
let color = Rgb::<Srgb>::new(255, 87, 51);

// Convert to CIE XYZ
let xyz: Xyz = color.to_xyz();

// Access components
let [x, y, z] = xyz.components();
```

### Parse hex codes

```rust
use farg::space::Srgb;

let color = Srgb::from_hexcode("#ff5733").unwrap();
let also_red = Srgb::from_hexcode("f00").unwrap();
```

### Work with chromaticity

```rust
use farg::chromaticity::Xy;
use farg::space::Xyz;

// Extract chromaticity from a color
let xyz = Xyz::new(0.95047, 1.0, 1.08883);
let xy: Xy = xyz.chromaticity();

// Reconstruct XYZ from chromaticity + luminance
let reconstructed = xy.to_xyz(1.0);
```

### Convert to cylindrical and subtractive spaces

```rust
use farg::space::{Hsl, Hsv, Hwb, Cmy, Cmyk, Rgb, Srgb};

let color = Rgb::<Srgb>::new(255, 87, 51);

// Convert to cylindrical spaces
let hsl: Hsl<Srgb> = color.to_hsl();
let hsv: Hsv<Srgb> = color.to_hsv();
let hwb: Hwb<Srgb> = color.to_hwb();

// Convert to subtractive spaces
let cmy: Cmy<Srgb> = color.to_cmy();
let cmyk: Cmyk<Srgb> = color.to_cmyk();
```

### Chromatic adaptation

```rust
use farg::{Cat, ColorimetricContext, Illuminant};
use farg::space::Xyz;

// Adapt a color from D65 to D50 illuminant
let d50_context = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_cat(Cat::BRADFORD);

let color = Xyz::new(0.95047, 1.0, 1.08883);
let adapted = color.adapt_to(d50_context);
```

### Spectral data

```rust
use farg::{Illuminant, Observer};
use farg::spectral::Table;

// Access illuminant spectral power distribution
let d65 = Illuminant::D65;
let spd = d65.spd();
let power_at_550nm = spd.at(550);

// Use observer color matching functions
let observer = Observer::CIE_1931_2D;
let cmf = observer.cmf();
let xyz = cmf.spectral_power_distribution_to_xyz(spd);
```

## Feature Flags

Farg uses granular feature flags so you only compile what you need.

### Meta Features

| Feature            | Contents                                         |
|--------------------|--------------------------------------------------|
| `full`             | Everything below                                 |
| `all-cats`         | All chromatic adaptation transforms              |
| `all-chromaticity` | All chromaticity coordinate systems              |
| `all-illuminants`  | All standard illuminants                         |
| `all-observers`    | All standard observers                           |
| `all-spaces`       | All color spaces (RGB, cylindrical, subtractive) |
| `all-rgb-spaces`   | All RGB color spaces                             |

### Individual Features

| Prefix           | Examples                                                            |
|------------------|---------------------------------------------------------------------|
| `cat-*`          | `cat-bradford` (default), `cat-cat02`, `cat-cat16`, `cat-von-kries` |
| `chromaticity-*` | `chromaticity-rg`, `chromaticity-uv`, `chromaticity-upvp`           |
| `illuminant-*`   | `illuminant-d50`, `illuminant-daylight`, `illuminant-led`           |
| `observer-*`     | `observer-cie-1964-10d`, `observer-cie-2006-2d`                     |
| `rgb-*`          | `rgb-display-p3`, `rgb-adobe-rgb`, `rgb-rec-2020`, `rgb-aces-cg`    |
| `space-*`        | `space-hsl`, `space-hsv`, `space-hwb`, `space-cmy`, `space-cmyk`    |

The D65 illuminant, CIE 1931 2° observer, sRGB, and XYZ/LMS spaces are always available.

## Documentation

- [API Documentation](https://docs.rs/farg)
- [Project Plan](docs/PLAN.md)
- [Contributing Guide](docs/CONTRIBUTING.md)

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
