use crate::error::{ParseError, ParseErrorKind, Result};
use log::trace;

/// ISO 8211 Record Leader (24 bytes fixed length)
///
/// The leader contains metadata about the record structure.
/// See ISO 8211 specification section 3.7.2
#[derive(Debug, Clone)]
pub struct Leader {
    /// Total length of the record in bytes (positions 0-4)
    pub record_length: u32,
    /// Interchange level (position 5)
    pub interchange_level: char,
    /// Leader identifier: 'L' for DDR, 'D' for DR (position 6)
    pub leader_identifier: char,
    /// Inline code extension indicator (position 7)
    pub inline_code_extension_indicator: char,
    /// Version number (position 8)
    pub version_number: char,
    /// Application indicator (position 9)
    pub application_indicator: char,
    /// Field control length (positions 10-11)
    pub field_control_length: String,
    /// Base address of field area (positions 12-16)
    pub base_address_of_field_area: u32,
    /// Extended character set indicator (positions 17-19)
    pub extended_character_set: String,
    /// Entry map: size of field length field (position 20)
    pub size_of_field_length_field: u8,
    /// Entry map: size of field position field (position 21)
    pub size_of_field_position_field: u8,
    /// Entry map: reserved (position 22)
    pub reserved: char,
    /// Entry map: size of field tag (position 23)
    pub size_of_field_tag: u8,
}

impl Leader {
    /// Parse a 24-byte leader
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 24 {
            return Err(ParseError::at(
                ParseErrorKind::InvalidLeader(format!(
                    "Leader must be 24 bytes, got {}",
                    data.len()
                )),
                0,
            ));
        }

        // Parse record length (bytes 0-4) - ASCII decimal number
        let record_length_str = std::str::from_utf8(&data[0..5])
            .map_err(|e| ParseError::at(ParseErrorKind::from(e), 0))?;
        let record_length: u32 = record_length_str.trim().parse().map_err(|_| {
            ParseError::at(
                ParseErrorKind::InvalidLeader(format!(
                    "Invalid record length: '{}'",
                    record_length_str
                )),
                0,
            )
        })?;

        // Single character fields
        let interchange_level = data[5] as char;
        let leader_identifier = data[6] as char;
        let inline_code_extension_indicator = data[7] as char;
        let version_number = data[8] as char;
        let application_indicator = data[9] as char;

        // Field control length (bytes 10-11)
        let field_control_length = std::str::from_utf8(&data[10..12])
            .map_err(|e| ParseError::at(ParseErrorKind::from(e), 10))?
            .to_string();

        // Base address of field area (bytes 12-16) - ASCII decimal number
        let base_addr_str = std::str::from_utf8(&data[12..17])
            .map_err(|e| ParseError::at(ParseErrorKind::from(e), 12))?;
        let base_address_of_field_area: u32 = base_addr_str.trim().parse().map_err(|_| {
            ParseError::at(
                ParseErrorKind::InvalidLeader(format!("Invalid base address: '{}'", base_addr_str)),
                12,
            )
        })?;

        // Extended character set indicator (bytes 17-19)
        let extended_character_set = std::str::from_utf8(&data[17..20])
            .map_err(|e| ParseError::at(ParseErrorKind::from(e), 17))?
            .to_string();

        // Entry map (bytes 20-23)
        let size_of_field_length_field = (data[20] as char)
            .to_digit(10)
            .ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidLeader("Invalid field length field size".to_string()),
                    20,
                )
            })?
            as u8;

        let size_of_field_position_field = (data[21] as char)
            .to_digit(10)
            .ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidLeader("Invalid field position field size".to_string()),
                    21,
                )
            })?
            as u8;

        let reserved = data[22] as char;

        let size_of_field_tag = (data[23] as char)
            .to_digit(10)
            .ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidLeader("Invalid field tag size".to_string()),
                    23,
                )
            })?
            as u8;

        trace!(
            "Parsed leader: length={}, type={}, base_addr={}, entry_size={}",
            record_length,
            leader_identifier,
            base_address_of_field_area,
            size_of_field_tag as usize + size_of_field_length_field as usize + size_of_field_position_field as usize
        );

        Ok(Leader {
            record_length,
            interchange_level,
            leader_identifier,
            inline_code_extension_indicator,
            version_number,
            application_indicator,
            field_control_length,
            base_address_of_field_area,
            extended_character_set,
            size_of_field_length_field,
            size_of_field_position_field,
            reserved,
            size_of_field_tag,
        })
    }

    /// Check if this is a Data Descriptive Record (DDR)
    pub fn is_ddr(&self) -> bool {
        self.leader_identifier == 'L'
    }

    /// Check if this is a Data Record (DR)
    pub fn is_dr(&self) -> bool {
        self.leader_identifier == 'D'
    }

    /// Get the size of a directory entry in bytes
    pub fn directory_entry_size(&self) -> usize {
        self.size_of_field_tag as usize
            + self.size_of_field_length_field as usize
            + self.size_of_field_position_field as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ddr_leader() {
        // Build a proper 24-byte DDR leader
        // Positions: 0-4(5) | 5(1) | 6(1) | 7(1) | 8(1) | 9(1) | 10-11(2) | 12-16(5) | 17-19(3) | 20-23(4)
        let data = concat!(
            "01582",  // Record length (5 bytes)
            "3",      // Interchange level (1 byte)
            "L",      // Leader identifier (1 byte)
            "E",      // Inline code extension (1 byte)
            "1",      // Version (1 byte)
            " ",      // Application indicator (1 byte)
            "09",     // Field control length (2 bytes)
            "00020",  // Base address of field area (5 bytes)
            " ! ",    // Extended character set (3 bytes)
            "3404"    // Entry map (4 bytes)
        ).as_bytes();

        assert_eq!(data.len(), 24, "Leader must be exactly 24 bytes");
        let leader = Leader::parse(data).unwrap();

        assert_eq!(leader.record_length, 1582);
        assert_eq!(leader.interchange_level, '3');
        assert_eq!(leader.leader_identifier, 'L');
        assert_eq!(leader.size_of_field_tag, 4);
    }

    #[test]
    fn test_parse_dr_leader() {
        // Build a proper 24-byte DR leader
        let data = concat!(
            "00321",  // Record length (5 bytes)
            " ",      // Interchange level (1 byte)
            "D",      // Leader identifier (1 byte)
            " ",      // Inline code extension (1 byte)
            " ",      // Version (1 byte)
            " ",      // Application indicator (1 byte)
            "  ",     // Field control length (2 bytes)
            "00065",  // Base address (5 bytes)
            "   ",    // Charset (3 bytes)
            "3304"    // Entry map (4 bytes)
        ).as_bytes();

        assert_eq!(data.len(), 24, "Leader must be exactly 24 bytes");
        let leader = Leader::parse(data).unwrap();

        assert_eq!(leader.record_length, 321);
        assert_eq!(leader.leader_identifier, 'D');
        assert_eq!(leader.size_of_field_tag, 4);
    }
}
