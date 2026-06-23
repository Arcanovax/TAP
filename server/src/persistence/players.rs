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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_db;

    #[test]
    fn save_then_load_resturns_same_player() {
        let db = test_db();

        let mut player = Player::new("test".to_string());
        player.max_hp = 30;
        player.location = "room.test".to_string();

        let _ = save_player(&db, &player);
        let loaded = load_player(&db, &player.name).unwrap();

        assert_eq!(Some(player), loaded);
    }

    #[test]
    fn load_unknown_name_returns_none() {
        let db = test_db();
        let loaded = load_player(&db, "test").unwrap();

        assert_eq!(loaded, None);
    }
}
