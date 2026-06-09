use macroquad::prelude::*;
use crate::Game;

const INV_SIZE: Vec2 = vec2(400.0, 400.0);

pub struct Inventory {
    pub data: [[&'static str; 4]; 3],
	pub equiped: Equiped,
	pub is_active: bool
}


pub struct Equiped {
	chestplate:String,
	helmet:String,
	weapon:String,
	shield:String
}

fn get_inv_rect() -> Rect {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    return Rect::new(
        center_x - (INV_SIZE.x / 2.0),
        center_y - (INV_SIZE.y / 2.0),
        INV_SIZE.x,
        INV_SIZE.y,
    )
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            data: [[""; 4]; 3],
            equiped: Equiped{
				chestplate:"basic".to_string(),
				helmet:"basic".to_string(),
				weapon:"basic".to_string(),
				shield:"basic".to_string()
			},
            is_active: false,
        }
    }
}


// fn draw_chat_selection(x: f32, y: f32,selected: i32, mouse: (f32, f32)) -> i32 {
//     let mouse_pos = Vec2::new(mouse.0, mouse.1);
// 	let mut selected_channel = selected;

//     for i in 0...len(){
//             let btn = Rect::new(x+(i as f32)*100.0, y, 100.0, 30.0);
//             let hovered = btn.contains(mouse_pos);
//             let bg = if i as i32 == selected { Color::new(0.3, 0.3, 0.3, 0.75) }
// 			else { Color::new(0.10, 0.10, 0.10, 0.75) };
//             draw_rectangle(btn.x, btn.y, btn.w, btn.h, bg);
//             draw_text(CHANNELS[i], btn.x+ 6.0, btn.y + 22.5, 30.0, WHITE);
// 			if hovered && is_mouse_button_pressed(MouseButton::Left) {
// 				selected_channel = i as i32;
//         	}
//         }
//     return selected_channel;
// }


pub fn update_inv(game: &mut Game) {
	if is_key_pressed(KeyCode::E){
        if !game.player.inventory.is_active {
            game.player.inventory.is_active = true;
        }
		else
        {
			game.player.inventory.is_active = false;
        }
    }
	return;
}

pub fn draw_inv(game: &mut Game) {
    let mouse = mouse_position();


    if game.player.inventory.is_active {

		let rect: Rect = get_inv_rect();
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));


    }
}
