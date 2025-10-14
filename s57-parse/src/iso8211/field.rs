/// ISO 8211 Field
///
/// Represents a single field from the field area.
/// Fields are identified by their tag and contain raw byte data.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field tag (e.g., "DSID", "FRID", "VRID")
    pub tag: String,
    /// Raw field data (including any subfield structure)
    pub data: Vec<u8>,
}

impl Field {
    /// Create a new field
    pub fn new(tag: String, data: Vec<u8>) -> Self {
        Field { tag, data }
    }

    /// Get field data as a string (if it's ASCII text)
    pub fn as_string(&self) -> Result<String, std::str::Utf8Error> {
        std::str::from_utf8(&self.data).map(|s| s.to_string())
    }

    /// Check if this is a specific field by tag
    pub fn is_tag(&self, tag: &str) -> bool {
        self.tag == tag
    }

    /// Parse the 0000 field control field (DDR only)
    ///
    /// According to ISO/IEC 8211, the field control field (tag "0000") defines
    /// the hierarchy of all fields in the DDR using parent/child tag pairs.
    ///
    /// Structure: Field controls | External file title | UT | List of field tag pairs | FT
    ///
    /// Returns (field_controls, external_title, tag_pairs)
    pub fn parse_field_control_field(&self) -> Option<(String, String, Vec<(String, String)>)> {
        if self.tag != "0000" {
            return None;
        }

        // Find the first unit terminator (0x1F)
        let first_ut = self.data.iter().position(|&b| b == 0x1F)?;

        // Everything before first UT is: field controls + external file title
        let before_ut = &self.data[..first_ut];

        // Field controls are first 9 bytes: "0000;&" + 3 spaces
        if before_ut.len() < 9 {
            return None;
        }

        let field_controls = String::from_utf8_lossy(&before_ut[..9]).to_string();
        let external_title = if before_ut.len() > 9 {
            String::from_utf8_lossy(&before_ut[9..]).trim().to_string()
        } else {
            String::new()
        };

        // After first UT and before FT (0x1E) is the list of field tag pairs
        let after_ut = &self.data[first_ut + 1..];
        let tag_pairs_end = after_ut
            .iter()
            .position(|&b| b == 0x1E)
            .unwrap_or(after_ut.len());
        let tag_pairs_data = &after_ut[..tag_pairs_end];

        // Parse tag pairs - each pair is 2 consecutive 4-character tags (parent, child)
        let tag_pairs_str = String::from_utf8_lossy(tag_pairs_data);
        let mut tag_pairs = Vec::new();

        let chars: Vec<char> = tag_pairs_str.chars().collect();
        let mut i = 0;
        while i + 7 < chars.len() {
            let parent: String = chars[i..i + 4].iter().collect();
            let child: String = chars[i + 4..i + 8].iter().collect();
            tag_pairs.push((parent, child));
            i += 8;
        }

        Some((field_controls, external_title, tag_pairs))
    }

    /// Parse the 0001 record identifier field (DDR only)
    ///
    /// In the DDR, tag 0001 is a data descriptive field that defines the structure
    /// of the record identifier field that appears in each data record.
    ///
    /// Structure: Field controls | Field name | UT | Array descriptor | UT | Format controls | FT
    ///
    /// Returns (field_controls, field_name, array_descriptor, format_controls)
    pub fn parse_record_identifier_field(&self) -> Option<(String, String, String, String)> {
        if self.tag != "0001" {
            return None;
        }

        // Check if this is DDR format (long text) or DR format (short binary)
        // DR format is typically 3 bytes: seq + reserved + FT
        // DDR format is much longer with text
        if self.data.len() < 20 {
            return None; // Too short to be DDR format
        }

        // Split by unit terminators (0x1F)
        let parts: Vec<&[u8]> = self.data.split(|&b| b == 0x1F).collect();

        if parts.len() < 3 {
            return None;
        }

        // First part: field controls + field name
        let first_part = parts[0];
        if first_part.len() < 9 {
            return None;
        }

        let field_controls = String::from_utf8_lossy(&first_part[..9]).to_string();
        let field_name = String::from_utf8_lossy(&first_part[9..]).trim().to_string();

        // Second part: array descriptor
        let array_descriptor = String::from_utf8_lossy(parts[1]).trim().to_string();

        // Third part: format controls (may have FT at end)
        let format_part = parts[2];
        let format_controls =
            if !format_part.is_empty() && format_part[format_part.len() - 1] == 0x1E {
                String::from_utf8_lossy(&format_part[..format_part.len() - 1])
                    .trim()
                    .to_string()
            } else {
                String::from_utf8_lossy(format_part).trim().to_string()
            };

        Some((
            field_controls,
            field_name,
            array_descriptor,
            format_controls,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation() {
        let field = Field::new("TEST".to_string(), vec![65, 66, 67]); // "ABC"
        assert_eq!(field.tag, "TEST");
        assert_eq!(field.as_string().unwrap(), "ABC");
    }

    #[test]
    fn test_field_is_tag() {
        let field = Field::new("DSID".to_string(), vec![]);
        assert!(field.is_tag("DSID"));
        assert!(!field.is_tag("FRID"));
    }
}
