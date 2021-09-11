//! Firebird database representation

use std::io::{Read, BufReader};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;

use rsfbclient_core::FbError;

use crate::Table;
use crate::page::HeaderPage;

/// The Firebird database
pub struct Database {
    pub header: HeaderPage,
    buffer: Rc<RefCell<BufReader<File>>>
}

impl Database {

    /// Read the database from a buffer
    pub fn open(buffer: Rc<RefCell<BufReader<File>>>) -> Result<Database, FbError> {

        let header = {
            let mut tag = [0 as u8; 1024];
            buffer.borrow_mut().read_exact(&mut tag)?;

            HeaderPage::from_bytes(tag)?
        };

        Ok(Self {
            header,
            buffer
        })
    }

    /// Read the database from a file with RO mode
    pub fn open_file(fpath: &str) -> Result<Database, FbError> {
        let f = File::open(fpath)
            .map_err(|e| e.to_string())?;
        let bfr = BufReader::new(f);

        Database::open(Rc::new(RefCell::new(bfr)))
    }

    pub fn tables(&mut self) -> Result<Vec<Table>, FbError> {
        Table::all()
    }
}
