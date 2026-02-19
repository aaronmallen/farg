# Farg Project Plan

This document describes the vision, scope, and technical design of Farg. It serves as a reference for
contributors to understand the project's goals and planned features. Not all features listed here are
implemented yet—this is a roadmap, not a changelog.

## Overview

Farg is a comprehensive Rust color library for web developers, designers, and colorimetrists.

## Design Principles

1. **Universal Access** — Any color can report any property (hue, lightness, etc.) without manual
  conversion
2. **Context-Aware** — Colors carry viewing context; changing context triggers automatic chromatic
  adaptation
3. **Idiomatic Rust** — `From`/`Into` traits, method chaining, zero-cost abstractions
4. **f64 Precision** — All internal calculations use 64-bit floating point
5. **Determinism** — Implicit conversions follow documented, deterministic paths

## Target Audiences

| Layer      | Audience       | Focus                               |
|------------|----------------|-------------------------------------|
| High-level | Web developers | `.adjust_contrast()`, `.to_css()`   |
| Mid-level  | Designers      | `.to_oklch()`, `.mix()`, gradients  |
| Low-level  | Colorimetrists | CMF, SPD, custom spaces             |

---

## Feature Checklist

- [x] [1. Core Color System](#1-core-color-system)
  - [x] [1.1 Viewing Context — ColorimetricContext](#11-viewing-context)
  - [ ] [1.1 Viewing Context — AppearanceContext](#11-viewing-context)
  - [x] [1.2 Component Behavior](#12-component-behavior)
  - [x] [1.3 Opacity](#13-opacity)
- [x] [2. Color Space Support](#2-color-space-support) (partial — core families complete)
  - [x] [2.1 CIE Spaces](#21-cie-spaces) — XYZ, xyY, Lab, LCH, Luv, LCHuv
  - [x] [2.2 RGB — Display](#22-rgb-family--display) — sRGB, Linear sRGB, scRGB, Display P3, DCI-P3, Adobe RGB,
    Wide Gamut, ProPhoto
  - [x] [2.3 RGB — Broadcast](#23-rgb-family--broadcast) — Rec. 601/709/2020/2100, NTSC, PAL/SECAM, SMPTE-C
  - [x] [2.4 RGB — Cinema/Camera](#24-rgb-family--cinemacamera) — ACES, ARRI, RED, Sony, Canon, Panasonic,
    Blackmagic, DaVinci, Filmlight
  - [x] [2.5 RGB — Legacy](#25-rgb-family--legacy) — CIE RGB, Apple RGB, ColorMatch, Best, Beta, Bruce, ECI,
    EktaSpace, Don RGB 4
  - [x] [2.6 Cylindrical Spaces](#26-cylindrical-spaces) — HSL, HSV/HSB, HSI, HWB, HSLuv, HPLuv
  - [ ] [2.6 Cylindrical Spaces — HCL](#26-cylindrical-spaces)
  - [x] [2.7 Oklab Family](#27-oklab-family) — Oklab, Oklch, Okhsl, Okhsv, Okhwb
  - [ ] [2.8 Color Appearance Models](#28-color-appearance-models) — CAM02, CAM16, HCT, ZCAM
  - [ ] [2.9 HDR Spaces](#29-hdr-spaces) — JzAzBz, JzCzhz, ICtCp
  - [ ] [2.10 Industrial Spaces](#210-industrial-spaces) — Hunter Lab, DIN99, OSA-UCS, IPT
  - [ ] [2.11 Video (Luma-Chroma)](#211-video-luma-chroma) — YCbCr, YPbPr, YCoCg, YUV, YIQ, YDbDr
  - [x] [2.12 Other](#212-other) — CMYK, LMS
- [x] [3. Conversion System](#3-conversion-system)
  - [x] [3.1 Hub-Based Architecture](#31-hub-based-architecture) — XYZ, Linear RGB, LMS hubs
  - [x] [3.2 Conversion Methods](#32-conversion-methods) — `.to_*()`, `From`/`Into`, `.adapt_to()`
- [x] [4. Universal Properties](#4-universal-properties) — hue, chroma, luminance, chromaticity
- [x] [5. Component Operations](#5-component-operations) — set/with/increment/decrement/scale
- [x] [6. Chromatic Adaptation](#6-chromatic-adaptation) — Bradford, Von Kries, CAT02, CAT16, Sharp, Fairchild,
  CMCCAT2000, HPE, CMC CAT97, XYZ Scaling
- [x] [7. Color Difference (ΔE)](#7-color-difference-δe) (partial)
  - [x] CIE76, CIE94, CIEDE2000, CMC(l:c)
  - [ ] CAM16-UCS distance
- [x] [8. Contrast Algorithms](#8-contrast-algorithms) — WCAG, APCA, Michelson, Weber
- [x] [9. Gamut Mapping](#9-gamut-mapping) — Clip, Chroma reduction, LMS Perceptual, Linear scale
- [ ] [10. ICC Profile Support](#10-icc-profile-support)
- [x] [11. String I/O](#11-string-io) (partial)
  - [x] Formatting — `to_hex()`, `to_css()`
  - [x] Parsing — hex codes
  - [ ] Parsing — CSS color strings (HSL, HWB, Color Level 4)
- [x] [12. Serialization (Serde)](#12-serialization-serde)
- [ ] [13. Named Colors](#13-named-colors) — CSS named, X11, custom palettes
- [ ] [14. Color Blindness Simulation](#14-color-blindness-simulation) — Dichromacy, Anomalous trichromacy
- [x] [15. Color Harmonies](#15-color-harmonies) — Complementary, Analogous, Triadic, Split-complementary,
  Tetradic, Monochromatic
- [ ] [16. Blend Modes](#16-blend-modes) — SVG/CSS blend modes
- [x] [17. Mixing and Interpolation](#17-mixing-and-interpolation) (partial)
  - [x] Linear, Cylindrical, Rectangular interpolation and gradients
  - [ ] CatmullRom, BSpline, Premultiplied alpha interpolation
- [x] [18. Spectral Data](#18-spectral-data) — SPD, CMF, SpectralTable, TristimulusResponse
- [x] [19. Additional Features](#19-additional-features) (partial)
  - [x] CCT — Ohno, Robertson, Hernandez-Andres, McCamy
  - [x] Chromaticity — xy, uv, u'v', rg
  - [x] Transfer Functions — sRGB, Gamma, PQ, HLG, ProPhoto, BT.601, BT.709
  - [x] Custom RGB Spaces — via `RgbSpec` trait
  - [x] Gamut Normalization
  - [ ] Dominant Wavelength
  - [ ] Munsell notation
- [x] [20. Reference Data](#20-reference-data) — Illuminants, Observers
- [x] [Feature Gating](#feature-gating)
- [ ] [Engine Configuration](#engine-configuration)
- [ ] [Ecosystem](#ecosystem) — farg-ansi, farg-image, farg-studio

---

## Feature Categories

### 1. Core Color System

#### 1.1 Viewing Context

Colors exist within a viewing context consisting of:

- **Illuminant** — The light source (e.g., D65 daylight, D50 for print)
- **Observer** — The human visual system model (e.g., CIE 1931 2°)
- **Chromatic Adaptation Transform (CAT)** — How the eye adjusts to different lighting

Two context types:

- `ColorimetricContext` — For standard conversions (XYZ, Lab, etc.)
- `AppearanceContext` — Extends colorimetric with adapting luminance, background luminance, and
  surround conditions for CAM16/ZCAM

Each color space has canonical default context. sRGB defaults to D65/CIE 1931 2°.

#### 1.2 Component Behavior

Components behave according to their nature:

- **Linear (RGB, lightness)** — Clamp at boundaries
- **Angular (hue)** — Wrap at 360°
- **Unbounded (XYZ)** — Preserve values as-is

Natural ranges per space: sRGB uses 0-255, Oklch uses 0.0-1.0 for lightness, hue is always 0-360°.

#### 1.3 Opacity

Every color carries an opacity value (default 1.0). Opacity is:

- Preserved through conversions
- Reflected in CSS output
- Supports arithmetic operations (increment, decrement, scale, contract)
- Supports alpha blending and premultiplied alpha

---

### 2. Color Space Support

70+ color spaces organized into families:

#### 2.1 CIE Spaces

| Space | Components | Notes                    |
|-------|------------|--------------------------|
| XYZ   | X, Y, Z    | Universal hub            |
| xyY   | x, y, Y    | Chromaticity + luminance |
| Lab   | L*, a*, b* | Perceptual, D50 default  |
| LCH   | L*, C*, h  | Cylindrical Lab          |
| Luv   | L*, u*, v* | Alternative to Lab       |
| LCHuv | L*, C*, h  | Cylindrical Luv          |

#### 2.2 RGB Family — Display

| Space          | White | Transfer | Standard      |
|----------------|-------|----------|---------------|
| sRGB           | D65   | sRGB     | IEC 61966-2-1 |
| Linear sRGB    | D65   | Linear   | —             |
| scRGB          | D65   | Linear   | IEC 61966-2-2 |
| Display P3     | D65   | sRGB     | Apple         |
| DCI-P3         | DCI   | γ 2.6    | SMPTE RP 431  |
| Adobe RGB      | D65   | γ 2.2    | Adobe         |
| Wide Gamut RGB | D50   | γ 2.2    | Adobe         |
| ProPhoto RGB   | D50   | γ 1.8    | ISO 22028-1   |

#### 2.3 RGB Family — Broadcast

Rec. 601, Rec. 709, Rec. 2020, Rec. 2100 (PQ/HLG), NTSC, PAL/SECAM, SMPTE-C

#### 2.4 RGB Family — Cinema/Camera

ACES (2065-1, ACEScg, ACEScc, ACEScct), ARRI Wide Gamut 3/4, RED Wide Gamut, Sony S-Gamut variants,
Canon Cinema Gamut, Panasonic V-Gamut, Blackmagic Wide Gamut, DaVinci Wide Gamut, Filmlight E-Gamut

#### 2.5 RGB Family — Legacy

CIE RGB, Apple RGB, ColorMatch RGB, Best RGB, Beta RGB, Bruce RGB, ECI RGB v2, EktaSpace PS5,
Don RGB 4

#### 2.6 Cylindrical Spaces

| Space   | Base | Components |
|---------|------|------------|
| HSL     | sRGB | H, S, L    |
| HSV/HSB | sRGB | H, S, V    |
| HSI     | sRGB | H, S, I    |
| HWB     | sRGB | H, W, B    |
| HSLuv   | Luv  | H, S, L    |
| HPLuv   | Luv  | H, P, L    |
| HCL     | Lab  | H, C, L    |

#### 2.7 Oklab Family

| Space | Components | Notes                     |
|-------|------------|---------------------------|
| Oklab | L, a, b    | Perceptually uniform      |
| Oklch | L, C, h    | Cylindrical, best for hue |
| Okhsl | H, S, L    | sRGB-gamut bounded        |
| Okhsv | H, S, V    | sRGB-gamut bounded        |
| Okhwb | H, W, B    | sRGB-gamut bounded        |

#### 2.8 Color Appearance Models

CAM02, CAM02-UCS, CAM16, CAM16-UCS, HCT (Material Design), ZCAM

#### 2.9 HDR Spaces

| Space  | Components | Transfer |
|--------|------------|----------|
| JzAzBz | Jz, Az, Bz | PQ       |
| JzCzhz | Jz, Cz, hz | PQ       |
| ICtCp  | I, Ct, Cp  | PQ/HLG   |

#### 2.10 Industrial Spaces

Hunter Lab, DIN99, DIN99o, OSA-UCS, IPT

#### 2.11 Video (Luma-Chroma)

YCbCr, YPbPr, YCoCg, YUV, YIQ, YDbDr

#### 2.12 Other

CMYK, LMS (cone response space)

---

### 3. Conversion System

#### 3.1 Hub-Based Architecture

Three primary hubs minimize implementation complexity:

- **XYZ** — Universal hub for device-independent spaces
- **Linear RGB** — Hub for RGB family (gamma decode → linear → hub)
- **LMS** — Cone response space for Oklab, CAM16, chromatic adaptation

Example path: `HSL → sRGB → Linear sRGB → XYZ → LMS → Oklab → Oklch`

#### 3.2 Conversion Methods

| Method                  | Description                          |
|-------------------------|--------------------------------------|
| `.to_*()` / `From/Into` | Default context (canonical settings) |
| `.to_*_with(&ctx)`      | Explicit context override            |
| `.try_to_*()`           | Fallible (parsing, profile loading)  |

---

### 4. Universal Properties

Any color can report any property; the library handles conversion automatically.

| Property         | Conversion Path | Notes                      |
|------------------|-----------------|----------------------------|
| `hue()`          | → Oklch         | Perceptually uniform hue   |
| `lightness()`    | → Oklch         | Perceptual lightness (0–1) |
| `chroma()`       | → Oklch         | Colorfulness               |
| `luminance()`    | → XYZ → Y       | Relative luminance         |
| `chromaticity()` | → XYZ → xy      | CIE 1931 chromaticity      |

For performance-critical code, convert once and access fields directly.

---

### 5. Component Operations

Consistent naming convention for all components:

| Operation | Mutable          | Immutable               |
|-----------|------------------|-------------------------|
| Set       | `set_*(val)`     | `with_*(val)`           |
| Add       | `increment_*(n)` | `with_incremented_*(n)` |
| Subtract  | `decrement_*(n)` | `with_decremented_*(n)` |
| Multiply  | `scale_*(n)`     | `with_scaled_*(n)`      |

---

### 6. Chromatic Adaptation

Automatic adaptation when illuminant changes. Supported CATs:

| CAT        | Notes                              |
|------------|------------------------------------|
| Bradford   | Default, best general-purpose      |
| Von Kries  | Simple diagonal adaptation         |
| CAT02      | Used in CIECAM02                   |
| CAT16      | Used in CAM16, improved from CAT02 |
| Sharp      | Süsstrunk et al.                   |
| Fairchild  | Fairchild's transform              |
| CMCCAT2000 | CMC 2000 standard                  |

---

### 7. Color Difference (ΔE)

| Formula   | Description                                         |
|-----------|-----------------------------------------------------|
| CIE76     | Euclidean distance in Lab                           |
| CIE94     | Weighted Lab distance                               |
| CIEDE2000 | Industry standard with L, C, hue corrections        |
| CMC(l:c)  | Textile industry standard, parameterized            |
| CAM16-UCS | Distance in CAM16 uniform space, appearance-based   |

Default: CIEDE2000

---

### 8. Contrast Algorithms

| Formula   | Description                                    |
|-----------|------------------------------------------------|
| WCAG      | W3C standard, 4.5:1 required for normal text   |
| APCA      | Advanced Perceptual Contrast, proposed WCAG 3  |
| Michelson | For periodic patterns                          |
| Weber     | For threshold detection                        |

---

### 9. Gamut Mapping

#### 9.1 Default Behavior

Standard conversions automatically apply gamut mapping. Raw methods bypass mapping for HDR/scientific
use.

| Method          | Gamut Mapping | Use Case                   |
|-----------------|---------------|----------------------------|
| `to_srgb()`     | Yes           | Display, web, most users   |
| `to_srgb_raw()` | No            | HDR, scientific, pipelines |

#### 9.2 Strategies

| Strategy   | Description                                           |
|------------|-------------------------------------------------------|
| Clip       | Clamp RGB to [0,1]. Fast, may shift hue.              |
| Chroma     | Reduce chroma until in gamut. Preserves hue/lightness |
| Perceptual | CSS Color 4 algorithm. Binary search in Oklch.        |

---

### 10. ICC Profile Support

Full ICC v4 support in phases:

| Phase | Scope                     | Coverage                        |
|-------|---------------------------|---------------------------------|
| 1     | Parse + Matrix/TRC        | Most display profiles           |
| 2     | LUT-based transforms      | Printer profiles, complex       |
| 3     | Device link + Named color | Precomputed workflows, spot     |

#### Rendering Intents

| Intent                | Use Case                                     |
|-----------------------|----------------------------------------------|
| Perceptual            | Photos, compress gamut to preserve relations |
| Relative Colorimetric | Match in-gamut exactly, clip out-of-gamut    |
| Saturation            | Graphics, maximize saturation                |
| Absolute Colorimetric | Proofing, simulate exact output              |

---

### 11. String I/O

#### Parsing

```rust
farg::parse("#ff5733")?;           // Hex
farg::parse("hsl(14, 100%, 60%)")?; // CSS HSL
farg::parse("lab(62% 56 46)")?;     // CSS Color 4 Lab
```

#### Formatting

```rust
color.to_hex()   // "#ff5733"
color.to_css()   // "rgb(255 87 51)"
```

---

### 12. Serialization (Serde)

Colors serialize to JSON objects with named fields:

```rust
srgb(128, 77, 26)  // → {"r": 128, "g": 77, "b": 26}
oklch(0.7, 0.15, 145.0)  // → {"l": 0.7, "c": 0.15, "h": 145.0}
```

---

### 13. Named Colors

- CSS named colors (`named("red")`)
- X11 color names (`x11("tomato")`)
- Custom palette registration

---

### 14. Color Blindness Simulation

#### Dichromacy (complete absence)

- Protanopia (no L cones, red-blind)
- Deuteranopia (no M cones, green-blind)
- Tritanopia (no S cones, blue-blind)

#### Anomalous Trichromacy (shifted response)

- Protanomaly, Deuteranomaly, Tritanomaly with severity 0.0-1.0

Algorithms: Machado 2009 (anomalous), Brettel 1997 (dichromacy)

---

### 15. Color Harmonies

Calculated in OkLCh for perceptual uniformity:

| Harmony             | Description                        |
|---------------------|------------------------------------|
| Complementary       | Opposite on wheel (180°)           |
| Analogous           | Adjacent colors (±30°)             |
| Triadic             | Three at 120° intervals            |
| Split-complementary | Complement's neighbors (±150°)     |
| Tetradic            | Four at 90° intervals              |
| Monochromatic       | Lightness variations, same hue     |

---

### 16. Blend Modes

SVG/CSS blend modes: Multiply, Screen, Overlay, Darken, Lighten, ColorDodge, ColorBurn, HardLight,
SoftLight, Difference, Exclusion, Hue, Saturation, Color, Luminosity

---

### 17. Mixing and Interpolation

| Method        | Description                           |
|---------------|---------------------------------------|
| Linear        | Straight line between colors          |
| CatmullRom    | Smooth spline through control points  |
| BSpline       | Smooth spline, approximates points    |
| Premultiplied | Correct for semi-transparent colors   |

---

### 18. Spectral Data

#### Spectral Power Distributions (SPD)

- Access illuminant spectral data
- Query at specific wavelengths
- Transform and integrate

#### Color Matching Functions (CMF)

- Integrate SPD with observer CMF to get XYZ
- Multiple observer models supported

---

### 19. Additional Features

| Feature              | Description                                     |
|----------------------|-------------------------------------------------|
| CCT                  | Correlated Color Temperature (Robertson/McCamy) |
| Chromaticity         | CIE xy, uv, u'v' coordinates                    |
| Transfer Functions   | sRGB, Gamma, PQ (HDR), HLG                      |
| Dominant Wavelength  | Spectral color + excitation purity              |
| Munsell              | Hue/Value/Chroma notation                       |
| Custom RGB Spaces    | Define from primaries + transfer function       |
| Gamut Normalization  | Normalize out-of-range component values         |

---

### 20. Reference Data

#### Illuminants

- **Daylight:** D50, D55, D65, D75, ID50, ID65
- **Incandescent:** A, B, C, E
- **Fluorescent:** FL1-FL12, FL3.1-FL3.15
- **Discharge:** HP1-HP5
- **LED:** LED-B1 through B5, LED-BH1, LED-RGB1, LED-V1, LED-V2

#### Observers

- **CMF (XYZ):** CIE 1931 2°, CIE 1964 10°, CIE 2006 2°/10°, Stiles-Burch, Stockman-Sharpe
- **Cone Fundamentals (LMS):** CIE 2006, Stockman-Sharpe, Stiles-Burch

---

## Feature Gating

Granular feature flags for compile time and binary size:

| Prefix         | Examples                                     |
|----------------|----------------------------------------------|
| `space-*`      | `space-oklab`, `space-cam16`, `space-jzazbz` |
| `cat-*`        | `cat-bradford`, `cat-cat16`                  |
| `illuminant-*` | `illuminant-d65`, `illuminant-d50`           |
| `observer-*`   | `observer-cie1931`, `observer-cie2006`       |
| `delta-e-*`    | `delta-e-2000`, `delta-e-cmc`                |
| `contrast-*`   | `contrast-wcag`, `contrast-apca`             |
| `icc-*`        | `icc-read`, `icc-matrix`, `icc-lut`          |

### Meta Features

| Feature  | Contents                                 |
|----------|------------------------------------------|
| `full`   | Everything                               |
| `common` | sRGB, Oklab, Lab, D65, Bradford          |
| `web`    | CSS parsing, WCAG, APCA, hex colors      |
| `video`  | Rec. 709/2020/2100, YCbCr, PQ, HLG       |
| `cinema` | ACES, DCI-P3, camera manufacturer gamuts |

---

## Engine Configuration

Explicit configuration avoids global mutable state:

```rust
let engine = farg::Engine::new()
    .with_gamut_strategy(GamutStrategy::Perceptual)
    .with_default_context(ColorimetricContext::d50_bradford());

let rgb = engine.convert::<Srgb>(&xyz);
```

---

## Ecosystem

Planned companion crates:

- **farg-ansi** — Terminal color output with capability detection
- **farg-image** — Integration with Rust image ecosystem
- **farg-studio** — Interactive GUI for exploring colorimetry

---

## Future Considerations

- SIMD batch conversions
- `no_std` support
- WebAssembly / wasm-bindgen
- Spectral/pigment mixing (Kubelka-Munk)
- Spectral recovery from XYZ/RGB
- Color quality metrics (CRI, TM-30 Rf, CQS)
- Whiteness/Yellowness indices
- Physiological observer modification (age, field size)
- Generic CMF response types
