use crate::{fight_func::is_it_my_turn::is_it_my_turn, global_func::{check_fight::check_fight, get_player::get_player_mut, is_he_there::is_he_there}, structures::{enums::{fight_outcome::FightOutput, npc_kind::NPCKind, state::State, turn_res::TurnRes}, fight::Fight, world::World}};

pub fn fight(player_name: &str, target: &str, world: &mut World) -> Result<FightOutput, &'static str> {
	match get_player_mut(&mut world.players, player_name) {
		Ok(player) => {
			if let Some(loc) = world.rooms.get(&player.location) {
	
				if !is_he_there(target, loc) {
					return Err("This target isn't here.");
				}
				if let NPCKind::Enemy { ref beaten, .. } = &world.npcs[target].kind {
					if *beaten {
						return Err("This target has already been defeated");
					}
				} else {
					return Err("This target isn't an enemy.");
				}
				match &player.status {
					State::Idle => {
						if let Some(fight) = check_fight(target, &mut world.fights){
							fight.fighters.push(player.name.clone());
						} else {
							world.fights.insert(target.to_string(), Fight{
								fighters: vec![player.name.clone()],
								turn: 0,
								enemy_turn: false
							});
						}
						player.status = State::InFight { target_id: (target.to_string()) };
						Ok(FightOutput::Enter(format!("{} says: 'Hello there!'", player.name)))
					},
					State::InFight { target_id: target } => {
						let fight = check_fight(&target, &mut world.fights).expect("There is no fight.");
	
						match is_it_my_turn(&player.name, fight){
							TurnRes::MyTurn => Ok(FightOutput::ReadyToAttack),
							TurnRes::NotMyTurn | TurnRes::EnemyTurn => Ok(FightOutput::WaitingToAttack)
						}
					},
					State::Discuss | State::Respawn => Err("You can't fight in your state.")
				}
			}else {
				Err("There is no one by that name here")
			}

		}
		Err(msg) => Err(msg)
	}
	}