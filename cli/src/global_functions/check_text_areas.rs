use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{enums::focus::Focus, structures::world::World};

pub fn check_text_areas(world: &mut World, key: KeyEvent) -> bool {
    if (world.room.focus == Focus::COMMAND || world.room.focus == Focus::CHATTEXT)
        && ![
            KeyCode::Tab,
            KeyCode::Enter,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::BackTab,
        ]
        .contains(&key.code)
    {
        if world.room.focus == Focus::COMMAND {
            world.room.text_area.input(key);
            world.index_command = 0;
        } else {
            world.room.chat_text_area.input(key);
        }
        true
    } else {
        false
    }
}
