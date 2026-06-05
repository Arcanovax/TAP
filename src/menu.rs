


use macroquad::prelude::*;

const MENU_SIZE: Vec2 = vec2(600.0, 400.0);
const LABELS: [&str; 3] = ["Play", "Options", "Quit"];

fn get_menu_rect() -> Rect {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    Rect::new(
        center_x - (MENU_SIZE.x / 2.0),
        center_y - (MENU_SIZE.y / 2.0),
        MENU_SIZE.x,
        MENU_SIZE.y,
    )
}

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

	let menu_rect = get_menu_rect();
	draw_rectangle(menu_rect.x, menu_rect.y, menu_rect.w, menu_rect.h, Color::new(255.0, 193.0, 0.0, 0.9));
	let mouse = mouse_position();

	for i in 0..LABELS.len(){
            let rect = Rect::new(menu_rect.x + 20.0, menu_rect.y + 50.0 + (i as f32 * 125.0), 180.0, 60.0);
            let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));
			if hovered && is_mouse_button_pressed(MouseButton::Left) {
				menu.state = i as i32 + 1;
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
		let menu_rect = get_menu_rect();
        draw_rectangle(menu_rect.x, menu_rect.y, menu_rect.w, menu_rect.h, Color::new(255.0, 193.0, 0.0, 0.9));

        let mouse = mouse_position();

        for (i, label) in LABELS.iter().enumerate() {
            let rect = Rect::new(menu_rect.x + 20.0, menu_rect.y + 50.0 + (i as f32 * 125.0), 180.0, 60.0);
            let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));

            let bg_color = if hovered {
                Color::new(1.0, 1.0, 1.0, 0.25)
            } else {
                Color::new(1.0, 1.0, 1.0, 0.5)
            };

            draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
            draw_text(label, rect.x, rect.y+50.0, 80.0, WHITE);
        }
    }

}

