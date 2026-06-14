use macroquad::prelude::*;
use crate::*;

const INV_SIZE: Vec2 = vec2(400.0, 300.0);
const ITEM_FLOOR_SIZE: Vec2 = vec2(400.0, 200.0);
const SLOT_SIZE: f32 = 50.0;
const ITEM_INFO: Vec2 = vec2(100.0, 150.0);

pub struct Inventory {
    pub data: HashMap<Item, i32>,
	pub equiped: Equiped,
	pub is_active: bool
}


pub struct Equiped {
	chestplate:String,
	helmet:String,
	weapon:String,
	shield:String
}

fn get_inv_rect() -> Rect {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    return Rect::new(
        center_x - (INV_SIZE.x / 2.0),
        center_y - (INV_SIZE.y / 2.0),
        INV_SIZE.x,
        INV_SIZE.y,
    )
}

fn get_item_floor_rect() -> Rect {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    return Rect::new(
        center_x - (ITEM_FLOOR_SIZE.x / 2.0),
        center_y - (ITEM_FLOOR_SIZE.y / 2.0),
        ITEM_FLOOR_SIZE.x,
        ITEM_FLOOR_SIZE.y,
    )
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            equiped: Equiped{
				chestplate:"basic".to_string(),
				helmet:"basic".to_string(),
				weapon:"basic".to_string(),
				shield:"basic".to_string()
			},
            is_active: false,
        }
    }
}

fn get_item_slot(rect: Rect, game: &mut Game, item_id: &String, mouse: (f32, f32)){
    let texture_param = DrawTextureParams {
        dest_size: Some(vec2(40.0, 40.0)),
        ..Default::default()
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
    let id: &str = item_id.strip_prefix("item.").unwrap();
    let hovered = rect.contains(Vec2::new(mouse.0, mouse.1));
    if let Some(item) = game.items.get(id) {
        draw_texture_ex(
            &item.texture,
            rect.x,
            rect.y,
            WHITE,
            texture_param.clone()
        );
        item.texture.set_filter(FilterMode::Nearest);        
    }
    if hovered{

    }
    
    
}


pub fn update_inv(game: &mut Game) {
	if game.player.inventory.is_active {

	}
	return;
}

pub fn draw_inv(game: &mut Game) {
    let mouse: (f32, f32) = mouse_position();

    if game.player.inventory.is_active {

		let mut inv_rect: Rect = get_inv_rect();
        inv_rect.y -= 100.0;
        draw_rectangle(inv_rect.x, inv_rect.y, inv_rect.w, inv_rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

        let mut flr_item_rect: Rect = get_item_floor_rect();
        flr_item_rect.y += 175.0;
        draw_rectangle(flr_item_rect.x, flr_item_rect.y, flr_item_rect.w, flr_item_rect.h, Color::new(0.0, 0.0, 0.0, 0.5));
        
        if let Some(mapdata) = game.map_data.clone(){
            for (i, item_id) in mapdata.items.iter().enumerate(){
                let item_rect = Rect::new(flr_item_rect.x, flr_item_rect.y+ 20.0 * (i as f32 + 1.0), SLOT_SIZE, SLOT_SIZE);
                get_item_slot(item_rect, game, item_id, mouse);
            }
        }
    }
}

pub fn handle_inv(game: &mut Game,) {
	draw_inv(game);
	update_inv(game);
	if is_key_pressed(KeyCode::E) && game.focus == InputFocus::Game{
        if !game.player.inventory.is_active {
            game.player.inventory.is_active = true;
        }
		else
        {
			game.player.inventory.is_active = false;
        }
    }

}
