//! Firebird table representation

use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use std::fmt;
use std::slice::Iter;
use byteorder::{ByteOrder, LittleEndian};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use rsfbclient_core::FbError;

use crate::data::*;
use crate::page::*;

/// Basic reference of a table
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub is_system_table: bool,
    pub relation: u16,
    pages: Rc<Vec<DataPage>>,
}

impl Table {
    /// Load all tables of database
    pub fn load(header: HeaderPage, buffer: &mut BufReader<File>) -> Result<Vec<Table>, FbError> {
        let pages = Rc::new(DataPage::load(header, buffer)?);
        let mut tables = vec![];

        for data in pages.iter() {
            // RDB$RELATIONS table
            if data.relation == 6 {
                for rec in data.get_records()? {
                    let rec_data = rec.read()?;

                    if rec_data.len() < 72 {
                        continue;
                    }
                    // RDB$RELATION_NAME field
                    let bname = &rec_data[42..72];
                    let name = String::from_utf8_lossy(&bname).trim().to_string();
                    let is_system_table = name.to_lowercase().contains("$");
                    let relation = rec_data[32] as u16;

                    tables.push(Table {
                        name,
                        is_system_table,
                        relation,
                        pages: pages.clone(),
                    })
                }
            }
        }

        Ok(tables)
    }

    /// Prepare the table for access its rows
    pub fn prepare<'a>(&'a self) -> Result<TablePreparated<'a>, FbError> {
        TablePreparated::load(&self)
    }
}

/// Preparated table for rows acesss
pub struct TablePreparated<'a> {
    pages: Iter<'a, DataPage>,
    current_page: Option<&'a DataPage>,
    current_record_idx: usize,
    table: &'a Table,
    pub columns: Vec<Column>,
}

impl<'a> TablePreparated<'a> {
    pub fn load(table: &'a Table) -> Result<Self, FbError> {
        let mut columns = vec![];

        for data in table.pages.iter() {
            // RDB$RELATIONS_FIELDS table
            if data.relation == 5 {
                for rec in data.get_records()? {
                    let rec_data = rec.read()?;

                    if rec_data.len() < 66 {
                        continue;
                    }

                    // RDB$RELATION_NAME field
                    let brname = &rec_data[35..66];
                    let rname = String::from_utf8_lossy(&brname).trim().to_string();

                    if rname != table.name {
                        continue;
                    }

                    // RDB$FIELD_NAME field
                    let bfname = &rec_data[4..35];
                    let fname = String::from_utf8_lossy(&bfname).trim().to_string();

                    // RDB$FIELD_SOURCE field
                    let bsource = &rec_data[65..96];
                    let source = String::from_utf8_lossy(&bsource).trim().to_string();

                    let mut size = 0;
                    let mut scale = 0;
                    let mut tp = ColumnType::Smallint;

                    // Firebird have a specific table to storage
                    // the infos about columns types
                    for fdata in table.pages.iter() {
                        // RDB$FIELDS table
                        if fdata.relation == 2 {
                            for frec in fdata.get_records()? {
                                let frec_data = frec.read()?;

                                if frec_data.len() < 30 {
                                    continue;
                                }

                                let bfield = &frec_data[4..35];
                                let field = String::from_utf8_lossy(&bfield).trim().to_string();

                                if field != source {
                                    continue;
                                }

                                let bsize = &frec_data[120..121];
                                size = bsize[0] as usize;

                                let bscale = &frec_data[122..124];
                                scale = LittleEndian::read_i16(bscale);

                                let btype = &frec_data[124..126];
                                let ptype = LittleEndian::read_i16(btype);
                                tp = ColumnType::try_from(ptype)
                                    .map_err(|e| FbError::from(e.to_string()))?;
                            }
                        }
                    }

                    // RDB$FIELD_POSITION field
                    let bposition = &rec_data[290..291];
                    let position = bposition[0] as usize;

                    columns.push(Column {
                        name: fname,
                        position,
                        size,
                        source,
                        scale,
                        tp
                    });
                }
            }
        }

        let pages = table.pages.iter();

        Ok(TablePreparated {
            columns,
            table,
            pages,
            current_record_idx: 0,
            current_page: None
        })
    }

    /// Return a row from the table using a cursor
    pub fn read(&mut self) -> Result<Option<Vec<Vec<u8>>>, FbError> {

        if let Some(data) = self.current_page {
            self.current_record_idx = self.current_record_idx + 1;
            if self.current_record_idx >= data.records.len() {
                self.current_page = None;
            }
        }

        if self.current_page.is_none() {
            while let Some(data) = self.pages.next() {
                if data.relation == self.table.relation {
                    self.current_page = Some(data);
                    self.current_record_idx = 0;
                }
            }

            if self.current_page.is_none() {
                return Ok(None);
            }
        }

        if let Some(data) = self.current_page {
            let idx = data.records[self.current_record_idx];
            if let Some(rec) = data.get_record(idx)? {

                let rec_data = rec.read()?;

                let mut row = vec![];

                let mut readed = 4;
                for col in &self.columns {
                    let bcol = &rec_data[readed..(readed + col.size)];
                    row.push(bcol.to_vec());
                    readed = readed + col.size;
                }

                return Ok(Some(row));
            }
        }

        Ok(None)
    }
}

/// Column definion
#[derive(Debug, PartialEq, Clone)]
pub struct Column {
    pub name: String,
    pub position: usize,
    pub source: String,
    pub size: usize,
    pub scale: i16,
    pub tp: ColumnType
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
