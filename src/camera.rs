use crate::*;

pub fn camera_handler(camera: &mut Camera2D, tile_size: f32) {
	let map_w = 25.0 * tile_size;
	let map_h = 14.0 * tile_size;

	camera.target = vec2(map_w / 2.0, map_h / 2.0);

	let scale_x = screen_width() / map_w;
	let scale_y = screen_height() / map_h;
	let scale = scale_x.min(scale_y);
	camera.zoom = vec2(scale * 2.0 / screen_width(), scale * 2.0 / screen_height());

	set_camera(camera);
}

pub fn world_to_screen_pos(world_pos: Vec2) -> Vec2 {
    let map_w = 25.0 * 16.0;
    let map_h = 14.0 * 16.0;

    let scale_x = screen_width() / map_w;
    let scale_y = screen_height() / map_h;
    let scale = scale_x.min(scale_y);

    let rendered_w = map_w * scale;
    let rendered_h = map_h * scale;
    let offset_x = (screen_width() - rendered_w) / 2.0;
    let offset_y = (screen_height() - rendered_h) / 2.0;

    return vec2(
        offset_x + world_pos.x * scale,
        offset_y + world_pos.y * scale,
    )
}
