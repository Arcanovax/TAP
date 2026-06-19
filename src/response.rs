use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    None,
    GroupList,
	Auth,
	GroupCreate(String),
	GroupJoin(String, String),
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
	Quest(String)
}


pub async fn handle_response(game: &mut Game, server_event: ServerEvent, msg: String){
	match &game.pending_action {
		PendingAction::GroupList => {
			if msg.contains("SUCCESS"){
				if let Some(data_val) = server_event.data {
					let data_str = data_val.to_string();
					match serde_json::from_str::<Vec<String>>(&data_str) {
						Ok(players) => {
							game.group.grouplist = players;
						}
						Err(e) => {
							println!("Error JSON: {}", e);
						}
					}
				}
				else if msg.contains("NOT_IN_GROUP") {
					game.group.in_group = false;
				}
			}
		}
		PendingAction::Auth => {
			if msg.contains("SUCCESS") {
				game.is_auth = true;
			} else if msg.contains("NAME_IN_USE") {
				println!("already use")
			}
		}
		PendingAction::GroupCreate(group_name) => {
			if msg.contains("SUCCESS") {
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
		PendingAction::GroupJoin(sender, group_name) => {
			if msg.contains("SUCCESS") {
				game.group.in_group = true;
				if sender == group_name{
					let name: String = format!("{}'s Group ",sender);
					game.group.name = name;}
				else{
					let name: String = format!("{}",group_name);
					game.group.name = name;
				}
			} else{
				println!("Failed join");
			}
		}
		PendingAction::GroupLeave => {
			if msg.contains("SUCCESS") {
				game.group.in_group = false;
				game.group.name = String::new();
				game.group.grouplist = Vec::new();
			}
		}
		PendingAction::GroupInvite(name) => {
			if msg.contains("SUCCESS") {
				let rp: String = format!("{} invited", name);
				game.group.invite_info = Some(InviteInfo{
					state: rp,
					color: GREEN,
					time: get_time()
				});
			} else{
				let rp: String = format!("{} is offline", name);
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
			if msg.contains("SUCCESS") {
				channel.push(text.to_string());
			} else{
				if let Some(error) = server_event.error{
					let rp: String = format!("[Error] {}", error);
					channel.push(rp);
				}
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
			if msg.contains("SUCCESS") {
				if let Some(data) = server_event.data{
						let rp: String = format!("[Server] {}", data);
						channel.push(rp);
					}
				}
			else {
				if let Some(error) = server_event.error{
						let rp: String = format!("[Server] {}", error);
						channel.push(rp);
					}
				}
			}
		PendingAction::Look => {
			if let Some(data_val) = &server_event.data {
				let data_str = data_val.to_string();
				match serde_json::from_str::<LookData>(&data_str) {
					Ok(look_data) => {
						game.map_data = Some(look_data);
					}
					Err(e) => {
					eprintln!("LOOK error: {}", e);
					}
				}

			}
		}
		PendingAction::Items => {
			if let Some(data_val) = &server_event.data {
				let data_str = data_val.to_string();
				match serde_json::from_str::<HashMap<String, ItemData>>(&data_str) {
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
		}
		PendingAction::Npcs => {
			if let Some(data_val) = &server_event.data {
				let data_str = data_val.to_string();
				match serde_json::from_str::<HashMap<String, NpcData>>(&data_str) {
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
		}
		PendingAction::Status => {
			if let Some(data_val) = &server_event.data {
				let data_str = data_val.to_string();
				match serde_json::from_str::<PlayerState>(&data_str) {
					Ok(state) => {
						game.player.state = Some(state)
					}
					Err(e) => {
					eprintln!("STATUS error: {}", e);
					}
				}

			}
		}
		PendingAction::Who => {
			if let Some(data_val) = &server_event.data {
				let data_str = data_val.to_string();
				match serde_json::from_str::<Players>(&data_str) {
					Ok(players) => {
						game.nb_players =players.players;
					}
					Err(e) => {
					eprintln!("Who error: {}", e);
					}
				}

			}
		}
		PendingAction::Move(new_spawn) => {

			if msg.contains("SUCCESS") && game.player.new_spawn == Spawn::None{
				if let Some(data_val) = server_event.data{
					let data_str = data_val.to_string();
					match serde_json::from_str::<MoveData>(&data_str) {
						Ok(parsed_map_data) => {
							game.player.new_spawn = new_spawn.clone();
							if let Some(ref mut mapdata) = game.map_data{

								mapdata.room.id = parsed_map_data.room;
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

			if msg.contains("SUCCESS"){
				if let Some(data_val) = server_event.data{
					let data_str = data_val.to_string();
					match serde_json::from_str::<Vec<String>>(&data_str) {
						Ok(texts) => {
							if let Some(npc) = game.loaded_npcs.get_mut(npc_id){
								npc.npc_talk = Some(NpcTalk{
								texts: texts,
								text_i: 0
							})
							}
						}
						Err(e) => {
							eprintln!("Talk error: {}", e);
						}
					}
				}
			}
		}
		PendingAction::Quest(ref npc_id) => {
			if msg.contains("SUCCESS"){
				if let Some(data_val) = server_event.data{
					let data_str = data_val.to_string();
					match serde_json::from_str::<QuestData>(&data_str) {
						Ok(quest) => {
							game.quests.push(Quest {
								npc_id: npc_id.to_string(),
								id: quest.id,
								name: quest.name,
								description: quest.description,
								reward: quest.reward,
								goals: quest.goals
							});
						}
						Err(e) => {
							eprintln!("MOVE error: {}", e);
						}
					}
				}
			}
		}
		PendingAction::Attack(ref npc_id) => {

			if msg.contains("SUCCESS"){
				if let Some(ref mut state) = game.player.state{
							state.status= Status::InFight { target_id: npc_id.to_string()}
					};

			}
		}

		PendingAction::Take => {
			if let Some(data_val) = server_event.data{
				let data_str = data_val.to_string();
				if !data_str.is_empty(){

					match serde_json::from_str::<Value>(&data_str) {
						Ok(json) => {
							let taken: String = json["taken"].as_str().unwrap_or("").to_string();
							if let Some(ref mut map_data) = game.map_data {
								if let Some(pos) = map_data.items.iter().position(|x| x == &taken) {
									map_data.items.remove(pos);
								}
								let current_count = game.player.inventory.data.get(&taken).copied().unwrap_or(0);
								game.player.inventory.data.insert(taken, current_count + 1);
							}
						}
						Err(e) => {
							println!("Take JSON: {}", e);
						}
					}

				}
			}

		}
		PendingAction::Drop => {
			if let Some(data_val) = server_event.data{
				let data_str = data_val.to_string();
				if !data_str.is_empty(){
					match serde_json::from_str::<Value>(&data_str) {
						Ok(json) => {
							let dropped: String = json["dropped"].as_str().unwrap_or("").to_string();
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
							println!("Error JSON: {}", e);
						}
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
pub struct MoveData {
	pub room: String,
}


#[derive(Deserialize, Debug)]
pub struct QuestData {
	pub id: String,
	pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum Goal {
    Collect { item: String, amount: u32 },
    Talk { dialog: String },
}



#[derive(Deserialize, Debug, Clone)]
pub struct LookData {
    pub room: RoomData,
	pub players: Vec<String>,
	pub items: Vec<String>,
    pub npcs: Vec<String>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct RoomData {
    pub id: String,
    pub name: String,
	pub description: String,
    pub exits: Vec<Exit>,

}



