use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    enums::{actions::PendingAction, channels::Channels, focus::Focus, states::States},
    structures::world::World,
};

pub fn trade_event(world: &mut World, key: KeyEvent, inventory: Vec<String>, npc_id: String) {
    match key.code {
        KeyCode::Tab => {
            world.room.focus = match world.room.focus {
                Focus::BUY => {
                    world.room.buy_list_state.select(None);
                    world.room.sell_list_state.select_first();
                    Focus::SELL
                }
                Focus::SELL => {
                    world.room.sell_list_state.select(None);
                    world.room.buy_list_state.select_first();
                    Focus::BUY
                }
                _ => Focus::BUY,
            }
        }
        KeyCode::Esc => {
            world.room.focus = Focus::COMMAND;
            world.state = States::Idle;
        }
        KeyCode::Enter => match world.room.focus {
            Focus::BUY => {
                if let Some(index) = world.room.buy_list_state.selected_mut()
                    && let Some(item) = inventory.get(*index) {
                        world
                            .output
                            .push_back(format!("\n> {}", format!("BUY {} {}\n", npc_id, item)));
                        world.room.output_scroll_pos.scroll_to_bottom();
                        let _ = world
                            .tx_to_serv
                            .try_send(format!("BUY {} {}\n", npc_id, item));
                        if let Some(item_obj) = world.list_items.get(item) {
                            world.action = PendingAction::Buy(item_obj.name.clone());
                        } else {
                            world.action = PendingAction::Buy(item.clone());
                        }
                    }
            }
            Focus::SELL => {
                if world.player.inventory.is_empty() {
                    world.output.push_back(String::from(
                        "Good job! You didn't sell anything, and you didn't earned anything!",
                    ));
                    world.room.output_scroll_pos.scroll_to_bottom();
                } else {
                    if let Some(index) = world.room.sell_list_state.selected_mut()
                        && let Some((item, ..)) = world.player.inventory.iter().nth(*index) {
                            world.output.push_back(format!(
                                "\n> {}",
                                format!("SELL {} {}\n", npc_id, item)
                            ));
                            world.room.output_scroll_pos.scroll_to_bottom();
                            let _ = world
                                .tx_to_serv
                                .try_send(format!("SELL {} {}\n", npc_id, item));
                            if let Some(item_obj) = world.list_items.get(item) {
                                world.action = PendingAction::Sell(item_obj.name.clone());
                            } else {
                                world.action = PendingAction::Sell(item.clone());
                            }
                        }
                }
            }
            _ => world.room.focus = Focus::BUY,
        },
        KeyCode::Up => match world.room.focus {
            Focus::BUY => world.room.buy_list_state.select_previous(),
            Focus::SELL => world.room.sell_list_state.select_previous(),
            _ => world.room.focus = Focus::BUY,
        },
        KeyCode::Down => match world.room.focus {
            Focus::BUY => world.room.buy_list_state.select_next(),
            Focus::SELL => world.room.sell_list_state.select_next(),
            _ => world.room.focus = Focus::BUY,
        },
        KeyCode::Right => match world.room.focus {
            Focus::SELL => {
                world.room.sell_list_state.select(None);
                world.room.buy_list_state.select_first();
                world.room.focus = Focus::BUY;
            }
            _ => world.room.focus = Focus::BUY,
        },
        KeyCode::Left => match world.room.focus {
            Focus::BUY => {
                world.room.buy_list_state.select(None);
                world.room.sell_list_state.select_first();
                world.room.focus = Focus::SELL
            }
            _ => world.room.focus = Focus::SELL,
        },
        KeyCode::F(number) => match number {
            7 => world.chat.channel = Channels::GLOBAL,
            8 => world.chat.channel = Channels::ROOM,
            9 => world.chat.channel = Channels::GROUP,
            _ => {}
        },
        _ => {}
    }
}
