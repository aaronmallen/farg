---
id: 0004
title: Spectral Data Architecture
status: active
tags: [architecture, spectral, colorimetry]
created: 2026-02-04
superseded-by:
---

# ADR-0004: Spectral Data Architecture

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Spectral data (SPDs, CMFs, cone fundamentals) are represented as static lookup tables with wavelength
keys. A common `Table` trait provides shared operations, while type-specific structs add domain methods
for spectral integration and color calculation.

## Context

Color science at the colorimetric level requires working with spectral data:

- **Spectral Power Distributions (SPD)**: How much power a light source emits at each wavelength
- **Color Matching Functions (CMF)**: How the human visual system responds to each wavelength (XYZ)
- **Cone Fundamentals**: Direct LMS cone sensitivity curves
- **Chromaticity Coordinates**: Spectral locus points for gamut visualization

These all share a common structure: a table mapping wavelengths (integers in nanometers) to response
values. However, they differ in:

1. **Value types**: SPD uses `f64`, CMF uses `TristimulusResponse` (XYZ), cone fundamentals use
  `ConeResponse` (LMS)
2. **Domain operations**: CMF can integrate with SPD to produce XYZ; cone fundamentals produce LMS
3. **Conversions**: CMF can be transformed to cone fundamentals or chromaticity coordinates

The design challenge is balancing code reuse (shared table operations) with type safety (preventing
nonsensical operations like integrating two CMFs together).

## Decision

### The Table Trait

A trait defines common operations for all spectral data types:

```rust
pub trait Table {
    type Value;

    fn table(&self) -> &[(u32, Self::Value)];

    // Provided methods
    fn at(&self, wavelength: u32) -> Option<&Self::Value>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn min_wavelength(&self) -> Option<u32>;
    fn max_wavelength(&self) -> Option<u32>;
    fn step(&self) -> u32;
    fn values(&self) -> impl Iterator<Item = &Self::Value>;
    fn wavelengths(&self) -> impl Iterator<Item = u32>;
}
```

Implementors only provide `table()`, getting all other operations for free.

### Type-Specific Structs

Each spectral data type wraps a static table reference:

```rust
pub struct SpectralPowerDistribution(&'static [(u32, f64)]);
pub struct ColorMatchingFunction(&'static [(u32, TristimulusResponse)]);
pub struct ConeFundamentals(&'static [(u32, ConeResponse)]);
pub struct ChromaticityCoordinates(&'static [(u32, Xy)]);
```

Using `&'static` enables:

- Compile-time construction of standard data (CIE 1931, D65, etc.)
- Zero-cost references to built-in spectral data
- No runtime allocation for standard illuminants/observers

### Response Value Types

Separate types for tristimulus (XYZ) and cone (LMS) responses at a wavelength:

```rust
pub struct TristimulusResponse([f64; 3]);  // X, Y, Z
pub struct ConeResponse([f64; 3]);          // L, M, S
```

These are distinct from `Xyz` and `Lms` color space structs because they represent per-wavelength
sensitivity coefficients, not actual color values. Keeping them separate prevents confusion and enables
specialized Display formatting.

### Integration Operations

CMF provides the core integration method:

```rust
impl ColorMatchingFunction {
    pub fn spectral_power_distribution_to_xyz(&self, spd: &Spd) -> Xyz {
        let step = self.step() as f64;
        let mut components = [0.0; 3];

        for (wavelength, response) in self.table() {
            if let Some(&power) = spd.at(*wavelength) {
                let xyz = response.components();
                components[0] += power * xyz[0] * step;
                components[1] += power * xyz[1] * step;
                components[2] += power * xyz[2] * step;
            }
        }

        Xyz::from(components)
    }
}
```

The step-weighted Riemann sum is standard for spectral integration.

### Conversions Between Types

CMF can be converted to other spectral types:

- `ChromaticityCoordinates::from(cmf)` - Projects XYZ to xy for spectral locus
- `ConeFundamentals::from(cmf)` - Transforms XYZ to LMS using chromatic adaptation

These conversions use `Box::leak` to create static references, enabling the converted data to have
the same lifetime characteristics as built-in data.

### Module Structure

```text
src/spectral/
├── mod.rs                         # Table trait + re-exports
├── chromaticity_coordinates.rs    # Spectral locus
├── color_matching_function.rs     # CMF (XYZ observer)
├── cone_fundamentals.rs           # LMS observer
├── cone_response.rs               # Per-wavelength LMS
├── spectral_power_distribution.rs # SPD (illuminant/light)
└── tristimulus_response.rs        # Per-wavelength XYZ
```

## Implementation Phases

- **Phase 1: Core Types** - Table trait, SPD, CMF, basic integration
- **Phase 2: Observers** - Built-in CIE 1931, 1964, 2006 observers
- **Phase 3: Illuminants** - Built-in D50, D65, A, F series SPDs
- **Phase 4: Advanced** - Spectral interpolation, custom observer construction

## Consequences

### Positive

- **Type safety**: Can't accidentally integrate CMF with CMF or SPD with SPD
- **Zero-cost standard data**: Built-in spectral data has no runtime allocation
- **Shared operations**: Common table operations implemented once via trait
- **Clear semantics**: Response types distinguish per-wavelength data from color values

### Negative

- **Static data requirement**: Custom spectral data must be leaked to get `&'static`
- **Memory for conversions**: `Box::leak` means converted data lives forever
- **Fixed wavelength type**: `u32` nanometers may not suit all applications (sub-nm precision)

## Open Questions

- Should we support interpolation for wavelengths not in the table?
- Should custom (non-static) spectral data be supported via a separate API?

## Future Work

- **Spectral interpolation**: Linear/spline interpolation for missing wavelengths
- **Spectral arithmetic**: Operations like SPD scaling, SPD multiplication
- **Spectral rendering**: Convert RGB to/from spectral representations

## References

- [CIE 015:2018](https://cie.co.at/publications/colorimetry-4th-edition) - Colorimetry standard
- ADR-0003: Transform Module Architecture - Integration functions follow transform patterns
