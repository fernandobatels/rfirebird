//! Firebird table representation

use std::io::{Read, BufReader, BufRead};
use std::fs::File;

use rsfbclient_core::FbError;

use crate::page::*;
use crate::record::*;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub is_system_table: bool
}

impl Table {

    /// Load all tables of database
    pub fn load(header: HeaderPage, buffer: &mut BufReader<File>) -> Result<Vec<Table>, FbError> {

        let mut pgt = [0 as u8; 1];
        let mut tables = vec![];

        while buffer.read(&mut pgt)? > 0 {

            // We only need the Data Page — Type 0x05
            if pgt[0] == 0x05 {
                buffer.seek_relative(-1)?;

                let mut poip = vec![0 as u8; header.page_size.into()];
                buffer.read_exact(&mut poip)?;

                let data = DataPage::from_bytes(poip.clone())?;
                if data.relation == 6 { // Id of RDB$RELATIONS
                    for idx in data.records {
                        if idx.length == 0 {
                            continue;
                        }

                        let rpoip = poip.clone()[idx.offset as usize..(idx.offset + idx.length) as usize].to_vec();

                        let rec = RecordHeader::from_bytes(rpoip)?;
                        let rec_data = rec.read()?;

                        if rec_data.len() < 73 {
                            continue;
                        }
                        // RDB$RELATION_NAME
                        let bname = &rec_data[42..73];
                        let name = String::from_utf8_lossy(&bname)
                            .trim()
                            .to_string();
                        let is_system_table = name.to_lowercase().contains("$");

                        tables.push(Table {
                            name,
                            is_system_table
                        })
                    }
                }
            } else {
                buffer.consume((header.page_size - 1).into());
            }
        }

        Ok(tables)
    }
}
