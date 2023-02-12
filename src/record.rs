//! Firebird records operations

use std::ptr;

use crate::Error;

/// Header for unfragmented firebird records
#[derive(Debug, Clone)]
pub struct RecordHeader {
    /// The id of the transaction that created this record
    pub transaction: i32,
    /// This is the record’s back pointer page
    pub b_page: i32,
    /// This is the record’s back line pointer
    pub b_line: u16,
    pub flags: u16,
    /// The record format version
    pub format: u8,
    /// This is the start of the compressed data.
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RecordHeaderRepr {
    pub transaction: i32,
    pub b_page: i32,
    pub b_line: u16,
    pub flags: u16,
    pub format: u8,
    pub data: [u8; 1024],
}

impl RecordHeader {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<RecordHeader, Error> {
        if bytes.len() > 1024 {
            return Err(Error::Overflow { limit: 1024, value: bytes.len(), msg: "supported data on header".to_string() });
        }

        let rrecord: RecordHeaderRepr = unsafe { ptr::read(bytes.as_ptr() as *const _) };

        let mut data = rrecord.data.to_vec();
        data.truncate(bytes.len() as usize);

        let record = RecordHeader {
            transaction: rrecord.transaction,
            b_page: rrecord.b_page,
            b_line: rrecord.b_line,
            flags: rrecord.flags,
            format: rrecord.format,
            data,
        };

        Ok(record)
    }

    /// Uncompress the data field
    pub fn read(&self) -> Result<Vec<u8>, Error> {
        Ok(rle_decode(&self.data))
    }
}

/// Decode the firebird record data
fn rle_decode(data: &Vec<u8>) -> Vec<u8> {
    // The compression is a type known as Run Length Encoding (RLE)
    // More infos: https://firebirdsql.org/file/documentation/html/en/firebirddocs/firebirdinternals/firebird-internals.html#fbint-p5-examine-data

    let mut result = Vec::<u8>::new();
    let mut iter = data.iter();
    let iter = iter.by_ref();

    while let Some(n) = iter.next() {
        let n = *n;
        let ni = n as i8;

        match ni {
            // 0 is the end of data, normally a padding byte
            0 => break,
            // The next 'n' bytes are stored 'verbatim'.
            ni if ni > 0 => {
                for next_byte in iter.take(n.into()) {
                    result.push(*next_byte);
                }
            }
            // The next byte is repeated 'n' times, but stored only once.
            _ => {
                if let Some(next_byte) = iter.next() {
                    let to = (ni as i16).abs();
                    for _ in 0..to {
                        result.push(*next_byte);
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn rle_decode_compressed() {
        let data = vec![
            0x01, 0xfe, 0xfd, 0x00, 0x03, 0x20, 0x00, 0x41, 0xfc, 0x61, 0x01, 0x42, 0xf7, 0x62,
            0x01, 0x43, 0xf2, 0x63, 0x02, 0x44, 0x44,
        ];

        // AaaaaBbbbbbbbbbCccccccccccccccDD
        let eresult = vec![
            0xfe, 0x00, 0x00, 0x00, 0x20, 0x00, 0x41, 0x61, 0x61, 0x61, 0x61, 0x42, 0x62, 0x62,
            0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x43, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63,
            0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x63, 0x44, 0x44,
        ];

        let result = rle_decode(&data);
        assert_eq!(eresult, result);
    }

    #[test]
    pub fn rle_decode_uncompressed() {
        let data = vec![
            0x01, 0xfe, 0xfd, 0x00, 0x0a, 0x08, 0x00, 0x46, 0x69, 0x72, 0x65, 0x62, 0x69, 0x72,
            0x64, 0x00,
        ];

        // Firebird
        let eresult = vec![
            0xfe, 0x00, 0x00, 0x00, 0x08, 0x00, 0x46, 0x69, 0x72, 0x65, 0x62, 0x69, 0x72, 0x64,
        ];

        let result = rle_decode(&data);
        assert_eq!(eresult, result);
    }

    #[test]
    pub fn rle_decode_compressed_varchar_with_666() {
        let data = vec![
            0x01, 0xfe, 0xfd, 0x00, 0x02, 0x03, 0x00, 0xfd, 0x36,
        ];

        let eresult = vec![
            0xFE, 0x00, 0x00, 0x00, 0x03, 0x00, 0x36, 0x36, 0x36,
        ];

        let result = rle_decode(&data);
        assert_eq!(eresult, result);
    }
}
