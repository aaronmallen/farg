---
id: 0003
title: Feature-Gated Components
status: active
tags: [architecture, features, compile-time]
created: 2026-02-04
superseded-by:
---

# ADR-0003: Feature-Gated Components

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Optional components (chromatic adaptation transforms, illuminants, observers, color spaces, etc.) are
conditionally compiled via Cargo feature flags. Each component category follows a consistent naming
convention and provides a priority-based `DEFAULT` constant when multiple implementations exist.

## Context

Color science libraries can grow large due to the number of supported:

- Chromatic adaptation transforms (Bradford, CAT02, CAT16, Von Kries, etc.)
- Illuminants (D50, D65, A, F2, etc.)
- Standard observers (CIE 1931 2°, CIE 1964 10°, etc.)
- Color spaces (70+ planned)

Most applications need only a subset of these. Including everything by default would:

1. **Increase compile times** unnecessarily
2. **Bloat binary sizes** for embedded/WASM targets
3. **Overwhelm users** with unused API surface

The tension is between comprehensive support and lightweight defaults.

## Decision

### Feature Flag Naming Convention

Each component category uses a prefix:

| Category                       | Prefix        | Example                     |
|--------------------------------|---------------|-----------------------------|
| Chromatic Adaptation Transform | `cat-`        | `cat-bradford`, `cat-cat16` |
| Illuminant                     | `illuminant-` | `illuminant-d65`            |
| Observer                       | `observer-`   | `observer-cie1931`          |
| Color Space                    | `space-`      | `space-oklab`               |
| Delta E Formula                | `delta-e-`    | `delta-e-2000`              |

### Meta-Features

Convenience features group related components:

```toml
[features]
all-cats = ["cat-bradford", "cat-cat02", "cat-cat16", ...]
common = ["space-srgb", "space-oklab", "illuminant-d65", "cat-bradford"]
full = ["all-cats", "all-illuminants", "all-observers", ...]
```

### Default Feature

A minimal `default` feature enables the most common components:

```toml
[features]
default = ["cat-bradford"]
```

### Priority-Based Defaults

When multiple implementations of a component type exist, one must be `DEFAULT`. Rather than requiring
explicit selection, we use compile-time priority:

```rust
// In cat-bradford (highest priority)
#[cfg(feature = "cat-bradford")]
impl ChromaticAdaptationTransform {
    pub const DEFAULT: Self = Self::BRADFORD;
}

// In cat-cat16 (second priority)
#[cfg(all(feature = "cat-cat16", not(feature = "cat-bradford")))]
impl ChromaticAdaptationTransform {
    pub const DEFAULT: Self = Self::CAT16;
}

// In cat-cat02 (third priority)
#[cfg(all(
    feature = "cat-cat02",
    not(feature = "cat-bradford"),
    not(feature = "cat-cat16")
))]
impl ChromaticAdaptationTransform {
    pub const DEFAULT: Self = Self::CAT02;
}
```

Priority order reflects industry consensus on best general-purpose choices.

### Fallback Implementation

One implementation (typically the simplest) is always available without feature flags to ensure
`DEFAULT` always exists:

```rust
// xyz_scaling.rs - no feature gate, always compiled
impl ChromaticAdaptationTransform {
    pub const XYZ_SCALING: Self = Self::new("XYZ Scaling", [[1.0, 0.0, 0.0], ...]);

    #[cfg(not(any(feature = "cat-bradford", feature = "cat-cat16", ...)))]
    pub const DEFAULT: Self = Self::XYZ_SCALING;
}
```

### Module Structure

Each feature-gated component lives in its own submodule:

```text
src/chromatic_adaptation_transform/
├── mod.rs           # Core type, always compiled
├── bradford.rs      # #[cfg(feature = "cat-bradford")]
├── cat02.rs         # #[cfg(feature = "cat-cat02")]
├── cat16.rs         # #[cfg(feature = "cat-cat16")]
└── xyz_scaling.rs   # Always compiled (fallback)
```

## Implementation Phases

- **Phase 1: CAT** - Chromatic adaptation transforms with feature gates
- **Phase 2: Illuminants** - Apply pattern to standard illuminants
- **Phase 3: Observers** - Apply pattern to color matching functions
- **Phase 4: Spaces** - Apply pattern to color spaces

## Consequences

### Positive

- **Minimal default binary**: Only Bradford CAT compiled by default
- **Granular control**: Users enable exactly what they need
- **Predictable defaults**: `DEFAULT` always exists and reflects best practices
- **Tree-shakeable**: Unused components are completely excluded from binary

### Negative

- **Feature flag complexity**: Many features to document and maintain
- **Conditional compilation verbosity**: Priority chains become long with many options
- **Testing burden**: Must test with various feature combinations

## Open Questions

- Should we provide a `no-default-features` guarantee for `no_std` environments?

## Future Work

- Add compile-time verification that exactly one `DEFAULT` exists
- Consider proc-macro to reduce priority chain boilerplate

## References

- [Cargo Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Feature Flags Best Practices](https://docs.rs/about/features)
