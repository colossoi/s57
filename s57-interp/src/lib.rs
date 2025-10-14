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

use ecs::{DatasetParams, FeatureAttributes, World};
use num_bigint::BigInt;
use s57_parse::ddr::{SubfieldValue, DDR};
use s57_parse::S57File;
use systems::{
    get_i32, get_u16, get_u32, FeatureBindSystem, FoidDecodeSystem, GeometrySystem,
    NameDecodeSystem, TopologySystem,
};

/// Build a World from an S57File
///
/// Processes all records in the S57 file and populates the ECS World with:
/// - Vector entities (from VRID records)
/// - Feature entities (from FRID/FOID records)
/// - Exact coordinates (from SG2D/SG3D records)
/// - Topology (from VRPT records)
/// - Feature bindings (from FSPT/FFPT records)
/// - Attributes (from ATTF/NATF records)
///
/// # Arguments
/// * `file` - Parsed S57File from s57-parse
///
/// # Returns
/// World populated with all entities and components, or ParseError on failure
pub fn build_world(file: &S57File) -> Result<World> {
    let mut world = World::new();
    let records = file.records();

    // Parse DDR first
    let ddr = if let Some(ddr_record) = records.first() {
        if ddr_record.leader.is_ddr() {
            DDR::parse(ddr_record)?
        } else {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField("First record is not DDR".to_string()),
                0,
            ));
        }
    } else {
        return Err(ParseError::at(
            ParseErrorKind::InvalidField("Empty file".to_string()),
            0,
        ));
    };

    // First pass: Extract dataset parameters from DSPM field
    for record in &records[1..] {
        if let Some(dspm_field) = record.fields.iter().find(|f| f.tag == "DSPM") {
            if let Ok(parsed) = ddr.parse_field_data(dspm_field) {
                if let Some(group) = parsed.groups().first() {
                    let comf = get_i32(group, "COMF").ok().flatten().unwrap_or(10_000_000);
                    let somf = get_i32(group, "SOMF").ok().flatten().unwrap_or(100);
                    let duni = get_u16(group, "DUNI").ok().flatten().unwrap_or(1);
                    let huni = get_u16(group, "HUNI").ok().flatten().unwrap_or(1);
                    let puni = get_u16(group, "PUNI").ok().flatten().unwrap_or(1);
                    let hdat = get_u16(group, "HDAT").ok().flatten().unwrap_or(2);
                    let vdat = get_u16(group, "VDAT").ok().flatten().unwrap_or(0);
                    let sdat = get_u16(group, "SDAT").ok().flatten().unwrap_or(0);
                    let cscl = get_u32(group, "CSCL").ok().flatten().unwrap_or(1);

                    world.dataset_params = Some(DatasetParams {
                        comf: BigInt::from(comf),
                        somf: BigInt::from(somf),
                        duni,
                        huni,
                        puni,
                        hdat,
                        vdat,
                        sdat,
                        cscl,
                    });
                    break;
                }
            }
        }
    }

    // Second pass: Create entities from VRID (vectors) and FRID/FOID (features)
    for (record_idx, record) in records[1..].iter().enumerate() {
        let record_num = record_idx + 1; // Adjust for 0-based indexing after skipping DDR

        // Process vector records
        if let Some(vrid_field) = record.fields.iter().find(|f| f.tag == "VRID") {
            if let Ok(parsed) = ddr.parse_field_data(vrid_field) {
                let entity = match NameDecodeSystem::process_vrid(&mut world, &parsed) {
                    Ok(e) => e,
                    Err(e) => {
                        // Log with record context for debugging
                        let groups = parsed.groups();
                        if !groups.is_empty() {
                            let group = &groups[0];
                            let fields: Vec<String> = group
                                .iter()
                                .map(|(label, val)| format!("{}={:?}", label, val))
                                .collect();
                            log::warn!(
                                "Skipping VRID at record {}: {} [fields: {}]",
                                record_num,
                                e,
                                fields.join(", ")
                            );
                        } else {
                            log::warn!("Skipping VRID at record {}: {} [no groups]", record_num, e);
                        }
                        continue;
                    }
                };

                // Process SG2D geometry if present
                if let Some(sg2d_field) = record.fields.iter().find(|f| f.tag == "SG2D") {
                    if let Ok(parsed_sg2d) = ddr.parse_field_data(sg2d_field) {
                        let _ = GeometrySystem::process_sg2d(&mut world, entity, &parsed_sg2d);
                    }
                }

                // Process SG3D geometry if present
                if let Some(sg3d_field) = record.fields.iter().find(|f| f.tag == "SG3D") {
                    if let Ok(parsed_sg3d) = ddr.parse_field_data(sg3d_field) {
                        let _ = GeometrySystem::process_sg3d(&mut world, entity, &parsed_sg3d);
                    }
                }

                // Process VRPT topology if present
                if let Some(vrpt_field) = record.fields.iter().find(|f| f.tag == "VRPT") {
                    if let Ok(parsed_vrpt) = ddr.parse_field_data(vrpt_field) {
                        let _ = TopologySystem::process_vrpt(&mut world, entity, &parsed_vrpt);
                    }
                }
            }
        }

        // Process feature records
        if let Some(frid_field) = record.fields.iter().find(|f| f.tag == "FRID") {
            if let Some(foid_field) = record.fields.iter().find(|f| f.tag == "FOID") {
                if let Ok(parsed_frid) = ddr.parse_field_data(frid_field) {
                    if let Ok(parsed_foid) = ddr.parse_field_data(foid_field) {
                        let entity = match FoidDecodeSystem::process_feature(
                            &mut world,
                            &parsed_frid,
                            &parsed_foid,
                        ) {
                            Ok(e) => e,
                            Err(e) => {
                                log::warn!("Skipping FRID/FOID at record {}: {}", record_num, e);
                                continue;
                            }
                        };

                        // Process ATTF attributes if present
                        if let Some(attf_field) = record.fields.iter().find(|f| f.tag == "ATTF") {
                            if let Ok(parsed_attf) = ddr.parse_field_data(attf_field) {
                                let mut attf = Vec::new();
                                for group in parsed_attf.groups() {
                                    let attl = get_u16(group, "ATTL").ok().flatten().unwrap_or(0);
                                    let atvl = get_string(group, "ATVL").unwrap_or_default();
                                    attf.push((attl, atvl));
                                }
                                let attrs = world
                                    .feature_attributes
                                    .entry(entity)
                                    .or_insert_with(FeatureAttributes::default);
                                attrs.attf = attf;
                            }
                        }

                        // Process NATF attributes if present
                        if let Some(natf_field) = record.fields.iter().find(|f| f.tag == "NATF") {
                            if let Ok(parsed_natf) = ddr.parse_field_data(natf_field) {
                                let mut natf = Vec::new();
                                for group in parsed_natf.groups() {
                                    let attl = get_u16(group, "ATTL").ok().flatten().unwrap_or(0);
                                    let atvl = get_string(group, "ATVL").unwrap_or_default();
                                    natf.push((attl, atvl));
                                }
                                let attrs = world
                                    .feature_attributes
                                    .entry(entity)
                                    .or_insert_with(FeatureAttributes::default);
                                attrs.natf = natf;
                            }
                        }

                        // Process FSPT spatial pointers if present
                        if let Some(fspt_field) = record.fields.iter().find(|f| f.tag == "FSPT") {
                            if let Ok(parsed_fspt) = ddr.parse_field_data(fspt_field) {
                                if let Err(e) = FeatureBindSystem::process_fspt(
                                    &mut world,
                                    entity,
                                    &parsed_fspt,
                                ) {
                                    log::warn!(
                                        "Failed to process FSPT at record {}: {}",
                                        record_num,
                                        e
                                    );
                                }
                            }
                        }

                        // Process FFPT feature pointers if present
                        if let Some(ffpt_field) = record.fields.iter().find(|f| f.tag == "FFPT") {
                            if let Ok(parsed_ffpt) = ddr.parse_field_data(ffpt_field) {
                                let _ = FeatureBindSystem::process_ffpt(
                                    &mut world,
                                    entity,
                                    &parsed_ffpt,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(world)
}

/// Helper: extract string value from subfield group
fn get_string(group: &[(String, SubfieldValue)], label: &str) -> Option<String> {
    group.iter().find(|(l, _)| l == label).and_then(|(_, v)| {
        if let SubfieldValue::String(s) = v {
            Some(s.clone())
        } else {
            None
        }
    })
}
