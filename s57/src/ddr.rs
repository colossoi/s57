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
    /// Binary form
    Binary,
    /// Character data (ASCII)
    Character,
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

        // Parse subfield definitions from format controls
        let subfields = Self::parse_format_controls(&array_descriptor, &format_controls);

        Ok(FieldDef {
            tag,
            name,
            array_descriptor,
            format_controls,
            subfields,
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
        // Extract content between parentheses
        let format_specs = if let Some(start) = format_str.find('(') {
            if let Some(end) = format_str.find(')') {
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

    /// Parse a single format specification (e.g., "b12", "A", "I", "B(40)")
    fn parse_format_spec(spec: &str) -> (FormatType, usize) {
        let format = match spec.chars().next() {
            Some('b') | Some('B') => FormatType::Binary,
            Some('A') | Some('a') => FormatType::Character,
            Some('I') => FormatType::Character, // Integer as ASCII
            Some('R') => FormatType::Character, // Real as ASCII
            _ => FormatType::Mixed,
        };

        // Extract width from format spec
        // Handle formats like "b12", "A", "B(40)"
        let width_str: String = spec.chars().skip(1).collect();
        let width = if width_str.is_empty() {
            0 // Variable width (e.g., "A" alone)
        } else if width_str.starts_with('(') {
            // Format like "B(40)" - extract number from parentheses
            if let Some(end) = width_str.find(')') {
                width_str[1..end].parse::<usize>().unwrap_or(0)
            } else {
                0
            }
        } else {
            // Format like "b12" - the digits encode the byte width
            // b11 = 1 byte, b12 = 2 bytes, b14 = 4 bytes, b24 = 4 bytes
            width_str.parse::<usize>().unwrap_or(0) / 10
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
            FormatType::Binary => match data.len() {
                1 => SubfieldValue::Integer(data[0] as i32),
                2 => SubfieldValue::Integer(u16::from_le_bytes([data[0], data[1]]) as i32),
                4 => SubfieldValue::Integer(u32::from_le_bytes([
                    data[0], data[1], data[2], data[3],
                ]) as i32),
                _ => SubfieldValue::Bytes(data.to_vec()),
            },
            FormatType::Character => {
                if let Ok(s) = std::str::from_utf8(data) {
                    let trimmed = s.trim();
                    // Try to parse as integer or float
                    if let Ok(i) = trimmed.parse::<i32>() {
                        SubfieldValue::Integer(i)
                    } else if let Ok(f) = trimmed.parse::<f64>() {
                        SubfieldValue::Real(f)
                    } else {
                        SubfieldValue::String(trimmed.to_string())
                    }
                } else {
                    SubfieldValue::Bytes(data.to_vec())
                }
            }
            FormatType::Mixed => SubfieldValue::Bytes(data.to_vec()),
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
