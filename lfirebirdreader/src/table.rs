//! Firebird table representation

use std::fs::File;
use std::io::BufReader;

use rsfbclient_core::FbError;

use crate::data::*;
use crate::page::*;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub is_system_table: bool,
}

impl Table {
    /// Load all tables of database
    pub fn load(header: HeaderPage, buffer: &mut BufReader<File>) -> Result<Vec<Table>, FbError> {
        let pages = DataPage::load(header, buffer)?;
        let mut tables = vec![];

        for data in pages {
            // RDB$RELATIONS table
            if data.relation == 6 {
                for rec in data.get_records()? {
                    let rec_data = rec.read()?;

                    if rec_data.len() < 73 {
                        continue;
                    }
                    // RDB$RELATION_NAME field
                    let bname = &rec_data[42..73];
                    let name = String::from_utf8_lossy(&bname).trim().to_string();
                    let is_system_table = name.to_lowercase().contains("$");

                    tables.push(Table {
                        name,
                        is_system_table,
                    })
                }
            }
        }

        Ok(tables)
    }
}
