use crate::game::Game;
use quicksilver::geom::Vector;
use quicksilver::graphics::Graphics;
use quicksilver::lifecycle::{Event, EventCache, EventStream, Window};

use crate::DIMENSIONS;

pub async fn handle_events(
    window: &Window,
    gfx: &Graphics,
    events: &mut EventStream,
    game_data: &mut Game,
) {
    let mut input_cache = game_data
        .resources
        .get_mut::<EventCache>()
        .expect("No button_state!");

    while let Some(event) = events.next_event().await {
        input_cache.process_event(&event);
        match event {
            Event::Resized(resized) => {
                let win_size = Vector::from(resized.logical_size()) * window.scale_factor();
                let new_viewport = game_data.resize_strategy.resize(DIMENSIONS, win_size);
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
}
