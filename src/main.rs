use golem::TextureFilter;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Color, Graphics, Image},
    lifecycle::{run, Event, EventStream, Settings, Window},
    Result, Timer,
};

// for input.rs
use engine::input::Button;
use engine::ButtonsState;
use quicksilver::lifecycle::EventCache;

#[macro_use]
extern crate log;

mod engine;

use engine::ResizeStrategy;

fn main() {
    run(
        Settings {
            size: Vector::new(320.0 * 3., 180.0 * 3.).into(),
            title: "Image Example",
            resizable: true,
            // fullscreen: true,
            // vsync: false,
            ..Settings::default()
        },
        app,
    );
}

#[derive(Default)]
struct Resources {
    input_cache: EventCache,
    button_state: ButtonsState,
}

pub const DIMENSIONS: Vector = Vector { x: 320., y: 180. };
pub const SCALE: f32 = 3.0;
//320 x 180 or 480 x 270
// This time we might return an error, so we use a Result
async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the image and wait for it to finish
    // We also use '?' to handle errors like file-not-found
    let image = Image::load(&gfx, "image.png").await?;
    image.set_magnification(TextureFilter::Nearest)?;

    let resize_strategy = ResizeStrategy::IntegerScale {
        width: DIMENSIONS.x as u32,
        height: DIMENSIONS.y as u32,
    };

    let coords = Rectangle::new(Vector::ZERO, image.size());

    // Work on input module
    let mut resources = Resources::default();

    let win_size = Vector::from(window.size()) * window.scale_factor();
    let camera = Transform::orthographic(Rectangle::new(Vector::ZERO, DIMENSIONS));
    let mut new_viewport = resize_strategy.resize(DIMENSIONS, win_size);
    gfx.set_viewport(
        new_viewport.x() as u32,
        new_viewport.y() as u32,
        new_viewport.width() as u32,
        new_viewport.height() as u32,
    );

    let fill = Rectangle::new_sized(Vector::new(320., 180.));

    let mut update_timer = Timer::time_per_second(60.0);
    let mut counter = 0;
    loop {
        while let Some(event) = events.next_event().await {
            resources.input_cache.process_event(&event);
            match event {
                Event::Resized(resized) => {
                    let win_size = Vector::from(resized.logical_size()) * window.scale_factor();
                    new_viewport = resize_strategy.resize(DIMENSIONS, win_size);
                    gfx.set_viewport(
                        new_viewport.x() as u32,
                        new_viewport.y() as u32,
                        new_viewport.width() as u32,
                        new_viewport.height() as u32,
                    );
                }
                _ => {}
            }
        }

        while update_timer.tick() {
            resources.button_state.update(&resources.input_cache);
            if resources.button_state.is_pressed(Button::Up) {
                debug!("Holding UP!");
            }
            if resources.button_state.pressed(Button::Jump) {
                debug!("Wow you just pressed the Jump button.");
            }
            if resources.button_state.released(Button::Jump) {
                debug!("Congrats on releasing the Jump button");
            }
            counter += 1;
            if counter >= 60 {
                // info!("Every {} seconds in Africa a minute passes.", counter);
                counter = 0;
            }
        }
        gfx.clear(Color::BLACK);
        gfx.set_projection(camera);
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
