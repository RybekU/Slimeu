use crate::engine::components::Position;
use legion::prelude::*;

use crate::phx::Velocity;
use crate::phx::{Body, PhysicsWorld};
use crate::UPDATE_RATE;
use resphys::BodyHandle;

/// Hitbox - offset should be set relative to the center
pub struct Hitbox {
    pub src: BodyHandle,
}

impl Hitbox {
    /// Position is the center of the hitbox
    pub fn new(pworld: &mut PhysicsWorld, body: Body) -> Self {
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
                //TODO: After updating `quicksilver` change to From...
                let pos_temp: mint::Vector2<f32> = pos.src.into();
                let vel_temp: mint::Vector2<f32> = vel.src.into();
                body.position = pos_temp.into();
                body.velocity = vel_temp.into();
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
            for event in pworld.events() {
                debug!("{:?}", event);
            }
            for (mut pos, mut vel, hitbox) in query.iter_mut(&mut world) {
                let body = pworld
                    .get_body(hitbox.src)
                    .expect("hitbox.rs: Handle to invalid body");
                pos.src = mint::Vector2::from(body.position).into();
                vel.src = mint::Vector2::from(body.velocity).into();
            }
        })
}
