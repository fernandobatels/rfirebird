//! Firebird table representation

use rsfbclient_core::FbError;

pub struct Table {
}

impl Table {

    pub fn all() -> Result<Vec<Table>, FbError> {

        Ok(vec![])
    }
}
