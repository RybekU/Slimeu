use slab::{Iter, IterMut, Slab};
use super::object::{BodyHandle, Body};
use super::collision::{self, Shape};

pub struct PhysicsWorld {
    // TODO: static bodies? dynamic bodies? ccd? separately?
    pub bodies: Slab<Body>,
}

impl PhysicsWorld {
    //TODO: with_capacity to set slab initial size
    pub fn new() -> Self {
        Self { bodies: Slab::with_capacity(128)}
    }
    pub fn add(&mut self, body: Body) -> BodyHandle {
        let key = self.bodies.insert(body);
        BodyHandle(key)
    }
    pub fn step(&mut self, dt: f32) {
        // apply velocity for every body
        for (_, body) in self.bodies.iter_mut() {
            if body.dynamic {
                body.position += body.velocity*dt;
            }
        }

        // detect collisions
        for (h1, body1) in self.bodies.iter() {
            if !body1.dynamic {
                continue
            }
            for (h2, body2) in self.bodies.iter() {
                if h1 == h2 {
                    continue
                }
                use Shape::*;
                match (body1.shape, body2.shape) {
                    (AABB(half_extents1), AABB(half_extents2)) => {
                        debug!("Is collision between {} and {}?: {}", h1, h2, collision::collision_aabb_aabb(body1.position, half_extents1, body2.position, half_extents2));
                        debug!("Collision information: {:#?}", collision::collision_aabb_aabb_manifold(body1.position, half_extents1, body2.position, half_extents2));
                    }
                }
            }
        }
        // resolve collisions

    }
    
}