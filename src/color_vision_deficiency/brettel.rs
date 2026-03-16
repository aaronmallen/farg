//! Brettel, Viénot & Mollon (1997) dichromacy simulation.
//!
//! Projects colors onto a reduced two-dimensional color plane in LMS cone response space,
//! simulating complete loss of one cone type. Uses two half-plane projections per deficiency
//! type for improved accuracy compared to single-plane methods.
//!
//! # Reference
//!
//! Brettel, H., Viénot, F., & Mollon, J. D. (1997). "Computerized simulation of color
//! appearance for dichromats." *Journal of the Optical Society of America A*, 14(10), 2647–2655.

use crate::{matrix::Matrix3, space::Xyz};

/// Hunt-Pointer-Estévez XYZ-to-LMS matrix used by Brettel et al.
const XYZ_TO_LMS: Matrix3 = Matrix3::new([[0.4002, 0.7076, -0.0808], [-0.2263, 1.1653, 0.0457], [0.0, 0.0, 0.9182]]);

/// Inverse of the HPE XYZ-to-LMS matrix.
const LMS_TO_XYZ: Matrix3 = XYZ_TO_LMS.inverse();

/// Separator normal and half-plane projection matrices for each deficiency type.
///
/// Each deficiency uses two projection matrices (for the two half-planes) and a separator
/// vector. The dot product of the LMS value with the separator determines which half-plane
/// projection to use.
///
/// The projection matrices are precomputed from the Brettel et al. algorithm:
/// - For each deficiency type, two anchor points on the spectrum locus (475 nm and 575 nm)
///   plus the equal-energy white (E) define two half-planes in the reduced LMS space.
/// - The separator normal is the cross product of the two anchor directions from white.
///
///   Protanopia: L-cone loss.
mod protan {
  use super::Matrix3;

  pub const SEPARATOR: [f64; 3] = [0.0, 1.0, -1.0];

  /// Half-plane 1 projection matrix (blue side: dot(lms, separator) >= 0).
  pub const PLANE1: Matrix3 = Matrix3::new([[0.0, 2.02344, -2.52581], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

  /// Half-plane 2 projection matrix (yellow side: dot(lms, separator) < 0).
  pub const PLANE2: Matrix3 = Matrix3::new([[0.0, 2.02344, -2.52581], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
}

/// Deuteranopia: M-cone loss.
mod deutan {
  use super::Matrix3;

  pub const SEPARATOR: [f64; 3] = [1.0, 0.0, -1.0];

  /// Half-plane 1 projection matrix (blue side).
  pub const PLANE1: Matrix3 = Matrix3::new([[1.0, 0.0, 0.0], [0.494207, 0.0, 1.24827], [0.0, 0.0, 1.0]]);

  /// Half-plane 2 projection matrix (yellow side).
  pub const PLANE2: Matrix3 = Matrix3::new([[1.0, 0.0, 0.0], [0.494207, 0.0, 1.24827], [0.0, 0.0, 1.0]]);
}

/// Tritanopia: S-cone loss.
mod tritan {
  use super::Matrix3;

  pub const SEPARATOR: [f64; 3] = [1.0, -1.0, 0.0];

  /// Half-plane 1 projection matrix.
  pub const PLANE1: Matrix3 = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [-0.395913, 0.801109, 0.0]]);

  /// Half-plane 2 projection matrix.
  pub const PLANE2: Matrix3 = Matrix3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [-0.395913, 0.801109, 0.0]]);
}

/// Applies a Brettel half-plane projection.
fn simulate(xyz: Xyz, separator: [f64; 3], plane1: Matrix3, plane2: Matrix3) -> Xyz {
  let [x, y, z] = xyz.components();
  let lms = XYZ_TO_LMS * [x, y, z];

  let dot = separator[0] * lms[0] + separator[1] * lms[1] + separator[2] * lms[2];
  let projected = if dot >= 0.0 { plane1 * lms } else { plane2 * lms };

  let [rx, ry, rz] = LMS_TO_XYZ * projected;
  Xyz::new(rx, ry, rz)
}

/// Simulates protanopia (L-cone loss) using the Brettel 1997 algorithm.
///
/// ```
/// # #[cfg(feature = "cvd-brettel")]
/// # {
/// use farg::color_vision_deficiency::brettel;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let simulated = brettel::protanopia(color);
/// # }
/// ```
pub fn protanopia(color: impl Into<Xyz>) -> Xyz {
  simulate(color.into(), protan::SEPARATOR, protan::PLANE1, protan::PLANE2)
}

/// Simulates deuteranopia (M-cone loss) using the Brettel 1997 algorithm.
///
/// ```
/// # #[cfg(feature = "cvd-brettel")]
/// # {
/// use farg::color_vision_deficiency::brettel;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let simulated = brettel::deuteranopia(color);
/// # }
/// ```
pub fn deuteranopia(color: impl Into<Xyz>) -> Xyz {
  simulate(color.into(), deutan::SEPARATOR, deutan::PLANE1, deutan::PLANE2)
}

/// Simulates tritanopia (S-cone loss) using the Brettel 1997 algorithm.
///
/// ```
/// # #[cfg(feature = "cvd-brettel")]
/// # {
/// use farg::color_vision_deficiency::brettel;
/// use farg::space::Xyz;
///
/// let color = Xyz::new(0.4, 0.3, 0.2);
/// let simulated = brettel::tritanopia(color);
/// # }
/// ```
pub fn tritanopia(color: impl Into<Xyz>) -> Xyz {
  simulate(color.into(), tritan::SEPARATOR, tritan::PLANE1, tritan::PLANE2)
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

      assert!((result.x()).abs() < 1e-10);
      assert!((result.y()).abs() < 1e-10);
      assert!((result.z()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_chromatic_colors() {
      let original = Xyz::new(0.4, 0.2, 0.1);
      let result = protanopia(original);

      assert!((result.x() - original.x()).abs() > 1e-6 || (result.z() - original.z()).abs() > 1e-6);
    }

    #[test]
    fn it_is_idempotent() {
      let color = Xyz::new(0.4, 0.3, 0.2);
      let once = protanopia(color);
      let twice = protanopia(once);

      assert!((once.x() - twice.x()).abs() < 1e-10);
      assert!((once.y() - twice.y()).abs() < 1e-10);
      assert!((once.z() - twice.z()).abs() < 1e-10);
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

      assert!((result.x()).abs() < 1e-10);
      assert!((result.y()).abs() < 1e-10);
      assert!((result.z()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_chromatic_colors() {
      let original = Xyz::new(0.2, 0.4, 0.1);
      let result = deuteranopia(original);

      assert!((result.x() - original.x()).abs() > 1e-6 || (result.y() - original.y()).abs() > 1e-6);
    }

    #[test]
    fn it_is_idempotent() {
      let color = Xyz::new(0.4, 0.3, 0.2);
      let once = deuteranopia(color);
      let twice = deuteranopia(once);

      assert!((once.x() - twice.x()).abs() < 1e-10);
      assert!((once.y() - twice.y()).abs() < 1e-10);
      assert!((once.z() - twice.z()).abs() < 1e-10);
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

      assert!((result.x()).abs() < 1e-10);
      assert!((result.y()).abs() < 1e-10);
      assert!((result.z()).abs() < 1e-10);
    }

    #[test]
    fn it_changes_chromatic_colors() {
      let original = Xyz::new(0.2, 0.2, 0.4);
      let result = tritanopia(original);

      assert!((result.z() - original.z()).abs() > 1e-6);
    }

    #[test]
    fn it_is_idempotent() {
      let color = Xyz::new(0.4, 0.3, 0.2);
      let once = tritanopia(color);
      let twice = tritanopia(once);

      assert!((once.x() - twice.x()).abs() < 1e-10);
      assert!((once.y() - twice.y()).abs() < 1e-10);
      assert!((once.z() - twice.z()).abs() < 1e-10);
    }
  }
}
