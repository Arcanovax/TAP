use uuid::Uuid;

pub struct Group {
    pub id: Uuid,
    pub players: Vec<Uuid>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            id: Uuid::new_v4(),
            players: Vec::new(),
        }
    }

    pub fn get_group_size(&self) -> usize {
        self.players.len()
    }
}
