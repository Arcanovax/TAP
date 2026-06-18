use crate::*;

#[derive(Deserialize, Debug)]
pub struct ItemData {
    pub kind: ItemKind,
    pub name: String,
	pub price: i32
}

#[derive(Deserialize, Debug)]
pub struct NpcData {
	pub name: String,
	pub kind: NPCKind,
	pub has_quest: bool
}


#[derive(Deserialize, Debug)]
pub struct MoveData {
	pub room: String,
}



#[derive(Deserialize, Debug, Clone)]
pub struct LookData {
    pub room: RoomData,
	pub players: Vec<String>,
	pub items: Vec<String>,
    pub npcs: Vec<String>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct RoomData {
    pub id: String,
    pub name: String,
	pub description: String,
    pub exits: Vec<Exit>,

}



#[derive(Deserialize, Debug, Clone)]
pub struct StatusData {
    pub hp: i32,
    pub max_hp: i32,
    pub status: String,
}
