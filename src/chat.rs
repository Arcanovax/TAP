use macroquad::prelude::*;

pub struct Chat {
    current_input: String,
    messages: Vec<String>,
    pub hidden: bool,
	prev: i32
}

impl Chat {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            messages: Vec::new(),
            hidden: true,
			prev: 0
        }
    }
}

pub fn update_chat(chat: &mut Chat) {
    if is_key_pressed(KeyCode::Enter) {
        if chat.hidden {
            chat.hidden = false;
        }
		else {
			if !chat.current_input.trim().is_empty() {
				chat.messages.push(chat.current_input.clone());
				chat.current_input.clear();
				chat.prev = 0;
        	}
			chat.hidden = true;
	}
	}
	if is_key_pressed(KeyCode::Escape) {
		chat.hidden = true;
	}
	
	if is_key_pressed(KeyCode::Up) && !chat.hidden && chat.prev<(chat.messages.len()as i32){
		if let Some(msg) = chat.messages.iter().rev().nth(chat.prev as usize){
			chat.current_input = msg.clone();
			chat.prev += 1;
		}
	}
	if is_key_pressed(KeyCode::Down) && !chat.hidden && chat.prev>0{
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




    if chat.hidden {
        while get_char_pressed().is_some() {

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
    let start_y = 70.0;
    let line_height = 20.0;

    let max_visible_messages = 5;
    let visible_messages = chat.messages.iter().rev().take(max_visible_messages).rev();

    for (i, msg) in visible_messages.enumerate() {
        draw_text(msg, 20.0, start_y + (i as f32 * line_height), 10.0, WHITE);
    }


    if !chat.hidden {

        draw_rectangle(15.0, start_y + 110.0, 400.0, 30.0, Color::new(0.0, 0.0, 0.0, 0.5));

        let display_text = format!("Chat: {}_", chat.current_input);
        draw_text(&display_text, 20.0, start_y + 130.0, 20.0, YELLOW);
    } else {
        draw_text("Appuyez sur [Entrée] pour parler", 20.0, start_y + 130.0, 16.0, GRAY);
    }
}
