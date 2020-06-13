mod hitbox;

pub use self::hitbox::{physics_post_sync, physics_pre_sync, Hitbox};

pub enum Category {
    Ground = 0b1 << 1,
    Ally = 0b1 << 2,
    // Enemy = 0b1 << 3,
}
