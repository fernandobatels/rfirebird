//! Firebird page's representation

use std::ptr;

use crate::Error;

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
    pub fn from_bytes(bytes: [u8; 1024]) -> Result<HeaderPage, Error> {
        let hdr: HeaderPage = unsafe { ptr::read(bytes.as_ptr() as *const _) };

        if hdr.pag.ptype != 0x01 {
            return Err(Error::InvalidPage {
                tpe: hdr.pag.ptype,
                expected: 0x01,
                desc: "header".to_string(),
            });
        }

        Ok(hdr)
    }
}
