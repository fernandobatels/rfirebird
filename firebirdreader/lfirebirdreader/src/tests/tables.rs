
use crate::Database;
use rsfbclient_core::FbError;

#[test]
fn list_tables() -> Result<(), FbError>{

    let mut db = Database::open()?;

    db.tables()?;

    Ok(())
}
