use ncollide2d::pipeline::{narrow_phase::ContactEvent};
use quicksilver::geom::Vector;

use super::{CollisionGroup, CollisionObject, CollisionWorld, CollisionHandle, Iso2, Vec2, CollisionData};
use crate::engine::components::Position;
use legion::prelude::*;

// TODO: New better name, it's the hashmap that keeps collisions for every entity
use super::{ActiveCollisions, PositionCorrection};

// Hitbox - offset should be set relative to the center
pub struct Hitbox {
    pub src: CollisionHandle,
}

// TODO: Study hitbox and hurtbox - decide if you need to differentiate them

impl Hitbox {
    // Position is the center of the hitbox
    pub fn new(
        cworld: &mut CollisionWorld,
        position: Vector,
        size: Vector,
        collision_group: CollisionGroup,
    ) -> (Self, &mut CollisionObject) {
        // If this will also be required somewhere else introduce cast
        let nalgebra_pos = nalgebra::Vector2::new(position.x, position.y);
        let nalgebra_size = nalgebra::Vector2::new(size.x, size.y);
        let ncollide_cuboid = ncollide2d::shape::Cuboid::new(nalgebra_size / 2.0);
        let iso = Iso2::new(nalgebra_pos, 0.0);
        let (src, obj_ref) = cworld.add(
            iso,
            ncollide2d::shape::ShapeHandle::new(ncollide_cuboid),
            collision_group.into(),
            ncollide2d::pipeline::GeometricQueryType::Contacts(0.0, 0.0),
            CollisionData{group: collision_group},
        );
        (Self { src }, obj_ref)
    }
}

pub fn sync_ncollide() -> Box<dyn Schedulable> {
    SystemBuilder::new("sync_ncollide")
        .write_resource::<CollisionWorld>()
        .with_query(<(Read<Position>, Write<Hitbox>)>::query().filter(changed::<Position>()))
        .build(move |_, mut world, cworld, query| {
            for (pos, hitbox) in query.iter_mut(&mut world) {
                let nobject = cworld.get_mut(hitbox.src).expect("sync_ncollide: nonexistent object");
                nobject.set_position(Iso2::from(*pos));
            }
        })
}

// Collisions are stored in Resources, components have to check if they got any collisions
pub fn ncollide_update() -> Box<dyn Schedulable> {
    SystemBuilder::new("ncollide_update")
        .write_resource::<CollisionWorld>()
        .write_resource::<ActiveCollisions>()
        .write_resource::<PositionCorrection>()
        .build(move |_, _, (cworld, active_collisions, pos_correct), _| {
            cworld.update();
            cworld.contact_events().into_iter().for_each(|event| {
                match event {
                    ContactEvent::Started{0:handle1, 1:handle2} => {
                        println!("Collision started {:?}, {:?}", *handle1, *handle2);

                        active_collisions.push((*handle1, *handle2));
                    },
                    ContactEvent::Stopped{0:handle1, 1:handle2} => {
                        println!("Collision stopped {:?}, {:?}", *handle1, *handle2);
                        let index = active_collisions.iter().position(|contact| *contact == (*handle1, *handle2)).expect("Bug in ncollide2d - stopped collision that didn't start");
                        active_collisions.swap_remove(index);
                    }
                }
            });
            // FUTURE: Could be handled more cleanly by wrapping whole CollisionWorld
            let mut to_remove: Vec<(CollisionHandle, CollisionHandle)> = vec![];
            for (handle1, handle2) in active_collisions.iter() {
                let obj1 = cworld.collision_object(*handle1);
                let obj2 = cworld.collision_object(*handle2);
                if obj1.is_none() {
                    to_remove.push((handle1.clone(), handle2.clone()));
                    continue;
                }
                if obj2.is_none() {
                    to_remove.push((handle1.clone(), handle2.clone()));
                    continue;
                }

                let obj1 = obj1.expect("it really should exist now");
                let obj2 = obj2.expect("it really should exist now");

                classify_collision(handle1, handle2, obj1, obj2, cworld, pos_correct);
                classify_collision(handle2, handle1, obj2, obj1, cworld, pos_correct);
            };

            to_remove.into_iter().for_each(|handles| {
                let index = active_collisions.iter().position(|contact| *contact == handles).expect("Bug in ncollide2d - stopped collision that didn't start");
                active_collisions.swap_remove(index);
            })

        })
}

fn classify_collision(handle1: &CollisionHandle, handle2: &CollisionHandle, object1: &CollisionObject, object2: &CollisionObject, cworld: &CollisionWorld, pos_correct: &mut PositionCorrection) {
    let obj1_data = object1.data();
    let obj2_data = object2.data();
    match (obj1_data.group, obj2_data.group) {
        (any, CollisionGroup::Terrain) => {
            if let Some((_, _, _, contact)) = cworld.contact_pair(*handle1, *handle2, true) {
                let deepest = contact.deepest_contact().unwrap().contact;
                let pos_correction = -deepest.normal.into_inner() * deepest.depth;
                let current_correction = pos_correct.entry(*handle1).or_insert(Vec2::zeros());
                // TODO: Take into account multiple collisions differently - take the maximum value and separately for every axis
                *current_correction = pos_correction;
                // println!("{:#?}", current_correction);
            }
        }
        (_first, _second) => {
            // debug!("Collision type that isn't considered: {:#?} with {:#?}", first, second);
        }
    }
}