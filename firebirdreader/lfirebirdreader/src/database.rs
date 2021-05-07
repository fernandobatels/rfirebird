//! Firebird database representation

use rsfbclient_core::FbError;

use crate::Table;

pub struct Database {
}

impl Database {

    pub fn open() -> Result<Database, FbError> {

        Ok(Self {
            
        })
    }

    pub fn tables(&mut self) -> Result<Vec<Table>, FbError> {
        Table::all()
    }
}
