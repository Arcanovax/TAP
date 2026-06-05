use std::collections::HashMap;

use crate::fight_func::is_it_my_turn::is_it_my_turn;

use crate::global_func::{
	is_he_there::is_he_there,
	check_fight::check_fight,
};

use crate::structures::enums::attack_res::AttackRes;
use crate::structures::enums::{
	exits::Exits,
	state::State,
	npc_kind::NPCKind,
	item_kind::ItemKind,
	fight_outcome::FightOutput
};

use crate::structures::items::Items;
use crate::structures::{
	location::Location,
	npc::NPC,
	fight::Fight
};

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub location: String,
	pub status: State,
	pub inventory: HashMap<String, u32>,
	pub available_quests: Vec<String>
}

impl Player {
	pub fn talk_to(&self, target: &str, list_loc: &HashMap<String, Location>, npc_list: &HashMap<String, NPC>) -> Result<String, &'static str> {
		if let Some(loc) = list_loc.get(&self.location) {

			if is_he_there(target, loc) {
				return Ok(npc_list[target].dialogue[0].clone());
			} else {
				return Err("No character by that name");
			};
		}
		Err("There is no one by that name here")
	}

	pub fn move_to(&mut self, list_loc: &HashMap<String, Location>, dest: &str) -> Result<(), &'static str>{
		if let Some(loc) = list_loc.get(&self.location) {

			for exit in &loc.exits {
				let (dir_name, target) = match exit {
					Exits::North { toward } => ("North", toward),
					Exits::South { toward } => ("South", toward),
					Exits::East { toward } => ("East", toward),
					Exits::West { toward } => ("West", toward),
				};
				// println!("{} et {} et {}", dir_name, dest, &loc.name);
				if dir_name == dest {
					// println!("{} et {}", dir_name, target);
					self.location = target.clone(); return Ok(());
				}
			}
			return Err("No gateway on that direction.");
		}
		Err("You're nowhere. I can't find you.")
	}

	pub fn fight(&mut self, target: &str, list_loc: &HashMap<String, Location>, list_fights: &mut HashMap<String, Fight>) -> Result<FightOutput, &'static str> {
		if let Some(loc) = list_loc.get(&self.location) {

			if !is_he_there(target, loc) {
				return Err("This target isn't here.");
			}
			match &self.status {
				State::Idle => {
					if let Some(fight) = check_fight(target, list_fights){
						fight.fighters.push(self.name.clone());
					} else {
						list_fights.insert(target.to_string(), Fight{
							fighters: vec![self.name.clone()],
							turn: 0
						});
					}
					self.status = State::InFight { target_id: (target.to_string()) };
					Ok(FightOutput::Enter(format!("{} says: 'Hello there!'", &self.name)))
				},
				State::InFight { target_id: target } => {
					let fight = check_fight(&target, list_fights).expect("There is no fight.");

					if is_it_my_turn(&self.name, fight){
						Ok(FightOutput::ReadyToAttack)
					} else {
						Ok(FightOutput::WaitingToAttack)
					}
				},
				State::Discuss | State::Respawn => Err("You can't fight in your state.")
			}
		}else {
			Err("There is no one by that name here")
		}
	}

	pub fn attack(&self, target: &str, list_npc: &mut HashMap<String, NPC>, list_items: &HashMap<String, Items>) -> AttackRes<String> {
	if let Some(enemy) = list_npc.get_mut(target) {
		let mut curr_damages: u32 = 15;
		for id in self.inventory.keys() {
			if let Some(item) = list_items.get(id) {
				if let ItemKind::Weapon { damages } = item.kind{
					if curr_damages < damages {
						curr_damages = damages;
					}
				}
				// match item.kind {
				// 	ItemKind::Weapon { damages } => {
				// 		if curr_damages < damages {
				// 			curr_damages = damages;
				// 		}
				// 	}
				// 	_ => (),
				// }
			}
		}

		if let NPCKind::Enemy { ref mut hp, ref loot, .. } = enemy.kind {
			if curr_damages <= *hp {
				*hp -= curr_damages;
				return AttackRes::Hit(format!("{name} deals {curr_damages} damages to {e_name}. Remains {hp} HP to the enemy", name=self.name, e_name=enemy.name, hp=*hp));
			} else {
				*hp = 0;
				return AttackRes::KillTarget(format!("{name} defeats {e_name}. Each fighter earn {loots:#?}", name=self.name, e_name=enemy.name, loots=*loot));
			}
		} else {
			return AttackRes::Peace("Peace man! This target is not an enemy.");
		}
	}
	AttackRes::NotFound("No Enemy")
}
}