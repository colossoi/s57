//! Systems for processing S-57 records into ECS components
//!
//! Systems are pure functions that transform parsed S-57 data into
//! structured entities and components. Each system focuses on a specific
//! transformation step in the pipeline.

use crate::ecs::{EntityType, ExactDepths, ExactPositions, FeatureMeta, VectorMeta, World};
use num_bigint::BigInt;
use num_rational::BigRational;
use s57_parse::bitstring::{FoidKey, NameKey};
use s57_parse::ddr::{ParsedField, SubfieldValue};
use s57_parse::{ParseError, ParseErrorKind, Result};

/// NameDecodeSystem: Process VRID records to create vector entities
///
/// Extracts vector metadata from VRID (Vector Record Identifier) fields:
/// - RCNM (Record Name): 110=VI, 120=VC, 130=VE, 140=VF
/// - RCID (Record ID): unique within RCNM
/// - RVER (Record Version): version number
/// - RUIN (Record Update Instruction): 1=insert, 2=delete, 3=modify
///
/// Creates a vector entity for each VRID and populates:
/// - VectorMeta component
/// - name_index: NameKey → EntityId mapping
pub struct NameDecodeSystem;

impl NameDecodeSystem {
    /// Process a VRID field and create/update vector entity
    ///
    /// # Arguments
    /// * `world` - ECS world to create entities in
    /// * `vrid` - Parsed VRID field from DDR
    ///
    /// # Returns
    /// EntityId of the created/updated vector entity
    pub fn process_vrid(world: &mut World, vrid: &ParsedField) -> Result<crate::ecs::EntityId> {
        // Extract VRID subfields
        let groups = vrid.groups();
        if groups.is_empty() {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField("VRID has no data".to_string()),
                0,
            ));
        }

        let group = &groups[0];

        // Extract RCNM (required)
        let rcnm = Self::get_int(group, "RCNM").ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("VRID missing RCNM".to_string()),
                0,
            )
        })? as u8;

        // Extract RCID (required)
        let rcid = Self::get_int(group, "RCID").ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("VRID missing RCID".to_string()),
                0,
            )
        })? as u32;

        // Extract RVER (optional, default 1)
        let rver = Self::get_int(group, "RVER").unwrap_or(1) as u16;

        // Extract RUIN (optional, default 1=insert)
        let ruin = Self::get_int(group, "RUIN").unwrap_or(1) as u8;

        // Create NameKey
        let name = NameKey { rcnm, rcid };

        // Check if entity already exists in index
        let entity = if let Some(&existing_entity) = world.name_index.get(&name) {
            // Update existing entity (for RUIN=3 modify)
            existing_entity
        } else {
            // Create new entity
            let entity = world.create_entity(EntityType::Vector);
            world.name_index.insert(name, entity);
            entity
        };

        // Create/update VectorMeta component
        let meta = VectorMeta { name, rver, ruin };
        world.vector_meta.insert(entity, meta);

        Ok(entity)
    }

    /// Helper: extract integer value from subfield group
    fn get_int(group: &[(String, SubfieldValue)], label: &str) -> Option<i32> {
        group
            .iter()
            .find(|(l, _)| l == label)
            .and_then(|(_, v)| v.as_int())
    }
}

/// FoidDecodeSystem: Process FRID/FOID records to create feature entities
///
/// Extracts feature metadata from FRID (Feature Record Identifier) and
/// FOID (Feature Object Identifier) fields:
/// - RCNM (Record Name): 100=feature
/// - RCID (Record ID): unique within RCNM
/// - PRIM (Primitive): 1=point, 2=line, 3=area, 255=N/A
/// - GRUP (Group): 1=geo, 2=meta, 3=collection, etc.
/// - OBJL (Object Label): object class code
/// - AGEN (Agency): producing agency
/// - FIDN (Feature ID Number): unique ID
/// - FIDS (Feature ID Subdivision): subdivision
///
/// Creates a feature entity for each FRID and populates:
/// - FeatureMeta component
/// - foid_index: FoidKey → EntityId mapping
pub struct FoidDecodeSystem;

impl FoidDecodeSystem {
    /// Process FRID and FOID fields to create/update feature entity
    ///
    /// # Arguments
    /// * `world` - ECS world to create entities in
    /// * `frid` - Parsed FRID field
    /// * `foid` - Parsed FOID field
    ///
    /// # Returns
    /// EntityId of the created/updated feature entity
    pub fn process_feature(
        world: &mut World,
        frid: &ParsedField,
        foid: &ParsedField,
    ) -> Result<crate::ecs::EntityId> {
        // Extract FRID subfields
        let frid_group = frid.groups().first().ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FRID has no data".to_string()),
                0,
            )
        })?;

        let _rcnm = Self::get_int(frid_group, "RCNM").ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FRID missing RCNM".to_string()),
                0,
            )
        })? as u8;

        let _rcid = Self::get_int(frid_group, "RCID").ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FRID missing RCID".to_string()),
                0,
            )
        })? as u32;

        let prim = Self::get_int(frid_group, "PRIM").unwrap_or(255) as u8;
        let grup = Self::get_int(frid_group, "GRUP").unwrap_or(1) as u8;
        let objl = Self::get_int(frid_group, "OBJL").unwrap_or(0) as u16;
        let rver = Self::get_int(frid_group, "RVER").unwrap_or(1) as u16;
        let ruin = Self::get_int(frid_group, "RUIN").unwrap_or(1) as u8;

        // Extract FOID subfields
        let foid_group = foid.groups().first().ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID has no data".to_string()),
                0,
            )
        })?;

        let agen = Self::get_int(foid_group, "AGEN").ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID missing AGEN".to_string()),
                0,
            )
        })? as u16;

        let fidn = Self::get_int(foid_group, "FIDN").ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID missing FIDN".to_string()),
                0,
            )
        })? as u32;

        let fids = Self::get_int(foid_group, "FIDS").ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID missing FIDS".to_string()),
                0,
            )
        })? as u16;

        // Create FoidKey
        let foid_key = FoidKey { agen, fidn, fids };

        // Check if entity already exists in index
        let entity = if let Some(&existing_entity) = world.foid_index.get(&foid_key) {
            // Update existing entity
            existing_entity
        } else {
            // Create new entity
            let entity = world.create_entity(EntityType::Feature);
            world.foid_index.insert(foid_key, entity);
            entity
        };

        // Create/update FeatureMeta component
        let meta = FeatureMeta {
            foid: foid_key,
            prim,
            grup,
            objl,
            rver,
            ruin,
        };
        world.feature_meta.insert(entity, meta);

        Ok(entity)
    }

    /// Helper: extract integer value from subfield group
    fn get_int(group: &[(String, SubfieldValue)], label: &str) -> Option<i32> {
        group
            .iter()
            .find(|(l, _)| l == label)
            .and_then(|(_, v)| v.as_int())
    }
}

/// GeometrySystem: Process SG2D/SG3D records into exact coordinates
///
/// Transforms already-parsed spatial geometry fields directly into exact
/// BigRational lat/lon/depth values:
/// - SG2D: (Y, X) → ExactPositions (lat = Y/COMF, lon = X/COMF)
/// - SG3D: (Y, X, Z) → ExactPositions + ExactDepths (depth = Z/SOMF)
///
/// Requires DatasetParams to be set in World for COMF/SOMF values.
///
/// Input: ParsedField from s57-parse + DatasetParams
/// Output: ExactPositions and ExactDepths components (BigRational)
pub struct GeometrySystem;

impl GeometrySystem {
    /// Process SG2D field into exact positions
    ///
    /// # Arguments
    /// * `world` - ECS world with DatasetParams
    /// * `entity` - Entity to attach positions to
    /// * `sg2d` - Parsed SG2D field
    ///
    /// # Returns
    /// Ok(()) if successful, or ParseError if params/data missing
    pub fn process_sg2d(
        world: &mut World,
        entity: crate::ecs::EntityId,
        sg2d: &ParsedField,
    ) -> Result<()> {
        // Get dataset params (required for COMF)
        let params = world.dataset_params.as_ref().ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("Dataset params not set".to_string()),
                0,
            )
        })?;

        let groups = sg2d.groups();
        if groups.is_empty() {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField("SG2D has no data".to_string()),
                0,
            ));
        }

        // Extract coordinate pairs and convert to exact BigRational
        let mut lat = Vec::with_capacity(groups.len());
        let mut lon = Vec::with_capacity(groups.len());

        for group in groups {
            let y = Self::get_int(group, "YCOO").ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG2D missing YCOO".to_string()),
                    0,
                )
            })?;
            let x = Self::get_int(group, "XCOO").ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG2D missing XCOO".to_string()),
                    0,
                )
            })?;

            // lat = y / COMF (degrees)
            lat.push(BigRational::new(BigInt::from(y), params.comf.clone()));
            // lon = x / COMF (degrees)
            lon.push(BigRational::new(BigInt::from(x), params.comf.clone()));
        }

        // Create ExactPositions component
        world
            .exact_positions
            .insert(entity, ExactPositions { lat, lon });

        Ok(())
    }

    /// Process SG3D field into exact positions and depths
    ///
    /// # Arguments
    /// * `world` - ECS world with DatasetParams
    /// * `entity` - Entity to attach positions/depths to
    /// * `sg3d` - Parsed SG3D field
    ///
    /// # Returns
    /// Ok(()) if successful, or ParseError if params/data missing
    pub fn process_sg3d(
        world: &mut World,
        entity: crate::ecs::EntityId,
        sg3d: &ParsedField,
    ) -> Result<()> {
        // Get dataset params (required for COMF/SOMF)
        let params = world.dataset_params.as_ref().ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("Dataset params not set".to_string()),
                0,
            )
        })?;

        let groups = sg3d.groups();
        if groups.is_empty() {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField("SG3D has no data".to_string()),
                0,
            ));
        }

        // Extract coordinate triplets and convert to exact BigRational
        let mut lat = Vec::with_capacity(groups.len());
        let mut lon = Vec::with_capacity(groups.len());
        let mut depth = Vec::with_capacity(groups.len());

        for group in groups {
            let y = Self::get_int(group, "YCOO").ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG3D missing YCOO".to_string()),
                    0,
                )
            })?;
            let x = Self::get_int(group, "XCOO").ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG3D missing XCOO".to_string()),
                    0,
                )
            })?;
            let z = Self::get_int(group, "VE3D").ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG3D missing VE3D".to_string()),
                    0,
                )
            })?;

            // lat = y / COMF (degrees)
            lat.push(BigRational::new(BigInt::from(y), params.comf.clone()));
            // lon = x / COMF (degrees)
            lon.push(BigRational::new(BigInt::from(x), params.comf.clone()));
            // depth = z / SOMF (DUNI units, typically metres)
            depth.push(BigRational::new(BigInt::from(z), params.somf.clone()));
        }

        // Create ExactPositions component
        world
            .exact_positions
            .insert(entity, ExactPositions { lat, lon });

        // Create ExactDepths component
        world.exact_depths.insert(
            entity,
            ExactDepths {
                depth,
                units: params.duni,
            },
        );

        Ok(())
    }

    /// Helper: extract integer value from subfield group
    fn get_int(group: &[(String, SubfieldValue)], label: &str) -> Option<i32> {
        group
            .iter()
            .find(|(l, _)| l == label)
            .and_then(|(_, v)| v.as_int())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use s57_parse::ddr::SubfieldValue;

    #[test]
    #[ignore] // TODO: Requires proper DDR setup for ParsedField mocking
    fn test_name_decode_system_creates_entity() {
        let mut world = World::new();

        // Create mock VRID field
        let vrid_data = vec![
            ("RCNM".to_string(), SubfieldValue::Integer(110)),
            ("RCID".to_string(), SubfieldValue::Integer(42)),
            ("RVER".to_string(), SubfieldValue::Integer(1)),
            ("RUIN".to_string(), SubfieldValue::Integer(1)),
        ];

        let vrid = create_mock_parsed_field("VRID", vec![vrid_data]);

        let entity = NameDecodeSystem::process_vrid(&mut world, &vrid).unwrap();

        // Verify entity created
        assert!(world.is_valid(entity));
        assert_eq!(world.entity_type(entity), Some(EntityType::Vector));

        // Verify VectorMeta component
        let meta = world.vector_meta.get(&entity).unwrap();
        assert_eq!(meta.name.rcnm, 110);
        assert_eq!(meta.name.rcid, 42);
        assert_eq!(meta.rver, 1);
        assert_eq!(meta.ruin, 1);

        // Verify name_index populated
        let name = NameKey {
            rcnm: 110,
            rcid: 42,
        };
        assert_eq!(world.name_index.get(&name), Some(&entity));
    }

    // Helper to create mock ParsedField for testing
    fn create_mock_parsed_field(
        _tag: &str,
        _groups: Vec<Vec<(String, SubfieldValue)>>,
    ) -> ParsedField<'static> {
        // This is a simplified mock - in real code ParsedField has lifetime tied to FieldDef
        // For testing purposes, we'll use unsafe to extend the lifetime
        // In production code, this would be handled properly with actual DDR parsing
        unimplemented!("Mock ParsedField creation needs proper DDR setup")
    }
}
