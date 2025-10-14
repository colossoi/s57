//! Topology-specific types for VRPT/FSPT traversal

use s57_parse::bitstring::NameKey;

/// Edge orientation for VRPT/FSPT traversal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Forward: use edge coordinates as-is (ORNT = 1)
    Forward,
    /// Reverse: reverse edge coordinates (ORNT = 2)
    Reverse,
    /// Not applicable (ORNT = 255)
    NA,
}

impl Orientation {
    /// Create from ORNT byte value
    pub fn from_ornt(ornt: u8) -> Self {
        match ornt {
            1 => Self::Forward,
            2 => Self::Reverse,
            255 => Self::NA,
            _ => Self::NA, // Default to NA for unknown values
        }
    }

    /// Should coordinates be reversed?
    pub fn should_reverse(&self) -> bool {
        matches!(self, Orientation::Reverse)
    }
}

/// VRPT pointer with metadata
///
/// Represents a reference from one vector to another in the topology graph
#[derive(Debug, Clone)]
pub struct VrptPointer {
    /// Target vector NAME (RCNM:RCID)
    pub name: NameKey,
    /// Orientation (forward/reverse)
    pub ornt: Orientation,
    /// Usage indicator
    pub usag: u8,
    /// Topology indicator (1=begin, 2=end, 3=left, 4=right, etc.)
    pub topi: u8,
    /// Masking value
    pub mask: u8,
}

/// FSPT pointer with metadata
///
/// Represents a reference from a feature to a spatial vector
#[derive(Debug, Clone)]
pub struct FsptPointer {
    /// Target vector NAME (RCNM:RCID)
    pub name: NameKey,
    /// Orientation (forward/reverse)
    pub ornt: Orientation,
    /// Usage indicator (1=exterior, 2=interior, etc.)
    pub usag: u8,
    /// Masking value
    pub mask: u8,
}
