//! CIE76 color difference (ΔE\*ab).
//!
//! Implements the [CIE76](https://en.wikipedia.org/wiki/Color_difference#CIE76) color difference
//! formula, which is the Euclidean distance in CIELAB space. This was the first standardized
//! color difference metric and remains widely used for its simplicity.

use crate::space::{Lab, Xyz};

/// Calculates the CIE76 color difference (ΔE\*ab) between two colors.
///
/// Returns `sqrt((ΔL*)² + (Δa*)² + (Δb*)²)` in CIELAB space. The result is always >= 0.0
/// and is order-independent.
pub fn calculate(color1: impl Into<Xyz>, color2: impl Into<Xyz>) -> f64 {
  let lab1 = Lab::from(color1.into());
  let lab2 = Lab::from(color2.into());

  let dl = lab1.l() - lab2.l();
  let da = lab1.a() - lab2.a();
  let db = lab1.b() - lab2.b();

  (dl * dl + da * da + db * db).sqrt()
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

      assert_eq!(calculate(a, b), calculate(b, a));
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

      assert!(calculate(black, white) > 90.0);
    }

    #[test]
    fn it_increases_with_greater_perceptual_difference() {
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let mid_gray = Xyz::new(0.2034, 0.2140, 0.2330);
      let dark_gray = Xyz::new(0.0500, 0.0527, 0.0573);

      assert!(calculate(dark_gray, white) > calculate(mid_gray, white));
    }

    #[test]
    fn it_detects_small_differences() {
      let a = Xyz::new(0.4, 0.5, 0.3);
      let b = Xyz::new(0.401, 0.501, 0.301);

      let delta_e = calculate(a, b);
      assert!(delta_e > 0.0);
      assert!(delta_e < 5.0);
    }
  }
}
