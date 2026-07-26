use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{enums::focus::Focus, structures::world::World};

pub fn check_text_areas(world: &mut World, key: KeyEvent) -> bool {
    if (world.room.focus == Focus::Command || world.room.focus == Focus::ChatText)
        && ![
            KeyCode::Tab,
            KeyCode::Enter,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::BackTab,
            KeyCode::F(1),
            KeyCode::F(2),
            KeyCode::F(3),
            KeyCode::F(4),
            KeyCode::F(5),
            KeyCode::F(6),
            KeyCode::F(7),
            KeyCode::F(8),
            KeyCode::F(9),
        ]
        .contains(&key.code)
    {
        if world.room.focus == Focus::Command {
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
