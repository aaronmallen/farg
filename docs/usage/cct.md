# Correlated Color Temperature

How to estimate the color temperature of a light source using Farg's four CCT algorithms.

## Quick Start

No CCT algorithm is enabled by default — pick one or use `all-cct`.

```toml
[dependencies]
farg = { version = "0.4", features = ["cct-ohno"] }
```

```rust
use farg::correlated_color_temperature::ohno;
use farg::space::Xyz;

// D65 white point (~6504 K)
let d65 = Xyz::new(0.95047, 1.0, 1.08883);
let cct = ohno::calculate(d65);

println!("CCT: {:.0} K", cct.value());   // ~6504 K
println!("MRD: {:.1}", cct.mrd());       // ~153.8 MRD
```

## ColorTemperature Type

All algorithms return a `ColorTemperature` value that provides both Kelvin and micro reciprocal degree
(MRD) representations:

```rust
use farg::correlated_color_temperature::ColorTemperature;

// Kelvin
let kelvin: f64 = cct.value();

// Micro reciprocal degrees (1,000,000 / K)
let mrd: f64 = cct.mrd();

// Convert directly to f64
let value: f64 = cct.into();
```

MRD is more perceptually uniform than Kelvin — equal MRD steps correspond to roughly equal perceived
color shifts along the Planckian locus.

## Algorithms

### Ohno (2014)

Ohno's method searches the Planckian locus in CIE 1960 UCS (u, v) space at 1 MRD steps, then applies
parabolic interpolation around the closest point for sub-MRD precision. It is the most accurate
general-purpose algorithm.

Requires: `cct-ohno`

```rust
use farg::correlated_color_temperature::ohno;
use farg::space::Xyz;

let d65 = Xyz::new(0.95047, 1.0, 1.08883);
let cct = ohno::calculate(d65);

assert!((cct.value() - 6504.0).abs() < 50.0);
```

**Range:** ~1,667 K to ~1,000,000 K
**Accuracy:** Sub-MRD precision through parabolic refinement

### Robertson (1968)

Robertson's method interpolates between 31 isotherms in CIE 1960 UCS (u, v) space from his original
paper. Reliable across the full range of standard illuminants.

Requires: `cct-robertson`

```rust
use farg::correlated_color_temperature::robertson;
use farg::space::Xyz;

let d65 = Xyz::new(0.95047, 1.0, 1.08883);
let cct = robertson::calculate(d65);

assert!((cct.value() - 6504.0).abs() < 50.0);
```

**Range:** ~1,667 K to ~infinity
**Accuracy:** Limited by the 31-entry isotherm table; good for standard illuminants

### Hernandez-Andres et al. (1999)

A higher-order exponential polynomial that extends the valid range far beyond McCamy's method. Uses two
coefficient sets selected automatically based on an initial estimate.

Requires: `cct-hernandez-andres`

```rust
use farg::correlated_color_temperature::hernandez_andres;
use farg::space::Xyz;

let d65 = Xyz::new(0.95047, 1.0, 1.08883);
let cct = hernandez_andres::calculate(d65);

assert!((cct.value() - 6504.0).abs() < 50.0);
```

**Range:** 3,000 K to 800,000 K
**Accuracy:** Higher accuracy than McCamy, especially at extreme temperatures

### McCamy (1992)

A simple third-degree polynomial in the CIE 1931 chromaticity epicenter. The fastest algorithm but
with the narrowest valid range.

Requires: `cct-mccamy`

```rust
use farg::correlated_color_temperature::mccamy;
use farg::space::Xyz;

let d65 = Xyz::new(0.95047, 1.0, 1.08883);
let cct = mccamy::calculate(d65);

assert!((cct.value() - 6504.0).abs() < 50.0);
```

**Range:** ~2,000 K to ~12,500 K
**Accuracy:** Good within range, degrades rapidly outside it

## Any Color Type Works

All CCT functions accept `impl Into<Xyz>`, so you can pass any color type directly — no manual
conversion needed.

```rust
use farg::correlated_color_temperature::ohno;
use farg::space::{Rgb, Srgb, Xyz};

let rgb = Rgb::<Srgb>::new(255, 200, 150);
let xyz = Xyz::new(0.95047, 1.0, 1.08883);

// Pass any color type
let cct_from_rgb = ohno::calculate(rgb);
let cct_from_xyz = ohno::calculate(xyz);
```

## ColorSpace Trait Integration

When any CCT feature is enabled, two convenience methods are available on every color type through the
`ColorSpace` trait. The algorithm is selected automatically using a priority chain:
Ohno > Robertson > Hernandez-Andres > McCamy.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let warm_white = Rgb::<Srgb>::new(255, 200, 150);

// Full method name
let cct = warm_white.correlated_color_temperature();

// Short alias
let cct = warm_white.cct();

println!("CCT: {:.0} K", cct.value());
```

## Choosing an Algorithm

| Algorithm        | Best for                      | Range              | Dependencies      |
|------------------|-------------------------------|--------------------|-------------------|
| Ohno             | General-purpose (recommended) | 1,667 to 1,000,000 | `chromaticity-uv` |
| Robertson        | Standard illuminant matching  | 1,667 to infinity  | `chromaticity-uv` |
| Hernandez-Andres | Wide-range including skylight | 3,000 to 800,000   | *(none)*          |
| McCamy           | Fast indoor lighting          | 2,000 to 12,500    | *(none)*          |

Ohno is the recommended default for most applications. McCamy is a good choice when speed matters and
the color temperature is known to fall within the indoor lighting range.

## Feature Flags

| Feature                | Algorithm        | Default |
|------------------------|------------------|---------|
| `cct-ohno`             | Ohno             | No      |
| `cct-robertson`        | Robertson        | No      |
| `cct-hernandez-andres` | Hernandez-Andres | No      |
| `cct-mccamy`           | McCamy           | No      |
| `all-cct`              | All above        | No      |

```toml
# A single algorithm
[dependencies]
farg = { version = "0.4", features = ["cct-ohno"] }

# Multiple algorithms
[dependencies]
farg = { version = "0.4", features = ["cct-ohno", "cct-mccamy"] }

# All CCT algorithms
[dependencies]
farg = { version = "0.4", features = ["all-cct"] }
```
