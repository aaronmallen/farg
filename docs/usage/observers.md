# Observers

How to use standard and custom observers in Farg.

## What Is an Observer?

An observer models the human visual system's response to light at different wavelengths. It packages three
related datasets:

- **Color Matching Functions (CMF)** — tristimulus weights mapping wavelengths to XYZ values
- **Chromaticity Coordinates** — the (x, y) locus derived from the CMF
- **Cone Fundamentals** — LMS cone sensitivity curves derived from the CMF

Every colorimetric calculation in Farg uses an observer. The default is the CIE 1931 2° standard
observer, which is always available with no feature flags.

## Quick Start

```rust
use farg::Observer;

// The default observer (CIE 1931 2°) — always available
let observer = Observer::CIE_1931_2D;
println!("{observer}"); // "CIE 1931 2°"

// Access the underlying data
let cmf = observer.cmf();
let coords = observer.chromaticity_coordinates();
let cones = observer.cone_fundamentals();
```

## Available Observers

### Always Available

| Constant                | Observer    | Field | Description                        |
|-------------------------|-------------|-------|------------------------------------|
| `Observer::CIE_1931_2D` | CIE 1931 2° | 2°    | The standard colorimetric observer |
| `Observer::DEFAULT`     | CIE 1931 2° | 2°    | Alias for `CIE_1931_2D`            |

### Feature-Gated

| Feature                         | Constant                         | Observer             | Field |
|---------------------------------|----------------------------------|----------------------|-------|
| `observer-cie-1931-judd-2d`     | `Observer::CIE_1931_JUDD_2D`     | CIE 1931 Judd 2°     | 2°    |
| `observer-cie-1931-judd-vos-2d` | `Observer::CIE_1931_JUDD_VOS_2D` | CIE 1931 Judd-Vos 2° | 2°    |
| `observer-cie-1964-10d`         | `Observer::CIE_1964_10D`         | CIE 1964 10°         | 10°   |
| `observer-cie-2006-2d`          | `Observer::CIE_2006_2D`          | CIE 2006 2°          | 2°    |
| `observer-cie-2006-10d`         | `Observer::CIE_2006_10D`         | CIE 2006 10°         | 10°   |
| `observer-stockman-sharpe-2d`   | `Observer::STOCKMAN_SHARPE_2D`   | Stockman-Sharpe 2°   | 2°    |
| `observer-stockman-sharpe-10d`  | `Observer::STOCKMAN_SHARPE_10D`  | Stockman-Sharpe 10°  | 10°   |

Enable all observers at once with the `all-observers` meta-feature.

```toml
# A specific observer
[dependencies]
farg = { version = "0.4", features = ["observer-cie-1964-10d"] }

# All observers
[dependencies]
farg = { version = "0.4", features = ["all-observers"] }
```

## Using Observers with ColorimetricContext

Observers are one of the three components of a `ColorimetricContext` (along with an illuminant and a
chromatic adaptation transform). The context determines how colors are interpreted and adapted.

```rust
use farg::{ColorimetricContext, Illuminant, Observer};

// Default context: D65 + CIE 1931 2° + Bradford CAT
let default_ctx = ColorimetricContext::new();

// Custom context with a 10° observer
let ctx = ColorimetricContext::new()
    .with_observer(Observer::CIE_1964_10D)    // observer-cie-1964-10d
    .with_illuminant(Illuminant::D50);         // illuminant-d50
```

### Chromatic Adaptation Between Contexts

When adapting colors between contexts that use different observers, the reference white point changes
because each observer integrates the illuminant SPD differently.

```rust
use farg::{ColorimetricContext, Observer};
use farg::space::Xyz;

let color = Xyz::new(0.95047, 1.0, 1.08883);

// Adapt from default (CIE 1931 2°) to CIE 1964 10°
let target = ColorimetricContext::new()
    .with_observer(Observer::CIE_1964_10D);

let adapted = color.adapt_to(target);
```

### Reference White Points

Each observer produces a different reference white when paired with the same illuminant, because
the CMF data differs.

```rust
use farg::{ColorimetricContext, Observer};

let ctx_2d = ColorimetricContext::new()
    .with_observer(Observer::CIE_1931_2D);

let ctx_10d = ColorimetricContext::new()
    .with_observer(Observer::CIE_1964_10D);

let white_2d = ctx_2d.reference_white();
let white_10d = ctx_10d.reference_white();
// These will differ slightly because the CMFs differ
```

## Accessing Observer Data

### Color Matching Functions

The CMF maps wavelengths to tristimulus (X, Y, Z) responses.

```rust
use farg::Observer;
use farg::spectral::SpectralTable;

let observer = Observer::CIE_1931_2D;
let cmf = observer.cmf();

// Number of wavelength samples
let len = cmf.len();

// Iterate over (wavelength, TristimulusResponse) pairs
for (wavelength, response) in cmf.table() {
    let [x, y, z] = response.components();
    println!("{wavelength} nm: X={x:.6}, Y={y:.6}, Z={z:.6}");
}
```

### Chromaticity Coordinates

The spectral locus in CIE (x, y) chromaticity.

```rust
use farg::Observer;
use farg::spectral::SpectralTable;

let observer = Observer::CIE_1931_2D;
let coords = observer.chromaticity_coordinates();

for (wavelength, xy) in coords.table() {
    println!("{wavelength} nm: x={}, y={}", xy.x(), xy.y());
}
```

### Cone Fundamentals

LMS cone sensitivity data derived from (or explicitly set on) the observer.

```rust
use farg::Observer;
use farg::spectral::SpectralTable;

let observer = Observer::CIE_1931_2D;
let cones = observer.cone_fundamentals();

for (wavelength, response) in cones.table() {
    let [l, m, s] = response.components();
    println!("{wavelength} nm: L={l:.6}, M={m:.6}, S={s:.6}");
}
```

## Building Custom Observers

Use the builder when you have your own CMF data (e.g., from measurements or literature).

```rust
use farg::Observer;

static MY_CMF: &[(u32, [f64; 3])] = &[
    (380, [0.001368, 0.000039, 0.006450]),
    (390, [0.004243, 0.000120, 0.020050]),
    (400, [0.014310, 0.000396, 0.067850]),
    // ... more wavelength data
];

let observer = Observer::builder("My Observer", 2.0)
    .with_cmf(MY_CMF)
    .build()
    .unwrap();

println!("{observer}"); // "My Observer 2°"
```

Chromaticity coordinates and cone fundamentals are automatically derived from the CMF. You can
override them with explicit data:

```rust
use farg::Observer;

static MY_CMF: &[(u32, [f64; 3])] = &[
    (380, [0.001368, 0.000039, 0.006450]),
    (390, [0.004243, 0.000120, 0.020050]),
];

static MY_COORDS: &[(u32, [f64; 2])] = &[
    (380, [0.1741, 0.0050]),
    (390, [0.1740, 0.0049]),
];

static MY_CONES: &[(u32, [f64; 3])] = &[
    (380, [0.001, 0.002, 0.003]),
    (390, [0.004, 0.005, 0.006]),
];

let observer = Observer::builder("Custom", 2.0)
    .with_cmf(MY_CMF)
    .with_chromaticity_coordinates(MY_COORDS)
    .with_cone_fundamentals(MY_CONES)
    .with_age(32)
    .build()
    .unwrap();
```

## Modifying Observers

The Fairchild modifier derives a new observer by applying physiological adjustments to an existing
observer's CMF data. This models observer metamerism — the variation in color matching between
individuals due to differences in age, visual field size, and other physiological factors.

### Field Size Adjustment

Adapt a 2° observer to approximate a wider or narrower field of view:

```rust
use farg::Observer;

let source = Observer::CIE_1931_2D;
let modified = source.modifier()
    .with_visual_field(10.0)
    .modify();

println!("{modified}"); // "CIE 1931 (Modified) 10°"
```

### Age-Based Adjustment

Model age-related lens yellowing:

```rust
use farg::Observer;

let source = Observer::CIE_1931_2D;
let aged = source.modifier()
    .with_age(60)
    .modify();
```

### Combined Adjustments

Chain multiple physiological parameters:

```rust
use farg::Observer;

let modified = Observer::CIE_1931_2D.modifier()
    .with_visual_field(10.0)
    .with_age(45)
    .with_rod_contribution_factor(0.05)
    .with_macular_peak(455.0)
    .modify();
```

The modifier applies several corrections in sequence:

| Correction         | Parameter methods                                                                   | Default behavior               |
|--------------------|-------------------------------------------------------------------------------------|--------------------------------|
| Lens yellowing     | `with_age()`, `with_age_yellowing_factor()`                                         | Field-size-based if age is 0   |
| Macular pigment    | `with_macular_peak()`, `with_macular_spread()`, `with_macular_density_decay_rate()` | Density varies with field size |
| Rod intrusion      | `with_rod_contribution_factor()`, `with_rod_peak()`, `with_rod_spread()`            | Active when narrowing field    |
| S-cone sensitivity | `with_s_cone_field_factor()`, `with_s_cone_peak()`, `with_s_cone_spread()`          | Adjusts with field size change |
| Blue absorption    | `with_blue_threshold()`, `with_blue_range()`                                        | 450 nm threshold, 50 nm range  |

The modified observer preserves the original luminance sum (Y channel) through normalization.

## Feature Flags

| Feature                         | Observer              | Default |
|---------------------------------|-----------------------|---------|
| *(always available)*            | CIE 1931 2°           | Yes     |
| `observer-cie-1931-judd-2d`     | CIE 1931 Judd 2°      | No      |
| `observer-cie-1931-judd-vos-2d` | CIE 1931 Judd-Vos 2°  | No      |
| `observer-cie-1964-10d`         | CIE 1964 10°          | No      |
| `observer-cie-2006-2d`          | CIE 2006 2°           | No      |
| `observer-cie-2006-10d`         | CIE 2006 10°          | No      |
| `observer-stockman-sharpe-2d`   | Stockman-Sharpe 2°    | No      |
| `observer-stockman-sharpe-10d`  | Stockman-Sharpe 10°   | No      |
| `all-observers`                 | All above             | No      |

```toml
# Default (CIE 1931 2° only)
[dependencies]
farg = "0.4"

# Add specific observers
[dependencies]
farg = { version = "0.4", features = ["observer-cie-1964-10d", "observer-cie-2006-2d"] }

# All observers
[dependencies]
farg = { version = "0.4", features = ["all-observers"] }
```
