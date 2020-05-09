use crate::game::Game;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
};

use crate::phx::{CollisionWorld, Hitbox};
use legion::prelude::*;
use ncollide2d::shape::Cuboid;

pub fn visualize_hitbox(gfx: &mut Graphics, game_data: &Game) {
    let query = <Read<Hitbox>>::query();
    let cworld = game_data
        .resources
        .get::<CollisionWorld>()
        .expect("CollisionWorld missing somehow");
    for hitbox in query.iter(&game_data.world) {
        let ncollide_rectangle = cworld
            .collision_object(hitbox.src)
            .expect("Debug_Info: Handle to invalid collision object");
        let position = ncollide_rectangle.position().translation;
        let half_rect = ncollide_rectangle
            .shape()
            .as_shape::<Cuboid<f32>>()
            .expect("Debug_Info: The shape isn't a cuboid")
            .half_extents();
        let area = Rectangle::new(
            Vector::new(position.x - half_rect.x, position.y - half_rect.y),
            Vector::new(half_rect.x, half_rect.y) * 2.0,
        );
        gfx.fill_rect(&area, Color::BLUE.with_alpha(0.2));
        gfx.stroke_rect(&area, Color::BLUE);
    }
}
