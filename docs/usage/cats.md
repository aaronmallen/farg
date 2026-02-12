# Chromatic Adaptation Transforms

How to use chromatic adaptation transforms (CATs) in Farg to adapt colors between different
illuminants.

## What Is a CAT?

A chromatic adaptation transform models how the human visual system adjusts to changes in
illumination. When you move a color from one lighting condition to another (e.g., daylight to
tungsten), the CAT adjusts the tristimulus values so the color appears perceptually consistent.

Each CAT is defined by a 3x3 matrix that converts XYZ values into a cone-response-like space.
Adaptation scaling is applied in that space, then the result is converted back to XYZ.

Farg provides 10 CATs. The default is Bradford (when the `cat-bradford` feature is enabled), and
XYZ Scaling is always available as a fallback.

## Quick Start

```rust
use farg::{Cat, ColorimetricContext, Illuminant};
use farg::space::Xyz;

// A color under D65 daylight
let color = Xyz::new(0.95047, 1.0, 1.08883);

// Adapt to D50 using the default CAT (Bradford)
let d50_ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50);

let adapted = color.adapt_to(d50_ctx);
```

## Available CATs

### Always Available

| Constant           | Transform   | Description                                     |
|--------------------|-------------|-------------------------------------------------|
| `Cat::XYZ_SCALING` | XYZ Scaling | Identity matrix -- scales X, Y, Z independently |
| `Cat::DEFAULT`     | *(varies)*  | Highest-priority enabled CAT (see below)        |

### Feature-Gated

| Feature                    | Constant                    | Transform            |
|----------------------------|-----------------------------|----------------------|
| `cat-bradford`             | `Cat::BRADFORD`             | Bradford             |
| `cat-cat02`                | `Cat::CAT02`                | CAT02                |
| `cat-cat16`                | `Cat::CAT16`                | CAT16                |
| `cat-cmc-cat97`            | `Cat::CMC_CAT97`            | CMC CAT97            |
| `cat-cmc-cat2000`          | `Cat::CMC_CAT2000`          | CMC CAT2000          |
| `cat-fairchild`            | `Cat::FAIRCHILD`            | Fairchild            |
| `cat-hunt-pointer-estevez` | `Cat::HUNT_POINTER_ESTEVEZ` | Hunt-Pointer-Estevez |
| `cat-sharp`                | `Cat::SHARP`                | Sharp                |
| `cat-von-kries`            | `Cat::VON_KRIES`            | Von Kries            |

`Cat::HPE` is a shorthand alias for `Cat::HUNT_POINTER_ESTEVEZ`. The feature `cat-hpe` is an alias
for `cat-hunt-pointer-estevez`.

Enable all CATs at once with the `all-cats` meta-feature.

```toml
# Default features include Bradford
[dependencies]
farg = "0.4"

# Add a specific CAT
[dependencies]
farg = { version = "0.4", features = ["cat-cat16"] }

# All CATs
[dependencies]
farg = { version = "0.4", features = ["all-cats"] }
```

### Default Priority

`Cat::DEFAULT` resolves to the highest-priority enabled CAT. The priority order is:

1. Bradford (`cat-bradford`) -- included in default features
2. CAT16
3. CAT02
4. CMC CAT2000
5. Von Kries
6. Hunt-Pointer-Estevez
7. Sharp
8. Fairchild
9. CMC CAT97
10. XYZ Scaling (always available, lowest priority)

With default features enabled, `Cat::DEFAULT` is `Cat::BRADFORD`.

## Using CATs with ColorimetricContext

The CAT is one of the three components of a `ColorimetricContext` (along with an illuminant and an
observer). It determines *how* adaptation is performed when moving between white points.

```rust
use farg::{Cat, ColorimetricContext, Illuminant};

// Default context uses Bradford
let default_ctx = ColorimetricContext::new();

// Use a specific CAT
let ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_cat(Cat::CAT16);   // cat-cat16
```

### adapt_to: Transforming Colors

The `adapt_to` method adapts a color from its current context to a target context. The target
context's CAT is used for the transform.

```rust
use farg::{Cat, ColorimetricContext, Illuminant};
use farg::space::Xyz;

let color = Xyz::new(0.95047, 1.0, 1.08883);

// Adapt to D50 with Bradford (default)
let d50_bradford = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50);
let adapted_bradford = color.adapt_to(d50_bradford);

// Adapt to D50 with CAT16
let d50_cat16 = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_cat(Cat::CAT16);
let adapted_cat16 = color.adapt_to(d50_cat16);

// Results differ slightly because the transforms use different matrices
```

`adapt_to` is available on all CIE and physiological color spaces: `Xyz`, `Lab`, `Lch`, `Luv`,
and `Lms`.

### with_context: Relabeling Without Transformation

Use `with_context` to change a color's context metadata without transforming its values. This is
useful when you know the values are already correct for the target context.

```rust
use farg::{ColorimetricContext, Illuminant};
use farg::space::Xyz;

let color = Xyz::new(0.95047, 1.0, 1.08883);
let d50_ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50);

let relabeled = color.with_context(d50_ctx);  // No transformation
let adapted = color.adapt_to(d50_ctx);         // Transforms values
```

## Using the adapt Method Directly

For lower-level control, you can call `adapt` directly on a CAT with explicit white points.

```rust
use farg::Cat;
use farg::space::Xyz;

let cat = Cat::BRADFORD;
let d65_white = Xyz::new(0.95047, 1.0, 1.08883);
let d50_white = Xyz::new(0.96422, 1.0, 0.82521);

let color = Xyz::new(0.4, 0.2, 0.1);
let adapted = cat.adapt(color, d65_white, d50_white);
```

Adapting to the same white point returns the original color unchanged:

```rust
use farg::Cat;
use farg::space::Xyz;

let cat = Cat::BRADFORD;
let white = Xyz::new(0.95047, 1.0, 1.08883);
let color = Xyz::new(0.4, 0.2, 0.1);

let same = cat.adapt(color, white, white);
// same == color (within floating-point precision)
```

## Inspecting a CAT

### Name and Display

```rust
use farg::Cat;

let cat = Cat::BRADFORD;

println!("{}", cat.name());   // "Bradford"
println!("{cat}");            // name + matrix with default precision
println!("{cat:.2}");         // name + matrix with 2 decimal places
```

### Matrix Access

Each CAT stores a forward matrix and its precomputed inverse.

```rust
use farg::Cat;

let cat = Cat::BRADFORD;

let matrix = cat.matrix();    // 3x3 forward transform
let inverse = cat.inverse();  // 3x3 inverse transform

// Access raw data
let row0 = matrix.data()[0];  // [0.8951, 0.2664, -0.1614]
```

## Creating Custom CATs

Define a custom CAT from any 3x3 matrix. The inverse is computed automatically.

```rust
use farg::Cat;

let my_cat = Cat::new("My Transform", [
    [0.8951, 0.2664, -0.1614],
    [-0.7502, 1.7135, 0.0367],
    [0.0389, -0.0685, 1.0296],
]);

println!("{}", my_cat.name()); // "My Transform"
```

### Using a Custom CAT in a Context

```rust
use farg::{Cat, ColorimetricContext, Illuminant};
use farg::space::Xyz;

let my_cat = Cat::new("Custom", [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
]);

let ctx = ColorimetricContext::new()
    .with_illuminant(Illuminant::D50)
    .with_cat(my_cat);

let color = Xyz::new(0.95047, 1.0, 1.08883);
let adapted = color.adapt_to(ctx);
```

## Choosing a CAT

| Transform            | Best for                                           |
|----------------------|----------------------------------------------------|
| Bradford             | General purpose, ICC profiles, most workflows      |
| CAT16                | CAM16 color appearance model                       |
| CAT02                | CIECAM02 color appearance model                    |
| Von Kries            | Simple diagonal adaptation, educational use        |
| Hunt-Pointer-Estevez | Physiological cone-response research               |
| Sharp                | Improved sharpened cone space (Susstrunk et al.)   |
| XYZ Scaling          | Simplest possible adaptation, testing              |
| CMC CAT2000          | CMC 2000 color appearance framework                |
| CMC CAT97            | CMC 1997 color appearance framework                |
| Fairchild            | Fairchild color appearance research                |

Bradford is the recommended default for most applications and is used by ICC color management.

## Feature Flags

| Feature                    | Transform            | Default |
|----------------------------|----------------------|---------|
| *(always available)*       | XYZ Scaling          | Yes     |
| `cat-bradford`             | Bradford             | Yes     |
| `cat-cat02`                | CAT02                | No      |
| `cat-cat16`                | CAT16                | No      |
| `cat-cmc-cat97`            | CMC CAT97            | No      |
| `cat-cmc-cat2000`          | CMC CAT2000          | No      |
| `cat-fairchild`            | Fairchild            | No      |
| `cat-hunt-pointer-estevez` | Hunt-Pointer-Estevez | No      |
| `cat-sharp`                | Sharp                | No      |
| `cat-von-kries`            | Von Kries            | No      |
| `all-cats`                 | All 10 transforms    | No      |

```toml
# Default features include Bradford
[dependencies]
farg = "0.4"

# Add specific CATs
[dependencies]
farg = { version = "0.4", features = ["cat-cat16", "cat-von-kries"] }

# All CATs
[dependencies]
farg = { version = "0.4", features = ["all-cats"] }
```
