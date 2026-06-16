use std::collections::HashMap;

use macroquad::prelude::*;


#[derive(Clone, PartialEq, Debug)]
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


pub async fn get_items() -> HashMap<String, Item> {
    let mut items: HashMap<String, Item> = HashMap::new();

	let beer = Item {
		id: "ale".to_string(),
		name: "Amber beer".to_string(),
 		texture: load_texture("assets/items/ale.png").await.unwrap(),
     	price: 10,
		kind: ItemKind::Potion { healing: 25 },
	};
	items.insert(beer.id.clone(), beer);

	let pint = Item {
        id: "pint".to_string(),
        name: "Amber pint".to_string(),
        texture: load_texture("assets/items/pint.png").await.unwrap(),
        price: 20,
        kind: ItemKind::Potion { healing: 55 },
    };
    items.insert(pint.id.clone(), pint);


    let peanuts = Item {
        id: "peanuts".to_string(),
        name: "Bag of peanuts".to_string(),
        texture: load_texture("assets/items/peanut.png").await.unwrap(),
        price: 5,
        kind: ItemKind::Potion { healing: 10 },
    };
    items.insert(peanuts.id.clone(), peanuts);


    let strawberry = Item {
        id: "strawberry".to_string(),
        name: "Strawberries".to_string(),
        texture: load_texture("assets/items/strawberry.png").await.unwrap(),
        price: 5,
        kind: ItemKind::Potion { healing: 10 },
    };
    items.insert(strawberry.id.clone(), strawberry);


    let orange = Item {
        id: "orange".to_string(),
        name: "Orange".to_string(),
        texture: load_texture("assets/items/orange.png").await.unwrap(),
        price: 5,
        kind: ItemKind::Potion { healing: 10 },
    };
    items.insert(orange.id.clone(), orange);


    let sword = Item {
        id: "sword".to_string(),
        name: "Excalibur".to_string(),
        texture: load_texture("assets/items/sword.png").await.unwrap(),
        price: 25,
        kind: ItemKind::Weapon { damages: 10 },
    };
    items.insert(sword.id.clone(), sword);


    let shield = Item {
        id: "shield".to_string(),
        name: "You_shall_not_pass".to_string(),
        texture: load_texture("assets/items/shield.png").await.unwrap(),
        price: 25,
        kind: ItemKind::Armor { protection: 20 },
    };
    items.insert(shield.id.clone(), shield);


    let helmet = Item {
        id: "helmet".to_string(),
        name: "Helmetique".to_string(),
        texture: load_texture("assets/items/helmet.png").await.unwrap(),
        price: 15,
        kind: ItemKind::Armor { protection: 10 },
    };
    items.insert(helmet.id.clone(), helmet);

    let gold = Item {
        id: "gold".to_string(),
        name: "Gold".to_string(),
        texture: load_texture("assets/items/gold.png").await.unwrap(),
        price: 1,
        kind: ItemKind::Miscellaneous,
    };
    items.insert(gold.id.clone(), gold);

	return items;
}
