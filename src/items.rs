

use macroquad::prelude::*;
use serde::Deserialize;


#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum ItemKind {
    Potion { healing: i32 },
    Weapon { damages: i32 },
    Armor { protection: i32 },
    Miscellaneous,
}

#[derive(Clone, PartialEq)]
pub struct Item {
    pub id: String,
	pub name: String,
    pub texture: Texture2D,
    pub price: i32,
	pub kind: ItemKind
}

pub async fn get_item_texture(item_id: &str) -> Texture2D {
    let path = match item_id {
        "item.ale" => "assets/items/ale.png",
        "item.pint" => "assets/items/pint.png",
        "item.peanuts" => "assets/items/peanut.png",
        "item.strawberry" => "assets/items/strawberry.png",
        "item.orange" => "assets/items/orange.png",
        "item.sword" => "assets/items/sword.png",
        "item.shield" => "assets/items/shield.png",
        "item.helmet" => "assets/items/helmet.png",
        "item.gold" => "assets/items/gold.png",
        _ => return Texture2D::empty(),
    };

    load_texture(path).await.unwrap()
}
