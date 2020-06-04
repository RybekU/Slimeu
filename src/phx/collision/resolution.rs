use crate::engine::components::Position;
use crate::phx::collision::{Hitbox, Vec2};
use crate::phx::Velocity;
use legion::prelude::*;

use crate::phx::CollisionWorld;
use crate::phx::PositionCorrection;

pub fn correct_position() -> Box<dyn Schedulable> {
    SystemBuilder::new("sync_ncollide")
        .write_resource::<PositionCorrection>()
        .read_resource::<CollisionWorld>()
        .with_query(
            <(Write<Position>, Write<Velocity>, Read<Hitbox>)>::query()
                .filter(changed::<Position>()),
        )
        .build(move |_, mut world, (pos_correction, cworld), query| {
            for (mut pos, mut vel, hitbox) in query.iter_mut(&mut world) {
                let obj = cworld
                    .collision_object(hitbox.src)
                    .expect("I swear it better be here");
                pos.src.x = obj.position().translation.x;
                pos.src.y = obj.position().translation.y;

                if let Some(correction) = pos_correction.get_mut(&hitbox.src) {
                    *correction = Vec2::zeros();
                }
            }
        })
}
