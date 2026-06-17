use std::{fs::OpenOptions, io::Write};

use crate::{enums::{actions::PendingAction, states::States}, structures::{server_event::ServerEvent, world::World}};

pub fn response_handling(world: &mut World, msg: &str, server: &ServerEvent) {
	match world.state {
		States::Login => {
			if world.action == PendingAction::Auth{
				if server.error == Some("SUCCESS".to_string()) {
					world.state = States::InGame;
					world.player.name = world.input.to_string();
					world.input.clear();
					let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
					// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
					// 	let _ = writeln!(file, "ko (State {:?}) : {:#?}", world.state, res);}
					world.action = PendingAction::Look;
				} else if msg.contains("NAME_IN_USE"){
					world.error = true;
					world.click = true;
				}
			}
			// let datas: LoginResponse = serde_json::from_str(&json.get_mut("data").unwrap()).unwrap();
			// self.room = datas.room;
			// self.player = datas.player;
			// self.state = States::InGame;
			// self.input.clear();
			// self.input = msg.to_string()
		},
		States::InGame => {
			match world.action {
				PendingAction::Look => {
					world.room = serde_json::from_value(server.data.clone().unwrap()).unwrap();
					world.action = PendingAction::None;
				},
				PendingAction::Move => {
					if msg.contains("SUCCESS") {
					let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
					world.action = PendingAction::Look;
					}
				}
				_ => {}
			}
		}
		_ => {}
	}
}