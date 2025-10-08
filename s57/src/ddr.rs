//! Data Descriptive Record (DDR) parser
//!
//! The DDR defines the structure of all fields in the file.
//! This module parses the DDR and creates field definitions that can be used
//! to parse data records.

use crate::error::{ParseError, ParseErrorKind, Result};
use crate::iso8211::{Field, Record};
use std::collections::HashMap;

/// Field format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatType {
    /// Binary integer (b11, b12, b14, b24)
    BinaryInt,
    /// ASCII text, variable-length (A) - terminated by UT/FT
    Ascii,
    /// ASCII text, fixed-length (A(n)) - read exactly n bytes
    AsciiFixed,
    /// Integer as ASCII, variable-length (I) - terminated by UT/FT
    IntegerAscii,
    /// Integer as ASCII, fixed-length (I(n)) - read exactly n characters
    IntegerAsciiFixed,
    /// Real number as binary IEEE 754 (R(n)) - n bytes, little-endian
    /// R(4) = 32-bit float, R(8) = 64-bit double
    RealBinary,
    /// Bit string (B(n)) - n bits
    BitString,
    /// Mixed binary and character
    Mixed,
}

/// Field subfield definition
#[derive(Debug, Clone)]
pub struct SubfieldDef {
    /// Subfield label (e.g., "RCNM", "RCID")
    pub label: String,
    /// Format type
    pub format: FormatType,
    /// Width in bytes (0 = variable)
    pub width: usize,
}

/// Field definition from DDR
#[derive(Debug, Clone)]
pub struct FieldDef {
    /// Field tag (e.g., "FRID", "VRID")
    pub tag: String,
    /// Field name
    pub name: String,
    /// Array descriptor
    pub array_descriptor: String,
    /// Format controls
    pub format_controls: String,
    /// Subfield definitions
    pub subfields: Vec<SubfieldDef>,
    /// Whether this is a repeating group (starts with *)
    pub is_repeating: bool,
}

impl FieldDef {
    /// Get the number of subfield labels defined in the array descriptor
    pub fn subfield_count(&self) -> usize {
        if self.array_descriptor.is_empty() {
            return 0;
        }

        // Strip leading '*' if present
        let labels = self.array_descriptor.trim_start_matches('*');

        // Count labels separated by '!'
        if labels.is_empty() {
            0
        } else {
            labels.split('!').filter(|s| !s.trim().is_empty()).count()
        }
    }
}

/// Data Descriptive Record parser
pub struct DDR {
    /// Field definitions indexed by tag
    field_defs: HashMap<String, FieldDef>,
}

impl DDR {
    /// Parse the DDR from record 0
    pub fn parse(record: &Record) -> Result<Self> {
        if !record.leader.is_ddr() {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField("Expected DDR record".to_string()),
                0,
            ));
        }

        let mut field_defs = HashMap::new();

        // The DDR contains field definitions in fields after 0000 and 0001
        // Each field (starting from index 2) is a data descriptive field where:
        // - The field's tag (from directory) is the tag being defined
        // - The field's data contains the definition (name, array descriptor, format controls)
        for field in &record.fields[2..] {
            // Skip the special 0000 and 0001 fields if they appear again
            if field.tag == "0000" || field.tag == "0001" {
                continue;
            }

            // Parse field definition from this field's data
            if let Ok(def) = Self::parse_field_definition(field) {
                field_defs.insert(def.tag.clone(), def);
            }
        }

        Ok(DDR { field_defs })
    }

    /// Parse a single field definition from a DDR field
    fn parse_field_definition(field: &Field) -> Result<FieldDef> {
        let tag = field.tag.clone();

        // Field descriptor structure:
        // Field controls (9 bytes) + Field name | UT | Array descriptor | UT | Format controls | FT
        // Split by unit terminators (0x1F)
        let parts: Vec<&[u8]> = field.data.split(|&b| b == 0x1F).collect();

        // First part contains field controls (9 bytes) + field name
        let (field_controls, name) = if !parts.is_empty() && parts[0].len() >= 9 {
            let controls = String::from_utf8_lossy(&parts[0][..9]).to_string();
            let field_name = String::from_utf8_lossy(&parts[0][9..]).trim().to_string();
            (controls, field_name)
        } else if !parts.is_empty() {
            (String::new(), String::from_utf8_lossy(parts[0]).trim().to_string())
        } else {
            (String::new(), String::new())
        };

        let array_descriptor = if parts.len() > 1 {
            String::from_utf8_lossy(parts[1]).trim().to_string()
        } else {
            String::new()
        };

        // Third part: format controls (may have FT at end)
        let format_controls = if parts.len() > 2 {
            let format_part = parts[2];
            if !format_part.is_empty() && format_part[format_part.len() - 1] == 0x1E {
                String::from_utf8_lossy(&format_part[..format_part.len() - 1]).trim().to_string()
            } else {
                String::from_utf8_lossy(format_part).trim().to_string()
            }
        } else {
            String::new()
        };

        // Check if this is a repeating group (starts with *)
        let is_repeating = array_descriptor.starts_with('*');

        // Parse subfield definitions from format controls
        let subfields = Self::parse_format_controls(&array_descriptor, &format_controls);

        Ok(FieldDef {
            tag,
            name,
            array_descriptor,
            format_controls,
            subfields,
            is_repeating,
        })
    }

    /// Parse format controls to extract subfield definitions
    /// Labels come from array_descriptor: "*LABEL1!LABEL2!LABEL3"
    /// Formats come from format_controls: "(format1,format2,format3)"
    fn parse_format_controls(array_descriptor: &str, format_str: &str) -> Vec<SubfieldDef> {
        let mut subfields = Vec::new();

        // Parse labels from array descriptor
        // May start with '*' indicating repeating field
        let labels_part = array_descriptor.trim_start_matches('*');
        if labels_part.is_empty() {
            return subfields;
        }
        let labels: Vec<&str> = labels_part.split('!').map(|s| s.trim()).collect();

        // Parse formats from format_controls
        // Extract content between outermost parentheses
        // Format like "(b11,b14,2A(8),R(4))" - need to find matching closing paren
        let format_specs = if let Some(start) = format_str.find('(') {
            if let Some(end) = format_str.rfind(')') {
                &format_str[start + 1..end]
            } else {
                return subfields;
            }
        } else {
            return subfields;
        };

        // Split format specs by comma
        let formats: Vec<&str> = format_specs.split(',').map(|s| s.trim()).collect();

        // Match labels to formats
        // A format with a digit prefix (e.g., "3b24") means 3 consecutive labels use that format
        let mut label_idx = 0;
        let mut format_idx = 0;

        while label_idx < labels.len() && format_idx < formats.len() {
            let format_spec = formats[format_idx];

            // Check for repeat count prefix (e.g., "3b24" means next 3 labels use b24)
            let (repeat_count, actual_format) = if let Some(digit_end) = format_spec.find(|c: char| !c.is_ascii_digit()) {
                let prefix = &format_spec[..digit_end];
                if !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit()) {
                    if let Ok(count) = prefix.parse::<usize>() {
                        (count, &format_spec[digit_end..])
                    } else {
                        (1, format_spec)
                    }
                } else {
                    (1, format_spec)
                }
            } else {
                (1, format_spec)
            };

            // Parse format type and width once for this format spec
            let (format, width) = Self::parse_format_spec(actual_format);

            // Apply this format to the next 'repeat_count' labels
            for _ in 0..repeat_count {
                if label_idx >= labels.len() {
                    break;
                }

                let label = labels[label_idx].trim();
                if !label.is_empty() {
                    subfields.push(SubfieldDef {
                        label: label.to_string(),
                        format,
                        width,
                    });
                }
                label_idx += 1;
            }

            // Move to next format spec
            format_idx += 1;
        }

        subfields
    }

    /// Parse a single format specification (e.g., "b12", "A", "A(8)", "I", "I(5)", "R(4)", "B(40)")
    ///
    /// According to ISO 8211 and IHO S-57:
    /// - b11, b12, b14, b24 = binary integers (1, 2, 4, 4 bytes respectively)
    /// - A = ASCII text, variable-length (terminated by UT/FT)
    /// - A(n) = ASCII text, fixed-length (exactly n bytes)
    /// - I = Integer as ASCII, variable-length (terminated by UT/FT)
    /// - I(n) = Integer as ASCII, fixed-length (exactly n characters)
    /// - R(n) = Real as binary IEEE 754, n bytes (R(4)=float, R(8)=double), little-endian
    /// - B(n) = Bit string, n bits
    fn parse_format_spec(spec: &str) -> (FormatType, usize) {
        let first_char = spec.chars().next();
        let width_str: String = spec.chars().skip(1).collect();
        let has_width = width_str.starts_with('(');

        let format = match first_char {
            Some('b') => FormatType::BinaryInt,  // b11, b12, b14, b24
            Some('B') => FormatType::BitString, // B(n)
            Some('A') | Some('a') => {
                if has_width {
                    FormatType::AsciiFixed  // A(n)
                } else {
                    FormatType::Ascii  // A
                }
            }
            Some('I') => {
                if has_width {
                    FormatType::IntegerAsciiFixed  // I(n)
                } else {
                    FormatType::IntegerAscii  // I
                }
            }
            Some('R') => FormatType::RealBinary,  // R(n) - binary IEEE 754
            _ => FormatType::Mixed,
        };

        // Extract width from format spec
        let width = match format {
            FormatType::BinaryInt => {
                // Binary formats: b11, b12, b14, b24
                // Pattern: bXY where X is sign (1=unsigned, 2=signed), Y is width in bytes (1/2/4)
                if let Ok(code) = width_str.parse::<usize>() {
                    let width_digit = code % 10;
                    match width_digit {
                        1 => 1, // b11, b21 = 1 byte
                        2 => 2, // b12, b22 = 2 bytes
                        4 => 4, // b14, b24 = 4 bytes
                        _ => 0,
                    }
                } else {
                    0
                }
            }
            FormatType::Ascii | FormatType::IntegerAscii => {
                // Variable-length: A or I (no parentheses)
                // Terminated by UT (0x1F) or FT (0x1E)
                0
            }
            FormatType::AsciiFixed | FormatType::IntegerAsciiFixed | FormatType::RealBinary | FormatType::BitString => {
                // Fixed-length formats: A(n), I(n), R(n), B(n)
                // Extract n from parentheses
                if width_str.starts_with('(') {
                    if let Some(end) = width_str.find(')') {
                        width_str[1..end].parse::<usize>().unwrap_or(0)
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            FormatType::Mixed => 0,
        };

        (format, width)
    }

    /// Get field definition by tag
    pub fn get_field_def(&self, tag: &str) -> Option<&FieldDef> {
        self.field_defs.get(tag)
    }

    /// Parse a field's data using its definition
    pub fn parse_field_data<'a, 'b>(&'a self, field: &'b Field) -> Result<ParsedField<'a>> {
        let def = self.get_field_def(&field.tag).ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField(format!("No definition for field {}", field.tag)),
                0,
            )
        })?;

        let mut subfield_values = Vec::new();
        let mut offset = 0;
        let data = &field.data;

        // Handle repeating fields (array descriptor "*")
        let is_repeating = def.array_descriptor.contains('*');

        // Safety: if no subfields defined, can't parse
        if def.subfields.is_empty() {
            return Ok(ParsedField {
                tag: field.tag.clone(),
                field_def: def,
                groups: vec![],
            });
        }

        loop {
            if offset >= data.len() || data[offset] == 0x1E {
                break;
            }

            let mut current_group = Vec::new();
            let start_offset = offset;

            for subfield_def in &def.subfields {
                if offset >= data.len() || data[offset] == 0x1E {
                    break;
                }

                // Skip unit terminators between subfields
                if data[offset] == 0x1F {
                    offset += 1;
                    if offset >= data.len() {
                        break;
                    }
                }

                let value = if subfield_def.width > 0 {
                    // Fixed width
                    let end = (offset + subfield_def.width).min(data.len());
                    let subfield_data = &data[offset..end];
                    offset = end;
                    Self::parse_subfield_value(subfield_data, &subfield_def.format)
                } else {
                    // Variable width - read until unit terminator or field terminator
                    let start = offset;
                    while offset < data.len() && data[offset] != 0x1F && data[offset] != 0x1E {
                        offset += 1;
                    }
                    let subfield_data = &data[start..offset];
                    Self::parse_subfield_value(subfield_data, &subfield_def.format)
                };

                current_group.push((subfield_def.label.clone(), value));
            }

            if !current_group.is_empty() {
                subfield_values.push(current_group);
            }

            // Skip unit terminator after group
            if offset < data.len() && data[offset] == 0x1F {
                offset += 1;
            }

            // Safety: if offset didn't advance, break to avoid infinite loop
            if offset == start_offset {
                break;
            }

            // If not repeating, stop after first group
            if !is_repeating {
                break;
            }
        }

        Ok(ParsedField {
            tag: field.tag.clone(),
            field_def: def,
            groups: subfield_values,
        })
    }

    /// Parse a subfield value based on its format
    fn parse_subfield_value(data: &[u8], format: &FormatType) -> SubfieldValue {
        if data.is_empty() {
            return SubfieldValue::Null;
        }

        match format {
            FormatType::BinaryInt => {
                // Binary integer: b11 (1 byte), b12 (2 bytes), b14 (4 bytes), b24 (4 bytes signed)
                match data.len() {
                    1 => SubfieldValue::Integer(data[0] as i32),
                    2 => SubfieldValue::Integer(u16::from_le_bytes([data[0], data[1]]) as i32),
                    4 => {
                        // b14 = unsigned 32-bit, b24 = signed 32-bit
                        // For now treat all as signed (we'd need format spec to distinguish)
                        SubfieldValue::Integer(i32::from_le_bytes([
                            data[0], data[1], data[2], data[3],
                        ]))
                    }
                    _ => SubfieldValue::Bytes(data.to_vec()),
                }
            }
            FormatType::Ascii | FormatType::AsciiFixed => {
                // ASCII text (A or A(n)) - keep as string, don't try to parse as number
                if let Ok(s) = std::str::from_utf8(data) {
                    SubfieldValue::String(s.trim().to_string())
                } else {
                    SubfieldValue::Bytes(data.to_vec())
                }
            }
            FormatType::IntegerAscii | FormatType::IntegerAsciiFixed => {
                // Integer as ASCII (I or I(n)) - parse to integer
                if let Ok(s) = std::str::from_utf8(data) {
                    let trimmed = s.trim();
                    if let Ok(i) = trimmed.parse::<i32>() {
                        SubfieldValue::Integer(i)
                    } else {
                        SubfieldValue::String(trimmed.to_string())
                    }
                } else {
                    SubfieldValue::Bytes(data.to_vec())
                }
            }
            FormatType::RealBinary => {
                // Real as binary IEEE 754 (R(n)) - little-endian
                match data.len() {
                    4 => {
                        // R(4) = 32-bit float
                        let bytes = [data[0], data[1], data[2], data[3]];
                        SubfieldValue::Real(f32::from_le_bytes(bytes) as f64)
                    }
                    8 => {
                        // R(8) = 64-bit double
                        let bytes = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]];
                        SubfieldValue::Real(f64::from_le_bytes(bytes))
                    }
                    _ => SubfieldValue::Bytes(data.to_vec()),
                }
            }
            FormatType::BitString | FormatType::Mixed => SubfieldValue::Bytes(data.to_vec()),
        }
    }

    /// Get all field definitions
    pub fn field_defs(&self) -> &HashMap<String, FieldDef> {
        &self.field_defs
    }
}

/// Parsed field with subfield values
#[derive(Debug)]
pub struct ParsedField<'a> {
    /// Field tag
    pub tag: String,
    /// Field definition
    pub field_def: &'a FieldDef,
    /// Groups of subfield values (one group per array element)
    pub groups: Vec<Vec<(String, SubfieldValue)>>,
}

impl<'a> ParsedField<'a> {
    /// Get value of a subfield by label from the first group
    pub fn get_value(&self, label: &str) -> Option<&SubfieldValue> {
        self.groups
            .first()?
            .iter()
            .find(|(l, _)| l == label)
            .map(|(_, v)| v)
    }

    /// Get all groups (for repeating fields)
    pub fn groups(&self) -> &[Vec<(String, SubfieldValue)>] {
        &self.groups
    }
}

/// Subfield value
#[derive(Debug, Clone)]
pub enum SubfieldValue {
    /// Null/empty value
    Null,
    /// Integer value
    Integer(i32),
    /// Real/float value
    Real(f64),
    /// String value
    String(String),
    /// Raw bytes
    Bytes(Vec<u8>),
}

impl SubfieldValue {
    /// Get as integer if possible
    pub fn as_int(&self) -> Option<i32> {
        match self {
            SubfieldValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as float if possible
    pub fn as_float(&self) -> Option<f64> {
        match self {
            SubfieldValue::Real(f) => Some(*f),
            SubfieldValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Get as string if possible
    pub fn as_str(&self) -> Option<&str> {
        match self {
            SubfieldValue::String(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iso8211::Field;

    #[test]
    fn test_parse_dsid_field_with_variable_length_dates() {
        // This is the actual DSID field data from US5PVDGD.000 record 1
        // Format: (b11,b14,2b11,3A,2A(8),R(4),b11,2A,b11,b12,A)
        // Labels: RCNM!RCID!EXPP!INTU!DSNM!EDTN!UPDN!UADT!ISDT!STED!PRSP!PSDN!PRED!PROF!AGEN!COMT
        let field_data: Vec<u8> = vec![
            // Binary fields (fixed width)
            0x0a, // RCNM = 10 (b11: 1 byte)
            0x01, 0x00, 0x00, 0x00, // RCID = 1 (b14: 4 bytes LE)
            0x01, // EXPP = 1 (b11: 1 byte)
            0x05, // INTU = 5 (b11: 1 byte)
            // Variable-length ASCII fields (3A)
            0x55, 0x53, 0x35, 0x50, 0x56, 0x44, 0x47, 0x44, 0x2e, 0x30, 0x30, 0x30, // DSNM = "US5PVDGD.000"
            0x1f, // UT
            0x34, // EDTN = "4"
            0x1f, // UT
            0x30, // UPDN = "0"
            0x1f, // UT
            // Here's where it gets interesting - UADT should be variable-length A(8)
            0x32, 0x30, 0x32, 0x35, 0x30, 0x37, 0x30, 0x33, // "20250703"
            0x32, 0x30, 0x32, 0x35, 0x30, 0x37, 0x30, 0x33, 0x30, 0x33, 0x2e, 0x31, // "2025070303.1"
            0x01, // This is something else (maybe STED?)
            0x1f, // UT
            0x32, 0x2e, 0x30, // "2.0"
            0x1f, // UT
            0x01, // PRSP = 1
            0x26, 0x02, // PSDN (2 bytes)
            0x50, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x4e, 0x4f, 0x41, 0x41, // "Produced by NOAA"
            0x1f, // UT
            0x1e, // FT
        ];

        // Create field definition matching the DDR
        let array_descriptor = "RCNM!RCID!EXPP!INTU!DSNM!EDTN!UPDN!UADT!ISDT!STED!PRSP!PSDN!PRED!PROF!AGEN!COMT".to_string();
        let format_controls = "(b11,b14,2b11,3A,2A(8),R(4),b11,2A,b11,b12,A)".to_string();

        // Parse subfield definitions
        let subfields = DDR::parse_format_controls(&array_descriptor, &format_controls);

        let field_def = FieldDef {
            tag: "DSID".to_string(),
            name: "Data set identification field".to_string(),
            array_descriptor: array_descriptor.clone(),
            format_controls: format_controls.clone(),
            subfields,
            is_repeating: false,
        };

        // Create a mock DDR with this field definition
        let mut ddr = DDR {
            field_defs: std::collections::HashMap::new(),
        };
        ddr.field_defs.insert("DSID".to_string(), field_def);

        // Create the field
        let field = Field {
            tag: "DSID".to_string(),
            data: field_data,
        };

        // Parse the field data
        let result = ddr.parse_field_data(&field);
        assert!(result.is_ok(), "Failed to parse DSID field: {:?}", result.err());

        let parsed = result.unwrap();
        let groups = parsed.groups();
        assert_eq!(groups.len(), 1, "Expected 1 group");

        let group = &groups[0];

        // Check some basic fields
        let rcnm = group.iter().find(|(label, _)| label == "RCNM");
        assert!(rcnm.is_some(), "RCNM field not found");
        if let Some((_, SubfieldValue::Integer(val))) = rcnm {
            assert_eq!(*val, 10, "RCNM should be 10");
        } else {
            panic!("RCNM should be an integer");
        }

        // Check DSNM
        let dsnm = group.iter().find(|(label, _)| label == "DSNM");
        assert!(dsnm.is_some(), "DSNM field not found");
        if let Some((_, SubfieldValue::String(val))) = dsnm {
            assert_eq!(val, "US5PVDGD.000", "DSNM should be 'US5PVDGD.000'");
        } else {
            panic!("DSNM should be a string");
        }

        // Check EDTN (ASCII format, should be string)
        let edtn = group.iter().find(|(label, _)| label == "EDTN");
        assert!(edtn.is_some(), "EDTN field not found");
        if let Some((_, SubfieldValue::String(val))) = edtn {
            assert_eq!(val, "4", "EDTN should be '4'");
        } else {
            panic!("EDTN should be a string (format A), got {:?}", edtn);
        }

        // Check UPDN (ASCII format, should be string)
        let updn = group.iter().find(|(label, _)| label == "UPDN");
        assert!(updn.is_some(), "UPDN field not found");
        if let Some((_, SubfieldValue::String(val))) = updn {
            assert_eq!(val, "0", "UPDN should be '0'");
        } else {
            panic!("UPDN should be a string (format A), got {:?}", updn);
        }

        // THE CRITICAL TEST: Check UADT (should be "20250703", NOT "202507032025070303.1")
        let uadt = group.iter().find(|(label, _)| label == "UADT");
        assert!(uadt.is_some(), "UADT field not found");
        if let Some((_, value)) = uadt {
            println!("UADT value: {:?}", value);
            if let SubfieldValue::String(val) = value {
                assert_eq!(val, "20250703", "UADT should be '20250703', not concatenated with next field");
            } else {
                panic!("UADT should be a string, got {:?}", value);
            }
        }

        // Check ISDT (should be separate from UADT)
        let isdt = group.iter().find(|(label, _)| label == "ISDT");
        assert!(isdt.is_some(), "ISDT field not found");
        if let Some((_, value)) = isdt {
            println!("ISDT value: {:?}", value);
            // ISDT should be its own value, not mixed with UADT
        }

        // Print all parsed values for debugging
        println!("\nAll parsed DSID fields:");
        for (label, value) in group {
            println!("  {}: {:?}", label, value);
        }
    }
}
