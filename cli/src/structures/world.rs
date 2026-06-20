use std::{
	any::type_name, collections::{HashMap, VecDeque}, fs::OpenOptions, io::{self, Write}, sync::mpsc::{
		Receiver,
		TryRecvError
	}
};

use ratatui::{
	DefaultTerminal,
	Frame,
	crossterm::event::{
		self, Event, KeyCode, MouseButton, MouseEventKind
	},
};
use serde_json::Deserializer;
use tokio::sync::mpsc::Sender;

use crate::{
	draw_functions::{
		discuss::draw_room_discuss, login::login_draw, rooms::draw_room, wait_server::draw_wait
	},
	enums::{
		actions::PendingAction, channels::Channels, exits::Exits, focus::Focus, states::States
	},
	global_functions::{find_action::find_action, response_handling::response_handling},
	structures::{
		chat::Chat, group::{Group, Invitation}, items::Item, npc::NPC, player::Player, room::Room, server_event::ServerEvent
	}
};

pub struct World<'a> {
	pub room: Room<'a>,
	pub player: Player,
	pub quit: bool,
	pub message: String,
	pub chat: Chat,
	pub output: VecDeque<String>,
	pub action: PendingAction,
	pub group: Group,
	pub counter: u32,
	pub error: bool,
	pub message_error: String,
	pub click: bool,
	pub list_items: HashMap<String, Item>,
	pub list_npcs: HashMap<String, NPC>,
	pub input: String,
	pub state: States,
	pub tx_to_serv: tokio::sync::mpsc::Sender<String>,
	pub rx_from_serv: std::sync::mpsc::Receiver<String>
}

impl World<'_>{

	pub fn new(tx_to_serv: Sender<String>, rx_from_serv: Receiver<String>) -> Self {
		Self {
			room: Room::new(),
			player: Player::new(),
			quit: false,
			message: String::from(""),
			chat: Chat::new(),
			output: VecDeque::new(),
			action: PendingAction::None,
			counter: 0,
			group: Group::new(),
			error: false,
			list_items: HashMap::new(),
			list_npcs: HashMap::new(),
			message_error: "".to_string(),
			click: false,
			input: "".to_string(),
			state: States::ServerWait,
			tx_to_serv: tx_to_serv,
			rx_from_serv: rx_from_serv
		}
	}

	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.quit {
			self.process_network();
			terminal.draw(|frame| self.draw(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}
	pub fn draw(&mut self, frame:&mut Frame) {
		match &self.state {
			States::ServerWait => draw_wait(frame),
			States::Login => login_draw(self, frame),
			States::InGame => {
				// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
				// 	let _ = writeln!(file, "RECU (State {:?}) : {:#?}", self.state, self.player.name);}
				draw_room(self, frame);
			},
			States::InDiscuss(name, sentence) => {
				draw_room_discuss(self, frame, name.to_string(), sentence.to_string());
			}
			_ => {}
		}
	}

	fn handle_events(&mut self) -> io::Result<()> {
		if event::poll(std::time::Duration::from_millis(16))? {
			match event::read()? {
				Event::Key(key) => {
					if key.code == KeyCode::Esc { self.quit = true };
					match &self.state {
						States::Login => {
							match key.code {
								KeyCode::Char(c) => {
									if self.input.len() < 20 {self.input.push(c);}
								},
								KeyCode::Backspace => { self.input.pop(); },
								KeyCode::Enter => {
									let _ = self.tx_to_serv.try_send(format!("CONNECT {}\n", self.input));
									self.action = PendingAction::Auth;
								}
								_ => {}
							}
						},
						States::InGame => {
							if self.room.focus == Focus::COMMAND && key.code != KeyCode::Tab && key.code != KeyCode::Enter{
									self.room.text_area.input(key);
							} else {
								match key.code {
									KeyCode::Down => {
										match self.room.focus {
											Focus::CHAT => self.room.chat_scroll_pos.scroll_down(),
											Focus::DESCR => self.room.descr_scroll_pos.scroll_down(),
											Focus::OUTPUT => self.room.output_scroll_pos.scroll_down(),
											Focus::NPC => self.room.npc_list_state.select_next(),
											Focus::INVENTORY => self.room.inventory_list_state.select_next(),
											Focus::EXITS => self.room.exits_list_state.select_next(),
											_ => {}
										}
										}
									KeyCode::Up => {
										match self.room.focus {
											Focus::CHAT => self.room.chat_scroll_pos.scroll_up(),
											Focus::DESCR => self.room.descr_scroll_pos.scroll_up(),
											Focus::OUTPUT => self.room.output_scroll_pos.scroll_up(),
											Focus::NPC => self.room.npc_list_state.select_previous(),
											Focus::INVENTORY => self.room.inventory_list_state.select_previous(),
											Focus::EXITS => self.room.exits_list_state.select_previous(),
											_ => {}
										}
									}
									KeyCode::Left => {
										match self.room.focus {
											Focus::CHAT => {
												self.chat.channel = match self.chat.channel {
													Channels::GLOBAL => Channels::GROUP,
													Channels::GROUP => Channels::ROOM,
													Channels::ROOM => Channels::GLOBAL
												}
											},
											_ => {}
										}
									}
									KeyCode::Right => {
										match self.room.focus {
											Focus::CHAT => {
												self.chat.channel = match self.chat.channel {
													Channels::GLOBAL => Channels::ROOM,
													Channels::GROUP => Channels::GLOBAL,
													Channels::ROOM => Channels::GROUP
												}
											},
											_ => {}
										}
									}
									KeyCode::Tab => {
										let current_index = Focus::iterator()
										.position(|f|f == &self.room.focus)
										.unwrap_or(0);
										
										let next_index = (current_index + 1) % Focus::iterator().len();
										self.room.focus = Focus::iterator().nth(next_index).unwrap().clone();
										self.room.exits_list_state.select(None);
										self.room.inventory_list_state.select(None);
										self.room.npc_list_state.select(None);
										match self.room.focus {
											Focus::EXITS => self.room.exits_list_state.select_first(),
											Focus::INVENTORY => self.room.inventory_list_state.select_first(),
											Focus::NPC => self.room.npc_list_state.select_first(),
											_ => {}
										}
									}
									KeyCode::Enter => {
										match self.room.focus {
											Focus::COMMAND => {
												let command = self.room.text_area.lines().join("");
												let split_command: Vec<&str> = command.split(" ").collect();
												// if ["TALK", "DROP", "TAKE", "LOOK", "MOVE", "WHO", "CHAT", "GROUP", "STATUS", "ATTACK", "INVENTORY", "QUEST"].contains(&split_command[0].to_uppercase().as_str()) {
												let _ = self.tx_to_serv.try_send(split_command.join(" ") + "\n");
												if !["CHAT"].contains(&split_command[0].to_uppercase().as_str()) {
													self.output.push_back(format!("\n> {}", split_command.join(" ")));
													self.room.output_scroll_pos.scroll_to_bottom();
												}
												// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
												// 	let _ = writeln!(file, "RECU (State {:?}) : {:#?}", self.state, split_command.join(" "));}
												if ["GROUP", "CHAT"].contains(&split_command[0].to_uppercase().as_str()){
													find_action(&split_command[..2].join(" "), self, Some(split_command[2..].join(" ")));
												} else {
													find_action(split_command[0], self, None);
												}
												// } else {
												// 	self.output.push_back("Unknown command.".to_string());
												// }
												self.room.text_area.clear();
											}
											Focus::EXITS => {
												if let Some(index) = self.room.exits_list_state.selected_mut() {
													if let Some(exit) = self.room.room_view.exits.get(*index) {
														let direction = match exit {
															Exits::East { .. } => "East",
															Exits::West { .. } => "West",
															Exits::North { .. } => "North",
															Exits::South { .. } => "South"
														};
														let _ = self.tx_to_serv.try_send(format!("MOVE {}\n", direction));
														self.action = PendingAction::Move;
													}
												}
											}
											Focus::NPC => {
												if let Some(index) = self.room.npc_list_state.selected_mut() {
													if let Some(selected_npc) = self.room.npcs.get(*index) {
														let _ = self.tx_to_serv.try_send(format!("TALK {}\n", selected_npc));
														self.action = PendingAction::Talk(selected_npc.clone());
													}
												}
											}
											Focus::INVENTORY => {todo!()}
											_ => {}
										}
									}
									_ => {}
								}
							}
						}
						States::InDiscuss(name, _) => {
							match key.code {
								KeyCode::Enter => {
									if let Some(new_sentence) = self.room.dialogs.pop_front() {
										self.state = States::InDiscuss(name.to_string(), new_sentence);
										self.message = String::new();
										self.counter = 0;
									} else {
										self.message = String::new();
										self.counter = 0;
										self.state = States::InGame;
										self.room.focus = Focus::COMMAND;
									}
								}
								_ => {}
							}
						}
						_ => {}
				}
			}
			Event::Mouse(event) => {
				if event.kind == MouseEventKind::Down(MouseButton::Left) {
					self.click = !self.click;
					self.error = false;
				}
			}
			_ => {}
		}
	}
	Ok(())
	}

	pub fn process_network(&mut self) {
		loop {
			match self.rx_from_serv.try_recv() {
				Ok(msg) => {
					// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
					// 			let _ = writeln!(file, "all (State {:?}) : {:#?}", self.state, msg);}
					if self.state == States::ServerWait {
						if msg.contains("OK hello proto") {self.state = States::Login};
					} else {
						let stream = Deserializer::from_str(&msg).into_iter::<ServerEvent>();
						for result in stream {
							if let Ok(server_event) = result {
								// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
								// 	let _ = writeln!(file, "ok (State {:?}) : {:#?}", self.state, server_event);}
								if server_event.event_type == "Response" {
									response_handling(self, &server_event);
								}
								if server_event.event_type == "Event" {
									if let Some(invite) = server_event.invite {
										self.group.invitation = Some(Invitation{sender: invite.sender, group_name: invite.group_name})
									}
									if let Some(new) = server_event.join {
										self.group.grouplist.push(new);
									}
									if let Some(leaver) = server_event.leave {
										self.group.grouplist.retain(|x| x != &leaver);
									}
									if let Some(msg) = server_event.chat {
										let channel = match msg.scope.as_str(){
										"ROOM" => &mut self.chat.room_messages,
										"GLOBAL" => &mut self.chat.global_messages,
										"GROUP" => &mut self.chat.group_messages,
										_ => continue
										};
										let text: String = format!("[{}] {}",msg.sender,msg.body);
										channel.push_back(text);

										if self.room.focus != Focus::CHAT {
											self.room.chat_scroll_pos.scroll_to_bottom();
										}
										// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
										// 	let _ = writeln!(file, "ok (State {:?}) : {:#?}", self.state, channel.);}
									}
								}
							}else {
								// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
								// 	let _ = writeln!(file, "Pas ok (State {:?}) : {:#?}", self.state, msg);}
							}
						}
					}
				}
				Err(TryRecvError::Empty) => break,
				Err(TryRecvError::Disconnected) => {
					self.quit = true;
					self.state = States::ServerError("Server not found".to_string());
					break;
				}
			}
		}
	}
}
