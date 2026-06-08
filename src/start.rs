use macroquad::{audio::play_sound_once, prelude::*};
use crate::Game;




fn draw_skin_selection(x: f32, y: f32,game: &mut Game, mouse: (f32, f32)) -> i32 {
    let size = 50.0; 
    let mouse_pos = Vec2::new(mouse.0, mouse.1);
    
  
    let btn_left = Rect::new(x, y, size, size);
    let left_hovered = btn_left.contains(mouse_pos);
    let bg_left = if left_hovered { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.15, 0.15, 0.15, 1.0) };
    draw_rectangle(btn_left.x, btn_left.y, btn_left.w, btn_left.h, bg_left);
    draw_text("<", btn_left.x+ 6.0, btn_left.y + 18.0, 30.0, WHITE);

    let sprite_width: f32 = 16.0;
    let sprite_height: f32 = 32.0;
    let cut_sheet = DrawTextureParams {
            source: Some(Rect::new(0.0, 0.0, sprite_width, sprite_height - 1.0)),
            dest_size: Some(vec2(sprite_width * 2.0, sprite_height* 2.0 - 1.0)),
            ..Default::default()
        };
    let current_skin = &game.skins[game.player.spritesheet_index as usize];
    draw_texture_ex(
            &current_skin.texture,
			x+75.0, y-5.0,
            WHITE,
            cut_sheet
        );

    

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


fn draw_name_input(x: f32, y: f32,game: &mut Game, mouse: (f32, f32)){
    let input_rect = Rect::new(x, y, 175.0, 40.0);
    let input_hovered = input_rect.contains(Vec2::new(mouse.0, mouse.1));
    
    let input_bg = if input_hovered { Color::new(0.2, 0.2, 0.2, 1.0) } else { Color::new(0.1, 0.1, 0.1, 1.0) };
    draw_rectangle(input_rect.x, input_rect.y, input_rect.w, input_rect.h, input_bg);
    draw_rectangle_lines(input_rect.x, input_rect.y, input_rect.w, input_rect.h, 2.0, GRAY);

    if is_key_pressed(KeyCode::Backspace) {
        game.player.name.pop();
    }
    while let Some(character) = get_char_pressed() {
        if character.is_ascii_graphic() || character == ' ' {
            if game.player.name.len() < 12 {
                game.player.name.push(character);
            }
        }
    }

    let mut display_name = game.player.name.clone();
    if get_time() % 1.0 < 0.5 {
        display_name.push('_');
    }

    if game.player.name.is_empty() && !input_hovered {
        draw_text("Type Name...", input_rect.x + 10.0, input_rect.y + 28.0, 25.0, DARKGRAY);
    } else {
        draw_text(&display_name, input_rect.x + 10.0, input_rect.y + 28.0, 25.0, YELLOW);
    }
}

pub fn handle_starter(game: &mut Game){
    

    let mouse = mouse_position();
	let menu_name: &str = "The answer protocol";

	let title_size = measure_text(menu_name, None, 70, 1.0);
	let title_pos = Vec2::new(
            	(screen_width() - title_size.width) / 2.0,
            	(screen_height() + title_size.height) / 10.0
        	);
    draw_text(menu_name, title_pos.x, title_pos.y+ 60.0, 70.0, WHITE);


    let start_x = title_pos.x;
    let current_y = title_pos.y + 110.0;

    draw_name_input(start_x, current_y + 150.0, game, mouse);


    let state_selector = draw_skin_selection(start_x, current_y + 50.0, game, mouse);
    let nb_skins: i32 = game.skins.len() as i32;
    if state_selector != 0 {
        game.player.spritesheet_index = ((game.player.spritesheet_index as i32 + state_selector + nb_skins) % nb_skins) as usize;
    }

    let btn_valid = Rect::new(start_x, current_y + 250.0, 80.0, 40.0);
    let valid_hovered = btn_valid.contains(Vec2::new(mouse.0, mouse.1));
    let bg_valid = if valid_hovered { Color::new(0.3, 0.3, 0.3, 1.0) } else { Color::new(0.15, 0.15, 0.15, 1.0) };
    draw_rectangle(btn_valid.x, btn_valid.y, btn_valid.w, btn_valid.h, bg_valid);
    draw_text("Continue", btn_valid.x+ 6.0, btn_valid.y + 18.0, 30.0, WHITE);

    if valid_hovered && is_mouse_button_pressed(MouseButton::Left){
        game.init_end = true;
    }

}

