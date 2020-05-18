mod collision_world;
mod detection;
mod hitbox;
mod resolution;

pub use self::collision_world::CollisionWorld;
pub use self::detection::ncollide_update;
pub use self::hitbox::{sync_ncollide, Hitbox};
pub use self::resolution::correct_position;

// collisions
pub type CollisionObject = ncollide2d::pipeline::CollisionObject<f32, CollisionData>;
pub type CollisionHandle = ncollide2d::pipeline::CollisionObjectSlabHandle;
pub type Iso2 = nalgebra::Isometry2<f32>;
pub type Vec2 = nalgebra::Vector2<f32>;

use fxhash::FxHashMap;
pub type PositionCorrection = FxHashMap<CollisionHandle, Vec2>;
// TODO: Wrap into a structure with interface that will make it possible to convert into hashmap or something if necessary
// pub type ActiveCollisions = Vec<(CollisionHandle, CollisionHandle)>;

#[derive(Debug, Clone, Copy)]
pub enum CollisionGroup {
    Terrain,
    Ally,
    Test1,
    Test2,
}
use ncollide2d::pipeline::CollisionGroups;
// Entity should be in only one of the groups for simplicity's sake
impl From<CollisionGroup> for CollisionGroups {
    fn from(cgroup: CollisionGroup) -> Self {
        use CollisionGroup::*;
        match cgroup {
            Terrain => CollisionGroups::new().with_membership(&[Terrain as usize]),
            Ally => CollisionGroups::new()
                .with_membership(&[Ally as usize])
                .with_whitelist(&[Terrain as usize, Test1 as usize, Test2 as usize]),
            // TODO: Move the whitelists into the enum itself as refs to static data
            e => panic! {"Unhandled CollisionGroup: {:#?}", e},
        }
    }
}

#[derive(Debug)]
pub struct CollisionData {
    pub group: CollisionGroup,
}

impl From<crate::engine::components::Position> for Iso2 {
    fn from(position: crate::engine::components::Position) -> Self {
        Iso2::new(nalgebra::Vector2::new(position.src.x, position.src.y), 0.0)
    }
}
