
use std::{fs::OpenOptions, io::Write};

use ratatui::{Frame, layout::Rect};

use crate::{enums::{channels::Channels, focus::Focus}, global_functions::{estimate_height::estimate_height, need_scrollbar::need_scrollbar}, structures::world::World};

pub fn draw_scrollbars(frame: &mut Frame, chat_area: &mut Rect, descr_area: &mut Rect, output_area: &mut Rect, world: &mut World) {
    
    world.room.available_focus.clear();
	for foc in [Focus::COMMAND, Focus::CHAT, Focus::NPC, Focus::INVENTORY, Focus::EXITS] {
		world.room.available_focus.push(foc);
	}
	let mut chat_messages: String = String::from("");
	
	match world.chat.channel {
		Channels::GLOBAL => {
			for st in world.chat.global_messages.iter() {
            chat_messages = format!("{}\n{}", chat_messages, st);
        	}
		},
		Channels::GROUP => {
			for st in world.chat.group_messages.iter() {
            chat_messages = format!("{}\n{}", chat_messages, st);
        	}
		},
		Channels::ROOM => {
			for st in world.chat.room_messages.iter() {
            chat_messages = format!("{}\n{}", chat_messages, st);
        	}
		}
	}
	
	let list: Vec<(&mut Rect, String, u16, bool, Focus)> = vec![
		(chat_area, chat_messages, world.room.chat_scroll_pos, world.room.focus == Focus::CHAT, Focus::CHAT),
        (output_area, world.output.to_string(), world.room.output_scroll_pos, world.room.focus == Focus::OUTPUT, Focus::OUTPUT),
        (descr_area, world.room.room_view.description.to_string(), world.room.descr_scroll_pos, world.room.focus == Focus::DESCR, Focus::DESCR)
        ];
		
        for (rec, message, pos, foc_bool, foc) in list {
			let mess_height = estimate_height(*rec, &message) as usize;
            if need_scrollbar(frame, *rec, mess_height, pos, foc_bool) {
				if foc != Focus::CHAT {
					world.room.available_focus.push(foc);
				} else {
					world.chat.scroll_bar = true;
				}
                rec.width = rec.width.saturating_sub(1);
            } else {
				if foc == Focus::CHAT {
					world.chat.scroll_bar = false;
				}
			}
        }
		// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
		// 		let _ = writeln!(file, "ko (State {:?}) : {:#?}", world.state, world.room.focus);}
        if !world.room.available_focus.contains(&world.room.focus) {
        world.room.focus = Focus::COMMAND;
    }
}
