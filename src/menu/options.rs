
use macroquad::{miniquad::window::set_window_size, prelude::*};
use crate::Player;

// const LABELS: [&Texture2D; 2] = [
//     load_texture("assets/skins/alex.png"),
//     load_texture("assets/skins/alex.png")
//     ];

pub struct OptionsSettings {
	pub	is_fullscreen: bool,

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

fn draw_skin_selection(x: f32, y: f32, mouse: (f32, f32)) -> i32 {
    let size = 24.0;
    let padding = 10.0; 
    let mouse_pos = Vec2::new(mouse.0, mouse.1);
    
  
    let btn_left_rect = Rect::new(x, y, size, size);
    let btn_right_rect = Rect::new(x + size + padding, y, size, size);

    let left_hovered = btn_left_rect.contains(mouse_pos);
    let right_hovered = btn_right_rect.contains(mouse_pos);


    let bg_left = if left_hovered { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.15, 0.15, 0.15, 1.0) };
    let bg_right = if right_hovered { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.15, 0.15, 0.15, 1.0) };

    draw_rectangle(btn_left_rect.x, btn_left_rect.y, size, size, bg_left);
    draw_text("<", btn_left_rect.x + 6.0, btn_left_rect.y + 18.0, 20.0, WHITE);

    draw_rectangle(btn_right_rect.x, btn_right_rect.y, size, size, bg_right);
    draw_text(">", btn_right_rect.x + 6.0, btn_right_rect.y + 18.0, 20.0, WHITE);

    if left_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return -1; 
    }
    if right_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return 1; 
    }
    return 0;
}


pub fn handle_options(options: &mut OptionsSettings , player: &mut Player) {
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

	if draw_checkbox(start_x, current_y, "Full screen", options.is_fullscreen, mouse){
        options.is_fullscreen = !options.is_fullscreen;
        if options.is_fullscreen {
            set_fullscreen(true);
        } else {
            set_fullscreen(false);
            set_window_size(1280, 720);
        }
    }

    let state_selector = draw_skin_selection(start_x, current_y + 50.0, mouse);
    if state_selector != 0 {
        player.spritesheet_index = ((player.spritesheet_index as i32 + state_selector + 4) % 4) as usize;
    }
}

