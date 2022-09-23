use crate::{Column, ColumnType, Database};
use rsfbclient_core::FbError;

#[test]
fn columns_of_table() -> Result<(), FbError> {
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
            position: 0,
            size: 3,
            source: "DEPTNO".to_string(),
            scale: 0,
            tp: ColumnType::Char
        }),
        ptable.columns.first()
    );
    assert_eq!(
        Column {
            name: "BUDGET".to_string(),
            position: 4,
            size: 8,
            source: "BUDGET".to_string(),
            scale: -2,
            tp: ColumnType::Bigint
        },
        ptable.columns[4]
    );
    assert_eq!(
        Some(&Column {
            name: "PHONE_NO".to_string(),
            position: 6,
            size: 20,
            source: "PHONENUMBER".to_string(),
            scale: 0,
            tp: ColumnType::Varchar
        }),
        ptable.columns.last()
    );

    Ok(())
}

#[test]
fn reading_some_row() -> Result<(), FbError> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "COUNTRY");
    assert!(table.is_some());
    let table = table.unwrap();

    let mut ptable = table.prepare()?;

    let row1 = ptable.read()?;
    assert_eq!(Some(vec![
        vec![0x3, 0x0, 0x55, 0x53, 0x41, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], // USA
        vec![0x0, 0x0, 0x0, 0x06, 0x0, 0x44, 0x6F, 0x6C, 0x6C, 0x61] // Dollar
    ]), row1);

    let row2 = ptable.read()?;
    assert_eq!(Some(vec![
        vec![0x07, 0x00, 0x45, 0x6E, 0x67, 0x6C, 0x61, 0x6E, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // England
        vec![0x00, 0x00, 0x00, 0x05, 0x00, 0x50, 0x6F, 0x75, 0x6E, 0x64] // Pound
    ]), row2);

    for _ in 1..14 {
        let _ = ptable.read()?;
    }

    let row16 = ptable.read()?;
    assert_eq!(Some(vec![
        vec![0x07, 0x00, 0x52, 0x6F, 0x6D, 0x61, 0x6E, 0x69, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Romania
        vec![0x00, 0x00, 0x00, 0x04, 0x00, 0x52, 0x4C, 0x65, 0x75, 0x00] // RLeu
    ]), row16);

    let row17 = ptable.read()?;
    assert_eq!(None, row17);

    Ok(())
}
