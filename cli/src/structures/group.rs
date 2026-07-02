
pub struct Invitation {
	pub sender: String
}

pub struct Group {
	pub in_group: bool,
	pub invitation: Vec<Invitation>,
}

impl Group {
	pub fn new() -> Self {
		Self {
			in_group: false,
			invitation: Vec::new(),
			}
	}
}