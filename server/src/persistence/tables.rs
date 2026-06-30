use crate::{
    persistence::bincode::Bincode,
    structures::{game::World, player::Player},
};
use redb::TableDefinition;

pub const PLAYERS: TableDefinition<&str, Bincode<Player>> = TableDefinition::new("players");
pub const WORLD: TableDefinition<&str, Bincode<World>> = TableDefinition::new("world");
