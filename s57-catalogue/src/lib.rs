//! S-57 Object Catalogue
//!
//! Type-safe representations of S-57 object classes and attributes from the
//! IHO S-57 Object Catalogue (Part 3, Appendix A).
//!
//! This crate provides:
//! - Complete enum types for all 286 object classes (OBJL codes)
//! - Complete struct for all 483 attributes (ATTL codes)
//! - Lookup functions for decoding raw codes
//!
//! Generated from GDAL reference CSVs using scripts/generate_catalogue.nu
//!
//! Reference: IHO S-57 Edition 3.1, November 2000

// Generated modules - DO NOT EDIT MANUALLY
mod attributes_generated;
mod objects_generated;

pub use attributes_generated::AttributeInfo;
pub use objects_generated::ObjectClass;

/// Decode object class from OBJL code
///
/// # Arguments
/// * `objl` - Object class code (OBJL field from FRID)
///
/// # Returns
/// ObjectClass enum or None if unknown
pub fn decode_object(objl: u16) -> Option<ObjectClass> {
    ObjectClass::from_code(objl)
}

/// Get attribute name from ATTL code
///
/// # Arguments
/// * `attl` - Attribute code (ATTL field from ATTF)
///
/// # Returns
/// Human-readable attribute name, or None if unknown
pub fn get_attribute_name(attl: u16) -> Option<&'static str> {
    AttributeInfo::attribute_name(attl)
}
