

use crate::*;

const SLOT_SIZE: Vec2 = vec2(100.0, 125.0);
pub struct Games {
	pub dice: Dice,
	pub slot: Slot
}

pub struct Dice {
	pub is_active: bool,
}

pub struct Slot {
	pub result: String
}


impl Games {
    pub fn new() -> Self {
        Self {
			dice: Dice { is_active: false },
			slot: Slot {  result: "-----".to_string()}
        }
    }
}

fn get_slot_rect() -> Rect {
    let pos_x = 180.0;
    let pos_y = 275.0;

    return Rect::new(
        pos_x - (SLOT_SIZE.x / 2.0),
        pos_y - (SLOT_SIZE.y / 2.0),
        SLOT_SIZE.x,
        SLOT_SIZE.y,
    )
}

fn draw_slotmachine(game: &mut Game){
	let rect = get_slot_rect();
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.85));
	draw_text_center_top(rect, "Slot Machine", 17, 12.5);
	let start_button = get_rect_centered_x(rect, vec2(75.0, 35.0), 25.0);
	if get_button(start_button, "Start", 20, WHITE, game.mouse){

	}
	draw_text_center_top(rect, &game.gambling.slot.result.to_string(), 25, 100.0);
}


pub fn handle_games(game: &mut Game){
	if let Some(map) = game.map_data.clone(){
		let slot_place = vec2(45.0, 95.0);
		let range = 20.0;
		let is_next: bool = (game.player.x - slot_place.x).abs() <= range 
						&& (game.player.y - slot_place.y).abs() <= range;
		
		if map.room.id == "room.game_room" && is_next{
			draw_slotmachine(game);
		}
	}
}


