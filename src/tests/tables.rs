use crate::*;

#[test]
fn list_tables() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;
    assert_eq!(61, tables.len());

    // user tables
    let tables: Vec<Table> = tables.into_iter().filter(|t| !t.is_system_table).collect();
    assert_eq!(11, tables.len());

    let tab0 = &tables[0];
    assert_eq!("COUNTRY", tab0.name);
    assert_eq!(128, tab0.relation);

    let tab1 = &tables[1];
    assert_eq!("JOB", tab1.name);
    assert_eq!(129, tab1.relation);

    let tab10 = &tables[10];
    assert_eq!("SALES", tab10.name);
    assert_eq!(138, tab10.relation);

    Ok(())
}

#[test]
fn columns_of_table_department() -> Result<(), Error> {
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
            tp: ColumnType::Char,
            not_null: true
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
            tp: ColumnType::Bigint,
            not_null: false
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
            tp: ColumnType::Varchar,
            not_null: false
        }),
        ptable.columns.last()
    );

    Ok(())
}

#[test]
fn columns_of_table_employee() -> Result<(), Error> {
    let mut db = Database::open_file("dbs/employee.fdb")?;

    let tables = db.tables()?;

    let table = tables.into_iter().find(|t| t.name == "EMPLOYEE");
    assert!(table.is_some());
    let table = table.unwrap();

    let ptable = table.prepare()?;

    assert_eq!(11, ptable.columns.len());
    assert_eq!(
        Some(&Column {
            name: "EMP_NO".to_string(),
            position: 0,
            size: 2,
            source: "EMPNO".to_string(),
            scale: 0,
            tp: ColumnType::Smallint,
            not_null: true
        }),
        ptable.columns.first()
    );
    assert_eq!(
        Column {
            name: "FIRST_NAME".to_string(),
            position: 1,
            size: 15,
            source: "FIRSTNAME".to_string(),
            scale: 0,
            tp: ColumnType::Varchar,
            not_null: true
        },
        ptable.columns[1]
    );
    assert_eq!(
        Some(&Column {
            name: "FULL_NAME".to_string(),
            position: 10,
            size: 37,
            source: "RDB$9".to_string(),
            scale: 0,
            tp: ColumnType::Varchar,
            not_null: false
        }),
        ptable.columns.last()
    );

    Ok(())
}
