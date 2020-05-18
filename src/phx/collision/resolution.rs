use crate::engine::components::Position;
use crate::phx::collision::{Hitbox, Vec2};
use legion::prelude::*;

use crate::phx::PositionCorrection;
// TODO: Remove usage of these 2 structs below when collision teleport bug gets fixed
use crate::phx::collision::CollisionWorld;
use crate::phx::Iso2;

pub fn correct_position() -> Box<dyn Schedulable> {
    SystemBuilder::new("sync_ncollide")
        .write_resource::<PositionCorrection>()
        .write_resource::<CollisionWorld>()
        .with_query(<(Write<Position>, Read<Hitbox>)>::query().filter(changed::<Position>()))
        .build(move |_, mut world, (pos_correction, cworld), query| {
            for (mut pos, hitbox) in query.iter_mut(&mut world) {
                let mut remove = false;
                if let Some(correction) = pos_correction.get_mut(&hitbox.src) {
                    if *correction == Vec2::zeros() {
                        remove = true
                    }
                    pos.src.x += correction.x;
                    pos.src.y += correction.y;

                    let nobject = cworld
                        .get_mut(hitbox.src)
                        .expect("sync_ncollide: nonexistent object");
                    nobject.set_position(Iso2::from(*pos));

                    *correction = Vec2::zeros();
                }
                if remove {
                    pos_correction.remove(&hitbox.src);
                }
            }
        })
}
