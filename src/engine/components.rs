use crate::UPDATE_RATE;
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::Image;
use std::ops::Mul;
// TODO: Move to phx/mod
// Position of the entity
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub src: Vector,
}

impl Mul<crate::phx::Velocity> for Position {
    type Output = Self;

    fn mul(self, rhs: crate::phx::Velocity) -> Self::Output {
        let x = self.src.x + rhs.src.x * 1. / UPDATE_RATE;
        let y = self.src.y + rhs.src.y * 1. / UPDATE_RATE;
        Self {
            src: Vector::new(x, y),
        }
    }
}

// Sprites are referenced by their center
// TODO: Hold the actual subrectangle that is supposed to be drawn
// TODO: Move to gfx/mod
pub struct Sprite {
    pub src: String,
    pub offset: Vector,
}

impl Sprite {
    pub fn new(name: String, image: &Image) -> Self {
        Self {
            src: name,
            offset: -image.size() / 2.,
        }
    }
}
