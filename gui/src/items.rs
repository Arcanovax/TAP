

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
        "item.ale" => "gui/assets/items/ale.png",
        "item.pint" => "gui/assets/items/pint.png",
        "item.peanuts" => "gui/assets/items/peanut.png",
        "item.strawberry" => "gui/assets/items/strawberry.png",
        "item.orange" => "gui/assets/items/orange.png",
        "item.sword" => "gui/assets/items/sword.png",
        "item.shield" => "gui/assets/items/shield.png",
        "item.helmet" => "gui/assets/items/helmet.png",
        "item.gold" => "gui/assets/items/gold.png",
        "item.golden_token" => "gui/assets/items/gold_token.png",
        "item.sacred_seashell" => "gui/assets/items/sacred_seashell.png",
        "item.goblin_tooth" => "gui/assets/items/goblin_tooth.png",
        "item.dagger" => "gui/assets/items/dagger.png",
        "item.cheese" => "gui/assets/items/cheese.png",
        "item.bread" => "gui/assets/items/bread.png",
        "item.breastplate" => "gui/assets/items/breastplate.png",
        _ => return Texture2D::empty(),
    };

    load_texture(path).await.unwrap()
}
