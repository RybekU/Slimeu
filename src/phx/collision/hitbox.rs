use quicksilver::geom::Vector;

use crate::engine::components::Position;
use legion::prelude::*;

use crate::engine::physics::{Body, BodyHandle, PhysicsWorld, Shape};
use crate::phx::Velocity;
use crate::UPDATE_RATE;

// Hitbox - offset should be set relative to the center
pub struct Hitbox {
    pub src: BodyHandle,
}

impl Hitbox {
    // Position is the center of the hitbox
    pub fn new(pworld: &mut PhysicsWorld, position: Vector, size: Vector, dynamic: bool) -> Self {
        let body = match dynamic {
            true => Body::new_dynamic(Shape::AABB(size / 2), position, Vector::ZERO),
            false => Body::new_static(Shape::AABB(size / 2), position),
        };
        let src = pworld.add(body);
        Self { src }
    }
}

pub fn physics_pre_sync() -> Box<dyn Schedulable> {
    SystemBuilder::new("physics_pre_sync")
        .write_resource::<PhysicsWorld>()
        .with_query(
            <(Read<Position>, Read<Velocity>, Read<Hitbox>)>::query().filter(changed::<Position>()),
        )
        .build(move |_, world, pworld, query| {
            for (pos, vel, hitbox) in query.iter(&world) {
                let body = pworld.mut_body(hitbox.src).expect("Handle to invalid body");
                body.position = pos.src;
                body.velocity = vel.src;
            }
            pworld.step(1. / UPDATE_RATE);
        })
}

pub fn physics_post_sync() -> Box<dyn Schedulable> {
    SystemBuilder::new("sync_physics")
        .write_resource::<PhysicsWorld>()
        .with_query(
            <(Write<Position>, Write<Velocity>, Read<Hitbox>)>::query()
                .filter(changed::<Position>()),
        )
        .build(move |_, mut world, pworld, query| {
            for (mut pos, mut vel, hitbox) in query.iter_mut(&mut world) {
                let body = pworld
                    .get_body(hitbox.src)
                    .expect("hitbox.rs: Handle to invalid body");
                pos.src = body.position;
                vel.src = body.velocity;
            }
        })
}
