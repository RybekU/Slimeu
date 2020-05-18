use quicksilver::geom::Vector;

use crate::engine::components::Position;
use crate::phx::{CollisionGroup, CollisionHandle, CollisionObject, CollisionWorld, Iso2};
use legion::prelude::*;

// Hitbox - offset should be set relative to the center
pub struct Hitbox {
    pub src: CollisionHandle,
}

impl Hitbox {
    // Position is the center of the hitbox
    pub fn new(
        cworld: &mut CollisionWorld,
        position: Vector,
        size: Vector,
        collision_group: CollisionGroup,
    ) -> (Self, &mut CollisionObject) {
        let (src, obj_ref) = cworld.add(position, size, collision_group);
        (Self { src }, obj_ref)
    }
}

pub fn sync_ncollide() -> Box<dyn Schedulable> {
    SystemBuilder::new("sync_ncollide")
        .write_resource::<CollisionWorld>()
        .with_query(<(Read<Position>, Write<Hitbox>)>::query().filter(changed::<Position>()))
        .build(move |_, mut world, cworld, query| {
            for (pos, hitbox) in query.iter_mut(&mut world) {
                let nobject = cworld
                    .get_mut(hitbox.src)
                    .expect("sync_ncollide: nonexistent object");
                nobject.set_position(Iso2::from(*pos));
            }
        })
}
