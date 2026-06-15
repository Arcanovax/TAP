use crate::{
    handlers::quest::quest_request,
    protocol::EventType,
    state::{SharedServer, Tx},
    structures::quest::{Goal, Quest},
};
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    command::Command,
    error::ErrorCode,
    handlers::{
        chat::chat_request, connect::connect_request, drop::drop_request, fight_func::fight::fight,
        group::group_request, inventory::inventory_request, look::look_request,
        movement::move_request, status::status_request, take::take_request, talk::talk_request,
        who::who_request,
    },
    protocol::Message,
};

pub fn handle_request(
    request: Message,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    let command_result = match &request {
        Message::Command { name, args } => match Command::parse(name) {
            Some(Command::CONNECT) => connect_request(args, server_info, peer_addr, tx),
            Some(Command::QUIT) => Message::Response {
                error: ErrorCode::SUCCESS,
                data: None,
            },
            Some(Command::WHO) => who_request(server_info),
            Some(Command::CHAT) => chat_request(args, server_info, peer_addr),
            Some(Command::GROUP) => group_request(args, server_info, peer_addr),
            Some(Command::STATUS) => status_request(server_info, peer_addr),
            Some(Command::MOVE) => move_request(server_info, peer_addr, args),
            Some(Command::TALK) => talk_request(peer_addr, args, server_info),
            Some(Command::ATTACK) => fight(peer_addr, args, server_info),
            Some(Command::LOOK) => look_request(server_info, peer_addr),
            Some(Command::DROP) => drop_request(server_info, peer_addr, args),
            Some(Command::TAKE) => take_request(server_info, peer_addr, args),
            Some(Command::INVENTORY) => inventory_request(server_info, peer_addr),
            Some(Command::QUEST) => quest_request(args, server_info, peer_addr),
            _ => Message::Response {
                error: ErrorCode::INVALID_COMMAND,
                data: None,
            },
        },
        _ => Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        },
    };

    update_quests(server_info, peer_addr, &command_result, &request);

    command_result
}

fn update_quests(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    result: &Message,
    request: &Message,
) {
    let command = match request {
        Message::Command { name, .. } => match Command::parse(name) {
            Some(c) => c,
            None => return,
        },
        _ => return,
    };
    match result {
        Message::Response { .. } => {}
        _ => return,
    };
    let mut binding = server_info.lock().unwrap();
    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(_) => return,
    };
    let mut to_advance = Vec::new();
    match command {
        Command::TAKE => {
            for (id, step) in &player.quests_in_progress {
                let quest = binding.world.quests.get(id).unwrap();
                match &quest.goals[*step] {
                    Goal::Collect { item, amount } => {
                        if player.inventory.get(item).unwrap_or(&0) < amount {
                            continue;
                        }
                        to_advance.push(id.clone());
                    }
                    _ => continue,
                }
            }
        }
        _ => return (),
    }
    if to_advance.len() == 0 {
        return;
    }
    for id in &to_advance {
        let new_step = {
            let player = binding.get_player_mut(peer_addr).unwrap();
            let step = player.quests_in_progress.entry(id.to_string()).or_insert(0);
            *step += 1;
            *step
        };
        let quest = binding.world.quests.get(id).unwrap();
        let tx = binding.get_connection(peer_addr).unwrap().tx.clone();
        if new_step == quest.goals.len() {
            send_quest_finish_event(&quest.name, tx);
            {
                let player = binding.get_player_mut(peer_addr).unwrap();
                player.finished_quest.insert(id.to_string());
                player.quests_in_progress.remove(id);
            }
        } else {
            send_quest_update_event(quest.clone(), new_step, tx);
        }
    }
}

fn send_quest_update_event(quest: Quest, step: usize, tx: UnboundedSender<Message>) {
    let _ = tx.send(Message::Event(EventType::QUEST_UPDATE {
        quest_name: quest.name,
        goal: quest.goals[step].clone(),
    }));
}

fn send_quest_finish_event(quest_name: &str, tx: UnboundedSender<Message>) {
    let _ = tx.send(Message::Event(EventType::QUEST_FINISH {
        quest_name: quest_name.to_string(),
    }));
}
