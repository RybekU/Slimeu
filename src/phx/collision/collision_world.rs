use super::{CollisionData, CollisionGroup, CollisionHandle, CollisionObject};
use crate::phx::Iso2;
use ncollide2d::pipeline::narrow_phase::{ContactAlgorithm, ContactEvent};
use ncollide2d::query::ContactManifold;
type World = ncollide2d::world::CollisionWorld<f32, CollisionData>;
type ActiveCollisions = Vec<(CollisionHandle, CollisionHandle)>;
use nalgebra::Vector2;

use quicksilver::geom::Vector;

use std::ops::Deref;

use ncollide2d::query::{self, TOI};
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
        self.src.narrow_phase.clear_events();

        // world.update();
        self.src.perform_broad_phase();
        self.ccd_phase();
        self.src.perform_broad_phase();
        self.src.perform_narrow_phase();

        let active_collisions = &mut self.active_collisions;
        let world = &mut self.src;

        world
            .contact_events()
            .into_iter()
            .for_each(|event| match event {
                ContactEvent::Started {
                    0: handle1,
                    1: handle2,
                } => {
                    // debug!("Collision started {:?}, {:?}", *handle1, *handle2);

                    active_collisions.push((*handle1, *handle2));
                }
                ContactEvent::Stopped {
                    0: handle1,
                    1: handle2,
                } => {
                    // debug!("Collision stopped {:?}, {:?}", *handle1, *handle2);
                    let index = active_collisions
                        .iter()
                        .position(|contact| *contact == (*handle1, *handle2))
                        .expect("Bug in ncollide2d - stopped collision that didn't start");
                    active_collisions.swap_remove(index);
                }
            });
    }

    fn ccd_phase(&mut self) {
        // due to using only AABB no rotation a collision can happen at max twice (one per axis) so no need to substep
        let dt = crate::UPDATE_RATE;
        let world = &mut self.src;

        // if it didnt collide with anything deal with them separately
        let mut no_collision: Vec<CollisionHandle> = vec![];
        let mut fix_collision: Vec<(CollisionHandle, CollisionHandle, Vector2<f32>, TOI<f32>)> =
            vec![];

        for (handle1, obj1) in world.objects.iter() {
            if let Some(pred_pos1) = obj1.predicted_position() {
                no_collision.push(handle1);
                // TODO: Make it an actual struct and implement min
                let mut toi_struct: Option<(
                    CollisionHandle,
                    CollisionHandle,
                    Vector2<f32>,
                    TOI<f32>,
                )> = None;
                for (h1, h2, interaction) in world.interactions_with(handle1, false).unwrap() {
                    if !interaction.is_contact() {
                        continue;
                    }
                    // debug!("Possible collision: {:?}, {:?}; interaction: {:?}", h1, h2, interaction.is_contact());
                    // debug!("Position: {:?}, Predicted: {:?}", obj.position(), pred_pos);
                    let handle2 = if handle1 == h1 { h2 } else { h1 };
                    let obj2 = world.collision_object(handle2).expect("Error in ncollide");
                    let pred_pos2 = obj2.predicted_position().unwrap_or(obj2.position());

                    let pos1 = obj1.position();
                    let pos2 = obj2.position();

                    let vel1 = Vector2::new(
                        pred_pos1.translation.x - pos1.translation.x,
                        pred_pos1.translation.y - pos1.translation.y,
                    );
                    let vel2 = Vector2::new(
                        pred_pos2.translation.x - pos2.translation.x,
                        pred_pos2.translation.y - pos2.translation.y,
                    );
                    let toi = query::time_of_impact(
                        pos1,
                        &vel1,
                        obj1.shape().deref(),
                        pos2,
                        &vel2,
                        obj2.shape().deref(),
                        dt,
                        0.01,
                    );
                    debug!("H1: {:#?}, H2: {:#?}, toi: {:#?}", handle1, handle2, toi);
                    if let Some(toi) = toi {
                        // take only the minimal toi
                        toi_struct = match toi_struct {
                            Some(min_toi) => {
                                if toi.toi < min_toi.3.toi {
                                    debug!("New toi: {} old toi: {}", toi.toi, min_toi.3.toi);
                                    Some((handle1, handle2, vel1, toi))
                                } else {
                                    Some(min_toi)
                                }
                            }
                            None => Some((handle1, handle2, vel1, toi)),
                        };
                    }
                }
                toi_struct
                    .into_iter()
                    .for_each(|toi_struct| fix_collision.push(toi_struct));
            }
        }

        fix_collision
            .into_iter()
            .for_each(|(h, h_other, vel, toi)| {
                let obj = world.get_mut(h).unwrap();
                let position = obj.position();
                // let pred_position = obj.predicted_position().expect("If it doesn't exist there's nothing to fix");
                let vel_used;
                if toi.toi > 1e-5 {
                    vel_used = vel * toi.toi;
                } else {
                    vel_used = Vector2::zeros();
                }

                let translation = position.translation;
                let new_position =
                    Iso2::translation(translation.x + vel_used.x, translation.y + vel_used.y);
                obj.set_position(new_position);
                non_linear_gauss_seidel(world, h, h_other, 20, 0.05);
            });

        // lastly move every body that still has any predicted position set
        no_collision.into_iter().for_each(|h| {
            if let Some(obj) = world.get_mut(h) {
                if let Some(pred_pos) = obj.predicted_position().copied() {
                    obj.set_position(pred_pos);
                }
            }
        });
    }
}

// An iterative algorithm to solve non-linear big matrix problems.
// hits is the body colliding that will get skin factor applied and is_hit is the one that got collided with
// TODO: test how many iterations are necessary on average
// Good default skin factor value is 0.05
fn non_linear_gauss_seidel(
    cworld: &mut World,
    hits: CollisionHandle,
    is_hit: CollisionHandle,
    max_iterations: u32,
    skin_factor: f32,
) {
    // add the skin factor
    let cuboid = {
        let object = cworld
            .get_mut(hits)
            .expect("Debug_Info: Handle to invalid collision object");
        let cuboid = object
            .shape()
            .as_shape::<ncollide2d::shape::Cuboid<f32>>()
            .expect("Debug_Info: The shape isn't a cuboid")
            .clone();
        let half_extents = cuboid.half_extents();
        let skin_cuboid =
            ncollide2d::shape::Cuboid::new(half_extents + Vector2::repeat(skin_factor));
        object.set_shape(ncollide2d::shape::ShapeHandle::new(skin_cuboid));
        cuboid
    };

    for n in 0..max_iterations {
        let pos_correction;
        cworld.perform_broad_phase();

        // TODO: match everything that collides!
        match cworld.contact_pair(hits, is_hit, true) {
            Some((_, _, _, contact)) => {
                let deepest = contact.deepest_contact().unwrap().contact;
                pos_correction = -deepest.normal.into_inner() * (deepest.depth);
            }
            //
            None => {
                // debug!("ngs finished because no collision after count {}", n);
                break;
            }
        }
        if pos_correction == crate::phx::Vec2::zeros() {
            // debug!("ngs finished because skin was 0 at count {}", n);
            break;
        }
        // correct the position slightly
        else {
            let obj = cworld.get_mut(hits).unwrap();
            let position = obj.position();
            let translation = position.translation;
            let new_position = Iso2::translation(
                translation.x + pos_correction.x,
                translation.y + pos_correction.y,
            );
            obj.set_position(new_position);
        }
    }
    // remove the skin factor
    cworld
        .get_mut(hits)
        .unwrap()
        .set_shape(ncollide2d::shape::ShapeHandle::new(cuboid));
}

// to_remove.into_iter().for_each(|handles| {
//     let index = active_collisions.iter().position(|contact| *contact == handles).expect("Bug in ncollide2d - stopped collision that didn't start");
//     active_collisions.swap_remove(index);
// })
