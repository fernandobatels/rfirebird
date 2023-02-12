//! Row definition and API

use byteorder::{ByteOrder, LittleEndian};
use std::fmt;

use crate::column::*;
use crate::Error;

/// Table row
pub struct Row {
    pub raw: Vec<Vec<u8>>,
    pub values: Vec<Option<Value>>,
}

impl Row {
    /// Load and prepare the row
    pub fn load(columns: &Vec<Column>, rec_data: Vec<u8>) -> Result<Self, Error> {
        let mut raw = vec![];
        let mut values = vec![];

        let mut readed = 4;
        for (icol, col) in columns.iter().enumerate() {
            if col.computed {
                values.push(None);
                continue;
            }

            let mut start = readed;
            let mut end = readed + col.size + (col.scale.abs() as usize);

            // 2 bytes for varying size info + regular field size
            if col.tp == ColumnType::Varchar {
                end = end + 2;
            }

            // we always need even char fields size
            if (col.tp == ColumnType::Varchar || col.tp == ColumnType::Char) && col.size % 2 != 0 {
                end = end + 1;
            }

            if col.tp == ColumnType::Timestamp {
                // If the timestamp start with 0000, we shift 2 position
                if rec_data[start..start + 4] == [0, 0, 0, 0] {
                    start = start + 2;
                    end = end + 2;
                }

                // If the previous and the next columns isn't an
                // other timestamp, we get more 2 positions
                let next = match columns.get(icol + 1) {
                    Some(ncol) => &ncol.tp,
                    None => &ColumnType::Timestamp,
                };
                let prev = match columns.get(icol - 1) {
                    Some(ncol) => &ncol.tp,
                    None => &ColumnType::Integer,
                };
                if next != &ColumnType::Timestamp && prev != &ColumnType::Timestamp {
                    end = end + 2;
                }
            }

            if end > rec_data.len() {
                end = rec_data.len();
            }

            let bcol = &rec_data[start..end];
            raw.push(bcol.to_vec());

            let val = match col.tp {
                ColumnType::Varchar => parse_varchar(bcol.to_vec())
                    .map_err(|e| Error::Other(format!("Parsing {} as varchar: {}", col.name, e)))?,
                ColumnType::Char => parse_char(col.size, bcol.to_vec())
                    .map_err(|e| Error::Other(format!("Parsing {} as char: {}", col.name, e)))?,
                ColumnType::Integer if col.scale == 0 => parse_integer(bcol.to_vec())
                    .map_err(|e| Error::Other(format!("Parsing {} as integer: {}", col.name, e)))?,
                ColumnType::Smallint => parse_smallinteger(bcol.to_vec()).map_err(|e| {
                    Error::Other(format!("Parsing {} as small integer: {}", col.name, e))
                })?,
                _ => None,
            };
            values.push(val);

            readed = end;
        }

        Ok(Self { values, raw })
    }
}

/// Cell value of a row
#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i32),
    SmallInt(i16),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(v) => write!(f, "{}", v),
            Value::Int(v) => write!(f, "{}", v),
            Value::SmallInt(v) => write!(f, "{}", v),
        }
    }
}

fn parse_smallinteger(bytes: Vec<u8>) -> Result<Option<Value>, String> {
    let it = LittleEndian::read_i16(&bytes);

    Ok(Some(Value::SmallInt(it)))
}

fn parse_integer(bytes: Vec<u8>) -> Result<Option<Value>, String> {
    let it = LittleEndian::read_i32(&bytes);

    Ok(Some(Value::Int(it)))
}

fn parse_char(size: usize, bytes: Vec<u8>) -> Result<Option<Value>, String> {
    let bytes = bytes[0..size].to_vec();
    let st = String::from_utf8(bytes)
        .map_err(|e| format!("Found column with an invalid UTF-8 string: {}", e))?;

    Ok(Some(Value::String(st)))
}

fn parse_varchar(bytes: Vec<u8>) -> Result<Option<Value>, String> {
    // varchar format:
    // {size}\0{byte}{byte}{byte}\0\0...

    let end = (bytes[0] + 2) as usize;
    if end > bytes.len() {
        return Err(format!(
            "Varchar size {} > {} buffer size",
            end,
            bytes.len()
        ));
    }
    let bytes = bytes[2..end].to_vec();

    let st = String::from_utf8(bytes)
        .map_err(|e| format!("Found column with an invalid UTF-8 string: {}", e))?;

    Ok(Some(Value::String(st)))
}
