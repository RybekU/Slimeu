use super::collision::{CollisionGraph, ContactManifold};
use super::event::PhysicsEvent;
use super::object::{collided, collision_info, Body, BodyHandle, BodyState, BodyType};
use slab::Slab;

type ContactInfo = (usize, usize, ContactManifold);

pub struct PhysicsWorld {
    pub bodies: Slab<Body>,
    pub collision_graph: CollisionGraph,
    pub manifolds: Vec<ContactInfo>,

    pub events: Vec<PhysicsEvent>,
}

impl PhysicsWorld {
    //TODO: with_capacity to set slab initial size
    pub fn new() -> Self {
        Self {
            bodies: Slab::with_capacity(128),
            collision_graph: CollisionGraph::with_capacity(128, 16),
            manifolds: Vec::with_capacity(128),
            events: Vec::with_capacity(16),
        }
    }
    pub fn add(&mut self, body: Body) -> BodyHandle {
        let key = self.bodies.insert(body);
        self.collision_graph.add_node(key);
        BodyHandle(key)
    }
    pub fn get_body(&self, handle: BodyHandle) -> Option<&Body> {
        self.bodies.get(handle.0)
    }
    pub fn mut_body(&mut self, handle: BodyHandle) -> Option<&mut Body> {
        self.bodies.get_mut(handle.0)
    }
    pub fn events(&self) -> &Vec<PhysicsEvent> {
        &self.events
    }

    pub fn step(&mut self, dt: f32) {
        self.manifolds.clear();
        self.events.clear();
        let bodies = &mut self.bodies;
        let manifolds = &mut self.manifolds;
        let collision_graph = &mut self.collision_graph;
        let events = &mut self.events;

        // apply velocity for every body
        for (_, body) in bodies.iter_mut() {
            if let BodyType::Dynamic = body.btype {
                body.position += body.velocity * dt;
            }
        }

        // TODO: Real broad phase and track starting/stopping of collisions
        // Makeshift broad-phase
        for (h1, body1) in bodies.iter() {
            if let BodyType::Static = body1.btype {
                continue;
            }

            for (h2, body2) in bodies.iter() {
                if h1 == h2 {
                    continue;
                }
                // only bodies with matching masks can collide
                let category_mismatch = ((body1.category_bits & body2.mask_bits) == 0)
                    || ((body2.category_bits & body1.mask_bits) == 0);
                if category_mismatch {
                    continue;
                }

                if collided(body1, body2) {
                    collision_graph.update_edge(h1, h2);
                }
            }
        }

        let mut removed_edges = vec![];
        // fake narrow-phase replacement
        for edge_id in collision_graph.src.edge_indices() {
            let (node_id1, node_id2) = collision_graph.src.edge_endpoints(edge_id).unwrap();
            let handle1 = collision_graph.src[node_id1];
            let handle2 = collision_graph.src[node_id2];
            let body1 = &bodies[handle1];
            let body2 = &bodies[handle2];
            // todo: move "collided" to broad phase, try to do "collision started/ended" thing
            let edge_status = collision_graph.src.edge_weight_mut(edge_id).unwrap();
            let remove_edge = detect_collision(
                handle1,
                &body1,
                handle2,
                &body2,
                edge_status,
                manifolds,
                events,
            );
            if remove_edge {
                removed_edges.push(edge_id);
            }
        }
        removed_edges.into_iter().for_each(|edge| {
            collision_graph.src.remove_edge(edge);
        });

        // resolve collisions TODO: resolve multiple collisions for one body
        for (h1, _h2, manifold) in manifolds.iter() {
            let body = bodies.get_mut(*h1).expect("Body missing post collision");
            let contact = manifold.best_contact();
            body.position -= contact.normal * contact.depth;

            body.velocity.x *= contact.normal.y.abs();
            body.velocity.y *= contact.normal.x.abs();
        }
    }
}

// Makeshift function for collision detection
fn detect_collision(
    h1: usize,
    body1: &Body,
    h2: usize,
    body2: &Body,
    new_edge: &mut bool,
    manifolds: &mut Vec<ContactInfo>,
    events: &mut Vec<PhysicsEvent>,
) -> bool {
    use BodyState::*;

    let remove_edge = match (&body1.state, &body2.state) {
        (Solid, Solid) => {
            if let Some(manifold) = collision_info(body1, body2) {
                if *new_edge {
                    debug!("Solid bodies started colliding: {} and {}", h1, h2);
                    events.push(PhysicsEvent::CollisionStarted(
                        BodyHandle(h1),
                        BodyHandle(h2),
                    ));
                }
                manifolds.push((h1, h2, manifold));
                false
            } else {
                if !*new_edge {
                    debug!("Solid bodies stopped colliding: {} and {}", h1, h2);
                    events.push(PhysicsEvent::CollisionEnded(BodyHandle(h1), BodyHandle(h2)));
                }
                true
            }
        }
        (Solid, Zone) => {
            if collided(body1, body2) {
                if *new_edge {
                    debug!("Solid {} entered zone {}", h1, h2);
                    events.push(PhysicsEvent::OverlapStarted(BodyHandle(h1), BodyHandle(h2)));
                }
                false
            } else {
                if !*new_edge {
                    debug!("Solid {} left zone {}", h1, h2);
                    events.push(PhysicsEvent::OverlapEnded(BodyHandle(h1), BodyHandle(h2)));
                }
                true
            }
        }
        (Zone, Solid) => {
            if collided(body1, body2) {
                if *new_edge {
                    debug!("Solid {} entered zone {}", h2, h1);
                    events.push(PhysicsEvent::OverlapStarted(BodyHandle(h2), BodyHandle(h1)));
                }
                false
            } else {
                if !*new_edge {
                    debug!("Solid {} left zone {}", h2, h1);
                    events.push(PhysicsEvent::OverlapEnded(BodyHandle(h2), BodyHandle(h1)));
                }
                true
            }
        }
        (Zone, Zone) => {
            if collided(body1, body2) {
                if *new_edge {
                    debug!("Zone {} overlapped with zone {}", h1, h2);
                    events.push(PhysicsEvent::OverlapStarted(BodyHandle(h1), BodyHandle(h2)));
                }
                false
            } else {
                if !*new_edge {
                    debug!("Zone {} separated with zone {}", h1, h2);
                    events.push(PhysicsEvent::OverlapEnded(BodyHandle(h1), BodyHandle(h2)));
                }
                true
            }
        }
    };
    *new_edge = false;
    remove_edge
}
