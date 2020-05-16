use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::Image;

// TODO: Move to phx/mod
// Position of the entity
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub src: Vector,
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
