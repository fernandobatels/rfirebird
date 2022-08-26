//! Firebird table representation

use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use rsfbclient_core::FbError;

use crate::data::*;
use crate::page::*;

/// Basic reference of a table
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub is_system_table: bool,
    pub relation: u16,
    pages: Rc<Vec<DataPage>>,
}

impl Table {
    /// Load all tables of database
    pub fn load(header: HeaderPage, buffer: &mut BufReader<File>) -> Result<Vec<Table>, FbError> {
        let pages = Rc::new(DataPage::load(header, buffer)?);
        let mut tables = vec![];

        for data in pages.iter() {
            // RDB$RELATIONS table
            if data.relation == 6 {
                for rec in data.get_records()? {
                    let rec_data = rec.read()?;

                    if rec_data.len() < 72 {
                        continue;
                    }
                    // RDB$RELATION_NAME field
                    let bname = &rec_data[42..72];
                    let name = String::from_utf8_lossy(&bname).trim().to_string();
                    let is_system_table = name.to_lowercase().contains("$");

                    tables.push(Table {
                        name,
                        is_system_table,
                        relation: data.relation,
                        pages: pages.clone(),
                    })
                }
            }
        }

        Ok(tables)
    }

    /// Prepare the table for access its rows
    pub fn prepare<'a>(&'a self) -> Result<TablePreparated<'a>, FbError> {
        TablePreparated::load(&self)
    }
}

/// Preparated table for rows acesss
pub struct TablePreparated<'a> {
    table: &'a Table,
    pub columns: Vec<Column>,
}

impl<'a> TablePreparated<'a> {
    pub fn load(table: &'a Table) -> Result<Self, FbError> {
        let mut columns = vec![];

        for data in table.pages.iter() {
            // RDB$RELATIONS_FIELDS table
            if data.relation == 5 {
                for rec in data.get_records()? {
                    let rec_data = rec.read()?;

                    if rec_data.len() < 65 {
                        continue;
                    }

                    // RDB$RELATION_NAME field
                    let brname = &rec_data[35..65];
                    let rname = String::from_utf8_lossy(&brname).trim().to_string();

                    if rname != table.name {
                        continue;
                    }

                    // RDB$FIELD_NAME field
                    let bfname = &rec_data[4..34];
                    let fname = String::from_utf8_lossy(&bfname).trim().to_string();

                    // RDB$FIELD_POSITION field
                    let bposition = &rec_data[290..291];
                    let position = bposition[0] as u16;

                    columns.push(Column {
                        name: fname,
                        position,
                    });
                }
            }
        }

        Ok(TablePreparated { table, columns })
    }
}

/// Column definion
#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub position: u16,
}
