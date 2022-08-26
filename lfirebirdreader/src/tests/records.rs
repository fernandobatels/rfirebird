use crate::{Column, Database};
use rsfbclient_core::FbError;

#[test]
fn reading_some_row() -> Result<(), FbError> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "DEPARTMENT");
    assert!(table.is_some());
    let table = table.unwrap();

    let ptable = table.prepare()?;

    assert_eq!(7, ptable.columns.len());
    assert_eq!(
        Some(&Column {
            name: "DEPT_NO".to_string(),
            position: 0
        }),
        ptable.columns.first()
    );
    assert_eq!(
        Some(&Column {
            name: "PHONE_NO".to_string(),
            position: 6
        }),
        ptable.columns.last()
    );

    Ok(())
}
