#[cfg(feature = "chromaticity-rg")]
mod rg;
#[cfg(feature = "chromaticity-upvp")]
mod upvp;
#[cfg(feature = "chromaticity-uv")]
mod uv;
mod xy;

#[cfg(feature = "chromaticity-rg")]
pub use rg::Rg;
#[cfg(feature = "chromaticity-upvp")]
pub use upvp::Upvp;
#[cfg(feature = "chromaticity-uv")]
pub use uv::Uv;
pub use xy::Xy;
