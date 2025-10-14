#!/usr/bin/env nu

# Generate Rust enums from S-57 Object and Attribute Catalogue CSVs
#
# Reads GDAL s57objectclasses.csv and s57attributes.csv to generate
# complete, type-safe Rust enums with strum support.

def sanitize_variant_name [name: string] {
    # Convert object/attribute name to valid Rust enum variant name
    # "Administration area (Named)" -> "AdministrationAreaNamed"
    $name
    | str replace --all --regex '[^\w\s]' ''  # Remove punctuation
    | split row ' '
    | each { str capitalize }
    | str join ''
}

def generate_object_classes [csv_path: path, output_path: path] {
    # Filter out code 0 entries (comment rows) and sort
    let objects = open $csv_path | where Code != 0 | sort-by Code

    let header = [
        "// Generated from s57objectclasses.csv"
        "// DO NOT EDIT - run scripts/generate_catalogue.nu to regenerate"
        ""
        "#![allow(non_camel_case_types)]"
        "#![allow(unreachable_patterns)]"
        ""
        "use strum_macros::{Display, EnumString};"
        ""
        "/// S-57 Object Class"
        "///"
        "/// Complete catalogue of IHO S-57 object classes from GDAL reference."
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]"
        "#[non_exhaustive]"
        "pub enum ObjectClass {"
    ]

    # Generate enum variants
    # Objects with code >= 10000 are national/custom extensions, append code to avoid duplicates
    let variants = $objects | each {|obj|
        let base_variant = sanitize_variant_name $obj.ObjectClass
        let variant = if $obj.Code >= 10000 {
            $base_variant + ($obj.Code | into string)
        } else {
            $base_variant
        }
        let strum_line = '    #[strum(serialize = "' + $obj.Acronym + '")]'
        [
            $"    /// ($obj.Acronym) \(($obj.Code)): ($obj.ObjectClass)"
            $strum_line
            $"    ($variant) = ($obj.Code),"
            ""
        ]
    } | flatten

    let enum_footer = [
        "    /// Unknown object class"
        "    Unknown,"
        "}"
        ""
    ]

    # Generate from_code() method
    let from_code_header = [
        "impl ObjectClass {"
        "    /// Decode object class from OBJL code"
        "    pub fn from_code(objl: u16) -> Option<Self> {"
        "        use ObjectClass::*;"
        "        let class = match objl {"
    ]

    let from_code_arms = $objects | each {|obj|
        let base_variant = sanitize_variant_name $obj.ObjectClass
        let variant = if $obj.Code >= 10000 {
            $base_variant + ($obj.Code | into string)
        } else {
            $base_variant
        }
        $"            ($obj.Code) => ($variant),"
    }

    let from_code_footer = [
        "            _ => return None,"
        "        };"
        "        Some(class)"
        "    }"
        ""
    ]

    # Generate code() method
    let code_header = [
        "    /// Get OBJL code for this object class"
        "    pub fn code(&self) -> u16 {"
        "        match self {"
    ]

    let code_arms = $objects | each {|obj|
        let base_variant = sanitize_variant_name $obj.ObjectClass
        let variant = if $obj.Code >= 10000 {
            $base_variant + ($obj.Code | into string)
        } else {
            $base_variant
        }
        $"            Self::($variant) => ($obj.Code),"
    }

    let code_footer = [
        "            Self::Unknown => 0,"
        "        }"
        "    }"
        ""
    ]

    # Generate name() method
    let name_header = [
        "    /// Get human-readable name"
        "    pub fn name(&self) -> &'static str {"
        "        use ObjectClass::*;"
        "        match self {"
    ]

    let name_arms = $objects | each {|obj|
        let base_variant = sanitize_variant_name $obj.ObjectClass
        let variant = if $obj.Code >= 10000 {
            $base_variant + ($obj.Code | into string)
        } else {
            $base_variant
        }
        let escaped_name = $obj.ObjectClass | str replace --all '"' '\"'
        $"            ($variant) => \"($escaped_name)\","
    }

    let name_footer = [
        "            Unknown => \"Unknown\","
        "        }"
        "    }"
        "}"
    ]

    # Combine all parts
    let output = [
        $header
        $variants
        $enum_footer
        $from_code_header
        $from_code_arms
        $from_code_footer
        $code_header
        $code_arms
        $code_footer
        $name_header
        $name_arms
        $name_footer
    ] | flatten | str join "\n"

    $output | save --force $output_path

    print $"Generated ($objects | length) object classes -> ($output_path)"
}

def generate_attributes [csv_path: path, output_path: path] {
    # Filter out code 0 entries (comment rows) and sort
    let attributes = open $csv_path | where Code != 0 | sort-by Code

    let header = [
        "// Generated from s57attributes.csv"
        "// DO NOT EDIT - run scripts/generate_catalogue.nu to regenerate"
        ""
        "#![allow(unreachable_patterns)]"
        ""
        "/// S-57 Attribute with code and name"
        "///"
        "/// Complete catalogue of IHO S-57 attributes from GDAL reference."
        "#[derive(Debug, Clone, PartialEq)]"
        "pub struct AttributeInfo {"
        "    pub code: u16,"
        "    pub acronym: &'static str,"
        "    pub name: &'static str,"
        "}"
        ""
        "impl AttributeInfo {"
        "    /// Get attribute info by code"
        "    pub fn from_code(code: u16) -> Option<Self> {"
        "        match code {"
    ]

    let match_arms = $attributes | each {|attr|
        let escaped_name = $attr.Attribute | str replace --all '"' '\"'
        [
            $"            ($attr.Code) => Some\(Self \{"
            $"                code: ($attr.Code),"
            $"                acronym: \"($attr.Acronym)\","
            $"                name: \"($escaped_name)\","
            $"            \}),"
        ]
    } | flatten

    let footer = [
        "            _ => None,"
        "        }"
        "    }"
        ""
        "    /// Get human-readable attribute name from ATTL code"
        "    pub fn attribute_name(attl: u16) -> Option<&'static str> {"
        "        Self::from_code(attl).map(|info| info.name)"
        "    }"
        "}"
    ]

    let output = [
        $header
        $match_arms
        $footer
    ] | flatten | str join "\n"

    $output | save --force $output_path

    print $"Generated ($attributes | length) attributes -> ($output_path)"
}

def main [] {
    # Paths
    let repo_root = $env.FILE_PWD | path dirname
    let specs_dir = $repo_root | path join "docs" "specs"
    let catalogue_src = $repo_root | path join "s57-catalogue" "src"

    # Input CSVs
    let objects_csv = $specs_dir | path join "s57objectclasses.csv"
    let attributes_csv = $specs_dir | path join "s57attributes.csv"

    # Output Rust files
    let objects_rs = $catalogue_src | path join "objects_generated.rs"
    let attributes_rs = $catalogue_src | path join "attributes_generated.rs"

    # Check inputs exist
    if not ($objects_csv | path exists) {
        print $"Error: ($objects_csv) not found"
        exit 1
    }
    if not ($attributes_csv | path exists) {
        print $"Error: ($attributes_csv) not found"
        exit 1
    }

    # Generate
    print "Generating S-57 catalogue from GDAL CSVs..."
    generate_object_classes $objects_csv $objects_rs
    generate_attributes $attributes_csv $attributes_rs

    print "\nDone! Run 'cargo fmt' to format generated code."
}
