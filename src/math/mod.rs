use angles::Angle;
pub use rand;

pub mod angles;
#[allow(non_upper_case_globals)]
pub mod constants;
pub mod phys;

/// sin(x) as ``f32``, returned as radians
pub fn sin_f32(x: f32) -> Angle {
    Angle::Radians(x.sin())
}

/// sin⁻¹(x) as ``f32``, returned as radians
pub fn asin_f32(x: f32) -> Angle {
    Angle::Radians(x.asin())
}

#[allow(non_snake_case)]
/// sin⁻¹(x) as ``Angle``
pub fn asin_Angle(x: Angle) -> Angle {
    Angle::Radians(x.as_radians().value().asin())
}

pub fn sin_f64(x: f64) -> Angle {
    Angle::Radians(x.sin() as f32)
}

pub fn cos_f32(x: f32) -> Angle {
    Angle::Radians(x.cos())
}
