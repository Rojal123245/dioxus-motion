pub use instant::Duration;

use crate::spring::Spring;

use easer::functions::{Easing, Linear};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tween {
    pub duration: Duration,
    pub easing: fn(f32, f32, f32, f32) -> f32,
}

impl Default for Tween {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            easing: Linear::ease_in_out,
        }
    }
}

impl Tween {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            easing: Linear::ease_in_out,
        }
    }

    pub fn with_easing(mut self, easing: fn(f32, f32, f32, f32) -> f32) -> Self {
        self.easing = easing;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationMode {
    Tween(Tween),
    Spring(Spring),
}

impl Default for AnimationMode {
    fn default() -> Self {
        Self::Tween(Tween::default())
    }
}

/// Represents the current state of an animation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Running,
    Completed,
}
