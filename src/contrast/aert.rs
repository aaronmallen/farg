//! AERT brightness difference contrast calculation.
//!
//! Implements the brightness difference formula from the
//! [W3C AERT](https://www.w3.org/TR/AERT/#color-contrast) accessibility evaluation. Computes
//! the absolute difference in BT.601 perceived brightness between two colors. Values range
//! from 0 (identical brightness) to 255 (maximum difference, black vs white).

use crate::space::{Rgb, Srgb, Xyz};

/// BT.601 luma coefficient for the red channel.
pub const RED_COEFFICIENT: f64 = 0.299;

/// BT.601 luma coefficient for the green channel.
pub const GREEN_COEFFICIENT: f64 = 0.587;

/// BT.601 luma coefficient for the blue channel.
pub const BLUE_COEFFICIENT: f64 = 0.114;

/// AERT recommended minimum brightness difference for accessible text.
pub const RECOMMENDED_MINIMUM: f64 = 125.0;

/// Calculates the AERT brightness difference between two colors.
///
/// Converts both colors to sRGB, computes BT.601 perceived brightness for each
/// (`0.299R + 0.587G + 0.114B` on a 0-255 scale), and returns the absolute difference.
/// The result ranges from 0.0 to 255.0.
pub fn calculate(color1: impl Into<Xyz>, color2: impl Into<Xyz>) -> f64 {
  let rgb1 = Rgb::<Srgb>::from(color1.into());
  let rgb2 = Rgb::<Srgb>::from(color2.into());

  (brightness(rgb1) - brightness(rgb2)).abs()
}

fn brightness(rgb: Rgb<Srgb>) -> f64 {
  RED_COEFFICIENT * rgb.red() as f64 + GREEN_COEFFICIENT * rgb.green() as f64 + BLUE_COEFFICIENT * rgb.blue() as f64
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
      let a = Xyz::new(0.2, 0.3, 0.1);
      let b = Xyz::new(0.5, 0.6, 0.7);

      assert_eq!(calculate(a, b), calculate(b, a));
    }

    #[test]
    fn it_returns_max_for_black_and_white() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let result = calculate(black, white);

      assert!((result - 255.0).abs() < 1.0);
    }

    #[test]
    fn it_exceeds_recommended_minimum_for_black_and_white() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);

      assert!(calculate(black, white) >= RECOMMENDED_MINIMUM);
    }

    #[test]
    fn it_increases_with_greater_brightness_difference() {
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let mid = Xyz::new(0.2, 0.2, 0.2);
      let dark = Xyz::new(0.05, 0.05, 0.05);

      assert!(calculate(dark, white) > calculate(mid, white));
    }
  }

  mod brightness_fn {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_zero_for_black() {
      let black = Rgb::<Srgb>::new(0, 0, 0);

      assert_eq!(brightness(black), 0.0);
    }

    #[test]
    fn it_returns_255_for_white() {
      let white = Rgb::<Srgb>::new(255, 255, 255);

      assert_eq!(brightness(white), 255.0);
    }

    #[test]
    fn it_weights_green_highest() {
      let red = Rgb::<Srgb>::new(255, 0, 0);
      let green = Rgb::<Srgb>::new(0, 255, 0);
      let blue = Rgb::<Srgb>::new(0, 0, 255);

      assert!(brightness(green) > brightness(red));
      assert!(brightness(red) > brightness(blue));
    }
  }
}
