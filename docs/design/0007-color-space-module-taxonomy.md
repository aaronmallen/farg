---
id: 0007
title: Color Space Module Taxonomy
status: active
tags: [architecture, color-space, modules]
created: 2026-02-08
superseded-by:
---

# ADR-0007: Color Space Module Taxonomy

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Color space modules are organized into category-based subdirectories within `src/space/` to scale to 70+ planned
color spaces. Categories include CIE, Physiological, RGB, Cylindrical, Subtractive, and future categories like
Perceptual, Appearance, HDR, Industrial, and Video. The public API at `farg::space::*` remains unchanged through
re-exports—users still access `farg::space::Xyz`, `farg::space::Hsl`, etc., regardless of internal organization.

## Context

The farg library plans to support over 70 color spaces spanning multiple color models and use cases. As documented in
`docs/PLAN.md`, these include:

- CIE-defined tristimulus spaces (XYZ, xyY, Lab, Luv, LCHab, LCHuv)
- Physiological models (LMS cone response space)
- RGB family (sRGB, Display P3, Adobe RGB, ProPhoto, Rec. 2020, ACES variants, camera gamuts, legacy
  spaces)
- Cylindrical/polar RGB derivatives (HSL, HSV/HSB, HWB, HSI, HSLuv, HPLuv)
- Subtractive/print models (CMY, CMYK)
- Perceptually uniform spaces (Oklab, Oklch, Okhsl, Okhsv, Okhwb, JzAzBz, JzCzhz)
- Color appearance models (CIECAM02, CAM16, CAM16-UCS, HCT, ZCAM)
- HDR/scene-referred spaces (ICtCp with PQ/HLG)
- Video/broadcast spaces (YCbCr, YPbPr, YCoCg, YUV, YIQ, YDbDr)
- Industrial spaces (Hunter Lab, DIN99, DIN99o, OSA-UCS, IPT)

Currently, the `src/space/` directory has 7 non-RGB space files (`xyz.rs`, `lms.rs`, `hsl.rs`, `hsv.rs`,
`hwb.rs`, `cmy.rs`, `cmyk.rs`) plus the `rgb/` subdirectory. A flat directory structure with 70+ files would be:

- **Unnavigable**: Finding related spaces becomes difficult
- **Unmaintainable**: No clear pattern for where new spaces belong
- **Undiscoverable**: Contributors can't find examples of similar spaces
- **Unscalable**: IDEs struggle with large flat directories

The challenge is establishing a taxonomy now while only a few spaces exist, ensuring it supports the full planned
scope without premature categorization.

## Decision

### Category-Based Taxonomy

Color space modules are organized into logical categories reflecting color science domains and use cases:

| Category        | Description                                             | Example Spaces                              |
|-----------------|---------------------------------------------------------|---------------------------------------------|
| `cie`           | CIE-defined tristimulus spaces                          | XYZ, xyY, Lab, Luv, LCHab, LCHuv            |
| `physiological` | Human visual system cone response models                | LMS                                         |
| `rgb`           | RGB color spaces (display, broadcast, cinema, legacy)   | sRGB, Display P3, Adobe RGB, Rec. 2020, etc.|
| `cylindrical`   | Cylindrical/polar RGB derivatives                       | HSL, HSV/HSB, HWB, HSI, HSLuv, HPLuv        |
| `subtractive`   | Subtractive/print color models                          | CMY, CMYK                                   |
| `perceptual`    | Perceptually uniform spaces (future)                    | Oklab, Oklch, Okhsl, Okhsv, JzAzBz, etc.    |
| `appearance`    | Color appearance models (future)                        | CIECAM02, CAM16, CAM16-UCS, HCT, ZCAM       |
| `hdr`           | HDR/scene-referred spaces (future)                      | ICtCp (PQ/HLG variants)                     |
| `industrial`    | Industry-specific spaces (future)                       | Hunter Lab, DIN99, DIN99o, OSA-UCS, IPT     |
| `video`         | Video/broadcast luma-chroma spaces (future)             | YCbCr, YPbPr, YCoCg, YUV, YIQ, YDbDr        |

### Rationale for Categories

The taxonomy reflects:

1. **Color science lineage**: CIE spaces are distinct from manufacturer-specific RGB spaces
2. **Physical vs. perceptual**: XYZ (tristimulus) differs from Lab (perceptual)
3. **Use case domains**: Video industry needs (YCbCr) differ from print (CMYK)
4. **Mathematical structure**: Cylindrical transformations share common patterns
5. **Industry standards**: Appearance models (CAM16) are specialized tools

The RGB category remains separate despite being large (30+ spaces) because:

- RGB spaces share common infrastructure (`RgbSpec` trait, transfer functions, primaries)
- RGB already has its own module structure established in ADR-0006
- RGB is a single color model with parametric variations, not conceptually distinct spaces

### Module Structure

```text
src/space/
├── mod.rs                      # Re-exports all categories with pub use
│
├── cie/
│   ├── mod.rs                  # Re-exports with #[cfg(feature = "...")]
│   ├── xyz.rs
│   ├── xyy.rs                  # Future
│   ├── lab.rs                  # Future
│   ├── luv.rs                  # Future
│   ├── lchab.rs                # Future
│   └── lchuv.rs                # Future
│
├── physiological/
│   ├── mod.rs
│   └── lms.rs
│
├── rgb/
│   ├── mod.rs
│   ├── space.rs                # Rgb<S> struct
│   ├── linear.rs               # LinearRgb<S> struct
│   ├── spec.rs                 # RgbSpec trait
│   ├── primaries.rs
│   ├── transfer.rs
│   └── space/
│       └── standard.rs         # Srgb, DisplayP3, etc.
│
├── cylindrical/
│   ├── mod.rs
│   ├── hsl.rs
│   ├── hsv.rs
│   ├── hwb.rs
│   ├── hsi.rs                  # Future
│   ├── hsluv.rs                # Future
│   └── hpluv.rs                # Future
│
├── subtractive/
│   ├── mod.rs
│   ├── cmy.rs
│   └── cmyk.rs
│
├── perceptual/                 # Future category
│   ├── mod.rs
│   ├── oklab.rs
│   ├── oklch.rs
│   ├── okhsl.rs
│   ├── okhsv.rs
│   ├── okhwb.rs
│   ├── jzazbz.rs
│   └── jzczhz.rs
│
├── appearance/                 # Future category
│   ├── mod.rs
│   ├── ciecam02.rs
│   ├── cam16.rs
│   ├── cam16_ucs.rs
│   ├── hct.rs
│   └── zcam.rs
│
├── hdr/                        # Future category
│   ├── mod.rs
│   └── ictcp.rs
│
├── industrial/                 # Future category
│   ├── mod.rs
│   ├── hunter_lab.rs
│   ├── din99.rs
│   ├── din99o.rs
│   ├── osa_ucs.rs
│   └── ipt.rs
│
└── video/                      # Future category
    ├── mod.rs
    ├── ycbcr.rs
    ├── ypbpr.rs
    ├── ycocg.rs
    ├── yuv.rs
    ├── yiq.rs
    └── ydbdr.rs
```

### Re-Export Strategy

Each category module re-exports its types, and `src/space/mod.rs` re-exports categories with wildcard imports:

```rust
// src/space/cie/mod.rs
mod xyz;
pub use xyz::Xyz;

#[cfg(feature = "space-xyy")]
mod xyy;
#[cfg(feature = "space-xyy")]
pub use xyy::XyY;

// ... other CIE spaces

// src/space/cylindrical/mod.rs
#[cfg(feature = "space-hsl")]
mod hsl;
#[cfg(feature = "space-hsl")]
pub use hsl::Hsl;

#[cfg(feature = "space-hsv")]
mod hsv;
#[cfg(feature = "space-hsv")]
pub use hsv::Hsv;

// ... other cylindrical spaces

// src/space/mod.rs
pub mod cie;
pub use cie::*;

pub mod physiological;
pub use physiological::*;

pub mod rgb;
pub use rgb::*;

pub mod cylindrical;
pub use cylindrical::*;

pub mod subtractive;
pub use subtractive::*;

#[cfg(feature = "space-oklab")]
pub mod perceptual;
#[cfg(feature = "space-oklab")]
pub use perceptual::*;

// ... future categories gated by their features
```

This maintains the flat public API: users still write `farg::space::Xyz`, `farg::space::Hsl`, etc., regardless of
internal organization.

### Feature Gating Within Categories

Per ADR-0003, color spaces are feature-gated with the `space-*` prefix. The feature gating is handled within each
category module:

```rust
// In src/space/cie/mod.rs
#[cfg(feature = "space-xyz")]
mod xyz;
#[cfg(feature = "space-xyz")]
pub use xyz::Xyz;

#[cfg(feature = "space-lab")]
mod lab;
#[cfg(feature = "space-lab")]
pub use lab::Lab;
```

This encapsulates feature complexity within categories and keeps `src/space/mod.rs` cleaner.

### Phased Implementation

Only categories needed for current spaces are created now. Future categories are added when the first space in that
category is implemented:

- **Phase 1 (current)**: `cie`, `physiological`, `rgb`, `cylindrical`, `subtractive`
- **Phase 2**: `perceptual` (when Oklab/Oklch are implemented)
- **Phase 3**: `appearance` (when CAM16/CIECAM02 are implemented)
- **Phase 4**: `hdr`, `industrial`, `video` (as needed)

This avoids premature empty directories while establishing the taxonomy early enough to inform space implementation.

### Boundary Cases

Some spaces could fit in multiple categories:

| Space           | Primary Category | Rationale                                             |
|-----------------|------------------|-------------------------------------------------------|
| LMS             | `physiological`  | Cone response space, not a practical color space      |
| XYZ             | `cie`            | CIE-defined tristimulus values, not perceptual        |
| Lab/Luv         | `cie`            | CIE-defined despite being perceptual                  |
| Oklab/Oklch     | `perceptual`     | Modern perceptually uniform, not CIE-standardized     |
| ICtCp           | `hdr`            | HDR-specific, though physiologically inspired         |
| Hunter Lab      | `industrial`     | Industry-specific variant of Lab                      |
| HSLuv/HPLuv     | `cylindrical`    | Cylindrical despite being based on Luv                |

The categorization prioritizes:

1. **Standardization body** (CIE > industry)
2. **Primary use case** (HDR, video, print)
3. **Mathematical structure** (cylindrical transformations)

## Implementation Phases

- [x] **Phase 1: Establish Categories** - Create `cie`, `physiological`, `cylindrical`, `subtractive`
  subdirectories
- [ ] **Phase 2: Migrate Existing Spaces** - Move `xyz.rs`, `lms.rs`, `hsl.rs`, `hsv.rs`, `hwb.rs`, `cmy.rs`,
  `cmyk.rs` into categories
- [ ] **Phase 3: Update Imports** - Update `src/space/mod.rs` and internal imports
- [ ] **Phase 4: Verify Public API** - Ensure `farg::space::*` unchanged in tests/docs
- [ ] **Phase 5: Document Category Guidelines** - Add contributor guide for categorizing new spaces

## Consequences

### Positive

- **Scales to 70+ spaces**: Clear home for each space without directory bloat
- **Logical grouping**: Related spaces are co-located for easy discovery
- **Navigation**: Contributors can find similar spaces quickly
- **Public API unchanged**: No breaking changes, `farg::space::Xyz` still works
- **Feature gating encapsulated**: Category modules handle conditional compilation
- **Phased approach**: Only create categories as needed, avoiding empty directories
- **Clear contributor guidelines**: New spaces have obvious categories

### Negative

- **One additional indirection**: Imports go through category modules
- **Learning curve**: Contributors must understand taxonomy to add spaces
- **Boundary ambiguity**: Some spaces fit multiple categories, requiring judgment
- **Refactoring burden**: Moving existing 7 space files into categories requires careful migration
- **Module path length**: Internal code uses `crate::space::cie::xyz` instead of `crate::space::xyz`

## Open Questions

- Should we provide a `space::prelude` module with commonly used spaces to reduce import verbosity?
- Should category modules be exposed in the public API (`farg::space::cie::Xyz`) or only the flat re-exports?
- Should we create a contributor guide documenting category selection criteria?

## Future Work

### Category Expansion

As new color science research and standards emerge, additional categories may be needed:

- **`spectral`**: Spectral-based color representations (if direct spectral color types are added)
- **`munsell`**: Munsell color system (if treated as first-class color space rather than conversion target)
- **`pantone`**: Pantone color system (if library-compatible licensing is achieved)

### Subcategories

If the RGB category grows beyond 30-40 spaces, it could be subdivided:

```text
src/space/rgb/
├── space/
│   ├── display/      # sRGB, Display P3, Adobe RGB
│   ├── broadcast/    # Rec. 709, Rec. 2020, Rec. 2100
│   ├── cinema/       # ACES variants, DCI-P3
│   ├── camera/       # ARRI, RED, Sony, Canon, etc.
│   └── legacy/       # Apple RGB, ColorMatch, etc.
```

This would follow the same re-export pattern to maintain the flat public API.

### Category Aliases

For spaces that fit multiple categories, type aliases could provide alternate access:

```rust
// In src/space/hdr/mod.rs
pub use crate::space::physiological::ICtCp;

// Both work:
use farg::space::physiological::ICtCp;
use farg::space::hdr::ICtCp;
```

This would aid discoverability without duplicating implementation.

## References

- ADR-0002: Universal Color Space Access (establishes `ColorSpace` trait implemented by all spaces)
- ADR-0003: Feature-Gated Components (establishes `space-*` feature flag pattern)
- ADR-0006: RGB Color Space Architecture (establishes existing RGB module structure)
- `docs/PLAN.md` - Lists all 70+ planned color spaces
