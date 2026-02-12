//! RMS (Root Mean Square) contrast calculation.
//!
//! Computes the standard deviation of the luminance values of two colors. For a pair of
//! colors this simplifies to `|L₁ - L₂| / 2`. While RMS contrast is primarily used for
//! image regions with many samples, it provides a simple absolute measure of luminance
//! spread for color pairs.

use crate::space::Xyz;

/// Calculates RMS contrast between two colors.
///
/// Returns the standard deviation of the two luminance values, which simplifies
/// to `|L₁ - L₂| / 2`. The result is order-independent and always >= 0.0.
pub fn calculate(color1: impl Into<Xyz>, color2: impl Into<Xyz>) -> f64 {
  let l1 = color1.into().luminance();
  let l2 = color2.into().luminance();

  (l1 - l2).abs() / 2.0
}

#[cfg(test)]
mod test {
  use super::*;

  mod calculate {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_zero_for_identical_colors() {
      let color = Xyz::new(0.4, 0.5, 0.3);

      assert_eq!(calculate(color, color), 0.0);
    }

    #[test]
    fn it_is_order_independent() {
      let a = Xyz::new(0.0, 0.2, 0.0);
      let b = Xyz::new(0.0, 0.8, 0.0);

      assert_eq!(calculate(a, b), calculate(b, a));
    }

    #[test]
    fn it_returns_half_the_luminance_difference() {
      let a = Xyz::new(0.0, 0.2, 0.0);
      let b = Xyz::new(0.0, 0.8, 0.0);
      let result = calculate(a, b);

      assert!((result - 0.3).abs() < 1e-10);
    }

    #[test]
    fn it_returns_half_for_black_and_white() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);

      assert_eq!(calculate(black, white), 0.5);
    }

    #[test]
    fn it_increases_with_greater_luminance_difference() {
      let anchor = Xyz::new(0.0, 0.5, 0.0);
      let close = Xyz::new(0.0, 0.6, 0.0);
      let far = Xyz::new(0.0, 0.9, 0.0);

      assert!(calculate(anchor, far) > calculate(anchor, close));
    }
  }
}
