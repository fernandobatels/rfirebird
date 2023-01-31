use crate::*;

#[test]
fn reading_some_row_typed() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "COUNTRY");
    assert!(table.is_some());
    let table = table.unwrap();

    let mut ptable = table.prepare()?;

    let row1 = ptable.read()?;
    assert!(row1.is_some());
    let row1 = row1.unwrap();
    assert_eq!(Some(Value::String("USA".to_string())), row1.values[0]);
    assert_eq!(Some(Value::String("Dollar".to_string())), row1.values[1]);

    let row2 = ptable.read()?;
    assert!(row2.is_some());
    let row2 = row2.unwrap();
    assert_eq!(Some(Value::String("England".to_string())), row2.values[0]);
    assert_eq!(Some(Value::String("Pound".to_string())), row2.values[1]);

    Ok(())
}

#[test]
fn reading_some_row_typed_costumer() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "CUSTOMER");
    assert!(table.is_some());
    let table = table.unwrap();

    let mut ptable = table.prepare()?;

    let row1 = ptable.read()?;
    assert!(row1.is_some());
    let row1 = row1.unwrap();
    assert_eq!(Some(Value::Int(1001)), row1.values[0]);
    Ok(())
}
