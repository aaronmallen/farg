---
id: 0005
title: Standard Observer System
status: active
tags: [architecture, colorimetry, spectral, features]
created: 2026-02-04
superseded-by:
---

# ADR-0005: Standard Observer System

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

The library provides a standard observer system representing how human vision perceives color. The CIE 1931
2° observer is always compiled as the baseline, while alternative observers (CIE 1964 10°, CIE 2006,
Stockman-Sharpe, Judd corrections) are feature-gated. Observers encapsulate color matching functions,
cone fundamentals, and chromaticity coordinates as static spectral data tables.

## Context

Color perception varies based on the observer model used. The CIE (Commission Internationale de
l'Eclairage) has defined several standard observers over the decades:

| Observer           | Year | Visual Field | Notes                                       |
|--------------------|------|--------------|---------------------------------------------|
| CIE 1931           | 1931 | 2°           | Original standard, most widely used         |
| CIE 1931 Judd      | 1951 | 2°           | Judd's correction for short wavelengths     |
| CIE 1931 Judd-Voss | 1978 | 2°           | Further refinement of Judd's correction     |
| CIE 1964           | 1964 | 10°          | Supplementary observer for larger fields    |
| CIE 2006           | 2006 | 2°/10°       | Physiologically-based, cone fundamental     |
| Stockman-Sharpe    | 2000 | 2°/10°       | High-precision cone fundamentals            |

Each observer includes:

- **Color Matching Functions (CMF)**: Tristimulus response curves (x̄, ȳ, z̄)
- **Cone Fundamentals**: LMS cone response curves
- **Chromaticity Coordinates**: xy coordinates derived from CMFs

This data is substantial—each observer adds 200-4000 spectral samples at 1nm or sub-nanometer resolution,
resulting in ~2,500-4,000 lines of static data per observer.

The CIE 1931 2° observer remains the industry standard for most color workflows (sRGB, ICC profiles,
web colors). Other observers are needed for specialized applications: large-field viewing (10°),
physiologically-accurate models (CIE 2006), or high-precision research (Stockman-Sharpe).

## Decision

### CIE 1931 2° as Unconditional Baseline

The CIE 1931 2° standard observer is always compiled without feature gates:

```rust
// src/observer/cie_1931_2d.rs - no feature gate
mod cie_1931_2d;
```

Rationale:

1. It's the most widely used observer in color science
2. Most color spaces (sRGB, Adobe RGB, Display P3) are defined relative to it
3. Ensures a working observer exists even with `default-features = false`

### Feature-Gated Alternative Observers

All other observers require explicit opt-in per ADR-0002:

```toml
[features]
observer-cie-1931-judd-2d = []
observer-cie-1931-judd-vos-2d = []
observer-cie-1964-10d = []
observer-cie-2006-2d = []
observer-cie-2006-10d = []
observer-stockman-sharpe-2d = []
observer-stockman-sharpe-10d = []
all-observers = [
  "observer-cie-1931-judd-2d",
  "observer-cie-1931-judd-vos-2d",
  "observer-cie-1964-10d",
  # ...
]
```

### Observer Structure

The `Observer` type encapsulates all spectral data for a standard observer:

```rust
pub struct Observer {
    name: &'static str,
    visual_field: f64,           // Degrees (2.0 or 10.0)
    cmf: Cmf,                    // Color matching functions
    chromaticity_coordinates: ChromaticityCoordinates,
    cone_fundamentals: ConeFundamentals,
    age: Option<u8>,             // For age-modified observers
}
```

### Static Data Strategy

Spectral data is stored as static arrays and leaked to obtain `'static` references:

```rust
static CMF_DATA: [(u32, TristimulusResponse); 471] = [
    (360, TristimulusResponse::new(0.000129900000, 0.000003917000, 0.000606100000)),
    (361, TristimulusResponse::new(0.000145847000, 0.000004393581, 0.000680879200)),
    // ... 469 more entries
];

pub fn build(&self) -> Observer {
    let cmf_data: Box<[(u32, TristimulusResponse)]> = self.cmf
        .iter()
        .map(|(wavelength, [x, y, z])| (*wavelength, TristimulusResponse::new(*x, *y, *z)))
        .collect();
    let cmf = Cmf::new(Box::leak(cmf_data));
    // ...
}
```

This approach:

- Keeps spectral data in static memory (no runtime allocation for lookups)
- Allows the builder to transform raw arrays into typed wrappers
- Trades some memory for zero-cost access patterns

### Builder Pattern

Observers are constructed via a builder to handle optional data:

```rust
Observer::builder("CIE 1931", 2.0)
    .with_cmf(&CMF_DATA)
    .with_chromaticity_coordinates(&CHROMATICITY_DATA)  // Optional: derived from CMF if omitted
    .with_cone_fundamentals(&CONE_DATA)                 // Optional: derived from CMF if omitted
    .with_age(32)                                       // Optional: for Fairchild modification
    .build()
```

## Implementation Phases

- **Phase 1: Core Observer Type** - `Observer` struct with CMF, cone fundamentals, chromaticity
- **Phase 2: CIE 1931 2°** - Baseline observer, always compiled
- **Phase 3: Alternative Observers** - Feature-gated CIE 1964, CIE 2006, Stockman-Sharpe, Judd
- **Phase 4: Fairchild Age Modification** - Age-dependent observer customization

## Consequences

### Positive

- **Minimal default footprint**: Only CIE 1931 2° compiled by default (~2,800 lines)
- **Comprehensive coverage**: Full suite of CIE standard observers available
- **Type-safe spectral access**: CMF, cone fundamentals, chromaticity coordinates are strongly typed
- **Future extensibility**: Builder pattern supports additional modifications (age, field size)

### Negative

- **Large source files**: Each observer is 2,500-4,000 lines of static data
- **Memory leak by design**: `Box::leak` prevents deallocation (acceptable for static data)
- **Binary size growth**: Each observer adds ~50-100KB to the binary

## Open Questions

- Should observers provide wavelength interpolation for non-integer queries?
- Should we support runtime-loaded observer data from external files?

## Future Work

### Fairchild Age Modification

The `age` field on `Observer` is groundwork for Mark Fairchild's age-dependent observer model. Human
color vision changes with age due to:

- Yellowing of the crystalline lens
- Decreased pupil size
- Reduced macular pigment density

The Fairchild modification adjusts color matching functions based on observer age, enabling more
accurate color matching for older observers. This will be implemented as a builder method:

```rust
Observer::builder("CIE 1931", 2.0)
    .with_cmf(&CMF_DATA)
    .with_age(65)
    .with_fairchild_modification()  // Future: applies age-based CMF adjustment
    .build()
```

### RGB Color Matching Functions

Some historical observers use RGB color matching functions (r̄, ḡ, b̄) with RG chromaticity coordinates
rather than the XYZ tristimulus system. Notable examples include:

- **Stiles & Burch (1955) 2°**: RGB CMFs derived from pilot experiments
- **Stiles & Burch (1959) 10°**: Large-field RGB observer, basis for CIE 1964

These observers define color matches in terms of three real primaries (monochromatic red, green, blue),
which can result in negative CMF values for certain wavelengths—physically meaningful since negative
values indicate the primary must be added to the test stimulus rather than the match field.

We plan to support RGB-based observers in the future, though the implementation approach is not yet
determined. Key considerations include:

- Whether to store RGB CMFs directly or derive them from XYZ via transformation matrices
- How to represent RG chromaticity coordinates alongside xy coordinates
- Whether the `Observer` struct needs generalization or if RGB observers warrant a separate type

## References

- [CIE 015:2018 - Colorimetry](https://cie.co.at/publications/colorimetry-4th-edition)
- [CIE 170-1:2006 - Fundamental Chromaticity Diagram][cie-170-1]
- [Stockman & Sharpe (2000) - Cone Fundamentals](https://www.cvrl.org/)
- Fairchild, M.D. (2013). *Color Appearance Models*, 3rd ed. Wiley.
- ADR-0002: Feature-Gated Components
- ADR-0004: Spectral Data Architecture

[cie-170-1]: https://cie.co.at/publications/fundamental-chromaticity-diagram-physiological-axes-part-1
