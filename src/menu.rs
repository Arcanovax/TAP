use macroquad::prelude::*;
mod options;


const MENU_SIZE: Vec2 = vec2(300.0, 400.0);
const LABELS: [&str; 3] = ["Play", "Options", "Quit"];


use crate::*;
use options::OptionsSettings;
use options::handle_options;


pub struct Menu {
    pub is_active: bool,
    state: i32,
	options: OptionsSettings,
}


impl Menu {
    pub fn new() -> Self {
        Self {
            is_active: false,
            state: 0,
			options: OptionsSettings{
				is_fullscreen: false,
			}
        }
    }
}



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



pub fn handle_menu(game: &mut Game) {
	let menu: &mut Menu = &mut game.menu;
    if is_key_pressed(KeyCode::Escape) && game.focus == InputFocus::Game{
        if !menu.is_active {
            menu.is_active = true;
        }
		else
        {
			menu.is_active = false;
			menu.state = 0;
        }
    }
    if !menu.is_active {
        return;
    }
    if menu.state == 4{
        return;
    }

	let menu_rect = get_menu_rect();
	let mouse = mouse_position();

	for i in 0..LABELS.len(){
            let rect = Rect::new(menu_rect.x + 20.0, menu_rect.y + 50.0 + (i as f32 * 125.0), 250.0, 75.0);
            let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));
			if hovered && is_mouse_button_pressed(MouseButton::Left) {
				menu.state = i as i32 + 1;
        	}
        }


    match menu.state {
        1 => {
            menu.state = 0;
            menu.is_active = false;
            return
        }
        2 => {
			menu.state = 4;
            return
        }
        3 => {
            std::process::exit(0);
        }
        _ => {}
}
}

pub fn draw_menu(game: &mut Game) {


    if !game.menu.is_active {
		return;
	}
	draw_rectangle(0.0,0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 150));

	if game.menu.state == 0{
		let menu_rect = get_menu_rect();
		let mouse = mouse_position();

		for (i, label) in LABELS.iter().enumerate() {
			let rect = Rect::new(menu_rect.x + 20.0, menu_rect.y + 50.0 + (i as f32 * 125.0), 250.0, 75.0);
			let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));

			let bg_color = if hovered {
				Color::new(1.0, 1.0, 1.0, 0.25)
			} else {
				Color::from_rgba(255,255, 255, 150)
			};


			draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
			let text_size = measure_text(label, None, 60, 1.0);
			let text_pos = Vec2::new(
            	rect.x + (rect.w - text_size.width) / 2.0,
            	rect.y + (rect.h + text_size.height) / 2.0 - 5.0
        	);
			draw_text(label, text_pos.x, text_pos.y, 60.0, BLACK);
		}
    }
	else if game.menu.state == 4{
		handle_options(game);
	}

}

