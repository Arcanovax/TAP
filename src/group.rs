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
	pub in_group: bool,
	pub typed: String,
	pub chat_is_active: bool
}

impl Group {
    pub fn new() -> Self {
        Self {
            is_active: false,
			in_group: false,
			typed: String::new(),
			chat_is_active: false
			}
	}
}


fn input_text(x: f32,y: f32,game: &mut Game, mouse: (f32, f32)) -> bool{
	let field: &mut String = &mut game.group.typed;
    let input_rect = Rect::new(x, y, 175.0, 40.0);
    let input_hovered = input_rect.contains(Vec2::new(mouse.0, mouse.1));

    let input_bg = if input_hovered { Color::new(0.2, 0.2, 0.2, 1.0) } else { Color::new(0.1, 0.1, 0.1, 1.0) };
	if input_hovered && is_mouse_button_pressed(MouseButton::Left)  {
		game.focus = InputFocus::GroupMenu;
		game.group.chat_is_active = true;
	}
	else if !input_hovered {
		game.group.chat_is_active = false;
	}
    draw_rectangle(input_rect.x, input_rect.y, input_rect.w, input_rect.h, input_bg);
    draw_rectangle_lines(input_rect.x, input_rect.y, input_rect.w, input_rect.h, 2.0, GRAY);

    if is_key_pressed(KeyCode::Backspace) {
        field.pop();
    }

    while let Some(character) = get_char_pressed() {
		println!("{}", character.clone());
        if character.is_ascii_graphic() || character == ' ' {
            if field.len() < 12 {
                field.push(character);
            }
        }
    }

    let mut display_name = field.clone();
    if get_time() % 1.0 < 0.5 {
        display_name.push('_');
    }

    if field.is_empty() && !input_hovered {
        draw_text("Type Name...", input_rect.x + 10.0, input_rect.y + 28.0, 25.0, DARKGRAY);
    } else {
        draw_text(&display_name, input_rect.x + 10.0, input_rect.y + 28.0, 25.0, YELLOW);
    }
	return false;
}

pub fn draw_group(game: &mut Game){
	let mouse = mouse_position();
	let rect: Rect = get_rect_right(RECT_MENU, screen_height());
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));



	if !game.group.in_group {
		let btn: Rect = Rect::new(rect.x,rect.y, 100.0, 20.0);
		let hovered = btn.contains(Vec2::new(mouse.0, mouse.1));
		let bg = if hovered { Color::new(0.3, 0.3, 0.3, 0.75) }
		else { Color::new(0.10, 0.10, 0.10, 0.75) };
		draw_rectangle(btn.x, btn.y, btn.w, btn.h, bg);
		draw_text("Create", btn.x+ 6.0, btn.y + 22.5, 30.0, WHITE);
		if hovered && is_mouse_button_pressed(MouseButton::Left) {
            game.tx_to_serv.try_send("GROUP JOIN test".to_string()).ok();
			game.group.in_group = true;
		}

		if input_text(rect.x, rect.y + 60.0,game, mouse){
			game.group.in_group = true;
		}
	}
	else {
		let text: String = format!("{}'s Group", game.player.name);
		let text_dimensions = measure_text(&text, None,30, 1.0);
		let text_x = rect.x + (rect.w - text_dimensions.width) / 2.0;

		draw_text(&text, text_x, rect.y + 30.0, 30.0, WHITE);
	}
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
	if !game.group.is_active {
		draw_icon(game, mouse);
		if is_key_pressed(KeyCode::F) && game.focus == InputFocus::Game {
            game.group.is_active = true;
        }
    }
	else{

		if is_key_pressed(KeyCode::F) && game.group.chat_is_active == false{
			game.group.is_active = false;
			game.focus = InputFocus::Game;
		}
		draw_group(game);
	}
}
