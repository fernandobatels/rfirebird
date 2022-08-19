//! Firebird page's representation

use std::ptr;

use rsfbclient_core::FbError;

/// Standard Database Page Header
///
/// Every page in a database has a 16-byte standard page header.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Page {
    pub ptype: u8,
    flags: u8,
    reserved: u16,
    generation: u32,
    scn: u32,
    pageno: u32,
}

/// Database Page Header
///
/// The first page of the first file of a Firebird database is
/// a very important page. It holds data that describes the
/// database, where its other files are to be found,
/// shadow file names, database page size, ODS version
/// and so on.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HeaderPage {
    pub pag: Page,
    /// Page size of database
    pub page_size: u16,
    /// Version of on-disk structure
    ods_version: u16,
    /// Page number of PAGES relation
    pages: u32,
    /// Page number of next hdr page
    next_page: u32,
    /// Oldest interesting transaction
    oldest_transaction: u32,
    /// Oldest transaction thought active
    oldest_active: u32,
    /// Next transaction id
    next_transaction: u32,
    /// sequence number of file
    sequence: u16,
    /// Flag settings, see below
    flags: u16,
    /// Date/time of creation
    create_data: [i32; 2],
    /// Next attachment id
    attachment_id: u32,
    /// Event count for shadow synchronization
    shadow_count: i32,
    /// CPU database was created on
    cpu: u8,
    /// OS database was created under
    os: u8,
    /// Compiler of engine on which database was created
    cc: u8,
    /// Cross-platform database transfer compatibility flags
    compatibility_flags: u8,
    /// Update version of ODS
    ods_minor: u16,
    /// offset of HDR_end in page
    end: u16,
    /// Page buffers for database cache
    page_buffers: u32,
    /// Oldest snapshot of active transactions
    oldest_snapshot: u32,
    /// The amount of pages in files locked for backup
    backup_pages: i32,
    /// Page at which processing is in progress
    cpypt_page: u32,
    /// Name of plugin used to crypt this DB
    crypt_plugin: [u8; 32],
    /// High word of the next attachment counter
    att_high: i16,
    /// High words of the transaction counters
    tra_high: [u8; 4],
    /// Misc data
    data: [u8; 1],
}

impl HeaderPage {

    pub fn from_bytes(bytes: [u8; 1024]) -> Result<HeaderPage, FbError> {
        let hdr: HeaderPage = unsafe { ptr::read(bytes.as_ptr() as *const _) };

        if hdr.pag.ptype != 0x01 {
            return Err(FbError::from("Invalid header page type"));
        }

        Ok(hdr)
    }
}

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
    pub records: Vec<DataPageRecord>
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct DataPageRep {
    pub pag: Page,
    pub sequence: u32,
    pub relation: u16,
    pub count: u16,
    pub records: [DataPageRecord; 512] // TODO: fix this hardcoded array size
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

    pub fn from_bytes(bytes: Vec<u8>) -> Result<DataPage, FbError> {

        if bytes[0] != 0x05 {
            return Err(FbError::from("Invalid data page type"));
        }

        let rdata: DataPageRep = unsafe { ptr::read(bytes.as_ptr() as *const _) };

        if rdata.count > 512 {
            return Err(FbError::from("Overflow supported records"));
        }

        let mut records = rdata.records.to_vec();
        records.truncate(rdata.count as usize);

        let data = DataPage {
            pag: rdata.pag,
            sequence: rdata.sequence,
            relation: rdata.relation,
            count: rdata.count,
            records
        };

        Ok(data)
    }
}
