use std::{time::{SystemTime, UNIX_EPOCH}};

use crate::{state::ServerInfo, structures::{
	enums::{
		enn_att_res::EnnAttRes,
		npc_kind::NPCKind,
		state::State
	},
	fight::Fight,
	npc::NPC,
	player::Player
}};

pub fn enemy_attack(opponent_id: &str, world: &mut ServerInfo) -> EnnAttRes {

    // let world = &mut *world_mut;
    let fight: &mut Fight = world.fights.get_mut(opponent_id).unwrap();
    let opponent: &NPC = world.npcs.get(opponent_id).unwrap();
    let target: &mut Player;
    let nb_fighters = fight.fighters.len();
	let mut target_index: usize = 0;

    if nb_fighters == 1 {
        target = &mut world.connections.get_mut(fight.fighters.get(0).unwrap()).unwrap().player;
    } else {
		let nb_fighters: usize = fight.fighters.len();
		let nanos = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.subsec_nanos() as usize;
		target_index = nanos % nb_fighters;
        target = &mut world.connections.get_mut(&fight.fighters[target_index]).unwrap().player;
    }
    if let NPCKind::Enemy { damages, ..} = opponent.kind {
        if damages < target.hp {
            target.hp -= damages;
            fight.enemy_turn = false;
            EnnAttRes::Hit(format!("{enn_name} deals {damages} damages to {pl}. Remains {hp} HP to {pl}", pl=target.name, enn_name=opponent.name, hp=target.hp))
        } else {
            target.hp = target.max_hp - 10;
            target.location = String::from("loc.city_square");
			fight.enemy_turn = false;
			target.status = State::Idle;
            if nb_fighters == 1 {
                world.fights.remove(opponent_id);
                EnnAttRes::KillAndWin(format!("{enn_name} have killed {pl}. {enn_name} won the fight. Shame on you players!", pl=target.name, enn_name=opponent.name))
            } else {
                fight.fighters.remove(target_index);
                EnnAttRes::Kill(format!("{enn_name} have killed {pl}.", pl=target.name, enn_name=opponent.name))
            }
        }
    } else {
        world.fights.remove(opponent_id);
        EnnAttRes::Error(format!("{enn_name} is not an enemy", enn_name=opponent.name))
    }

}