use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    None,
    GroupList,
	Auth,
	GroupCreate(String),
	GroupJoin(String),
	GroupInvite(String),
	GroupLeave,
	SendChat(String, String),
	Look,
	Command(String, String),
	Status,
	Move(Spawn),
	Take,
	Drop,
	Who,
	Items,
	Npcs,
	Talk(String),
	Attack(String),
	Quest(String),
	Inventory,
	Quests,
	Gold,
	Flee,
	Consume(String),
	Buy(String),
	Sell(String)
}


pub async fn handle_response(game: &mut Game, answer: &str, state: &str){
	match &game.pending_action {
		PendingAction::GroupList => {
			if state =="OK"{
					match serde_json::from_str::<Vec<String>>(answer) {
						Ok(players) => {
							game.group.grouplist = players;
						}
						Err(e) => {
							println!("Error JSON: {}", e);
						}
					}
				}
				else{
					game.group.in_group = false;
				}
		}

		PendingAction::Auth => {
			if state =="OK"{
				game.is_auth = true;
			} else {
				println!("already use")
			}
		}
		PendingAction::GroupCreate(group_name) => {
			if state =="OK"{
				game.group.in_group = true;
				if group_name.is_empty(){
					let name: String = format!("{}'s Group ",game.player.name.clone());
					game.group.name = name;}
				else{
					let name: String = format!("Group {}",group_name);
					game.group.name = name;
				}
			}
			else{
				println!("Failed group create");
			}
		}
		PendingAction::GroupJoin(sender) => {
			if state =="OK" {
				game.group.in_group = true;
				let name: String = format!("{}'s Group ",sender);
				game.group.name = name;

			} else{
				println!("Failed join");
			}
		}
		PendingAction::GroupLeave => {
			if state =="OK"{
				game.group.in_group = false;
				game.group.name = String::new();
				game.group.grouplist = Vec::new();
			}
		}
		PendingAction::GroupInvite(name) => {
			if state =="OK" {
				let rp: String = format!("{} invited", name);
				game.group.invite_info = Some(InviteInfo{
					state: rp,
					color: GREEN,
					time: get_time()
				});
			} else{
				let rp: String = format!("cannot invite {}", name);
				game.group.invite_info = Some(InviteInfo{
					state: rp,
					color: RED,
					time: get_time()
				});
			}
		}
		PendingAction::SendChat(channel, text) => {
			let channel = match channel.as_str(){
				"Room" => &mut game.chat.room_messages,
				"Global" => &mut game.chat.global_messages,
				"Group" => &mut game.chat.group_messages,
				_ => {
						game.pending_action = PendingAction::None;
						return;
					}
				};
			if state =="OK" {
				channel.push(text.to_string());
			} else{
				let rp: String = format!("[Error] {}", answer);
				channel.push(rp);
			}
		}
		PendingAction::Command(channel, cmd) => {
			let channel = match channel.as_str(){
				"Room" => &mut game.chat.room_messages,
				"Global" => &mut game.chat.global_messages,
				"Group" => &mut game.chat.group_messages,
				_ => {
						game.pending_action = PendingAction::None;
						return;
					}
				};
				channel.push(cmd.to_string());
				if state =="OK" {
					let rp: String = format!("[Server] {}", answer);
					channel.push(rp);
				}
				else{
					let rp: String = format!("[Error] {}", answer);
					channel.push(rp);
				}
			}
		PendingAction::Look => {

			match serde_json::from_str::<LookData>(answer) {
				Ok(look_data) => {
					game.map_data = Some(look_data);
				}
				Err(e) => {
				eprintln!("LOOK error: {}", e);
				}
			}
		}

		PendingAction::Items => {
			match serde_json::from_str::<HashMap<String, ItemData>>(answer) {
					Ok(items_data) => {
						for (item_id, item_data) in items_data{
							let texture: Texture2D = get_item_texture(&item_id).await;
							let item = Item{
								id: item_id.clone(),
								name: item_data.name,
								texture: texture,
								price: item_data.price,
								kind: item_data.kind
							};
							game.loaded_items.insert(item_id, item);
						}
					}
					Err(e) => {
					eprintln!("Items error: {}", e);
					}
			}
		}


		PendingAction::Npcs => {
			match serde_json::from_str::<HashMap<String, NpcData>>(answer) {
					Ok(npcs_data) => {
						for (npc_id , npc_data) in npcs_data{
							let texture: Texture2D = get_npc_texture(&npc_id).await;
							let npc: Npc = Npc::new(
								npc_id.clone(),
								texture,
								npc_data.name,
								npc_data.kind,
								npc_data.has_quest,
							);

							game.loaded_npcs.insert(npc_id, npc);
						}
					}
					Err(e) => {
					eprintln!("Npcs error: {}", e);
					}
				}

			}
		PendingAction::Status => {
			match serde_json::from_str::<PlayerState>(answer) {
				Ok(state) => {
					game.player.state = Some(state)
				}
				Err(e) => {
				eprintln!("STATUS error: {}", e);
				}
			}
		}
		PendingAction::Who => {
			if let Some(val_str) = answer.strip_prefix("players=") {
				match val_str.trim().parse::<i32>() {
					Ok(nb) => {
						game.nb_players = nb;
					}
					Err(e) => {
						println!("Who error parsing: {}", e);
					}
				}
			}
		}
		PendingAction::Move(new_spawn) => {
			if state =="OK" && game.player.new_spawn == Spawn::None{
				if let Some(val_str) = answer.strip_prefix("room=") {
					match val_str.trim().parse::<String>() {
						Ok(spawn) => {
							game.player.new_spawn = new_spawn.clone();
							if let Some(ref mut mapdata) = game.map_data{

								mapdata.room.id =spawn.to_string();
							}
						}
						Err(e) => {
							eprintln!("MOVE error: {}", e);
						}
					}
				}
			}
		}


		PendingAction::Talk(ref npc_id) => {
		if state =="OK"{
			if let Some(npc) = game.loaded_npcs.get_mut(npc_id){
							npc.npc_talk = Some(NpcTalk{
							texts: answer.to_string(),
							text_i: 0,
						})
					}
			}
		}
		PendingAction::Attack(ref npc_id) => {
			if state =="OK"{
				match serde_json::from_str::<FightData>(answer) {
					Ok(fight_data) => {
						if let Some(ref mut state) = game.player.state{
							state.hp = fight_data.attacker_hp;
							if let Some(ref mut fight) = &mut game.active_fight{
								fight.enemy_hp = fight_data.target_hp;
								fight.chat.push(format!("You attack and deal {} damage", fight_data.damage));
							}
							else{
								state.status= fight_data.status;
								game.active_fight = Some(Fight{
									enemy: game.loaded_npcs[npc_id].clone(),
									players: fight_data.fighters,
									enemy_hp: fight_data.target_hp,
									chat:vec!["You joined the fight".to_string()],
									consume_menu_open: false
								});

							}

						}
					}
					Err(e) => {
						eprintln!("Attack error: {}", e);
					}

				}
			}
			else {
				if answer.contains("NPC_NOT_HOSTILE"){
					if let Some(npc) = game.loaded_npcs.get_mut(npc_id){
							npc.npc_talk = Some(NpcTalk{
							texts:  "I am not Hostile".to_string(),
							text_i: 0,
						});
					}
				}
				if answer.contains("DEFEATED_FIGHTER"){
					if let Some(npc) = game.loaded_npcs.get_mut(npc_id){
							npc.npc_talk = Some(NpcTalk{
							texts:"You already lost".to_string(),
							text_i: 0,
						});
					}
				}
				if answer.contains("NOT_YOUR_TURN"){
					let Some(ref mut fight) = game.active_fight else { return };
					fight.chat.push("Wait for your turn".to_string());
				}
			}

		}

		PendingAction::Take => {
			if let Some(val_str) = answer.strip_prefix("taken=") {
				match val_str.trim().parse::<String>() {
					Ok(taken) => {
						if let Some(ref mut map_data) = game.map_data {
							if let Some(pos) = map_data.items.iter().position(|x| x == &taken) {
								map_data.items.remove(pos);
							}
							let current_count = game.player.inventory.data.get(&taken).copied().unwrap_or(0);
							game.player.inventory.data.insert(taken, current_count + 1);
						}
					}
					Err(e) => {
						println!("TAKE error parsing: {}", e);
					}
				}
			}
		}

		PendingAction::Drop => {
			if let Some(val_str) = answer.strip_prefix("dropped=") {
				match val_str.trim().parse::<String>() {
					Ok(dropped) => {
						if let Some(ref mut map_data) = game.map_data {
							let current_count = game.player.inventory.data.get(&dropped).copied().unwrap_or(0);
							if current_count > 0 {
								let new_count = current_count - 1;
								if new_count <= 0 {
									game.player.inventory.data.remove(&dropped);
								} else {
									game.player.inventory.data.insert(dropped.clone(), new_count);
								}
							}
							map_data.items.push(dropped);
						}
					}
					Err(e) => {
						println!("DROP error parsing: {}", e);
					}
				}
			}
		}
		PendingAction::Inventory => {
			if state =="OK"{
					match serde_json::from_str::<Vec<String>>(answer) {
						Ok(inventory) => {
							game.player.inventory.is_load = true;
							game.player.inventory.data = count_items(inventory);
						}
						Err(e) => {
							println!("Error JSON: {}", e);
						}
					}
				}
				else{
					game.group.in_group = false;
				}
		}
		PendingAction::Quests => {
			if state =="OK"{

					match serde_json::from_str::<Vec<QuestsData>>(answer) {
						Ok(quests) => {
							game.quests.is_load = true;
							for quest in quests{
								game.quests.all.push(
									Quest { npc_id: String::new(), quest_id: quest.quest_id, description: quest.progress, reward: String::new(), goal: None}
								)
							}

						}
						Err(e) => {
							println!("Error JSON: {}", e);
						}
					}
				}
				else{
					game.group.in_group = false;
				}
		}
		PendingAction::Quest(ref npc_id) => {
			if  state =="OK"{
				match serde_json::from_str::<QuestData>(answer) {
					Ok(quest) => {
						game.quests.all.push(Quest {
							npc_id: npc_id.to_string(),
							quest_id: quest.quest_id,
							description: quest.description,
							reward: quest.reward,
							goal: None
						});
					}
					Err(e) => {
						eprintln!("QUEST error: {}", e);
					}
				}
			}
		}
		PendingAction::Gold => {
			if let Some(val_str) = answer.strip_prefix("gold=") {
				match val_str.trim().parse::<i32>() {
					Ok(nb) => {
						game.player.gold = Some(nb);
					}
					Err(e) => {
						println!("GOLD error parsing: {}", e);
					}
				}
			}
		}
		PendingAction::Flee => {
			if state=="OK"{
				game.player.state = None;
				game.active_fight = None;
			}
		}
		PendingAction::Consume(used) => {
			if state=="OK"{
				let current_count = game.player.inventory.data.get(used).copied().unwrap_or(0);
				if current_count > 0 {
					let new_count = current_count - 1;
					if new_count <= 0 {
						game.player.inventory.data.remove(used);
					} else {
						game.player.inventory.data.insert(used.clone(), new_count);
					}
				}
			}
		}
		PendingAction::Buy(item_key) => {
			let color = if state == "OK" { GREEN } else { RED };
			game.npc_shop.buy_info = Some(InfoShop {
				color,
				time: get_time(),
			});

			if state == "OK" {
				game.player.inventory.is_load = false;
				if let Some(item) = game.loaded_items.get(item_key) {
					if let Some(ref mut gold) = game.player.gold {
						*gold -= item.price;
					}
				}
    		}
		}
		PendingAction::Sell(item_key) => {
			let color = if state == "OK" { GREEN } else { RED };
			game.npc_shop.sell_info = Some(InfoShop {
				color,
				time: get_time(),
			});

			if state == "OK" {
				game.player.inventory.is_load = false;
				if let Some(item) = game.loaded_items.get(item_key) {
					if let Some(ref mut gold) = game.player.gold {
						*gold += item.price;
					}
				}
    		}
		}
		_ => {
			game.pending_action = PendingAction::None;
		}
	}
			game.pending_action = PendingAction::None;

}



#[derive(Deserialize, Debug)]
pub struct ItemData {
    pub kind: ItemKind,
    pub name: String,
	pub price: i32
}

#[derive(Deserialize, Debug)]
pub struct NpcData {
	pub name: String,
	pub kind: NPCKind,
	pub has_quest: bool
}



#[derive(Deserialize, Debug)]
pub struct QuestData {
	pub quest_id: String,
    pub description: String,
    pub reward: String,
    pub status: String,
}


#[derive(Deserialize, Debug)]
pub struct QuestsData {
	pub progress: String,
    pub quest_id: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct FightData  {
	pub attacker_hp: i32,
    pub attacker_name: String,
	pub damage: i32,
    pub fighters: HashMap<String,i32>,
    pub status: Status,
	pub target_hp: i32
}



#[derive(Deserialize, Debug, Clone)]
pub struct LookData {
    pub room: RoomData,
	pub players: Vec<String>,
	pub items: Vec<String>,
    pub npcs: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RoomData {
    pub id: String,
    pub name: String,
	pub description: String,
    pub exits: HashMap<Direction, String>,

}



