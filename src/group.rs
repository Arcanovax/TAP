use crate::*;
const RECT_MENU: Vec2 = vec2(250.0, 300.0);
const RECT_ICON: Vec2 = vec2(40.0, 40.0);


fn get_rect_bottom(size: Vec2, x: f32) -> Rect {
    let max_y = screen_height();
	let pos_x: f32 = if x < screen_width()/2.0 {x} else {x - size.x};

    return Rect::new(
        pos_x,
        max_y - size.y,
        size.x,
        size.y,
    )
}

fn get_rect_right(size: Vec2, y: f32) -> Rect {
	let max_x: f32 = screen_width();
	let pos_y: f32 = if y < screen_height()/2.0 {y} else {y - size.y};
    return Rect::new(
        max_x- size.x,
        pos_y,
        size.x,
        size.y,
    )
}



pub struct Group {
	pub is_active: bool,
	pub is_created: bool,
}



pub fn update_group(game: &mut Game) {
	let rect: Rect = get_rect_right(RECT_MENU, screen_height());
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));

	let text: String = format!("{}'s Group", game.player.name);
	let text_dimensions = measure_text(&text, None,30, 1.0);
	let text_x = rect.x + (rect.w - text_dimensions.width) / 2.0;

	draw_text(text, text_x, rect.y + 30.0, 30.0, WHITE);
	return;
}

pub fn draw_icon(game: &mut Game, mouse: (f32, f32)){
	let mut icon: Rect = get_rect_bottom(RECT_ICON, screen_width());
	icon.x -= 10.0;
	icon.y -= 10.0;
    let hovered = icon.contains(Vec2::new(mouse.0, mouse.1));
    let bg = if hovered { Color::new(0.3, 0.3, 0.3, 0.75) }
	else { Color::new(0.10, 0.10, 0.10, 0.75) };
	draw_rectangle(icon.x, icon.y, icon.w, icon.h, bg);
    draw_text("GR", icon.x+ 6.0, icon.y + 22.5, 30.0, WHITE);
	if hovered && is_mouse_button_pressed(MouseButton::Left) {
		game.group.is_active = true
	}
}

pub fn handle_group(game: &mut Game) {
    let mouse = mouse_position();

	if is_key_pressed(KeyCode::F){
        if !game.group.is_active {
            game.group.is_active = true;
        }
		else
        {
			game.group.is_active = false;
        }
    }

	if !game.group.is_active {
		draw_icon(game, mouse);
    }


    if game.group.is_active {
		update_group(game)
    }
}
