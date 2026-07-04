use std::{
	collections::{HashMap, VecDeque}, fs::OpenOptions, io::{self, Write}, sync::mpsc::{
		Receiver,
		TryRecvError
	}, time::{Duration, Instant}
};

use ratatui::{
	DefaultTerminal, Frame, crossterm::event::{
		self, Event, KeyCode
	}
};

use tokio::sync::mpsc::Sender;

use crate::{
	draw_functions::{
		discuss::draw_room_discuss, fights::draw_room_fight, login::login_draw, rooms::draw_room, wait_server::draw_wait
	}, enums::{
		actions::PendingAction, focus::Focus, states::States
	}, global_functions::{discuss_event::discuss_event, escape_handling::escape_handling, event_handling::event_handling, handle_mouse::handle_mouse, handle_local_events::idle_event, login_event::login_event, response_handling::response_handling}, structures::{
		chat::Chat, group::Group, items::Item, npc::NPC, player::Player, room::Room
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
	pub old_command: VecDeque<String>,
	pub index_command: usize,
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
			old_command: VecDeque::new(),
			index_command: 0,
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
			States::Idle => draw_room(self, frame),
			States::InFight { .. } => draw_room_fight(self, frame),
			States::InDiscuss(name, sentence) => draw_room_discuss(self, frame, name.to_string(), sentence.to_string()),
			States::Quit(step, prev_state, cancelled_instant) => {
				match cancelled_instant {
					Some(instant) => {
						if instant.elapsed() >= Duration::from_secs(2) {
							self.state = *prev_state.clone();
						} else {
							escape_handling(self, frame, step.clone(), cancelled_instant.clone());
						}
					}
					None => escape_handling(self, frame, step.clone(), cancelled_instant.clone()),
				}
			} 
			_ => {}
		}
	}

	fn handle_events(&mut self) -> io::Result<()> {
		if event::poll(std::time::Duration::from_millis(16))? {
			match event::read()? {
				Event::Key(key) => {
					if key.code == KeyCode::Esc {
						// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
						// 		let _ = writeln!(file, "all (State {:?}) : {:#?}", self.room.focus, self.state);}
						if self.room.focus == Focus::BAG {
							self.room.fight.bag = false;
							self.room.focus = Focus::COMMAND;
							self.room.bag = Vec::new();
						} else {
							match &self.state {
								States::Quit(step, prev_state, cancelled_instant) => self.state = States::Quit(*step + 1, Box::new(*prev_state.clone()), *cancelled_instant),
								_ => self.state = States::Quit(1, Box::new(self.state.clone()), None)
							}
						}
					} else {
						match &self.state {
							States::Login => login_event(key, self),
							States::Idle
							| States::InFight { .. } => idle_event(key, self),
							States::InDiscuss(name, _) => discuss_event(key, self, name.clone()),
							States::Quit( step, prev_state, ..) => {
								if key.code == KeyCode::Enter {
									self.state = States::Quit(*step, Box::new(*prev_state.clone()), Some(Instant::now()));
								}
							},
							_ => {}
					}
					}
			}
			Event::Mouse(event) => {
				handle_mouse(event, self);
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
