# Color Vision Deficiency Simulation

How to simulate color blindness using Farg's three CVD algorithms.

## Quick Start

Brettel (dichromacy) and Machado (anomalous trichromacy) are enabled by default.

```toml
[dependencies]
farg = "0.4"
```

```rust
use farg::color_vision_deficiency::{brettel, machado};
use farg::space::{Rgb, Srgb, Xyz};

// Simulate complete red-blindness (protanopia)
let color = Rgb::<Srgb>::new(255, 87, 51);
let simulated: Xyz = brettel::protanopia(color.to_xyz());

// Simulate partial green-blindness (deuteranomaly) at 50% severity
let simulated: Xyz = machado::deuteranomaly(color.to_xyz(), 0.5);
```

## Deficiency Types

Color vision deficiencies fall into two categories:

**Dichromacy** — complete loss of one cone type:

| Condition    | Affected Cone | Prevalence (male) | Algorithms      |
|--------------|---------------|-------------------|-----------------|
| Protanopia   | L (red)       | ~1%               | Brettel, Viénot |
| Deuteranopia | M (green)     | ~1%               | Brettel, Viénot |
| Tritanopia   | S (blue)      | ~0.002%           | Brettel, Viénot |

**Anomalous trichromacy** — reduced sensitivity of one cone type:

| Condition     | Affected Cone | Prevalence (male) | Algorithm |
|---------------|---------------|-------------------|-----------|
| Protanomaly   | L (red)       | ~1%               | Machado   |
| Deuteranomaly | M (green)     | ~5%               | Machado   |
| Tritanomaly   | S (blue)      | Rare              | Machado   |

## Algorithms

### Brettel (1997)

The most accurate dichromacy simulation. Projects colors onto a reduced color plane in LMS cone
response space using two half-plane projections per deficiency type, selected by a separator normal
vector.

Requires: `cvd-brettel`

```rust
use farg::color_vision_deficiency::brettel;
use farg::space::Xyz;

let color = Xyz::new(0.4, 0.3, 0.2);

let protan = brettel::protanopia(color);
let deutan = brettel::deuteranopia(color);
let tritan = brettel::tritanopia(color);
```

### Viénot (1999)

A simplified single-plane projection that operates in linear sRGB space using a single 3x3 matrix
per deficiency type. Faster than Brettel but slightly less accurate for extreme colors.

Requires: `cvd-vienot`

```rust
use farg::color_vision_deficiency::vienot;
use farg::space::Xyz;

let color = Xyz::new(0.4, 0.3, 0.2);

let protan = vienot::protanopia(color);
let deutan = vienot::deuteranopia(color);
let tritan = vienot::tritanopia(color);
```

### Machado (2009)

Severity-parameterized simulation of anomalous trichromacy. Uses precomputed 3x3 matrices in linear
sRGB for 11 severity levels (0–10), linearly interpolated for intermediate values.

Severity ranges from 0.0 (normal vision) to 1.0 (complete dichromacy).

Requires: `cvd-machado`

```rust
use farg::color_vision_deficiency::machado;
use farg::space::Xyz;

let color = Xyz::new(0.4, 0.3, 0.2);

// Mild protanomaly
let mild = machado::protanomaly(color, 0.3);

// Severe deuteranomaly
let severe = machado::deuteranomaly(color, 0.9);

// Full severity (equivalent to dichromacy)
let full = machado::tritanomaly(color, 1.0);
```

## Any Color Type Works

All CVD functions accept `impl Into<Xyz>`, so you can pass any color type directly — no manual
conversion needed.

```rust
use farg::color_vision_deficiency::brettel;
use farg::space::{Rgb, Srgb, Xyz};

let rgb = Rgb::<Srgb>::new(255, 87, 51);
let xyz = Xyz::new(0.4, 0.3, 0.2);

// Pass any color type
let from_rgb = brettel::protanopia(rgb);
let from_xyz = brettel::protanopia(xyz);
```

## ColorSpace Trait Integration

When CVD features are enabled, convenience methods are available on every color type through the
`ColorSpace` trait.

### Dichromacy

Dichromacy methods use a priority chain: Brettel > Viénot. If both features are enabled, Brettel
is used automatically.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let color = Rgb::<Srgb>::new(255, 87, 51);

let protan: Rgb<Srgb> = color.simulate_protanopia();
let deutan: Rgb<Srgb> = color.simulate_deuteranopia();
let tritan: Rgb<Srgb> = color.simulate_tritanopia();
```

### Anomalous Trichromacy

Anomalous trichromacy methods require the `cvd-machado` feature and take a severity parameter.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let color = Rgb::<Srgb>::new(255, 87, 51);

let mild: Rgb<Srgb> = color.simulate_protanomaly(0.3);
let moderate: Rgb<Srgb> = color.simulate_deuteranomaly(0.5);
let severe: Rgb<Srgb> = color.simulate_tritanomaly(0.9);
```

## Choosing an Algorithm

| Algorithm | Type                  | Best for                                 | Speed  |
|-----------|-----------------------|------------------------------------------|--------|
| Brettel   | Dichromacy            | Accurate dichromacy simulation (default) | Medium |
| Viénot    | Dichromacy            | Fast previews, less accuracy at extremes | Fast   |
| Machado   | Anomalous trichromacy | Severity-parameterized simulation        | Fast   |

Brettel and Machado together cover the full range of CVD types and are both enabled by default.
Viénot is available as a faster alternative when Brettel's half-plane accuracy isn't needed.

## Feature Flags

| Feature       | Algorithm | Default |
|---------------|-----------|---------|
| `cvd-brettel` | Brettel   | Yes     |
| `cvd-machado` | Machado   | Yes     |
| `cvd-vienot`  | Viénot    | No      |
| `all-cvd`     | All above | No      |

```toml
# Default includes Brettel and Machado
[dependencies]
farg = "0.4"

# Add Viénot
[dependencies]
farg = { version = "0.4", features = ["cvd-vienot"] }

# All CVD algorithms
[dependencies]
farg = { version = "0.4", features = ["all-cvd"] }
```
