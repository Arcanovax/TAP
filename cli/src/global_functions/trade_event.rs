use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    enums::{actions::PendingAction, channels::Channels, focus::Focus, states::States},
    structures::world::World,
};

pub fn trade_event(world: &mut World, key: KeyEvent, inventory: Vec<String>, npc_id: String) {
    match key.code {
        KeyCode::Tab => {
            world.room.focus = match world.room.focus {
                Focus::Buy => {
                    world.room.buy_list_state.select(None);
                    world.room.sell_list_state.select_first();
                    Focus::Sell
                }
                Focus::Sell => {
                    world.room.sell_list_state.select(None);
                    world.room.buy_list_state.select_first();
                    Focus::Buy
                }
                _ => Focus::Buy,
            }
        }
        KeyCode::Esc => {
            world.room.focus = Focus::Command;
            world.state = States::Idle;
        }
        KeyCode::Enter => match world.room.focus {
            Focus::Buy => {
                if let Some(index) = world.room.buy_list_state.selected_mut()
                    && let Some(item) = inventory.get(*index)
                {
                    world
                        .output
                        .push_back(format!("\n> BUY {} {}\n", npc_id, item));
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
            Focus::Sell => {
                if world.player.inventory.is_empty() {
                    world.output.push_back(String::from(
                        "Good job! You didn't sell anything, and you didn't earned anything!",
                    ));
                    world.room.output_scroll_pos.scroll_to_bottom();
                } else {
                    if let Some(index) = world.room.sell_list_state.selected_mut()
                        && let Some((item, ..)) = world.player.inventory.iter().nth(*index)
                    {
                        world
                            .output
                            .push_back(format!("\n> SELL {} {}\n", npc_id, item));
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
            _ => world.room.focus = Focus::Buy,
        },
        KeyCode::Up => match world.room.focus {
            Focus::Buy => world.room.buy_list_state.select_previous(),
            Focus::Sell => world.room.sell_list_state.select_previous(),
            _ => world.room.focus = Focus::Buy,
        },
        KeyCode::Down => match world.room.focus {
            Focus::Buy => world.room.buy_list_state.select_next(),
            Focus::Sell => world.room.sell_list_state.select_next(),
            _ => world.room.focus = Focus::Buy,
        },
        KeyCode::Right => match world.room.focus {
            Focus::Sell => {
                world.room.sell_list_state.select(None);
                world.room.buy_list_state.select_first();
                world.room.focus = Focus::Buy;
            }
            _ => world.room.focus = Focus::Buy,
        },
        KeyCode::Left => match world.room.focus {
            Focus::Buy => {
                world.room.buy_list_state.select(None);
                world.room.sell_list_state.select_first();
                world.room.focus = Focus::Sell
            }
            _ => world.room.focus = Focus::Sell,
        },
        KeyCode::F(number) => match number {
            7 => world.chat.channel = Channels::Global,
            8 => world.chat.channel = Channels::Room,
            9 => world.chat.channel = Channels::Group,
            _ => {}
        },
        _ => {}
    }
}
