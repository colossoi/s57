//! Entity-Component System for S-57 data
//!
//! This module provides an ECS architecture for organizing parsed S-57 data
//! into rendering-ready entities with components.
//!
//! Design principles:
//! - EntityId: slotmap keys for stable, generational references
//! - Components: stored in Vec-based SoA (Structure of Arrays) layout
//! - Sparse storage: HashMap<EntityId, ComponentData> for optional components
//! - Systems: pure functions that operate on component slices

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use s57_parse::bitstring::{FoidKey, NameKey};
use slotmap::{new_key_type, SlotMap};
use std::collections::HashMap;

// Define EntityId as a slotmap key type (generational index)
new_key_type! {
    /// EntityId: Stable reference to an entity
    ///
    /// Uses slotmap for generational indices - can detect stale references
    /// and reuse slots after entity deletion.
    pub struct EntityId;
}

/// World: Top-level container for all entities and components
///
/// Follows ECS pattern with:
/// - entities: SlotMap for entity lifecycle management
/// - components: Separate storage for each component type
/// - indices: Fast lookup from S-57 keys to EntityId
#[derive(Debug, Default)]
pub struct World {
    /// Entity allocator (slotmap handles generation/reuse)
    entities: SlotMap<EntityId, EntityMeta>,

    /// Fast lookups from S-57 keys to entities
    pub name_index: HashMap<NameKey, EntityId>,
    pub foid_index: HashMap<FoidKey, EntityId>,

    /// Component storage (sparse - not all entities have all components)
    pub dataset_params: Option<DatasetParams>,
    pub vector_meta: HashMap<EntityId, VectorMeta>,
    pub vector_topology: HashMap<EntityId, VectorTopology>,
    pub feature_meta: HashMap<EntityId, FeatureMeta>,
    pub feature_attributes: HashMap<EntityId, FeatureAttributes>,
    pub feature_pointers: HashMap<EntityId, FeaturePointers>,
    pub exact_positions: HashMap<EntityId, ExactPositions>,
    pub exact_depths: HashMap<EntityId, ExactDepths>,
}

/// EntityMeta: Minimal metadata stored in slotmap
#[derive(Debug, Clone)]
pub struct EntityMeta {
    /// Entity type tag (for debugging/filtering)
    pub entity_type: EntityType,
}

/// EntityType: Categorizes entities for filtering/debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    /// Vector record (spatial geometry)
    Vector,
    /// Feature record (semantic object)
    Feature,
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate a new entity with given type
    pub fn create_entity(&mut self, entity_type: EntityType) -> EntityId {
        self.entities.insert(EntityMeta { entity_type })
    }

    /// Remove an entity and all its components
    pub fn remove_entity(&mut self, entity: EntityId) {
        self.entities.remove(entity);
        self.vector_meta.remove(&entity);
        self.vector_topology.remove(&entity);
        self.feature_meta.remove(&entity);
        self.feature_attributes.remove(&entity);
        self.feature_pointers.remove(&entity);
        self.exact_positions.remove(&entity);
        self.exact_depths.remove(&entity);
    }

    /// Check if an entity exists (not deleted)
    pub fn is_valid(&self, entity: EntityId) -> bool {
        self.entities.contains_key(entity)
    }

    /// Get entity type
    pub fn entity_type(&self, entity: EntityId) -> Option<EntityType> {
        self.entities.get(entity).map(|meta| meta.entity_type)
    }

    /// Get all entities of a given type
    pub fn entities_of_type(&self, entity_type: EntityType) -> Vec<EntityId> {
        self.entities
            .iter()
            .filter(|(_, meta)| meta.entity_type == entity_type)
            .map(|(id, _)| id)
            .collect()
    }
}

//
// Component definitions
//

/// DatasetParams: Global dataset parameters from DSPM record
///
/// These define the scaling factors for converting raw integer coordinates
/// to exact lat/lon and depth values.
#[derive(Debug, Clone)]
pub struct DatasetParams {
    /// Coordinate multiplication factor (e.g., 10,000,000 for 7 decimal places)
    pub comf: BigInt,
    /// Sounding (depth) multiplication factor
    pub somf: BigInt,
    /// Units of depth (1=metres, 2=fathoms/feet, 3=feet, 4=fathoms/fractions)
    pub duni: u16,
    /// Units of height (1=metres, 2=feet)
    pub huni: u16,
    /// Units of positional accuracy (1=metres, 2=degrees)
    pub puni: u16,
    /// Horizontal geodetic datum (e.g., WGS84)
    pub hdat: u16,
    /// Vertical datum (e.g., mean lower low water)
    pub vdat: u16,
    /// Sounding datum (e.g., mean lower low water)
    pub sdat: u16,
    /// Compilation scale (e.g., 80000 for 1:80000)
    pub cscl: u32,
}

/// VectorMeta: Metadata for vector (spatial) records
#[derive(Debug, Clone)]
pub struct VectorMeta {
    /// NAME key (rcnm, rcid) for cross-references
    pub name: NameKey,
    /// Record version
    pub rver: u16,
    /// Record update instruction (1=insert, 2=delete, 3=modify)
    pub ruin: u8,
}

/// VectorTopology: Vector relationships from VRPT field
///
/// Defines how vectors connect to form edges, faces, etc.
#[derive(Debug, Clone)]
pub struct VectorTopology {
    /// Neighboring vector NAMEs with topology flags
    pub neighbors: Vec<VectorNeighbor>,
}

/// VectorNeighbor: Single neighbor relationship from VRPT
#[derive(Debug, Clone, Copy)]
pub struct VectorNeighbor {
    /// NAME of neighboring vector
    pub name: NameKey,
    /// Orientation (1=forward, 2=reverse, 255=not relevant)
    pub ornt: u8,
    /// Usage indicator (1=exterior, 2=interior, 3=exterior boundary truncated)
    pub usag: u8,
    /// Topology indicator (1=beginning node, 2=end node, 3=left face, 4=right face, etc.)
    pub topi: u8,
    /// Masking indicator (1=mask, 2=show, 255=not relevant)
    pub mask: u8,
}

/// FeatureMeta: Metadata for feature (semantic object) records
#[derive(Debug, Clone)]
pub struct FeatureMeta {
    /// Feature object identifier (agen, fidn, fids)
    pub foid: FoidKey,
    /// Primitive type (1=point, 2=line, 3=area, 255=not applicable)
    pub prim: u8,
    /// Group (1=geo, 2=meta, 3=collection, 4=national, 5=chart)
    pub grup: u8,
    /// Object label/class code (e.g., 42=buoy, 301=depth contour)
    pub objl: u16,
    /// Record version
    pub rver: u16,
    /// Record update instruction (1=insert, 2=delete, 3=modify)
    pub ruin: u8,
}

/// FeatureAttributes: Attributes from ATTF/NATF fields
///
/// Stores attribute label (ATTL) and value (ATVL) pairs.
#[derive(Debug, Clone, Default)]
pub struct FeatureAttributes {
    /// Feature record attributes (ATTF)
    pub attf: Vec<(u16, String)>,
    /// National attributes (NATF)
    pub natf: Vec<(u16, String)>,
}

/// FeaturePointers: Cross-references from FFPT/FSPT fields
///
/// Links features to other features (FFPT) and to spatial vectors (FSPT).
#[derive(Debug, Clone, Default)]
pub struct FeaturePointers {
    /// Feature-to-feature pointers (FFPT): related features by LNAM
    pub related_features: Vec<FoidKey>,
    /// Feature-to-spatial pointers (FSPT): spatial vectors by NAME with flags
    pub spatial_refs: Vec<SpatialRef>,
}

/// SpatialRef: Single spatial reference from FSPT
#[derive(Debug, Clone, Copy)]
pub struct SpatialRef {
    /// NAME of referenced vector
    pub name: NameKey,
    /// Orientation (1=forward, 2=reverse, 255=not relevant)
    pub ornt: u8,
    /// Usage indicator
    pub usag: u8,
    /// Masking indicator
    pub mask: u8,
}

/// ExactPositions: Exact lat/lon coordinates (BigRational)
///
/// Computed from SG2D/SG3D fields by dividing by COMF:
/// lat = y / COMF (degrees), lon = x / COMF (degrees)
///
/// All processing should use exact math. Convert to f64 only at render time.
#[derive(Debug, Clone)]
pub struct ExactPositions {
    /// Latitude in degrees (exact rational)
    pub lat: Vec<BigRational>,
    /// Longitude in degrees (exact rational)
    pub lon: Vec<BigRational>,
}

impl ExactPositions {
    /// Convert to f64 for rendering (on-demand, not cached)
    ///
    /// Returns (lat, lon) vectors as f64. Use only at final rendering boundary.
    pub fn to_f64(&self) -> (Vec<f64>, Vec<f64>) {
        let lat_f64 = self.lat.iter().map(|r| r.to_f64().unwrap_or(0.0)).collect();
        let lon_f64 = self.lon.iter().map(|r| r.to_f64().unwrap_or(0.0)).collect();
        (lat_f64, lon_f64)
    }
}

/// ExactDepths: Exact depth values (BigRational)
///
/// Computed from SG3D fields by dividing by SOMF:
/// depth = z / SOMF (in DUNI units, typically metres)
///
/// All processing should use exact math. Convert to f64 only at render time.
#[derive(Debug, Clone)]
pub struct ExactDepths {
    /// Depth values (exact rational, positive down)
    pub depth: Vec<BigRational>,
    /// Units (from DUNI: 1=metres, 2=fathoms/feet, etc.)
    pub units: u16,
}

impl ExactDepths {
    /// Convert to f64 for rendering (on-demand, not cached)
    ///
    /// Returns depths as f64. Use only at final rendering boundary.
    /// TODO: Add unit conversion if units != 1 (metres)
    pub fn to_f64(&self) -> Vec<f64> {
        self.depth
            .iter()
            .map(|r| r.to_f64().unwrap_or(0.0))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_create_entity() {
        let mut world = World::new();
        let entity = world.create_entity(EntityType::Vector);
        assert!(world.is_valid(entity));
        assert_eq!(world.entity_type(entity), Some(EntityType::Vector));
    }

    #[test]
    fn test_world_remove_entity() {
        let mut world = World::new();
        let entity = world.create_entity(EntityType::Feature);
        world.remove_entity(entity);
        assert!(!world.is_valid(entity));
    }

    #[test]
    fn test_world_entities_of_type() {
        let mut world = World::new();
        let v1 = world.create_entity(EntityType::Vector);
        let _f1 = world.create_entity(EntityType::Feature);
        let v2 = world.create_entity(EntityType::Vector);

        let vectors = world.entities_of_type(EntityType::Vector);
        assert_eq!(vectors.len(), 2);
        assert!(vectors.contains(&v1));
        assert!(vectors.contains(&v2));
    }
}
