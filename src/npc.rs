use crate::*;
use macroquad::{prelude::*};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum NPCKind {
    Merchant,
    Citizen,
    Enemy {
        hp: u32,
        max_hp: u32,
        defeated: bool
    },
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
	pub texts: Vec<String>
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

	let rect: Rect = Rect::new(place.x + 18.0, place.y, 40.0, 38.0);
	draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

	set_default_camera();

	if let Some(npc_talk) = npc.npc_talk.clone() {
		let talk_pos = world_to_screen_pos(vec2(place.x, place.y - 20.0));
		draw_rectangle(talk_pos.x, talk_pos.y, 200.0,30.0, WHITE);
		draw_text(npc_talk.texts[npc_talk.text_i].clone(), talk_pos.x, talk_pos.y + 20.0, 25.0, BLACK);
	}


	let screen_pos = world_to_screen_pos(vec2(rect.x, rect.y));
	let npc_info = format!("{}", npc.name);
	draw_text(npc_info, screen_pos.x, screen_pos.y + 20.0, 25.0, WHITE);

	let mouse = mouse_position();
	let btn_talk: Rect = Rect::new(screen_pos.x, screen_pos.y + 30.0, 100.0, 25.0);

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
		let btn_quest: Rect = Rect::new(screen_pos.x, screen_pos.y + 60.0, 100.0, 25.0);
		if get_button(btn_quest, "Quest", 25, WHITE, mouse){
			let rq: String = format!("QUEST {}\n",npc.id);
			game.tx_to_serv.try_send(rq).ok();
			game.pending_action = PendingAction::Quest(npc.id.clone());
		}
	}
	let btn_attack: Rect = Rect::new(screen_pos.x, screen_pos.y + 90.0, 100.0, 25.0);
	if get_button(btn_attack, "Attack", 25, WHITE, mouse){
		let rq: String = format!("ATTACK {}\n",npc.id);
		game.tx_to_serv.try_send(rq).ok();
		game.pending_action = PendingAction::Attack(npc.id.clone());

	}
	camera_handler(game);

}


pub async fn get_npc_texture(item_id: &str) -> Texture2D {
    let path = match item_id {
        "npc.city_gard" => "assets/npc/city_gard.png",
		"npc.goblins" => "assets/npc/goblin.png",
        _ => return Texture2D::empty(),
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
