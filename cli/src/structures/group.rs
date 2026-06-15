pub struct Invitation {
	pub sender: String,
	pub group_name: String
}

pub struct Group {
	pub is_active: bool,
	pub in_group: bool,
	pub typed: String,
	pub chat_is_active: bool,
	pub invitation: Option<Invitation>,
	pub name: String,
	pub grouplist: Vec<String>,
	pub invite_info: Option<InviteInfo>
}

pub struct InviteInfo{
	pub state: String,
	pub time: f64
}

impl Group {
	pub fn new() -> Self {
		Self {
			is_active: false,
			in_group: false,
			typed: String::new(),
			chat_is_active: false,
			invitation: None,
			name: String::new(),
			grouplist: Vec::new(),
			invite_info: None
			}
	}
}