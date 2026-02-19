#![cfg(feature = "serde")]

use farg::space::{ColorSpace, Rgb, Srgb, Xyz};

mod xyz {
  use super::*;

  #[test]
  fn it_roundtrips_through_json() {
    let color = Xyz::new(0.5, 0.4, 0.3);
    let json = serde_json::to_string(&color).unwrap();
    let back: Xyz = serde_json::from_str(&json).unwrap();

    assert_eq!(color.x(), back.x());
    assert_eq!(color.y(), back.y());
    assert_eq!(color.z(), back.z());
    assert_eq!(back.alpha(), 1.0);
  }

  #[test]
  fn it_skips_alpha_when_opaque() {
    let color = Xyz::new(0.5, 0.4, 0.3);
    let json = serde_json::to_string(&color).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert!(value.get("alpha").is_none());
  }

  #[test]
  fn it_includes_alpha_when_translucent() {
    let color = Xyz::new(0.5, 0.4, 0.3).with_alpha(0.7);
    let json = serde_json::to_string(&color).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert!(value.get("alpha").is_some());
  }

  #[test]
  fn it_defaults_alpha_to_one_on_deserialize() {
    let json = r#"{"x":0.5,"y":0.4,"z":0.3}"#;
    let color: Xyz = serde_json::from_str(json).unwrap();

    assert_eq!(color.alpha(), 1.0);
  }

  #[test]
  fn it_roundtrips_with_alpha() {
    let color = Xyz::new(0.5, 0.4, 0.3).with_alpha(0.5);
    let json = serde_json::to_string(&color).unwrap();
    let back: Xyz = serde_json::from_str(&json).unwrap();

    assert!((back.alpha() - 0.5).abs() < 1e-10);
  }
}

mod rgb {
  use super::*;

  #[test]
  fn it_roundtrips_through_json() {
    let color = Rgb::<Srgb>::new(200, 100, 50);
    let json = serde_json::to_string(&color).unwrap();
    let back: Rgb<Srgb> = serde_json::from_str(&json).unwrap();

    assert_eq!(color.red(), back.red());
    assert_eq!(color.green(), back.green());
    assert_eq!(color.blue(), back.blue());
  }

  #[test]
  fn it_serializes_with_r_g_b_fields() {
    let color = Rgb::<Srgb>::new(200, 100, 50);
    let value: serde_json::Value = serde_json::to_value(&color).unwrap();

    assert!(value.get("r").is_some());
    assert!(value.get("g").is_some());
    assert!(value.get("b").is_some());
  }
}

mod lms {
  use farg::space::Lms;

  #[test]
  fn it_roundtrips_through_json() {
    let color = Lms::new(0.5, 0.4, 0.3);
    let json = serde_json::to_string(&color).unwrap();
    let back: Lms = serde_json::from_str(&json).unwrap();

    assert_eq!(color.l(), back.l());
    assert_eq!(color.m(), back.m());
    assert_eq!(color.s(), back.s());
  }
}

#[cfg(feature = "space-xyy")]
mod xyy {
  use farg::space::{ColorSpace, Xyy};

  #[test]
  fn it_uses_renamed_fields() {
    let color = Xyy::new(0.3127, 0.3290, 1.0);
    let value: serde_json::Value = serde_json::to_value(&color).unwrap();

    assert!(value.get("x").is_some());
    assert!(value.get("y").is_some());
    assert!(value.get("Y").is_some());
    assert!(value.get("x_chrom").is_none());
    assert!(value.get("big_y").is_none());
  }

  #[test]
  fn it_roundtrips_through_json() {
    let color = Xyy::new(0.3127, 0.3290, 1.0);
    let json = serde_json::to_string(&color).unwrap();
    let back: Xyy = serde_json::from_str(&json).unwrap();

    assert!((color.x() - back.x()).abs() < 1e-10);
    assert!((color.y() - back.y()).abs() < 1e-10);
    assert!((color.big_y() - back.big_y()).abs() < 1e-10);
  }

  #[test]
  fn it_deserializes_with_capital_y() {
    let json = r#"{"x":0.3127,"y":0.329,"Y":1.0}"#;
    let color: Xyy = serde_json::from_str(json).unwrap();

    assert!((color.big_y() - 1.0).abs() < 1e-10);
  }

  #[test]
  fn it_roundtrips_with_alpha() {
    let color = Xyy::new(0.3127, 0.3290, 1.0).with_alpha(0.3);
    let json = serde_json::to_string(&color).unwrap();
    let back: Xyy = serde_json::from_str(&json).unwrap();

    assert!((back.alpha() - 0.3).abs() < 1e-10);
  }
}

#[cfg(feature = "space-lab")]
mod lab {
  use farg::space::Lab;

  #[test]
  fn it_roundtrips_through_json() {
    let color = Lab::new(50.0, 25.0, -10.0);
    let json = serde_json::to_string(&color).unwrap();
    let back: Lab = serde_json::from_str(&json).unwrap();

    assert_eq!(color.l(), back.l());
    assert_eq!(color.a(), back.a());
    assert_eq!(color.b(), back.b());
  }
}

#[cfg(feature = "space-oklab")]
mod oklab {
  use farg::space::Oklab;

  #[test]
  fn it_roundtrips_through_json() {
    let color = Oklab::new(0.7, 0.1, -0.05);
    let json = serde_json::to_string(&color).unwrap();
    let back: Oklab = serde_json::from_str(&json).unwrap();

    assert_eq!(color.l(), back.l());
    assert_eq!(color.a(), back.a());
    assert_eq!(color.b(), back.b());
  }
}

#[cfg(feature = "space-hsl")]
mod hsl {
  use farg::space::{ColorSpace, Hsl, Srgb};

  #[test]
  fn it_roundtrips_through_json() {
    let color = Hsl::<Srgb>::new(180.0, 50.0, 50.0);
    let json = serde_json::to_string(&color).unwrap();
    let back: Hsl<Srgb> = serde_json::from_str(&json).unwrap();

    assert_eq!(color.hue(), back.hue());
    assert_eq!(color.saturation(), back.saturation());
    assert_eq!(color.lightness(), back.lightness());
  }

  #[test]
  fn it_roundtrips_with_alpha() {
    let color = Hsl::<Srgb>::new(180.0, 50.0, 50.0).with_alpha(0.8);
    let json = serde_json::to_string(&color).unwrap();
    let back: Hsl<Srgb> = serde_json::from_str(&json).unwrap();

    assert!((back.alpha() - 0.8).abs() < 1e-10);
  }
}

#[cfg(feature = "space-cmyk")]
mod cmyk {
  use farg::space::{Cmyk, Srgb};

  #[test]
  fn it_roundtrips_through_json() {
    let color = Cmyk::<Srgb>::new(25.0, 50.0, 75.0, 10.0);
    let json = serde_json::to_string(&color).unwrap();
    let back: Cmyk<Srgb> = serde_json::from_str(&json).unwrap();

    assert_eq!(color.cyan(), back.cyan());
    assert_eq!(color.magenta(), back.magenta());
    assert_eq!(color.yellow(), back.yellow());
    assert_eq!(color.key(), back.key());
  }

  #[test]
  fn it_serializes_four_component_fields() {
    let color = Cmyk::<Srgb>::new(25.0, 50.0, 75.0, 10.0);
    let value: serde_json::Value = serde_json::to_value(&color).unwrap();

    assert!(value.get("c").is_some());
    assert!(value.get("m").is_some());
    assert!(value.get("y").is_some());
    assert!(value.get("k").is_some());
  }
}
