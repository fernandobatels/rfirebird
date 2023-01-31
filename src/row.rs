//! Row definition and API

use byteorder::{ByteOrder, LittleEndian};

use crate::Error;
use crate::column::*;

/// Table row
pub struct Row {
    pub raw: Vec<Vec<u8>>,
    pub values: Vec<Option<Value>>
}

impl Row {
    /// Load and prepare the row
    pub fn load(columns: &Vec<Column>, rec_data: Vec<u8>) -> Result<Self, Error> {
        let mut raw = vec![];
        let mut values = vec![];

        let mut readed = 4;
        for col in columns {
            let mut start = readed;
            let mut end = readed + col.size;

            // TODO: get how varchar size really works
            if col.tp == ColumnType::Varchar {
                end = end + 2;

                if rec_data[start] == 0 {
                    start = start + 1;

                    if rec_data[start] != 0 {
                        end = end + 1;
                    }
                }
            }

            let bcol = &rec_data[start..end];
            raw.push(bcol.to_vec());

            let val = match col.tp {
                ColumnType::Varchar => parse_varchar(bcol.to_vec())?,
                ColumnType::Char => parse_char(bcol.to_vec())?,
                ColumnType::Integer => parse_integer(bcol.to_vec())?,
                _ => None
            };
            values.push(val);

            readed = end;
        }

        Ok(Self {
            values,
            raw
        })
    }
}

/// Cell value of a row
#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i32)
}

fn parse_integer(bytes: Vec<u8>) -> Result<Option<Value>, Error> {
    let it = LittleEndian::read_i32(&bytes);

    Ok(Some(Value::Int(it)))
}

fn parse_char(bytes: Vec<u8>) -> Result<Option<Value>, Error> {

    let st = String::from_utf8(bytes)
        .map_err(|e| Error::Other(format!("Found column with an invalid UTF-8 string: {}", e)))?;

    Ok(Some(Value::String(st)))
}

fn parse_varchar(bytes: Vec<u8>) -> Result<Option<Value>, Error> {
    // varchar format:
    // {size}\0{byte}{byte}{byte}\0\0...

    println!("?? {} {}", bytes[0], bytes.len());
    let end = (bytes[0] + 2) as usize;
    let bytes = bytes[2..end].to_vec();

    let st = String::from_utf8(bytes)
        .map_err(|e| Error::Other(format!("Found column with an invalid UTF-8 string: {}", e)))?;

    Ok(Some(Value::String(st)))
}
