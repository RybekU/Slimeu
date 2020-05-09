mod hitbox;

pub use hitbox::Hitbox;

// collisions
pub type CollisionWorld = ncollide2d::world::CollisionWorld<f32, ObjectID>;
pub type Iso2 = nalgebra::Isometry2<f32>;

pub mod collide {
    pub const SOMETHING: usize = 1;
}
pub type ObjectID = Option<legion::entity::Entity>;
