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
}
