mod hitbox;

pub use self::hitbox::{physics_post_sync, physics_pre_sync, Hitbox};

use bitflags::bitflags;

use crate::engine::physics::{Body as B, PhysicsWorld as Pworld};

pub type PhysicsWorld = Pworld<BodyTag>;
pub type Body = B<BodyTag>;

#[derive(Clone, Copy, Debug)]
pub enum BodyTag {
    PC,
    DummyArea,
    RectangleUnder,
    RectangleRight,
}

bitflags! {
    pub struct Category: u32 {
        const GROUND = 0b1 << 1;
        const ALLY = 0b1 << 2;
        //const ENEMY = 0b1 << 3;
    }
}
