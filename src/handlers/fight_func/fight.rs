use crate::{
	error::ErrorCode::{self, SUCCESS},
	handlers::{
		fight_func::{
			attack::execute_attack,
			enemy_attack::enemy_attack,
			is_it_my_turn::is_it_my_turn
		},
		global_func::{
			check_fight::check_fight,
			get_player::get_player_mut,
			is_he_there::is_he_there
		}
	},
	protocol::Message,
	state::SharedServer,
	structures::{
		enums::{
			attack_res::AttackRes,
			enn_att_res::EnnAttRes,
			fight_outcome::FightOutput,
			npc_kind::NPCKind,
			state::State,
			turn_res::TurnRes
		},
	fight::Fight,
	}
};

pub fn fight(peer_addr: &str, target: &str, world: &SharedServer) -> Message {
	if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
	let mut world_mut: &mut ServerInfo = world.lock().unwrap();
	match get_player_mut(world_mut.players, peer_addr) {
		Ok(player) => {
			if let Some(loc) = world_mut.rooms.get(&player.location) {
	
				if !is_he_there(target, loc) {
					return Message::Response {
						error: ErrorCode::NPC_NOT_FOUND,
						data: Some(serde_json::to_string("This target isn't here.").unwrap()),
					};
				}
				if let NPCKind::Enemy { ref defeated, .. } = &world_mut.npcs[target].kind {
					if *defeated {
						return Message::Response {
							error: ErrorCode::DEFEATED_ENEMY,
							data: Some(serde_json::to_string("This target has already been defeated").unwrap()),
						};
					}
				} else {
					return Message::Response {
							error: ErrorCode::NPC_NOT_HOSTILE,
							data: Some(serde_json::to_string("This target isn't an enemy.").unwrap()),
						};
				}
				match &player.status {
					State::Idle => {
						if let Some(fight) = check_fight(target, &mut world_mut.fights){
							fight.fighters.push(player.name.clone());
						} else {
							world_mut.fights.insert(target.to_string(), Fight{
								fighters: vec![player.name.clone()],
								turn: 0,
								enemy_turn: false
							});
						}
						player.status = State::InFight { target_id: (target.to_string()) };
						Message::Response {
							error: ErrorCode::SUCCESS,
							data: Some(serde_json::to_string(format!("{} says: 'Hello there!'", player.name)).unwrap())
						}
					},
					State::InFight { target_id: target } => {
						let fight = check_fight(&target, &mut world_mut.fights).expect("There is no fight.");
	
						match is_it_my_turn(&player.name, fight){
							TurnRes::MyTurn => {
								match execute_attack(splitted[0], splitted[2], &mut world) {
										AttackRes::Hit(message) => {
											if world.fights.get(splitted[2]).unwrap().enemy_turn {
												match enemy_attack(splitted[2], &mut world) {
													EnnAttRes::Hit(msg) |
													EnnAttRes::Kill(msg) |
													EnnAttRes::KillAndWin(msg) |
													EnnAttRes::Error(msg) => Message::Response {
														error: ErrorCode::SUCCESS,
														data: Some(serde_json::to_string(format!("{}\n{}",message, msg)).unwrap())
													}
												}
											} else {
												Message::Response {
														error: ErrorCode::SUCCESS,
														data: Some(serde_json::to_string(msg).unwrap())
													}
											}
										},
										AttackRes::KillTarget(msg) => {
											Message::Response {
														error: ErrorCode::SUCCESS,
														data: Some(serde_json::to_string(msg).unwrap())
													}
										},
										AttackRes::Peace(msg) => Message::Response {
														error: ErrorCode::SUCCESS,
														data: Some(serde_json::to_string(msg).unwrap())
													}
								}
							}
							TurnRes::NotMyTurn | TurnRes::EnemyTurn => Message::Response {
								error: ErrorCode::SUCCESS,
								data: Some(serde_json::to_string("It's not your turn!").unwrap())
							}
						}
					},
					State::Discuss => Message::Response {
								error: ErrorCode::INVALID_COMMAND,
								data: Some(serde_json::to_string("You can't fight in your state.").unwrap())
							}
				}
			} else {
				Message::Response {
					error: ErrorCode::ROOM_NOT_FOUND,
					data: Some(serde_json::to_string("You're nowhere. I can't find you.").unwrap()),
				}
			}

		}
		Err(msg) => Message::Response {
					error: ErrorCode::PLAYER_NOT_FOUND,
					data: Some(serde_json::to_string(msg).unwrap()),
				}
	}
	}