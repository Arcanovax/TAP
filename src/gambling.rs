

use crate::*;

const GAME_SIZE: Vec2 = vec2(100.0, 125.0);
pub struct Games {
	pub dice: Dice,
	pub slot: Slot
}

pub struct Dice {
	pub is_active: bool,
}

pub struct Slot {
	pub result: String,
	pub color: Color
}


impl Games {
    pub fn new() -> Self {
        Self {
			dice: Dice { is_active: false },
			slot: Slot {  result: "-----".to_string(), color: WHITE}
        }
    }
}

fn get_games_rect(pos_x: f32, pos_y:f32) -> Rect {
    return Rect::new(
        pos_x - (GAME_SIZE.x / 2.0),
        pos_y - (GAME_SIZE.y / 2.0),
        GAME_SIZE.x,
        GAME_SIZE.y,
    )
}

fn draw_slotmachine(game: &mut Game){
	let rect = get_games_rect(180.0, 275.0);
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.85));
	draw_text_center_top(rect, "Slot Machine", 17, 12.5);
	let start_button = get_rect_centered_x(rect, vec2(75.0, 35.0), 25.0);
	if get_button(start_button, "Start", 20, WHITE, game.mouse){
		game.tx_to_serv.try_send("SLOT_MACHINE \n".to_string()).ok();
		game.pending_action = PendingAction::SlotMachine;
	}
	draw_text_center_top_c(rect, &game.gambling.slot.result.to_string(), 30, 100.0,game.gambling.slot.color);
}


fn draw_dice(game: &mut Game){
	let rect = get_games_rect(305.0, 450.0);
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.85));
	draw_text_center_top(rect, "Dice", 17, 12.5);
	let start_button = get_rect_centered_x(rect, vec2(75.0, 35.0), 25.0);
	if get_button(start_button, "Start", 20, WHITE, game.mouse){
		game.tx_to_serv.try_send("DICES 10\n".to_string()).ok();
		game.pending_action = PendingAction::Dices;
	}
	draw_text_center_top_c(rect, &game.gambling.slot.result.to_string(), 30, 100.0,game.gambling.slot.color);
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

		let dice_place = vec2(85.0, 155.0);
		let range = 25.0;
		let is_next: bool = (game.player.x - dice_place.x).abs() <= range 
						&& (game.player.y - dice_place.y).abs() <= range;
		if map.room.id == "room.game_room" && is_next{
			draw_dice(game);
		}
	}
}


