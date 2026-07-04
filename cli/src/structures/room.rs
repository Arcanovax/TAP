use std::{collections::VecDeque, fmt::Display};

use ratatui::widgets::ListState;
use ratatui_textarea::TextArea;
use serde::Deserialize;
use tui_widgets::scrollview::ScrollViewState;

use crate::{
    enums::focus::Focus,
    structures::{fight::Fight, room_view::RoomView},
};

#[derive(Deserialize, Debug)]
pub struct RoomPayload {
    pub items: Vec<String>,
    pub npcs: Vec<String>,
    pub players: Vec<String>,
    pub room: RoomView,
}

#[derive(Deserialize, Debug)]
pub struct Room<'a> {
    pub items: Vec<String>,
    pub npcs: Vec<String>,
    pub players: Vec<String>,
    pub room: RoomView,

    #[serde(skip)]
    pub focus: Focus,
    #[serde(skip)]
    pub bag: Vec<String>,
    #[serde(skip)]
    pub fight: Fight,
    #[serde(skip)]
    pub dialogs: VecDeque<String>,
    #[serde(skip)]
    pub chat_scroll_pos: ScrollViewState,
    #[serde(skip)]
    pub bag_state: ListState,
    #[serde(skip)]
    pub discuss_scroll_pos: ScrollViewState,
    #[serde(skip)]
    pub output_scroll_pos: ScrollViewState,
    #[serde(skip)]
    pub descr_scroll_pos: ScrollViewState,
    #[serde(skip)]
    pub npc_list_state: ListState,
    #[serde(skip)]
    pub sell_list_state: ListState,
    #[serde(skip)]
    pub buy_list_state: ListState,
    #[serde(skip)]
    pub inventory_list_state: ListState,
    #[serde(skip)]
    pub exits_list_state: ListState,
    #[serde(skip)]
    pub text_area: TextArea<'a>,
    #[serde(skip)]
    pub chat_text_area: TextArea<'a>,
}

impl Room<'_> {
    pub fn new() -> Self {
        Room {
            room: RoomView::new(),
            npcs: Vec::new(),
            items: Vec::new(),
            bag: Vec::new(),
            fight: Fight::new(),
            dialogs: VecDeque::new(),
            players: Vec::new(),
            focus: Focus::COMMAND,
            chat_scroll_pos: ScrollViewState::new(),
            discuss_scroll_pos: ScrollViewState::new(),
            output_scroll_pos: ScrollViewState::new(),
            bag_state: ListState::default(),
            descr_scroll_pos: ScrollViewState::new(),
            npc_list_state: ListState::default(),
            inventory_list_state: ListState::default(),
            sell_list_state: ListState::default(),
            buy_list_state: ListState::default(),
            exits_list_state: ListState::default(),
            text_area: TextArea::default(),
            chat_text_area: TextArea::default(),
        }
    }

    pub fn apply_update(&mut self, payload: RoomPayload) {
        self.items = payload.items;
        self.npcs = payload.npcs;
        self.players = payload.players;
        self.room = payload.room;
    }
}

impl Display for Room<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items_list: Vec<String> = Vec::new();
        for item in &self.items {
            items_list.push(format!("- {}", item));
        }
        if items_list.len() == 0 {
            items_list.push("Nothing.".to_string());
        }
        let mut npc_list: Vec<String> = Vec::new();
        for npc in &self.npcs {
            npc_list.push(format!("- {}", npc));
        }
        if npc_list.len() == 0 {
            npc_list.push("Nobody.".to_string());
        }
        let mut players_list: Vec<String> = Vec::new();
        for player in &self.players {
            players_list.push(format!("- {}", player));
        }
        if players_list.len() == 0 {
            players_list.push("You are alone.".to_string());
        }
        write!(
            f,
            "{}\nItems you can take:\n{}\nNPCS:\n{}\nPlayers:\n{}",
            self.room,
            items_list.join("\n"),
            npc_list.join("\n"),
            players_list.join("\n")
        )
    }
}
