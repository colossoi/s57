//! Systems for processing S-57 records into ECS components
//!
//! Systems are pure functions that transform parsed S-57 data into
//! structured entities and components. Each system focuses on a specific
//! transformation step in the pipeline.

use crate::ecs::{
    EntityType, ExactDepths, ExactPositions, FeatureMeta, FeaturePointers, SpatialRef, VectorMeta,
    VectorNeighbor, VectorTopology, World,
};
use num_bigint::BigInt;
use num_rational::BigRational;
use s57_parse::bitstring::{FoidKey, NameKey};
use s57_parse::ddr::{ParsedField, SubfieldValue};
use s57_parse::{ParseError, ParseErrorKind, Result};

/// Helper: Extract u8 from subfield group
/// Returns Ok(None) if field not present, Err if present but wrong type or out of range
pub(crate) fn get_u8(group: &[(String, SubfieldValue)], label: &str) -> Result<Option<u8>> {
    match group.iter().find(|(l, _)| l == label) {
        None => Ok(None),
        Some((_, SubfieldValue::Integer(i))) if *i >= 0 && *i <= u8::MAX as i32 => {
            Ok(Some(*i as u8))
        }
        Some((_, SubfieldValue::UnsignedInteger(u))) if *u <= u8::MAX as u32 => Ok(Some(*u as u8)),
        Some((_, _)) => Err(ParseError::at(
            ParseErrorKind::InvalidField(format!(
                "{} has wrong type or value out of range for u8",
                label
            )),
            0,
        )),
    }
}

/// Helper: Extract u16 from subfield group
/// Returns Ok(None) if field not present, Err if present but wrong type or out of range
pub(crate) fn get_u16(group: &[(String, SubfieldValue)], label: &str) -> Result<Option<u16>> {
    match group.iter().find(|(l, _)| l == label) {
        None => Ok(None),
        Some((_, SubfieldValue::Integer(i))) if *i >= 0 && *i <= u16::MAX as i32 => {
            Ok(Some(*i as u16))
        }
        Some((_, SubfieldValue::UnsignedInteger(u))) if *u <= u16::MAX as u32 => {
            Ok(Some(*u as u16))
        }
        Some((_, _)) => Err(ParseError::at(
            ParseErrorKind::InvalidField(format!(
                "{} has wrong type or value out of range for u16",
                label
            )),
            0,
        )),
    }
}

/// Helper: Extract u32 from subfield group
/// Returns Ok(None) if field not present, Err if present but wrong type or negative
pub(crate) fn get_u32(group: &[(String, SubfieldValue)], label: &str) -> Result<Option<u32>> {
    match group.iter().find(|(l, _)| l == label) {
        None => Ok(None),
        Some((_, SubfieldValue::Integer(i))) if *i >= 0 => Ok(Some(*i as u32)),
        Some((_, SubfieldValue::UnsignedInteger(u))) => Ok(Some(*u)),
        Some((_, _)) => Err(ParseError::at(
            ParseErrorKind::InvalidField(format!(
                "{} has wrong type or negative value for u32",
                label
            )),
            0,
        )),
    }
}

/// Helper: Extract i32 from subfield group
/// Returns Ok(None) if field not present, Err if present but wrong type
pub(crate) fn get_i32(group: &[(String, SubfieldValue)], label: &str) -> Result<Option<i32>> {
    match group.iter().find(|(l, _)| l == label) {
        None => Ok(None),
        Some((_, SubfieldValue::Integer(i))) => Ok(Some(*i)),
        Some((_, SubfieldValue::UnsignedInteger(u))) if *u <= i32::MAX as u32 => {
            Ok(Some(*u as i32))
        }
        Some((_, _)) => Err(ParseError::at(
            ParseErrorKind::InvalidField(format!(
                "{} has wrong type or value out of range for i32",
                label
            )),
            0,
        )),
    }
}

/// Helper: Extract bytes from subfield group
/// Returns Ok(None) if field not present, Err if present but wrong type
fn get_bytes<'a>(group: &'a [(String, SubfieldValue)], label: &str) -> Result<Option<&'a [u8]>> {
    match group.iter().find(|(l, _)| l == label) {
        None => Ok(None),
        Some((_, SubfieldValue::Bytes(bytes))) => Ok(Some(bytes.as_slice())),
        Some((_, _)) => Err(ParseError::at(
            ParseErrorKind::InvalidField(format!("{} has wrong type, expected bytes", label)),
            0,
        )),
    }
}

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
        let rcnm = get_u8(group, "RCNM")?.ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("VRID missing RCNM".to_string()),
                0,
            )
        })?;

        // Extract RCID (required)
        let rcid = get_u32(group, "RCID")?.ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("VRID missing RCID".to_string()),
                0,
            )
        })?;

        // Extract RVER (optional, default 1)
        let rver = get_u16(group, "RVER")?.unwrap_or(1);

        // Extract RUIN (optional, default 1=insert)
        let ruin = get_u8(group, "RUIN")?.unwrap_or(1);

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

        let _rcnm = get_u8(frid_group, "RCNM")?.ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FRID missing RCNM".to_string()),
                0,
            )
        })?;

        let _rcid = get_u32(frid_group, "RCID")?.ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FRID missing RCID".to_string()),
                0,
            )
        })?;

        let prim = get_u8(frid_group, "PRIM")?.unwrap_or(255);
        let grup = get_u8(frid_group, "GRUP")?.unwrap_or(1);
        let objl = get_u16(frid_group, "OBJL")?.unwrap_or(0);
        let rver = get_u16(frid_group, "RVER")?.unwrap_or(1);
        let ruin = get_u8(frid_group, "RUIN")?.unwrap_or(1);

        // Extract FOID subfields
        let foid_group = foid.groups().first().ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID has no data".to_string()),
                0,
            )
        })?;

        let agen = get_u16(foid_group, "AGEN")?.ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID missing AGEN".to_string()),
                0,
            )
        })?;

        let fidn = get_u32(foid_group, "FIDN")?.ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID missing FIDN".to_string()),
                0,
            )
        })?;

        let fids = get_u16(foid_group, "FIDS")?.ok_or_else(|| {
            ParseError::at(
                ParseErrorKind::InvalidField("FOID missing FIDS".to_string()),
                0,
            )
        })?;

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
            let y = get_i32(group, "YCOO")?.ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG2D missing YCOO".to_string()),
                    0,
                )
            })?;
            let x = get_i32(group, "XCOO")?.ok_or_else(|| {
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
            let y = get_i32(group, "YCOO")?.ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG3D missing YCOO".to_string()),
                    0,
                )
            })?;
            let x = get_i32(group, "XCOO")?.ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("SG3D missing XCOO".to_string()),
                    0,
                )
            })?;
            let z = get_i32(group, "VE3D")?.ok_or_else(|| {
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
}

/// TopologySystem: Process VRPT records to build vector topology
///
/// Extracts vector topology relationships from VRPT (Vector Record Pointer) fields:
/// - NAME: pointer to neighboring vector (B40 bitstring)
/// - ORNT: orientation (1=forward, 2=reverse, 255=N/A)
/// - USAG: usage (1=exterior, 2=interior, 3=exterior truncated)
/// - TOPI: topology indicator (1=begin node, 2=end node, 3=left face, 4=right face, etc.)
/// - MASK: masking (1=mask, 2=show, 255=N/A)
///
/// Creates VectorTopology component with list of VectorNeighbor relationships.
///
/// Input: ParsedField from VRPT
/// Output: VectorTopology component
pub struct TopologySystem;

impl TopologySystem {
    /// Process VRPT field to extract topology relationships
    ///
    /// # Arguments
    /// * `world` - ECS world to update
    /// * `entity` - Entity to attach topology to
    /// * `vrpt` - Parsed VRPT field
    ///
    /// # Returns
    /// Ok(()) if successful, or ParseError if data missing
    pub fn process_vrpt(
        world: &mut World,
        entity: crate::ecs::EntityId,
        vrpt: &ParsedField,
    ) -> Result<()> {
        let groups = vrpt.groups();
        if groups.is_empty() {
            return Err(ParseError::at(
                ParseErrorKind::InvalidField("VRPT has no data".to_string()),
                0,
            ));
        }

        // Extract topology relationships from repeating groups
        let mut neighbors = Vec::with_capacity(groups.len());

        for group in groups {
            // Extract NAME (B40 bitstring - 5 bytes)
            let name_bytes = get_bytes(group, "NAME")?.ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("VRPT missing NAME".to_string()),
                    0,
                )
            })?;

            // Decode NAME bitstring to NameKey
            let name = NameKey::decode(name_bytes).map_err(|e| {
                ParseError::at(
                    ParseErrorKind::InvalidField(format!("Failed to decode NAME: {}", e)),
                    0,
                )
            })?;

            // Resolve NAME to EntityId via name_index
            let neighbor_entity = *world.name_index.get(&name).ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField(format!(
                        "Referenced vector NAME not found: rcnm={}, rcid={}",
                        name.rcnm, name.rcid
                    )),
                    0,
                )
            })?;

            // Extract orientation (optional, default 255=N/A)
            let ornt = get_u8(group, "ORNT")?.unwrap_or(255);

            // Extract usage (optional, default 255=N/A)
            let usag = get_u8(group, "USAG")?.unwrap_or(255);

            // Extract topology indicator (optional, default 255=N/A)
            let topi = get_u8(group, "TOPI")?.unwrap_or(255);

            // Extract masking (optional, default 255=N/A)
            let mask = get_u8(group, "MASK")?.unwrap_or(255);

            neighbors.push(VectorNeighbor {
                entity: neighbor_entity,
                ornt,
                usag,
                topi,
                mask,
            });
        }

        // Create VectorTopology component
        world
            .vector_topology
            .insert(entity, VectorTopology { neighbors });

        Ok(())
    }
}

/// FeatureBindSystem: Process FSPT/FFPT records to link features to vectors/features
///
/// Extracts feature relationship pointers:
/// - FSPT (Feature-to-Spatial): Links features to vector entities (geometry)
///   - NAME: pointer to vector (B40 bitstring)
///   - ORNT: orientation (1=forward, 2=reverse, 255=N/A)
///   - USAG: usage indicator
///   - MASK: masking indicator
/// - FFPT (Feature-to-Feature): Links features to other features
///   - LNAM: pointer to feature (B64 bitstring)
///   - RIND: relationship indicator
///
/// Creates/updates FeaturePointers component with EntityId references.
///
/// Input: ParsedField from FSPT or FFPT
/// Output: FeaturePointers component
pub struct FeatureBindSystem;

impl FeatureBindSystem {
    /// Process FSPT field to extract feature-to-spatial pointers
    ///
    /// # Arguments
    /// * `world` - ECS world with indices
    /// * `entity` - Feature entity to update
    /// * `fspt` - Parsed FSPT field
    ///
    /// # Returns
    /// Ok(()) if successful, or ParseError if data missing
    pub fn process_fspt(
        world: &mut World,
        entity: crate::ecs::EntityId,
        fspt: &ParsedField,
    ) -> Result<()> {
        let groups = fspt.groups();
        if groups.is_empty() {
            // FSPT can be empty (feature with no spatial)
            return Ok(());
        }

        // Extract spatial references from repeating groups
        let mut spatial_refs = Vec::with_capacity(groups.len());

        for group in groups {
            // Extract NAME (B40 bitstring - 5 bytes)
            let name_bytes = get_bytes(group, "NAME")?.ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("FSPT missing NAME".to_string()),
                    0,
                )
            })?;

            // Decode NAME bitstring to NameKey
            let name = NameKey::decode(name_bytes).map_err(|e| {
                ParseError::at(
                    ParseErrorKind::InvalidField(format!("Failed to decode NAME: {}", e)),
                    0,
                )
            })?;

            // Resolve NAME to EntityId via name_index
            let vector_entity = *world.name_index.get(&name).ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField(format!(
                        "Referenced vector NAME not found: rcnm={}, rcid={}",
                        name.rcnm, name.rcid
                    )),
                    0,
                )
            })?;

            // Extract flags (optional, default 255=N/A)
            let ornt = get_u8(group, "ORNT")?.unwrap_or(255);
            let usag = get_u8(group, "USAG")?.unwrap_or(255);
            let mask = get_u8(group, "MASK")?.unwrap_or(255);

            spatial_refs.push(SpatialRef {
                entity: vector_entity,
                ornt,
                usag,
                mask,
            });
        }

        // Get or create FeaturePointers component
        let pointers = world
            .feature_pointers
            .entry(entity)
            .or_insert_with(FeaturePointers::default);

        pointers.spatial_refs = spatial_refs;

        Ok(())
    }

    /// Process FFPT field to extract feature-to-feature pointers
    ///
    /// # Arguments
    /// * `world` - ECS world with indices
    /// * `entity` - Feature entity to update
    /// * `ffpt` - Parsed FFPT field
    ///
    /// # Returns
    /// Ok(()) if successful, or ParseError if data missing
    pub fn process_ffpt(
        world: &mut World,
        entity: crate::ecs::EntityId,
        ffpt: &ParsedField,
    ) -> Result<()> {
        let groups = ffpt.groups();
        if groups.is_empty() {
            // FFPT can be empty (no related features)
            return Ok(());
        }

        // Extract feature references from repeating groups
        let mut related_features = Vec::with_capacity(groups.len());

        for group in groups {
            // Extract LNAM (B64 bitstring - 8 bytes)
            let lnam_bytes = get_bytes(group, "LNAM")?.ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField("FFPT missing LNAM".to_string()),
                    0,
                )
            })?;

            // Decode LNAM bitstring to FoidKey
            let foid = FoidKey::decode(lnam_bytes).map_err(|e| {
                ParseError::at(
                    ParseErrorKind::InvalidField(format!("Failed to decode LNAM: {}", e)),
                    0,
                )
            })?;

            // Resolve LNAM to EntityId via foid_index
            let feature_entity = *world.foid_index.get(&foid).ok_or_else(|| {
                ParseError::at(
                    ParseErrorKind::InvalidField(format!(
                        "Referenced feature LNAM not found: agen={}, fidn={}, fids={}",
                        foid.agen, foid.fidn, foid.fids
                    )),
                    0,
                )
            })?;

            related_features.push(feature_entity);
        }

        // Get or create FeaturePointers component
        let pointers = world
            .feature_pointers
            .entry(entity)
            .or_insert_with(FeaturePointers::default);

        pointers.related_features = related_features;

        Ok(())
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
