use std::time::Duration;

use super::constants::{SOS_f32, SOS_f64};

pub struct Frequency {
    pub cycles: f32,
    pub t: Duration,
}

impl Frequency {
    pub fn new(cycles: f32, t: Duration) -> Self {
        return Frequency { cycles, t };
    }

    pub fn value(&self) -> f32 {
        self.cycles / self.t.as_secs_f32()
    }

    pub fn period(&self) -> f32 {
        1.0 / self.value()
    }
}

/// Speed of sound in m/s as ``f32``, ``temp`` is °C
pub fn speed_of_sound_f32(temp: f32) -> f32 {
    SOS_f32 + (0.6 * temp)
}

/// Speed of sound in m/s as ``f64``, ``temp`` is °C
pub fn speed_of_sound_f64(temp: f64) -> f64 {
    SOS_f64 + (0.6 * temp)
}
