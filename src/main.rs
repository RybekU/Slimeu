use golem::TextureFilter;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Color, Graphics, Image},
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

pub use game::DIMENSIONS;

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

// This time we might return an error, so we use a Result
async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the image and wait for it to finish
    // We also use '?' to handle errors like file-not-found
    let image = Image::load(&gfx, "image.png").await?;
    image.set_magnification(TextureFilter::Nearest)?;

    let coords = Rectangle::new(Vector::ZERO, image.size());

    let mut game_data = Game::new();
    game_data.resize_strategy = set_resize_strategy(&window, &gfx);

    // Experimentally add Arc<Window> to gamedata
    // use std::sync::Arc;
    // game_data.resources.insert(Arc::<Window>::new(window));

    let camera = Transform::orthographic(Rectangle::new(Vector::ZERO, DIMENSIONS));
    gfx.set_projection(camera);

    let fill = Rectangle::new_sized(Vector::new(320., 180.));

    let mut update_timer = Timer::time_per_second(60.0);
    let mut counter = 0;
    loop {
        // TODO: Move event handling to events.rs
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
        gfx.clear(Color::BLACK);
        gfx.set_transform(Transform::IDENTITY);

        gfx.fill_rect(&fill, Color::CYAN);
        gfx.draw_image(&image, Rectangle::new_sized(image.size()));

        //gfx.set_transform(Transform::translate(location) * Transform::translate(-image.size()/2) *Transform::rotate(30));
        //gfx.draw_image(&image, Rectangle::new(image.size()/2, image.size()));

        let center = coords.center();
        gfx.set_transform(
            Transform::translate(center) * Transform::rotate(30.0) * Transform::translate(-center),
        );
        gfx.draw_image(&image, coords);

        gfx.present(&window)?;
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
