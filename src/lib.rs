//! A Rust library for colorimetry, color space conversions, and color manipulation.
//!
//! Farg provides context-aware color conversions with f64 precision, spectral data processing,
//! and chromatic adaptation. It's designed to serve web developers with sensible defaults while
//! giving colorimetrists full control over illuminants, observers, and adaptation transforms.
//!
//! # Quick Start
//!
//! Create colors, convert between spaces, and manipulate components:
//!
//! ```
//! use farg::space::{ColorSpace, Rgb, Srgb, Xyz};
//!
//! // Create an sRGB color from 8-bit values
//! let color = Rgb::<Srgb>::new(255, 87, 51);
//!
//! // Convert to CIE XYZ
//! let xyz: Xyz = color.to_xyz();
//! let [x, y, z] = xyz.components();
//!
//! // Adjust luminance while preserving chromaticity
//! let brighter = xyz.with_luminance_scaled_by(1.2);
//! ```
//!
//! # Architecture
//!
//! Colors in farg exist within a **viewing context** consisting of:
//!
//! - An [`Illuminant`] — the light source (e.g., D65 daylight, D50 for print)
//! - An [`Observer`] — the human visual system model (e.g., CIE 1931 2°)
//! - A [`ChromaticAdaptationTransform`] — how the eye adjusts to different lighting
//!
//! By default, colors use the D65 illuminant with the CIE 1931 2° observer and the Bradford
//! chromatic adaptation transform. This matches the standard sRGB viewing environment.
//!
//! Conversions flow through hub color spaces to minimize implementation complexity:
//!
//! - **[`Xyz`](space::Xyz)** — universal hub for device-independent spaces
//! - **[`Lms`](space::Lms)** — cone response space used for chromatic adaptation
//! - **Linear RGB** — intermediate hub for the RGB family (gamma decode → linear → XYZ)
//!
//! # Color Spaces
//!
//! The [`space`] module contains all color space types and the [`ColorSpace`](space::ColorSpace)
//! trait. [`Xyz`](space::Xyz), [`Lms`](space::Lms), and [`Srgb`](space::Srgb) are always
//! available. Additional RGB spaces are enabled through feature flags:
//!
//! ```toml
//! [dependencies]
//! farg = { version = "0.1", features = ["rgb-display-p3", "rgb-adobe-rgb"] }
//! ```
//!
//! All color spaces implement the [`ColorSpace`](space::ColorSpace) trait, providing a common
//! interface for conversions, luminance operations, and component access.
//!
//! # Chromatic Adaptation
//!
//! Adapt colors between different illuminants using a [`ChromaticAdaptationTransform`]:
//!
//! ```
//! # #[cfg(feature = "illuminant-d50")]
//! # {
//! use farg::{Cat, ColorimetricContext, Illuminant};
//! use farg::space::Xyz;
//!
//! let d50_context = ColorimetricContext::new()
//!     .with_illuminant(Illuminant::D50)
//!     .with_cat(Cat::BRADFORD);
//!
//! let color = Xyz::new(0.95047, 1.0, 1.08883);
//! let adapted = color.adapt_to(d50_context);
//! # }
//! ```
//!
//! # Spectral Data
//!
//! Farg includes spectral power distribution (SPD) data for all standard illuminants and color
//! matching function (CMF) data for all standard observers. Access spectral data through the
//! [`SpectralTable`] trait:
//!
//! ```
//! use farg::{Illuminant, Observer, SpectralTable};
//!
//! let d65 = Illuminant::D65;
//! let spd = d65.spd();
//! let power_at_550nm = spd.at(550);
//!
//! let observer = Observer::CIE_1931_2D;
//! let cmf = observer.cmf();
//! let xyz = cmf.spectral_power_distribution_to_xyz(&spd);
//! ```
//!
//! # Chromaticity
//!
//! The [`chromaticity`] module provides coordinate systems for representing color independent
//! of luminance. [`Xy`](chromaticity::Xy) (CIE 1931) is always available; additional systems
//! like [`Uv`](chromaticity::Uv) and [`Upvp`](chromaticity::Upvp) are feature-gated.
//!
//! # Feature Flags
//!
//! Farg uses granular feature flags so you only compile what you need. The `default` feature
//! enables `cat-bradford`. The D65 illuminant, CIE 1931 2° observer, sRGB, and XYZ/LMS
//! spaces are always available regardless of feature selection.
//!
//! | Feature | Contents |
//! |---------|----------|
//! | `full` | Everything |
//! | `all-cats` | All chromatic adaptation transforms |
//! | `all-chromaticity` | All chromaticity coordinate systems |
//! | `all-illuminants` | All standard illuminants |
//! | `all-observers` | All standard observers |
//! | `all-rgb-spaces` | All RGB color spaces |

mod chromatic_adaptation_transform;
pub mod chromaticity;
mod component;
mod context;
mod error;
mod illuminant;
mod matrix;
mod observer;
pub mod space;
mod spectral;

pub use chromatic_adaptation_transform::{Cat, ChromaticAdaptationTransform};
pub use context::ColorimetricContext;
pub use error::Error;
pub use illuminant::{Illuminant, IlluminantType};
pub use observer::Observer;
pub use spectral::{
  ChromaticityCoordinates, Cmf, ColorMatchingFunction, ConeFundamentals, ConeResponse, Spd, SpectralPowerDistribution,
  Table as SpectralTable, TristimulusResponse,
};
