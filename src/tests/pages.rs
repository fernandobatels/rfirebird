use crate::*;

#[test]
fn header_page() -> Result<(), Error> {
    let db = Database::open_file("dbs/employee.fdb")?;

    assert_eq!(0x01, db.header.pag.ptype);
    assert_eq!(8192u16, db.header.page_size);

    Ok(())
}
