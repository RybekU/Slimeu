use ncollide2d::pipeline::{CollisionGroups, CollisionObject, CollisionObjectSlabHandle};
use quicksilver::geom::Vector;

use super::{collide, CollisionWorld, Iso2, ObjectID};

// Hitbox - offset should be set relative to the center
pub struct Hitbox {
    pub src: CollisionObjectSlabHandle,
}

// TODO: Study hitbox and hurtbox - decide if you need to differentiate them

impl Hitbox {
    // Position is the center of the hitbox
    pub fn new_without_entity(
        cworld: &mut CollisionWorld,
        position: Vector,
        size: Vector,
    ) -> (Self, &mut CollisionObject<f32, ObjectID>) {
        // If this will also be required somewhere else introduce cast
        let nalgebra_pos = nalgebra::Vector2::new(position.x, position.y);
        let nalgebra_size = nalgebra::Vector2::new(size.x, size.y);
        let ncollide_cuboid = ncollide2d::shape::Cuboid::new(nalgebra_size / 2.0);
        let iso = Iso2::new(nalgebra_pos, 0.0);
        let (src, obj_ref) = cworld.add(
            iso,
            ncollide2d::shape::ShapeHandle::new(ncollide_cuboid),
            CollisionGroups::new().with_membership(&[collide::SOMETHING]),
            ncollide2d::pipeline::GeometricQueryType::Contacts(0.0, 0.0),
            None,
        );
        (Self { src }, obj_ref)
    }
}
