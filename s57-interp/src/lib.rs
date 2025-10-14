//! S-57 Interpretation Layer
//!
//! This crate provides an Entity-Component-System (ECS) architecture for
//! organizing parsed S-57 data into rendering-ready entities with exact
//! coordinate math using BigRational.
//!
//! It sits on top of s57-parse and transforms raw ISO 8211 records into
//! structured spatial and feature entities with:
//! - Exact lat/lon coordinates (BigRational)
//! - Exact depth values (BigRational)
//! - Topology relationships
//! - Feature attributes and cross-references

pub mod ecs;
pub mod systems;

// Re-export key types from s57-parse for convenience
pub use s57_parse::bitstring::{FoidKey, NameKey};
pub use s57_parse::{ParseError, ParseErrorKind, Result};
