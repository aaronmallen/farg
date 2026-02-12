//! Michelson contrast calculation.
//!
//! Implements the [Michelson contrast](https://en.wikipedia.org/wiki/Contrast_(vision)#Michelson_contrast)
//! formula, also known as visibility or modulation contrast. Defined as
//! `(L_max - L_min) / (L_max + L_min)`, it produces values from 0.0 (no contrast)
//! to 1.0 (maximum contrast). Originally developed for sinusoidal gratings.

use crate::space::Xyz;

/// Calculates Michelson contrast between two colors.
///
/// The result ranges from 0.0 (identical luminance) to 1.0 (maximum contrast)
/// and is order-independent.
pub fn calculate(color1: impl Into<Xyz>, color2: impl Into<Xyz>) -> f64 {
  let l1 = color1.into().luminance();
  let l2 = color2.into().luminance();

  let (l_max, l_min) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
  let sum = l_max + l_min;

  if sum == 0.0 { 0.0 } else { (l_max - l_min) / sum }
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
    fn it_returns_one_for_maximum_contrast() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);

      assert_eq!(calculate(black, white), 1.0);
    }

    #[test]
    fn it_is_order_independent() {
      let a = Xyz::new(0.0, 0.2, 0.0);
      let b = Xyz::new(0.0, 0.8, 0.0);

      assert_eq!(calculate(a, b), calculate(b, a));
    }

    #[test]
    fn it_returns_zero_for_two_black_colors() {
      let black = Xyz::new(0.0, 0.0, 0.0);

      assert_eq!(calculate(black, black), 0.0);
    }

    #[test]
    fn it_returns_value_between_zero_and_one() {
      let a = Xyz::new(0.0, 0.3, 0.0);
      let b = Xyz::new(0.0, 0.7, 0.0);
      let result = calculate(a, b);

      assert!(result > 0.0);
      assert!(result < 1.0);
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
