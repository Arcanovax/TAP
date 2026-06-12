use std::{fs::OpenOptions, io, sync::mpsc::{Receiver, TryRecvError}};
use ratatui::{
	DefaultTerminal,
	Frame,
	crossterm::event::{
		self, Event::self, KeyCode, MouseButton, MouseEventKind
	},
};
use std::io::Write;
use tokio::sync::mpsc::Sender;

use crate::{
	draw_functions::{login::login_draw, wait_server::draw_wait}, enums::states::States, structures::room::Room
};

pub struct World {
	pub room: Option<Room>,
	pub quit: bool,
	pub message: String,
	pub counter: u32,
	pub click: bool,
	pub input: String,
	pub state: States,
	pub tx_to_serv: tokio::sync::mpsc::Sender<String>,
	pub rx_from_serv: std::sync::mpsc::Receiver<String>
}

impl World {

	pub fn new(tx_to_serv: Sender<String>, rx_from_serv: Receiver<String>) -> Self {
		Self {
			room: None,
			quit: false,
			message: String::from(""),
			counter: 0,
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
							}
							_ => {}
						}
					}
				},
				Event::Mouse(event) => {
					if event.kind == MouseEventKind::Down(MouseButton::Left) {
						self.click = !self.click;
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
					//Debug
					if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
						let _ = writeln!(file, "RECU (State {:?}) : {:?}", self.state, msg);
					}
					match self.state {
						States::ServerWait => {
							if msg.contains("OK hello proto") {self.state = States::Login};
						},
						States::Login => {
							self.input.clear();
							self.input = msg.trim().to_string();
						},
						_ => {}
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
