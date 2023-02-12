//! Firebird raw reader

use tabled::{Tabled, Table as TabledTable, Style, builder::Builder};
use argopt::{subcmd, cmd_group};

use rfirebird::{Database, ColumnType, Error};

#[cmd_group(commands = [tables, columns, rows])]
fn main() -> Result<(), Error> {
}

/// Show all avaliable tables of the database
#[subcmd]
fn tables(
    file: String,
    /// Show system tables
    #[opt(long, default_value = "y")]
    system_tables: String
) -> Result<(), Error> {
    let mut db = Database::open_file(&file)?;

    let tables = db.tables()?;

    let data = tables.iter()
        .filter(|t| !t.is_system_table || system_tables == "y")
        .map(|t| TablePrintable {
            name: t.name.clone(),
            is_system_table: t.is_system_table,
            relation: t.relation
        });

    let printable = TabledTable::new(data)
        .with(Style::psql());

    println!("{}", printable);

    Ok(())
}

/// Show columns of a database table
#[subcmd]
fn columns(
    file: String,
    table: String
) -> Result<(), Error> {

    let mut db = Database::open_file(&file)?;

    let tables = db.tables()?;

    let otable = tables.into_iter()
        .find(|t| t.name.to_lowercase() == table.to_lowercase().trim());

    if let Some(table) = otable {

        let ptable = table.prepare()?;

        let data = ptable.columns.iter()
            .map(|c| ColumnPrintable {
                name: c.name.clone(),
                position: c.position,
                size: c.size,
                tp: c.tp.clone(),
                scale: c.scale,
                is_not_null: c.not_null,
                is_computed: c.computed
            });

        let printable = TabledTable::new(data)
            .with(Style::psql());

        println!("{}", printable);

        return Ok(());
    }

    return Err(Error::from("Table not found"));
}

/// Show all rows values of a database table
#[subcmd]
fn rows(
    file: String,
    table: String
) -> Result<(), Error> {

    let mut db = Database::open_file(&file)?;

    let tables = db.tables()?;

    let otable = tables.into_iter()
        .find(|t| t.name.to_lowercase() == table.to_lowercase().trim());

    if let Some(table) = otable {

        let mut ptable = table.prepare()?;
        let mut builder = Builder::default();

        let columns = ptable.columns.iter()
            .map(|c| c.name.clone());
        builder.set_columns(columns);

        while let Some(row) = ptable.read()? {
            let mut prow = vec![];

            for cval in row.values {
                prow.push(match cval {
                    Some(val) => val.to_string(),
                    None => "".to_string()
                });
            }

            builder.add_record(prow);
        }

        let printable = builder.build()
            .with(Style::psql());

        println!("{}", printable);

        return Ok(());
    }

    return Err(Error::from("Table not found"));
}

#[derive(Tabled)]
struct TablePrintable {
    pub name: String,
    pub is_system_table: bool,
    pub relation: u16,
}

#[derive(Tabled)]
pub struct ColumnPrintable {
    pub position: usize,
    pub name: String,
    pub size: usize,
    #[tabled(rename = "type")]
    pub tp: ColumnType,
    pub scale: i16,
    pub is_not_null: bool,
    pub is_computed: bool
}
