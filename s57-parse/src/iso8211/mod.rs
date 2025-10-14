// ! ISO 8211 parser module
//!
//! ISO 8211 is the underlying format for S-57 files.
//! Each ISO 8211 file consists of:
//! - Data Descriptive Record (DDR) - describes the structure
//! - Data Records (DR) - contain the actual data

mod directory;
mod field;
mod leader;

pub use directory::{Directory, DirectoryEntry};
pub use field::Field;
pub use leader::Leader;

use crate::error::{ParseError, ParseErrorKind, Result};
use log::{debug, trace};

/// ISO 8211 logical record
#[derive(Debug)]
pub struct Record {
    pub leader: Leader,
    pub directory: Directory,
    pub fields: Vec<Field>,
}

/// Parse an entire ISO 8211 file
pub fn parse_file(data: &[u8]) -> Result<Vec<Record>> {
    debug!("Parsing ISO 8211 file, total size: {} bytes", data.len());
    let mut records = Vec::new();
    let mut offset = 0;

    while offset < data.len() {
        trace!("Parsing record at offset {}", offset);
        let (record, bytes_read) = parse_record(&data[offset..], offset)?;
        debug!(
            "Parsed record {}: {} fields, {} bytes",
            records.len(),
            record.fields.len(),
            bytes_read
        );
        records.push(record);
        offset += bytes_read;

        // Check if we've reached the end
        if offset >= data.len() {
            break;
        }
    }

    debug!("Finished parsing {} records", records.len());
    Ok(records)
}

/// Parse a single ISO 8211 record
fn parse_record(data: &[u8], file_offset: usize) -> Result<(Record, usize)> {
    if data.len() < 24 {
        return Err(ParseError::at(ParseErrorKind::UnexpectedEof, file_offset));
    }

    // Parse leader (24 bytes)
    let leader = Leader::parse(&data[0..24])?;
    let record_length = leader.record_length as usize;

    if data.len() < record_length {
        return Err(ParseError::at(
            ParseErrorKind::RecordTooLarge {
                record_length,
                available: data.len(),
            },
            file_offset,
        ));
    }

    let record_data = &data[0..record_length];

    // Parse directory
    let base_addr = leader.base_address_of_field_area as usize;
    let directory_data = &record_data[24..base_addr];
    let directory = Directory::parse(directory_data, &leader, file_offset + 24)?;

    // Parse fields
    let field_area = &record_data[base_addr..];
    let fields = parse_fields(field_area, &directory, file_offset + base_addr)?;

    Ok((
        Record {
            leader,
            directory,
            fields,
        },
        record_length,
    ))
}

/// Parse field data based on directory entries
fn parse_fields(
    field_area: &[u8],
    directory: &Directory,
    base_offset: usize,
) -> Result<Vec<Field>> {
    let mut fields = Vec::new();

    for entry in &directory.entries {
        let start = entry.position as usize;
        let length = entry.length as usize;

        if start + length > field_area.len() {
            return Err(ParseError::at(
                ParseErrorKind::FieldOutOfBounds {
                    start,
                    length,
                    area_len: field_area.len(),
                },
                base_offset + start,
            ));
        }

        let field_data = &field_area[start..start + length];
        let field = Field::new(entry.tag.clone(), field_data.to_vec());
        fields.push(field);
    }

    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_leader() {
        // Build a proper 24-byte leader
        let leader_bytes = concat!(
            "01582", // Record length (5 bytes)
            "3",     // Interchange level (1 byte)
            "L",     // Leader identifier (1 byte)
            "E",     // Inline code extension (1 byte)
            "1",     // Version (1 byte)
            " ",     // Application indicator (1 byte)
            "09",    // Field control length (2 bytes)
            "00020", // Base address of field area (5 bytes)
            " ! ",   // Extended character set (3 bytes)
            "3404"   // Entry map (4 bytes)
        )
        .as_bytes();

        assert_eq!(leader_bytes.len(), 24);
        let leader = Leader::parse(leader_bytes).unwrap();
        assert_eq!(leader.record_length, 1582);
    }
}
