//! Firebird data page representation

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ptr;

use crate::Error;
use crate::page::*;
use crate::record::*;

/// Data Page
///
/// A data page belongs exclusively to a single table. The page starts
/// off, as usual, with the standard page header and is followed by an
/// array of pairs of unsigned two byte values representing the 'table
/// of contents' for this page. This array fills from the top of the
/// page (lowest address, increasing) while the actual data it points to
/// is stored on the page and fills from the bottom of the page (highest
/// address, descending).
#[derive(Debug, Clone)]
pub struct DataPage {
    pub pag: Page,
    /// Sequence number for this page in the list of pages assigned to this table within the database. The first page of any table has sequence zero.
    pub sequence: u32,
    /// The relation number for this table. This corresponds to RDB$RELATIONS.RDB$RELATION_ID.
    pub relation: u16,
    /// The number of records (or record fragments) on this page. In other words, the number of entries in the dpg_rpt array.
    pub count: u16,
    /// Counts upwards from the low address to the higher address as each new record fragment is added.
    pub records: Vec<DataPageRecord>,
    /// Raw content of this page
    pub raw: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct DataPageRep {
    pub pag: Page,
    pub sequence: u32,
    pub relation: u16,
    pub count: u16,
    pub records: [DataPageRecord; 512], // TODO: fix this hardcoded array size
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DataPageRecord {
    /// The offset on the page where the record fragment starts. If the value here is zero and the length is zero, then this is an unused array entry. The offset is from the start address of the page.
    pub offset: u16,
    /// The length of this record fragment in bytes.
    pub length: u16,
}

impl DataPage {
    /// Parse the DataPage from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Result<DataPage, Error> {
        if bytes[0] != 0x05 {
            return Err(Error::InvalidPage { tpe: bytes[0], expected: 0x05, desc: "data".to_string() });
        }

        let rdata: DataPageRep = unsafe { ptr::read(bytes.as_ptr() as *const _) };

        if rdata.count > 512 {
            return Err(Error::Overflow { limit: 512, value: rdata.count as usize, msg: "supported records".to_string() });
        }

        let mut records = rdata.records.to_vec();
        records.truncate(rdata.count as usize);

        let data = DataPage {
            pag: rdata.pag,
            sequence: rdata.sequence,
            relation: rdata.relation,
            count: rdata.count,
            records,
            raw: bytes,
        };

        Ok(data)
    }

    /// Load all data pages of buffer
    pub fn load(
        header: HeaderPage,
        buffer: &mut BufReader<File>,
    ) -> Result<Vec<DataPage>, Error> {
        let mut pgt = [0 as u8; 1];
        let mut pages = vec![];

        while buffer.read(&mut pgt)? > 0 {
            // We only need the Data Page — Type 0x05
            if pgt[0] == 0x05 {
                buffer.seek_relative(-1)?;

                let mut poip = vec![0 as u8; header.page_size.into()];
                buffer.read_exact(&mut poip)?;

                let data = DataPage::from_bytes(poip.clone())?;
                pages.push(data);
            } else {
                buffer.consume((header.page_size - 1).into());
            }
        }

        Ok(pages)
    }

    /// Read all records of this data page
    pub fn get_records(&self) -> Result<Vec<RecordHeader>, Error> {
        let mut records = vec![];

        for idx in self.records.clone() {
            let orec = self.get_record(idx)?;
            if let Some(rec) = orec {
                records.push(rec);
            }
        }

        Ok(records)
    }

    /// Read a specific record
    pub fn get_record(&self, idx: DataPageRecord) -> Result<Option<RecordHeader>, Error> {
        if idx.length == 0 {
            return Ok(None);
        }

        let rpoip = self.raw[idx.offset as usize..(idx.offset + idx.length) as usize].to_vec();

        let rec = RecordHeader::from_bytes(rpoip)?;

        Ok(Some(rec))
    }
}
