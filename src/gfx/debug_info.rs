use crate::game::Game;
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Color, Graphics},
};

use crate::phx::Hitbox;
use crate::phx::PhysicsWorld;
use legion::prelude::*;
use resphys::{BodyState, Shape};

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
                    mint::Vector2::from(position - half_extents),
                    mint::Vector2::from(half_extents * 2.0),
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
                        let points: Vec<Vector> = vec![
                            mint::Vector2::from(c.contact_point).into(),
                            mint::Vector2::from(c.contact_point - c.normal * c.depth).into(),
                        ];
                        gfx.stroke_circle(
                            &Circle::new(mint::Vector2::from(c.contact_point), 2.),
                            *color,
                        );
                        gfx.stroke_path(&points, *color);
                    }
                }
            }
        }
    }
}
