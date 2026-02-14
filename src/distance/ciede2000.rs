//! CIEDE2000 color difference (ΔE\*00).
//!
//! Implements the [CIEDE2000](https://en.wikipedia.org/wiki/Color_difference#CIEDE2000) color
//! difference formula, the most perceptually uniform CIE metric. It includes corrections for
//! lightness, chroma, and hue, plus an interactive term for the blue region and a rotation
//! term for chroma/hue interaction.

use crate::space::{Lab, Xyz};

/// Just Noticeable Difference threshold. Two colors with ΔE\*00 < 1.0 are generally
/// considered perceptually indistinguishable.
pub const JND: f64 = 1.0;

/// Calculates the CIEDE2000 color difference with default parametric factors (kL=kC=kH=1.0).
///
/// The result is always >= 0.0 and is order-independent.
pub fn calculate(color1: impl Into<Xyz>, color2: impl Into<Xyz>) -> f64 {
  calculate_parametric(color1, color2, 1.0, 1.0, 1.0)
}

/// Calculates the CIEDE2000 color difference with custom parametric factors.
///
/// - `kl` — lightness weighting factor
/// - `kc` — chroma weighting factor
/// - `kh` — hue weighting factor
pub fn calculate_parametric(color1: impl Into<Xyz>, color2: impl Into<Xyz>, kl: f64, kc: f64, kh: f64) -> f64 {
  let lab1 = Lab::from(color1.into());
  let lab2 = Lab::from(color2.into());

  let l1 = lab1.l();
  let a1 = lab1.a();
  let b1 = lab1.b();
  let l2 = lab2.l();
  let a2 = lab2.a();
  let b2 = lab2.b();

  // Step 1: Calculate C'ab and h'ab
  let c_star_1 = (a1 * a1 + b1 * b1).sqrt();
  let c_star_2 = (a2 * a2 + b2 * b2).sqrt();
  let c_star_avg = (c_star_1 + c_star_2) / 2.0;

  let c_star_avg_7 = c_star_avg.powi(7);
  let twenty_five_7: f64 = 25.0_f64.powi(7);
  let g = 0.5 * (1.0 - (c_star_avg_7 / (c_star_avg_7 + twenty_five_7)).sqrt());

  let a1_prime = a1 * (1.0 + g);
  let a2_prime = a2 * (1.0 + g);

  let c_prime_1 = (a1_prime * a1_prime + b1 * b1).sqrt();
  let c_prime_2 = (a2_prime * a2_prime + b2 * b2).sqrt();

  let h_prime_1 = hue_angle(b1, a1_prime);
  let h_prime_2 = hue_angle(b2, a2_prime);

  // Step 2: Calculate ΔL', ΔC', ΔH'
  let dl_prime = l2 - l1;
  let dc_prime = c_prime_2 - c_prime_1;

  let dh_prime = if c_prime_1 * c_prime_2 == 0.0 {
    0.0
  } else {
    let diff = h_prime_2 - h_prime_1;
    if diff.abs() <= 180.0 {
      diff
    } else if diff > 180.0 {
      diff - 360.0
    } else {
      diff + 360.0
    }
  };

  let dh_prime_big = 2.0 * (c_prime_1 * c_prime_2).sqrt() * (dh_prime.to_radians() / 2.0).sin();

  // Step 3: Calculate CIEDE2000
  let l_prime_avg = (l1 + l2) / 2.0;
  let c_prime_avg = (c_prime_1 + c_prime_2) / 2.0;

  let h_prime_avg = if c_prime_1 * c_prime_2 == 0.0 {
    h_prime_1 + h_prime_2
  } else if (h_prime_1 - h_prime_2).abs() <= 180.0 {
    (h_prime_1 + h_prime_2) / 2.0
  } else if h_prime_1 + h_prime_2 < 360.0 {
    (h_prime_1 + h_prime_2 + 360.0) / 2.0
  } else {
    (h_prime_1 + h_prime_2 - 360.0) / 2.0
  };

  let t = 1.0 - 0.17 * (h_prime_avg - 30.0).to_radians().cos()
    + 0.24 * (2.0 * h_prime_avg).to_radians().cos()
    + 0.32 * (3.0 * h_prime_avg + 6.0).to_radians().cos()
    - 0.20 * (4.0 * h_prime_avg - 63.0).to_radians().cos();

  let l_prime_avg_50_sq = (l_prime_avg - 50.0).powi(2);
  let sl = 1.0 + 0.015 * l_prime_avg_50_sq / (20.0 + l_prime_avg_50_sq).sqrt();
  let sc = 1.0 + 0.045 * c_prime_avg;
  let sh = 1.0 + 0.015 * c_prime_avg * t;

  let c_prime_avg_7 = c_prime_avg.powi(7);
  let rc = 2.0 * (c_prime_avg_7 / (c_prime_avg_7 + twenty_five_7)).sqrt();

  let d_theta = 30.0 * (-((h_prime_avg - 275.0) / 25.0).powi(2)).exp();
  let rt = -(2.0 * d_theta).to_radians().sin() * rc;

  let term_l = dl_prime / (kl * sl);
  let term_c = dc_prime / (kc * sc);
  let term_h = dh_prime_big / (kh * sh);

  (term_l * term_l + term_c * term_c + term_h * term_h + rt * term_c * term_h).sqrt()
}

fn hue_angle(b: f64, a_prime: f64) -> f64 {
  if a_prime == 0.0 && b == 0.0 {
    return 0.0;
  }
  let angle = b.atan2(a_prime).to_degrees();
  if angle < 0.0 { angle + 360.0 } else { angle }
}

#[cfg(test)]
mod test {
  use super::*;

  mod calculate {
    use super::*;

    #[test]
    fn it_returns_zero_for_identical_colors() {
      let color = Xyz::new(0.4, 0.5, 0.3);

      assert_eq!(calculate(color, color), 0.0);
    }

    #[test]
    fn it_is_order_independent() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.4, 0.5, 0.6);

      assert!((calculate(a, b) - calculate(b, a)).abs() < 1e-10);
    }

    #[test]
    fn it_returns_positive_for_different_colors() {
      let a = Xyz::new(0.0, 0.0, 0.0);
      let b = Xyz::new(0.9505, 1.0, 1.089);

      assert!(calculate(a, b) > 0.0);
    }

    #[test]
    fn it_returns_large_value_for_black_vs_white() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);

      assert!(calculate(black, white) > 50.0);
    }

    #[test]
    fn it_returns_below_jnd_for_near_identical_colors() {
      let a = Xyz::new(0.5, 0.5, 0.5);
      let b = Xyz::new(0.5001, 0.5001, 0.5001);

      assert!(calculate(a, b) < JND);
    }

    #[test]
    fn it_increases_with_greater_difference() {
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let near = Xyz::new(0.8, 0.9, 0.95);
      let far = Xyz::new(0.2, 0.3, 0.25);

      assert!(calculate(far, white) > calculate(near, white));
    }

    #[test]
    fn it_produces_known_result_for_lab_pair() {
      // L*=50, a*=2.6772, b*=-79.7751 and L*=50, a*=0, b*=-82.7485
      // Expected ΔE*00 ≈ 2.0425
      let lab1 = Lab::new(50.0, 2.6772, -79.7751);
      let lab2 = Lab::new(50.0, 0.0, -82.7485);

      let result = calculate(lab1.to_xyz(), lab2.to_xyz());
      assert!((result - 2.0425).abs() < 0.01);
    }
  }

  mod calculate_parametric {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_matches_default_with_unit_params() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      assert_eq!(calculate_parametric(a, b, 1.0, 1.0, 1.0), calculate(a, b));
    }

    #[test]
    fn it_changes_with_different_kl() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      let default = calculate(a, b);
      let kl2 = calculate_parametric(a, b, 2.0, 1.0, 1.0);

      assert!((default - kl2).abs() > 1e-10);
    }
  }

  mod hue_angle {
    use super::*;

    #[test]
    fn it_returns_zero_for_achromatic() {
      assert_eq!(hue_angle(0.0, 0.0), 0.0);
    }

    #[test]
    fn it_returns_zero_for_positive_a_axis() {
      assert_eq!(hue_angle(0.0, 1.0), 0.0);
    }

    #[test]
    fn it_returns_90_for_positive_b_axis() {
      assert!((hue_angle(1.0, 0.0) - 90.0).abs() < 1e-10);
    }

    #[test]
    fn it_returns_180_for_negative_a_axis() {
      assert!((hue_angle(0.0, -1.0) - 180.0).abs() < 1e-10);
    }

    #[test]
    fn it_returns_270_for_negative_b_axis() {
      assert!((hue_angle(-1.0, 0.0) - 270.0).abs() < 1e-10);
    }
  }
}
