//! Cursors for iterating over resolved geometry
//!
//! Provides high-level iterators over edges, rings, and feature boundaries.

use super::walker::EdgeWalker;
use super::{errors::TopologyResult, TraversalContext};
use num_rational::BigRational;
use s57_parse::bitstring::FoidKey;

/// Cursor for iterating over feature boundaries
///
/// Resolves FSPT pointers to build complete boundary rings for area features
pub struct FeatureBoundaryCursor<'a> {
    ctx: &'a TraversalContext<'a>,
    foid: FoidKey,
}

impl<'a> FeatureBoundaryCursor<'a> {
    /// Create a new feature boundary cursor
    pub fn new(ctx: &'a TraversalContext<'a>, foid: FoidKey) -> Self {
        Self { ctx, foid }
    }

    /// Resolve all boundary rings for this feature
    ///
    /// Returns vector of rings, where first is outer boundary and rest are holes.
    /// Each ring is a closed polyline (first point == last point).
    ///
    /// # Algorithm
    ///
    /// 1. Look up feature data by FOID
    /// 2. Get FSPT spatial pointers
    /// 3. Group pointers by USAG (1=exterior, 2=interior/hole)
    /// 4. For each group, resolve edges via EdgeWalker
    /// 5. Stitch edges into rings
    /// 6. Close rings if not already closed
    pub fn resolve_rings(&self) -> TopologyResult<Vec<Vec<(BigRational, BigRational)>>> {
        // Look up feature entity by FOID
        let entity = self
            .ctx
            .world
            .foid_index
            .get(&self.foid)
            .copied()
            .ok_or_else(|| super::errors::TopologyError::NoGeometry {
                vector: s57_parse::bitstring::NameKey {
                    rcnm: 100, // Feature record
                    rcid: self.foid.fidn,
                },
            })?;

        // Get spatial references (FSPT pointers)
        let feature_pointers = self
            .ctx
            .world
            .feature_pointers
            .get(&entity)
            .ok_or_else(|| super::errors::TopologyError::NoGeometry {
                vector: s57_parse::bitstring::NameKey {
                    rcnm: 100,
                    rcid: self.foid.fidn,
                },
            })?;

        if feature_pointers.spatial_refs.is_empty() {
            // No spatial references - return empty
            return Ok(vec![]);
        }

        // Create edge walker for resolving vectors
        let mut walker = EdgeWalker::new(self.ctx);

        // Separate exterior (USAG=1) from interior/holes (USAG=2)
        let exterior_refs: Vec<_> = feature_pointers
            .spatial_refs
            .iter()
            .filter(|r| r.usag == 1)
            .collect();

        let interior_refs: Vec<_> = feature_pointers
            .spatial_refs
            .iter()
            .filter(|r| r.usag == 2)
            .collect();

        let mut rings = Vec::new();

        // Resolve exterior rings (USAG=1)
        // Note: typically there's one exterior ring, but could be multiple disconnected boundaries
        if !exterior_refs.is_empty() {
            // Try resolving all exterior refs as one connected ring
            let exterior_ring = self.resolve_ring_from_refs(&mut walker, &exterior_refs)?;
            if !exterior_ring.is_empty() {
                rings.push(exterior_ring);
            }
            // TODO: handle case where exterior refs form multiple disconnected rings
            // This would require connectivity analysis to group refs into separate rings
        }

        // Resolve interior rings (USAG=2) - holes/islands
        // Each interior ref typically represents a separate island/hole
        // If an island boundary spans multiple edges, they should be sequential in the FSPT list
        if !interior_refs.is_empty() {
            // Current approach: each interior ref is resolved as its own ring
            // This works when each ref is either:
            // - A single closed edge forming a complete island boundary
            // - Part of a sequence that resolve_ring_from_refs will stitch together
            for iref in interior_refs {
                let interior_ring = self.resolve_ring_from_refs(&mut walker, &[iref])?;
                if !interior_ring.is_empty() {
                    rings.push(interior_ring);
                }
            }
            // TODO: For complex cases with multiple edges per island, implement connectivity
            // analysis to group interior refs by which island they belong to
        }

        Ok(rings)
    }

    /// Resolve a single ring from a set of FSPT references
    fn resolve_ring_from_refs(
        &self,
        walker: &mut EdgeWalker,
        refs: &[&crate::ecs::SpatialRef],
    ) -> TopologyResult<Vec<(BigRational, BigRational)>> {
        let mut ring: Vec<(BigRational, BigRational)> = Vec::new();

        for sref in refs {
            // Get the vector NAME from the entity
            let vector_name = self
                .ctx
                .world
                .vector_meta
                .get(&sref.entity)
                .map(|meta| meta.name)
                .ok_or_else(|| super::errors::TopologyError::DanglingReference {
                    from: s57_parse::bitstring::NameKey {
                        rcnm: 100,
                        rcid: self.foid.fidn,
                    },
                    to: s57_parse::bitstring::NameKey { rcnm: 0, rcid: 0 },
                })?;

            // Resolve the edge geometry
            let mut edge_coords = walker.resolve_line_2d(vector_name)?;

            // Apply orientation
            let ornt = super::types::Orientation::from_ornt(sref.ornt);
            if ornt.should_reverse() {
                edge_coords.reverse();
            }

            if edge_coords.is_empty() {
                continue; // Skip empty edges
            }

            // Append to ring (avoiding duplicate endpoints)
            if !ring.is_empty() && !edge_coords.is_empty() {
                let last_pt = ring.last().unwrap();
                let first_pt = &edge_coords[0];

                // If endpoints match, skip the duplicate
                if last_pt.0 == first_pt.0 && last_pt.1 == first_pt.1 {
                    ring.extend_from_slice(&edge_coords[1..]);
                } else {
                    // Endpoints don't match - just append
                    // (continuity policy in walker already handled this)
                    ring.extend(edge_coords);
                }
            } else {
                ring.extend(edge_coords);
            }
        }

        // Close the ring if not already closed
        if !ring.is_empty() && !Self::is_closed(&ring) {
            let first = ring[0].clone();
            ring.push(first);
        }

        Ok(ring)
    }

    /// Check if a ring is closed (first == last point)
    fn is_closed(ring: &[(BigRational, BigRational)]) -> bool {
        if ring.len() < 2 {
            return false;
        }
        let first = &ring[0];
        let last = &ring[ring.len() - 1];
        first.0 == last.0 && first.1 == last.1
    }
}
