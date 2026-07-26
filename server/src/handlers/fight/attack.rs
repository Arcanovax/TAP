use std::{collections::HashMap, net::SocketAddr};

use tracing::info;

use crate::{
    handlers::fight::enemy_attack::enemy_attack,
    protocol::{EventType, Message},
    state::ServerInfo,
    structures::{
        attack_result::AttackResult,
        dungeon::parse_dungeon_id,
        enums::{item_kind::ItemKind, npc_kind::NPCKind, state::State},
    },
};

pub fn execute_attack(
    player_id: SocketAddr,
    target_id: &str,
    world_mut: &mut ServerInfo,
) -> AttackResult {
    let (curr_damages, player_hp, player_name) = {
        let player = &world_mut.connections.get(&player_id).unwrap().player;
        let mut dmg: u32 = 15;

        for id in player.inventory.keys() {
            if let Some(item) = world_mut.resolve_item(id)
                && let ItemKind::Weapon { damages } = item.kind
                    && dmg < damages {
                        dmg = damages;
                    }
        }
        (dmg, player.hp, player.name.clone())
    };

    let mut trigger_enemy_attack = false;
    let mut enemy_died = false;
    let mut target_hp_after = 0;
    let fighters_list;
    let mut loot_list = Vec::new();

    let mut advance_turn = false;
    {
        let enemy = world_mut.resolve_npc_mut(target_id).unwrap();

        if let NPCKind::Enemy {
            ref mut hp,
            ref loot,
            ref mut defeated,
            ..
        } = enemy.kind
        {
            if curr_damages < *hp {
                *hp -= curr_damages;
                target_hp_after = *hp;
                advance_turn = true;
            } else {
                *hp = 0;
                *defeated = true;
                target_hp_after = 0;
                enemy_died = true;
                loot_list = loot.clone();
            }
        }
    }

    if enemy_died {
        info!(attacker = %player_name, target = %target_id, damage = curr_damages, loot = ?loot_list, "enemy defeated");
    } else {
        info!(attacker = %player_name, target = %target_id, damage = curr_damages, enemy_hp = target_hp_after, "attack landed");
    }

    {
        let fight = world_mut.fights.get_mut(target_id).unwrap();
        fighters_list = fight.fighters.clone();

        if advance_turn {
            if fight.turn == fight.fighters.len() as u32 - 1 {
                fight.turn = 0;
                trigger_enemy_attack = true;
            } else {
                fight.turn += 1;
            }
        }
    }

    for fighter in &fighters_list {
        if *fighter != player_name
            && let Some(con) = world_mut
                .connections
                .values()
                .find(|c| c.player.name == *fighter)
            {
                let _ = con.tx.send(Message::Event(EventType::ATTACK {
                    player_name: player_name.clone(),
                    damages: curr_damages,
                    enemy_hp: target_hp_after,
                    loot: loot_list.clone(),
                }));
            }
    }

    let mut fighters: HashMap<String, u32> = HashMap::new();

    if enemy_died {
        let mut status: State = State::Idle;
        for pl_name in &fighters_list {
            if let Some(conn) = world_mut
                .connections
                .values_mut()
                .find(|c| &c.player.name == pl_name)
            {
                let pl = &mut conn.player;
                pl.status = State::Idle;
                status = pl.status.clone();
                for loot_item in &loot_list {
                    if loot_item == "item.gold" {
                        pl.gold += 50;
                    } else {
                        *pl.inventory.entry(loot_item.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        for fighter_name in &fighters_list {
            if let Some(fighter) = world_mut
                .connections
                .values()
                .find(|c| &c.player.name == fighter_name)
            {
                fighters.insert(fighter.player.name.clone(), fighter.player.hp);
            }
        }

        if let Some(gid) = parse_dungeon_id(target_id) {
            let cleared = world_mut
                .dungeons
                .get(&gid)
                .map(|d| {
                    d.npcs.values().all(|npc| match &npc.kind {
                        NPCKind::Enemy { defeated, .. } => *defeated,
                        _ => true,
                    })
                })
                .unwrap_or(false);

            if cleared {
                world_mut.close_dungeon(gid);
                info!(dungeon = %gid, "dungeon cleared");
            }
        }

        world_mut.fights.remove(target_id);

        return AttackResult {
            attacker_hp: player_hp,
            attacker_name: player_name,
            target_hp: target_hp_after,
            damage: curr_damages,
            status,
            fighters: Some(fighters),
            loot: loot_list,
        };
    }

    if trigger_enemy_attack {
        enemy_attack(target_id, world_mut)
    }

    for fighter_name in &fighters_list {
        if let Some(fighter) = world_mut
            .connections
            .values()
            .find(|c| &c.player.name == fighter_name)
        {
            fighters.insert(fighter.player.name.clone(), fighter.player.hp);
        }
    }

    AttackResult {
        attacker_hp: player_hp,
        attacker_name: player_name,
        target_hp: target_hp_after,
        damage: curr_damages,
        status: world_mut
            .connections
            .get(&player_id)
            .unwrap()
            .player
            .status
            .clone(),
        fighters: Some(fighters),
        loot: loot_list,
    }
}
