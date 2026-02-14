//! CMC l:c color difference.
//!
//! Implements the [CMC l:c](https://en.wikipedia.org/wiki/Color_difference#CMC_l:c_(1984))
//! color difference formula developed by the Colour Measurement Committee of the Society
//! of Dyers and Colourists. The formula uses CIE LCh components and is **not symmetric** —
//! the first argument is treated as the reference color.

use crate::space::{Lch, Xyz};

/// Calculates the CMC l:c color difference for perceptibility (l=1, c=1).
///
/// The first argument is the reference color; the second is the sample.
/// This function is **not** order-independent.
pub fn calculate(reference: impl Into<Xyz>, sample: impl Into<Xyz>) -> f64 {
  calculate_parametric(reference, sample, 1.0, 1.0)
}

/// Calculates the CMC l:c color difference for acceptability (l=2, c=1).
///
/// The first argument is the reference color; the second is the sample.
/// This function is **not** order-independent.
pub fn calculate_acceptability(reference: impl Into<Xyz>, sample: impl Into<Xyz>) -> f64 {
  calculate_parametric(reference, sample, 2.0, 1.0)
}

/// Calculates the CMC l:c color difference with custom l and c factors.
///
/// The first argument is the reference color; the second is the sample.
/// This function is **not** order-independent.
///
/// - `l` — lightness weighting factor (1.0 for perceptibility, 2.0 for acceptability)
/// - `c` — chroma weighting factor (typically 1.0)
pub fn calculate_parametric(reference: impl Into<Xyz>, sample: impl Into<Xyz>, l: f64, c: f64) -> f64 {
  let ref_lch = Lch::from(reference.into());
  let smp_lch = Lch::from(sample.into());

  let l1 = ref_lch.l();
  let c1 = ref_lch.c();
  let h1 = ref_lch.h();

  let l2 = smp_lch.l();
  let c2 = smp_lch.c();

  let dl = l1 - l2;
  let dc = c1 - c2;

  // Compute ΔH from Lab differences
  let ref_lab = crate::space::Lab::from(ref_lch.to_xyz());
  let smp_lab = crate::space::Lab::from(smp_lch.to_xyz());
  let da = ref_lab.a() - smp_lab.a();
  let db = ref_lab.b() - smp_lab.b();
  let dh_sq = (da * da + db * db - dc * dc).max(0.0);

  let sl = if l1 < 16.0 {
    0.511
  } else {
    0.040975 * l1 / (1.0 + 0.01765 * l1)
  };
  let sc = 0.0638 * c1 / (1.0 + 0.0131 * c1) + 0.638;

  let h1_rad = h1.to_radians();
  let f = (c1.powi(4) / (c1.powi(4) + 1900.0)).sqrt();
  let t = if (164.0..=345.0).contains(&h1) {
    0.56 + (0.2 * (h1_rad + 168.0_f64.to_radians()).cos()).abs()
  } else {
    0.36 + (0.4 * (h1_rad + 35.0_f64.to_radians()).cos()).abs()
  };
  let sh = sc * (f * t + 1.0 - f);

  let term_l = dl / (l * sl);
  let term_c = dc / (c * sc);
  let term_h = dh_sq / (sh * sh);

  (term_l * term_l + term_c * term_c + term_h).sqrt()
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
    fn it_returns_positive_for_different_colors() {
      let a = Xyz::new(0.0, 0.0, 0.0);
      let b = Xyz::new(0.9505, 1.0, 1.089);

      assert!(calculate(a, b) > 0.0);
    }

    #[test]
    fn it_increases_with_greater_difference() {
      let reference = Xyz::new(0.9505, 1.0, 1.089);
      let near = Xyz::new(0.8, 0.9, 0.95);
      let far = Xyz::new(0.2, 0.3, 0.25);

      assert!(calculate(reference, far) > calculate(reference, near));
    }
  }

  mod calculate_acceptability {
    use super::*;

    #[test]
    fn it_returns_zero_for_identical_colors() {
      let color = Xyz::new(0.4, 0.5, 0.3);

      assert_eq!(calculate_acceptability(color, color), 0.0);
    }

    #[test]
    fn it_differs_from_perceptibility() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      let perceptibility = calculate(a, b);
      let acceptability = calculate_acceptability(a, b);

      assert!((perceptibility - acceptability).abs() > 1e-10);
    }
  }

  mod calculate_parametric {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_matches_perceptibility_with_same_params() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      assert_eq!(calculate_parametric(a, b, 1.0, 1.0), calculate(a, b));
    }

    #[test]
    fn it_matches_acceptability_with_same_params() {
      let a = Xyz::new(0.1, 0.2, 0.3);
      let b = Xyz::new(0.5, 0.6, 0.7);

      assert_eq!(calculate_parametric(a, b, 2.0, 1.0), calculate_acceptability(a, b));
    }
  }
}
