use super::{CollisionData, CollisionGroup, CollisionHandle, CollisionObject};
use crate::phx::Iso2;
use ncollide2d::pipeline::narrow_phase::{ContactAlgorithm, ContactEvent};
use ncollide2d::query::ContactManifold;
type World = ncollide2d::world::CollisionWorld<f32, CollisionData>;
type ActiveCollisions = Vec<(CollisionHandle, CollisionHandle)>;

use quicksilver::geom::Vector;

pub struct CollisionWorld {
    src: World,
    pub active_collisions: ActiveCollisions,
}

// For documentation check nphysic's CollisionWorld
impl CollisionWorld {
    pub fn new(margin: f32) -> Self {
        let src = World::new(margin);
        let active_collisions = ActiveCollisions::with_capacity(25);
        Self {
            src,
            active_collisions,
        }
    }
    // adds only rectangular objects
    pub fn add(
        &mut self,
        position: Vector,
        size: Vector,
        collision_group: CollisionGroup,
    ) -> (CollisionHandle, &mut CollisionObject) {
        // If this will also be required somewhere else introduce cast
        let nalgebra_pos = nalgebra::Vector2::new(position.x, position.y);
        let nalgebra_size = nalgebra::Vector2::new(size.x, size.y);
        let ncollide_cuboid = ncollide2d::shape::Cuboid::new(nalgebra_size / 2.0);
        let iso = Iso2::new(nalgebra_pos, 0.0);
        self.src.add(
            iso,
            ncollide2d::shape::ShapeHandle::new(ncollide_cuboid),
            collision_group.into(),
            ncollide2d::pipeline::GeometricQueryType::Contacts(0.0, 0.0),
            CollisionData {
                group: collision_group,
            },
        )
    }

    pub fn collision_object(&self, handle: CollisionHandle) -> Option<&CollisionObject> {
        self.src.collision_object(handle)
    }
    pub fn get_mut(&mut self, handle: CollisionHandle) -> Option<&mut CollisionObject> {
        self.src.get_mut(handle)
    }
    pub fn contact_pair(
        &self,
        handle1: CollisionHandle,
        handle2: CollisionHandle,
        effective_only: bool,
    ) -> Option<(
        CollisionHandle,
        CollisionHandle,
        &ContactAlgorithm<f32>,
        &ContactManifold<f32>,
    )> {
        self.src.contact_pair(handle1, handle2, effective_only)
    }
    pub fn remove(&mut self, handles: &[CollisionHandle]) {
        self.src.remove(handles);
        self.active_collisions
            .retain(|(handle1, handle2)| !(handles.contains(handle1) || handles.contains(handle2)));
    }
    // also calls ncollide's contact_events to update the inner state
    pub fn update(&mut self) {
        let active_collisions = &mut self.active_collisions;
        let world = &mut self.src;
        world.update();
        world.contact_events().into_iter().for_each(|event| {
            match event {
                ContactEvent::Started {
                    0: handle1,
                    1: handle2,
                } => {
                    // println!("Collision started {:?}, {:?}", *handle1, *handle2);

                    active_collisions.push((*handle1, *handle2));
                }
                ContactEvent::Stopped {
                    0: handle1,
                    1: handle2,
                } => {
                    // println!("Collision stopped {:?}, {:?}", *handle1, *handle2);
                    let index = active_collisions
                        .iter()
                        .position(|contact| *contact == (*handle1, *handle2))
                        .expect("Bug in ncollide2d - stopped collision that didn't start");
                    active_collisions.swap_remove(index);
                }
            }
        });
    }
}

// to_remove.into_iter().for_each(|handles| {
//     let index = active_collisions.iter().position(|contact| *contact == handles).expect("Bug in ncollide2d - stopped collision that didn't start");
//     active_collisions.swap_remove(index);
// })
