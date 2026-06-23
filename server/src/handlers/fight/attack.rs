use std::{collections::HashMap, fs::OpenOptions, net::SocketAddr, io::Write};

use crate::{
    handlers::fight::enemy_attack::enemy_attack, protocol::{EventType, Message}, state::ServerInfo, structures::{attack_result::Attack_Result, enums::{
        item_kind::ItemKind, npc_kind::NPCKind, state::State,
    }},
};

pub fn execute_attack<'a>(
    player_id: SocketAddr,
    target_id: &str,
    world_mut: &mut ServerInfo,
) -> Attack_Result {
    
    let (curr_damages, player_hp, player_name) = {
        let player = &world_mut.connections.get(&player_id).unwrap().player;
        let mut dmg: u32 = 15;
        
        for id in player.inventory.keys() {
            if let Some(item) = world_mut.world.items.get(id) {
                if let ItemKind::Weapon { damages } = item.kind {
                    if dmg < damages {
                        dmg = damages;
                    }
                }
            }
        }
        (dmg, player.hp.clone(), player.name.clone())
    };

    let mut trigger_enemy_attack = false;
    let mut enemy_died = false;
    let mut target_hp_after = 0;
    let mut fighters_list = Vec::new();
    let mut loot_list = Vec::new();

    {
        let enemy = world_mut.world.npcs.get_mut(target_id).unwrap();
        let fight = world_mut.fights.get_mut(target_id).unwrap();
        fighters_list = fight.fighters.clone();

        if let NPCKind::Enemy { ref mut hp, ref loot, ref mut defeated, .. } = enemy.kind {
            if curr_damages < *hp {
                *hp -= curr_damages;
                target_hp_after = *hp;
                
                if fight.turn == fight.fighters.len() as u32 - 1 {
                    fight.turn = 0;
                    trigger_enemy_attack = true;
                } else {
                    fight.turn += 1;
                }
            } else {
                *hp = 0;
                *defeated = true;
                target_hp_after = 0;
                enemy_died = true;
                
                loot_list = loot.clone();
            }
        }
    }

    // if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
    //             let _ = writeln!(file, "coucou {:#?}", fighters_list);}
    for fighter in &fighters_list {
        if let Some(con) = world_mut.connections.values().find(|c| c.player.name == *fighter) {
            // if con.player.name != player_name {
                let _ = con.tx.send(Message::Event(EventType::ATTACK {
                    player_name: player_name.clone(),
                    damages: curr_damages,
                    enemy_hp: target_hp_after 
                }));
            // }
        } else {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
                let _ = writeln!(file, "coucou");}
        }
    }

    let mut fighters: HashMap<String, u32> = HashMap::new();

    if enemy_died {
        let mut status: State = State::Idle;
        for pl_name in &fighters_list {
            if let Some(conn) = world_mut.connections.values_mut().find(|c| &c.player.name == pl_name) {
                let pl = &mut conn.player;
                pl.status = State::Idle;
                status = pl.status.clone();
                for loot_item in &loot_list {
                    let amount = if loot_item == "item.gold" { 50 } else { 1 };
                    *pl.inventory.entry(loot_item.clone()).or_insert(0) += amount;
                }
            }
        }


        for fighter_name in &fighters_list {
            if let Some(fighter) = world_mut.connections.values().find(|c| &c.player.name == fighter_name){
                fighters.insert(fighter.player.name.clone(), fighter.player.hp);
            }
        }

        return Attack_Result {
            attacker_hp: player_hp,
            attacker_name: player_name,
            target_hp: target_hp_after,
            damage: curr_damages,
            status: status,
            enemy_attack: None,
            fighters: Some(fighters)
        };
    }

    let enemy_atk = if trigger_enemy_attack {
        Some(enemy_attack(target_id, world_mut))
    } else {
        None
    };

    for fighter_name in &fighters_list {
        if let Some(fighter) = world_mut.connections.values().find(|c| &c.player.name == fighter_name){
            fighters.insert(fighter.player.name.clone(), fighter.player.hp);
        }
    }

    Attack_Result {
        attacker_hp: player_hp,
        attacker_name: player_name,
        target_hp: target_hp_after,
        damage: curr_damages,
        status: world_mut.connections.get(&player_id).unwrap().player.status.clone(),
        enemy_attack: enemy_atk,
        fighters: Some(fighters)
    }
}


// pub fn execute_attack<'a>(
//     player_id: SocketAddr,
//     target_id: Vec<String>,
//     world_mut: &mut ServerInfo,
// ) -> Attack_Result {
//     // let mut pre_world_mut = world.lock().unwrap();
//     // let world_mut = &mut *world;
//     let player = &mut world_mut.connections.get_mut(&player_id).unwrap().player;
//     let enemy = world_mut.world.npcs.get_mut(&target_id[0]).unwrap();
//     let fight = world_mut.fights.get_mut(&target_id[0]).unwrap();

//     let mut curr_damages: u32 = 15;
//     for id in player.inventory.keys() {
//         if let Some(item) = world_mut.world.items.get(id) {
//             if let ItemKind::Weapon { damages } = item.kind {
//                 if curr_damages < damages {
//                     curr_damages = damages;
//                 }
//             }
//         }
//     }

//     if let NPCKind::Enemy {
//         ref mut hp,
//         ref loot,
//         ref mut defeated,
//         ..
//     } = enemy.kind
//     {
//         if curr_damages < *hp {
//             *hp -= curr_damages;
//             if fight.turn == fight.fighters.len() as u32 - 1 {
//                 // fight.enemy_turn = true;
//                 fight.turn = 0;
//                 Attack_Result {
//                     attacker_hp: player.hp.clone(),
//                     attacker_name: player.name.clone(),
//                     target_hp: *hp,
//                     damage: curr_damages,
//                     status: FighterStatus::COMBAT,
//                     enemy_attack: Some(enemy_attack(&target_id[0], world_mut))
//                 }
//             } else {
//                 fight.turn += 1;
//                 Attack_Result {
//                     attacker_hp: player.hp.clone(),
//                     attacker_name: player.name.clone(),
//                     target_hp: *hp,
//                     damage: curr_damages,
//                     status: FighterStatus::COMBAT,
//                     enemy_attack: None
//                 }
//             }
//         } else {
//             *hp = 0;
//             *defeated = true;
//             for pl_name in &mut fight.fighters {
//                 let pl = &mut world_mut.connections.get_mut(pl_name).unwrap().player;
//                 pl.status = State::Idle;
//                 for loot_item in loot {
//                     let amount = if loot_item == "item.gold" { 50 } else { 1 };
//                     *pl.inventory.entry(loot_item.clone()).or_insert(1) += amount;
//                 }
//             }
//             Attack_Result {
//                     attacker_hp: player.hp.clone(),
//                     attacker_name: player.name.clone(),
//                     target_hp: *hp,
//                     damage: curr_damages,
//                     status: FighterStatus::COMBAT,
//                     enemy_attack: None
//                 }
//         };
//     }
//     Attack_Result {
//         attacker_hp: player.hp.clone(),
//         attacker_name: player.name.clone(),
//         target_hp: 0,
//         damage: 0,
//         status: FighterStatus::COMBAT,
//         enemy_attack: None
//     }
// }

