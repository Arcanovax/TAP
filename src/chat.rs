use macroquad::prelude::*;
const CHANNELS: [&str; 3] = ["Room", "Global", "Group"];
use crate::*;

pub struct Chat {
    current_input: String,
    sended_messages: Vec<String>,
    pub is_active: bool,
	prev: i32,
    channel: i32,
    pub all_messages: Vec<String>
}


impl Chat {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            sended_messages: Vec::new(),
            is_active: false,
			prev: 0,
            channel: 0,
            all_messages: Vec::new()
        }
    }
}


fn draw_chat_selection(x: f32, y: f32,selected: i32, mouse: (f32, f32)) -> i32 {
    let mouse_pos = Vec2::new(mouse.0, mouse.1);
	let mut selected_channel = selected;

    for i in 0..CHANNELS.len(){
            let btn = Rect::new(x+(i as f32)*100.0, y, 100.0, 30.0);
            let hovered = btn.contains(mouse_pos);
            let bg = if i as i32 == selected { Color::new(0.3, 0.3, 0.3, 0.75) }
			else { Color::new(0.10, 0.10, 0.10, 0.75) };
            draw_rectangle(btn.x, btn.y, btn.w, btn.h, bg);
            draw_text(CHANNELS[i], btn.x+ 6.0, btn.y + 22.5, 30.0, WHITE);
			if hovered && is_mouse_button_pressed(MouseButton::Left) {
				selected_channel = i as i32;
        	}
        }
    return selected_channel;
}


pub fn update_chat(game: &mut Game) {
    let chat = &mut game.chat;

	if is_key_pressed(KeyCode::Enter) {
		if !chat.current_input.trim().is_empty() {
			chat.all_messages.push(chat.current_input.clone());
			chat.sended_messages.push(chat.current_input.clone());
			if chat.current_input.starts_with("/"){
				let rq: String = format!("{}\n",&chat.current_input[1..].to_string());
				println!("{}", rq);
				game.tx_to_serv.try_send(rq).ok();
			}
			else{
				let rq: String = format!("CHAT {} {}\n", CHANNELS[chat.channel as usize],chat.current_input);
				game.tx_to_serv.try_send(rq).ok();
			}

			chat.current_input.clear();
			chat.prev = 0;
		}
	}
	if is_key_pressed(KeyCode::Escape) {
		chat.is_active = false;
		game.focus = InputFocus::Game;
	}
    if !chat.is_active{
        return;
    }

	if is_key_pressed(KeyCode::Right){
		chat.channel = ((chat.channel as usize + CHANNELS.len() + 1) % CHANNELS.len()) as i32;
	}
	if is_key_pressed(KeyCode::Left){
		chat.channel = ((chat.channel as usize + CHANNELS.len() - 1) % CHANNELS.len()) as i32;
	}

	if is_key_pressed(KeyCode::Up) && chat.prev < (chat.sended_messages.len()as i32){
		if let Some(msg) = chat.sended_messages.iter().rev().nth(chat.prev as usize){
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
			if let Some(msg) = chat.sended_messages.iter().rev().nth((chat.prev - 1) as usize){
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

pub fn draw_chat(chat:&mut Chat) {
    let mouse = mouse_position();
    let bottom_y = screen_height();
    let line_height = 25.0;

    let max_visible_messages = 5;
    let visible_messages = chat.all_messages.iter().rev().take(max_visible_messages);

    for (i, msg) in visible_messages.enumerate() {
        draw_text(msg, 20.0, bottom_y - 80.0 - (i as f32 * line_height), 32.0, WHITE);
    }


    if chat.is_active {

        draw_rectangle(15.0, bottom_y - 15.0, 600.0, -40.0, Color::new(0.0, 0.0, 0.0, 0.5));

        let display_text = format!("Chat: {}_", chat.current_input);
        draw_text(&display_text, 20.0, bottom_y - 25.0, 35.0, YELLOW);

        chat.channel =  draw_chat_selection(15.0, bottom_y - 85.0, chat.channel,mouse);

    } else {
		draw_rectangle(15.0, bottom_y - 15.0, 600.0, -40.0, Color::new(0.0, 0.0, 0.0, 0.3));
		if chat.current_input.trim().is_empty(){
			draw_text("Press [Enter]", 20.0, bottom_y - 25.0, 35.0, WHITE);
		}
		else {
			let text = format!("Chat: {}_", chat.current_input.clone());
			draw_text(text, 20.0, bottom_y - 25.0, 35.0, WHITE);
		}


    }
}


pub fn handle_chat(game: &mut Game) {
	draw_chat(&mut game.chat);
	if game.chat.is_active{
		game.focus = InputFocus::Chat;
		update_chat(game);
		if is_key_pressed(KeyCode::Escape) {
            game.chat.is_active = false;
            game.focus = InputFocus::Game;
        }
	}
	else if is_key_pressed(KeyCode::Enter) && game.focus == InputFocus::Game {
        game.chat.is_active = true;

	}
}
