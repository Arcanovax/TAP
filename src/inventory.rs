use macroquad::prelude::*;
use crate::*;

const INV_SIZE: Vec2 = vec2(400.0, 300.0);
const ITEM_FLOOR_SIZE: Vec2 = vec2(400.0, 200.0);
const SLOT_SIZE: f32 = 50.0;
const ITEM_SIZE: f32 = 45.0;
const ITEM_INFO: Vec2 = vec2(120.0, 100.0);

pub struct Inventory {
    pub data: HashMap<String, i32>,
	pub is_load: bool,
	pub is_active: bool,
    pub active_item_info: Option<(Rect, Item)>,
	pub scroll_pos: f32,
    pub is_dragging:bool,
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
            active_item_info: None,
			scroll_pos : 0.0,
			is_dragging: false
        }
    }
}




pub fn draw_item_info(rect: Rect, item: &Item){
	let text_size = measure_text(item.name.clone(), None, 25, 1.0);
	let width = text_size.width.max(ITEM_INFO.x) + 10.0;
	let item_rect = Rect::new(rect.x + rect.w + 5.0, rect.y, width, ITEM_INFO.y);

	draw_rectangle(item_rect.x, item_rect.y, item_rect.w, item_rect.h, Color::new(0.0, 0.0, 0.0, 0.8));
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.1));
	draw_text_center_top(item_rect, item.name.as_ref(), 25, 25.0);

	let item_price = format!("Price: {}", item.price);

    let (type_name, characteristic) = match item.kind {
        ItemKind::Potion { healing } => ("Potion", Some(format!("Healing: {}", healing))),
        ItemKind::Weapon { damages } => ("Weapon", Some(format!("Damages: {}", damages))),
        ItemKind::Armor { protection } => ("Armor", Some(format!("Protection: {}", protection))),
        ItemKind::Miscellaneous => ("Miscellaneous", None),
    };
	let item_type = format!("Type: {}", type_name);

    draw_text(&item_price, item_rect.x + 5.0, item_rect.y + 50.0, 20.0, YELLOW);
    draw_text(&item_type, item_rect.x + 5.0, item_rect.y + 70.0, 20.0, YELLOW);
    if let Some(stat_text) = characteristic {
        draw_text(&stat_text, item_rect.x + 5.0, item_rect.y + 90.0, 20.0, YELLOW);
    }
}


pub fn get_item_slot_inv(inv_rect: Rect, slot_rect: Rect, game: &mut Game, item: &Item, amount: &i32) -> bool{

    draw_rectangle(slot_rect.x, slot_rect.y, slot_rect.w, slot_rect.h, GRAY);
    let hovered = slot_rect.contains(game.mouse);

	let inside = slot_rect.y >= inv_rect.y && slot_rect.y + SLOT_SIZE <= inv_rect.y + inv_rect.h;
	if hovered && inside{
        game.player.inventory.active_item_info = Some((slot_rect, item.clone()));
	}

    draw_item_center(slot_rect, &item);
    	if *amount > 1{
		let amount_str: &str = &format!("{}", amount).to_string();
		draw_text_bottom(slot_rect, amount_str, 30,0.0);
	}
	if hovered && is_mouse_button_pressed(MouseButton::Left) && game.pending_action == PendingAction::None{
		return true;
	}
	return false;
}


pub fn draw_item_center(rect: Rect, item: &Item){
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

pub fn get_slot_pos(rect: Rect, i: usize, columns: usize, offset: f32) -> Vec2 {
    let col = i % columns;
    let row = i / columns ;

    Vec2::new(
        rect.x + (col as f32) * SLOT_SIZE,
        rect.y + (row as f32 * SLOT_SIZE) - offset
    )
}

fn push_cut_rect(rect: Rect) {
    unsafe {
        let mut gl = get_internal_gl();
        gl.flush();
        let dpi = miniquad::window::dpi_scale();
        gl.quad_gl.scissor(Some((
            (rect.x * dpi) as i32,
            (rect.y * dpi) as i32,
            (rect.w * dpi) as i32,
            (rect.h * dpi) as i32,
        )));
    }
}

fn pop_cut_rect() {
    unsafe {
        let mut gl = get_internal_gl();
        gl.flush();
        gl.quad_gl.scissor(None);
    }
}


pub fn draw_inv(game: &mut Game) {
    if game.player.inventory.is_active {
		let Some(mapdata) = game.map_data.clone() else { return };
		let mut inv_rect: Rect = get_inv_rect();
        inv_rect.y -= 100.0;
        draw_rectangle(inv_rect.x, inv_rect.y, inv_rect.w, inv_rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

        let mut flr_item_rect: Rect = get_item_floor_rect();
        flr_item_rect.y += 175.0;
        draw_rectangle(flr_item_rect.x, flr_item_rect.y, flr_item_rect.w, flr_item_rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

		for (i, (item_id, amount)) in game.player.inventory.data.clone().iter().enumerate(){
            let item_pos = get_slot_pos(inv_rect, i, 8, 0.0);
			let item_rect = Rect::new(item_pos.x, item_pos.y, SLOT_SIZE, SLOT_SIZE);
			let item: Option<Item> = game.loaded_items.get(item_id).cloned();
			if let Some(item) = item {
				if get_item_slot_inv(inv_rect, item_rect, game, &item, amount){
					let rq: String = format!("DROP {}\n",item.id);
					game.tx_to_serv.try_send(rq).ok();
					game.pending_action = PendingAction::Drop;
				}
			}
		}

		let columns = 8;
        let total_h = (mapdata.items.len() as f32 / columns as f32).ceil() * SLOT_SIZE;
		let max_offset = (total_h - flr_item_rect.h).max(0.0);


		handle_sroll_bar(game, flr_item_rect, total_h, max_offset);
		let offset = game.player.inventory.scroll_pos * max_offset;

		push_cut_rect(flr_item_rect);
        for (i, item_id) in mapdata.items.iter().enumerate(){
			let item_pos = get_slot_pos(flr_item_rect, i, columns, offset);
			let item_rect = Rect::new(item_pos.x, item_pos.y , SLOT_SIZE, SLOT_SIZE);
			let item: Option<Item> = game.loaded_items.get(item_id).cloned();
			if let Some(item) = item {
				if get_item_slot_inv(flr_item_rect, item_rect, game, &item, &1){
					let rq: String = format!("TAKE {}\n",item.id);
					game.tx_to_serv.try_send(rq).ok();
					game.pending_action = PendingAction::Take;
				}
			}
		}
		pop_cut_rect();

	}
}

fn handle_sroll_bar(game: &mut Game, rect: Rect,total_h:f32, max_offset: f32){
	let bar: Rect = Rect::new(rect.x + rect.w, rect.y, 15.0, rect.h);
	let ratio = (rect.h / total_h).min(1.0);
	let handle_h = (bar.h * ratio).max(20.0);
	let mut handle_y = bar.y + game.player.inventory.scroll_pos * (bar.h - handle_h);
	let mouse_y = game.mouse.y;
	let (_, wheel_y) = mouse_wheel();

    if wheel_y != 0.0 && rect.contains(game.mouse) && max_offset > 0.0 {
		let scroll_px = 40.0;
		let delta = wheel_y.signum() * scroll_px / max_offset;
		game.player.inventory.scroll_pos =
		(game.player.inventory.scroll_pos - delta).clamp(0.0, 1.0);
	}

	if is_mouse_button_pressed(MouseButton::Left) && bar.contains(game.mouse){
        game.player.inventory.is_dragging = true;
    }

	if is_mouse_button_released(MouseButton::Left) {
        game.player.inventory.is_dragging = false;
    }

	if game.player.inventory.is_dragging {
        let new_y = mouse_y - handle_h / 2.0;
		game.player.inventory.scroll_pos = ((new_y - bar.y) / (bar.h - handle_h)).clamp(0.0, 1.0);
    }

	handle_y = bar.y + game.player.inventory.scroll_pos * (bar.h - handle_h);

	let handle_color = if game.player.inventory.is_dragging { WHITE } else { LIGHTGRAY };
	if handle_h != bar.h{
		draw_rectangle(bar.x, bar.y, bar.w, bar.h, GRAY);
		draw_rectangle(bar.x, handle_y, bar.w, handle_h, handle_color);
	}
}

pub fn handle_inv(game: &mut Game) {
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
