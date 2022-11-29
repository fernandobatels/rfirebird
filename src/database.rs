//! Firebird database representation

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::rc::Rc;

use crate::Error;
use crate::page::HeaderPage;
use crate::Table;

/// The Firebird database
pub struct Database {
    pub header: HeaderPage,
    buffer: Rc<RefCell<BufReader<File>>>,
}

impl Database {
    /// Read the database from a buffer
    pub fn open(buffer: Rc<RefCell<BufReader<File>>>) -> Result<Database, Error> {
        let header = {
            let mut tag = [0 as u8; 1024];
            buffer.borrow_mut().read_exact(&mut tag)?;

            HeaderPage::from_bytes(tag)?
        };

        buffer
            .borrow_mut()
            .consume((header.page_size - 1024).into());

        Ok(Self { header, buffer })
    }

    /// Read the database from a file with RO mode
    pub fn open_file(fpath: &str) -> Result<Database, Error> {
        let f = File::open(fpath).map_err(|e| e.to_string())?;
        let bfr = BufReader::new(f);

        Database::open(Rc::new(RefCell::new(bfr)))
    }

    pub fn tables(&mut self) -> Result<Vec<Table>, Error> {
        Table::load(self.header, &mut self.buffer.borrow_mut())
    }
}
