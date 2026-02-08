# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Break Versioning](https://www.taoensso.com/break-versioning).

## [Unreleased]

### Added

- Add `Cmy` color space with full conversion support, feature-gated behind `space-cmy`
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

[unreleased]: https://git.aaronmallen.dev/farg/farg/-/compare/0.1.1...main
[v0.1.1]: https://git.aaronmallen.dev/farg/farg/-/compare/0.1.0..0.1.1
