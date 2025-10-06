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
