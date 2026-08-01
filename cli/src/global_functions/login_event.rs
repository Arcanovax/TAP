use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{enums::actions::PendingAction, structures::world::World};

pub fn login_event(key: KeyEvent, world: &mut World) {
    match key.code {
        KeyCode::Char(c) => {
            if world.input.len() < 20 {
                world.input.push(c);
            }
        }
        KeyCode::Backspace => {
            world.input.pop();
        }
        KeyCode::Enter => {
            let _ = world
                .tx_to_serv
                .try_send(format!("CONNECT {}\n", world.input));
            world.action = PendingAction::Auth;
        }
        _ => {}
    }
}
