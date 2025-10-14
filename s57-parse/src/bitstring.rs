//! Bitstring decoders for S-57 NAME and LNAM fields
//!
//! S-57 uses bitstring fields (B(n)) to encode compact pointers between records.
//! These are stored as raw bytes with little-endian bit ordering.
//!
//! - NAME (B40): 40 bits (5 bytes) = RCNM (8 bits) | RCID (32 bits)
//! - LNAM (B64): 64 bits (8 bytes) = structured FOID with agency/object IDs

use crate::error::{ParseError, ParseErrorKind, Result};

/// NameKey: Decoded NAME field (B40) - identifies a vector record
///
/// Format: 40 bits (5 bytes), little-endian bit order
/// - RCNM: Record Name (lower 8 bits) - record type (VI=110, VC=120, VE=130, VF=140)
/// - RCID: Record ID (next 32 bits) - unique within record type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NameKey {
    /// Record name (type): VI=110, VC=120, VE=130, VF=140
    pub rcnm: u8,
    /// Record identification number (unique within RCNM)
    pub rcid: u32,
}

impl NameKey {
    /// Decode a NAME field from B(40) bitstring (5 bytes)
    ///
    /// The encoding is little-endian:
    /// - Byte 0: RCNM (bits 0-7)
    /// - Bytes 1-4: RCID (bits 8-39), little-endian u32
    ///
    /// # Examples
    /// ```
    /// # use s57_parse::bitstring::NameKey;
    /// // NAME for vector record: RCNM=110 (VI), RCID=42
    /// let data = [110, 42, 0, 0, 0];
    /// let name = NameKey::decode(&data).unwrap();
    /// assert_eq!(name.rcnm, 110);
    /// assert_eq!(name.rcid, 42);
    /// ```
    pub fn decode(data: &[u8]) -> Result<Self> {
        if data.len() != 5 {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField(format!(
                    "NAME (B40) must be exactly 5 bytes, got {}",
                    data.len()
                )),
                0,
            ));
        }

        // Extract RCNM from first byte (bits 0-7)
        let rcnm = data[0];

        // Extract RCID from next 4 bytes (bits 8-39), little-endian
        let rcid = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);

        Ok(NameKey { rcnm, rcid })
    }

    /// Encode a NameKey back to B(40) bitstring (5 bytes)
    pub fn encode(&self) -> [u8; 5] {
        let rcid_bytes = self.rcid.to_le_bytes();
        [
            self.rcnm,
            rcid_bytes[0],
            rcid_bytes[1],
            rcid_bytes[2],
            rcid_bytes[3],
        ]
    }
}

/// FoidKey: Decoded LNAM field (B64) - identifies a feature record
///
/// Format: 64 bits (8 bytes), structured as:
/// - AGEN: Producing agency code (16 bits)
/// - FIDN: Feature identification number (32 bits)
/// - FIDS: Feature identification subdivision (16 bits)
///
/// This follows the FOID (Feature Object Identifier) structure from S-57.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FoidKey {
    /// Producing agency code (e.g., 550 = NOAA)
    pub agen: u16,
    /// Feature identification number
    pub fidn: u32,
    /// Feature identification subdivision
    pub fids: u16,
}

impl FoidKey {
    /// Decode an LNAM field from B(64) bitstring (8 bytes)
    ///
    /// The encoding is little-endian:
    /// - Bytes 0-1: AGEN (16 bits), little-endian u16
    /// - Bytes 2-5: FIDN (32 bits), little-endian u32
    /// - Bytes 6-7: FIDS (16 bits), little-endian u16
    ///
    /// # Examples
    /// ```
    /// # use s57_parse::bitstring::FoidKey;
    /// // LNAM: AGEN=550 (NOAA), FIDN=12345, FIDS=1
    /// let data = [0x26, 0x02, 0x39, 0x30, 0x00, 0x00, 0x01, 0x00];
    /// let foid = FoidKey::decode(&data).unwrap();
    /// assert_eq!(foid.agen, 550);
    /// assert_eq!(foid.fidn, 12345);
    /// assert_eq!(foid.fids, 1);
    /// ```
    pub fn decode(data: &[u8]) -> Result<Self> {
        if data.len() != 8 {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField(format!(
                    "LNAM (B64) must be exactly 8 bytes, got {}",
                    data.len()
                )),
                0,
            ));
        }

        // Extract AGEN (bytes 0-1), little-endian
        let agen = u16::from_le_bytes([data[0], data[1]]);

        // Extract FIDN (bytes 2-5), little-endian
        let fidn = u32::from_le_bytes([data[2], data[3], data[4], data[5]]);

        // Extract FIDS (bytes 6-7), little-endian
        let fids = u16::from_le_bytes([data[6], data[7]]);

        Ok(FoidKey { agen, fidn, fids })
    }

    /// Encode a FoidKey back to B(64) bitstring (8 bytes)
    pub fn encode(&self) -> [u8; 8] {
        let agen_bytes = self.agen.to_le_bytes();
        let fidn_bytes = self.fidn.to_le_bytes();
        let fids_bytes = self.fids.to_le_bytes();

        [
            agen_bytes[0],
            agen_bytes[1],
            fidn_bytes[0],
            fidn_bytes[1],
            fidn_bytes[2],
            fidn_bytes[3],
            fids_bytes[0],
            fids_bytes[1],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_key_decode() {
        // Test case: RCNM=110 (VI - isolated node), RCID=42
        let data = [110, 42, 0, 0, 0];
        let name = NameKey::decode(&data).unwrap();
        assert_eq!(name.rcnm, 110);
        assert_eq!(name.rcid, 42);
    }

    #[test]
    fn test_name_key_roundtrip() {
        let original = NameKey {
            rcnm: 130,
            rcid: 999999,
        };
        let encoded = original.encode();
        let decoded = NameKey::decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_name_key_invalid_length() {
        let data = [110, 42, 0, 0]; // Only 4 bytes
        let result = NameKey::decode(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_foid_key_decode() {
        // Test case: AGEN=550 (NOAA), FIDN=12345, FIDS=1
        // 550 = 0x0226, 12345 = 0x00003039
        let data = [0x26, 0x02, 0x39, 0x30, 0x00, 0x00, 0x01, 0x00];
        let foid = FoidKey::decode(&data).unwrap();
        assert_eq!(foid.agen, 550);
        assert_eq!(foid.fidn, 12345);
        assert_eq!(foid.fids, 1);
    }

    #[test]
    fn test_foid_key_roundtrip() {
        let original = FoidKey {
            agen: 550,
            fidn: 987654,
            fids: 99,
        };
        let encoded = original.encode();
        let decoded = FoidKey::decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_foid_key_invalid_length() {
        let data = [0x26, 0x02, 0x39, 0x30, 0x00, 0x00, 0x01]; // Only 7 bytes
        let result = FoidKey::decode(&data);
        assert!(result.is_err());
    }
}
