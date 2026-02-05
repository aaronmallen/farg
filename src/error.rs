use std::{
  error::Error as StdError,
  fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
  InvalidHexCharacter { input: String },
  InvalidHexLength { input: String, length: usize },
  MissingColorMatchingFunction,
  MissingIlluminantType,
  MissingSpectralPowerDistribution,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::InvalidHexCharacter {
        input,
      } => write!(f, "invalid hex character in '{input}'"),
      Self::InvalidHexLength {
        input,
        length,
      } => {
        write!(f, "invalid hex length {length} for '{input}', expected 3 or 6")
      }
      Self::MissingColorMatchingFunction => write!(f, "color matching function is required"),
      Self::MissingIlluminantType => write!(f, "illuminant type is required"),
      Self::MissingSpectralPowerDistribution => write!(f, "spectral power distribution is required"),
    }
  }
}

impl StdError for Error {}
