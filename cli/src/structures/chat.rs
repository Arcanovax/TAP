
use std::collections::VecDeque;

use crate::{enums::channels::Channels};

pub struct Chat {
    current_input: String,
    sended_messages: Vec<String>,
    pub is_active: bool,
	prev: i32,
    pub channel: Channels,
	pub scroll_bar: bool,
    pub global_messages: VecDeque<String>,
	pub room_messages: VecDeque<String>,
	pub group_messages: VecDeque<String>,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            sended_messages: Vec::new(),
            is_active: false,
			prev: 0,
            channel: Channels::GLOBAL,
			scroll_bar: false,
            global_messages: VecDeque::new(),
            room_messages: VecDeque::new(),
            group_messages: VecDeque::new(),
        }
    }
}