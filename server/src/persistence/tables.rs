use crate::{persistence::bincode::Bincode, structures::player::Player};
use redb::TableDefinition;

pub const PLAYERS: TableDefinition<&str, Bincode<Player>> = TableDefinition::new("players");
