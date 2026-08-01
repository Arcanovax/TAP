use crate::Game;
use macroquad::{miniquad::window::set_window_size, prelude::*};
pub struct OptionsSettings {
    pub is_fullscreen: bool,
}

fn draw_checkbox(x: f32, y: f32, label: &str, checked: bool, mouse: Vec2) -> bool {
    let size = 24.0;
    let rect = Rect::new(x, y, size, size);
    let hovered = rect.contains(mouse);

    let bg = if hovered {
        Color::new(0.3, 0.3, 0.3, 1.0)
    } else {
        Color::new(0.15, 0.15, 0.15, 1.0)
    };
    draw_rectangle(x, y, size, size, bg);
    draw_rectangle_lines(x, y, size, size, 4.0, WHITE);

    if checked {
        draw_rectangle(x + 5.0, y + 5.0, size - 10.0, size - 10.0, WHITE);
    }

    draw_text(label, x + size + 15.0, y + 18.0, 35.0, WHITE);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

pub fn handle_options(game: &mut Game) {
    let mouse = game.mouse;
    let menu_name: &str = "OPTIONS";

    let title_size = measure_text(menu_name, None, 70, 1.0);
    let title_pos = Vec2::new(
        (screen_width() - title_size.width) / 2.0,
        (screen_height() + title_size.height) / 10.0,
    );
    draw_text(menu_name, title_pos.x, title_pos.y + 60.0, 70.0, WHITE);

    let start_x = title_pos.x;
    let current_y = title_pos.y + 110.0;

    if draw_checkbox(
        start_x,
        current_y,
        "Full screen",
        game.menu.options.is_fullscreen,
        mouse,
    ) {
        game.menu.options.is_fullscreen = !game.menu.options.is_fullscreen;
        if game.menu.options.is_fullscreen {
            set_fullscreen(true);
        } else {
            set_fullscreen(false);
            set_window_size(1280, 720);
        }
    }
}
