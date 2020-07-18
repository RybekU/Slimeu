pub use super::super::collision::Shape;
pub use super::{Body, BodyHandle, BodyState, BodyType};
use quicksilver::geom::Vector;

#[derive(Debug, Clone)]
pub struct BodyBuilder {
    pub shape: Shape,
    pub position: Vector,

    pub velocity: Vector,
    pub btype: BodyType,
    pub state: BodyState,

    pub category_bits: u32,
    pub mask_bits: u32,
}

impl BodyBuilder {
    pub fn new(shape: Shape, position: Vector) -> Self {
        Self {
            shape,
            position,
            velocity: Vector::ZERO,
            btype: BodyType::Dynamic,
            state: BodyState::Solid,
            category_bits: 1,
            mask_bits: u32::MAX,
        }
    }
    pub fn with_position(mut self, position: Vector) -> Self {
        self.position = position;
        self
    }
    pub fn with_velocity(mut self, velocity: Vector) -> Self {
        self.velocity = velocity;
        self
    }
    pub fn make_static(mut self) -> Self {
        self.btype = BodyType::Static;
        self
    }
    pub fn sensor(mut self) -> Self {
        self.state = BodyState::Sensor;
        self
    }
    pub fn with_category(mut self, category_bits: u32) -> Self {
        self.category_bits = category_bits;
        self
    }
    pub fn with_mask(mut self, mask_bits: u32) -> Self {
        self.mask_bits = mask_bits;
        self
    }
    pub fn build(self) -> Body {
        Body::new(
            self.shape,
            self.position,
            self.velocity,
            self.btype,
            self.state,
            self.category_bits,
            self.mask_bits,
        )
    }
}
