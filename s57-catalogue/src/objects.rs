//! S-57 Object Classes (OBJL codes)
//!
//! Defines geo-object types from IHO S-57 Object Catalogue.
//! This is a subset of the most common classes - the full catalogue
//! contains 400+ classes.

/// S-57 Object Class
///
/// Represents the type of geographic object (OBJL field from FRID).
/// Each object class has a numeric code and semantic meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ObjectClass {
    // Meta objects (1-19)
    /// M_COVR (1): Coverage area
    Coverage = 1,
    /// BCNCAR (7): Beacon cardinal
    BeaconCardinal = 7,
    /// BCNLAT (8): Beacon lateral
    BeaconLateral = 8,
    /// M_NPUB (12): Navigational publication
    NavPublication = 12,
    /// M_QUAL (13): Quality of data
    DataQuality = 13,
    /// BOYLAT (14): Buoy lateral
    BuoyLateral = 14,
    /// BOYSAW (15): Buoy safe water
    BuoySafeWater = 15,
    /// BOYISD (16): Buoy isolated danger
    BuoyIsolatedDanger = 16,

    // Cartographic objects (20-39)
    /// $COMPS (21): Compilation scale
    CompilationScale = 21,
    /// $TEXTS (25): Text
    Text = 25,
    /// BRIDGE (27): Bridge
    Bridge = 27,

    // Geo objects - Natural features (40-69)
    /// DEPARE (42): Depth area
    DepthArea = 42,
    /// DEPCNT (43): Depth contour
    DepthContour = 43,
    /// DRGARE (46): Dredged area
    DredgedArea = 46,
    /// SBDARE (44): Seabed area
    SeabedArea = 44,
    /// LNDARE (71): Land area
    LandArea = 71,
    /// LIGHTS (75): Light
    Light = 75,

    // Geo objects - Artificial features (70-139)
    /// BUAARE (84): Built-up area
    BuiltUpArea = 84,
    /// BUISGL (85): Building single
    BuildingSingle = 85,
    /// RIVERS (114): River
    River = 114,

    /// Unknown object class
    Unknown,
}

impl ObjectClass {
    /// Decode object class from OBJL code
    pub fn from_code(objl: u16) -> Option<Self> {
        use ObjectClass::*;
        let class = match objl {
            1 => Coverage,
            7 => BeaconCardinal,
            8 => BeaconLateral,
            12 => NavPublication,
            13 => DataQuality,
            14 => BuoyLateral,
            15 => BuoySafeWater,
            16 => BuoyIsolatedDanger,
            21 => CompilationScale,
            25 => Text,
            27 => Bridge,
            42 => DepthArea,
            43 => DepthContour,
            44 => SeabedArea,
            46 => DredgedArea,
            71 => LandArea,
            75 => Light,
            84 => BuiltUpArea,
            85 => BuildingSingle,
            114 => River,
            _ => return None,
        };
        Some(class)
    }

    /// Get the OBJL code for this object class
    pub fn code(&self) -> u16 {
        *self as u16
    }

    /// Get the human-readable name
    pub fn name(&self) -> &'static str {
        use ObjectClass::*;
        match self {
            Coverage => "Coverage",
            NavPublication => "Navigational Publication",
            DataQuality => "Data Quality",
            CompilationScale => "Compilation Scale",
            Text => "Text",
            DepthArea => "Depth Area",
            DepthContour => "Depth Contour",
            DredgedArea => "Dredged Area",
            LandArea => "Land Area",
            River => "River",
            SeabedArea => "Seabed Area",
            Bridge => "Bridge",
            BuiltUpArea => "Built-up Area",
            BuildingSingle => "Building (Single)",
            Light => "Light",
            BuoyLateral => "Buoy (Lateral)",
            BuoySafeWater => "Buoy (Safe Water)",
            BuoyIsolatedDanger => "Buoy (Isolated Danger)",
            BeaconCardinal => "Beacon (Cardinal)",
            BeaconLateral => "Beacon (Lateral)",
            Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_decode() {
        assert_eq!(ObjectClass::from_code(42), Some(ObjectClass::DepthArea));
        assert_eq!(ObjectClass::from_code(75), Some(ObjectClass::Light));
        assert_eq!(ObjectClass::from_code(999), None);
    }

    #[test]
    fn test_object_name() {
        assert_eq!(ObjectClass::DepthContour.name(), "Depth Contour");
        assert_eq!(ObjectClass::Light.name(), "Light");
    }
}
