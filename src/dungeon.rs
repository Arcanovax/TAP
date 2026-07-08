use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Dungeon {
	pub players: HashMap<String, i32>,
	pub not_owner: bool,
	pub rooms: HashMap <String,RoomData>
}

impl Dungeon {
	pub fn new() -> Self {
		Self {
			players:HashMap::new(),
			not_owner: false,
			rooms: HashMap::new()
			}
	}
}

pub async fn get_dungeon_map(data: &RoomData) -> Room {
	Room { id: data.id.clone(), colliders:
		[
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],

			], first_layer: load_texture("assets/map/mine/layer1.png").await.unwrap(), second_layer: None, spawns: HashMap::from([
				(Spawn::Center, vec2(200.0, 130.0))]) }
}

use macroquad::prelude::*;

fn merge_textures(tex_a: &Texture2D, tex_b: &Texture2D, width: u32, height: u32) -> Texture2D {
    let render_target = render_target(width, height);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut cam = Camera2D::from_display_rect(
        Rect::new(0., 0., width as f32, height as f32)
    );
    cam.render_target = Some(render_target.clone());

    set_camera(&cam);
    clear_background(BLANK);
    draw_texture(tex_a, 0., 0., WHITE);
    draw_texture(tex_b, 0., 0., WHITE);
    set_default_camera();


    render_target.texture.clone()
}
