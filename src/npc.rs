use std::collections::HashMap;

use macroquad::prelude::*;


#[derive(Clone, PartialEq)]
pub struct Npc {
    pub texture: Texture2D,
}


// pub async fn get_npc() -> HashMap<String, Npc> {
//     let mut npcs = HashMap::new();
// 	let guard = Npc { texture: ()};
//     npcs.insert("npc.city_gard", guard);
// }

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
