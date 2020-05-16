use crate::engine::components::Position;
use crate::UPDATE_RATE;
use legion::prelude::*;
use quicksilver::geom::Vector;
// Unit not decided yet
#[derive(Debug, Clone, Default)]
pub struct Velocity {
    pub src: Vector,
}

pub fn movement() -> Box<dyn Schedulable> {
    SystemBuilder::new("apply_velocity")
        .with_query(<(Write<Position>, Read<Velocity>)>::query())
        .build(move |_, mut world, _, query| {
            for (mut pos, vel) in query.iter_mut(&mut world) {
                pos.src.x += vel.src.x * 1. / UPDATE_RATE;
                pos.src.y += vel.src.y * 1. / UPDATE_RATE;
            }
        })
}
