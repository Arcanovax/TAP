use std::collections::HashMap;

use crate::{enums::goals::Goal, structures::{npc::NPC, quest::Quest}};

pub fn check_goals(list_npcs: HashMap<String, NPC>, quest: &mut Quest) {
	for goal in quest.goals.iter_mut() {
		match goal {
			Goal::Retrieve { item, amount, dialog } => {
				let split_dialog: Vec<&str> = dialog.split(".").collect();
				let id = split_dialog[..2].join(".");
				if let Some(npc) = list_npcs.get(&id) {
					let npc_name = npc.name.clone();
					*goal = Goal::Retrieve { item: item.clone(), amount: *amount, dialog: npc_name };
				}
			},
			Goal::Talk { dialog } => {
				let split_dialog: Vec<&str> = dialog.split(".").collect();
				let id = split_dialog[..2].join(".");
				if let Some(npc) = list_npcs.get(&id) {
					let npc_name = npc.name.clone();
					*goal = Goal::Talk { dialog: npc_name };
				}
			},
			_ => {}
		}
	}
}