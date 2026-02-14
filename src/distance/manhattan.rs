//! Manhattan distance in CIE XYZ space.
//!
//! Computes the taxicab (L1) distance between two colors in the CIE XYZ tristimulus space.
//! This sums the absolute differences along each axis rather than computing a straight-line
//! distance.

use crate::space::Xyz;

/// Calculates the Manhattan distance between two colors in CIE XYZ space.
///
/// Returns `|X₁-X₂| + |Y₁-Y₂| + |Z₁-Z₂|`. The result is always >= 0.0
/// and is order-independent.
pub fn calculate(color1: impl Into<Xyz>, color2: impl Into<Xyz>) -> f64 {
  let [x1, y1, z1] = color1.into().components();
  let [x2, y2, z2] = color2.into().components();

  (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
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
    fn it_computes_expected_distance() {
      let a = Xyz::new(0.0, 0.0, 0.0);
      let b = Xyz::new(3.0, 4.0, 5.0);

      assert!((calculate(a, b) - 12.0).abs() < 1e-10);
    }

    #[test]
    fn it_is_always_gte_euclidean() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.4, 0.6, 0.8);
      let manhattan = calculate(a, b);
      let euclidean = {
        let [x1, y1, z1] = a.components();
        let [x2, y2, z2] = b.components();
        ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
      };

      assert!(manhattan >= euclidean);
    }

    #[test]
    fn it_handles_single_axis_difference() {
      let a = Xyz::new(0.0, 0.0, 0.0);
      let b = Xyz::new(0.0, 0.5, 0.0);

      assert!((calculate(a, b) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn it_increases_with_greater_separation() {
      let origin = Xyz::new(0.0, 0.0, 0.0);
      let near = Xyz::new(0.1, 0.1, 0.1);
      let far = Xyz::new(0.5, 0.5, 0.5);

      assert!(calculate(origin, far) > calculate(origin, near));
    }
  }
}
