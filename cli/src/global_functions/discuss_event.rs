use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    enums::{focus::Focus, states::States},
    structures::world::World,
};

pub fn discuss_event(key: KeyEvent, world: &mut World, name: String) {
    if key.code == KeyCode::Enter {
        if let Some(new_sentence) = world.room.dialogs.pop_front() {
            world.state = States::InDiscuss(name.to_string(), new_sentence);
            world.message = String::new();
            world.counter = 0;
            world.index_sentence = 0;
        } else {
            world.message = String::new();
            world.counter = 0;
            world.index_sentence = 0;
            world.state = States::Idle;
            world.room.focus = Focus::Command;
        }
    }
}
