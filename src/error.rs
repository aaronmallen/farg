use std::{
  error::Error as StdError,
  fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
  MissingColorMatchingFunction,
  MissingIlluminantType,
  MissingSpectralPowerDistribution,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::MissingColorMatchingFunction => write!(f, "color matching function is required"),
      Self::MissingIlluminantType => write!(f, "illuminant type is required"),
      Self::MissingSpectralPowerDistribution => write!(f, "spectral power distribution is required"),
    }
  }
}

impl StdError for Error {}
