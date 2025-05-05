use std::ops::{Add, AddAssign, Div, Mul, Sub};

use super::constants::PI_f32;

/// Degrees, radians, and milliradians as ``f32``
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    Degrees(f32),
    Radians(f32),
    MilliRadians(f32),
}

impl Angle {
    pub fn as_degress(self) -> Self {
        return match self {
            Angle::Degrees(_d) => self,
            Angle::Radians(r) => Angle::Degrees(r * (180.0 / PI_f32)),
            Angle::MilliRadians(mr) => Angle::Degrees(mr * 180.0 / (PI_f32 * 1000.0)),
        };
    }

    pub fn as_radians(self) -> Self {
        return match self {
            Angle::Degrees(d) => Angle::Radians(d * (PI_f32 / 180.0)),
            Angle::Radians(_r) => self,
            Angle::MilliRadians(mr) => Angle::Radians(mr / 1000.0),
        };
    }

    pub fn as_milliradians(self) -> Self {
        return match self {
            Angle::Degrees(d) => Angle::MilliRadians(d * (1000.0 * PI_f32) / 180.0),
            Angle::Radians(r) => Angle::MilliRadians(r * 1000.0),
            Angle::MilliRadians(_mr) => self,
        };
    }

    pub fn value(self) -> f32 {
        return match self {
            Angle::Degrees(d) => d,
            Angle::Radians(r) => r,
            Angle::MilliRadians(mr) => mr,
        };
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return match self {
            Angle::Degrees(d) => Angle::Degrees(d + rhs.as_degress().value()),
            Angle::Radians(r) => Angle::Radians(r + rhs.as_radians().value()),
            Angle::MilliRadians(mr) => Angle::MilliRadians(mr + rhs.as_milliradians().value()),
        };
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = match self {
            Angle::Degrees(d) => Angle::Degrees(*d + rhs.as_degress().value()),
            Angle::Radians(r) => Angle::Radians(*r + rhs.as_radians().value()),
            Angle::MilliRadians(mr) => Angle::MilliRadians(*mr + rhs.as_milliradians().value()),
        };
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return match self {
            Angle::Degrees(d) => Angle::Degrees(d - rhs.as_degress().value()),
            Angle::Radians(r) => Angle::Radians(r - rhs.as_radians().value()),
            Angle::MilliRadians(mr) => Angle::MilliRadians(mr - rhs.as_milliradians().value()),
        };
    }
}

impl Mul for Angle {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return match self {
            Angle::Degrees(d) => Angle::Degrees(d * rhs.as_degress().value()),
            Angle::Radians(r) => Angle::Radians(r * rhs.as_radians().value()),
            Angle::MilliRadians(mr) => Angle::MilliRadians(mr * rhs.as_milliradians().value()),
        };
    }
}

impl Div for Angle {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        return match self {
            Angle::Degrees(d) => Angle::Degrees(d / rhs.as_degress().value()),
            Angle::Radians(r) => Angle::Radians(r / rhs.as_radians().value()),
            Angle::MilliRadians(mr) => Angle::MilliRadians(mr / rhs.as_milliradians().value()),
        };
    }
}
