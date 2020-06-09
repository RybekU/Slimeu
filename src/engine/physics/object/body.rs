use super::super::collision::{self, ContactManifold, Shape};
// TODO: maybe mint vector?
use quicksilver::geom::Vector;

/// Describes a body in shape of `Shape`
//TODO: BodyBuilder for creation of bodies
pub struct Body {
    pub shape: Shape,
    pub position: Vector,
    /// static body CAN have velocity - it just behaves as if it had infinite mass
    /// and doesn't collide with other static bodies
    pub velocity: Vector,
    //TODO: change into enum
    pub dynamic: bool,
    //TODO: collision layer
    /// Whether to treat the body as physical or not
    pub state: BodyState,
}

impl Body {
    pub fn new_static(shape: Shape, position: Vector) -> Self {
        Self {
            shape,
            position,
            velocity: Vector::ZERO,
            dynamic: false,
            state: BodyState::Solid,
        }
    }
    pub fn new_dynamic(shape: Shape, position: Vector, velocity: Vector) -> Self {
        Self {
            shape,
            position,
            velocity,
            dynamic: true,
            state: BodyState::Solid,
        }
    }
    pub fn new_static_zone(shape: Shape, position: Vector) -> Self {
        Self {
            shape,
            position,
            velocity: Vector::ZERO,
            dynamic: false,
            state: BodyState::Zone,
        }
    }
}

/// Boolean test whether two bodies collided.
pub fn collided(body1: &Body, body2: &Body) -> bool {
    use Shape::*;
    match (body1.shape, body2.shape) {
        (AABB(half_extents1), AABB(half_extents2)) => collision::collision_aabb_aabb(
            body1.position,
            half_extents1,
            body2.position,
            half_extents2,
        ),
    }
}

/// Generates a ContactManifold if two bodies collided.
pub fn collision_info(body1: &Body, body2: &Body) -> Option<ContactManifold> {
    // if a or b zone - return none, perhaps return an event?
    use Shape::*;
    match (body1.shape, body2.shape) {
        (AABB(half_extents1), AABB(half_extents2)) => collision::collision_aabb_aabb_manifold(
            body1.position,
            half_extents1,
            body2.position,
            half_extents2,
        ),
    }
}

/// Unique identifier of an object stored in the world
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BodyHandle(pub usize);

/// State of the body, determines collision resolution and types of events sent.
#[derive(Copy, Clone, Debug)]
pub enum BodyState {
    /// Solid body resolves collision.
    Solid,
    /// Zone sends events about enter/exit/inside.
    Zone,
}
