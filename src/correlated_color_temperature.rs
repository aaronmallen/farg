//! Correlated Color Temperature (CCT) estimation.
//!
//! Estimates the temperature (in Kelvin) of the blackbody radiator whose chromaticity most
//! closely matches a given color. Multiple algorithms are available, each offering different
//! trade-offs between speed, accuracy, and valid range.
//!
//! All algorithms accept any color type convertible to [`Xyz`](crate::space::Xyz) and return
//! a [`ColorTemperature`] value in Kelvin.

#[cfg(feature = "cct-hernandez-andres")]
pub mod hernandez_andres;
#[cfg(feature = "cct-mccamy")]
pub mod mccamy;
#[cfg(feature = "cct-ohno")]
pub mod ohno;
#[cfg(feature = "cct-robertson")]
pub mod robertson;

/// One million — the conversion factor between Kelvin and micro reciprocal degrees (MRD).
///
/// MRD = MRD_FACTOR / K, K = MRD_FACTOR / MRD.
const MRD_FACTOR: f64 = 1_000_000.0;

/// A correlated color temperature value in Kelvin.
///
/// Wraps an `f64` representing the temperature of the nearest blackbody radiator.
/// Provides access to both Kelvin and micro reciprocal degree (MRD) representations.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ColorTemperature(f64);

impl ColorTemperature {
  /// Returns the temperature in micro reciprocal degrees (MRD).
  ///
  /// MRD = 1,000,000 / K. This scale is more perceptually uniform than Kelvin
  /// and is used internally by several CCT algorithms.
  pub fn mrd(&self) -> f64 {
    MRD_FACTOR / self.0
  }

  /// Returns the temperature in Kelvin.
  pub fn value(&self) -> f64 {
    self.0
  }
}

impl From<ColorTemperature> for f64 {
  fn from(ct: ColorTemperature) -> Self {
    ct.0
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod color_temperature {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_value_in_kelvin() {
      let ct = ColorTemperature(6500.0);

      assert_eq!(ct.value(), 6500.0);
    }

    #[test]
    fn it_converts_to_mrd() {
      let ct = ColorTemperature(5000.0);

      assert_eq!(ct.mrd(), 200.0);
    }

    #[test]
    fn it_converts_to_f64() {
      let ct = ColorTemperature(6500.0);
      let value: f64 = ct.into();

      assert_eq!(value, 6500.0);
    }

    #[test]
    fn it_compares_values() {
      let a = ColorTemperature(5000.0);
      let b = ColorTemperature(6500.0);

      assert!(a < b);
      assert!(b > a);
      assert_eq!(a, ColorTemperature(5000.0));
    }
  }
}
