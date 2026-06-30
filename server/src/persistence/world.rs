use crate::{persistence::tables::WORLD, structures::game::World};
use redb::{Database, ReadableDatabase};

pub fn save_world(db: &Database, world: &World) -> Result<(), redb::Error> {
    let txn = db.begin_write()?;
    {
        let mut table = txn.open_table(WORLD)?;
        table.insert("world", world)?;
    }
    txn.commit()?;
    Ok(())
}

pub fn load_world(db: &Database) -> Result<Option<World>, redb::Error> {
    let txn = db.begin_read()?;
    let table = match txn.open_table(WORLD) {
        Ok(table) => table,
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let world = table.get("world")?.map(|guard| guard.value());
    Ok(world)
}
