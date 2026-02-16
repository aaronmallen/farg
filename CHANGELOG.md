# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Break Versioning](https://www.taoensso.com/break-versioning).

## [Unreleased]

## [v0.4.2] - 2026-02-15

### Added

- Add `Xyy` (CIE xyY) chromaticity + luminance color space with full conversion support, feature-gated behind
  `space-xyy`
- Add color mixing and interpolation with three strategies on the `ColorSpace` trait: `mix()` / `mixed_with()` for
  perceptually uniform cylindrical interpolation (Oklch or LCh), `mix_linear()` / `mixed_with_linear()` for physically
  correct additive light mixing in linear sRGB, and `mix_rectangular()` / `mixed_with_rectangular()` for rectangular
  Oklab or L\*a\*b\* interpolation — cylindrical and rectangular variants feature-gated behind their respective color
  spaces
- Add `gradient()`, `gradient_linear()`, and `gradient_rectangular()` to the `ColorSpace` trait for generating
  evenly-spaced multi-step color sequences between two colors
- Add native `mix()`, `mixed_with()`, and `gradient()` methods to `Lab`, `Lch`, `Oklab`, `Oklch`, and `Rgb` color
  spaces with CSS Color Level 4 shortest-arc hue interpolation and achromatic handling on cylindrical spaces
- Add `Component::lerp()` for linear interpolation between component values
- Add color harmony methods to the `ColorSpace` trait: `analogous()`, `complementary()`, `split_complementary()`,
  `triadic()`, and `tetradic()` for hue-based harmonies, plus `monochromatic()` for luminance variations — hue-based
  methods feature-gated behind any cylindrical or perceptual color space

## [v0.4.1] - 2026-02-14

### Added

- Add `correlated_color_temperature` module with four CCT estimation algorithms: McCamy (1992), Hernandez-Andres
  (1999), Robertson (1968), and Ohno (2014) — each feature-gated behind `cct-*` flags and bundled under `all-cct`
- Add `ColorTemperature` type with `value()` (Kelvin) and `mrd()` (micro reciprocal degrees) representations
- Add `cct()` and `correlated_color_temperature()` convenience methods to the `ColorSpace` trait with a priority
  chain (Ohno > Robertson > Hernandez-Andres > McCamy)
- Add `distance` module with six color distance algorithms: CIE76 (ΔE\*76), CIE94 (ΔE\*94), CMC l:c, CIEDE2000
  (ΔE\*00), Euclidean, and Manhattan — each feature-gated behind `distance-*` flags and bundled under `all-distance`
- Add `closest_match()`, `is_perceptually_equivalent()`, and `is_distinguishable_from()` convenience methods to the
  `ColorSpace` trait, feature-gated behind `distance-ciede2000`

## [v0.4.0] - 2026-02-12

### Added

- Add gamut mapping with four strategies on `Rgb` and the `ColorSpace` trait: `clip_to_gamut()` (clamp),
  `scale_to_gamut()` (linear RGB scaling), `compress_to_gamut()` (CIELAB chroma reduction, feature-gated behind
  `space-lab`), and `perceptually_map_to_gamut()` (LMS scaling relative to reference white) — each with
  `with_gamut_*` builder variants
- Add `is_in_gamut()` on `Rgb` and the `ColorSpace` trait for checking whether a color falls within an RGB gamut
- Add `is_realizable()` on `Xyz` and the `ColorSpace` trait for checking physical realizability against the
  observer's spectral locus
- Add `contrast` module with six perceptual contrast algorithms: AERT brightness difference, APCA lightness
  contrast, Michelson, RMS, WCAG 2.x contrast ratio, and Weber contrast — each feature-gated behind `contrast-*`
  flags and bundled under `all-contrast`
- Add `ContrastRatio` and `LightnessContrast` wrapper types with threshold-checking methods for WCAG and APCA
  conformance levels
- Add `contrast_ratio()` and `lightness_contrast()` convenience methods to the `ColorSpace` trait, feature-gated
  behind `contrast-wcag` and `contrast-apca` respectively
- Add `Okhwb` perceptual color space (HWB model in the Oklab framework) with full conversion support,
  feature-gated behind `space-okhwb`
- Add `Luv` (CIE 1976 L\*u\*v\*) color space with full conversion support, feature-gated behind `space-luv`
- Add `chromaticity_rg()`, `chromaticity_upvp()`, and `chromaticity_uv()` convenience methods to the `ColorSpace`
  trait, feature-gated behind `chromaticity-rg`, `chromaticity-upvp`, and `chromaticity-uv` respectively
- Add `FairchildModifier` for deriving new observers with adjusted physiological parameters (age-related lens
  yellowing, macular pigment density, rod intrusion, S-cone field-size sensitivity) via `Observer::modifier()`
- Re-export `ObserverBuilder`, `IlluminantBuilder`, and `FairchildModifier` from the crate root

### Changed

- **BREAKING:** `Rgb::from_normalized` and `LinearRgb::from_normalized` no longer clamp values to 0.0-1.0,
  preserving out-of-gamut information for gamut mapping workflows
- **BREAKING:** `IlluminantBuilder::new` now requires `name` and `IlluminantType` at construction time;
- `IlluminantBuilder` now accepts non-`'static` references, allowing construction from dynamically generated data
- `ObserverBuilder` now accepts non-`'static` references, allowing construction from dynamically generated data

### Removed

- **BREAKING:** Remove `Error::MissingIlluminantType` variant (illuminant type is now required at builder construction)
- **BREAKING:** Remove `with_kind()` from `IlluminantBuilder`

## [v0.3.0] - 2026-02-10

### Added

- Add alpha/opacity support to all color spaces (`Xyz`, `Lms`, `Rgb`, `LinearRgb`, `Hsl`, `Hsv`, `Hwb`, `Cmy`, `Cmyk`)
  with `alpha()`, `set_alpha()`, `with_alpha()`, `opacity()`, `set_opacity()`, `with_opacity()`, and
  increment/decrement/scale variants on the `ColorSpace` trait
- Add `Lab` (CIE 1976 L\*a\*b\*) color space with full conversion support, feature-gated behind `space-lab`
- Add `LCh` (CIE 1976 L\*C\*h\*) color space (cylindrical form of Lab) with full conversion support,
  feature-gated behind `space-lch`
- Add `Oklab` perceptual color space with full conversion support, feature-gated behind `space-oklab`
- Add `Oklch` perceptual color space (cylindrical form of Oklab) with full conversion support,
  feature-gated behind `space-oklch`
- Add `Okhsl` perceptual color space with full conversion support, feature-gated behind `space-okhsl`
- Add `Okhsv` perceptual color space with full conversion support, feature-gated behind `space-okhsv`
- Add alpha compositing on `Rgb` via `flatten_alpha()`, `flatten_alpha_against()`, and their `with_*` builder variants
- Add `Rgb::BLACK` and `Rgb::WHITE` associated constants
- Add `hue()` and `chroma()` accessors to the `ColorSpace` trait with full mutation API
  (`set_hue`, `set_chroma`, `with_hue`, `with_chroma`, and increment/decrement/scale variants),
  feature-gated with a priority chain across cylindrical and perceptual color spaces

### Changed

- **BREAKING:** `ColorSpace` trait now requires `alpha()`, `set_alpha()` implementations
- **BREAKING:** `PartialEq` on all color spaces now includes the alpha channel in comparisons
- `Display` formatting on all color spaces now includes opacity percentage when alpha is below 1.0

## [v0.2.0] - 2026-02-08

### Added

- Add `Cmy` color space with full conversion support, feature-gated behind `space-cmy`
- Add `Cmyk` color space with full conversion support, feature-gated behind `space-cmyk`
- Add `Hsl` color space with full conversion support, feature-gated behind `space-hsl`
- Add `Hsb`/`Hsv` color space with full conversion support, feature-gated behind `space-hsb` / `space-hsv`
- Add `Hwb` color space with full conversion support, feature-gated behind `space-hwb`

### Changed

- **BREAKING:** `Rgb` short-name methods (`set_b`/`set_g`/`set_r`, `decrement_b`/`decrement_g`/`decrement_r`,
  `increment_b`/`increment_g`/`increment_r`, `with_b`/`with_g`/`with_r`, and their `_decremented_by`/`_incremented_by`
  builder variants) now accept normalized values (0.0-1.0) instead of 0-255 scale values. The long-name variants
  (`set_blue`/`set_green`/`set_red`, etc.) retain the 0-255 scale behavior. These methods are no longer aliases.
- Generalize arithmetic operators (`Add`, `Sub`, `Mul`, `Div`) on `Rgb`, `Xyz`, `Lms`, and `Hsl` to accept
  `impl Into<Self>`, enabling cross-type arithmetic (e.g., `xyz + rgb`)

### Fixed

- `Rgb::set_components` now correctly applies normalized values; previously `set_r`/`set_g`/`set_b` divided by 255,
  causing double-normalization when called from the `ColorSpace` trait

## [v0.1.1] - 2026-02-07

### Added

- `TryFrom<String>` for `Xyz` and `Lms` color spaces (parse from CSS color strings via sRGB conversion)

### Fixed

- Allow `LinearRgb::new` to accept `impl Into<Component>` via new `from_u8` constructor

## v0.1.0 - 2026-02-05

Initial release

[unreleased]: https://git.aaronmallen.dev/farg/farg/-/compare/0.4.2...main
[v0.1.1]: https://git.aaronmallen.dev/farg/farg/-/compare/0.1.0..0.1.1
[v0.2.0]: https://git.aaronmallen.dev/farg/farg/-/compare/0.1.1..0.2.0
[v0.3.0]: https://git.aaronmallen.dev/farg/farg/-/compare/0.2.0..0.3.0
[v0.4.0]: https://git.aaronmallen.dev/farg/farg/-/compare/0.3.0..0.4.0
[v0.4.1]: https://git.aaronmallen.dev/farg/farg/-/compare/0.4.0..0.4.1
[v0.4.2]: https://git.aaronmallen.dev/farg/farg/-/compare/0.4.1..0.4.2
