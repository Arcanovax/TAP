use std::{io, sync::mpsc::Receiver};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Layout}, widgets::{Block, Borders}};
use tokio::sync::mpsc::Sender;

use crate::{
	enums::states::States,
	structures::room::Room
};

pub struct World {
	room: Option<Room>,
	quit: bool,
	message: String,
	state: States,
	tx_to_serv: tokio::sync::mpsc::Sender<String>,
	rx_from_serv: std::sync::mpsc::Receiver<String>
}

impl World {

	pub fn new(tx_to_serv: Sender<String>, rx_from_serv: Receiver<String>) -> Self {
		Self {
			room: None,
			quit: false,
			message: String::from("Choose your username: "),
			state: States::Login,
			tx_to_serv: tx_to_serv,
			rx_from_serv: rx_from_serv
		}
	}

	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.quit {
			terminal.draw(|frame| self.draw(frame))?;
		}
		Ok(())
	}

	pub fn draw(&self, frame:&mut Frame) {
		let layout = Layout::default()
		.direction(ratatui::layout::Direction::Vertical)
		.constraints(vec![
			Constraint::Percentage(50),
			Constraint::Percentage(50),
		])
		.split(frame.area());
	let b = Block::default()
		.borders(Borders::ALL);
		for x in 0 .. layout.len() - 1 {
			frame.render_widget(&b, layout[x]);
		}
	}
}
