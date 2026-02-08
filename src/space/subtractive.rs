#[cfg(feature = "space-cmy")]
mod cmy;
#[cfg(feature = "space-cmyk")]
mod cmyk;

#[cfg(feature = "space-cmy")]
pub use cmy::Cmy;
#[cfg(feature = "space-cmyk")]
pub use cmyk::Cmyk;
