
use ratatui::{Frame, layout::Rect};

use crate::{enums::focus::Focus, global_functions::{estimate_height::estimate_height, need_scrollbar::need_scrollbar}, structures::world::World};

pub fn draw_scrollbars(frame: &mut Frame, chat_area: &mut Rect, descr_area: &mut Rect, output_area: &mut Rect, world: &mut World) {
    
    world.room.available_focus.clear();
	for foc in [Focus::COMMAND, Focus::EXITS, Focus::INVENTORY, Focus::NPC] {
		world.room.available_focus.push(foc);
	}
    let mut global_messages: String = String::from("");
        for st in world.chat.global_messages.iter() {
            global_messages = format!("{}\n{}", global_messages, st);
        }

        let list: Vec<(&mut Rect, String, u16, bool, Focus)> = vec![
        (chat_area, global_messages, world.room.chat_scroll_pos, world.room.focus == Focus::CHAT, Focus::CHAT),
        (output_area, world.output.to_string(), world.room.output_scroll_pos, world.room.focus == Focus::OUTPUT, Focus::OUTPUT),
        (descr_area, world.room.description.to_string(), world.room.descr_scroll_pos, world.room.focus == Focus::DESCR, Focus::DESCR)
        ];

        for (rec, message, pos, foc_bool, foc) in list {
            let mess_height = estimate_height(*rec, &message) as usize;
            if need_scrollbar(frame, *rec, mess_height, pos, foc_bool) {
                world.room.available_focus.push(foc);
                rec.width = rec.width.saturating_sub(1);
            }
        }
        if !world.room.available_focus.contains(&world.room.focus) {
        world.room.focus = world.room.available_focus.first().cloned().unwrap_or(Focus::NONE);
    }
}
