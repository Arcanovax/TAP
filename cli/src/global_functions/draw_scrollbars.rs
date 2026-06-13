use std::rc::Rc;

use ratatui::{Frame, layout::Rect};

use crate::{enums::focus::Focus, global_functions::{estimate_height::estimate_height, need_scrollbar::need_scrollbar}, structures::world::World};

pub fn draw_scrollbars(frame: &mut Frame, blocks_left: &Rc<[Rect]>, blocks_right: &Rc<[Rect]>, world: &mut World) {
    
    world.room.available_focus.clear();
    let mut global_messages: String = String::from("");
        for st in world.chat.global_messages.iter() {
            global_messages = format!("{}\n{}", global_messages, st);
        }
        
        let list: Vec<(Rect, String, u16, bool, Focus)> = vec![
        (blocks_left[2], global_messages, world.room.chat_scroll_pos, world.room.focus == Focus::CHAT, Focus::CHAT),
        (blocks_left[3], world.output.to_string(), world.room.output_scroll_pos, world.room.focus == Focus::OUTPUT, Focus::OUTPUT),
        (blocks_right[1], world.room.description.to_string(), world.room.descr_scroll_pos, world.room.focus == Focus::DESCR, Focus::DESCR)
        ];

        for (rec, message, pos, foc_bool, foc) in list {
            let mess_height = estimate_height(rec, &message) as usize;
            if need_scrollbar(frame, rec, mess_height, pos, foc_bool) {
                world.room.available_focus.push(foc);
            }
        }
}
