use super::Leader;
use crate::error::{ParseError, ParseErrorKind, Result};
use log::trace;

const FIELD_TERMINATOR: u8 = 0x1E; // ASCII 30 (1/14)

/// ISO 8211 Directory
///
/// The directory contains entries that map field tags to their
/// positions and lengths in the field area.
#[derive(Debug, Clone)]
pub struct Directory {
    pub entries: Vec<DirectoryEntry>,
}

/// A single directory entry
#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    /// Field tag (4 characters for S-57)
    pub tag: String,
    /// Length of the field in bytes
    pub length: u32,
    /// Position of the field relative to the start of the field area
    pub position: u32,
}

impl Directory {
    /// Parse the directory from the data between the leader and field area
    pub fn parse(data: &[u8], leader: &Leader, base_offset: usize) -> Result<Self> {
        let mut entries = Vec::new();
        let entry_size = leader.directory_entry_size();

        let mut offset = 0;

        while offset < data.len() {
            // Check for field terminator
            if data[offset] == FIELD_TERMINATOR {
                break;
            }

            if offset + entry_size > data.len() {
                return Err(ParseError::at(
                    ParseErrorKind::InvalidDirectory(format!(
                        "Not enough data for directory entry at offset {}",
                        offset
                    )),
                    base_offset + offset,
                ));
            }

            let entry_data = &data[offset..offset + entry_size];
            let entry = DirectoryEntry::parse(entry_data, leader, base_offset + offset)?;
            entries.push(entry);

            offset += entry_size;
        }

        trace!("Parsed {} directory entries", entries.len());
        Ok(Directory { entries })
    }
}

impl DirectoryEntry {
    /// Parse a single directory entry
    fn parse(data: &[u8], leader: &Leader, base_offset: usize) -> Result<Self> {
        let tag_size = leader.size_of_field_tag as usize;
        let length_size = leader.size_of_field_length_field as usize;
        let position_size = leader.size_of_field_position_field as usize;

        let mut offset = 0;

        // Parse tag
        let tag = std::str::from_utf8(&data[offset..offset + tag_size])
            .map_err(|e| ParseError::at(ParseErrorKind::from(e), base_offset + offset))?
            .to_string();
        offset += tag_size;

        // Parse length
        let length_str = std::str::from_utf8(&data[offset..offset + length_size])
            .map_err(|e| ParseError::at(ParseErrorKind::from(e), base_offset + offset))?;
        let length: u32 = length_str.trim().parse().map_err(|_| {
            ParseError::at(
                ParseErrorKind::InvalidDirectory(format!("Invalid field length: '{}'", length_str)),
                base_offset + offset,
            )
        })?;
        offset += length_size;

        // Parse position
        let position_str = std::str::from_utf8(&data[offset..offset + position_size])
            .map_err(|e| ParseError::at(ParseErrorKind::from(e), base_offset + offset))?;
        let position: u32 = position_str.trim().parse().map_err(|_| {
            ParseError::at(
                ParseErrorKind::InvalidDirectory(format!(
                    "Invalid field position: '{}'",
                    position_str
                )),
                base_offset + offset,
            )
        })?;

        trace!(
            "Parsed directory entry: tag={}, length={}, position={}",
            tag,
            length,
            position
        );

        Ok(DirectoryEntry {
            tag,
            length,
            position,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directory_entry() {
        // Build a proper 24-byte leader
        let leader_data = concat!(
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

        assert_eq!(leader_data.len(), 24);
        let leader = Leader::parse(leader_data).unwrap();

        // Example directory entry: tag=DSID (4), length=165 (3), position=0170 (4)
        let entry_data = b"DSID1650170";
        let entry = DirectoryEntry::parse(entry_data, &leader, 24).unwrap();

        assert_eq!(entry.tag, "DSID");
        assert_eq!(entry.length, 165);
        assert_eq!(entry.position, 170);
    }
}
