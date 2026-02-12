//! WCAG 2.x contrast ratio calculation.
//!
//! Implements the [WCAG 2.2 contrast ratio](https://www.w3.org/TR/WCAG22/#dfn-contrast-ratio)
//! formula. The contrast ratio ranges from 1:1 (no contrast) to 21:1 (maximum contrast,
//! black on white).

use crate::space::Xyz;

/// WCAG AA minimum contrast ratio for normal text (4.5:1).
pub const AA_NORMAL_TEXT: f64 = 4.5;

/// WCAG AA minimum contrast ratio for large text (3:1).
pub const AA_LARGE_TEXT: f64 = 3.0;

/// WCAG AAA minimum contrast ratio for normal text (7:1).
pub const AAA_NORMAL_TEXT: f64 = 7.0;

/// WCAG AAA minimum contrast ratio for large text (4.5:1).
pub const AAA_LARGE_TEXT: f64 = 4.5;

/// WCAG 2.x contrast ratio between two colors.
///
/// The ratio ranges from 1.0 (identical luminance) to 21.0 (black on white).
/// Use the threshold methods to check against WCAG conformance levels.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ContrastRatio(f64);

impl ContrastRatio {
  /// Returns `true` if the ratio meets WCAG AA for normal text (>= 4.5:1).
  pub fn meets_aa(&self) -> bool {
    self.0 >= AA_NORMAL_TEXT
  }

  /// Returns `true` if the ratio meets WCAG AA for large text (>= 3:1).
  pub fn meets_aa_large_text(&self) -> bool {
    self.0 >= AA_LARGE_TEXT
  }

  /// Returns `true` if the ratio meets WCAG AAA for normal text (>= 7:1).
  pub fn meets_aaa(&self) -> bool {
    self.0 >= AAA_NORMAL_TEXT
  }

  /// Returns `true` if the ratio meets WCAG AAA for large text (>= 4.5:1).
  pub fn meets_aaa_large_text(&self) -> bool {
    self.0 >= AAA_LARGE_TEXT
  }

  /// Returns the raw contrast ratio value.
  pub fn value(&self) -> f64 {
    self.0
  }
}

impl From<ContrastRatio> for f64 {
  fn from(ratio: ContrastRatio) -> Self {
    ratio.0
  }
}

/// Calculates the WCAG 2.x contrast ratio between two colors.
///
/// The result is always >= 1.0 and is order-independent (swapping the two colors
/// produces the same ratio).
pub fn contrast_ratio(color1: impl Into<Xyz>, color2: impl Into<Xyz>) -> ContrastRatio {
  let l1 = color1.into().luminance();
  let l2 = color2.into().luminance();

  let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };

  ContrastRatio((lighter + 0.05) / (darker + 0.05))
}

#[cfg(test)]
mod test {
  use super::*;

  mod contrast_ratio_fn {
    use super::*;

    #[test]
    fn it_returns_max_ratio_for_black_on_white() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let ratio = contrast_ratio(black, white);

      assert!((ratio.value() - 21.0).abs() < 0.01);
    }

    #[test]
    fn it_returns_one_for_identical_colors() {
      let color = Xyz::new(0.4, 0.5, 0.3);
      let ratio = contrast_ratio(color, color);

      assert_eq!(ratio.value(), 1.0);
    }

    #[test]
    fn it_is_order_independent() {
      let a = Xyz::new(0.0, 0.2, 0.0);
      let b = Xyz::new(0.0, 0.8, 0.0);

      assert_eq!(contrast_ratio(a, b).value(), contrast_ratio(b, a).value());
    }

    #[test]
    fn it_meets_aa_for_black_on_white() {
      let black = Xyz::new(0.0, 0.0, 0.0);
      let white = Xyz::new(0.9505, 1.0, 1.089);
      let ratio = contrast_ratio(black, white);

      assert!(ratio.meets_aa());
      assert!(ratio.meets_aaa());
    }

    #[test]
    fn it_depends_only_on_luminance() {
      let a = Xyz::new(0.1, 0.5, 0.9);
      let b = Xyz::new(0.9, 0.5, 0.1);
      let ratio = contrast_ratio(a, b);

      assert_eq!(ratio.value(), 1.0);
    }

    #[test]
    fn it_never_returns_below_one() {
      let a = Xyz::new(0.0, 0.001, 0.0);
      let b = Xyz::new(0.0, 0.002, 0.0);
      let ratio = contrast_ratio(a, b);

      assert!(ratio.value() >= 1.0);
    }

    #[test]
    fn it_fails_aa_for_low_contrast_pair() {
      let a = Xyz::new(0.0, 0.5, 0.0);
      let b = Xyz::new(0.0, 0.6, 0.0);
      let ratio = contrast_ratio(a, b);

      assert!(!ratio.meets_aa());
    }
  }

  mod contrast_ratio_type {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_returns_value() {
      let ratio = ContrastRatio(4.5);

      assert_eq!(ratio.value(), 4.5);
    }

    #[test]
    fn it_converts_to_f64() {
      let ratio = ContrastRatio(4.5);
      let value: f64 = ratio.into();

      assert_eq!(value, 4.5);
    }

    #[test]
    fn it_meets_aa_at_threshold() {
      assert!(ContrastRatio(4.5).meets_aa());
      assert!(!ContrastRatio(4.49).meets_aa());
    }

    #[test]
    fn it_meets_aa_large_text_at_threshold() {
      assert!(ContrastRatio(3.0).meets_aa_large_text());
      assert!(!ContrastRatio(2.99).meets_aa_large_text());
    }

    #[test]
    fn it_meets_aaa_at_threshold() {
      assert!(ContrastRatio(7.0).meets_aaa());
      assert!(!ContrastRatio(6.99).meets_aaa());
    }

    #[test]
    fn it_meets_aaa_large_text_at_threshold() {
      assert!(ContrastRatio(4.5).meets_aaa_large_text());
      assert!(!ContrastRatio(4.49).meets_aaa_large_text());
    }

    #[test]
    fn it_fails_all_thresholds_at_one() {
      let ratio = ContrastRatio(1.0);

      assert!(!ratio.meets_aa());
      assert!(!ratio.meets_aa_large_text());
      assert!(!ratio.meets_aaa());
      assert!(!ratio.meets_aaa_large_text());
    }
  }
}
