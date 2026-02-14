//! Ohno (2014) CCT estimation.
//!
//! A triangular solver with parabolic refinement that searches the Planckian locus in
//! CIE 1960 UCS (u, v) space. Provides sub-MRD precision through parabolic interpolation
//! around the closest point on the locus.
//!
//! # Reference
//!
//! Ohno, Y. (2014). "Practical Use and Calculation of CCT and Duv."
//! *LEUKOS*, 10(1), 47–55.

use super::{ColorTemperature, MRD_FACTOR};
use crate::{chromaticity::Xy, space::Xyz};

/// Temperature threshold (K) between the two Kim et al. approximation ranges.
const KIM_THRESHOLD: f64 = 4000.0;

/// Kim et al. (2002) blackbody xy approximation coefficients for T <= 4000 K.
mod kim_low {
  /// x chromaticity polynomial coefficients (in 1/T^3, 1/T^2, 1/T, constant).
  pub const X3: f64 = -0.2661239e9;
  pub const X2: f64 = -0.2343589e6;
  pub const X1: f64 = 0.8776956e3;
  pub const X0: f64 = 0.179910;

  /// y chromaticity polynomial coefficients (in x^3, x^2, x, constant).
  pub const Y3: f64 = -1.1063814;
  pub const Y2: f64 = -1.34811020;
  pub const Y1: f64 = 2.18555832;
  pub const Y0: f64 = -0.20219683;
}

/// Kim et al. (2002) blackbody xy approximation coefficients for T > 4000 K.
mod kim_high {
  /// x chromaticity polynomial coefficients (in 1/T^3, 1/T^2, 1/T, constant).
  pub const X3: f64 = -3.0258469e9;
  pub const X2: f64 = 2.1070379e6;
  pub const X1: f64 = 0.2226347e3;
  pub const X0: f64 = 0.240390;

  /// y chromaticity polynomial coefficients (in x^3, x^2, x, constant).
  pub const Y3: f64 = 3.0817580;
  pub const Y2: f64 = -5.87338670;
  pub const Y1: f64 = 3.75112997;
  pub const Y0: f64 = -0.37001483;
}

/// Start of the MRD search range (1 MRD = 1,000,000 K).
const MRD_SEARCH_START: i32 = 1;

/// End of the MRD search range (600 MRD = ~1,667 K).
const MRD_SEARCH_END: i32 = 600;

/// Minimum denominator magnitude for parabolic interpolation to avoid division by near-zero.
const PARABOLIC_EPSILON: f64 = 1e-20;

/// Calculates the correlated color temperature using Ohno's method.
///
/// Converts the color to CIE 1960 UCS (u, v) coordinates, searches the Planckian locus
/// at 1 MRD steps to find the closest point, then applies parabolic interpolation for
/// sub-MRD precision. Accurate from ~1,000 K to ~20,000 K.
///
/// ```
/// # #[cfg(feature = "cct-ohno")]
/// # {
/// use farg::correlated_color_temperature::ohno;
/// use farg::space::Xyz;
///
/// // D65 white point (~6504 K)
/// let d65 = Xyz::new(0.95047, 1.0, 1.08883);
/// let cct = ohno::calculate(d65);
/// assert!((cct.value() - 6504.0).abs() < 50.0);
/// # }
/// ```
pub fn calculate(color: impl Into<Xyz>) -> ColorTemperature {
  let [u_test, v_test] = color.into().chromaticity().to_uv().components();

  let mut min_dist = f64::MAX;
  let mut min_mrd = MRD_SEARCH_START;

  for mrd in MRD_SEARCH_START..=MRD_SEARCH_END {
    let t = MRD_FACTOR / mrd as f64;
    let [u_bb, v_bb] = planckian_locus_uv(t);
    let d = dist_sq(u_test, v_test, u_bb, v_bb);

    if d < min_dist {
      min_dist = d;
      min_mrd = mrd;
    }
  }

  let mrd_lo = (min_mrd - 1).max(MRD_SEARCH_START) as f64;
  let mrd_mid = min_mrd as f64;
  let mrd_hi = (min_mrd + 1).min(MRD_SEARCH_END) as f64;

  let [u_lo, v_lo] = planckian_locus_uv(MRD_FACTOR / mrd_lo);
  let [u_mid, v_mid] = planckian_locus_uv(MRD_FACTOR / mrd_mid);
  let [u_hi, v_hi] = planckian_locus_uv(MRD_FACTOR / mrd_hi);

  let d_lo = dist_sq(u_test, v_test, u_lo, v_lo);
  let d_mid = dist_sq(u_test, v_test, u_mid, v_mid);
  let d_hi = dist_sq(u_test, v_test, u_hi, v_hi);

  let denom = d_lo - 2.0 * d_mid + d_hi;
  let mrd_refined = if denom.abs() > PARABOLIC_EPSILON {
    mrd_mid + 0.5 * (d_lo - d_hi) / denom
  } else {
    mrd_mid
  };

  ColorTemperature(MRD_FACTOR / mrd_refined)
}

/// Squared distance between two points in uv space.
fn dist_sq(u1: f64, v1: f64, u2: f64, v2: f64) -> f64 {
  (u1 - u2) * (u1 - u2) + (v1 - v2) * (v1 - v2)
}

/// Calculates the Planckian locus coordinates in CIE 1960 UCS for a given temperature.
///
/// Uses Kim et al. (2002) approximation for CIE 1931 xy of a blackbody at temperature T,
/// then converts to CIE 1960 uv.
fn planckian_locus_uv(t: f64) -> [f64; 2] {
  let t2 = t * t;
  let t3 = t2 * t;

  let (x, y) = if t <= KIM_THRESHOLD {
    let x = kim_low::X3 / t3 + kim_low::X2 / t2 + kim_low::X1 / t + kim_low::X0;
    let x2 = x * x;
    let x3 = x2 * x;
    let y = kim_low::Y3 * x3 + kim_low::Y2 * x2 + kim_low::Y1 * x + kim_low::Y0;
    (x, y)
  } else {
    let x = kim_high::X3 / t3 + kim_high::X2 / t2 + kim_high::X1 / t + kim_high::X0;
    let x2 = x * x;
    let x3 = x2 * x;
    let y = kim_high::Y3 * x3 + kim_high::Y2 * x2 + kim_high::Y1 * x + kim_high::Y0;
    (x, y)
  };

  Xy::new(x, y).to_uv().components()
}

#[cfg(test)]
mod test {
  use super::*;

  mod planckian_locus_uv_fn {
    use super::*;

    #[test]
    fn it_returns_valid_coordinates_at_3000k() {
      let [u, v] = planckian_locus_uv(3000.0);

      assert!(u > 0.0 && u < 1.0);
      assert!(v > 0.0 && v < 1.0);
    }

    #[test]
    fn it_returns_valid_coordinates_at_6500k() {
      let [u, v] = planckian_locus_uv(6500.0);

      assert!(u > 0.0 && u < 1.0);
      assert!(v > 0.0 && v < 1.0);
    }

    #[test]
    fn it_shifts_coordinates_with_temperature() {
      let [u_warm, _] = planckian_locus_uv(3000.0);
      let [u_cool, _] = planckian_locus_uv(10000.0);

      assert!(u_warm > u_cool);
    }
  }

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
      let warm = crate::chromaticity::Xy::new(0.4369, 0.4041).to_xyz(1.0);
      let cct = calculate(warm);

      assert!((cct.value() - 3000.0).abs() < 100.0);
    }

    #[test]
    fn it_estimates_cool_white() {
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
    fn it_estimates_5000k() {
      let d50 = crate::chromaticity::Xy::new(0.3457, 0.3585).to_xyz(1.0);
      let cct = calculate(d50);

      assert!((cct.value() - 5000.0).abs() < 100.0);
    }
  }
}
