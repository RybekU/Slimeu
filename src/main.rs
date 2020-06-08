use golem::TextureFilter;
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Graphics, Image},
    lifecycle::{run, EventStream, Settings, Window},
    Result, Timer,
};

use engine::ResizeStrategy;

use game::Game;

#[macro_use]
extern crate log;

mod engine;
mod events;
mod game;
mod gfx;
mod phx;

pub use game::DIMENSIONS;
pub use game::UPDATE_RATE;

// To add test entities
use crate::engine::components::{Position, Sprite};

// To test velocity
use crate::phx::Velocity;

fn main() {
    run(
        Settings {
            size: Vector::new(320.0 * 3., 180.0 * 3.).into(),
            title: "Slimeu",
            // resizable: true,
            // fullscreen: true,
            // vsync: false,
            ..Settings::default()
        },
        app,
    );
}
struct Player;
// This time we might return an error, so we use a Result
async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the image and wait for it to finish
    // We also use '?' to handle errors like file-not-found
    let image = Image::load(&gfx, "image.png").await?;
    let image_copy = image.clone();

    image.set_magnification(TextureFilter::Nearest)?;

    let mut game_data = Game::new();
    game_data.resize_strategy = set_resize_strategy(&window, &gfx);
    game_data.images.insert("image".into(), image);

    {
        use engine::physics::PhysicsWorld;
        let mut cool = game_data
            .resources
            .get_mut::<PhysicsWorld>()
            .expect("PhysicsWorld missing somehow");

        // Test add some entities with Position and Image use crate::engine::components::{Position, Sprite};
        let _entities = game_data
            .world
            .insert(
                (),
                vec![
                    (
                        Position { src: Vector::ZERO },
                        Sprite::new("image".into(), &image_copy),
                    ),
                    (
                        Position {
                            src: Vector::new(25., 25.),
                        },
                        Sprite::new("image".into(), &image_copy),
                    ),
                ],
            )
            .to_vec();
        use crate::phx::Hitbox;
        // Test adding collision to entity
        let hitbox = Hitbox::new(&mut cool, Vector::new(120., 95.), image_copy.size(), true);
        let _with_collision = game_data
            .world
            .insert(
                (),
                vec![(
                    Position {
                        src: Vector::new(120., 95.),
                    },
                    Sprite::new("image".into(), &image_copy),
                    hitbox,
                    Velocity {
                        src: Vector::new(25., 16.),
                    },
                    // Player,
                )],
            )
            .to_vec();
    }
    {
        use engine::physics::PhysicsWorld;
        let mut cool = game_data
            .resources
            .get_mut::<PhysicsWorld>()
            .expect("PhysicsWorld missing somehow");

        // Test add some entities with Position and Image use crate::engine::components::{Position, Sprite};
        // Test adding collision to entity
        use crate::phx::Hitbox;
        let hitbox = Hitbox::new(&mut cool, Vector::new(150., 150.), image_copy.size(), false);
        let hitbox2 = Hitbox::new(&mut cool, Vector::new(125., 125.), image_copy.size(), false);

        let _with_collision = game_data
            .world
            .insert(
                (),
                vec![
                    (
                        Position {
                            src: Vector::new(150., 150.),
                        },
                        Sprite::new("image".into(), &image_copy),
                        hitbox,
                        Velocity {
                            src: Vector::new(0., 0.),
                        },
                    ),
                    (
                        Position {
                            src: Vector::new(125., 125.),
                        },
                        Sprite::new("image".into(), &image_copy),
                        hitbox2,
                        Velocity {
                            src: Vector::new(0., 0.),
                        },
                    ),
                ],
            )
            .to_vec();
    }
    let camera = Transform::orthographic(Rectangle::new(Vector::ZERO, DIMENSIONS));
    gfx.set_projection(camera);

    let mut update_timer = Timer::time_per_second(UPDATE_RATE);
    let mut counter = 0;
    loop {
        crate::events::handle_events(&window, &gfx, &mut events, &mut game_data).await;

        while update_timer.tick() {
            game_data
                .schedule
                .execute(&mut game_data.world, &mut game_data.resources);

            counter += 1;
            if counter >= 60 {
                // info!("Every {} seconds in Africa a minute passes.", counter);
                counter = 0;
            }
        }

        crate::gfx::render(&window, &mut gfx, &game_data);
    }
}

fn set_resize_strategy(window: &Window, gfx: &Graphics) -> ResizeStrategy {
    let win_size = Vector::from(window.size()) * window.scale_factor();
    let resize_strategy = ResizeStrategy::IntegerScale {
        width: DIMENSIONS.x as u32,
        height: DIMENSIONS.y as u32,
    };

    let new_viewport = resize_strategy.resize(DIMENSIONS, win_size);
    gfx.set_viewport(
        new_viewport.x() as u32,
        new_viewport.y() as u32,
        new_viewport.width() as u32,
        new_viewport.height() as u32,
    );

    resize_strategy
}

//gfx.set_transform(Transform::translate(location) * Transform::translate(-image.size()/2) *Transform::rotate(30));
//gfx.draw_image(&image, Rectangle::new(image.size()/2, image.size()));

// gfx.set_transform(
//     Transform::translate(center) * Transform::rotate(30.0) * Transform::translate(-center),
// );
