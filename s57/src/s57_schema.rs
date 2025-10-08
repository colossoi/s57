//! Override schema for ISO 8211 field parsing
//!
//! This module contains metadata about fields that extends the ISO 8211 DDR,
//! such as optionality rules from S-57 Appendix B.1 and format type corrections.
//!
//! Fields are identified by their tag. Within a field, subfields can be optional
//! based on context (record type). By default, all subfields are required unless
//! explicitly marked as optional.

use std::collections::{HashMap, HashSet};
use crate::ddr::FormatType;

/// Override schema database for ISO 8211 parsing
/// Contains field-specific metadata not present in the DDR
///
/// Stores only the OPTIONAL subfields for each field tag.
/// All subfields not listed are assumed to be REQUIRED.
///
/// Also stores format type overrides for fields where the DDR format
/// doesn't match S-57's actual encoding (e.g., R(4) in ASCII column).
pub struct OverrideSchema {
    /// Map of field tag -> set of optional subfield labels
    optional_subfields: HashMap<String, HashSet<String>>,
    /// Map of (field_tag, subfield_label) -> format type override
    format_overrides: HashMap<(String, String), FormatType>,
}

impl OverrideSchema {
    /// Create a new override schema with S-57 field definitions
    ///
    /// Based on S-57 Appendix B.1:
    /// - Section 6.3.2 (EN - New Edition): PSDN is optional
    /// - Section 6.4.2 (ER - New Edition): UADT and PSDN are optional
    ///
    /// Note: The context (EN vs ER vs other) is determined by the RCNM field value.
    /// For simplicity, we mark fields as optional if they're optional in ANY context.
    pub fn new() -> Self {
        let mut optional_subfields = HashMap::new();
        let mut format_overrides = HashMap::new();

        // DSID - Data Set Identification (tag from DDR)
        // All record types (DS, EN, ER, etc.) use the same DSID structure
        // but have different optionality rules based on RCNM value.
        //
        // From S-57 Appendix B.1:
        // - PSDN: optional in EN (RCNM=1), optional in ER (RCNM=2)
        // - UADT: optional in ER (RCNM=2)
        // - PRED: optional (based on main spec table 7.3.1.1)
        // - COMT: optional (based on main spec table 7.3.1.1)
        let mut dsid_optional = HashSet::new();
        dsid_optional.insert("PSDN".to_string());
        dsid_optional.insert("PRED".to_string());
        dsid_optional.insert("UADT".to_string());  // optional in ER context
        dsid_optional.insert("COMT".to_string());
        optional_subfields.insert("DSID".to_string(), dsid_optional);

        // DSID format overrides:
        // STED is listed as R(4) in the DDR, but per S-57 Appendix B.1,
        // this is in the ASCII column, meaning it's a 4-character ASCII string
        // representation of a real number (e.g., "03.1"), not binary IEEE 754.
        format_overrides.insert(
            ("DSID".to_string(), "STED".to_string()),
            FormatType::AsciiFixed
        );

        // TODO: Add other fields as we discover their optional subfields

        OverrideSchema {
            optional_subfields,
            format_overrides,
        }
    }

    /// Check if a subfield is optional for a given field tag
    ///
    /// Returns true if the subfield is marked as optional, false otherwise.
    /// Default assumption: all subfields are REQUIRED unless explicitly listed.
    pub fn is_optional(&self, tag: &str, label: &str) -> bool {
        self.optional_subfields
            .get(tag)
            .map(|optionals| optionals.contains(label))
            .unwrap_or(false)
    }

    /// Get format type override for a specific subfield
    ///
    /// Returns Some(FormatType) if there's an override, None otherwise.
    /// This is used when the DDR format doesn't match S-57's actual encoding.
    pub fn get_format_override(&self, tag: &str, label: &str) -> Option<FormatType> {
        self.format_overrides
            .get(&(tag.to_string(), label.to_string()))
            .copied()
    }
}

impl Default for OverrideSchema {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsid_optionality() {
        let schema = OverrideSchema::new();

        // Required fields (default)
        assert!(!schema.is_optional("DSID", "RCNM"));
        assert!(!schema.is_optional("DSID", "DSNM"));
        assert!(!schema.is_optional("DSID", "PROF"));

        // Optional fields (explicitly listed)
        assert!(schema.is_optional("DSID", "PSDN"));
        assert!(schema.is_optional("DSID", "PRED"));
        assert!(schema.is_optional("DSID", "UADT"));
        assert!(schema.is_optional("DSID", "COMT"));
    }

    #[test]
    fn test_unknown_field_default_required() {
        let schema = OverrideSchema::new();

        // Unknown fields default to required
        assert!(!schema.is_optional("UNKNOWN", "ANYFIELD"));
    }
}
