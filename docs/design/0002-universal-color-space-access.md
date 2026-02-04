---
id: 0002
title: Universal Color Space Access
status: active
tags: [core, api, color-spaces, traits]
created: 2026-02-04
superseded-by:
---

# ADR-0002: Universal Color Space Access

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

The `ColorSpace` trait provides universal access to all color properties and operations across 70+
color spaces. Color spaces implement core conversion methods (`to_xyz` and `From<Xyz>`), and blanket
implementations automatically provide access to properties from any color model (RGB, HSL, Oklch,
etc.) and operations (attenuation, adaptation, etc.). Operations always return `Self`, converting
back to the original color space automatically.

## Context

The farg library supports over 70 color spaces spanning multiple color models: rectangular (RGB,
XYZ), cylindrical (HSL, Oklch), and specialized spaces (CAM16, JzAzBz). Users work in their
preferred color space but need access to properties and operations from other models without manual
conversion overhead.

Traditional approaches present trade-offs:

1. **Manual conversions everywhere**: Users write `color.to_oklch().hue()` or
  `color.to_xyz().attenuate(0.5).to_rgb()`, which is verbose and error-prone
2. **Multiple small traits**: Separate `RgbAccessor`, `CylindricalAccessor`, `Operations` traits
  add complexity without benefit since all spaces need all functionality anyway
3. **Direct implementations**: Implementing every method on every struct duplicates code across 70+
  types with no compile-time API consistency guarantees

The key insight is that with XYZ as the universal interchange format, any color space can provide
any property or operation through automatic conversion. The question is how to structure this
capability ergonomically and maintainably.

## Decision

### Core Principles

1. **Native properties take precedence**: Color spaces that natively support a property return their
  own value. `hsl.hue()` returns the HSL hue, not an Oklch hue. The trait's default implementation
  is a fallback for spaces that don't have a native concept of that property.

2. **Inherent methods + trait delegation**: Methods are implemented directly on structs, and the
  trait implementation delegates to them. This ensures methods work with or without importing the
  trait.

3. **Mutative and immutable variants**: Operations have both mutative (`attenuate`) and immutable
  (`attenuated_by`) versions. Mutative methods modify in place; past-participle `_by` methods
  return a new value. Both use a `set_components` method that each color space implements.

4. **Optimal conversion paths**: Implementations use the fastest available conversion route, not
  necessarily through XYZ. If RGB ↔ HSL is faster than RGB ↔ XYZ ↔ HSL, use the direct path.

### Single ColorSpace Trait

All color space functionality lives in one trait:

```rust
pub trait ColorSpace<const N: usize>: Copy + Clone + From<Xyz> {
  // Methods - core (each space implements these)
  fn components(&self) -> [Component; N];
  fn set_components(&mut self, components: [Component; N]);
  fn to_xyz(&self) -> Xyz;

  // Methods - mutative operations (provided via default impl)
  fn attenuate(&mut self, factor: impl Into<Component>) {
    self.set_components(self.attenuated_by(factor).components());
  }

  fn chromatic_adapt(&mut self, source: Illuminant, target: Illuminant) {
    self.set_components(self.chromatic_adapted_by(source, target).components());
  }

  // Methods - immutable operations (provided via default impl)
  fn attenuated_by(&self, factor: impl Into<Component>) -> Self {
    Self::from(self.to_xyz().attenuated_by(factor))
  }

  fn chromatic_adapted_by(&self, source: Illuminant, target: Illuminant) -> Self {
    Self::from(self.to_xyz().chromatic_adapted_by(source, target))
  }

  // Methods - property accessors (provided via default impl)
  fn blue(&self) -> Component { /* converts to RGB */ }
  fn chroma(&self) -> Component { /* converts to Oklch */ }
  fn green(&self) -> Component { /* converts to RGB */ }
  fn hue(&self) -> Component { /* converts to Oklch */ }
  fn lightness(&self) -> Component { /* converts to Oklch */ }
  fn red(&self) -> Component { /* converts to RGB */ }

  // ... 400+ other accessors and operations
}
```

### Inherent Methods + Trait Delegation

Only the "native" space for an operation implements an inherent method. Other spaces use the trait
default, which converts to the native space automatically.

```rust
// XYZ is the native space for attenuate (3 components)
impl Xyz {
  pub fn attenuated_by(&self, factor: impl Into<Component>) -> Self {
    // Native XYZ attenuation logic
  }

  pub fn components(&self) -> [Component; 3] {
    [self.x, self.y, self.z]
  }

  pub fn set_components(&mut self, components: [Component; 3]) {
    self.x = components[0];
    self.y = components[1];
    self.z = components[2];
  }
}

// From<Xyz> for Xyz is provided by std's blanket impl: impl<T> From<T> for T
impl ColorSpace<3> for Xyz {
  fn components(&self) -> [Component; 3] { Xyz::components(self) }

  fn set_components(&mut self, c: [Component; 3]) { Xyz::set_components(self, c) }

  fn to_xyz(&self) -> Xyz { *self }

  // attenuated_by() uses inherent method via Rust's method resolution
}

// CMYK has 4 components
impl From<Xyz> for Cmyk {
  fn from(xyz: Xyz) -> Self { /* ... */ }
}

impl ColorSpace<4> for Cmyk {
  fn components(&self) -> [Component; 4] { [self.c, self.m, self.y, self.k] }

  fn set_components(&mut self, c: [Component; 4]) { /* ... */ }

  fn to_xyz(&self) -> Xyz { /* ... */ }
}

// RGB is NOT native for attenuate - uses trait default
impl From<Xyz> for Rgb {
  fn from(xyz: Xyz) -> Self { /* ... */ }
}

impl ColorSpace<3> for Rgb {
  fn components(&self) -> [Component; 3] { [self.r, self.g, self.b] }

  fn set_components(&mut self, c: [Component; 3]) { /* ... */ }

  fn to_xyz(&self) -> Xyz { /* ... */ }

  // attenuated_by() NOT overridden - trait default handles it
}

// HSL is the native space for its own hue
impl Hsl {
  pub fn hue(&self) -> Component {
    self.h // returns native HSL hue
  }
}

impl From<Xyz> for Hsl {
  fn from(xyz: Xyz) -> Self { /* XYZ -> RGB -> HSL */ }
}

impl ColorSpace<3> for Hsl {
  fn components(&self) -> [Component; 3] { [self.h, self.s, self.l] }

  fn hue(&self) -> Component {
    Hsl::hue(self) // returns HSL hue, not Oklch hue
  }

  fn set_components(&mut self, c: [Component; 3]) { /* ... */ }

  fn to_xyz(&self) -> Xyz { /* HSL -> RGB -> XYZ */ }
}
```

This pattern ensures:

- Minimal implementation burden (only native spaces implement inherent methods)
- Methods work without importing the trait (`xyz.attenuate(0.5)` always works)
- Generic code works via trait bounds (`fn dim<T: ColorSpace>(c: T) -> T`)
- Native properties return native values (`hsl.hue()` returns HSL hue)

### Universal Access Pattern

Users access any property from any color space:

```rust
// Native properties return native values
let hsl = Hsl::new(180.0, 0.5, 0.7);
let h = hsl.hue();      // Returns HSL hue (180.0) - NO conversion

// Non-native properties convert automatically
let rgb = Rgb::new(255, 0, 0);
let h = rgb.hue();      // RGB -> Oklch, returns Oklch hue

// Access RGB properties from cylindrical space
let oklch = Oklch::new(0.7, 0.15, 180.0);
let r = oklch.red();    // Oklch -> RGB (via optimal path), returns Component

// Access XYZ properties from any space
let xyz_y = oklch.xyz_y();  // Returns XYZ Y component
```

### Operations Return Self

Immutable `*ed_by` operations convert to the appropriate working space, perform the operation, then
convert back to the original space:

```rust
// Attenuation (happens in XYZ)
let rgb = Rgb::new(255, 100, 50);
let dimmed = rgb.attenuated_by(0.5); // Rgb -> XYZ -> attenuate -> Rgb
assert_eq!(type_of(dimmed), "Rgb");

// Chromatic adaptation (happens in XYZ)
let oklch = Oklch::new(0.7, 0.15, 180.0);
let adapted = oklch.chromatic_adapted_by(Illuminant::D65, Illuminant::D50);
assert_eq!(type_of(adapted), "Oklch"); // Stays in Oklch

// Hue rotation (happens in Oklch)
let rgb = Rgb::new(255, 0, 0);
let rotated = rgb.hue_rotated_by(30.0); // Rgb -> Oklch -> rotate -> Rgb
assert_eq!(type_of(rotated), "Rgb");
```

Chain immutable operations:

```rust
let color = Rgb::new(255, 100, 50)
  .attenuated_by(0.8) // returns Rgb
  .chromatic_adapted_by(D65, D50) // returns Rgb
  .hue_rotated_by(15.0) // returns Rgb
  .desaturated_by(0.2); // returns Rgb
```

Or use mutative methods for in-place updates:

```rust
let mut color = Rgb::new(255, 100, 50);
color.attenuate(0.8);
color.rotate_hue(15.0);
```

The naming convention follows the pattern:

| Mutative (imperative) | Immutable (past participle) |
|-----------------------|-----------------------------|
| `attenuate()`         | `attenuated_by()`           |
| `rotate_hue()`        | `hue_rotated_by()`          |
| `chromatic_adapt()`   | `chromatic_adapted_by()`    |
| `desaturate()`        | `desaturated_by()`          |

### Property Access Categories

The trait groups properties by source color model:

| Category      | Properties                          | Source Space |
|---------------|-------------------------------------|--------------|
| Rectangular   | `red`, `green`, `blue`              | RGB          |
| XYZ           | `xyz_x`, `xyz_y`, `xyz_z`           | XYZ          |
| Cylindrical   | `hue`, `chroma`, `lightness`        | Oklch        |
| Perceptual    | `j`, `a`, `b` (CAM16)               | CAM16        |
| Lab-like      | `lab_l`, `lab_a`, `lab_b`           | Oklab        |

### Operations Categories

Each operation has mutative and immutable (past participle `*ed_by`) variants:

| Category       | Mutative                 | Immutable                   | Working Space |
|----------------|--------------------------|-----------------------------|---------------|
| Luminance      | `attenuate`, `brighten`  | `attenuated_by`, etc.       | XYZ           |
| Chromatic      | `chromatic_adapt`        | `chromatic_adapted_by`      | XYZ/Oklch     |
| Hue            | `rotate_hue`             | `hue_rotated_by`            | Oklch         |
| Blending       | `mix`, `blend`           | `mixed_by`, `blended_by`    | Oklab         |
| Transformations| `invert`, `grayscale`    | `inverted_by`, etc.         | Various       |

### Variable Component Counts

The const generic `N` supports color spaces with different numbers of components:

| Components | Color Spaces                          |
|------------|---------------------------------------|
| 1          | Grayscale                             |
| 3          | RGB, XYZ, Oklab, Oklch, HSL, etc.     |
| 4          | CMYK                                  |

## Implementation Phases

- [ ] **Phase 1: Core Trait** - Define `ColorSpace` trait with conversion methods
- [ ] **Phase 2: Basic Spaces** - Implement for XYZ, RGB, Oklch (bootstrap conversions)
- [ ] **Phase 3: Property Accessors** - Add blanket impls for all property accessors
- [ ] **Phase 4: Basic Operations** - Add blanket impls for attenuate, adapt, rotate
- [ ] **Phase 5: Full Coverage** - Add remaining 67 color spaces and all operations
- [ ] **Phase 6: Optimization** - Profile and optimize hot conversion paths

## Consequences

### Positive

- **Simple mental model**: Implement conversion methods, get 400+ methods free
- **Ergonomic API**: `color.red()` works everywhere, no manual conversions
- **No trait import required**: Inherent methods work without `use farg::ColorSpace`
- **Type stability**: Operations return the original color space type, enabling natural chaining
- **Flexible mutation**: Choose immutable chaining or in-place mutation per use case
- **Native property preservation**: `hsl.hue()` returns HSL hue, not a converted value
- **Compile-time consistency**: All color spaces guaranteed to have identical APIs
- **Easy maintenance**: Adding a new property/operation updates all 70+ spaces automatically

### Negative

- **Conversion overhead**: Accessing `rgb.hue()` converts RGB -> Oklch
- **Hidden complexity**: Simple-looking code like `color.red()` may do significant work
- **Large trait**: Single trait with 400+ methods (immutable + mutative) may impact compile times
- **API surface area**: Every operation has two variants (`attenuate` + `attenuate_mut`)

These trade-offs favor ergonomics and maintainability over raw performance. Conversions are
relatively cheap (matrix multiplications and simple math), and users who need maximum performance
can manually convert to the working space once and stay there.

## Open Questions

- Should the trait be sealed to prevent external implementations?

### Alternatives Considered

**Multiple Smaller Traits**: Separate `RgbAccessor`, `CylindricalAccessor`, `Operations` traits
could be composed per color space.

Rejected because:

- All color spaces need all functionality anyway (blanket impls would apply to all)
- More traits = more imports, more complexity for users
- No benefit in terms of implementation burden (still need conversions)
- Harder to maintain API consistency across traits

**Direct Method Implementation Only**: Implement all methods directly on each struct without a trait.

Rejected because:

- No compile-time guarantee that all spaces have the same API
- Generic code (`fn foo<T: ColorSpace>()`) would be impossible
- Adding a new operation requires updating 70+ files manually

Note: We DO use direct method implementation for native operations, but combine it with trait
delegation for API consistency and generic programming support.

**Operations Return Native Space**: Operations could return the space they're performed in (e.g.,
`rgb.attenuate()` returns `Xyz`).

Rejected because:

- Forces users to manually convert back: `rgb.attenuate(0.5).to_rgb()`
- Breaks method chaining in the user's preferred color space
- Mental overhead tracking which space you're in after each operation
- User code becomes verbose and conversion-heavy

**Lazy Conversions**: Store operations and only convert when properties are accessed.

Rejected because:

- Requires complex internal state tracking
- Makes `Copy` trait impossible (needed for ergonomic color handling)
- Delayed error detection (conversion errors appear far from source)
- Higher complexity for marginal performance benefit

## Future Work

- **Conversion caching**: Memoize conversions when chaining multiple operations
- **SIMD optimization**: Vectorized batch conversions for arrays of colors
- **Trait sealing**: Prevent external types from implementing ColorSpace to maintain consistency

## References

- ADR-0001: Component Precision (establishes `Component` type used in accessors)
- [CIE XYZ Color Space](https://en.wikipedia.org/wiki/CIE_1931_color_space) (universal interchange format)
- [Oklab and Oklch](https://bottosson.github.io/posts/oklab/) (perceptual color spaces)
