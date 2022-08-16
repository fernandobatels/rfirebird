//! Firebird table representation

use std::io::{Read, BufReader, BufRead};
use std::fs::File;

use rsfbclient_core::FbError;

use crate::page::*;

pub struct Table {
}

impl Table {

    /// Load all tables of database
    pub fn load(header: HeaderPage, buffer: &mut BufReader<File>) -> Result<Vec<Table>, FbError> {

        let mut pgt = [0 as u8; 1];

        while buffer.read(&mut pgt)? > 0 {

            // We only need the Data Page — Type 0x05
            if pgt[0] == 0x05 {
                buffer.seek_relative(-1)?;

                let mut poip = vec![0 as u8; header.page_size.into()];
                buffer.read_exact(&mut poip)?;

                let test = String::from_utf8_lossy(&poip);
                println!("{:?}", test);

            } else {
                buffer.consume((header.page_size - 1).into());
            }
        }

        Ok(vec![])
    }
}
