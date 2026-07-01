use std::{fs::OpenOptions, io::Write};

use ratatui::{crossterm::{event::{MouseButton, MouseEvent, MouseEventKind}}, layout::Position};

use crate::{enums::{actions::PendingAction, states::States}, structures::world::World};

pub fn handle_mouse(event: MouseEvent, world: &mut World) {
	if event.kind == MouseEventKind::Down(MouseButton::Left) {
		match &world.state {
			States::Login => {
				world.click = !world.click;
				world.error = false;
			},
			States::InFight { target_id } => {
				// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
				// 	let _ = writeln!(file, "Entre (State {:?}) ", target_id);}
				let (x, y) = (event.column, event.row);
				let position = Position::new(x, y);
				for (action, button) in &world.room.fight.buttons {
					if button.contains(position) && !world.room.fight.bag{
						match action.as_str() {
							"ATTACK" => {
								let mut target = target_id;
								for (name, npc) in &world.list_npcs {
									if npc.name == *target_id {
										target = name;
									}
								}
								// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
								// 	let _ = writeln!(file, "target (State {:?}) : {:#?}", target, target_id);}
								let _ = world.tx_to_serv.try_send(format!("ATTACK {}\n", target));
								world.action = PendingAction::Attack(target_id.clone());
							},
							"FLEE" => {
								let mut target = target_id;
								for (name, npc) in &world.list_npcs {
									if npc.name == *target_id {
										target = name;
									}
								}
								let _ = world.tx_to_serv.try_send(format!("FLEE {}\n", target));
								world.action = PendingAction::Flee;
							},
							"BAG" => {
								world.room.fight.bag = true;
								world.room.bag_state.select_first();
							},
							_ => {}
						}
					}
				}
				
			},
			_ => {}
		}
	}
}