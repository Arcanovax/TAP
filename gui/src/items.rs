

use macroquad::prelude::*;
use serde::Deserialize;


#[derive(Clone, PartialEq, Debug, Deserialize)]
pub enum ItemKind {
    Potion { healing: i32 },
    Weapon { damages: i32 },
    Armor { protection: i32 },
    Miscellaneous,
	QuestItem,
	None
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
        "item.golden_token" => "assets/items/gold_token.png",
        "item.sacred_seashell" => "assets/items/sacred_seashell.png",
        "item.goblin_tooth" => "assets/items/goblin_tooth.png",
        "item.dagger" => "assets/items/dagger.png",
        "item.cheese" => "assets/items/cheese.png",
        "item.bread" => "assets/items/bread.png",
        "item.breastplate" => "assets/items/breastplate.png",
		"dead" => "assets/items/dead.png",
        _ => return Texture2D::empty(),
    };

    load_texture(path).await.unwrap()
}
