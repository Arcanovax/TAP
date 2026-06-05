use macroquad::{miniquad::window::set_window_size, prelude::*};

const MENU_SIZE: Vec2 = vec2(300.0, 400.0);
const  OPTION_SIZE: Vec2 = vec2(400.0, 600.0);
const LABELS: [&str; 3] = ["Play", "Options", "Quit"];


pub struct OptionsSettings {
    pub charactere: usize,
	pub	is_fullscreen: bool,

}

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
				charactere: 0
			}
        }
    }
}

fn draw_checkbox(x: f32, y: f32, label: &str, checked: bool, mouse: (f32, f32)) -> bool {
    let size = 24.0;
    let rect = Rect::new(x, y, size, size);
    let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));


    let bg = if hovered { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.15, 0.15, 0.15, 1.0) };
    draw_rectangle(x, y, size, size, bg);
    draw_rectangle_lines(x, y, size, size, 4.0, WHITE);


    if checked {
        draw_rectangle(x + 5.0, y + 5.0, size - 10.0, size - 10.0, WHITE);
    }


    draw_text(label, x + size + 15.0, y + 18.0, 35.0, WHITE);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

pub fn process_options_menu(settings: &mut OptionsSettings) {
    let mouse = mouse_position();
	let menu_name: &str = "OPTIONS";

	let title_size = measure_text(menu_name, None, 70, 1.0);
	let title_pos = Vec2::new(
            	(screen_width() - title_size.width) / 2.0,
            	(screen_height() + title_size.height) / 10.0
        	);
    draw_text(menu_name, title_pos.x, title_pos.y+ 60.0, 70.0, WHITE);


    let start_x = title_pos.x;
    let current_y = title_pos.y + 110.0;

	if draw_checkbox(start_x, current_y, "Full screen", settings.is_fullscreen, mouse) {
        settings.is_fullscreen = !settings.is_fullscreen;
        if settings.is_fullscreen {
            set_fullscreen(true);
        } else {
            set_fullscreen(false);
            set_window_size(1280, 720);
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



pub fn update_menu(menu: &mut Menu, chat_active: bool) {
    if is_key_pressed(KeyCode::Escape) && !chat_active{
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

pub fn draw_menu(menu: &mut Menu) {


    if !menu.is_active {
		return;
	}
	draw_rectangle(0.0,0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 150));

	if menu.state == 0{
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
	else if menu.state == 4{
		process_options_menu(&mut menu.options);
	}

}

