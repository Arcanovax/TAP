use ratatui::widgets::ListState;
use ratatui_textarea::TextArea;
use serde::Deserialize;
use tui_widgets::scrollview::ScrollViewState;

use crate::{enums::{focus::Focus}, structures::room_view::RoomView};

#[derive(Deserialize, Debug)]
pub struct Room<'a> {
	#[serde(rename = "room")]
	pub room_view: RoomView,
	// pub id: String,
    // pub name: String,
    // pub exits: Vec<Exits>,
    // pub description: String,
    pub npcs: Vec<String>,
    pub items: Vec<String>,
    pub players: Vec<String>,

	#[serde(skip)]
	pub focus: Focus,
	// #[serde(skip)]
	// pub available_focus: Vec<Focus>,
	#[serde(skip)]
	pub chat_scroll_pos: ScrollViewState,
	#[serde(skip)]
	pub output_scroll_pos: ScrollViewState,
	#[serde(skip)]
	pub descr_scroll_pos: ScrollViewState,
	#[serde(skip)]
	pub npc_list_state: ListState,
	#[serde(skip)]
	pub inventory_list_state: ListState,
	#[serde(skip)]
	pub exits_list_state: ListState,
	#[serde(skip)]
	pub text_area: TextArea<'a>,
}

impl Room<'_> {
	pub fn new() -> Self {
		Room {
			room_view: RoomView::new(),
			npcs: Vec::new(),
			items: Vec::new(),
			players: Vec::new(),
			focus: Focus::COMMAND,
			// available_focus: vec![Focus::COMMAND, Focus::CHAT, Focus::NPC, Focus::OUTPUT, Focus::DESCR, Focus::INVENTORY, Focus::EXITS],
			chat_scroll_pos: ScrollViewState::new(),
			output_scroll_pos: ScrollViewState::new(),
			descr_scroll_pos: ScrollViewState::new(),
			npc_list_state: ListState::default(),
			inventory_list_state: ListState::default(),
			exits_list_state: ListState::default(),
			text_area: TextArea::default()
		}
	}
}