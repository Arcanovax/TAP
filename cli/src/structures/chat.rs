use std::collections::VecDeque;

use crate::enums::channels::Channels;

pub struct Chat {
    pub channel: Channels,
    pub global_messages: VecDeque<String>,
    pub room_messages: VecDeque<String>,
    pub group_messages: VecDeque<String>,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            channel: Channels::Global,
            global_messages: VecDeque::new(),
            room_messages: VecDeque::new(),
            group_messages: VecDeque::new(),
        }
    }
}
