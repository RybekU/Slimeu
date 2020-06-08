use super::super::collision::Shape;
// TODO: maybe mint vector?
use quicksilver::geom::Vector;

pub struct Body {
    pub shape: Shape,
    pub position: Vector,
    // static body CAN have velocity - it just behaves as if it had infinite mass
    // and doesn't collide with other static bodies
    pub velocity: Vector,
    // TODO: resolve this on storage level?
    pub dynamic: bool,
    //TODO: collision layer
    //TODO: resolution type
}

//TODO: body.is_colliding bool
//TODO: body.get_collision Some(collision_info_struct)
impl Body {
    pub fn new_static(shape: Shape, position: Vector) -> Self {
        Self {
            shape,
            position,
            velocity: Vector::ZERO,
            dynamic: false,
        }
    }
    pub fn new_dynamic(shape: Shape, position: Vector, velocity: Vector) -> Self {
        Self {
            shape,
            position,
            velocity,
            dynamic: true,
        }
    }
}

// Unique identifier of an object stored in the world
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BodyHandle(pub usize);
