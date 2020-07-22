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

// To test physics tags
use crate::phx::BodyTag;

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
        use crate::phx::PhysicsWorld;
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

        new_player(
            &mut game_data.world,
            &mut cool,
            mint::Vector2 { x: 120., y: 95. },
            &image_copy,
        );
        new_obstacle(
            &mut game_data.world,
            &mut cool,
            mint::Vector2 { x: 150., y: 150. },
            &image_copy,
        );
        new_obstacle(
            &mut game_data.world,
            &mut cool,
            mint::Vector2 { x: 200., y: 120. },
            &image_copy,
        );
        new_zone(
            &mut game_data.world,
            &mut cool,
            mint::Vector2 { x: 125., y: 125. },
            &image_copy,
        );
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

// TODO: Proper spawning instead of this... thing
fn new_player(
    world: &mut legion::prelude::World,
    cworld: &mut crate::phx::PhysicsWorld,
    position: mint::Vector2<f32>,
    image: &Image,
) {
    use crate::phx::{Category, Hitbox};
    use resphys::builder::{BodyBuilder, Shape};

    let img_size: mint::Vector2<f32> = (image.size() / 2).into();
    let body = BodyBuilder::new(Shape::AABB(img_size.into()), position.into(), BodyTag::PC)
        .with_category(Category::ALLY.bits())
        .with_velocity(mint::Vector2 { x: 25., y: 16. }.into())
        .build();
    let hitbox = Hitbox::new(cworld, body);

    let _with_collision = world
        .insert(
            (),
            vec![(
                Position {
                    src: position.into(),
                },
                Sprite::new("image".into(), &image),
                hitbox,
                Velocity {
                    src: Vector::new(25., 16.),
                },
                // Player,
            )],
        )
        .to_vec();
}

fn new_obstacle(
    world: &mut legion::prelude::World,
    cworld: &mut crate::phx::PhysicsWorld,
    position: mint::Vector2<f32>,
    image: &Image,
) {
    use crate::phx::{Category, Hitbox};
    use resphys::builder::{BodyBuilder, Shape};

    let img_size: mint::Vector2<f32> = (image.size() / 2).into();
    let body = BodyBuilder::new(
        Shape::AABB(img_size.into()),
        position.into(),
        BodyTag::Obstacle,
    )
    .with_category(Category::GROUND.bits())
    .make_static()
    .build();
    let hitbox = Hitbox::new(cworld, body);

    let _with_collision = world
        .insert(
            (),
            vec![(
                Position {
                    src: position.into(),
                },
                Sprite::new("image".into(), &image),
                hitbox,
                Velocity {
                    src: Vector::new(0., 0.),
                },
                // Player,
            )],
        )
        .to_vec();
}

fn new_zone(
    world: &mut legion::prelude::World,
    cworld: &mut crate::phx::PhysicsWorld,
    position: mint::Vector2<f32>,
    image: &Image,
) {
    use crate::phx::{Category, Hitbox};
    use resphys::builder::{BodyBuilder, Shape};

    let img_size: mint::Vector2<f32> = (image.size() / 2).into();
    let body = BodyBuilder::new(
        Shape::AABB(img_size.into()),
        position.into(),
        BodyTag::DummyArea,
    )
    .with_category(Category::GROUND.bits())
    .make_static()
    .sensor()
    .build();
    let hitbox = Hitbox::new(cworld, body);

    let _with_collision = world
        .insert(
            (),
            vec![(
                Position {
                    src: position.into(),
                },
                Sprite::new("image".into(), &image),
                hitbox,
                Velocity {
                    src: Vector::new(0., 0.),
                },
                // Player,
            )],
        )
        .to_vec();
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
