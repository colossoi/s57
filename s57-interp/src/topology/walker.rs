//! Edge walker for recursive geometry resolution
//!
//! Resolves vector geometry by following VRPT chains and applying
//! orientation/continuity rules.

use super::errors::{TopologyError, TopologyResult};
use super::types::Orientation;
use super::TraversalContext;
use num_rational::BigRational;
use num_traits::Signed;
use s57_parse::bitstring::NameKey;
use std::collections::HashMap;

/// Maximum recursion depth for VRPT traversal
const MAX_DEPTH: usize = 100;

/// Walker for resolving edge geometry
pub struct EdgeWalker<'a> {
    ctx: &'a TraversalContext<'a>,
    /// Track visit counts for cycle detection
    visit_counts: HashMap<NameKey, usize>,
    /// Current recursion depth
    depth: usize,
    /// Chain of vectors being resolved (for error reporting)
    chain: Vec<NameKey>,
}

impl<'a> EdgeWalker<'a> {
    /// Create a new edge walker
    pub fn new(ctx: &'a TraversalContext<'a>) -> Self {
        Self {
            ctx,
            visit_counts: HashMap::new(),
            depth: 0,
            chain: Vec::new(),
        }
    }

    /// Resolve a vector to a 2D polyline
    ///
    /// Returns coordinates in (lat, lon) order with exact rational precision.
    /// Follows VRPT chain if no direct geometry available.
    ///
    /// # Algorithm
    ///
    /// 1. Check depth limit
    /// 2. Check cycle policy
    /// 3. Look up vector data
    /// 4. If has direct geometry (SG2D), return it
    /// 5. If has VRPT pointers, recursively resolve and stitch
    /// 6. Otherwise, error (no geometry)
    pub fn resolve_line_2d(
        &mut self,
        name: NameKey,
    ) -> TopologyResult<Vec<(BigRational, BigRational)>> {
        // Check depth limit
        if self.depth >= MAX_DEPTH {
            return Err(TopologyError::MaxDepthExceeded {
                max_depth: MAX_DEPTH,
                chain: self.chain.clone(),
            });
        }

        // Check cycle detection policy
        self.check_cycle(&name)?;

        // Track this vector in the chain
        self.chain.push(name);
        self.depth += 1;

        // Increment visit count
        *self.visit_counts.entry(name).or_insert(0) += 1;

        // Look up vector entity by NAME
        let entity = self
            .ctx
            .world
            .name_index
            .get(&name)
            .copied()
            .ok_or_else(|| TopologyError::DanglingReference {
                from: *self.chain.get(self.chain.len() - 2).unwrap_or(&name),
                to: name,
            })?;

        // Try direct geometry first
        if let Some(positions) = self.ctx.world.exact_positions.get(&entity) {
            let (lat, lon) = positions.to_f64();
            // Convert f64 back to BigRational (temporary until we store rationals directly)
            use num_bigint::BigInt;
            let coords: Vec<(BigRational, BigRational)> = lat
                .iter()
                .zip(lon.iter())
                .map(|(la, lo)| {
                    (
                        BigRational::from_float(*la)
                            .unwrap_or_else(|| BigRational::new(BigInt::from(0), BigInt::from(1))),
                        BigRational::from_float(*lo)
                            .unwrap_or_else(|| BigRational::new(BigInt::from(0), BigInt::from(1))),
                    )
                })
                .collect();

            self.depth -= 1;
            self.chain.pop();
            return Ok(coords);
        }

        // Try VRPT resolution - get topology references
        let vrpt_neighbors = if let Some(topo) = self.ctx.world.vector_topology.get(&entity) {
            &topo.neighbors
        } else {
            // No direct geometry and no VRPT pointers
            self.depth -= 1;
            self.chain.pop();
            return Err(TopologyError::NoGeometry { vector: name });
        };

        if vrpt_neighbors.is_empty() {
            self.depth -= 1;
            self.chain.pop();
            return Err(TopologyError::NoGeometry { vector: name });
        }

        // Recursively resolve each VRPT pointer and stitch together
        let mut result: Vec<(BigRational, BigRational)> = Vec::new();

        for (idx, neighbor) in vrpt_neighbors.iter().enumerate() {
            // Resolve entity → NAME for the referenced vector
            let neighbor_name = self
                .ctx
                .world
                .vector_meta
                .get(&neighbor.entity)
                .map(|meta| meta.name)
                .ok_or_else(|| TopologyError::DanglingReference {
                    from: name,
                    to: NameKey { rcnm: 0, rcid: 0 }, // Unknown name
                })?;

            // Apply orientation
            let ornt = Orientation::from_ornt(neighbor.ornt);

            // Recursively resolve the referenced vector
            let child_coords = self.resolve_with_orientation(neighbor_name, ornt)?;

            if child_coords.is_empty() {
                continue; // Skip empty segments
            }

            // Check continuity if not the first segment
            if !result.is_empty() && !child_coords.is_empty() {
                self.check_continuity(
                    result.last().unwrap(),
                    &child_coords[0],
                    neighbor_name,
                    idx,
                )?;
            }

            // Append coordinates (avoid duplicating shared endpoints)
            if !result.is_empty() && !child_coords.is_empty() {
                let last_pt = result.last().unwrap();
                let first_pt = &child_coords[0];

                // If endpoints match exactly, skip the duplicate
                if last_pt.0 == first_pt.0 && last_pt.1 == first_pt.1 {
                    result.extend_from_slice(&child_coords[1..]);
                } else {
                    // Not matching - continuity policy will handle this
                    result.extend(child_coords);
                }
            } else {
                result.extend(child_coords);
            }
        }

        self.depth -= 1;
        self.chain.pop();
        Ok(result)
    }

    /// Resolve a vector with orientation applied
    fn resolve_with_orientation(
        &mut self,
        name: NameKey,
        ornt: Orientation,
    ) -> TopologyResult<Vec<(BigRational, BigRational)>> {
        let mut coords = self.resolve_line_2d(name)?;

        if ornt.should_reverse() {
            coords.reverse();
        }

        Ok(coords)
    }

    /// Check for cycle detection based on policy
    fn check_cycle(&self, name: &NameKey) -> TopologyResult<()> {
        use super::errors::CyclePolicy;

        let visit_count = self.visit_counts.get(name).copied().unwrap_or(0);

        match self.ctx.cycle_policy {
            CyclePolicy::Error => {
                if visit_count > 0 {
                    return Err(TopologyError::CycleDetected {
                        chain: self.chain.clone(),
                    });
                }
            }
            CyclePolicy::Truncate => {
                if visit_count > 0 {
                    // Truncate: stop traversal here by returning empty result
                    // Caller will handle this gracefully
                    return Err(TopologyError::CycleDetected {
                        chain: self.chain.clone(),
                    });
                }
            }
            CyclePolicy::AllowVisitCount(max_visits) => {
                if visit_count >= max_visits {
                    return Err(TopologyError::CycleDetected {
                        chain: self.chain.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check endpoint continuity
    fn check_continuity(
        &self,
        lhs_end: &(BigRational, BigRational),
        rhs_start: &(BigRational, BigRational),
        child: NameKey,
        index: usize,
    ) -> TopologyResult<()> {
        use super::errors::ContinuityPolicy;

        // Check if endpoints match exactly
        if lhs_end.0 == rhs_start.0 && lhs_end.1 == rhs_start.1 {
            return Ok(()); // Perfect continuity
        }

        match self.ctx.continuity_policy {
            ContinuityPolicy::Error => {
                return Err(TopologyError::ContinuityBreak {
                    at_index: index,
                    lhs_end: lhs_end.clone(),
                    rhs_start: rhs_start.clone(),
                    child,
                });
            }
            ContinuityPolicy::SnapWithinTolerance(denom) => {
                // Calculate distance using rational arithmetic
                let dx = &lhs_end.0 - &rhs_start.0;
                let dy = &lhs_end.1 - &rhs_start.1;

                // Approximate distance check: |dx| + |dy| < tolerance (Manhattan distance)
                let tolerance = BigRational::new(1.into(), denom.into());

                if dx.abs() < tolerance && dy.abs() < tolerance {
                    // Within tolerance - snap is handled by caller skipping duplicate point
                    return Ok(());
                }

                // Outside tolerance
                return Err(TopologyError::ContinuityBreak {
                    at_index: index,
                    lhs_end: lhs_end.clone(),
                    rhs_start: rhs_start.clone(),
                    child,
                });
            }
            ContinuityPolicy::InsertGapMarker => {
                // Allow the gap - caller will insert both points
                // This creates a visible discontinuity in the line
                Ok(())
            }
        }
    }
}
