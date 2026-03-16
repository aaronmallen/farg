//! Viénot, Brettel & Mollon (1999) dichromacy simulation.
//!
//! A simplified single-plane projection method that operates directly in linear sRGB space
//! using precomputed 3x3 simulation matrices. Faster than Brettel's half-plane method but
//! slightly less accurate for extreme colors.
//!
//! # Reference
//!
//! Viénot, F., Brettel, H., & Mollon, J. D. (1999). "Digital video colourmaps for checking
//! the legibility of displays by dichromats." *Color Research & Application*, 24(4), 243–252.

use crate::{
  matrix::Matrix3,
  space::{Srgb, Xyz},
};

/// Viénot protanopia simulation matrix (linear sRGB).
///
/// Derived from Viénot et al. 1999, Table 1.
const PROTAN: Matrix3 = Matrix3::new([
  [0.152286, 1.052583, -0.204868],
  [0.114503, 0.786281, 0.099216],
  [-0.003882, -0.048116, 1.051998],
]);

/// Viénot deuteranopia simulation matrix (linear sRGB).
///
/// Derived from Viénot et al. 1999, Table 1.
const DEUTAN: Matrix3 = Matrix3::new([
  [0.367322, 0.860646, -0.227968],
  [0.280085, 0.672501, 0.047413],
  [-0.011820, 0.042940, 0.968881],
]);

/// Viénot tritanopia simulation matrix (linear sRGB).
///
/// Derived from Brettel et al. 1997 single-plane approximation for tritanopia.
const TRITAN: Matrix3 = Matrix3::new([
  [1.255528, -0.076749, -0.178779],
  [-0.078411, 0.930809, 0.147602],
  [0.004733, 0.691367, 0.303900],
]);

/// Applies a Viénot simulation matrix in linear sRGB space.
fn simulate(xyz: Xyz, matrix: Matrix3) -> Xyz {
  let rgb = xyz.to_rgb::<Srgb>();
  let linear = rgb.to_linear().components();
  let simulated = matrix * linear;
  let result = crate::space::LinearRgb::<Srgb>::from_normalized(simulated[0], simulated[1], simulated[2]);
  result.to_encoded().to_xyz()
}

/// Simulates protanopia (L-cone loss) using the Viénot 1999 algorithm.
///
/// ```
/// # #[cfg(feature = "cvd-vienot")]
/// # {
/// use farg::color_vision_deficiency::vienot;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let simulated = vienot::protanopia(color);
/// # }
/// ```
pub fn protanopia(color: impl Into<Xyz>) -> Xyz {
  simulate(color.into(), PROTAN)
}

/// Simulates deuteranopia (M-cone loss) using the Viénot 1999 algorithm.
///
/// ```
/// # #[cfg(feature = "cvd-vienot")]
/// # {
/// use farg::color_vision_deficiency::vienot;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let simulated = vienot::deuteranopia(color);
/// # }
/// ```
pub fn deuteranopia(color: impl Into<Xyz>) -> Xyz {
  simulate(color.into(), DEUTAN)
}

/// Simulates tritanopia (S-cone loss) using the Viénot 1999 algorithm.
///
/// ```
/// # #[cfg(feature = "cvd-vienot")]
/// # {
/// use farg::color_vision_deficiency::vienot;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let simulated = vienot::tritanopia(color);
/// # }
/// ```
pub fn tritanopia(color: impl Into<Xyz>) -> Xyz {
  simulate(color.into(), TRITAN)
}

#[cfg(test)]
mod test {
  use super::*;

  mod protanopia_fn {
    use super::*;

    #[test]
    fn it_returns_valid_xyz() {
      let result = protanopia(Xyz::new(0.4, 0.3, 0.2));

      assert!(result.x().is_finite());
      assert!(result.y().is_finite());
      assert!(result.z().is_finite());
    }

    #[test]
    fn it_preserves_black() {
      let result = protanopia(Xyz::new(0.0, 0.0, 0.0));

      assert!(result.x().abs() < 1e-6);
      assert!(result.y().abs() < 1e-6);
      assert!(result.z().abs() < 1e-6);
    }

    #[test]
    fn it_changes_chromatic_colors() {
      let original = Xyz::new(0.4, 0.2, 0.1);
      let result = protanopia(original);

      assert!((result.x() - original.x()).abs() > 1e-4 || (result.z() - original.z()).abs() > 1e-4);
    }
  }

  mod deuteranopia_fn {
    use super::*;

    #[test]
    fn it_returns_valid_xyz() {
      let result = deuteranopia(Xyz::new(0.4, 0.3, 0.2));

      assert!(result.x().is_finite());
      assert!(result.y().is_finite());
      assert!(result.z().is_finite());
    }

    #[test]
    fn it_preserves_black() {
      let result = deuteranopia(Xyz::new(0.0, 0.0, 0.0));

      assert!(result.x().abs() < 1e-6);
      assert!(result.y().abs() < 1e-6);
      assert!(result.z().abs() < 1e-6);
    }

    #[test]
    fn it_changes_chromatic_colors() {
      let original = Xyz::new(0.2, 0.4, 0.1);
      let result = deuteranopia(original);

      assert!((result.x() - original.x()).abs() > 1e-4 || (result.y() - original.y()).abs() > 1e-4);
    }
  }

  mod tritanopia_fn {
    use super::*;

    #[test]
    fn it_returns_valid_xyz() {
      let result = tritanopia(Xyz::new(0.4, 0.3, 0.2));

      assert!(result.x().is_finite());
      assert!(result.y().is_finite());
      assert!(result.z().is_finite());
    }

    #[test]
    fn it_preserves_black() {
      let result = tritanopia(Xyz::new(0.0, 0.0, 0.0));

      assert!(result.x().abs() < 1e-6);
      assert!(result.y().abs() < 1e-6);
      assert!(result.z().abs() < 1e-6);
    }

    #[test]
    fn it_changes_chromatic_colors() {
      let original = Xyz::new(0.2, 0.2, 0.4);
      let result = tritanopia(original);

      assert!((result.z() - original.z()).abs() > 1e-4);
    }
  }
}
