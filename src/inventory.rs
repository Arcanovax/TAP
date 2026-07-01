use macroquad::prelude::*;
use crate::*;

const INV_SIZE: Vec2 = vec2(400.0, 300.0);
const ITEM_FLOOR_SIZE: Vec2 = vec2(400.0, 200.0);
const SLOT_SIZE: f32 = 50.0;
const ITEM_SIZE: f32 = 45.0;
const ITEM_INFO: Vec2 = vec2(100.0, 120.0);

pub struct Inventory {
    pub data: HashMap<String, i32>,
	pub is_load: bool,
	pub is_active: bool
}

use std::collections::HashMap;

pub fn count_items(items: Vec<String>) -> HashMap<String, i32> {
    let mut items_map = HashMap::new();

    for item in items {
        *items_map.entry(item.to_string()).or_insert(0) += 1;
    }

    return items_map
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

fn get_item_floor_rect() -> Rect {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    return Rect::new(
        center_x - (ITEM_FLOOR_SIZE.x / 2.0),
        center_y - (ITEM_FLOOR_SIZE.y / 2.0),
        ITEM_FLOOR_SIZE.x,
        ITEM_FLOOR_SIZE.y,
    )
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            is_load: false,
            is_active: false,
        }
    }
}

fn draw_item_info(rect: Rect, item: &Item){
	let text_size = measure_text(item.name.clone(), None, 25, 1.0);
	let width = text_size.width.max(ITEM_INFO.x) + 10.0;
	let item_rect = Rect::new(rect.x + rect.w, rect.y-width+SLOT_SIZE, width, ITEM_INFO.y);
	draw_rectangle(item_rect.x, item_rect.y, item_rect.w, item_rect.h, Color::new(0.0, 0.0, 0.0, 1.0));
	draw_text_center_top(item_rect, item.name.as_ref(), 25, 25.0);
	let item_price = format!("Price: {}", item.price);
	let item_type = format!("Type: {:?}", item.kind);
	draw_text(item_price, item_rect.x+ 5.0, item_rect.y + 50.0, 20.0, WHITE);
	draw_text(item_type, item_rect.x+ 5.0, item_rect.y + 70.0, 20.0, WHITE);
}

fn get_item_slot_floor(slot_rect: Rect, game: &mut Game, item: &Item, mouse: (f32, f32)){
    draw_rectangle(slot_rect.x, slot_rect.y, slot_rect.w, slot_rect.h, GRAY);
    let hovered = slot_rect.contains(Vec2::new(mouse.0, mouse.1));

	draw_item_center(slot_rect, item);

	if hovered{
		draw_item_info(slot_rect, item);
	}
	if hovered && is_mouse_button_pressed(MouseButton::Left) && game.pending_action == PendingAction::None{
		let rq: String = format!("TAKE {}\n",item.id);
		game.tx_to_serv.try_send(rq).ok();
		game.pending_action = PendingAction::Take;
	}
}


fn get_item_slot_inv(slot_rect: Rect, game: &mut Game, item: &Item, amount: &i32 , mouse: (f32, f32)){

    draw_rectangle(slot_rect.x, slot_rect.y, slot_rect.w, slot_rect.h, GRAY);
    let hovered = slot_rect.contains(Vec2::new(mouse.0, mouse.1));

	draw_item_center(slot_rect, &item);
	let amount_str: &str = &format!("{}", amount).to_string();
	draw_text_bottom(slot_rect, amount_str, 30,0.0);

	if hovered{
		draw_item_info(slot_rect, &item);
	}
	if hovered && is_mouse_button_pressed(MouseButton::Left) && game.pending_action == PendingAction::None{
		let rq: String = format!("DROP {}\n",item.id);
		game.tx_to_serv.try_send(rq).ok();
		game.pending_action = PendingAction::Drop;
	}
}


fn draw_item_center(rect: Rect, item: &Item){
	let texture_param = DrawTextureParams {
        dest_size: Some(vec2(ITEM_SIZE, ITEM_SIZE)),
        ..Default::default()
    };
	let texture_x = rect.x + (rect.w - ITEM_SIZE) / 2.0;
    let texture_y = rect.y + (rect.h - ITEM_SIZE) / 2.0;

	draw_texture_ex(
        &item.texture,
        texture_x,
        texture_y,
        WHITE,
        texture_param.clone()
    );
	item.texture.set_filter(FilterMode::Nearest);
}

pub fn update_inv(game: &mut Game) {
	if game.player.inventory.is_active {

	}
	return;
}

pub fn draw_inv(game: &mut Game) {
    let mouse: (f32, f32) = mouse_position();

    if game.player.inventory.is_active {

		let mut inv_rect: Rect = get_inv_rect();
        inv_rect.y -= 100.0;
        draw_rectangle(inv_rect.x, inv_rect.y, inv_rect.w, inv_rect.h, Color::new(0.0, 0.0, 0.0, 0.5));
		for (i, (item_id, amount)) in game.player.inventory.data.clone().iter().enumerate(){
			let item_rect = Rect::new(inv_rect.x, inv_rect.y+ SLOT_SIZE * (i as f32), SLOT_SIZE, SLOT_SIZE);
			let item: Option<Item> = game.loaded_items.get(item_id).cloned();
			if let Some(item) = item {
				get_item_slot_inv(item_rect, game, &item, amount, mouse);
			}

		}


        let mut flr_item_rect: Rect = get_item_floor_rect();
        flr_item_rect.y += 175.0;
        draw_rectangle(flr_item_rect.x, flr_item_rect.y, flr_item_rect.w, flr_item_rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

        if let Some(mapdata) = game.map_data.clone(){
            for (i, item_id) in mapdata.items.iter().enumerate(){
                let item_rect = Rect::new(flr_item_rect.x, flr_item_rect.y+ SLOT_SIZE * (i as f32), SLOT_SIZE, SLOT_SIZE);
				let item: Option<Item> = game.loaded_items.get(item_id).cloned();
				if let Some(item) = item {
                get_item_slot_floor(item_rect, game, &item, mouse);
			}
            }
        }
    }
}

pub fn handle_inv(game: &mut Game,) {
	draw_inv(game);
	update_inv(game);
	if is_key_pressed(KeyCode::E) && game.focus == InputFocus::Game{
        if !game.player.inventory.is_active {
            game.player.inventory.is_active = true;
        }
		else
        {
			game.player.inventory.is_active = false;
        }
    }

}
