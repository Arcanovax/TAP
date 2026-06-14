use std::{
	io, sync::mpsc::{
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
use tokio::sync::mpsc::Sender;

use crate::{
	draw_functions::{
		login::login_draw,
		rooms::draw_room,
		wait_server::draw_wait
	},
	enums::{
		actions::PendingAction, focus::Focus, states::States
	},
	global_functions::{find_action::find_action, response_handling::response_handling},
	structures::{
		chat::Chat, player::Player, room::Room, server_event::ServerEvent
	}
};

pub struct World<'a> {
	pub room: Room<'a>,
	pub player: Player,
	pub quit: bool,
	pub message: String,
	pub chat: Chat,
	pub output: String,
	pub action: PendingAction,
	pub counter: u32,
	pub error: bool,
	pub click: bool,
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
			output: String::from(""),
			action: PendingAction::None,
			counter: 0,
			error: false,
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
		match self.state {
			States::ServerWait => draw_wait(frame),
			States::Login => login_draw(self, frame),
			States::InGame => {
				// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
				// 	let _ = writeln!(file, "RECU (State {:?}) : {:#?}", self.state, self.player.name);}
				draw_room(self, frame);
			},
			_ => {}
		}
	}

	fn handle_events(&mut self) -> io::Result<()> {
		if event::poll(std::time::Duration::from_millis(16))? {
			match event::read()? {
				Event::Key(key) => {
					if key.code == KeyCode::Esc { self.quit = true };
					if self.state == States::Login {
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
					} else if self.state == States::InGame {
						if self.room.focus == Focus::COMMAND && key.code != KeyCode::Tab && key.code != KeyCode::Enter{
								self.room.text_area.input(key);
						} else {
							match key.code {
								KeyCode::Down => {
									match self.room.focus {
										Focus::CHAT => self.room.chat_scroll_pos =  self.room.chat_scroll_pos.saturating_add(1),
										Focus::DESCR => self.room.descr_scroll_pos =  self.room.descr_scroll_pos.saturating_add(1),
										Focus::OUTPUT => self.room.output_scroll_pos =  self.room.output_scroll_pos.saturating_add(1),
										_ => {}
									}
									}
								KeyCode::Up => {
									match self.room.focus {
										Focus::CHAT => self.room.chat_scroll_pos =  self.room.chat_scroll_pos.saturating_sub(1),
										Focus::DESCR => self.room.descr_scroll_pos =  self.room.descr_scroll_pos.saturating_sub(1),
										Focus::OUTPUT => self.room.output_scroll_pos =  self.room.output_scroll_pos.saturating_sub(1),
										_ => {}
									}
								}
								KeyCode::Tab => {
									if !self.room.available_focus.is_empty() {
										let current_index = self.room.available_focus
										.iter()
										.position(|f|f == &self.room.focus)
										.unwrap_or(0);
										
										let next_index = (current_index + 1) % self.room.available_focus.len();
										self.room.focus = self.room.available_focus[next_index].clone();
										match self.room.focus {
											Focus::EXITS => self.room.exits_list_state.select_first(),
											Focus::INVENTORY => self.room.inventory_list_state.select_first(),
											Focus::NPC => self.room.npc_list_state.select_first(),
											_ => {}
										}
									}
								}
								KeyCode::Enter => {
									match self.room.focus {
										Focus::COMMAND => {
											let command = self.room.text_area.lines().join("");
											let split_command: Vec<&str> = command.split(" ").collect();
											if ["TALK", "DROP", "TAKE", "LOOK", "MOVE"].contains(&split_command[0].to_uppercase().as_str()) {
												let _ = self.tx_to_serv.try_send(split_command.join(" ") + "\n");
												// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
												// 	let _ = writeln!(file, "RECU (State {:?}) : {:#?}", self.state, split_command.join(" "));}
												find_action(split_command[0], self);
											} else {
												self.output += "Unknown command.";
											}
										}
										_ => {}
									}
								}
								_ => {}
							}
						}
					}
				},
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
						if let Ok(server_event) = serde_json::from_str::<ServerEvent>(&msg) {
							// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
							// 	let _ = writeln!(file, "ok (State {:?}) : {:#?}", self.state, server_event);}
							if server_event.event_type == "Response" {
								response_handling(self, &msg, &server_event);
							}
						}else {
							// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
							// 	let _ = writeln!(file, "Pas ok (State {:?}) : {:#?}", self.state, msg);}
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
