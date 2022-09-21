
use tabled::{Tabled, Table as TabledTable, Style};
use argopt::{subcmd, cmd_group};
use rsfbclient_core::FbError;

use rfirebird::{Database, Table};

#[cmd_group(commands = [tables])]
fn main() -> Result<(), FbError> {
}

/// Show all avaliable tables of the database
#[subcmd]
fn tables(
    file: String,
    /// Show system tables
    #[opt(long, default_value = "y")]
    system_tables: String
) -> Result<(), FbError> {
    let mut db = Database::open_file(&file)?;

    let tables = db.tables()?;

    let data = tables.iter()
        .filter(|t| !t.is_system_table || system_tables == "y")
        .map(|t| TablePrintable::new(t));

    let printable = TabledTable::new(data)
        .with(Style::psql())
        .to_string();

    println!("{}", printable);

    Ok(())
}

#[derive(Tabled)]
struct TablePrintable {
    pub name: String,
    pub is_system_table: bool,
    pub relation: u16,
}

impl TablePrintable {
    fn new(table: &Table) -> TablePrintable {
        TablePrintable {
            name: table.name.clone(),
            is_system_table: table.is_system_table,
            relation: table.relation
        }
    }
}
