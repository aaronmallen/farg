//! Color Vision Deficiency (CVD) simulation.
//!
//! Simulates how colors appear to individuals with various forms of color blindness.
//! Three algorithms are available, each offering different trade-offs between accuracy
//! and performance.
//!
//! All algorithms accept any color type convertible to [`Xyz`](crate::space::Xyz) and return
//! an [`Xyz`](crate::space::Xyz) value representing the simulated color.

#[cfg(feature = "cvd-brettel")]
pub mod brettel;
#[cfg(feature = "cvd-machado")]
pub mod machado;
#[cfg(feature = "cvd-vienot")]
pub mod vienot;
