
use std::{fs::OpenOptions, io::Write};

use ratatui::{Frame, layout::Rect};

use crate::{enums::{channels::Channels, focus::Focus}, global_functions::{estimate_height::estimate_height, need_scrollbar::need_scrollbar}, structures::world::World};

pub fn draw_scrollbars(frame: &mut Frame, chat_area: &mut Rect, descr_area: &mut Rect, output_area: &mut Rect, world: &mut World) {
    
    world.room.available_focus.clear();
	for foc in [Focus::COMMAND, Focus::CHAT, Focus::NPC, Focus::OUTPUT, Focus::INVENTORY, Focus::EXITS] {
		world.room.available_focus.push(foc);
	}
	let mut chat_messages: String = String::from("");
	let output_messages: String = world.chat.global_messages.iter().cloned().collect::<Vec<String>>().join("");
	
	match world.chat.channel {
		Channels::GLOBAL => {
			let msgs: Vec<String> = world.chat.global_messages.iter().cloned().collect();
            chat_messages = msgs.join("");
		},
		Channels::GROUP => {
			let msgs: Vec<String> = world.chat.group_messages.iter().cloned().collect();
            chat_messages = msgs.join("");
		},
		Channels::ROOM => {
			let msgs: Vec<String> = world.chat.room_messages.iter().cloned().collect();
            chat_messages = msgs.join("");
		}
	}
	
	let list: Vec<(&mut Rect, String, u16, bool, Focus)> = vec![
		(chat_area, chat_messages, world.room.chat_scroll_pos, world.room.focus == Focus::CHAT, Focus::CHAT),
        // (output_area, output_messages, world.room.output_scroll_pos, world.room.focus == Focus::OUTPUT, Focus::OUTPUT),
        (descr_area, world.room.room_view.description.to_string(), world.room.descr_scroll_pos, world.room.focus == Focus::DESCR, Focus::DESCR)
        ];
		
        for (rec, message, pos, foc_bool, foc) in list {
			let mess_height = estimate_height(*rec, &message, false) as usize;

			let max_scroll = mess_height.saturating_sub(rec.height as usize) as u16;
  			let clamped_pos = pos.min(max_scroll);

			match foc {
            Focus::CHAT => {
				if world.room.focus != Focus::CHAT {
				world.room.chat_scroll_pos = max_scroll;
				} else {
					world.room.chat_scroll_pos = clamped_pos;
				}
			},
            // Focus::OUTPUT => world.room.output_scroll_pos = clamped_pos,
            Focus::DESCR => world.room.descr_scroll_pos = clamped_pos,
            _ => {}
    		}

            if need_scrollbar(frame, *rec, mess_height, pos, foc_bool) {
				if foc != Focus::CHAT {
					world.room.available_focus.push(foc.clone());
				} else {
					world.chat.scroll_bar = true;
				}
                rec.width = rec.width.saturating_sub(1);
				// if world.chat.scroll_bar {
				// 	if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
				// 			let _ = writeln!(file, "ko (State {:?}) : {:#?}", foc, rec.width);}
				// }
            } else {
				if foc == Focus::CHAT {
					world.chat.scroll_bar = false;
				}
			}
        }
        if !world.room.available_focus.contains(&world.room.focus) {
        world.room.focus = Focus::COMMAND;
    }
}
