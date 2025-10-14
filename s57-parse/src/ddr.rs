//! Data Descriptive Record (DDR) parser
//!
//! The DDR defines the structure of all fields in the file.
//! This module parses the DDR and creates field definitions that can be used
//! to parse data records.

use crate::error::{ParseError, ParseErrorKind, Result};
use crate::iso8211::{Field, Record};
use crate::s57_schema::OverrideSchema;
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
    /// Width in bytes (None = variable-length)
    pub width: Option<usize>,
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
    /// Override schema for S-57 field optionality
    schema: OverrideSchema,
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
        let schema = OverrideSchema::new();

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
            if let Ok(mut def) = Self::parse_field_definition(field) {
                // Apply format overrides from schema
                for subfield in &mut def.subfields {
                    // Apply format type override if present
                    if let Some(override_format) =
                        schema.get_format_override(&def.tag, &subfield.label)
                    {
                        subfield.format = override_format;
                        // For AsciiFixed, ensure width is set correctly
                        if matches!(override_format, FormatType::AsciiFixed)
                            && subfield.width.is_none()
                        {
                            // Default to 4 for R(4) -> A(4) conversion
                            subfield.width = Some(4);
                        }
                    }
                }
                field_defs.insert(def.tag.clone(), def);
            }
        }

        Ok(DDR { field_defs, schema })
    }

    /// Parse a single field definition from a DDR field
    fn parse_field_definition(field: &Field) -> Result<FieldDef> {
        let tag = field.tag.clone();

        // Field descriptor structure:
        // Field controls (9 bytes) + Field name | UT | Array descriptor | UT | Format controls | FT
        // Split by unit terminators (0x1F)
        let parts: Vec<&[u8]> = field.data.split(|&b| b == 0x1F).collect();

        // First part contains field controls (9 bytes) + field name
        let (_field_controls, name) = if !parts.is_empty() && parts[0].len() >= 9 {
            let controls = String::from_utf8_lossy(&parts[0][..9]).to_string();
            let field_name = String::from_utf8_lossy(&parts[0][9..]).trim().to_string();
            (controls, field_name)
        } else if !parts.is_empty() {
            (
                String::new(),
                String::from_utf8_lossy(parts[0]).trim().to_string(),
            )
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
                String::from_utf8_lossy(&format_part[..format_part.len() - 1])
                    .trim()
                    .to_string()
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
            let (repeat_count, actual_format) =
                if let Some(digit_end) = format_spec.find(|c: char| !c.is_ascii_digit()) {
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
    /// - b11 = 1-byte unsigned integer (0-255)
    /// - b12 = 2-byte unsigned integer, little-endian (0-65535)
    /// - b14 = 4-byte unsigned integer, little-endian (0-4,294,967,295)
    /// - b24 = 4-byte signed integer, little-endian, two's complement (±2,147,483,648)
    /// - A = ASCII text, variable-length (terminated by UT/FT)
    /// - A(n) = ASCII text, fixed-length (exactly n bytes)
    /// - I = Integer as ASCII, variable-length (terminated by UT/FT)
    /// - I(n) = Integer as ASCII, fixed-length (exactly n characters)
    /// - R(n) = Real as binary IEEE 754, n bytes (R(4)=float, R(8)=double), little-endian
    /// - B(n) = Bit string, n bits
    fn parse_format_spec(spec: &str) -> (FormatType, Option<usize>) {
        let first_char = spec.chars().next();
        let width_str: String = spec.chars().skip(1).collect();
        let has_width = width_str.starts_with('(');

        let format = match first_char {
            Some('b') => FormatType::BinaryInt, // b11, b12, b14, b24
            Some('B') => FormatType::BitString, // B(n)
            Some('A') | Some('a') => {
                if has_width {
                    FormatType::AsciiFixed // A(n)
                } else {
                    FormatType::Ascii // A
                }
            }
            Some('I') => {
                if has_width {
                    FormatType::IntegerAsciiFixed // I(n)
                } else {
                    FormatType::IntegerAscii // I
                }
            }
            Some('R') => FormatType::RealBinary, // R(n) - binary IEEE 754
            _ => FormatType::Mixed,
        };

        // Extract width from format spec
        // Return Some(width) for fixed-width formats, None for variable-length
        let width = match format {
            FormatType::BinaryInt => {
                // Binary formats per ISO/IEC 8211:
                // b11 = 1 byte unsigned
                // b12 = 2 bytes unsigned
                // b14 = 4 bytes unsigned
                // b24 = 4 bytes signed (two's complement)
                // All binary formats are fixed-width
                if let Ok(code) = width_str.parse::<usize>() {
                    match code {
                        11 | 21 => Some(1), // b11, b21 = 1 byte
                        12 | 22 => Some(2), // b12, b22 = 2 bytes
                        14 | 24 => Some(4), // b14, b24 = 4 bytes
                        _ => None,
                    }
                } else {
                    None
                }
            }
            FormatType::Ascii | FormatType::IntegerAscii => {
                // Variable-length: A or I (no parentheses)
                // Terminated by UT (0x1F) or FT (0x1E)
                None
            }
            FormatType::AsciiFixed
            | FormatType::IntegerAsciiFixed
            | FormatType::RealBinary
            | FormatType::BitString => {
                // Fixed-length formats: A(n), I(n), R(n), B(n)
                // Extract n from parentheses
                if width_str.starts_with('(') {
                    if let Some(end) = width_str.find(')') {
                        width_str[1..end].parse::<usize>().ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            FormatType::Mixed => None,
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

            for (subfield_idx, subfield_def) in def.subfields.iter().enumerate() {
                if offset >= data.len() || data[offset] == 0x1E {
                    break;
                }

                let value = if let Some(width) = subfield_def.width {
                    // Fixed width - read exact number of bytes
                    // No unit terminators between fixed-width fields
                    let end = (offset + width).min(data.len());
                    let subfield_data = &data[offset..end];
                    offset = end;
                    Self::parse_subfield_value(
                        subfield_data,
                        &subfield_def.format,
                        &subfield_def.label,
                    )
                } else {
                    // Variable width - check if this is an optional field that may be omitted
                    let is_optional = self.schema.is_optional(&field.tag, &subfield_def.label);

                    if is_optional && matches!(subfield_def.format, FormatType::Ascii) {
                        // Optional variable-length ASCII field: use lookahead to detect omission
                        let current_byte = data[offset];

                        if current_byte == 0x1E {
                            // FT → field and all remaining omitted
                            break;
                        } else {
                            // Check if this looks like the next field (binary data when next field is binary)
                            let next_field_is_binary = def
                                .subfields
                                .get(subfield_idx + 1)
                                .map(|next_def| {
                                    matches!(
                                        next_def.format,
                                        FormatType::BinaryInt | FormatType::RealBinary
                                    )
                                })
                                .unwrap_or(false);

                            // If current byte is non-ASCII and next field is binary, field is omitted
                            if next_field_is_binary
                                && (current_byte < 0x20 || current_byte >= 0x7F)
                                && current_byte != 0x1F
                            {
                                // Field omitted - don't advance offset, skip to next subfield
                                SubfieldValue::Null
                            } else {
                                // Read ASCII until UT/FT (handles empty case too: if current_byte == 0x1F,
                                // we read 0 bytes and then consume the UT)
                                let start = offset;
                                while offset < data.len()
                                    && data[offset] != 0x1F
                                    && data[offset] != 0x1E
                                {
                                    offset += 1;
                                }
                                let subfield_data = &data[start..offset];
                                // Consume the UT after reading (if present, not FT)
                                if offset < data.len() && data[offset] == 0x1F {
                                    offset += 1;
                                }
                                // For ASCII fields, empty data = empty string (not null)
                                if subfield_data.is_empty() {
                                    SubfieldValue::String(String::new())
                                } else {
                                    Self::parse_subfield_value(
                                        subfield_data,
                                        &subfield_def.format,
                                        &subfield_def.label,
                                    )
                                }
                            }
                        }
                    } else {
                        // Required field or non-ASCII: read until unit terminator or field terminator
                        let start = offset;
                        while offset < data.len() && data[offset] != 0x1F && data[offset] != 0x1E {
                            offset += 1;
                        }
                        let subfield_data = &data[start..offset];
                        // Consume the UT after reading (if present, not FT)
                        if offset < data.len() && data[offset] == 0x1F {
                            offset += 1;
                        }
                        // For ASCII fields, empty data = empty string (not null)
                        if subfield_data.is_empty()
                            && matches!(
                                subfield_def.format,
                                FormatType::Ascii | FormatType::AsciiFixed
                            )
                        {
                            SubfieldValue::String(String::new())
                        } else {
                            Self::parse_subfield_value(
                                subfield_data,
                                &subfield_def.format,
                                &subfield_def.label,
                            )
                        }
                    }
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

    /// Parse a subfield value based on its format and label
    fn parse_subfield_value(data: &[u8], format: &FormatType, _label: &str) -> SubfieldValue {
        if data.is_empty() {
            return SubfieldValue::Null;
        }

        match format {
            FormatType::BinaryInt => {
                // Binary integer per ISO/IEC 8211:
                // b11 = 1 byte unsigned
                // b12 = 2 bytes unsigned, little-endian
                // b14 = 4 bytes unsigned, little-endian
                // b24 = 4 bytes signed, little-endian, two's complement
                match data.len() {
                    1 => SubfieldValue::Integer(data[0] as i32),
                    2 => SubfieldValue::Integer(u16::from_le_bytes([data[0], data[1]]) as i32),
                    4 => {
                        // 4-byte integer - could be b14 (unsigned) or b24 (signed)
                        // Read as signed i32 (works for both)
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
                        let bytes = [
                            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                        ];
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
    fn test_parse_full_dsid_from_actual_file() {
        // Full DSID field from US5PVDGD.000 record 1
        // Format: (b11,b14,2b11,3A,2A(8),R(4),b11,2A,b11,b12,A)
        // But note: A(8) in S-57 is actually variable-length with UT terminators!
        // Hex breakdown:
        // 0a                       = RCNM (b11) = 10
        // 01 00 00 00              = RCID (b14) = 1
        // 01                       = EXPP (b11) = 1
        // 05                       = INTU (b11) = 5
        // 55...30 30 30            = DSNM (A) = "US5PVDGD.000"
        // 1f                       = UT
        // 34                       = EDTN (A) = "4"
        // 1f                       = UT
        // 30                       = UPDN (A) = "0"
        // 1f                       = UT
        // 32 30 32 35 30 37 30 33  = UADT (A, not A(8)!) = "20250703"
        // 32 30 32 35 30 37 30 33  = ISDT (A, not A(8)!) = "20250703"
        // 30 33 2e 31              = STED (A(4) override) = "03.1"
        // 01                       = PRSP (b11) = 1
        // 1f                       = UT
        // 32 2e 30                 = ... (we're misaligned)
        let field_data: Vec<u8> = vec![
            0x0a, 0x01, 0x00, 0x00, 0x00, 0x01, 0x05, 0x55, 0x53, 0x35, 0x50, 0x56, 0x44, 0x47,
            0x44, 0x2e, 0x30, 0x30, 0x30, 0x1f, 0x34, 0x1f, 0x30, 0x1f, 0x32, 0x30, 0x32, 0x35,
            0x30, 0x37, 0x30, 0x33, 0x32, 0x30, 0x32, 0x35, 0x30, 0x37, 0x30, 0x33, 0x30, 0x33,
            0x2e, 0x31, 0x01, 0x1f, 0x32, 0x2e, 0x30, 0x1f, 0x01, 0x26, 0x02, 0x50, 0x72, 0x6f,
            0x64, 0x75, 0x63, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x4e, 0x4f, 0x41, 0x41, 0x1f,
            0x1e,
        ];

        let array_descriptor =
            "RCNM!RCID!EXPP!INTU!DSNM!EDTN!UPDN!UADT!ISDT!STED!PRSP!PSDN!PRED!PROF!AGEN!COMT"
                .to_string();
        let format_controls = "(b11,b14,2b11,3A,2A(8),R(4),b11,2A,b11,b12,A)".to_string();

        let mut subfields = DDR::parse_format_controls(&array_descriptor, &format_controls);

        // Create schema and apply overrides (simulating what DDR::parse does)
        let schema = OverrideSchema::new();
        for subfield in &mut subfields {
            if let Some(override_format) = schema.get_format_override("DSID", &subfield.label) {
                subfield.format = override_format;
                // For AsciiFixed, ensure width is set correctly
                if matches!(override_format, FormatType::AsciiFixed) && subfield.width.is_none() {
                    subfield.width = Some(4);
                }
            }
        }

        let field_def = FieldDef {
            tag: "DSID".to_string(),
            name: "Data set identification field".to_string(),
            array_descriptor: array_descriptor.clone(),
            format_controls: format_controls.clone(),
            subfields,
            is_repeating: false,
        };

        let mut ddr = DDR {
            field_defs: std::collections::HashMap::new(),
            schema,
        };
        ddr.field_defs.insert("DSID".to_string(), field_def);

        let field = Field {
            tag: "DSID".to_string(),
            data: field_data,
        };

        let result = ddr.parse_field_data(&field);
        assert!(
            result.is_ok(),
            "Failed to parse DSID field: {:?}",
            result.err()
        );

        let parsed = result.unwrap();
        let groups = parsed.groups();
        assert_eq!(groups.len(), 1);
        let group = &groups[0];

        // Print all fields for debugging
        println!("\nFull DSID field parsing from actual file:");
        for (label, value) in group {
            println!("  {}: {:?}", label, value);
        }

        // Validate expected values
        // RCNM should be 10
        let rcnm = group.iter().find(|(label, _)| label == "RCNM");
        assert!(rcnm.is_some(), "RCNM not found");
        if let Some((_, SubfieldValue::Integer(val))) = rcnm {
            assert_eq!(*val, 10, "RCNM should be 10");
        }

        // DSNM should be "US5PVDGD.000"
        let dsnm = group.iter().find(|(label, _)| label == "DSNM");
        assert!(dsnm.is_some(), "DSNM not found");
        if let Some((_, SubfieldValue::String(val))) = dsnm {
            assert_eq!(val, "US5PVDGD.000", "DSNM should be 'US5PVDGD.000'");
        }

        // UADT should be "20250703" (8 characters)
        let uadt = group.iter().find(|(label, _)| label == "UADT");
        assert!(uadt.is_some(), "UADT not found");
        if let Some((_, value)) = uadt {
            if let SubfieldValue::String(val) = value {
                assert_eq!(val.len(), 8, "UADT should be 8 characters, got: '{}'", val);
                assert_eq!(val, "20250703", "UADT should be '20250703', got: '{}'", val);
            } else {
                panic!("UADT should be a string, got {:?}", value);
            }
        }

        // ISDT should be "20250703" (8 characters)
        let isdt = group.iter().find(|(label, _)| label == "ISDT");
        assert!(isdt.is_some(), "ISDT not found");
        if let Some((_, value)) = isdt {
            if let SubfieldValue::String(val) = value {
                assert_eq!(val.len(), 8, "ISDT should be 8 characters, got: '{}'", val);
                assert_eq!(val, "20250703", "ISDT should be '20250703', got: '{}'", val);
            } else {
                panic!("ISDT should be a string, got {:?}", value);
            }
        }

        // STED should be "03.1" (4 characters after format override)
        let sted = group.iter().find(|(label, _)| label == "STED");
        assert!(sted.is_some(), "STED not found");
        if let Some((_, value)) = sted {
            if let SubfieldValue::String(val) = value {
                assert_eq!(val, "03.1", "STED should be '03.1', got: '{}'", val);
            } else {
                panic!("STED should be a string, got {:?}", value);
            }
        }

        // PRSP should be 1 (b11)
        let prsp = group.iter().find(|(label, _)| label == "PRSP");
        assert!(prsp.is_some(), "PRSP not found");
        if let Some((_, SubfieldValue::Integer(val))) = prsp {
            assert_eq!(*val, 1, "PRSP should be 1, got: {}", val);
        } else {
            panic!("PRSP should be an integer");
        }

        // PSDN should be empty string (optional, present but empty - just UT with no content)
        let psdn = group.iter().find(|(label, _)| label == "PSDN");
        assert!(psdn.is_some(), "PSDN not found");
        if let Some((_, value)) = psdn {
            if let SubfieldValue::String(val) = value {
                assert_eq!(val, "", "PSDN should be empty string, got: '{}'", val);
            } else {
                panic!("PSDN should be a string (empty), got {:?}", value);
            }
        }

        // PRED is optional and may be empty string or omitted
        // In this file it appears to be empty
        let pred = group.iter().find(|(label, _)| label == "PRED");
        assert!(pred.is_some(), "PRED not found");
        // Just check it exists, value can vary

        // PROF should be 1 (b11) for ENC profile
        let prof = group.iter().find(|(label, _)| label == "PROF");
        assert!(prof.is_some(), "PROF not found");
        if let Some((_, SubfieldValue::Integer(val))) = prof {
            assert_eq!(*val, 1, "PROF should be 1 for ENC, got: {}", val);
        } else {
            panic!("PROF should be an integer");
        }

        // AGEN should be 550 (b12) for NOAA
        let agen = group.iter().find(|(label, _)| label == "AGEN");
        assert!(agen.is_some(), "AGEN not found");
        if let Some((_, SubfieldValue::Integer(val))) = agen {
            assert_eq!(*val, 550, "AGEN should be 550 for NOAA, got: {}", val);
        } else {
            panic!("AGEN should be an integer");
        }

        // COMT should be "Produced by NOAA"
        let comt = group.iter().find(|(label, _)| label == "COMT");
        assert!(comt.is_some(), "COMT not found");
        if let Some((_, value)) = comt {
            if let SubfieldValue::String(val) = value {
                assert_eq!(
                    val, "Produced by NOAA",
                    "COMT should be 'Produced by NOAA', got: '{}'",
                    val
                );
            } else {
                panic!("COMT should be a string, got {:?}", value);
            }
        }
    }

    #[test]
    fn test_parse_sg3d_repeating_b24_fields() {
        // SG3D field with repeating (3b24) groups: YCOO, XCOO, VE3D
        // From actual US5PVDGD.000 record 6
        // Format: (3b24) means 3 subfields of b24 (4-byte signed LE integers)
        // Each group is 12 bytes (3 * 4), field has 180 bytes data + 1 byte FT = 181 total
        // Expected: 15 groups, all VE3D values should be < 35
        let field_data: Vec<u8> = vec![
            0x3b, 0xa6, 0xe4, 0x18, 0x65, 0xbd, 0x73, 0xd5, 0x16, 0x00, 0x00, 0x00, 0xf2, 0x68,
            0xe4, 0x18, 0xdb, 0xdb, 0x73, 0xd5, 0x16, 0x00, 0x00, 0x00, 0x3b, 0x0a, 0xe1, 0x18,
            0xfe, 0xa2, 0x74, 0xd5, 0x15, 0x00, 0x00, 0x00, 0xb0, 0x4e, 0xe4, 0x18, 0xe4, 0xce,
            0x75, 0xd5, 0x15, 0x00, 0x00, 0x00, 0x38, 0x3d, 0xe4, 0x18, 0x01, 0xf7, 0x75, 0xd5,
            0x15, 0x00, 0x00, 0x00, 0xca, 0x9e, 0xe3, 0x18, 0x63, 0x13, 0x76, 0xd5, 0x1f, 0x00,
            0x00, 0x00, 0x01, 0x12, 0xe4, 0x18, 0x0d, 0x4f, 0x76, 0xd5, 0x15, 0x00, 0x00, 0x00,
            0xec, 0xf3, 0xe3, 0x18, 0x81, 0x79, 0x76, 0xd5, 0x16, 0x00, 0x00, 0x00, 0x3a, 0x64,
            0xe1, 0x18, 0xeb, 0x7c, 0x76, 0xd5, 0x20, 0x00, 0x00, 0x00, 0xc6, 0x8a, 0xe1, 0x18,
            0xb0, 0x97, 0x76, 0xd5, 0x15, 0x00, 0x00, 0x00, 0xa9, 0xc7, 0xe3, 0x18, 0xd6, 0x9e,
            0x76, 0xd5, 0x16, 0x00, 0x00, 0x00, 0x58, 0x65, 0xe3, 0x18, 0x16, 0xa8, 0x76, 0xd5,
            0x16, 0x00, 0x00, 0x00, 0x63, 0x8b, 0xe3, 0x18, 0x73, 0xaa, 0x76, 0xd5, 0x16, 0x00,
            0x00, 0x00, 0x9a, 0xb0, 0xe0, 0x18, 0xba, 0xa6, 0x77, 0xd5, 0x16, 0x00, 0x00, 0x00,
            0x79, 0x7a, 0xe0, 0x18, 0x0a, 0x10, 0x78, 0xd5, 0x1f, 0x00, 0x00, 0x00, 0x1e,
        ];

        let array_descriptor = "*YCOO!XCOO!VE3D".to_string();
        let format_controls = "(3b24)".to_string();

        let subfields = DDR::parse_format_controls(&array_descriptor, &format_controls);

        // Verify subfield definitions
        assert_eq!(subfields.len(), 3, "Should have 3 subfields");
        assert_eq!(subfields[0].label, "YCOO");
        assert_eq!(subfields[1].label, "XCOO");
        assert_eq!(subfields[2].label, "VE3D");
        assert_eq!(subfields[0].width, Some(4), "b24 should be 4 bytes");
        assert_eq!(subfields[1].width, Some(4), "b24 should be 4 bytes");
        assert_eq!(subfields[2].width, Some(4), "b24 should be 4 bytes");

        let field_def = FieldDef {
            tag: "SG3D".to_string(),
            name: "3-D coordinate (sounding array) field".to_string(),
            array_descriptor: array_descriptor.clone(),
            format_controls: format_controls.clone(),
            subfields,
            is_repeating: true,
        };

        let mut ddr = DDR {
            field_defs: std::collections::HashMap::new(),
            schema: OverrideSchema::new(),
        };
        ddr.field_defs.insert("SG3D".to_string(), field_def);

        let field = Field {
            tag: "SG3D".to_string(),
            data: field_data.clone(),
        };

        let result = ddr.parse_field_data(&field);
        assert!(
            result.is_ok(),
            "Failed to parse SG3D field: {:?}",
            result.err()
        );

        let parsed = result.unwrap();
        let groups = parsed.groups();

        // Field is 181 bytes total, minus 1 for FT = 180 bytes of data
        // 180 / 12 = 15 groups expected
        assert_eq!(
            groups.len(),
            15,
            "Expected 15 groups (180 bytes / 12 bytes per group)"
        );

        // Validate each group
        println!("\nParsed SG3D groups:");
        for (i, group) in groups.iter().enumerate() {
            assert_eq!(group.len(), 3, "Each group should have 3 subfields");

            let ycoo = group.iter().find(|(label, _)| label == "YCOO");
            let xcoo = group.iter().find(|(label, _)| label == "XCOO");
            let ve3d = group.iter().find(|(label, _)| label == "VE3D");

            assert!(ycoo.is_some(), "YCOO not found in group {}", i);
            assert!(xcoo.is_some(), "XCOO not found in group {}", i);
            assert!(ve3d.is_some(), "VE3D not found in group {}", i);

            if let Some((_, SubfieldValue::Integer(y))) = ycoo {
                if let Some((_, SubfieldValue::Integer(x))) = xcoo {
                    if let Some((_, SubfieldValue::Integer(z))) = ve3d {
                        println!("  group_{}: YCOO={}, XCOO={}, VE3D={}", i, y, x, z);

                        // Critical validation: all VE3D values should be < 35
                        assert!(*z < 35, "VE3D should be < 35, got {} in group {}", z, i);
                        assert!(
                            *z >= 0,
                            "VE3D should be >= 0 (unsigned), got {} in group {}",
                            z,
                            i
                        );
                    } else {
                        panic!("VE3D should be an integer in group {}", i);
                    }
                } else {
                    panic!("XCOO should be an integer in group {}", i);
                }
            } else {
                panic!("YCOO should be an integer in group {}", i);
            }
        }

        // Verify specific expected values from manual hex parsing
        let group0 = &groups[0];
        let ycoo0 = group0.iter().find(|(label, _)| label == "YCOO").unwrap();
        let xcoo0 = group0.iter().find(|(label, _)| label == "XCOO").unwrap();
        let ve3d0 = group0.iter().find(|(label, _)| label == "VE3D").unwrap();

        if let SubfieldValue::Integer(y) = ycoo0.1 {
            assert_eq!(y, 417637947, "group_0 YCOO");
        }
        if let SubfieldValue::Integer(x) = xcoo0.1 {
            assert_eq!(x, -713835163, "group_0 XCOO");
        }
        if let SubfieldValue::Integer(z) = ve3d0.1 {
            assert_eq!(z, 22, "group_0 VE3D");
        }
    }
}
