use crate::structures::{enums::{enn_att_res::EnnAttRes, npc_kind::NPCKind, state::State}, fight::Fight, npc::NPC, player::Player, world::World };

pub fn enemy_attack(opponent_id: &str, world: &mut World) -> EnnAttRes {
    let fight: &mut Fight = world.fights.get_mut(opponent_id).unwrap();
    let opponent: &NPC = world.npcs.get(opponent_id).unwrap();
    let mut target: &mut Player = world.players.get_mut(fight.fighters.get(0).unwrap()).unwrap() ;
    let nb_fighters = fight.fighters.len();
    let mut res: i32 = 0;

    if nb_fighters == 1 {
        target = world.players.get_mut(fight.fighters.get(0).unwrap()).unwrap() 
    } else {
        for fighter in &fight.fighters {
            let name_len: i32 = fighter.len()as i32;
            res += name_len;
            while res > nb_fighters as i32 - 1 { res = (nb_fighters as i32 - res).abs() }
            target = world.players.get_mut(fight.fighters.get(res as usize).unwrap()).unwrap() 
        }
    }
    if let NPCKind::Enemy { damages, ..} = opponent.kind {
        if damages < target.hp {
            target.hp -= damages;
            fight.enemy_turn = false;
            EnnAttRes::Hit(format!("{enn_name} deals {damages} damages to {pl}. Remains {hp} HP to {pl}", pl=target.name, enn_name=opponent.name, hp=target.hp))
        } else {
            target.hp = target.max_hp - 10;
            target.location = String::from("loc.city_square");
			target.status = State::Idle;
            if nb_fighters == 1 {
                world.fights.remove(opponent_id);
                EnnAttRes::KillAndWin(format!("{enn_name} have killed {pl}. {enn_name} won the fight. Shame on you players!", pl=target.name, enn_name=opponent.name))
            } else {
                fight.fighters.remove(res as usize);
                EnnAttRes::Kill(format!("{enn_name} have killed {pl}.", pl=target.name, enn_name=opponent.name))
            }
        }
    } else {
        world.fights.remove(opponent_id);
        EnnAttRes::Error(format!("{enn_name} is not an enemy", enn_name=opponent.name))
    }

}