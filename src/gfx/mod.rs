use crate::game::Game;
use quicksilver::lifecycle::Window;
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Color, Graphics},
};

use crate::engine::components::{Position, Sprite};
use legion::prelude::*;

mod debug_info;

pub async fn render(window: &Window, gfx: &mut Graphics, game_data: &Game) {
    let fill = Rectangle::new_sized(Vector::new(320., 180.));
    gfx.clear(Color::BLACK);
    gfx.set_transform(Transform::IDENTITY);
    gfx.fill_rect(&fill, Color::CYAN);

    let query = <(Read<Position>, Read<Sprite>)>::query();
    for (pos, img) in query.iter(&game_data.world) {
        // TODO: Handle the error by using default texture
        let image = game_data.images.get(&img.src).unwrap();
        gfx.draw_image(&image, Rectangle::new(pos.src + img.offset, image.size()));
    }

    if cfg!(feature = "debug-info") {
        self::debug_info::visualize_hitbox(gfx, game_data);
    }

    let _ = gfx.present(&window);
}
