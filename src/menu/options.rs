
use macroquad::{miniquad::window::set_window_size, prelude::*};
use crate::Game;
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

fn draw_skin_selection(x: f32, y: f32,current_name: &str, mouse: (f32, f32)) -> i32 {
    let size = 25.0; 
    let mouse_pos = Vec2::new(mouse.0, mouse.1);
    
  
    let btn_left = Rect::new(x, y, size, size);
    let left_hovered = btn_left.contains(mouse_pos);
    let bg_left = if left_hovered { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.15, 0.15, 0.15, 1.0) };
    draw_rectangle(btn_left.x, btn_left.y, btn_left.w, btn_left.h, bg_left);
    draw_text("<", btn_left.x+ 6.0, btn_left.y + 18.0, 30.0, WHITE);


    draw_text(current_name, x+ 30.0, y+20.0, 30.0, WHITE);

    let btn_right = Rect::new(x+125.0, y, 35.0, 35.0);
    let right_hovered = btn_right.contains(mouse_pos);
    let bg_right = if right_hovered { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.15, 0.15, 0.15, 1.0) };
    draw_rectangle(btn_right.x, btn_right.y, size, size, bg_right);
    draw_text(">", btn_right.x + 6.0, btn_right.y + 18.0, 20.0, WHITE);

    if left_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return -1; 
    }
    if right_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return 1; 
    }
    return 0;
}


pub fn handle_options(game: &mut Game) {
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

	if draw_checkbox(start_x, current_y, "Full screen", game.menu.options.is_fullscreen, mouse){
        game.menu.options.is_fullscreen = !game.menu.options.is_fullscreen;
        if game.menu.options.is_fullscreen {
            set_fullscreen(true);
        } else {
            set_fullscreen(false);
            set_window_size(1280, 720);
        }
    }


    let skin_name = &game.skins[game.player.spritesheet_index].name.to_string();
    let state_selector = draw_skin_selection(start_x, current_y + 50.0, skin_name, mouse);
    let nb_skins: i32 = game.skins.len() as i32;
    if state_selector != 0 {
        game.player.spritesheet_index = ((game.player.spritesheet_index as i32 + state_selector + nb_skins) % nb_skins) as usize;
    }
}

