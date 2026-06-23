use crate::{persistence::tables::PLAYERS, structures::player::Player};
use redb::{Database, ReadableDatabase};

pub fn save_player(db: &Database, player: &Player) -> Result<(), redb::Error> {
    let txn = db.begin_write()?;
    {
        let mut table = txn.open_table(PLAYERS)?;
        table.insert(player.name.as_str(), player)?;
    }
    txn.commit()?;
    Ok(())
}

pub fn load_player(db: &Database, player_name: &str) -> Result<Option<Player>, redb::Error> {
    let txn = db.begin_read()?;
    let table = match txn.open_table(PLAYERS) {
        Ok(table) => table,
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let player = table.get(player_name)?.map(|guard| guard.value());
    Ok(player)
}
