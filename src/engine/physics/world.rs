use super::collision::ContactManifold;
use super::object::{collided, collision_info, Body, BodyHandle, BodyState, BodyType};
use slab::Slab;

type ContactInfo = (usize, usize, ContactManifold);

pub struct PhysicsWorld {
    // TODO: static bodies? dynamic bodies? ccd? separately?
    pub bodies: Slab<Body>,

    pub manifolds: Vec<ContactInfo>,
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
            if let BodyType::Dynamic = body.btype {
                body.position += body.velocity * dt;
            }
        }

        // detect collisions
        for (h1, body1) in bodies.iter() {
            if let BodyType::Static = body1.btype {
                continue;
            }
            for (h2, body2) in bodies.iter() {
                if h1 == h2 {
                    continue;
                }
                detect_collision(h1, &body1, h2, &body2, manifolds);
            }
        }
        // resolve collisions
        for (h1, _h2, manifold) in manifolds.iter() {
            let body = bodies.get_mut(*h1).expect("Body missing post collision");
            let contact = manifold.best_contact();
            body.position -= contact.normal * contact.depth;

            body.velocity.x *= contact.normal.y.abs();
            body.velocity.y *= contact.normal.x.abs();
        }
    }
}

fn detect_collision(
    h1: usize,
    body1: &Body,
    h2: usize,
    body2: &Body,
    manifolds: &mut Vec<ContactInfo>,
) {
    use BodyState::*;
    match (&body1.state, &body2.state) {
        (Solid, Solid) => {
            let manifold = collision_info(body1, body2);
            // debug!("Collision information: {:#?}", manifold);
            manifold
                .into_iter()
                .for_each(|m| manifolds.push((h1, h2, m)));
        }
        (Solid, Zone) => {
            if collided(body1, body2) {
                // debug!("Solid {} in zone {}", h1, h2);
            }
        }
        (Zone, Solid) => {
            if collided(body1, body2) {
                // debug!("Zone {} intercepted solid {}", h1, h2);
            }
        }
        (Zone, Zone) => {
            if collided(body1, body2) {
                // debug!("Zone {} collided with zone {}", h1, h2);
            }
        }
    }
}
