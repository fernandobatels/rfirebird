//! Row definition and API

use crate::Error;
use crate::column::Column;

/// Table row
pub struct Row {
    pub raw: Vec<Vec<u8>>
}

impl Row {
    /// Load and prepare the row
    pub fn load(columns: &Vec<Column>, rec_data: Vec<u8>) -> Result<Self, Error> {
        let mut raw = vec![];

        let mut readed = 4;
        for col in columns {
            let bcol = &rec_data[readed..(readed + col.size)];
            raw.push(bcol.to_vec());
            readed = readed + col.size;
        }

        Ok(Self {
            raw
        })
    }
}
