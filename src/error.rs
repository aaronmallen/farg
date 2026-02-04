use std::{
  error::Error as StdError,
  fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
  MissingColorMatchingFunction,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::MissingColorMatchingFunction => write!(f, "color matching function is required"),
    }
  }
}

impl StdError for Error {}
