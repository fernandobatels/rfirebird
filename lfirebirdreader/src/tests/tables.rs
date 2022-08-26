use crate::{Database, Table};
use rsfbclient_core::FbError;

#[test]
fn list_tables() -> Result<(), FbError> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;
    assert_eq!(61, tables.len());

    // user tables
    let tables: Vec<Table> = tables.into_iter().filter(|t| !t.is_system_table).collect();
    assert_eq!(11, tables.len());

    let tab0 = &tables[0];
    assert_eq!("COUNTRY", tab0.name);
    let tab1 = &tables[1];
    assert_eq!("JOB", tab1.name);
    let tab10 = &tables[10];
    assert_eq!("SALES", tab10.name);

    Ok(())
}
