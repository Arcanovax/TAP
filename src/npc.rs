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


// #[derive(Clone, PartialEq, Debug, Deserialize)]
// pub enum NPCKind {
// 	Merchant {
// 		inventory: Vec<String>,
// 		gold: u32
// 	},
// 	Enemy {
// 		hp: u32,
// 		max_hp: u32,
// 		damages: u32,
// 		loot: Vec<String>,
// 		defeated: bool
// 	},
// 	Citizen
// }

#[derive(Clone, PartialEq, Debug)]
pub struct Npc {
    pub id: String,
    pub texture: Texture2D,
	pub name: String,
	pub kind: NPCKind,
	pub has_quest: bool,
	pub npc_talk: Option<NpcTalk>

}

#[derive(Clone, PartialEq, Debug)]
pub struct NpcTalk{
	pub text_i: usize,
	pub texts: String
}


pub struct NpcShop {
	pub is_active: bool,
}

impl Npc {
	pub fn new(id:String, texture: Texture2D, name: String, kind: NPCKind, has_quest: bool) -> Self {
		Self {
			id: id,
			texture: texture,
			name: name,
			kind: kind,
			has_quest: has_quest,
			npc_talk: None
		}
	}
}


pub fn handle_npc_interactions(game: &mut Game, place: Vec2, npc: Npc){
	draw_flat_triangle(place.x + 8.0, place.y);
	let w_pos: Vec2 = vec2(place.x, place.y);
	let s_pos = world_to_screen_pos(w_pos);

	set_default_camera();

	if let Some(npc_talk) = npc.npc_talk.clone() {
		let talk_pos = vec2(s_pos.x, s_pos.y - 20.0);
		draw_rectangle(talk_pos.x, talk_pos.y, 200.0,30.0, WHITE);
		draw_text(npc_talk.texts.clone(), talk_pos.x, talk_pos.y + 20.0, 25.0, BLACK);
	}

	let rect = Rect::new(s_pos.x + 62.5, s_pos.y, 175.0, 125.0);
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

	draw_text_center_top(rect, npc.name.as_str(), 30, 22.0);

	let mouse = mouse_position();

	let btn_talk:Rect = get_rect_center_x(rect, vec2(125.0, 25.0), 30.0);
	if get_button(btn_talk, "Talk", 25, WHITE, mouse){
		if let Some(npc) = game.loaded_npcs.get_mut(&npc.id) {
			if let Some(ref mut npc_talk) = npc.npc_talk {
				npc_talk.text_i = (npc_talk.text_i + 1) % npc_talk.texts.len();
			}
			else {
				let rq: String = format!("TALK {}\n",npc.id);
				game.tx_to_serv.try_send(rq).ok();
				game.pending_action = PendingAction::Talk(npc.id.clone());
			}
		}
	}
	if npc.has_quest{
		let btn_quest = get_rect_center_x(rect, vec2(125.0, 25.0), 60.0);
		if get_button(btn_quest, "Quest", 25, WHITE, mouse){
			let rq: String = format!("QUEST {}\n",npc.id);
			game.tx_to_serv.try_send(rq).ok();
			game.pending_action = PendingAction::Quest(npc.id.clone());
		}
	}
	let btn_attack: Rect = get_rect_center_x(rect, vec2(125.0, 25.0), 90.0);
	if get_button(btn_attack, "Attack", 25, WHITE, mouse){
		let rq: String = format!("ATTACK {}\n",npc.id);
		game.tx_to_serv.try_send(rq).ok();
		game.pending_action = PendingAction::Attack(npc.id.clone());

	}
	if let NPCKind::Merchant { inventory, gold } = &npc.kind {
		let btn_buy = get_rect_center_x(rect, vec2(125.0, 25.0), 120.0);
		if get_button(btn_buy, "Shop", 25, WHITE, mouse){
			game.npc_shop.is_active = !game.npc_shop.is_active
		}
		if game.npc_shop.is_active {
			let shop_rect = get_center_rect(vec2(400.0, 250.0));
			draw_rectangle(shop_rect.x,shop_rect.y,shop_rect.w,shop_rect.h,Color::new(0.0, 0.0, 0.0, 1.0),);
			let item_size = 40.0;

			for (i, item) in inventory.iter().enumerate() {
				let line = shop_rect.y + i as f32 * 50.0;

				let slot = Rect::new(
					shop_rect.x + 10.0,
					line + (50.0 - item_size) / 2.0,
					item_size,
					item_size,
				);
				draw_item_center(slot, &game.loaded_items[item]);
				draw_text(
					&game.loaded_items[item].name,
					slot.x + slot.w + 15.0,
					slot.y + item_size * 0.75,
					30.0,
					WHITE,
				);
			}
		}
	}

	camera_handler(game);

}


pub async fn get_npc_texture(item_id: &str) -> Texture2D {
    let path = match item_id {
        "npc.city_gard" => "assets/npc/city_gard.png",
		"npc.goblins" => "assets/npc/goblin.png",
		"npc.h" => "assets/npc/goblin.png",
        _ => "assets/npc/black-smith.png",
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
