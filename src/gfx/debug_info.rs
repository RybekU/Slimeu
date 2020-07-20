use crate::game::Game;
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, Graphics},
};

use resphys::{BodyState, Shape};
use crate::phx::Hitbox;
use crate::phx::PhysicsWorld;
use legion::prelude::*;

pub fn visualize_hitbox(gfx: &mut Graphics, game_data: &Game) {
    let query = <Read<Hitbox>>::query();
    let pworld = game_data
        .resources
        .get::<PhysicsWorld>()
        .expect("PhysicsWorld missing somehow");
    for hitbox in query.iter(&game_data.world) {
        let physics_body = pworld
            .get_body(hitbox.src)
            .expect("Debug_Info: Handle to invalid collision object");
        let position = physics_body.position;
        use Shape::*;
        match physics_body.shape {
            AABB(half_extents) => {
                let area = Rectangle::new(
                    Vector::new(position.x - half_extents.x, position.y - half_extents.y),
                    Vector::new(half_extents.x, half_extents.y) * 2.0,
                );
                let color = match physics_body.state {
                    BodyState::Solid => Color::BLUE,
                    BodyState::Sensor => Color::YELLOW,
                };
                gfx.fill_rect(&area, color.with_alpha(0.2));
                gfx.stroke_rect(&area, color);
            }
        }
        // visualise the contacts
        for (_, _, manifold) in pworld.manifolds.iter() {
            for (contact, color) in manifold
                .contacts
                .iter()
                .zip([Color::ORANGE, Color::RED].iter())
            {
                match contact {
                    None => break,
                    Some(c) => {
                        let points = vec![c.contact_point, c.contact_point - c.normal * c.depth];
                        gfx.stroke_circle(&Circle::new(c.contact_point, 2.), *color);
                        gfx.stroke_path(&points, *color);
                    }
                }
            }
        }
    }
}
