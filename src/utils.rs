use macroquad::prelude::*;

pub fn get_rect_centered_x(container: Rect, size: Vec2, y:f32) -> Rect {
		let pos_x: f32 = container.x + (container.w - size.x) / 2.0;
		let pos_y: f32 = container.y + y;

		return Rect::new(
			pos_x,
			pos_y,
			size.x,
			size.y,
		)
}

pub fn get_center_rect(size: Vec2) -> Rect {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    Rect::new(
        center_x - (size.x / 2.0),
        center_y - (size.y / 2.0),
        size.x,
        size.y,
    )
}

pub fn get_center_rect_x(size: Vec2, y:f32) -> Rect {
    let center_x = screen_width() / 2.0;

    Rect::new(
        center_x - (size.x / 2.0),
        y,
        size.x,
        size.y,
    )
}

pub fn get_rect_bottom(size: Vec2, x: f32) -> Rect {
		let max_y = screen_height();
		let pos_x: f32 = if x < screen_width()/2.0 {x} else {x - size.x};

		return Rect::new(
			pos_x,
			max_y - size.y,
			size.x,
			size.y,
		)
	}

pub fn get_rect_right(size: Vec2, y: f32) -> Rect {
		let max_x: f32 = screen_width();
		let pos_y: f32 = if y < screen_height()/2.0 {y} else {y - size.y};
		return Rect::new(
			max_x- size.x,
			pos_y,
			size.x,
			size.y,
		)
	}

pub fn draw_text_center(rect: Rect,text: &str, font_size: u16){
	let text_size = measure_text(text, None, font_size, 1.0);
	let text_pos = Vec2::new(
            	rect.x + (rect.w - text_size.width) / 2.0,
            	rect.y + (rect.h + text_size.height) / 2.0 - font_size as f32 * 0.1
        	);
	draw_text(text, text_pos.x, text_pos.y, font_size as f32, WHITE);
}

pub fn draw_text_center_top(rect: Rect,text: &str, font_size: u16, y: f32){
	let text_size = measure_text(text, None, font_size, 1.0);
	let text_pos = Vec2::new(
            	rect.x + (rect.w - text_size.width) / 2.0,
            	rect.y + y
        	);
	draw_text(text, text_pos.x, text_pos.y, font_size as f32, WHITE);
}

pub fn draw_text_center_top_c(rect: Rect,text: &str, font_size: u16, y: f32, color: Color){
	let text_size = measure_text(text, None, font_size, 1.0);
	let text_pos = Vec2::new(
            	rect.x + (rect.w - text_size.width) / 2.0,
            	rect.y + y
        	);
	draw_text(text, text_pos.x, text_pos.y, font_size as f32, color);
}

pub fn draw_text_bottom(rect: Rect,text: &str, font_size: u16, x: f32){
	let text_size = measure_text(text, None, font_size, 1.0);
	let text_pos = Vec2::new(
            	rect.x + x,
            	rect.y + rect.h - text_size.height* 0.2,
        	);
	draw_text(text, text_pos.x, text_pos.y, font_size as f32, WHITE);
}


pub fn get_button(rect: Rect,text: &str, font_size: u16, color: Color, mouse: Vec2) -> bool {
		let button: Rect = rect;
		let hovered = button.contains(mouse);
		let bg = if hovered { Color::new(0.3, 0.3, 0.3, 0.75) }
		else { Color::new(0.10, 0.10, 0.10, 0.75) };
		let text_size = measure_text(text, None, font_size, 1.0);
		let text_pos = Vec2::new(
            	button.x + (button.w - text_size.width) / 2.0,
            	button.y + (button.h + text_size.height) / 2.0  - font_size as f32 * 0.1
        	);
		draw_rectangle(button.x, button.y, button.w, button.h, bg);
		draw_text(text, text_pos.x, text_pos.y, font_size as f32, color);
		if hovered && is_mouse_button_pressed(MouseButton::Left) {
			return true;
		}
		return false;
}


pub fn input_text(input_rect: Rect,field: &mut String,is_active: bool, mouse: Vec2) -> bool{
		let input_hovered = input_rect.contains(mouse);
		let input_bg = if input_hovered { Color::new(0.2, 0.2, 0.2, 1.0) } else { Color::new(0.1, 0.1, 0.1, 1.0) };

		draw_rectangle(input_rect.x, input_rect.y, input_rect.w, input_rect.h, input_bg);
		draw_rectangle_lines(input_rect.x, input_rect.y, input_rect.w, input_rect.h, 2.0, GRAY);

		if is_key_pressed(KeyCode::Backspace) {
			field.pop();
		}

		while let Some(character) = get_char_pressed() {
			if character.is_ascii_graphic() || character == ' ' {
				field.push(character);
			}
		}

		let mut display_name = field.clone();
		if get_time() % 1.0 < 0.5 {
			display_name.push('_');
		}
		if field.is_empty() && !is_active{
			draw_text("Type...", input_rect.x + 10.0, input_rect.y + 28.0, 25.0, DARKGRAY);
		} else {
			draw_text(&display_name, input_rect.x + 10.0, input_rect.y + 28.0, 25.0, YELLOW);
		}
		return input_hovered;
	}


pub fn draw_flat_triangle(x: f32, y: f32){
	let triangle_width = 5.0;
	let triangle_height = 3.0;
	let bottom_point = vec2(
		x,
		y
	);

	let top_left_point = vec2(
		bottom_point.x - (triangle_width / 2.0),
		bottom_point.y - triangle_height
	);

	let top_right_point = vec2(
		bottom_point.x + (triangle_width / 2.0),
		bottom_point.y - triangle_height
	);

	draw_triangle(top_left_point, top_right_point, bottom_point, WHITE);
	draw_triangle_lines(top_left_point, top_right_point, bottom_point, 0.5, BLACK);
}


