use macroquad::prelude::*;

pub struct Chat {
    current_input: String,
    messages: Vec<String>,
    pub is_active: bool,
	prev: i32
}

impl Chat {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            messages: Vec::new(),
            is_active: false,
			prev: 0
        }
    }
}

pub fn update_chat(chat: &mut Chat, menu_active:bool) {
    if is_key_pressed(KeyCode::Enter) && !menu_active{
        if !chat.is_active {
            chat.is_active = true;
        }
		else {
			if !chat.current_input.trim().is_empty() {
				chat.messages.push(chat.current_input.clone());
				chat.current_input.clear();
				chat.prev = 0;
        	}
			chat.is_active = false;
	}
	}
	if is_key_pressed(KeyCode::Escape) {
		chat.is_active = false;
	}
    if !chat.is_active {
        while get_char_pressed().is_some() {
        }
    }
    if !chat.is_active{
        return;
    }

	if is_key_pressed(KeyCode::Up) && chat.prev < (chat.messages.len()as i32){
		if let Some(msg) = chat.messages.iter().rev().nth(chat.prev as usize){
			chat.current_input = msg.clone();
			chat.prev += 1;
		}
	}
	if is_key_pressed(KeyCode::Down) && chat.prev>0{
		chat.prev -= 1;

		if chat.prev == 0{
			chat.current_input.clear();
		}
		else{
			if let Some(msg) = chat.messages.iter().rev().nth((chat.prev - 1) as usize){
				chat.current_input = msg.clone();
			}
		}
	}



    if is_key_pressed(KeyCode::Backspace) {
        chat.current_input.pop();
		chat.prev = 0;
    }

    while let Some(character) = get_char_pressed() {
        if character.is_ascii_graphic() || character == ' '{
            chat.current_input.push(character);
			chat.prev = 0;
        }
    }
}

pub fn draw_chat(chat: &Chat) {
    let bottom_y = screen_height();
    let line_height = 25.0;

    let max_visible_messages = 5;
    let visible_messages = chat.messages.iter().rev().take(max_visible_messages);

    for (i, msg) in visible_messages.enumerate() {
        draw_text(msg, 20.0, bottom_y - 80.0 - (i as f32 * line_height), 32.0, WHITE);
    }


    if chat.is_active {

        draw_rectangle(15.0, bottom_y - 15.0, 600.0, -40.0, Color::new(0.0, 0.0, 0.0, 0.5));

        let display_text = format!("Chat: {}_", chat.current_input);
        draw_text(&display_text, 20.0, bottom_y - 25.0, 35.0, YELLOW);
    } else {
		draw_rectangle(15.0, bottom_y - 15.0, 600.0, -40.0, Color::new(0.0, 0.0, 0.0, 0.3));
        draw_text("Press [Enter]", 20.0, bottom_y - 25.0, 35.0, WHITE);

    }
}
