use macroquad::prelude::*;
const CHANNELS: [&str; 3] = ["Room", "Global", "Group"];
const ALLOWED_COMMANDS: &[&str] = &[
    "/LOOK",
    "/WHO",
	"/STATUS",
	"/QUESTS",
	"/INVENTORY",
	"/QUESTS",
	"/GOLD",
	"/BUY"
];
use crate::*;

pub struct Chat {
    current_input: String,
    sended_messages: Vec<String>,
    pub is_active: bool,
	prev: i32,
    channel: i32,
    pub global_messages: Vec<String>,
	pub room_messages: Vec<String>,
	pub group_messages: Vec<String>,
}


impl Chat {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            sended_messages: Vec::new(),
            is_active: false,
			prev: 0,
            channel: 0,
            global_messages: Vec::new(),
			room_messages: Vec::new(),
			group_messages: Vec::new()
        }
    }
}


fn draw_chat_selection(x: f32, y: f32,selected: i32, mouse: Vec2) -> i32 {
	let mut selected_channel = selected;

    for i in 0..CHANNELS.len(){
		let btn = Rect::new(x+(i as f32)*100.0, y, 100.0, 30.0);
		if i as i32 == selected {
			if get_button(btn, CHANNELS[i], 30,Color::new(1.0, 1.0, 1.0, 1.0) , mouse){
				selected_channel = i as i32;
			}
		}
		else{
			if get_button(btn, CHANNELS[i], 30,Color::new(0.7, 0.7, 0.7, 1.0) , mouse){
				selected_channel = i as i32;
			}
		}
	}
    return selected_channel;
}


pub fn update_chat(game: &mut Game) {


	if is_key_pressed(KeyCode::Enter) {
		if !game.chat.current_input.trim().is_empty() {
			game.chat.sended_messages.push(game.chat.current_input.clone());
			if game.chat.current_input.starts_with("/"){
				handle_direct_command(game);
			}
			else{
				let rq: String = format!("CHAT {} {}\n", CHANNELS[game.chat.channel as usize],game.chat.current_input);
				game.tx_to_serv.try_send(rq).ok();
				let text: String = format!("[{}] {}\n",game.player.name, game.chat.current_input);
				game.pending_action = PendingAction::SendChat(CHANNELS[game.chat.channel as usize].to_string(), text);
			}

			game.chat.current_input.clear();
			game.chat.prev = 0;
		}
	}
	let chat = &mut game.chat;
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
    let mouse:Vec2  = vec2(mouse_position().0, mouse_position().1);
    let bottom_y = screen_height();
    let line_height = 25.0;




	let messages = match chat.channel {
    0 => &chat.room_messages,
    1 => &chat.global_messages,
    _ => &chat.group_messages,
	};

	let max_visible_lines = 10;
	let max_chars_per_line = 38;

	let mut visible_msg_in_lines: Vec<String> = Vec::new();

    for msg in messages.iter().rev() {
        let chars: Vec<char> = msg.chars().collect();
        let chunks: Vec<&[char]> = chars.chunks(max_chars_per_line).collect();
        for chunk in chunks.iter().rev() {
            visible_msg_in_lines.push(chunk.iter().collect());
            if visible_msg_in_lines.len() >= max_visible_lines {
                break;
            }
        }
        if visible_msg_in_lines.len() >= max_visible_lines {
            break;
        }
    }


    if chat.is_active {
		for (i, line) in visible_msg_in_lines.iter().enumerate() {
			draw_text(line, 20.0, bottom_y - 100.0 - (i as f32 * line_height), 32.0, WHITE);
		}
		chat.channel =  draw_chat_selection(15.0, bottom_y - 85.0, chat.channel,mouse);
        draw_rectangle(15.0, bottom_y - 15.0, 600.0, -40.0, Color::new(0.0, 0.0, 0.0, 0.5));
        let display_text = format!("Chat: {}_", chat.current_input);

        let chars: Vec<char> = display_text.chars().collect();
        let lines: Vec<&[char]> = chars.chunks(max_chars_per_line).collect();
		let line_str: String = lines[lines.len()-1].iter().collect();
		let y_pos = bottom_y - 25.0;
		draw_text(&line_str, 20.0, y_pos, 35.0, YELLOW);

    } else {
		for (i, line) in visible_msg_in_lines.iter().enumerate()  {
			draw_text(line, 20.0, bottom_y - 65.0 - (i as f32 * line_height), 32.0, WHITE);
		}
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

fn handle_direct_command(game: &mut Game){
	let rq: String = format!("{}\n",game.chat.current_input.clone()[1..].to_string());
	let cmd: String = format!("[{}] {}\n",game.player.name, game.chat.current_input);
	if !ALLOWED_COMMANDS.contains(&game.chat.current_input.to_ascii_uppercase().as_str()) {
		game.tx_to_serv.try_send("\n".to_string()).ok();
    }
	else{
		game.tx_to_serv.try_send(rq.clone()).ok();
	}
	game.pending_action = PendingAction::Command(CHANNELS[game.chat.channel as usize].to_string(), cmd);
}

