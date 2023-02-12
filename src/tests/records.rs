use crate::*;

#[test]
fn reading_some_row_raw() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "COUNTRY");
    assert!(table.is_some());
    let table = table.unwrap();

    let mut ptable = table.prepare()?;

    let row1 = ptable.read()?;
    assert!(row1.is_some());
    assert_eq!(vec![
        vec![0x3, 0x0, 0x55, 0x53, 0x41, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], // USA
        vec![0x06, 0x0, 0x44, 0x6F, 0x6C, 0x6C, 0x61, 0x72, 0x0, 0x0, 0x0, 0x0] // Dollar
    ], row1.unwrap().raw);

    let row2 = ptable.read()?;
    assert!(row2.is_some());
    assert_eq!(vec![
        vec![0x07, 0x00, 0x45, 0x6E, 0x67, 0x6C, 0x61, 0x6E, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // England
        vec![0x05, 0x00, 0x50, 0x6F, 0x75, 0x6E, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00] // Pound
    ], row2.unwrap().raw);

    for _ in 1..14 {
        let _ = ptable.read()?;
    }

    let row16 = ptable.read()?;
    assert!(row16.is_some());
    assert_eq!(vec![
        vec![0x07, 0x00, 0x52, 0x6F, 0x6D, 0x61, 0x6E, 0x69, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Romania
        vec![0x04, 0x00, 0x52, 0x4C, 0x65, 0x75, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] // RLeu
    ], row16.unwrap().raw);

    let row17 = ptable.read()?;
    assert!(row17.is_none());

    Ok(())
}

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

#[test]
fn reading_some_row_typed_sales() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "SALES");
    assert!(table.is_some());
    let table = table.unwrap();

    let mut ptable = table.prepare()?;

    let row1 = ptable.read()?;
    assert!(row1.is_some());

    let row1 = row1.unwrap();
    assert_eq!(Some(Value::String("V91E0210".to_string())), row1.values[0]);
    assert_eq!(Some(Value::Int(1004)), row1.values[1]);
    assert_eq!(Some(Value::SmallInt(11)), row1.values[2]);
    assert_eq!(Some(Value::String("shipped".to_string())), row1.values[3]);
    assert_eq!(None, row1.values[4]); // timestamp not supported yet
    assert_eq!(None, row1.values[5]); // timestamp not supported yet
    assert_eq!(None, row1.values[6]); // timestamp not supported yet
    assert_eq!(Some(Value::String("y".to_string())), row1.values[7]);
    //assert_eq!(Some(Value::Int(10)), row1.values[8]);
    assert_eq!(None, row1.values[9]); // decimal not supported yet
    assert_eq!(None, row1.values[10]); // float not supported yet
    assert_eq!(Some(Value::String("hardware".to_string())), row1.values[11]);

    Ok(())
}

#[test]
fn reading_some_row_typed_salary_history() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "SALARY_HISTORY");
    assert!(table.is_some());
    let table = table.unwrap();

    let mut ptable = table.prepare()?;

    let row1 = ptable.read()?;
    assert!(row1.is_some());

    let row1 = row1.unwrap();
    assert_eq!(Some(Value::SmallInt(28)), row1.values[0]);
    assert_eq!(None, row1.values[1]); // timestamp not supported yet
    assert_eq!(Some(Value::String("admin2".to_string())), row1.values[2]);
    assert_eq!(None, row1.values[3]); // numeric not supported yet
    assert_eq!(None, row1.values[4]); // numeric not supported yet
    assert_eq!(None, row1.values[5]); // numeric not supported

    Ok(())
}

#[test]
fn reading_some_row_typed_employee() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "EMPLOYEE");
    assert!(table.is_some());
    let table = table.unwrap();

    let mut ptable = table.prepare()?;

    let row1 = ptable.read()?;
    assert!(row1.is_some());

    let row1 = row1.unwrap();
    assert_eq!(Some(Value::SmallInt(2)), row1.values[0]);
    assert_eq!(Some(Value::String("Robert".to_string())), row1.values[1]);
    assert_eq!(Some(Value::String("Nelson".to_string())), row1.values[2]);
    assert_eq!(Some(Value::String("250".to_string())), row1.values[3]);
    assert_eq!(None, row1.values[4]); // timestamp not supported yet
    assert_eq!(Some(Value::String("600".to_string())), row1.values[5]);
    assert_eq!(Some(Value::String("VP".to_string())), row1.values[6]);
    assert_eq!(Some(Value::SmallInt(2)), row1.values[7]);
    assert_eq!(Some(Value::String("USA".to_string())), row1.values[8]);
    assert_eq!(None, row1.values[9]); // numeric not supported yet
    assert_eq!(None, row1.values[10]); // computed not supported

    let row2 = ptable.read()?;
    assert!(row2.is_some());

    let row2 = row2.unwrap();
    assert_eq!(Some(Value::SmallInt(4)), row2.values[0]);
    assert_eq!(Some(Value::String("Bruce".to_string())), row2.values[1]);
    assert_eq!(Some(Value::String("Young".to_string())), row2.values[2]);
    assert_eq!(Some(Value::String("233".to_string())), row2.values[3]);
    assert_eq!(None, row2.values[4]); // timestamp not supported yet
    assert_eq!(Some(Value::String("621".to_string())), row2.values[5]);
    assert_eq!(Some(Value::String("Eng".to_string())), row2.values[6]);
    assert_eq!(Some(Value::SmallInt(2)), row2.values[7]);
    assert_eq!(Some(Value::String("USA".to_string())), row2.values[8]);
    assert_eq!(None, row2.values[9]); // numeric not supported yet
    assert_eq!(None, row2.values[10]); // computed not supported

    Ok(())
}
