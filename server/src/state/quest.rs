use crate::structures::{
    enums::{error::ErrorCode, game_event::GameEvent},
    quest::Quest,
};

use super::*;
use serde_json::Value;

impl ServerInfo {
    pub fn try_accept_quest(
        &mut self,
        peer_addr: SocketAddr,
        npc_name: &str,
    ) -> Result<Value, ErrorCode> {
        let npc = match self.world.npcs.get(npc_name) {
            Some(npc) => npc,
            None => return Err(ErrorCode::NPC_NOT_FOUND),
        };
        if let None = npc.quest {
            return Err(ErrorCode::NO_QUEST_AVAILABLE);
        }
        let quest_ref = npc.quest.clone().unwrap();
        let player = self.get_player_mut(peer_addr)?;
        if player.finished_quest.contains(&quest_ref)
            || player.quests_in_progress.contains_key(&quest_ref)
        {
            return Err(ErrorCode::NO_QUEST_AVAILABLE);
        }
        player.quests_in_progress.insert(quest_ref.clone(), 0);
        let quest = self.world.quests.get(&quest_ref).unwrap();
        Ok(serde_json::to_value(quest).unwrap())
    }

    pub fn advance_quests(&mut self, peer_addr: SocketAddr, event: Option<&GameEvent>) {
        let player = match self.get_player(peer_addr) {
            Ok(player) => player,
            Err(_) => return,
        };

        let mut to_advance: Vec<String> = Vec::new();
        for (quest_ref, step) in &player.quests_in_progress {
            let Some(quest) = self.world.quests.get(quest_ref) else {
                continue;
            };
            if quest.goals[*step].is_satisfied(player, event) {
                to_advance.push(quest_ref.clone());
            }
        }

        for id in &to_advance {
            let new_step = {
                let player = self.get_player_mut(peer_addr).unwrap();
                let step = player.quests_in_progress.entry(id.to_string()).or_insert(0);
                *step += 1;
                *step
            };
            let quest = self.world.quests.get(id).unwrap();
            let tx = self.get_connection(peer_addr).unwrap().tx.clone();
            if new_step == quest.goals.len() {
                send_quest_finish_event(&quest.name, tx);
                {
                    let player = self.get_player_mut(peer_addr).unwrap();
                    player.finished_quest.insert(id.to_string());
                    player.quests_in_progress.remove(id);
                }
            } else {
                send_quest_update_event(quest.clone(), new_step, tx);
            }
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
