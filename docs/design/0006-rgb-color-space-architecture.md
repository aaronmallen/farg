---
id: 0006
title: RGB Color Space Architecture
status: active
tags: [architecture, rgb, color-space]
created: 2026-02-04
superseded-by:
---

# ADR-0006: RGB Color Space Architecture

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

RGB color spaces are represented as phantom-typed `Rgb<S: RgbSpec>` structs where the type parameter provides
compile-time differentiation between color spaces (sRGB, Display P3, Adobe RGB). The `RgbSpec` trait encapsulates color
space parameters as associated constants (primaries, transfer function, white point), enabling const construction and
zero-cost abstractions. A separate `LinearRgb<S>` type prevents accidental mixing of linear and gamma-encoded values.

## Context

RGB color spaces vary in three fundamental ways:

1. **Primaries**: The chromaticity coordinates defining the red, green, and blue primaries
2. **Transfer Function**: The gamma curve or electro-optical transfer function (EOTF)
3. **White Point**: The reference illuminant (typically D50 or D65)

For example:

| Color Space | Primaries | Transfer Function | White Point |
|-------------|-----------|-------------------|-------------|
| sRGB        | BT.709    | sRGB (γ≈2.2)      | D65         |
| Display P3  | P3        | sRGB (γ≈2.2)      | D65         |
| Adobe RGB   | Adobe     | Gamma 2.2         | D65         |
| ProPhoto    | ROMM      | Gamma 1.8         | D50         |

The design challenge is balancing:

- **Type safety**: Preventing accidental mixing of color values from different spaces (e.g., adding sRGB to Display P3)
- **Ergonomics**: Users shouldn't need to constantly specify which color space they're using
- **Performance**: Color space conversions should be zero-cost when possible
- **Extensibility**: Adding new color spaces should be straightforward

Additional complexity arises from the distinction between gamma-encoded RGB (display-ready) and linear RGB (for
physically-based operations like blending and tone mapping). These must be kept separate at the type level to prevent
errors.

## Decision

### Phantom-Typed `Rgb<S>` Pattern

The core RGB type uses a phantom type parameter to encode the color space specification:

```rust
pub struct Rgb<S = Srgb>
where
  S: RgbSpec,
{
  r: Component,
  g: Component,
  b: Component,
  context: ColorimetricContext,
  _spec: PhantomData<S>,
}
```

Key characteristics:

- **Type-level differentiation**: `Rgb<Srgb>` and `Rgb<DisplayP3>` are distinct types
- **Default to sRGB**: Most users work with sRGB, so it's the default
- **Zero runtime cost**: `PhantomData<S>` is zero-sized
- **Prevents accidental mixing**: `Rgb<Srgb> + Rgb<DisplayP3>` is a type error

The alternative approach of using a runtime enum discriminator (`enum RgbSpace { Srgb, DisplayP3, ... }`) would require:

- Runtime checks for every operation
- Less clarity about which operations are valid between different spaces
- No compile-time guarantees about color space consistency

### `RgbSpec` Trait with Associated Constants

Each color space implements a specification trait:

```rust
pub trait RgbSpec: Clone + Copy + Send + Sync {
  const CONTEXT: ColorimetricContext;
  const NAME: &'static str;
  const PRIMARIES: RgbPrimaries;
  const TRANSFER_FUNCTION: TransferFunction;

  fn xyz_matrix() -> &'static Matrix3;
  fn inversed_xyz_matrix() -> &'static Matrix3;
}
```

Example implementation for sRGB:

```rust
impl RgbSpec for Srgb {
  const CONTEXT: ColorimetricContext = ColorimetricContext::new()
    .with_illuminant(Illuminant::D65)
    .with_observer(Observer::CIE_1931_2D);
  const NAME: &'static str = "sRGB";
  const PRIMARIES: RgbPrimaries = RgbPrimaries::new_const(
    Xy::new_const(0.64, 0.33),  // Red
    Xy::new_const(0.30, 0.60),  // Green
    Xy::new_const(0.15, 0.06),  // Blue
  );
  const TRANSFER_FUNCTION: TransferFunction = TransferFunction::Srgb;
}
```

Using associated constants rather than methods enables:

- Const evaluation for static RGB colors
- Zero-cost access to space parameters
- Clear compile-time guarantees about space properties

### `OnceLock` for XYZ Transformation Matrices

The RGB-to-XYZ conversion matrix is computed lazily on first use:

```rust
fn xyz_matrix() -> &'static Matrix3 {
  static MATRIX: OnceLock<Matrix3> = OnceLock::new();
  MATRIX.get_or_init(|| Self::PRIMARIES.calculate_xyz_matrix(Self::CONTEXT.reference_white()))
}
```

The matrix calculation involves:

1. Converting primaries from xy chromaticity to XYZ tristimulus
2. Solving for scaling factors that normalize to the white point
3. Constructing a 3×3 transformation matrix

This is too complex for const evaluation in current Rust. The `OnceLock` pattern provides:

- Lazy computation only when needed
- Thread-safe initialization
- Amortized zero cost after first use

Precomputing and hardcoding matrices would be brittle—primaries or white points might be adjusted, and keeping the
matrices in sync would be error-prone.

### `TransferFunction` as Closed Enum

Transfer functions are represented as a closed enum:

```rust
pub enum TransferFunction {
  Srgb,        // sRGB/BT.709 piecewise curve
  Bt601,       // BT.601 (same as BT.709)
  Bt709,       // BT.709 piecewise curve
  Pq,          // PQ (ST 2084) HDR
  Hlg,         // HLG (Hybrid Log-Gamma) HDR
  ProPhotoRgb, // ProPhoto RGB piecewise
  Gamma(f64),  // Simple power curve
  Linear,      // No transfer function
}

impl TransferFunction {
  pub fn encode(&self, linear: f64) -> f64;
  pub fn decode(&self, encoded: f64) -> f64;
}
```

Each variant has a corresponding encode/decode implementation. The closed enum approach provides:

- Pattern matching exhaustiveness checks
- Clear display formatting
- Efficient dispatch via match statements
- Known memory layout

The trade-off is that users cannot define custom transfer functions without modifying the library. An alternative
trait-based approach would enable open extension:

```rust
pub trait TransferFunction {
  fn encode(&self, linear: f64) -> f64;
  fn decode(&self, encoded: f64) -> f64;
}
```

However, this would require:

- Dynamic dispatch or monomorphization for every encode/decode
- No const construction for color spaces with custom functions
- More complex ergonomics for the common case

The closed enum is preferred because:

- Standard transfer functions cover 99% of use cases
- Custom transfer functions are rare in practice
- Users needing custom functions can convert through XYZ

### `LinearRgb<S>` as Separate Type

Linear RGB is a distinct type from gamma-encoded RGB:

```rust
pub struct LinearRgb<S>
where
  S: RgbSpec,
{
  r: Component,
  g: Component,
  b: Component,
  _spec: PhantomData<S>,
}
```

Conversions are explicit:

```rust
let encoded: Rgb<Srgb> = Rgb::new(128, 64, 32);
let linear: LinearRgb<Srgb> = encoded.to_linear();
let back: Rgb<Srgb> = linear.to_encoded();
```

This prevents errors like:

```rust
// Compile error: mismatched types
let wrong = Rgb::new(255, 0, 0) + LinearRgb::from_normalized(0.5, 0.5, 0.5);
```

The separation is critical because:

- **Blending**: Must be done in linear space to be physically accurate
- **Tone mapping**: HDR operations require linear values
- **Color grading**: Many operations assume linear light

A single type with a runtime flag (`is_linear: bool`) would be error-prone and lose compile-time guarantees.

### Module Structure

```text
src/space/rgb/
├── mod.rs             # Re-exports
├── space.rs           # Rgb<S> struct
├── space/
│   └── standard.rs    # Srgb implementation
├── linear.rs          # LinearRgb<S> struct
├── spec.rs            # RgbSpec trait
├── primaries.rs       # RgbPrimaries struct
└── transfer.rs        # TransferFunction enum
```

Standard color spaces (sRGB, Display P3, Adobe RGB, etc.) live in `space/standard.rs` and are always available. Future
ADRs may introduce feature-gated RGB spaces for specialized uses.

## Implementation Phases

- **Phase 1: Core Types** - `Rgb<S>`, `LinearRgb<S>`, `RgbSpec` trait, `TransferFunction`
- **Phase 2: Standard sRGB** - `Srgb` spec implementation
- **Phase 3: XYZ Conversions** - RGB ↔ XYZ using transformation matrices
- **Phase 4: Additional Spaces** - Display P3, Adobe RGB, ProPhoto RGB, BT.2020

## Consequences

### Positive

- **Type safety**: Cannot mix RGB values from different color spaces at compile time
- **Zero-cost abstractions**: PhantomData and associated constants compile to zero overhead
- **Const construction**: Color constants can be evaluated at compile time
- **Clear semantics**: Linear vs. encoded distinction enforced by type system
- **Easy extension**: New color spaces require only a `RgbSpec` implementation
- **Performance**: Matrix operations cached via `OnceLock`, no repeated computation

### Negative

- **Closed transfer functions**: Custom transfer functions require library modification
- **Type complexity**: Generic bounds propagate through the codebase (`S: RgbSpec`)
- **User confusion**: Novice users may not understand why they need to specify `<Srgb>` sometimes
- **Boilerplate**: Each standard space requires a small boilerplate `RgbSpec` implementation
- **Matrix initialization**: First conversion has a one-time initialization cost (acceptable but non-zero)

## Open Questions

- Should we provide a convenience type alias `type Srgb = Rgb<Srgb>` to reduce verbosity?
- Should custom transfer functions be supported via a separate API?
- Should we support non-const color spaces defined at runtime?

## Future Work

### Additional Standard Spaces

Future implementations will add:

- Display P3 (wider gamut for modern displays)
- Adobe RGB (1998) (photography standard)
- ProPhoto RGB (ROMM RGB) (maximum gamut)
- Rec. 2020 (BT.2020) (HDR/UHD TV)
- DCI-P3 (digital cinema)

### Color Space Conversion Optimization

Currently, all RGB-to-RGB conversions go through XYZ:

```text
Rgb<Srgb> → Xyz → Rgb<DisplayP3>
```

For spaces with the same primaries (e.g., sRGB and BT.709 have identical primaries, differing only in transfer
function), a direct conversion would be more efficient:

```rust
Rgb<Srgb> → LinearRgb<Srgb> → LinearRgb<Bt709> → Rgb<Bt709>
```

This could be implemented as a trait method on `RgbSpec`:

```rust
trait RgbSpec {
  fn can_convert_directly<T: RgbSpec>() -> bool;
  fn convert_direct<T: RgbSpec>(rgb: LinearRgb<Self>) -> LinearRgb<T>;
}
```

### Feature-Gated Wide-Gamut Spaces

Per ADR-0002, wide-gamut and HDR color spaces may be feature-gated to reduce default binary size:

```toml
[features]
rgb-display-p3 = []
rgb-adobe = []
rgb-prophoto = []
rgb-rec2020 = []
all-rgb-spaces = ["rgb-display-p3", "rgb-adobe", "rgb-prophoto", "rgb-rec2020"]
```

### Extensible Transfer Functions

For advanced users needing custom transfer functions, we could introduce a trait-based extension:

```rust
pub trait CustomTransferFunction {
  fn encode(&self, linear: f64) -> f64;
  fn decode(&self, encoded: f64) -> f64;
}

pub struct CustomRgb<S, T>
where
  S: RgbSpec<TransferFunction = T>,
  T: CustomTransferFunction,
{ /* ... */ }
```

This would be a separate type from `Rgb<S>` to maintain the performance and const guarantees of the closed enum.

## References

- [IEC 61966-2-1:1999](https://webstore.iec.ch/publication/6169) - sRGB specification
- [ITU-R BT.709-6](https://www.itu.int/rec/R-REC-BT.709) - HDTV color space
- [ITU-R BT.2020-2](https://www.itu.int/rec/R-REC-BT.2020) - UHDTV color space
- [SMPTE ST 2084:2014](https://ieeexplore.ieee.org/document/7291452) - PQ transfer function
- [ITU-R BT.2100-2](https://www.itu.int/rec/R-REC-BT.2100) - HLG and PQ for HDR
- ADR-0002: Feature-Gated Components
- ADR-0004: Spectral Data Architecture
- ADR-0005: Standard Observer System
