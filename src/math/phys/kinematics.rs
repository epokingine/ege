use std::time::Duration;

use crate::math::angles::Angle;

#[derive(Debug, Clone, Copy)]
pub struct Acceleration {
    pub v1: f32,
    pub v2: f32,
    pub t: Duration,
    pub direction: Angle,
}

impl Acceleration {
    pub fn new(v1: f32, v2: f32, t: Duration, direction: Option<Angle>) -> Self {
        return Acceleration {
            v1,
            v2,
            t,
            direction: direction.unwrap_or(Angle::Degrees(0.0)),
        };
    }

    pub fn value(self) -> f32 {
        (self.v2 - self.v1) / self.t.as_secs_f32()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub ms: f32,
    pub direction: Angle,
}

impl Velocity {
    pub fn accelerate(&mut self, a: Acceleration) {
        self.ms += a.value();
        self.direction += a.direction;
    }
}
