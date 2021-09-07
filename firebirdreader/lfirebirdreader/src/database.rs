//! Firebird database representation

use std::io::{Read, BufReader};
use std::fs::File;

use rsfbclient_core::FbError;

use crate::Table;
use crate::page::HeaderPage;

/// The Firebird database
pub struct Database {
    pub header: HeaderPage,
    raw: Vec<u8>
}

impl Database {

    /// Read the database from a buffer
    pub fn open(bf_reader: &mut BufReader<File>) -> Result<Database, FbError> {

        let header = {
            let mut tag = [0 as u8; 1024];
            bf_reader.read_exact(&mut tag)?;

            HeaderPage::from_bytes(tag)?
        };

        let mut raw = vec![];
        bf_reader.read_to_end(&mut raw)?;

        Ok(Self {
            header,
            raw
        })
    }

    /// Read the database from a file with RO mode
    pub fn open_file(fpath: &str) -> Result<Database, FbError> {
        let f = File::open(fpath)
            .map_err(|e| e.to_string())?;
        let mut bfr = BufReader::new(f);

        Database::open(&mut bfr)
    }

    pub fn tables(&mut self) -> Result<Vec<Table>, FbError> {
        Table::all()
    }
}
