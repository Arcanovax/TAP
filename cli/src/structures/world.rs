use std::{io, sync::mpsc::Receiver};
// use crate::structures::popup::Popup;
use ratatui::{
	DefaultTerminal,
	Frame,
	crossterm::event::{
		self, Event::{self, Mouse}, KeyCode, MouseButton, MouseEvent, MouseEventKind
	}, layout::{
		Constraint::{self, Length, Percentage},
		Direction::Vertical,
		Layout
	}, prelude::Stylize, style::{Color, Style}, text::{Line, Text}, widgets::{
		Block, BorderType, Borders, Paragraph, Wrap
	}
};
use tokio::sync::mpsc::Sender;
use tui_big_text::{BigText, PixelSize};

use crate::{
	draw_functions::login::{self, login_draw}, enums::states::States, structures::{popup::Popup, room::Room}
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
			state: States::Login,
			tx_to_serv: tx_to_serv,
			rx_from_serv: rx_from_serv
		}
	}

	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.quit {
			terminal.draw(|frame| self.draw(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}
	pub fn draw(&mut self, frame:&mut Frame) {
		if self.state == States::Login {
			login_draw(self, frame);
		}
	}

	fn handle_events(&mut self) -> io::Result<()> {
		if event::poll(std::time::Duration::from_millis(16))? {
			match event::read()? {
				Event::Key(key) => {
					if self.state == States::Login {
						match key.code {
							KeyCode::Esc => self.quit = true,
							KeyCode::Char(c) => {
								if self.input.len() < 20 {self.input.push(c);}
							},
							KeyCode::Backspace => { self.input.pop(); },
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
}
