use std::collections::VecDeque;

pub struct Invitation {
	pub sender: String,
	pub group_name: String
}

pub struct Group {
	pub in_group: bool,
	// pub typed: String,
	pub invitation: Vec<Invitation>,
	// pub name: String,
	// pub grouplist: VecDeque<String>,
	// pub invite_info: Option<InviteInfo>
}

pub struct InviteInfo{
	pub state: String,
	pub time: f64
}

impl Group {
	pub fn new() -> Self {
		Self {
			in_group: false,
			// typed: String::new(),
			invitation: Vec::new(),
			// name: String::new(),
			// grouplist: VecDeque::new(),
			// invite_info: None
			}
	}
}