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
}

impl BodyBuilder {
    pub fn new(shape: Shape, position: Vector) -> Self {
        Self {
            shape,
            position,
            velocity: Vector::ZERO,
            btype: BodyType::Dynamic,
            state: BodyState::Solid,
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
    pub fn non_solid(mut self) -> Self {
        self.state = BodyState::Zone;
        self
    }
    pub fn build(self) -> Body {
        Body::new(
            self.shape,
            self.position,
            self.velocity,
            self.btype,
            self.state,
        )
    }
}
