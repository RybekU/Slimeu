use crate::engine::components::Position;
use crate::phx::collision::{Hitbox, Vec2};
use legion::prelude::*;

use crate::phx::PositionCorrection;

pub fn correct_position() -> Box<dyn Schedulable> {
    SystemBuilder::new("sync_ncollide")
        .write_resource::<PositionCorrection>()
        .with_query(<(Write<Position>, Read<Hitbox>)>::query().filter(changed::<Position>()))
        .build(move |_, mut world, pos_correction, query| {
            for (mut pos, hitbox) in query.iter_mut(&mut world) {
                let mut remove = false;
                if let Some(correction) = pos_correction.get_mut(&hitbox.src) {
                    if *correction == Vec2::zeros() {
                        remove = true
                    }
                    pos.src.x += correction.x;
                    pos.src.y += correction.y;

                    *correction = Vec2::zeros();
                }
                if remove {
                    pos_correction.remove(&hitbox.src);
                }
            }
        })
}
