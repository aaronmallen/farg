use std::{
  error::Error as StdError,
  fmt::{Display, Formatter, Result as FmtResult},
};

/// Errors that can occur during color operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
  /// A hex color code contained an invalid character.
  InvalidHexCharacter { input: String },
  /// A hex color code had an invalid length (expected 3 or 6 characters).
  InvalidHexLength { input: String, length: usize },
  /// An observer builder was missing required color matching function data.
  MissingColorMatchingFunction,
  /// An illuminant builder was missing required spectral power distribution data.
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
      Self::MissingSpectralPowerDistribution => write!(f, "spectral power distribution is required"),
    }
  }
}

impl StdError for Error {}
