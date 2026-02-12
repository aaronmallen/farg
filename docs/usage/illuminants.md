# Illuminants

How to use standard and custom illuminants in Farg.

## What Is an Illuminant?

An illuminant represents a light source defined by its spectral power distribution (SPD) -- the
relative power emitted at each wavelength across the visible spectrum. Illuminants are essential for
colorimetric calculations because the appearance of a color depends on the light under which it is
viewed.

Every color in Farg exists within a viewing context that includes an illuminant. The default is CIE
Illuminant D65 (noon daylight), which is always available with no feature flags.

## Quick Start

```rust
use farg::Illuminant;

// The default illuminant (D65) -- always available
let d65 = Illuminant::D65;
println!("{d65}");          // "D65"
println!("{:?}", d65.kind()); // Daylight

// Access the spectral power distribution
let spd = d65.spd();
```

## Available Illuminants

Farg provides 45 standard illuminants across 6 categories. D65 is always available; the rest require
feature flags.

### Always Available

| Constant              | Illuminant | Type     | Description            |
|-----------------------|------------|----------|------------------------|
| `Illuminant::D65`     | D65        | Daylight | Noon daylight (6504 K) |
| `Illuminant::DEFAULT` | D65        | Daylight | Alias for `D65`        |

### Standard (`illuminant-standard`)

| Constant          | Feature        | Type         | Description                 |
|-------------------|----------------|--------------|-----------------------------|
| `Illuminant::A`   | `illuminant-a` | Incandescent | Tungsten lamp (2856 K)      |
| `Illuminant::B`   | `illuminant-b` | Daylight     | Obsolete daylight simulator |
| `Illuminant::C`   | `illuminant-c` | Daylight     | Obsolete daylight simulator |
| `Illuminant::E`   | `illuminant-e` | Equal energy | Equal-energy illuminant     |

### Daylight (`illuminant-daylight`)

| Constant            | Feature           | Description                   |
|---------------------|-------------------|-------------------------------|
| `Illuminant::D50`   | `illuminant-d50`  | Horizon daylight (5003 K)     |
| `Illuminant::D55`   | `illuminant-d55`  | Mid-morning daylight (5503 K) |
| `Illuminant::D75`   | `illuminant-d75`  | North sky daylight (7504 K)   |
| `Illuminant::ID50`  | `illuminant-id50` | Indoor daylight D50           |
| `Illuminant::ID65`  | `illuminant-id65` | Indoor daylight D65           |

### Fluorescent (`illuminant-fluorescent`)

12 fluorescent lamp illuminants: `Illuminant::FL1` through `Illuminant::FL12`.

```rust
use farg::Illuminant;

let fl2 = Illuminant::FL2; // Cool white fluorescent (4230 K)
```

### Fluorescent Series 3 (`illuminant-fluorescent-3`)

15 third-generation fluorescent illuminants: `Illuminant::FL3_1` through `Illuminant::FL3_15`.

```rust
use farg::Illuminant;

let fl3_7 = Illuminant::FL3_7; // Broadband D65 simulator
```

### High-Pressure Discharge (`illuminant-hp`)

5 gas discharge lamp illuminants: `Illuminant::HP1` through `Illuminant::HP5`.

### LED (`illuminant-led`)

9 LED illuminants: `Illuminant::LED_B1` through `Illuminant::LED_B5`, `Illuminant::LED_BH1`,
`Illuminant::LED_RGB1`, `Illuminant::LED_V1`, and `Illuminant::LED_V2`.

```rust
use farg::Illuminant;

let led = Illuminant::LED_B3; // Blue phosphor LED (4103 K)
```

See the [Feature Flags](features.md) reference for the complete list of all 45 illuminants and their
feature flags.

## Using Illuminants with ColorimetricContext

Illuminants are one of the three components of a `ColorimetricContext` (along with an observer and a
chromatic adaptation transform). The context determines how colors are interpreted.

```rust
use farg::{ColorimetricContext, Illuminant, Observer};

// Default context: D65 + CIE 1931 2 degree + Bradford CAT
let default_ctx = ColorimetricContext::new();

// Custom context with D50 illuminant
let d50_ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50);  // illuminant-d50

// Context with Illuminant A
let tungsten_ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::A);    // illuminant-a
```

### Chromatic Adaptation

When adapting colors between contexts with different illuminants, the chromatic adaptation transform
adjusts for the change in white point.

```rust
use farg::{ColorimetricContext, Illuminant};
use farg::space::Xyz;

// A color under D65 illumination
let color = Xyz::new(0.95047, 1.0, 1.08883);

// Adapt to D50 illumination (common for print workflows)
let d50_ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50);

let adapted = color.adapt_to(d50_ctx);
```

### Reference White Points

Each illuminant produces a different reference white point when integrated with an observer's color
matching functions. This white point anchors all relative colorimetric calculations.

```rust
use farg::{ColorimetricContext, Illuminant};

let d65_white = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .reference_white();

let d50_white = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .reference_white();

// D65 and D50 have different white points
let [x65, y65, z65] = d65_white.components();
let [x50, y50, z50] = d50_white.components();
```

## Accessing Illuminant Data

### Name and Type

```rust
use farg::Illuminant;

let d65 = Illuminant::D65;

let name: &str = d65.name();          // "D65"
let kind = d65.kind();                // IlluminantType::Daylight
println!("{d65}");                    // "D65" (Display trait)
```

### Spectral Power Distribution

The SPD gives the relative power at each wavelength. Use the `SpectralTable` trait to access the
data.

```rust
use farg::Illuminant;
use farg::spectral::SpectralTable;

let d65 = Illuminant::D65;
let spd = d65.spd();

// Number of wavelength samples
let len = spd.len();

// Look up power at a specific wavelength
if let Some(power) = spd.at(550) {
    println!("Power at 550 nm: {power}");
}

// Iterate over all (wavelength, power) pairs
for (wavelength, power) in spd.table() {
    println!("{wavelength} nm: {power:.4}");
}
```

### Illuminant Types

The `IlluminantType` enum categorizes illuminants:

| Variant               | Description                              |
|-----------------------|------------------------------------------|
| `Blackbody`           | Planckian (blackbody) radiator           |
| `Custom`              | User-defined illuminant                  |
| `Daylight`            | CIE daylight (D50, D65, etc.)            |
| `EqualEnergy`         | Equal-energy illuminant (E)              |
| `Fluorescent`         | Fluorescent lamp                         |
| `GasDischarge`        | Gas discharge lamp (HP series)           |
| `Incandescent`        | Incandescent lamp (Illuminant A)         |
| `Led`                 | LED source                               |
| `NarrowBand`          | Narrow-band illuminant                   |

## Building Custom Illuminants

Use the builder when you have your own SPD data (e.g., from a spectrophotometer measurement).

```rust
use farg::{Illuminant, IlluminantType};

static MY_SPD: &[(u32, f64)] = &[
    (380, 0.10),
    (400, 0.45),
    (420, 0.80),
    (440, 0.95),
    (460, 1.00),
    (480, 0.90),
    (500, 0.75),
    // ... more wavelength data
];

let illuminant = Illuminant::builder("My Light", IlluminantType::Custom)
    .with_spd(MY_SPD)
    .build()
    .unwrap();

println!("{illuminant}"); // "My Light"
```

### Using a Custom Illuminant in a Context

```rust
use farg::{ColorimetricContext, Illuminant, IlluminantType};
use farg::space::Xyz;

static MY_SPD: &[(u32, f64)] = &[
    (380, 0.10), (400, 0.45), (420, 0.80),
    (440, 0.95), (460, 1.00), (480, 0.90),
];

let my_light = Illuminant::builder("Studio Light", IlluminantType::Custom)
    .with_spd(MY_SPD)
    .build()
    .unwrap();

let ctx = ColorimetricContext::new()
    .with_illuminant(my_light);

// Adapt a color to the custom illuminant
let color = Xyz::new(0.95047, 1.0, 1.08883);
let adapted = color.adapt_to(ctx);
```

## Common Workflows

### Print: D65 to D50

Print standards (ICC profiles) use D50 as the reference illuminant, while displays typically use D65.

```rust
use farg::{ColorimetricContext, Illuminant};
use farg::space::Xyz;

let screen_color = Xyz::new(0.4124, 0.2126, 0.0193);

let print_ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50);

let print_color = screen_color.adapt_to(print_ctx);
```

### Comparing Illuminants

```rust
use farg::Illuminant;
use farg::spectral::SpectralTable;

let d65 = Illuminant::D65;
let a = Illuminant::A;

// Compare SPD at a specific wavelength
let d65_at_560 = d65.spd().at(560);
let a_at_560 = a.spd().at(560);
```

## Feature Flags

| Feature                    | Illuminants                | Default |
|----------------------------|----------------------------|---------|
| *(always available)*       | D65                        | Yes     |
| `illuminant-standard`      | A, B, C, E                 | No      |
| `illuminant-daylight`      | D50, D55, D75, ID50, ID65  | No      |
| `illuminant-fluorescent`   | FL1--FL12                  | No      |
| `illuminant-fluorescent-3` | FL3.1--FL3.15              | No      |
| `illuminant-hp`            | HP1--HP5                   | No      |
| `illuminant-led`           | LED-B1 through LED-V2      | No      |
| `all-illuminants`          | All 45 illuminants         | No      |

Individual illuminants can also be enabled directly (e.g., `illuminant-d50`, `illuminant-a`).

```toml
# Default (D65 only)
[dependencies]
farg = "0.4"

# Add specific illuminants
[dependencies]
farg = { version = "0.4", features = ["illuminant-d50", "illuminant-a"] }

# Enable an entire category
[dependencies]
farg = { version = "0.4", features = ["illuminant-daylight"] }

# All illuminants
[dependencies]
farg = { version = "0.4", features = ["all-illuminants"] }
```
