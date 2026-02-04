---
id: 0001
title: Component Precision
status: active
tags: [core, precision, numerics]
created: 2026-02-04
superseded-by:
---

# ADR-0001: Component Precision

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

All color space components use a `Component` newtype wrapping `f64` for internal representation. This type
accepts any standard numeric input via `From` implementations, enabling ergonomic construction while
maintaining precision for color science calculations.

## Context

Color science involves matrix multiplications, gamma corrections, and iterative algorithms where numerical
precision matters. At the same time, users constructing colors expect natural syntax—they shouldn't need to
write `255.0_f64` when `255` is obviously an integer that can be promoted.

The tension is between:

1. **Precision**: f64 provides sufficient precision for colorimetric calculations (15-17 significant digits)
2. **Ergonomics**: Users want to write `srgb(255, 0, 0)` not `srgb(255.0, 0.0, 0.0)`
3. **Simplicity**: Avoiding generics over `f32`/`f64` reduces complexity and compile times

Color libraries in other languages often use f32 (sufficient for display) or make precision a generic
parameter (complex API). We choose f64 everywhere because:

- Modern CPUs handle f64 efficiently (often same throughput as f32)
- Intermediate calculations benefit from extra precision
- Output to f32 for GPU/image use is a simple cast at the boundary

## Decision

### The Component Type

A newtype wrapper around `f64`:

```rust
#[derive(Clone, Copy, Debug)]
pub struct Component(pub(crate) f64);
```

The inner value is `pub(crate)` to allow direct access within the library while maintaining encapsulation
for external users.

### Universal Numeric Input

`Component` implements `From<T>` for all standard numeric types:

| Type Family | Types                                     |
|-------------|-------------------------------------------|
| Floats      | f32, f64                                  |
| Signed      | i8, i16, i32, i64, i128, isize            |
| Unsigned    | u8, u16, u32, u64, u128, usize            |

This enables APIs that accept `impl Into<Component>`:

```rust
impl Xyz {
    pub fn new(
        x: impl Into<Component>,
        y: impl Into<Component>,
        z: impl Into<Component>,
    ) -> Self {
        // ...
    }
}

// All of these work:
let a = Xyz::new(0.95047, 1.0, 1.08883);     // f64
let b = Xyz::new(0.95047_f32, 1.0, 1.08883); // mixed f32/f64
let c = Xyz::new(0, 1, 0);                   // integers
let d = Xyz::new(95047_u32, 100000, 108883); // large integers
```

### Arithmetic Operations

`Component` implements standard arithmetic traits with generic right-hand sides:

```rust
impl<T> Add<T> for Component where T: Into<Self> {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Self(self.0 + rhs.into().0)
    }
}
```

This allows natural arithmetic:

```rust
let c = Component::new(1.0);
let result = c + 0.5;         // f64
let result = c * 2;           // integer
let result = c / 3.0_f32;     // f32
```

Supported operations: `Add`, `AddAssign`, `Sub`, `SubAssign`, `Mul`, `MulAssign`, `Div`, `DivAssign`, `Neg`

### Comparison

`PartialEq` and `PartialOrd` compare the underlying f64 values. Note that `Component` does **not** implement
`Eq` or `Ord` because f64 has NaN values.

```rust
impl<T> PartialEq<T> for Component where T: Into<Self> + Copy {
    fn eq(&self, other: &T) -> bool {
        self.0 == (*other).into().0
    }
}
```

### Display

Display uses configurable precision, defaulting to 4 decimal places:

```rust
impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:.precision$}", self.0, precision = f.precision().unwrap_or(4))
    }
}
```

Usage:

```rust
let c = Component::new(1.23456789);
format!("{}", c)      // "1.2346" (default)
format!("{:.2}", c)   // "1.23"
format!("{:.6}", c)   // "1.234568"
```

### Const Construction

For compile-time color definitions, `Component::new_const` accepts only f64. Color spaces expose their own
`new_const` functions that wrap this internally:

```rust
impl Component {
    pub const fn new_const(value: f64) -> Self {
        Self(value)
    }
}

impl Xyz {
    pub const fn new_const(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Component::new_const(x),
            y: Component::new_const(y),
            z: Component::new_const(z),
        }
    }
}

// Enables const color definitions
const D65_WHITE: Xyz = Xyz::new_const(0.95047, 1.0, 1.08883);
```

## Implementation Phases

- **Phase 1: Core Type** - Implement Component with From traits and arithmetic
- **Phase 2: Integration** - Use Component in color space structs
- **Phase 3: Specialized Methods** - Add utility methods (e.g., `clamp`, `abs`)

## Consequences

### Positive

- **Ergonomic API**: Users write `srgb(255, 0, 0)` instead of `srgb(255.0, 0.0, 0.0)`
- **Consistent precision**: All calculations use f64, eliminating precision-related bugs
- **No generic complexity**: Library code is simpler without `T: Float` bounds everywhere
- **Compile-time support**: `new_const` enables const color definitions

### Negative

- **Memory overhead**: f64 uses 8 bytes per component vs 4 for f32 (24 bytes vs 12 bytes for RGB)
- **Conversion cost**: Integer/f32 inputs convert to f64 (negligible in practice)
- **No f32 optimization path**: Applications that only need f32 precision still pay for f64

## Open Questions

- Should `Component` implement approximate equality (within epsilon) for floating-point comparisons?

### Rejected Alternatives

**Angular Component Type**: A specialized `AngularComponent` that automatically wraps values to 0-360° was
considered but rejected. Color spaces with angular components (like hue in Oklch) handle wrapping in their
own methods. Additionally, automatic wrapping cannot be implemented in `const fn` because `rem_euclid` is
not const-stable, which would prevent const construction of colors with hue components.

## Future Work

- **SIMD Batch Operations**: Vectorized operations for processing arrays of components

## References

- [IEEE 754 Double Precision](https://en.wikipedia.org/wiki/Double-precision_floating-point_format)
