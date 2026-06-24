use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::error::ErrorCode,
};
use serde::Serialize;
use std::net::SocketAddr;
use tracing::info;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
enum Status {
    Completed,
    Active { progress: String },
}

#[derive(Serialize)]
struct QuestView<'a> {
    quest_id: &'a String,
    #[serde(flatten)]
    status: Status,
}

pub fn quests_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let binding = server_info.lock().unwrap();

    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };

    let mut quests: Vec<QuestView> = Vec::new();

    for (quest_id, step) in &player.quests_in_progress {
        let quest = binding.world.quests.get(quest_id).unwrap();
        quests.push(QuestView {
            quest_id: quest_id,
            status: Status::Active {
                progress: format!("{}/{}", step, quest.goals.len()),
            },
        });
    }

    for quest_id in &player.finished_quest {
        quests.push(QuestView {
            quest_id,
            status: Status::Completed,
        });
    }

    info!("Get quest list");

    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(serde_json::to_value(quests).unwrap()),
    }
}
