use crate::phx::{CollisionGroup, CollisionHandle, CollisionObject, CollisionWorld, Vec2};
use legion::prelude::*;

use crate::phx::PositionCorrection;

// Collisions are stored in Resources, components have to check if they got any collisions
pub fn ncollide_update() -> Box<dyn Schedulable> {
    SystemBuilder::new("ncollide_update")
        .write_resource::<CollisionWorld>()
        .write_resource::<PositionCorrection>()
        .build(move |_, _, (cworld, pos_correct), _| {
            cworld.update();
            let active_collisions = &cworld.active_collisions;
            for (handle1, handle2) in active_collisions.iter() {
                let obj1 = cworld
                    .collision_object(*handle1)
                    .expect("it really should exist now");
                let obj2 = cworld
                    .collision_object(*handle2)
                    .expect("it really should exist now");

                classify_collision(handle1, handle2, obj1, obj2, cworld, pos_correct);
                classify_collision(handle2, handle1, obj2, obj1, cworld, pos_correct);
            }
        })
}

fn classify_collision(
    handle1: &CollisionHandle,
    handle2: &CollisionHandle,
    object1: &CollisionObject,
    object2: &CollisionObject,
    cworld: &CollisionWorld,
    pos_correct: &mut PositionCorrection,
) {
    let obj1_data = object1.data();
    let obj2_data = object2.data();
    match (obj1_data.group, obj2_data.group) {
        (_any, CollisionGroup::Terrain) => {
            if let Some((_, _, _, contact)) = cworld.contact_pair(*handle1, *handle2, true) {
                let deepest = contact.deepest_contact().unwrap().contact;
                let pos_correction = -deepest.normal.into_inner() * deepest.depth;
                if pos_correction == Vec2::zeros() {
                    return;
                }
                let current_correction = pos_correct.entry(*handle1).or_insert(Vec2::zeros());
                *current_correction = Vec2::new(
                    best_correction(current_correction.x, pos_correction.x),
                    best_correction(current_correction.y, pos_correction.y),
                );
                if deepest.depth.abs() > 40. {
                    println!("Correction: {:#?}", current_correction);
                    println!(
                        "Depth {:#?}, {:#?}",
                        deepest.depth,
                        -deepest.normal.into_inner()
                    );
                    println!("Position1: {:#?}", object1.position());
                    println!("Position2: {:#?}", object2.position());
                }
            }
        }
        (_first, _second) => {}
    }
}

fn best_correction(a: f32, b: f32) -> f32 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}
