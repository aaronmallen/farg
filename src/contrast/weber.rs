//! Weber contrast calculation.
//!
//! Implements the [Weber contrast](https://en.wikipedia.org/wiki/Contrast_(vision)#Weber_contrast)
//! formula, defined as `(L_target - L_background) / L_background`. Used in psychophysics for
//! measuring the visibility of a target against a uniform background. The result is signed:
//! positive when the target is brighter than the background, negative when darker.

use crate::space::Xyz;

/// Calculates Weber contrast of a target color against a background.
///
/// Returns `(L_target - L_background) / L_background`. Positive values indicate the
/// target is brighter; negative values indicate it is darker. Returns [`f64::INFINITY`]
/// when the background is black and the target is not, or 0.0 when both are black.
pub fn calculate(color: impl Into<Xyz>, background: impl Into<Xyz>) -> f64 {
  let l_target = color.into().luminance();
  let l_bg = background.into().luminance();

  if l_bg == 0.0 {
    if l_target == 0.0 { 0.0 } else { f64::INFINITY }
  } else {
    (l_target - l_bg) / l_bg
  }
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
    fn it_returns_positive_for_brighter_target() {
      let bright = Xyz::new(0.0, 0.8, 0.0);
      let dark = Xyz::new(0.0, 0.2, 0.0);

      assert!(calculate(bright, dark) > 0.0);
    }

    #[test]
    fn it_returns_negative_for_darker_target() {
      let dark = Xyz::new(0.0, 0.2, 0.0);
      let bright = Xyz::new(0.0, 0.8, 0.0);

      assert!(calculate(dark, bright) < 0.0);
    }

    #[test]
    fn it_returns_infinity_for_nonblack_on_black() {
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let black = Xyz::new(0.0, 0.0, 0.0);

      assert!(calculate(white, black).is_infinite());
    }

    #[test]
    fn it_returns_zero_for_black_on_black() {
      let black = Xyz::new(0.0, 0.0, 0.0);

      assert_eq!(calculate(black, black), 0.0);
    }

    #[test]
    fn it_computes_expected_ratio() {
      let target = Xyz::new(0.0, 0.6, 0.0);
      let background = Xyz::new(0.0, 0.2, 0.0);
      let result = calculate(target, background);

      assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn it_returns_negative_one_for_black_on_any_background() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let background = Xyz::new(0.0, 0.5, 0.0);

      assert_eq!(calculate(black, background), -1.0);
    }
  }
}
