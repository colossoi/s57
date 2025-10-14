//! Error types and policies for topology traversal

use num_rational::BigRational;
use s57_parse::bitstring::NameKey;
use std::fmt;

/// Result type for topology operations
pub type TopologyResult<T> = Result<T, TopologyError>;

/// Errors that can occur during topology traversal
#[derive(Debug, Clone)]
pub enum TopologyError {
    /// Reference to a vector that doesn't exist
    DanglingReference {
        /// Source vector making the reference
        from: NameKey,
        /// Target vector that doesn't exist
        to: NameKey,
    },

    /// Cycle detected in topology graph
    CycleDetected {
        /// Chain of vectors forming the cycle
        chain: Vec<NameKey>,
    },

    /// Endpoints don't match when stitching edges
    ContinuityBreak {
        /// Index in the chain where break occurs
        at_index: usize,
        /// Last point of previous edge
        lhs_end: (BigRational, BigRational),
        /// First point of next edge
        rhs_start: (BigRational, BigRational),
        /// Child vector causing the break
        child: NameKey,
    },

    /// Mixed 2D and 3D geometry in same chain
    MixedDimensionality {
        /// Expected dimensionality
        expected: &'static str,
        /// Found dimensionality
        found: &'static str,
        /// Vector with mismatched dimensionality
        at_vector: NameKey,
    },

    /// Dataset parameters (COMF, SOMF) not available
    MissingDatasetParams,

    /// Vector has no geometry (neither direct SG2D/SG3D nor VRPT)
    NoGeometry {
        /// Vector with missing geometry
        vector: NameKey,
    },

    /// Maximum recursion depth exceeded
    MaxDepthExceeded {
        /// Maximum allowed depth
        max_depth: usize,
        /// Current chain
        chain: Vec<NameKey>,
    },
}

impl fmt::Display for TopologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopologyError::DanglingReference { from, to } => {
                write!(f, "Dangling reference from {:?} to {:?}", from, to)
            }
            TopologyError::CycleDetected { chain } => {
                write!(f, "Cycle detected in chain: ")?;
                for (i, name) in chain.iter().enumerate() {
                    if i > 0 {
                        write!(f, " → ")?;
                    }
                    write!(f, "{:?}", name)?;
                }
                Ok(())
            }
            TopologyError::ContinuityBreak {
                at_index,
                lhs_end,
                rhs_start,
                child,
            } => {
                write!(
                    f,
                    "Continuity break at index {} (child {:?}): end {:?} ≠ start {:?}",
                    at_index, child, lhs_end, rhs_start
                )
            }
            TopologyError::MixedDimensionality {
                expected,
                found,
                at_vector,
            } => {
                write!(
                    f,
                    "Mixed dimensionality at {:?}: expected {}, found {}",
                    at_vector, expected, found
                )
            }
            TopologyError::MissingDatasetParams => {
                write!(f, "Dataset parameters (DSPM) not available")
            }
            TopologyError::NoGeometry { vector } => {
                write!(f, "Vector {:?} has no geometry", vector)
            }
            TopologyError::MaxDepthExceeded { max_depth, chain } => {
                write!(
                    f,
                    "Maximum recursion depth {} exceeded, chain length: {}",
                    max_depth,
                    chain.len()
                )
            }
        }
    }
}

impl std::error::Error for TopologyError {}

/// Policy for handling cycle detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CyclePolicy {
    /// Raise error on cycle detection
    Error,
    /// Truncate at cycle point (return partial geometry)
    Truncate,
    /// Allow visiting each edge up to N times
    AllowVisitCount(usize),
}

/// Policy for handling continuity breaks
#[derive(Debug, Clone, Copy)]
pub enum ContinuityPolicy {
    /// Raise error on continuity break
    Error,
    /// Snap endpoints within tolerance (rational epsilon)
    SnapWithinTolerance(i64), // Denominator for tolerance (e.g., 1000000 = 1e-6)
    /// Insert gap marker (keep both segments, mark discontinuity)
    InsertGapMarker,
}
