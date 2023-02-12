//! Columns defitions and operations

use num_enum::TryFromPrimitive;
use std::fmt;

/// Column definion
#[derive(Debug, PartialEq, Clone)]
pub struct Column {
    pub name: String,
    pub position: usize,
    pub source: String,
    pub size: usize,
    pub scale: i16,
    pub tp: ColumnType,
    pub not_null: bool,
    pub computed: bool
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone)]
#[repr(i16)]
pub enum ColumnType {
    Smallint = 7,
    Integer = 8,
    Float = 10,
    Date = 12,
    Time = 13,
    Char = 14,
    Bigint = 16,
    DoublePrecision = 27,
    Timestamp = 35,
    Varchar = 37,
    Blob = 261
}

impl fmt::Display for ColumnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
