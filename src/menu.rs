use macroquad::prelude::*;

use crate::config;

pub struct Menu {
    pub is_active: bool,
    state: i32,
}


impl Menu {
    pub fn new() -> Self {
        Self {
            is_active: false,
            state: 0
        }
    }
}

pub fn update_menu(menu: &mut Menu, chat_active: bool) {
    if is_key_pressed(KeyCode::Escape) && !chat_active{
        if !menu.is_active {
            menu.is_active = true;
        }
		else
        {
			menu.is_active = false;
        }
    }
    if !menu.is_active {
        return;
    }
    let buttons = [
        (1, Rect::new(110.0, 80.0,  180.0, 25.0)),
        (2, Rect::new(110.0, 112.0, 180.0, 25.0)),
        (3, Rect::new(110.0, 144.0, 180.0, 25.0)),
    ];
    let mouse = mouse_position();

    for (id, rect) in &buttons {
        let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));
        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            menu.state = *id;
            menu.is_active = false;
        }
    }

    match menu.state {
        1 => {
            menu.state = 0;
            menu.is_active = false;
            return
        }
        2 => {
            menu.state = 0;
			config().fullscreen = true;
            menu.is_active = false;
            return
        }
        3 => {
            std::process::exit(0);
        }
        _ => {}
}
}

pub fn draw_menu(menu: &Menu) {

    if menu.is_active {
        draw_rectangle(100.0, 70.0 , 200.0, 100.0, Color::new(255.0, 193.0, 0.0, 0.9));
        let labels = ["Jouer", "Options", "Quitter"];
        let mouse = mouse_position();

        for (i, label) in labels.iter().enumerate() {
            let rect = Rect::new(110.0, 80.0 + i as f32 * 32.0, 180.0, 25.0);
            let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));

            let bg_color = if hovered {
                Color::new(1.0, 1.0, 1.0, 1.0)
            } else {
                Color::new(1.0, 1.0, 1.0, 0.05)
            };

            draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
            draw_text(label, rect.x + 8.0, rect.y + 17.0, 18.0, WHITE);
        }
    }

}

