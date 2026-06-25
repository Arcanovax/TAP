use std::collections::VecDeque;

use ratatui::widgets::ListState;
use ratatui_textarea::TextArea;
use serde::Deserialize;
use tui_widgets::scrollview::ScrollViewState;

use crate::{enums::{focus::Focus}, structures::{fight::Fight, room_view::RoomView}};

#[derive(Deserialize, Debug)]
pub struct Room<'a> {
    pub items: Vec<String>,
    pub npcs: Vec<String>,
    pub players: Vec<String>,
	pub room: RoomView,

	#[serde(skip)]
	pub focus: Focus,
	#[serde(skip)]
	pub fight: Fight,
	#[serde(skip)]
	pub dialogs: VecDeque<String>,
	#[serde(skip)]
	pub chat_scroll_pos: ScrollViewState,
	#[serde(skip)]
	pub discuss_scroll_pos: ScrollViewState,
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
			room: RoomView::new(),
			npcs: Vec::new(),
			items: Vec::new(),
			fight: Fight::new(),
			dialogs: VecDeque::new(),
			players: Vec::new(),
			focus: Focus::COMMAND,
			chat_scroll_pos: ScrollViewState::new(),
			discuss_scroll_pos: ScrollViewState::new(),
			output_scroll_pos: ScrollViewState::new(),
			descr_scroll_pos: ScrollViewState::new(),
			npc_list_state: ListState::default(),
			inventory_list_state: ListState::default(),
			exits_list_state: ListState::default(),
			text_area: TextArea::default()
		}
	}
}
