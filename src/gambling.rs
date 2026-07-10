use crate::*;

const SLOT_SIZE: Vec2 = vec2(100.0, 50.0);
pub struct Games {
	pub dice: Dice,
	pub slot: Slot
}

pub struct Dice {
	pub is_active: bool,
}

pub struct Slot {
	pub is_active: bool,
}


impl Games {
    pub fn new() -> Self {
        Self {
			dice: Dice { is_active: false },
			slot: Slot { is_active: false }
        }
    }
}

fn get_slot_rect() -> Rect {
    let pos_x = 300.0;
    let pos_y = 300.0;

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
}


pub fn handle_games(game: &mut Game){
	if game.gambling.slot.is_active {
		draw_slotmachine(game);
	}
}


