//! Hernandez-Andres et al. (1999) CCT estimation.
//!
//! A higher-order exponential polynomial that extends the valid range far beyond McCamy's
//! method. Uses two coefficient sets: one for 3,000–50,000 K and another for 50,000–800,000 K.
//!
//! # Reference
//!
//! Hernandez-Andres, J., Lee, R. L., & Romero, J. (1999). "Calculating correlated color
//! temperatures across the entire gamut of daylight and skylight chromaticities."
//! *Applied Optics*, 38(27), 5703–5709.

use super::ColorTemperature;
use crate::space::Xyz;

/// Chromaticity epicenter x coordinate.
const EPICENTER_X: f64 = 0.3366;

/// Chromaticity epicenter y coordinate.
const EPICENTER_Y: f64 = 0.1735;

/// Temperature threshold (K) between low-range and high-range coefficient sets.
const HIGH_RANGE_THRESHOLD: f64 = 50000.0;

/// Low-range (3,000–50,000 K) exponential polynomial coefficients.
mod low_range {
  /// Constant offset.
  pub const A0: f64 = -949.86315;
  /// First exponential amplitude.
  pub const A1: f64 = 6253.80338;
  /// First exponential divisor.
  pub const T1: f64 = 0.92159;
  /// Second exponential amplitude.
  pub const A2: f64 = 28.70599;
  /// Second exponential divisor.
  pub const T2: f64 = 0.20039;
  /// Third exponential amplitude.
  pub const A3: f64 = 0.00004;
  /// Third exponential divisor.
  pub const T3: f64 = 0.07125;
}

/// High-range (50,000–800,000 K) exponential polynomial coefficients.
mod high_range {
  /// Constant offset.
  pub const A0: f64 = 36284.48953;
  /// First exponential amplitude.
  pub const A1: f64 = 0.00228;
  /// First exponential divisor.
  pub const T1: f64 = 0.07861;
  /// Second exponential amplitude.
  pub const A2: f64 = 5.4535e-36;
  /// Second exponential divisor.
  pub const T2: f64 = 0.01543;
}

/// Calculates the correlated color temperature using the Hernandez-Andres method.
///
/// Converts the color to CIE 1931 xy chromaticity and applies an exponential polynomial
/// approximation. Accurate from 3,000 K to 800,000 K, using two coefficient sets that
/// are selected automatically based on an initial estimate.
///
/// ```
/// # #[cfg(feature = "cct-hernandez-andres")]
/// # {
/// use farg::correlated_color_temperature::hernandez_andres;
/// use farg::space::Xyz;
///
/// // D65 white point (~6504 K)
/// let d65 = Xyz::new(0.95047, 1.0, 1.08883);
/// let cct = hernandez_andres::calculate(d65);
/// assert!((cct.value() - 6504.0).abs() < 50.0);
/// # }
/// ```
pub fn calculate(color: impl Into<Xyz>) -> ColorTemperature {
  let xy = color.into().chromaticity();
  let n = (xy.x() - EPICENTER_X) / (xy.y() - EPICENTER_Y);

  let cct = low_range::A0
    + low_range::A1 * (-n / low_range::T1).exp()
    + low_range::A2 * (-n / low_range::T2).exp()
    + low_range::A3 * (-n / low_range::T3).exp();

  if cct > HIGH_RANGE_THRESHOLD {
    let cct_high =
      high_range::A0 + high_range::A1 * (-n / high_range::T1).exp() + high_range::A2 * (-n / high_range::T2).exp();
    ColorTemperature(cct_high)
  } else {
    ColorTemperature(cct)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod calculate {
    use super::*;

    #[test]
    fn it_estimates_d65_white_point() {
      let d65 = Xyz::new(0.95047, 1.0, 1.08883);
      let cct = calculate(d65);

      assert!((cct.value() - 6504.0).abs() < 50.0);
    }

    #[test]
    fn it_estimates_warm_white() {
      // Approximate xy for ~3000K
      let warm = crate::chromaticity::Xy::new(0.4369, 0.4041).to_xyz(1.0);
      let cct = calculate(warm);

      assert!((cct.value() - 3000.0).abs() < 100.0);
    }

    #[test]
    fn it_estimates_cool_white() {
      // Approximate xy for ~10000K
      let cool = crate::chromaticity::Xy::new(0.2807, 0.2884).to_xyz(1.0);
      let cct = calculate(cool);

      assert!((cct.value() - 10000.0).abs() < 200.0);
    }

    #[test]
    fn it_returns_positive_for_typical_illuminants() {
      let d65 = Xyz::new(0.95047, 1.0, 1.08883);

      assert!(calculate(d65).value() > 0.0);
    }

    #[test]
    fn it_handles_high_color_temperature() {
      // Approximate xy for ~25000K
      let cool = crate::chromaticity::Xy::new(0.2520, 0.2521).to_xyz(1.0);
      let cct = calculate(cool);

      assert!(cct.value() > 15000.0);
    }
  }
}
