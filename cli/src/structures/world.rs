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
		discuss::draw_room_discuss, fights::draw_room_fight, login::login_draw, rooms::draw_room, wait_server::draw_wait
	}, enums::{
		actions::PendingAction, channels::Channels, exits::Exits, focus::Focus, states::States
	}, global_functions::{event_handling::event_handling, find_action::find_action, response_handling::response_handling}, structures::{
		chat::Chat, enn_attack::EnnAttack, fight::Fight, group::{Group, Invitation}, items::Item, npc::NPC, player::Player, room::Room, server_event::ServerEvent
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
			States::Idle => {
				// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
				// 	let _ = writeln!(file, "RECU (State {:?}) : {:#?}", self.state, self.player.name);}
				draw_room(self, frame);
			},
			States::InFight { .. } => {
				self.room.focus = Focus::COMMAND;
				draw_room_fight(self, frame);
			}
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
									// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
									// 	let _ = writeln!(file, "dir (State) {:#?}", self.input);}
									let _ = self.tx_to_serv.try_send(format!("CONNECT {}\n", self.input));
									self.action = PendingAction::Auth;
								}
								_ => {}
							}
						},
						States::Idle
						| States::InFight { .. } => {
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
												let _ = self.tx_to_serv.try_send(split_command.join(" ") + "\n");

												if !["CHAT"].contains(&split_command[0].to_uppercase().as_str()) {
													self.output.push_back(format!("\n> {}", split_command.join(" ")));
													self.room.output_scroll_pos.scroll_to_bottom();
												}

												find_action(split_command, self);
												self.room.text_area.clear();
											}
											Focus::EXITS => {
												if let Some(index) = self.room.exits_list_state.selected_mut() {
													if let Some(dir) = self.room.room.exits.keys().nth(*index) {
														let _ = self.tx_to_serv.try_send(format!("MOVE {}\n", dir));
														self.action = PendingAction::Move;
													} else {
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
										self.state = States::Idle;
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
					let answers = msg.lines();
					for answer in answers {
						let parts: Vec<&str> = answer.split_whitespace().collect();
						if parts.is_empty() { return; }
						if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
									let _ = writeln!(file, "all (State {:?}) : {:#?}", parts, msg);}
						match parts[0] {
							"OK" | "ERR" => response_handling(self, parts),
							"EVT" => event_handling(self, parts),
							_ => {}
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
