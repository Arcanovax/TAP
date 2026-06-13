use serde::Deserialize;

use crate::{enums::{exits::Exits, focus::Focus}};

#[derive(Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub exits: Vec<Exits>,
    pub description: String,
    pub npc: Vec<String>,
    pub items: Vec<String>,

	#[serde(skip)]
	pub focus: Focus,
	#[serde(skip)]
	pub available_focus: Vec<Focus>,
	#[serde(skip)]
	pub chat_scroll_pos: u16,
	#[serde(skip)]
	pub output_scroll_pos: u16,
	#[serde(skip)]
	pub descr_scroll_pos: u16,
}

impl Room {
	pub fn new() -> Self {
		Room {
			name: String::from(""),
			exits: Vec::new(),
			description: String::from(""),
			npc: Vec::new(),
			focus: Focus::NONE,
			available_focus: Vec::new(),
			chat_scroll_pos: 0,
			output_scroll_pos: 0,
			descr_scroll_pos: 0,
			items: Vec::new()
		}
	}
}