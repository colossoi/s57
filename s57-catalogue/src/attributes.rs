//! S-57 Attributes (ATTL codes)
//!
//! Defines attribute types and values from IHO S-57 Object Catalogue.
//! This is a subset of the most common attributes - the full catalogue
//! contains 150+ attributes.

// Define enum attribute types first (before Attribute enum that uses them)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryOfShorelineConstruction {
    Breakwater = 1,
    Groyne = 2,
    Mole = 3,
    Pier = 4,
    Promenade = 5,
    Revetment = 6,
    Seawall = 7,
    Wharf = 8,
}

impl CategoryOfShorelineConstruction {
    fn from_value(s: &str) -> Option<Self> {
        use CategoryOfShorelineConstruction::*;
        let val = s.parse::<u8>().ok()?;
        Some(match val {
            1 => Breakwater,
            2 => Groyne,
            3 => Mole,
            4 => Pier,
            5 => Promenade,
            6 => Revetment,
            7 => Seawall,
            8 => Wharf,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryOfLight {
    Directional = 1,
    Horizontal = 2,
    Vertical = 3,
    Upper = 4,
    Lower = 5,
    Leading = 6,
    Aero = 7,
    AirObstruction = 8,
    Fog = 9,
    Flood = 10,
}

impl CategoryOfLight {
    fn from_value(s: &str) -> Option<Self> {
        use CategoryOfLight::*;
        let val = s.parse::<u8>().ok()?;
        Some(match val {
            1 => Directional,
            2 => Horizontal,
            3 => Vertical,
            4 => Upper,
            5 => Lower,
            6 => Leading,
            7 => Aero,
            8 => AirObstruction,
            9 => Fog,
            10 => Flood,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White = 1,
    Black = 2,
    Red = 3,
    Green = 4,
    Blue = 5,
    Yellow = 6,
    Grey = 7,
    Brown = 8,
    Amber = 9,
    Violet = 10,
    Orange = 11,
}

impl Colour {
    fn from_value(s: &str) -> Option<Self> {
        use Colour::*;
        let val = s.parse::<u8>().ok()?;
        Some(match val {
            1 => White,
            2 => Black,
            3 => Red,
            4 => Green,
            5 => Blue,
            6 => Yellow,
            7 => Grey,
            8 => Brown,
            9 => Amber,
            10 => Violet,
            11 => Orange,
            _ => return None,
        })
    }

    fn from_value_list(s: &str) -> Vec<Self> {
        s.split(',')
            .filter_map(|part| Self::from_value(part.trim()))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatureOfSurface {
    Mud = 1,
    Clay = 2,
    Silt = 3,
    Sand = 4,
    Stone = 5,
    Gravel = 6,
    Pebbles = 7,
    Cobbles = 8,
    Rock = 9,
    Coral = 11,
    Shell = 14,
    Boulder = 17,
}

impl NatureOfSurface {
    fn from_value(s: &str) -> Option<Self> {
        use NatureOfSurface::*;
        let val = s.parse::<u8>().ok()?;
        Some(match val {
            1 => Mud,
            2 => Clay,
            3 => Silt,
            4 => Sand,
            5 => Stone,
            6 => Gravel,
            7 => Pebbles,
            8 => Cobbles,
            9 => Rock,
            11 => Coral,
            14 => Shell,
            17 => Boulder,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Permanent = 1,
    Occasional = 2,
    Recommended = 3,
    NotInUse = 4,
    Periodic = 5,
    Reserved = 6,
    Temporary = 7,
    Private = 8,
    Mandatory = 9,
    Destroyed = 10,
    Extinguished = 11,
    Illuminated = 12,
    Historic = 13,
    Public = 14,
    Synchronized = 15,
    Watched = 16,
    UnWatched = 17,
    ExistenceDoubtful = 18,
}

impl Status {
    fn from_value(s: &str) -> Option<Self> {
        use Status::*;
        let val = s.parse::<u8>().ok()?;
        Some(match val {
            1 => Permanent,
            2 => Occasional,
            3 => Recommended,
            4 => NotInUse,
            5 => Periodic,
            6 => Reserved,
            7 => Temporary,
            8 => Private,
            9 => Mandatory,
            10 => Destroyed,
            11 => Extinguished,
            12 => Illuminated,
            13 => Historic,
            14 => Public,
            15 => Synchronized,
            16 => Watched,
            17 => UnWatched,
            18 => ExistenceDoubtful,
            _ => return None,
        })
    }

    fn from_value_list(s: &str) -> Vec<Self> {
        s.split(',')
            .filter_map(|part| Self::from_value(part.trim()))
            .collect()
    }
}

/// S-57 Attribute with typed value
///
/// Represents an attribute (ATTL code) with its decoded value.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Attribute {
    /// CATSLN (19): Category of shoreline construction
    CategoryOfShorelineConstruction(CategoryOfShorelineConstruction),
    /// CATLIT (33): Category of light
    CategoryOfLight(CategoryOfLight),
    /// COLOUR (35): Colour
    Colour(Vec<Colour>),
    /// DRVAL1 (87): Depth range value 1
    DepthRangeValue1(f64),
    /// DRVAL2 (88): Depth range value 2
    DepthRangeValue2(f64),
    /// NATSUR (57): Nature of surface
    NatureOfSurface(NatureOfSurface),
    /// OBJNAM (116): Object name
    ObjectName(String),
    /// STATUS (118): Status
    Status(Vec<Status>),
    /// VALNMR (171): Value of nominal range
    ValueOfNominalRange(f64),

    /// Unknown attribute (code, raw value)
    Unknown(u16, String),
}

impl Attribute {
    /// Get human-readable attribute name from ATTL code
    ///
    /// Returns a descriptive name for common S-57 attributes, or None for unknown codes.
    pub fn attribute_name(attl: u16) -> Option<&'static str> {
        match attl {
            19 => Some("CATSLN (Category of shoreline construction)"),
            33 => Some("CATLIT (Category of light)"),
            35 => Some("COLOUR (Colour)"),
            57 => Some("NATSUR (Nature of surface)"),
            87 => Some("DRVAL1 (Depth range value 1)"),
            88 => Some("DRVAL2 (Depth range value 2)"),
            116 => Some("OBJNAM (Object name)"),
            118 => Some("STATUS (Status)"),
            171 => Some("VALNMR (Value of nominal range)"),
            _ => None,
        }
    }

    /// Decode attribute from code and string value
    pub fn from_code_and_value(attl: u16, atvl: &str) -> Option<Self> {
        let attr = match attl {
            19 => Attribute::CategoryOfShorelineConstruction(
                CategoryOfShorelineConstruction::from_value(atvl)?,
            ),
            33 => Attribute::CategoryOfLight(CategoryOfLight::from_value(atvl)?),
            35 => Attribute::Colour(Colour::from_value_list(atvl)),
            87 => Attribute::DepthRangeValue1(atvl.parse().ok()?),
            88 => Attribute::DepthRangeValue2(atvl.parse().ok()?),
            57 => Attribute::NatureOfSurface(NatureOfSurface::from_value(atvl)?),
            116 => Attribute::ObjectName(atvl.to_string()),
            118 => Attribute::Status(Status::from_value_list(atvl)),
            171 => Attribute::ValueOfNominalRange(atvl.parse().ok()?),
            _ => Attribute::Unknown(attl, atvl.to_string()),
        };
        Some(attr)
    }

    /// Get the ATTL code
    pub fn code(&self) -> u16 {
        use Attribute::*;
        match self {
            CategoryOfShorelineConstruction(_) => 19,
            CategoryOfLight(_) => 33,
            Colour(_) => 35,
            DepthRangeValue1(_) => 87,
            DepthRangeValue2(_) => 88,
            NatureOfSurface(_) => 57,
            ObjectName(_) => 116,
            Status(_) => 118,
            ValueOfNominalRange(_) => 171,
            Unknown(code, _) => *code,
        }
    }
}

/// Attribute value types (for simpler attributes)
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Integer(i32),
    Float(f64),
    String(String),
    Enum(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_decode() {
        let attr = Attribute::from_code_and_value(57, "4").unwrap();
        assert!(matches!(
            attr,
            Attribute::NatureOfSurface(NatureOfSurface::Sand)
        ));

        let attr = Attribute::from_code_and_value(116, "Test Light").unwrap();
        assert!(matches!(attr, Attribute::ObjectName(_)));
    }

    #[test]
    fn test_colour_list() {
        let colours = Colour::from_value_list("3,4");
        assert_eq!(colours, vec![Colour::Red, Colour::Green]);
    }
}
