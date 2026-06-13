pub struct Chat {
    current_input: String,
    sended_messages: Vec<String>,
    pub is_active: bool,
	prev: i32,
    channel: i32,
    pub global_messages: Vec<String>,
	pub room_messages: Vec<String>,
	pub group_messages: Vec<String>,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            sended_messages: Vec::new(),
            is_active: false,
			prev: 0,
            channel: 0,
            global_messages: Vec::new(),
			room_messages: Vec::new(),
			group_messages: Vec::new()
        }
    }
}