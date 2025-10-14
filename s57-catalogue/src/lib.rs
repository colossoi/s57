//! S-57 Object Catalogue
//!
//! Type-safe representations of S-57 object classes and attributes from the
//! IHO S-57 Object Catalogue (Part 3, Appendix A).
//!
//! This crate provides:
//! - Enum types for object classes (OBJL codes)
//! - Enum types for attributes (ATTL codes)
//! - Lookup functions for decoding raw codes
//!
//! Reference: IHO S-57 Edition 3.1, November 2000

pub mod attributes;
pub mod objects;

pub use attributes::{Attribute, AttributeValue};
pub use objects::ObjectClass;

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

/// Decode attribute from ATTL code and string value
///
/// # Arguments
/// * `attl` - Attribute code (ATTL field from ATTF)
/// * `atvl` - Attribute value as string (ATVL field from ATTF)
///
/// # Returns
/// Attribute enum with typed value, or None if unknown
pub fn decode_attribute(attl: u16, atvl: &str) -> Option<Attribute> {
    Attribute::from_code_and_value(attl, atvl)
}
