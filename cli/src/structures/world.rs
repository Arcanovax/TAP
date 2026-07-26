use std::{
    collections::{HashMap, VecDeque},
    io::{self},
    sync::mpsc::{Receiver, TryRecvError},
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
};

use tokio::sync::mpsc::Sender;

use crate::{
    draw_functions::{
        discuss::draw_room_discuss, fights::draw_room_fight, login::login_draw, rooms::draw_room,
        trade::draw_trade, wait_server::draw_wait,
    },
    enums::{actions::PendingAction, states::States},
    global_functions::{
        discuss_event::discuss_event, escape_handling::escape_handling,
        event_handling::event_handling, handle_escape::handle_escape,
        handle_local_events::handle_global_events, handle_mouse::handle_mouse,
        login_event::login_event, parse_dungeon_id::parse_dungeon_id,
        response_handling::response_handling, trade_event::trade_event,
    },
    structures::{
        chat::Chat, group::Group, items::Item, npc::NPC, player::Player, room::Room,
        rooms_view::RoomsView,
    },
};

pub struct World<'a> {
    pub room: Room<'a>,
    pub rooms: HashMap<String, RoomsView>,
    pub player: Player,
    pub quit: bool,
    pub message: String,
    pub chat: Chat,
    pub output: VecDeque<String>,
    pub action: PendingAction,
    pub group: Group,
    pub counter: u32,
    pub dungeon: bool,
    pub index_sentence: usize,
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
    pub rx_from_serv: std::sync::mpsc::Receiver<String>,
}

impl World<'_> {
    pub fn new(tx_to_serv: Sender<String>, rx_from_serv: Receiver<String>) -> Self {
        Self {
            room: Room::new(),
            rooms: HashMap::new(),
            player: Player::new(),
            quit: false,
            message: String::from(""),
            chat: Chat::new(),
            output: VecDeque::new(),
            dungeon: false,
            old_command: VecDeque::new(),
            index_command: 0,
            action: PendingAction::None,
            counter: 0,
            index_sentence: 0,
            group: Group::new(),
            error: false,
            list_items: HashMap::new(),
            list_npcs: HashMap::new(),
            message_error: "".to_string(),
            click: false,
            input: "".to_string(),
            state: States::ServerWait,
            tx_to_serv,
            rx_from_serv,
        }
    }

    pub fn in_dungeon(&self) -> bool {
        parse_dungeon_id(&self.room.room.id).is_some()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            self.process_network();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        match &self.state {
            States::ServerWait => draw_wait(frame),
            States::Login => login_draw(self, frame),
            States::Idle => draw_room(self, frame),
            States::InFight { .. } => draw_room_fight(self, frame),
            States::InDiscuss(name, sentence) => {
                draw_room_discuss(self, frame, name.to_string(), sentence.clone())
            }
            States::Quit(step, prev_state, cancelled_instant) => match cancelled_instant {
                Some(instant) => {
                    if instant.elapsed() >= Duration::from_secs(2) {
                        self.state = *prev_state.clone();
                    } else {
                        escape_handling(self, frame, step.clone(), cancelled_instant.clone());
                    }
                }
                None => escape_handling(self, frame, step.clone(), cancelled_instant.clone()),
            },
            States::Trade(inventory, ..) => draw_trade(self, frame, inventory.clone()),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Esc {
                        handle_escape(self);
                    } else {
                        match &self.state {
                            States::Login => login_event(key, self),
                            States::Idle | States::InFight { .. } => {
                                handle_global_events(key, self)
                            }
                            States::InDiscuss(name, _) => discuss_event(key, self, name.clone()),
                            States::Quit(step, prev_state, ..) => {
                                if key.code == KeyCode::Enter {
                                    self.state = States::Quit(
                                        *step,
                                        Box::new(*prev_state.clone()),
                                        Some(Instant::now()),
                                    );
                                }
                            }
                            States::Trade(inventory, npc_id) => {
                                trade_event(self, key, inventory.clone(), npc_id.clone())
                            }
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
                        if parts.is_empty() {
                            return;
                        }
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
