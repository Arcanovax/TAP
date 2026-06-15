use std::collections::HashMap;

use macroquad::prelude::*;


#[derive(Clone, PartialEq)]
pub struct Item {
    pub id: String,
	pub name: String,
    pub texture: Texture2D,
    pub price: i32,
	pub kind: String
}


pub async fn get_items() -> HashMap<String, Item> {
    let mut items: HashMap<String, Item> = HashMap::new();

	let farm = Item {
		id: "ale".to_string(),
		name: "Amber beer".to_string(),
 		texture: load_texture("assets/items/ale.png").await.unwrap(),
     	price: 10,
		kind: String::new()
		};
    items.insert(farm.id.clone(), farm);
	return items;
}
