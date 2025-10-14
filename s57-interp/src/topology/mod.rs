//! Topology Traversal System (TTS)
//!
//! Resolves renderable coordinate streams for S-57 vectors and features where
//! geometry may be direct (SG2D/SG3D) or derived via topology (VRPT → nodes/edges).
//!
//! ## Architecture
//!
//! - **Cursors**: Lazy iterators over coordinate streams
//! - **Walkers**: Recursive edge resolution with cycle detection
//! - **Policies**: Configurable error handling and continuity rules

pub mod cursors;
pub mod errors;
pub mod types;
pub mod walker;

pub use cursors::FeatureBoundaryCursor;
pub use errors::{ContinuityPolicy, CyclePolicy, TopologyError, TopologyResult};
pub use types::{FsptPointer, Orientation, VrptPointer};
pub use walker::EdgeWalker;

use crate::ecs::World;

/// Main context for topology traversal operations
///
/// Provides access to the ECS World and configuration policies
pub struct TraversalContext<'a> {
    /// ECS World with all vector/feature data
    pub world: &'a World,
    /// Cycle detection policy
    pub cycle_policy: CyclePolicy,
    /// Continuity break policy
    pub continuity_policy: ContinuityPolicy,
}

impl<'a> TraversalContext<'a> {
    /// Create a new traversal context with default policies
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            cycle_policy: CyclePolicy::Error,
            continuity_policy: ContinuityPolicy::Error,
        }
    }

    /// Set cycle detection policy
    pub fn with_cycle_policy(mut self, policy: CyclePolicy) -> Self {
        self.cycle_policy = policy;
        self
    }

    /// Set continuity break policy
    pub fn with_continuity_policy(mut self, policy: ContinuityPolicy) -> Self {
        self.continuity_policy = policy;
        self
    }
}
