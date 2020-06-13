use super::super::collision::{self, ContactManifold, Shape};
// TODO: maybe mint vector?
use quicksilver::geom::Vector;

/// Describes a body in shape of `Shape`.
///
/// Currently there's no "fixture" like in Box2D and body has only 1 shape attached.
#[derive(Clone, Debug)]
pub struct Body {
    pub shape: Shape,
    pub position: Vector,
    /// static body CAN have velocity - it just behaves as if it had infinite mass
    /// and doesn't collide with other static bodies
    pub velocity: Vector,
    /// Type of body - `static` or `dynamic`
    pub btype: BodyType,
    /// Whether to treat the body as physical or not
    pub state: BodyState,
    /// Ideally only one bit should be set
    pub category_bits: u32,
    /// Bodies only collide if both of their masks match
    pub mask_bits: u32,
}

impl Body {
    pub fn new(
        shape: Shape,
        position: Vector,
        velocity: Vector,
        btype: BodyType,
        state: BodyState,
        category_bits: u32,
        mask_bits: u32,
    ) -> Self {
        Self {
            shape,
            position,
            velocity,
            btype,
            state,
            category_bits,
            mask_bits,
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

/// Type of the body, determines collision resolution and how it's affected by other bodies.
#[derive(Copy, Clone, Debug)]
pub enum BodyType {
    /// Even when it moves it never collides with anything.
    Static,
    /// Collides with both static and dynamic bodies.
    Dynamic,
}

/// State of the body, determines collision resolution and types of events sent.
#[derive(Copy, Clone, Debug)]
pub enum BodyState {
    /// Solid body resolves collision.
    Solid,
    /// Zone sends events about enter/exit/inside.
    Zone,
}
