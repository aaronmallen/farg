//! McCamy (1992) CCT estimation.
//!
//! A simple third-degree polynomial in the CIE 1931 chromaticity epicenter.
//! Fast and accurate in the range ~2000 K to ~12,500 K.
//!
//! # Reference
//!
//! McCamy, C. S. (1992). "Correlated color temperature as an explicit function of
//! chromaticity coordinates." *Color Research & Application*, 17(2), 142–144.

use super::ColorTemperature;
use crate::space::Xyz;

/// Constant polynomial coefficient.
const A0: f64 = 5520.33;

/// First-degree polynomial coefficient (n term).
const A1: f64 = -6823.3;

/// Second-degree polynomial coefficient (n^2 term).
const A2: f64 = 3525.0;

/// Third-degree polynomial coefficient (n^3 term).
const A3: f64 = -449.0;

/// Chromaticity epicenter x coordinate from McCamy's paper.
const EPICENTER_X: f64 = 0.3320;

/// Chromaticity epicenter y coordinate from McCamy's paper.
const EPICENTER_Y: f64 = 0.1858;

/// Calculates the correlated color temperature using McCamy's method.
///
/// Converts the color to CIE 1931 xy chromaticity and applies a third-degree polynomial
/// approximation. Most accurate between 2,000 K and 12,500 K.
///
/// ```
/// # #[cfg(feature = "cct-mccamy")]
/// # {
/// use farg::correlated_color_temperature::mccamy;
/// use farg::space::Xyz;
///
/// // D65 white point (~6504 K)
/// let d65 = Xyz::new(0.95047, 1.0, 1.08883);
/// let cct = mccamy::calculate(d65);
/// assert!((cct.value() - 6504.0).abs() < 50.0);
/// # }
/// ```
pub fn calculate(color: impl Into<Xyz>) -> ColorTemperature {
  let xy = color.into().chromaticity();
  let n = (xy.x() - EPICENTER_X) / (xy.y() - EPICENTER_Y);
  let cct = A3 * n * n * n + A2 * n * n + A1 * n + A0;

  ColorTemperature(cct)
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
      let warm = Xyz::from(crate::chromaticity::Xy::new(0.4369, 0.4041).to_xyz(1.0));
      let cct = calculate(warm);

      assert!((cct.value() - 3000.0).abs() < 100.0);
    }

    #[test]
    fn it_estimates_cool_white() {
      let cool = Xyz::from(crate::chromaticity::Xy::new(0.2807, 0.2884).to_xyz(1.0));
      let cct = calculate(cool);

      assert!((cct.value() - 10000.0).abs() < 200.0);
    }

    #[test]
    fn it_returns_positive_for_typical_illuminants() {
      let d65 = Xyz::new(0.95047, 1.0, 1.08883);

      assert!(calculate(d65).value() > 0.0);
    }

    #[test]
    fn it_provides_mrd_conversion() {
      let d65 = Xyz::new(0.95047, 1.0, 1.08883);
      let cct = calculate(d65);

      assert!((cct.mrd() - 1_000_000.0 / cct.value()).abs() < 1e-10);
    }
  }
}
