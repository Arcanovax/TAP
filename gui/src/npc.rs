use crate::*;
use macroquad::{prelude::*};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum NPCKind {
	Merchant {
		inventory: Vec<String>,
		gold: Option<u32>
	},
	Enemy {
		hp: u32,
		max_hp: u32,
		damages: Option<u32>,
		loot: Option<Vec<String>>,
		defeated: bool
	},
	Citizen
}


#[derive(Clone, PartialEq, Debug)]
pub struct Npc {
    pub id: String,
    pub texture: Texture2D,
	pub name: String,
	pub kind: NPCKind,
	pub has_quest: bool,
	pub npc_talk: Option<NpcTalk>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct InfoShop{
	pub time: f64,
	pub color: Color,
}


#[derive(Clone, PartialEq, Debug)]
pub struct NpcTalk{
	pub text_i: usize,
	pub texts: Vec<String>,
}


pub struct NpcShop {
	pub is_active: bool,
	pub buy_info: Option<InfoShop>,
	pub sell_info: Option<InfoShop>
}

impl Npc {
	pub fn new(id:String, texture: Texture2D, name: String, kind: NPCKind, has_quest: bool) -> Self {
		Self {
			id: id,
			texture: texture,
			name: name,
			kind: kind,
			has_quest: has_quest,
			npc_talk: None,
		}
	}
}


pub fn handle_npc_interactions(game: &mut Game, place: Vec2, npc: Npc){
	draw_flat_triangle(place.x + 8.0, place.y);
	let w_pos: Vec2 = vec2(place.x, place.y);
	let s_pos = world_to_screen_pos(w_pos);

	set_default_camera();

	if let Some(npc_talk) = npc.npc_talk.clone() {
		let text = npc_talk.texts[npc_talk.text_i % npc_talk.texts.len()].clone();
		let font_size = 22.5;
		let max_width = 200.0;
		let line_height = font_size * 1.15;
		let mut lines: Vec<String> = Vec::new();
		let mut current = String::new();
		for word in text.split_whitespace() {
			if current.is_empty(){
				current = format!("{} ",word.to_string())
			}
			else {
				current.push_str(&format!("{} ",word).to_string());
			};
			let w = measure_text(&current, None, font_size as u16, 1.0).width;
			if w > max_width{
				lines.push(current);
				current = String::new();
			}
		}
		if !current.is_empty() {
			lines.push(current);
		}

		let box_width = lines
			.iter()
			.map(|l| measure_text(l, None, font_size as u16, 1.0).width)
			.fold(0.0_f32, f32::max);
		let box_height = lines.len() as f32 * line_height + 10.0;

		let talk_rect = Rect::new(s_pos.x, s_pos.y - 15.0 - box_height, box_width, box_height);

		draw_rectangle(talk_rect.x , talk_rect.y, talk_rect.w, talk_rect.h, WHITE);
		draw_rectangle_lines(talk_rect.x , talk_rect.y, talk_rect.w, talk_rect.h,2.5, BLACK);
		for (i, line) in lines.iter().enumerate() {
			let pos_y = talk_rect.y  + font_size + line_height * i as f32;
			draw_text(line, talk_rect.x + 5.0, pos_y, font_size, BLACK);
		}
	}


	let mouse = game.mouse;
	let mut rect = Rect::new(s_pos.x + 62.5, s_pos.y, 175.0, 30.0);
	let mut n_slots = 2;
	if npc.has_quest {
		n_slots += 1;
	}
	if matches!(npc.kind, NPCKind::Merchant { .. }) {
		n_slots += 1;
	}

	rect.h += n_slots as f32 * 30.0;
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

	draw_text_center_top(rect, npc.name.as_str(), 30, 22.0);
	let mut slot_y = 30.0;

	let btn_talk:Rect = get_rect_centered_x(rect, vec2(125.0, 25.0), slot_y);
	slot_y += 30.0;
	if get_button(btn_talk, "Talk", 25, WHITE, mouse){
		if let Some(npc) = game.loaded_npcs.get_mut(&npc.id) {
			let rq: String = format!("TALK {}\n",npc.id);
			game.tx_to_serv.try_send(rq).ok();
			game.pending_action = PendingAction::Talk(npc.id.clone());
		}

	}

	if npc.has_quest{
		let btn_quest = get_rect_centered_x(rect, vec2(125.0, 25.0), slot_y);
		slot_y += 30.0;
		if get_button(btn_quest, "Quest", 25, WHITE, mouse){
			let rq: String = format!("QUEST {}\n",npc.id);
			game.tx_to_serv.try_send(rq).ok();
			game.pending_action = PendingAction::Quest(npc.id.clone());
		}
	}
	let btn_attack: Rect = get_rect_centered_x(rect, vec2(125.0, 25.0), slot_y);
	slot_y += 30.0;
	if get_button(btn_attack, "Attack", 25, WHITE, mouse){
		let rq: String = format!("ATTACK {}\n",npc.id);
		game.tx_to_serv.try_send(rq).ok();
		game.pending_action = PendingAction::Attack(npc.id.clone());

	}
	if let NPCKind::Merchant { .. } = &npc.kind {
		let btn_shop = get_rect_centered_x(rect, vec2(125.0, 25.0), slot_y);
		if get_button(btn_shop, "Shop", 25, WHITE, game.mouse){
			game.npc_shop.is_active = !game.npc_shop.is_active
		}
		if game.npc_shop.is_active {
			handle_shop(game, &npc);
		}
}


fn handle_shop(game: &mut Game, npc: &Npc){
	if let NPCKind::Merchant { inventory, .. } = &npc.kind {
			let shop_rect = get_center_rect(vec2(400.0, 200.0));
			draw_rectangle(shop_rect.x,shop_rect.y,shop_rect.w,shop_rect.h,Color::new(0.0, 0.0, 0.0, 0.85));
			let item_size = 50.0;
			let columns = 1;
			let total_h = (inventory.len() as f32 / columns as f32).ceil() * 55.0 + 15.0;
			let max_offset = (total_h - shop_rect.h).max(0.0);

			draw_rectangle(shop_rect.x, shop_rect.y, shop_rect.w, shop_rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

			handle_sroll_bar(game.mouse, shop_rect, total_h, max_offset, &mut game.player.inventory.dropped_scroll);
			let offset = game.player.inventory.dropped_scroll.scroll_pos * max_offset;

			push_cut_rect(shop_rect);
			for (i, item) in inventory.iter().enumerate() {
				let line = shop_rect.y + 15.0 - offset + i  as f32 * 55.0;
				let slot = Rect::new(
					shop_rect.x + 10.0,
					line + (50.0 - item_size) / 2.0,
					item_size,
					item_size,
				);
				if let Some(item) = game.loaded_items.get(item).cloned() {
					get_item_slot_inv(shop_rect, slot, game, &item, &1);
				}
				draw_text(
					&game.loaded_items[item].name,
					slot.x + slot.w + 15.0,
					slot.y + item_size * 0.75,
					25.0,
					WHITE,
				);

				let btn_buy = Rect::new(shop_rect.x + shop_rect.w - 115.0,line + (50.0 - item_size) / 2.0, 50.0,50.0);
				if let Some(info) = game.npc_shop.buy_info.as_ref() {
					if get_time() - info.time > 0.5 {
						game.npc_shop.buy_info = None;
					}
				}
				let info = game.npc_shop.buy_info.as_ref().filter(|_| btn_buy.contains(game.mouse));
				let color = info.map(|info| info.color).unwrap_or(WHITE);
				let allowed = info.is_none();
				if get_button(btn_buy, "Buy", 25, color, game.mouse) && allowed {
					let rq = format!("BUY {} {} \n", npc.id, item);
					game.tx_to_serv.try_send(rq).ok();
					game.pending_action = PendingAction::Buy(item.to_string());
				}

				let btn_sell = Rect::new(shop_rect.x + shop_rect.w - 60.0, line + (50.0 - item_size) / 2.0, 50.0,50.0);
				if let Some(info) = game.npc_shop.sell_info.as_ref() {
					if get_time() - info.time > 0.5 {
						game.npc_shop.sell_info = None;
					}
				}
				let info = game.npc_shop.sell_info.as_ref().filter(|_| btn_sell.contains(game.mouse));
				let color = info.map(|info| info.color).unwrap_or(WHITE);
				let allowed = info.is_none();
				if get_button(btn_sell, "Sell", 25, color, game.mouse) && allowed {
					let rq = format!("SELL {} {} \n", npc.id, item);
					game.tx_to_serv.try_send(rq).ok();
					game.pending_action = PendingAction::Sell(item.to_string());
				}

			}
			pop_cut_rect();
		}
	}
}

pub async fn get_npc_texture(item_id: &str) -> Texture2D {
    let path = match item_id {
        "npc.city_gard" => "assets/npc/city_gard.png",
		"npc.goblins" => "assets/npc/goblin.png",
		"npc.h" => "assets/npc/goblin.png",
		"npc.blacksmith" => "assets/npc/black-smith.png",
		"npc.old_man" => "assets/npc/old_man.png",
		"npc.farmer" => "assets/npc/farmer.png",
		"npc.cow" => "assets/npc/cow.png",
		"npc.dealer" => "assets/npc/dealer.png",
		"npc.seller" => "assets/npc/seller.png",
		"npc.citizen_3" => "assets/npc/citizen_3.png",
		"npc.citizen_4" => "assets/npc/citizen_4.png",
		"npc.citizen_5" => "assets/npc/citizen_5.png",
		"npc.citizen_6" => "assets/npc/citizen_6.png",
		"npc.door" => "assets/npc/door.png",
		"npc.bookseller" => "assets/npc/bookseller.png",
		"npc.client_1" => "assets/npc/client_1.png",
		"npc.client_2" => "assets/npc/client_2.png",
		"npc.bar_owner" => "assets/npc/bar_owner.png",
		
        _ => "assets/npc/monster.png",
    };

    load_texture(path).await.unwrap()
}

pub fn find_npc_spawns(colliders: &[[i32; 25]; 15], tile_size: f32) -> Vec<Vec2> {
    let mut positions = Vec::new();

    for (row, line) in colliders.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == 8 {
                positions.push(vec2(
                    col as f32 * tile_size,
                    row as f32 * tile_size,
                ));
            }
        }
    }
    return positions;
}
