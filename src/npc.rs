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



pub async fn get_npc_texture(item_id: &str) -> Texture2D {
    let path = match item_id {
        "npc.city_gard" => "assets/npc/city_gard.png",
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
