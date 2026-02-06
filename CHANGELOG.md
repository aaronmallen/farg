# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Break Versioning](https://www.taoensso.com/break-versioning).

## [Unreleased]

### Added

- `TryFrom<String>` for `Xyz` and `Lms` color spaces (parse from CSS color strings via sRGB conversion)

### Fixed

- Allow `LinearRgb::new` to accept `impl Into<Component>` via new `from_u8` constructor

## [v0.1.0] - 2026-02-05

Initial release

[unreleased]: https://git.aaronmallen.dev/farg/farg/-/compare/0.1.0...main
