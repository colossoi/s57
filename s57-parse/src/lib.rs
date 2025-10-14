//! S-57 Electronic Navigational Chart (ENC) parser library
//!
//! This library provides functionality to parse S-57 format files used by NOAA
//! and other hydrographic organizations for Electronic Navigational Charts.
//!
//! S-57 files are encoded using the ISO 8211 standard.

pub mod bitstring;
pub mod ddr;
pub mod error;
pub mod interpret;
pub mod iso8211;
pub mod s57_schema;

pub use error::{ParseError, ParseErrorKind, Result};

/// Represents an S-57 file
pub struct S57File {
    records: Vec<iso8211::Record>,
}

impl S57File {
    /// Parse an S-57 file from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let records = iso8211::parse_file(data)?;
        Ok(S57File { records })
    }

    /// Get all records in the file
    pub fn records(&self) -> &[iso8211::Record] {
        &self.records
    }
}
