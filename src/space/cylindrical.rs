#[cfg(feature = "space-hsl")]
mod hsl;
#[cfg(feature = "space-hsv")]
mod hsv;
#[cfg(feature = "space-hwb")]
mod hwb;

#[cfg(feature = "space-hsl")]
pub use hsl::Hsl;
#[cfg(feature = "space-hsv")]
pub use hsv::{Hsb, Hsv};
#[cfg(feature = "space-hwb")]
pub use hwb::Hwb;
