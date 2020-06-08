use super::collision::{self, ContactManifold, Shape};
use super::object::{Body, BodyHandle};
use slab::Slab;

pub struct PhysicsWorld {
    // TODO: static bodies? dynamic bodies? ccd? separately?
    pub bodies: Slab<Body>,

    pub manifolds: Vec<(usize, usize, ContactManifold)>,
}

impl PhysicsWorld {
    //TODO: with_capacity to set slab initial size
    pub fn new() -> Self {
        Self {
            bodies: Slab::with_capacity(128),
            manifolds: Vec::with_capacity(128),
        }
    }
    pub fn add(&mut self, body: Body) -> BodyHandle {
        let key = self.bodies.insert(body);
        BodyHandle(key)
    }
    pub fn get_body(&self, handle: BodyHandle) -> Option<&Body> {
        self.bodies.get(handle.0)
    }
    pub fn mut_body(&mut self, handle: BodyHandle) -> Option<&mut Body> {
        self.bodies.get_mut(handle.0)
    }

    pub fn step(&mut self, dt: f32) {
        self.manifolds.clear();
        let bodies = &mut self.bodies;
        let manifolds = &mut self.manifolds;

        // apply velocity for every body
        for (_, body) in bodies.iter_mut() {
            if body.dynamic {
                body.position += body.velocity * dt;
            }
        }

        // detect collisions
        for (h1, body1) in bodies.iter() {
            if !body1.dynamic {
                continue;
            }
            for (h2, body2) in bodies.iter() {
                if h1 == h2 {
                    continue;
                }
                use Shape::*;
                match (body1.shape, body2.shape) {
                    (AABB(half_extents1), AABB(half_extents2)) => {
                        if collision::collision_aabb_aabb(
                            body1.position,
                            half_extents1,
                            body2.position,
                            half_extents2,
                        ) {
                            // debug!("Collision between {} and {}", h1, h2);
                            let manifold = collision::collision_aabb_aabb_manifold(
                                body1.position,
                                half_extents1,
                                body2.position,
                                half_extents2,
                            );
                            // debug!("Collision information: {:#?}", manifold);
                            manifold
                                .into_iter()
                                .for_each(|m| manifolds.push((h1, h2, m)));
                        }
                    }
                }
            }
        }
        // resolve collisions
        for (h1, _h2, manifold) in manifolds.iter() {
            let body = bodies.get_mut(*h1).expect("Body missing post collision");
            let contact = manifold.best_contact();
            body.position -= contact.normal * contact.depth;
            println!("{:#?}", contact.normal);
            body.velocity.x *= contact.normal.y.abs();
            body.velocity.y *= contact.normal.x.abs();
        }
    }
}
