# Feature Flags Reference

Farg uses granular feature flags so you only compile what you need. This document lists every feature flag,
what it provides, and any implicit dependencies.

## Always Available

The following are included with no feature flags enabled:

| Item                    | Description                                                         |
|-------------------------|---------------------------------------------------------------------|
| `Xyz`                   | CIE 1931 XYZ tristimulus color space                                |
| `Lms`                   | LMS cone response color space                                       |
| `Rgb<Srgb>`             | Standard RGB (IEC 61966-2-1)                                        |
| `Illuminant::D65`       | Noon daylight reference illuminant                                  |
| `Observer::CIE_1931_2D` | CIE 1931 2° standard colorimetric observer                          |
| `Xy` chromaticity       | CIE 1931 (x, y) chromaticity coordinates                            |
| XYZ Scaling CAT         | Identity-matrix chromatic adaptation (scales X, Y, Z independently) |

## Default Features

Enabled by `default`:

| Feature              | Provides                                |
|----------------------|-----------------------------------------|
| `cat-bradford`       | Bradford chromatic adaptation transform |
| `contrast-wcag`      | WCAG 2.x contrast ratio                 |
| `contrast-apca`      | APCA lightness contrast                 |
| `distance-ciede2000` | CIEDE2000 color difference              |

```toml
[dependencies]
farg = "0.4"
```

## Meta-Features

Group features that enable entire categories at once.

| Feature            | Enables                                                                   |
|--------------------|---------------------------------------------------------------------------|
| `full`             | All categories below                                                      |
| `all-cats`         | All 9 chromatic adaptation transforms                                     |
| `all-cct`          | All 4 correlated color temperature algorithms                             |
| `all-chromaticity` | All 3 chromaticity coordinate systems                                     |
| `all-contrast`     | All 6 contrast algorithms                                                 |
| `all-distance`     | All 6 color distance algorithms                                           |
| `all-illuminants`  | All illuminant sub-groups (standard, daylight, fluorescent, FL3, HP, LED) |
| `all-observers`    | All 7 additional observers                                                |
| `all-rgb-spaces`   | All 37 additional RGB color spaces                                        |
| `all-spaces`       | All color spaces (CIE, cylindrical, perceptual, subtractive, and all RGB) |

```toml
[dependencies]
farg = { version = "0.4", features = ["full"] }
```

## Chromatic Adaptation Transforms

Nine CATs available behind individual `cat-*` flags. XYZ Scaling is always available.

| Feature                    | Transform               | Description                              |
|----------------------------|-------------------------|------------------------------------------|
| `cat-bradford`             | Bradford                | Best general-purpose CAT (default)       |
| `cat-cat02`                | CAT02                   | From the CIECAM02 color appearance model |
| `cat-cat16`                | CAT16                   | From the CAM16 color appearance model    |
| `cat-cmc-cat97`            | CMC CAT97               | CMC CAT97 transform                      |
| `cat-cmc-cat2000`          | CMC CAT2000             | CMC CAT2000 transform                    |
| `cat-fairchild`            | Fairchild               | Fairchild transform                      |
| `cat-hunt-pointer-estevez` | Hunt-Pointer-Estevez    | HPE cone-response transform              |
| `cat-sharp`                | Sharp                   | Sharp transform (Susstrunk et al.)       |
| `cat-von-kries`            | Von Kries               | Diagonal adaptation with HPE-like cones  |

> `cat-hpe` is an alias for `cat-hunt-pointer-estevez`.

## Chromaticity Coordinate Systems

Three additional chromaticity types. CIE 1931 (x, y) is always available.

| Feature              | Type  | Description                                          |
|----------------------|-------|------------------------------------------------------|
| `chromaticity-rg`    | Rg    | RGB-relative chromaticity (r, g)                     |
| `chromaticity-upvp`  | Upvp  | CIE 1976 UCS chromaticity (u', v')                   |
| `chromaticity-uv`    | Uv    | CIE 1960 UCS chromaticity (u, v)                     |

## Correlated Color Temperature

Four CCT estimation algorithms available behind individual `cct-*` flags.

| Feature                | Algorithm        | Description                                       | Dependencies      |
|------------------------|------------------|---------------------------------------------------|-------------------|
| `cct-hernandez-andres` | Hernandez-Andres | Higher-order polynomial (3,000–800,000 K)         | -                 |
| `cct-mccamy`           | McCamy           | Third-degree polynomial (~2,000–12,500 K)         | -                 |
| `cct-ohno`             | Ohno             | Planckian locus search with parabolic refinement  | `chromaticity-uv` |
| `cct-robertson`        | Robertson        | Isotherm interpolation from 31-entry lookup table | `chromaticity-uv` |

## Contrast Algorithms

Six contrast algorithms available behind individual `contrast-*` flags.

| Feature              | Algorithm | Description                                          |
|----------------------|-----------|------------------------------------------------------|
| `contrast-aert`      | AERT      | W3C AERT brightness difference (0-255)               |
| `contrast-apca`      | APCA      | Accessible Perceptual Contrast Algorithm (Lc values) |
| `contrast-michelson` | Michelson | Visibility/modulation contrast (0.0-1.0)             |
| `contrast-rms`       | RMS       | Root mean square luminance contrast                  |
| `contrast-wcag`      | WCAG      | WCAG 2.x contrast ratio (1:1 to 21:1)                |
| `contrast-weber`     | Weber     | Weber contrast for target-on-background visibility   |

## Color Distance Algorithms

Six color distance algorithms available behind individual `distance-*` flags.

| Feature              | Algorithm | Description                                          | Dependencies |
|----------------------|-----------|------------------------------------------------------|--------------|
| `distance-ciede2000` | CIEDE2000 | Most perceptually uniform CIE metric (default)       | `space-lab`  |
| `distance-cie76`     | CIE76     | Euclidean distance in CIELAB space                   | `space-lab`  |
| `distance-cie94`     | CIE94     | Weighted CIELAB difference (graphic arts / textiles) | `space-lab`  |
| `distance-ciecmc`    | CMC l:c   | LCh-based difference for textile industry            | `space-lch`  |
| `distance-euclidean` | Euclidean | Straight-line distance in XYZ space                  | -            |
| `distance-manhattan` | Manhattan | Taxicab distance in XYZ space                        | -            |

## Illuminants

44 illuminants across 6 sub-groups. D65 is always available.

### Standard (`illuminant-standard`)

| Feature        | Illuminant | Type         | Description                 |
|----------------|------------|--------------|-----------------------------|
| `illuminant-a` | A          | Incandescent | Tungsten lamp (2856 K)      |
| `illuminant-b` | B          | Daylight     | Obsolete daylight simulator |
| `illuminant-c` | C          | Daylight     | Obsolete daylight simulator |
| `illuminant-e` | E          | Equal energy | Equal-energy illuminant     |

### Daylight (`illuminant-daylight`)

| Feature           | Illuminant | Description                      |
|-------------------|------------|----------------------------------|
| `illuminant-d50`  | D50        | Horizon daylight (5003 K)        |
| `illuminant-d55`  | D55        | Mid-morning daylight (5503 K)    |
| `illuminant-d75`  | D75        | North sky daylight (7504 K)      |
| `illuminant-id50` | ID50       | Indoor daylight D50              |
| `illuminant-id65` | ID65       | Indoor daylight D65              |

### Fluorescent (`illuminant-fluorescent`)

| Feature            | Illuminant | Description                          |
|--------------------|------------|--------------------------------------|
| `illuminant-fl1`   | FL1        | Daylight fluorescent (6430 K)        |
| `illuminant-fl2`   | FL2        | Cool white fluorescent (4230 K)      |
| `illuminant-fl3`   | FL3        | White fluorescent (3450 K)           |
| `illuminant-fl4`   | FL4        | Warm white fluorescent (2940 K)      |
| `illuminant-fl5`   | FL5        | Daylight fluorescent (6350 K)        |
| `illuminant-fl6`   | FL6        | Lite white fluorescent (4150 K)      |
| `illuminant-fl7`   | FL7        | Broadband daylight fluorescent       |
| `illuminant-fl8`   | FL8        | Broadband cool white fluorescent     |
| `illuminant-fl9`   | FL9        | Broadband warm white fluorescent     |
| `illuminant-fl10`  | FL10       | Narrowband daylight fluorescent      |
| `illuminant-fl11`  | FL11       | Narrowband cool white fluorescent    |
| `illuminant-fl12`  | FL12       | Narrowband warm white fluorescent    |

### Fluorescent Series 3 (`illuminant-fluorescent-3`)

| Feature             | Illuminant | Description                       |
|---------------------|------------|-----------------------------------|
| `illuminant-fl3-1`  | FL3.1      | Halophosphate daylight (6430 K)   |
| `illuminant-fl3-2`  | FL3.2      | Halophosphate cool white (4230 K) |
| `illuminant-fl3-3`  | FL3.3      | Halophosphate white (3450 K)      |
| `illuminant-fl3-4`  | FL3.4      | Halophosphate warm white (2940 K) |
| `illuminant-fl3-5`  | FL3.5      | Three-band daylight (6350 K)      |
| `illuminant-fl3-6`  | FL3.6      | Three-band cool white (4150 K)    |
| `illuminant-fl3-7`  | FL3.7      | Broadband D65 simulator           |
| `illuminant-fl3-8`  | FL3.8      | Broadband sRGB simulator          |
| `illuminant-fl3-9`  | FL3.9      | Broadband warm white              |
| `illuminant-fl3-10` | FL3.10     | Three-band indoor daylight        |
| `illuminant-fl3-11` | FL3.11     | Three-band cool white indoor      |
| `illuminant-fl3-12` | FL3.12     | Three-band warm white indoor      |
| `illuminant-fl3-13` | FL3.13     | Narrowband daylight simulator     |
| `illuminant-fl3-14` | FL3.14     | Narrowband cool white simulator   |
| `illuminant-fl3-15` | FL3.15     | Narrowband white + red boost      |

### High-Pressure Discharge (`illuminant-hp`)

| Feature           | Illuminant | Description                         |
|-------------------|------------|-------------------------------------|
| `illuminant-hp1`  | HP1        | Sodium standard                     |
| `illuminant-hp2`  | HP2        | Sodium colour-enhanced              |
| `illuminant-hp3`  | HP3        | Metal halide (high CRI)             |
| `illuminant-hp4`  | HP4        | High-pressure mercury               |
| `illuminant-hp5`  | HP5        | High-pressure mercury (high CRI)    |

### LED (`illuminant-led`)

| Feature               | Illuminant | Description                  |
|-----------------------|------------|------------------------------|
| `illuminant-led-b1`   | LED-B1     | Blue phosphor LED (2733 K)   |
| `illuminant-led-b2`   | LED-B2     | Blue phosphor LED (2998 K)   |
| `illuminant-led-b3`   | LED-B3     | Blue phosphor LED (4103 K)   |
| `illuminant-led-b4`   | LED-B4     | Blue phosphor LED (5109 K)   |
| `illuminant-led-b5`   | LED-B5     | Blue phosphor LED (6598 K)   |
| `illuminant-led-bh1`  | LED-BH1    | Blue-hybrid LED (2851 K)     |
| `illuminant-led-rgb1` | LED-RGB1   | RGB LED (2840 K)             |
| `illuminant-led-v1`   | LED-V1     | Violet phosphor LED (2724 K) |
| `illuminant-led-v2`   | LED-V2     | Violet phosphor LED (4070 K) |

## Observers

Seven additional observers. CIE 1931 2° is always available.

| Feature                         | Observer             | Description                     |
|---------------------------------|----------------------|---------------------------------|
| `observer-cie-1931-judd-2d`     | CIE 1931 Judd 2°     | Judd-modified 1931 observer     |
| `observer-cie-1931-judd-vos-2d` | CIE 1931 Judd-Vos 2° | Judd-Vos-modified 1931 observer |
| `observer-cie-1964-10d`         | CIE 1964 10°         | Supplementary standard observer |
| `observer-cie-2006-2d`          | CIE 2006 2°          | Physiologically-relevant (2°)   |
| `observer-cie-2006-10d`         | CIE 2006 10°         | Physiologically-relevant (10°)  |
| `observer-stockman-sharpe-2d`   | Stockman-Sharpe 2°   | Cone fundamentals (2°)          |
| `observer-stockman-sharpe-10d`  | Stockman-Sharpe 10°  | Cone fundamentals (10°)         |

## RGB Color Spaces

37 additional RGB spaces. sRGB is always available.

### Cinema and VFX

| Feature                     | Space                 | Illuminant | Description                     |
|-----------------------------|-----------------------|------------|---------------------------------|
| `rgb-aces-2065-1`           | ACES 2065-1           | D60        | ACES archival/interchange (AP0) |
| `rgb-aces-cc`               | ACEScc                | D60        | ACES color correction           |
| `rgb-aces-cct`              | ACEScct               | D60        | ACES color correction with toe  |
| `rgb-aces-cg`               | ACEScg                | D60        | ACES CG working space (AP1)     |
| `rgb-arri-wide-gamut-3`     | ARRI Wide Gamut 3     | D65        | ARRI Alexa camera               |
| `rgb-arri-wide-gamut-4`     | ARRI Wide Gamut 4     | D65        | ARRI Alexa 35 camera            |
| `rgb-blackmagic-wide-gamut` | Blackmagic Wide Gamut | D65        | Blackmagic Design cameras       |
| `rgb-canon-cinema-gamut`    | Canon Cinema Gamut    | D65        | Canon Cinema EOS cameras        |
| `rgb-davinci-wide-gamut`    | DaVinci Wide Gamut    | D65        | DaVinci Resolve                 |
| `rgb-filmlight-e-gamut`     | FilmLight E-Gamut     | D65        | FilmLight Baselight             |
| `rgb-panasonic-v-gamut`     | Panasonic V-Gamut     | D65        | Panasonic VariCam cameras       |
| `rgb-red-wide-gamut-rgb`    | RED Wide Gamut RGB    | D65        | RED cameras                     |
| `rgb-sony-s-gamut-3`        | Sony S-Gamut3         | D65        | Sony cameras                    |
| `rgb-sony-s-gamut-3-cine`   | Sony S-Gamut3.Cine    | D65        | Sony cameras (cinema)           |

### Broadcast and Video

| Feature            | Space         | Illuminant | Description               |
|--------------------|---------------|------------|---------------------------|
| `rgb-rec-601`      | Rec. 601      | D65        | ITU-R BT.601 (SD video)   |
| `rgb-rec-709`      | Rec. 709      | D65        | ITU-R BT.709 (HD video)   |
| `rgb-rec-2020`     | Rec. 2020     | D65        | ITU-R BT.2020 (UHD video) |
| `rgb-rec-2100-hlg` | Rec. 2100 HLG | D65        | BT.2100 with HLG transfer |
| `rgb-rec-2100-pq`  | Rec. 2100 PQ  | D65        | BT.2100 with PQ transfer  |
| `rgb-ntsc`         | NTSC          | C *        | NTSC (1953) primaries     |
| `rgb-pal-secam`    | PAL/SECAM     | D65        | PAL/SECAM primaries       |
| `rgb-smpte-c`      | SMPTE-C       | D65        | SMPTE-C (NTSC successor)  |

### Display

| Feature           | Space       | Illuminant | Description                    |
|-------------------|-------------|------------|--------------------------------|
| `rgb-display-p3`  | Display P3  | D65        | Apple Display P3               |
| `rgb-dci-p3`      | DCI-P3      | D65        | DCI-P3 digital cinema          |
| `rgb-adobe-rgb`   | Adobe RGB   | D65        | Adobe RGB (1998)               |
| `rgb-linear-srgb` | Linear sRGB | D65        | sRGB without transfer function |
| `rgb-scrgb`       | scRGB       | D65        | Extended-range linear sRGB     |
| `rgb-apple-rgb`   | Apple RGB   | D65        | Apple legacy display           |

### Wide Gamut and Specialty

| Feature               | Space             | Illuminant | Description                  |
|-----------------------|-------------------|------------|------------------------------|
| `rgb-prophoto-rgb`    | ProPhoto RGB      | D50 *      | ROMM RGB for photography     |
| `rgb-wide-gamut-rgb`  | Wide Gamut RGB    | D50 *      | Adobe Wide Gamut RGB         |
| `rgb-best-rgb`        | Best RGB          | D50 *      | Wide gamut reference         |
| `rgb-beta-rgb`        | Beta RGB          | D50 *      | Wide gamut (D50)             |
| `rgb-bruce-rgb`       | Bruce RGB         | D65        | Bruce Fraser's space         |
| `rgb-cie-rgb`         | CIE RGB           | E *        | Original CIE RGB (1931)      |
| `rgb-colormatch-rgb`  | ColorMatch RGB    | D50 *      | Press/prepress proofing      |
| `rgb-don-rgb-4`       | Don RGB 4         | D50 *      | Wide gamut (D50)             |
| `rgb-eci-rgb-v2`      | ECI RGB v2        | D50 *      | European Color Initiative    |
| `rgb-ektargb-ps5`     | Ekta Space PS5    | D50 *      | Kodak Ektachrome space       |

> Entries marked with **\*** implicitly enable the required illuminant feature (`illuminant-d50`,
> `illuminant-c`, or `illuminant-e`).

## Color Spaces

Non-RGB color spaces available behind individual `space-*` flags.

### CIE Color Spaces

| Feature       | Space | Description                                                | Dependencies |
|---------------|-------|------------------------------------------------------------|--------------|
| `space-lab`   | Lab   | CIE 1976 L\*a\*b\* perceptual color space                  | -            |
| `space-lch`   | LCh   | Cylindrical form of CIE L\*a\*b\* (lightness, chroma, hue) | `space-lab`  |
| `space-luv`   | Luv   | CIE 1976 L\*u\*v\* perceptual color space                  | -            |
| `space-lchuv` | LChuv | Cylindrical form of CIE L\*u\*v\* (lightness, chroma, hue) | `space-luv`  |
| `space-xyy`   | xyY   | CIE xyY chromaticity + luminance color space               | -            |

### Oklab Family

| Feature              | Space | Description                                 | Dependencies  |
|----------------------|-------|---------------------------------------------|---------------|
| `space-oklab`        | Oklab | Perceptual color space for image processing | -             |
| `space-oklch`        | Oklch | Cylindrical form of Oklab                   | `space-oklab` |
| `space-okhsl`        | Okhsl | Perceptually uniform HSL via Oklab          | `space-oklab` |
| `space-okhsv`        | Okhsv | Perceptually uniform HSV via Oklab          | `space-oklab` |
| `space-okhwb`        | Okhwb | Perceptually uniform HWB via Oklab          | `space-okhsv` |
| `space-oklab-family` | *all* | Enables Oklab, Oklch, Okhsl, Okhsv, Okhwb   | -             |

### HSLuv / HPLuv

| Feature       | Space | Description                                    | Dependencies  |
|---------------|-------|------------------------------------------------|---------------|
| `space-hsluv` | HSLuv | Perceptually uniform HSL via CIE LCh(uv)       | `space-lchuv` |
| `space-hpluv` | HPLuv | Hue-preserving variant with inscribed-circle S | `space-lchuv` |

### Cylindrical (RGB-derived)

| Feature      | Space   | Description                                    | Dependencies |
|--------------|---------|------------------------------------------------|--------------|
| `space-hsi`  | HSI     | Hue, Saturation, Intensity                     | -            |
| `space-hsl`  | HSL     | Hue, Saturation, Lightness                     | -            |
| `space-hsv`  | HSV/HSB | Hue, Saturation, Value (Brightness)            | -            |
| `space-hwb`  | HWB     | Hue, Whiteness, Blackness                      | -            |

> `space-hsb` is an alias for `space-hsv`.

### Subtractive

| Feature      | Space | Description                                     | Dependencies |
|--------------|-------|-------------------------------------------------|--------------|
| `space-cmy`  | CMY   | Cyan, Magenta, Yellow (complement of RGB)       | -            |
| `space-cmyk` | CMYK  | Cyan, Magenta, Yellow, Key/Black                | -            |

## Serialization

| Feature | Description                                                            |
|---------|------------------------------------------------------------------------|
| `serde` | `Serialize` and `Deserialize` impls for all color spaces via Serde 1.x |

```toml
[dependencies]
farg = { version = "0.4", features = ["serde"] }
```
