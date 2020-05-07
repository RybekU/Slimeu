use quicksilver::geom::{Rectangle, Shape, Vector};
use quicksilver::graphics::Image;

// Position of the entity
pub struct Position {
    pub src: Vector,
}

// Sprites are referenced by their center
// TODO: Hold the actual subrectangle that is supposed to be drawn
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

// TODO: Debug information goal - draw hitboxes with special feature flag
// Hitbox - offset should be set relative to the center
pub struct Hitbox {
    pub src: Rectangle,
    pub offset: Vector,
}

// TODO: Study hitbox and hurtbox
