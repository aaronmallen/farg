# Color Mixing

How to interpolate between colors and generate gradients using the `ColorSpace` trait.

## Quick Start

Perceptual mixing requires a cylindrical or rectangular color space feature. Oklch is recommended.

```toml
[dependencies]
farg = { version = "0.4", features = ["space-oklch"] }
```

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let coral = Rgb::<Srgb>::new(255, 87, 51);
let teal = Rgb::<Srgb>::new(0, 128, 128);

// Perceptual 50/50 mix in Oklch
let midpoint = coral.mix(teal, 0.5);

// 5-step gradient between the two colors
let gradient = coral.gradient(teal, 5);
```

## Mixing Strategies

Farg offers three interpolation strategies, each suited to a different use case.

### Perceptual (Cylindrical)

`mix` interpolates in Oklch (or CIE LCh) with shortest-arc hue handling per the CSS Color
Level 4 specification. This produces the most perceptually uniform transitions and is the
recommended default.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let red = Rgb::<Srgb>::new(255, 0, 0);
let blue = Rgb::<Srgb>::new(0, 0, 255);

// t=0.0 returns self, t=1.0 returns other
let quarter = red.mix(blue, 0.25);
let half = red.mix(blue, 0.5);
let three_quarters = red.mix(blue, 0.75);

// Values outside 0.0–1.0 extrapolate beyond the endpoints
let extrapolated = red.mix(blue, 1.5);
```

Requires `space-oklch` (preferred) or `space-lch`.

### Rectangular

`mix_rectangular` interpolates directly in Oklab (or CIE L\*a\*b\*) rectangular coordinates.
This avoids hue-interpolation desaturation and handles neutrals naturally, making it a good
choice when mixing colors that pass through or near gray.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let yellow = Rgb::<Srgb>::new(255, 255, 0);
let purple = Rgb::<Srgb>::new(128, 0, 128);

let mixed = yellow.mix_rectangular(purple, 0.5);
```

Requires `space-oklab` (preferred) or `space-lab`.

### Linear-Light

`mix_linear` interpolates in linearized sRGB, producing physically correct additive light
mixing. This matches how light actually combines and is always available with no extra features.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let red = Rgb::<Srgb>::new(255, 0, 0);
let green = Rgb::<Srgb>::new(0, 255, 0);

// Additive light mix — red + green yields yellow-ish
let mixed = red.mix_linear(green, 0.5);
```

No extra feature flags required.

## Gradients

Each mixing strategy has a corresponding gradient method that generates a sequence of
evenly-spaced colors including both endpoints.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let start = Rgb::<Srgb>::new(255, 87, 51);
let end = Rgb::<Srgb>::new(0, 128, 128);

// Perceptual gradient (Oklch)
let perceptual = start.gradient(end, 10);

// Linear-light gradient
let linear = start.gradient_linear(end, 10);

// Rectangular gradient (Oklab)
let rectangular = start.gradient_rectangular(end, 10);
```

When `steps` is 0 the result is empty. When `steps` is 1 the result contains only the
starting color. Both endpoints are always included for `steps >= 2`.

## In-Place Mutation

Every mix method has a `mixed_with` counterpart that modifies the color in place.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let mut color = Rgb::<Srgb>::new(255, 0, 0);
let target = Rgb::<Srgb>::new(0, 0, 255);

// Mutating equivalents
color.mixed_with(target, 0.5);           // perceptual
// color.mixed_with_rectangular(target, 0.5); // rectangular
// color.mixed_with_linear(target, 0.5);      // linear-light
```

## Alpha Interpolation

All mixing methods interpolate the alpha channel alongside color components. The alpha of the
result is the linear interpolation of the two input alphas at the same `t` value.

```rust
use farg::space::{ColorSpace, Rgb, Srgb};

let opaque = Rgb::<Srgb>::new(255, 0, 0);
let transparent = Rgb::<Srgb>::new(0, 0, 255).with_alpha(0.0);

// Alpha at t=0.5 will be 0.5 (midpoint of 1.0 and 0.0)
let half = opaque.mix(transparent, 0.5);
```

## Any Color Type Works

All mixing methods are available on every type that implements `ColorSpace`, so you can call
them on any color type directly.

```rust
use farg::space::{ColorSpace, Xyz};

let a = Xyz::new(0.95047, 1.0, 1.08883);
let b = Xyz::new(0.18048, 0.07219, 0.95030);

let mixed = a.mix(b, 0.5);
let gradient = a.gradient_linear(b, 5);
```

## Feature Flags

| Method                     | Requires               | Fallback    |
|----------------------------|------------------------|-------------|
| `mix`                      | `space-oklch`          | `space-lch` |
| `mixed_with`               | `space-oklch`          | `space-lch` |
| `gradient`                 | `space-oklch`          | `space-lch` |
| `mix_rectangular`          | `space-oklab`          | `space-lab` |
| `mixed_with_rectangular`   | `space-oklab`          | `space-lab` |
| `gradient_rectangular`     | `space-oklab`          | `space-lab` |
| `mix_linear`               | *(always available)*   | —           |
| `mixed_with_linear`        | *(always available)*   | —           |
| `gradient_linear`          | *(always available)*   | —           |

```toml
# Oklch + Oklab (recommended — enables all three strategies)
[dependencies]
farg = { version = "0.4", features = ["space-oklch", "space-oklab"] }

# Or just linear-light mixing with no extra features
[dependencies]
farg = { version = "0.4" }
```
