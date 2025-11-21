mod features;
mod index;
mod render;
mod svg;

use clap::{Parser, Subcommand, ValueEnum};
use s57_parse::S57File;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "s57")]
#[command(about = "Parse and inspect S-57 Electronic Navigational Chart files", long_about = None)]
struct Cli {
    /// S-57 file to parse
    #[arg(value_name = "FILE")]
    file: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display summary information about the file
    Info,

    /// Print file contents
    Print {
        /// Output format
        #[arg(short, long, value_enum, default_value = "yaml")]
        format: OutputFormat,

        /// Print only a specific record number
        #[arg(short, long)]
        record: Option<usize>,

        /// Limit number of records to print (only for yaml format)
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// List all feature objects in the file
    ListFeatures,

    /// Show detailed data for a specific feature object
    ShowObject {
        /// Feature record ID (RCID) to display
        #[arg(value_name = "RCID")]
        rcid: u32,
    },

    /// Calculate geographic extent (bounding box) of the chart
    Extent {
        /// SQLite database path to store feature index
        #[arg(short, long, value_name = "DB")]
        database: Option<PathBuf>,
    },

    /// Render features to SVG
    Render {
        /// Output SVG file path
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,

        /// Maximum number of features to render
        #[arg(short, long)]
        limit: Option<usize>,

        /// Render only a specific feature by FIDN (Feature ID Number)
        #[arg(long, value_name = "FIDN")]
        feature: Option<u32>,

        /// Filter by comma-separated list of S-57 object class codes (e.g., "COALNE,BRIDGE,WRECKS")
        /// Use 6-character S-57 codes from the object catalogue.
        #[arg(
            long,
            value_name = "CLASSES",
            value_delimiter = ',',
            default_value = "COALNE,LNDARE,DEPARE,DEPCNT,SEAARE,BRIDGE,BUISGL,LNDMRK,LIGHTS,BCNCAR,BCNLAT,BCNISD,BCNSAW,BOYCAR,BOYLAT,BOYISD,BOYSAW,ACHARE,WRECKS,OBSTRN,RIVERS,LAKARE,CANALS,DAMCON,BERTHS,HRBARE,RESARE,FAIRWY,PILBOP,OSPARE"
        )]
        classes: Vec<String>,

        /// Canvas width in pixels
        #[arg(long, default_value = "1200")]
        width: u32,

        /// Canvas height in pixels
        #[arg(long, default_value = "800")]
        height: u32,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// YAML format with semantic interpretation
    Yaml,
    /// Hex dump of ISO 8211 fields
    Hex,
}

fn main() {
    let cli = Cli::parse();

    // Initialize logger
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    // Read the file
    let data = match std::fs::read(&cli.file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Parse the S-57 file
    let file = match S57File::from_bytes(&data) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            std::process::exit(1);
        }
    };

    match &cli.command {
        Commands::Info => {
            print_info(&cli.file, data.len(), &file);
        }
        Commands::Print {
            format,
            record,
            limit,
        } => match format {
            OutputFormat::Yaml => {
                print_yaml(&file, *record, *limit);
            }
            OutputFormat::Hex => {
                print_hex(&file, *record);
            }
        },
        Commands::ListFeatures => {
            features::list_features(&file);
        }
        Commands::ShowObject { rcid } => {
            features::show_object(&file, *rcid);
        }
        Commands::Extent { database } => {
            features::print_extent(&file, &cli.file, database.as_deref());
        }
        Commands::Render {
            output,
            limit,
            feature,
            classes,
            width,
            height,
        } => {
            render::render_to_svg(&file, output, *limit, *feature, classes, *width, *height);
        }
    }
}

fn print_info(path: &PathBuf, file_size: usize, file: &S57File) {
    let records = file.records();

    println!("File: {}", path.display());
    println!("Size: {} bytes", file_size);
    println!("Records: {}", records.len());

    // Parse DDR if available
    if let Some(ddr_record) = records.first() {
        if ddr_record.leader.is_ddr() {
            match s57_parse::ddr::DDR::parse(ddr_record) {
                Ok(ddr) => {
                    println!("\nData Descriptive Record (DDR):");
                    println!("  Field definitions: {}", ddr.field_defs().len());

                    // Count different record types in data records
                    let mut record_types = std::collections::HashMap::new();
                    for record in &records[1..] {
                        for field in &record.fields {
                            if field.tag == "DSID" || field.tag == "FRID" || field.tag == "VRID" {
                                *record_types.entry(field.tag.clone()).or_insert(0) += 1;
                            }
                        }
                    }

                    if !record_types.is_empty() {
                        println!("\nRecord types:");
                        for (tag, count) in record_types.iter() {
                            let description = match tag.as_str() {
                                "DSID" => "Data Set Identification",
                                "FRID" => "Feature Records",
                                "VRID" => "Vector Records",
                                _ => "Unknown",
                            };
                            println!("  {}: {} ({})", tag, count, description);
                        }
                    }

                    // List field definitions
                    println!("\nField definitions:");
                    for (tag, def) in ddr.field_defs() {
                        if !tag.starts_with('0') {
                            let repeating = if def.is_repeating { " [repeating]" } else { "" };
                            println!(
                                "  {}: {} ({} subfields{})",
                                tag,
                                def.name,
                                def.subfield_count(),
                                repeating
                            );
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse DDR: {}", e);
                }
            }
        }
    }
}

fn print_yaml(file: &S57File, record_filter: Option<usize>, limit: Option<usize>) {
    let records = file.records();

    // Filter to specific record if requested
    let records_to_print: Vec<_> = if let Some(record_num) = record_filter {
        if record_num < records.len() {
            vec![&records[record_num]]
        } else {
            eprintln!(
                "Error: Record {} not found (file has {} records)",
                record_num,
                records.len()
            );
            std::process::exit(1);
        }
    } else {
        records.iter().collect()
    };

    // Parse DDR first
    if let Some(ddr_record) = records.first() {
        match s57_parse::ddr::DDR::parse(ddr_record) {
            Ok(ddr) => {
                print_yaml_structure_with_ddr(&records_to_print, record_filter, limit, &ddr);
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse DDR: {}", e);
                print_yaml_structure(&records_to_print, record_filter, limit);
            }
        }
    } else {
        print_yaml_structure(&records_to_print, record_filter, limit);
    }
}

fn print_hex(file: &S57File, record_filter: Option<usize>) {
    let records = file.records();

    // Filter to specific record if requested
    let records_to_print: Vec<_> = if let Some(record_num) = record_filter {
        if record_num < records.len() {
            vec![(record_num, &records[record_num])]
        } else {
            eprintln!(
                "Error: Record {} not found (file has {} records)",
                record_num,
                records.len()
            );
            std::process::exit(1);
        }
    } else {
        records.iter().enumerate().collect()
    };

    for (i, record) in records_to_print {
        let record_type = if record.leader.is_ddr() { "DDR" } else { "DR" };
        println!("Record {} ({}):", i, record_type);
        println!("  Leader:");
        println!("    Length: {} bytes", record.leader.record_length);
        println!(
            "    Interchange Level: '{}'",
            record.leader.interchange_level
        );
        println!("    Leader ID: '{}'", record.leader.leader_identifier);
        println!(
            "    Base Address: {}",
            record.leader.base_address_of_field_area
        );

        println!("  Fields:");
        for field in &record.fields {
            println!("    Tag: {}", field.tag);
            println!("    Size: {} bytes", field.data.len());
            println!("    Data:");

            // Print hex dump in rows of 16 bytes
            for (offset, chunk) in field.data.chunks(16).enumerate() {
                let hex: String = chunk
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                let ascii: String = chunk
                    .iter()
                    .map(|&b| {
                        if b >= 0x20 && b <= 0x7E {
                            b as char
                        } else if b == 0x1E {
                            '⊣' // field terminator
                        } else if b == 0x1F {
                            '⊢' // unit terminator
                        } else {
                            '·'
                        }
                    })
                    .collect();

                println!("      {:04x}: {:<48} {}", offset * 16, hex, ascii);
            }
            println!();
        }
        println!();
    }
}

fn print_yaml_structure_with_ddr(
    records: &[&s57_parse::iso8211::Record],
    record_filter: Option<usize>,
    limit: Option<usize>,
    ddr: &s57_parse::ddr::DDR,
) {
    use s57_parse::interpret::*;

    // Only show field definitions if not filtering to a specific record
    if record_filter.is_none() {
        println!("# Field definitions found in DDR:");
        for (tag, def) in ddr.field_defs() {
            if !tag.starts_with('0') {
                let repeating = if def.is_repeating { " [repeating]" } else { "" };
                println!(
                    "#   {}: {} ({} subfields{})",
                    tag,
                    def.name,
                    def.subfield_count(),
                    repeating
                );
            }
        }
        println!();
    }

    println!("records:");
    let records_to_show = if record_filter.is_some() {
        records.len()
    } else {
        limit.unwrap_or_else(|| records.len())
    };
    for (idx, record) in records.iter().enumerate().take(records_to_show) {
        // When filtering, use the actual record number; otherwise use the index
        let i = record_filter.unwrap_or(idx);
        let record_type = if record.leader.is_ddr() {
            "DDR (Data Descriptive Record)"
        } else {
            "DR (Data Record)"
        };

        println!("  - record_{}:  # {}", i, record_type);
        println!("      leader:");
        println!(
            "        record_length: {}  # bytes",
            record.leader.record_length
        );
        println!(
            "        interchange_level: '{}'",
            record.leader.interchange_level
        );
        println!(
            "        leader_identifier: '{}'  # {}",
            record.leader.leader_identifier,
            if record.leader.is_ddr() {
                "Data Descriptive"
            } else {
                "Data"
            }
        );
        println!(
            "        base_address_of_field_area: {}",
            record.leader.base_address_of_field_area
        );

        println!("      fields:");
        for field in &record.fields {
            let interpretation = interpret_field_tag(&field.tag);
            println!("        - tag: {}  # {}", field.tag, interpretation);
            println!("          size: {}  # bytes", field.data.len());

            // Special handling for 0000 field (DDR field control)
            if field.tag == "0000" {
                if let Some((_controls, title, tag_pairs)) = field.parse_field_control_field() {
                    println!("          data:");
                    if !title.is_empty() {
                        println!("            external_title: \"{}\"", title);
                    }
                    if !tag_pairs.is_empty() {
                        println!("            field_hierarchy:  # Parent-child tag pairs defining tree structure");
                        for (parent, child) in tag_pairs {
                            println!("              - {{ parent: {}, child: {} }}", parent, child);
                        }
                    }
                } else {
                    println!("          data: <binary>  # {} bytes", field.data.len());
                }
            // Special handling for 0001 field
            } else if field.tag == "0001" {
                // Try parsing as DDR record identifier definition first
                if let Some((controls, name, array_desc, format_controls)) =
                    field.parse_record_identifier_field()
                {
                    // DDR: data descriptive field defining record identifier structure
                    println!("          data:");
                    println!("            field_controls: \"{}\"", controls);
                    println!("            field_name: \"{}\"", name);
                    if !array_desc.is_empty() {
                        println!("            array_descriptor: \"{}\"", array_desc);
                    }
                    println!(
                        "            format_controls: \"{}\"  # Format for record ID in DRs",
                        format_controls
                    );
                } else if let Some((seq_num, _reserved)) = parse_field_control(&field.data) {
                    // Data record: contains sequence number
                    println!("          data:");
                    println!(
                        "            sequence_number: {}  # Record sequence in file",
                        seq_num
                    );
                } else {
                    println!("          data: <binary>  # {} bytes", field.data.len());
                }
            } else if record.leader.is_ddr() {
                // In DDR: this field is a definition, not data
                // Parse it as a data descriptive field (DDF)
                if let Some(field_def) = ddr.get_field_def(&field.tag) {
                    println!("          definition:  # Data Descriptive Field (DDF)");
                    println!("            field_name: \"{}\"", field_def.name);
                    if !field_def.array_descriptor.is_empty() {
                        println!(
                            "            array_descriptor: \"{}\"  # Subfield labels",
                            field_def.array_descriptor
                        );
                    }
                    if !field_def.format_controls.is_empty() {
                        println!(
                            "            format_controls: \"{}\"  # Subfield types",
                            field_def.format_controls
                        );
                    }
                    println!(
                        "            subfield_count: {}  # Number of subfield labels",
                        field_def.subfield_count()
                    );
                    if field_def.is_repeating {
                        println!(
                            "            repeating_group: true  # Group can repeat multiple times"
                        );
                    }
                } else {
                    println!("          data: <binary>  # {} bytes", field.data.len());
                }
            } else {
                // Data Record: parse using DDR definitions
                match ddr.parse_field_data(field) {
                    Ok(parsed) => {
                        println!("          data:");
                        for (group_idx, group) in parsed.groups().iter().enumerate() {
                            if parsed.groups().len() > 1 {
                                println!("            group_{}:", group_idx);
                            }
                            for (label, value) in group {
                                let indent = if parsed.groups().len() > 1 {
                                    "              "
                                } else {
                                    "            "
                                };
                                match value {
                                    s57_parse::ddr::SubfieldValue::Null => {
                                        println!("{}{}: null", indent, label);
                                    }
                                    s57_parse::ddr::SubfieldValue::Integer(i) => {
                                        let comment = match label.as_str() {
                                            "RCNM" => {
                                                format!("  # {}", interpret_record_name(*i as u8))
                                            }
                                            "PRIM" => {
                                                format!("  # {}", interpret_primitive(*i as u8))
                                            }
                                            "OBJL" => {
                                                format!("  # {}", interpret_object_label(*i as u16))
                                            }
                                            "RUIN" => format!(
                                                "  # {}",
                                                interpret_update_instruction(*i as u8)
                                            ),
                                            "ORNT" => {
                                                format!("  # {}", interpret_orientation(*i as u8))
                                            }
                                            _ => String::new(),
                                        };
                                        println!("{}{}: {}{}", indent, label, i, comment);
                                    }
                                    s57_parse::ddr::SubfieldValue::UnsignedInteger(u) => {
                                        // Large unsigned values like FIDN, RCID, etc.
                                        println!("{}{}: {}", indent, label, u);
                                    }
                                    s57_parse::ddr::SubfieldValue::Real(f) => {
                                        println!("{}{}: {:.6}", indent, label, f);
                                    }
                                    s57_parse::ddr::SubfieldValue::String(s) => {
                                        println!("{}{}: \"{}\"", indent, label, s);
                                    }
                                    s57_parse::ddr::SubfieldValue::Bytes(b) => {
                                        let hex: String = b
                                            .iter()
                                            .take(8)
                                            .map(|byte| format!("{:02x}", byte))
                                            .collect::<Vec<_>>()
                                            .join(" ");
                                        let more = if b.len() > 8 { "..." } else { "" };
                                        println!("{}{}: <hex: {}{}>", indent, label, hex, more);
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Fall back to manual interpretation
                        print_field_interpretation(&field.tag, &field.data);
                    }
                }
            }
        }
        println!();
    }

    // Only show "more records" message when not filtering and there are more records
    if record_filter.is_none() && records.len() > records_to_show {
        println!(
            "  # ... {} more records (use --limit to see more)",
            records.len() - records_to_show
        );
    }
}

fn print_yaml_structure(
    records: &[&s57_parse::iso8211::Record],
    record_filter: Option<usize>,
    limit: Option<usize>,
) {
    use s57_parse::interpret::*;

    println!("records:");
    let records_to_show = if record_filter.is_some() {
        records.len()
    } else {
        limit.unwrap_or_else(|| records.len())
    };
    for (idx, record) in records.iter().enumerate().take(records_to_show) {
        let i = record_filter.unwrap_or(idx);
        let record_type = if record.leader.is_ddr() {
            "DDR (Data Descriptive Record)"
        } else {
            "DR (Data Record)"
        };

        println!("  - record_{}:  # {}", i, record_type);
        println!("      leader:");
        println!(
            "        record_length: {}  # bytes",
            record.leader.record_length
        );
        println!(
            "        interchange_level: '{}'",
            record.leader.interchange_level
        );
        println!(
            "        leader_identifier: '{}'  # {}",
            record.leader.leader_identifier,
            if record.leader.is_ddr() {
                "Data Descriptive"
            } else {
                "Data"
            }
        );
        println!(
            "        base_address_of_field_area: {}",
            record.leader.base_address_of_field_area
        );

        println!("      fields:");
        for field in &record.fields {
            let interpretation = interpret_field_tag(&field.tag);
            println!("        - tag: {}  # {}", field.tag, interpretation);
            println!("          size: {}  # bytes", field.data.len());

            // Try to interpret field data based on tag
            print_field_interpretation(&field.tag, &field.data);
        }
        println!();
    }

    // Only show "more records" message when not filtering and there are more records
    if record_filter.is_none() && records.len() > records_to_show {
        println!(
            "  # ... {} more records (use --limit to see more)",
            records.len() - records_to_show
        );
    }
}

fn print_field_interpretation(tag: &str, data: &[u8]) {
    use s57_parse::interpret::*;

    match tag {
        "0001" => {
            if let Some((seq_num, _reserved)) = parse_field_control(data) {
                // Data record: contains sequence number
                println!("          data:");
                println!(
                    "            sequence_number: {}  # Record sequence in file",
                    seq_num
                );
            } else {
                // DDR: contains field control metadata (text format)
                if let Ok(text) = std::str::from_utf8(&data[..data.len().saturating_sub(1)]) {
                    println!("          data: \"{}\"  # ISO 8211 metadata", text.trim());
                } else {
                    println!("          data: <binary>  # {} bytes", data.len());
                }
            }
        }
        "FRID" if data.len() >= 12 => {
            let rcnm = data[0];
            let rcid = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
            let prim = data[5];
            let grup = data[6];
            let objl = u16::from_le_bytes([data[7], data[8]]);
            let rver = u16::from_le_bytes([data[9], data[10]]);
            let ruin = data[11];

            println!("          data:");
            println!(
                "            RCNM: {}  # {}",
                rcnm,
                interpret_record_name(rcnm)
            );
            println!("            RCID: {}  # Record ID", rcid);
            println!(
                "            PRIM: {}  # {}",
                prim,
                interpret_primitive(prim)
            );
            println!("            GRUP: {}  # Group", grup);
            println!(
                "            OBJL: {}  # {}",
                objl,
                interpret_object_label(objl)
            );
            println!("            RVER: {}  # Record version", rver);
            println!(
                "            RUIN: {}  # {}",
                ruin,
                interpret_update_instruction(ruin)
            );
        }
        "VRID" if data.len() >= 8 => {
            let rcnm = data[0];
            let rcid = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
            let rver = u16::from_le_bytes([data[5], data[6]]);
            let ruin = data[7];

            println!("          data:");
            println!(
                "            RCNM: {}  # {}",
                rcnm,
                interpret_record_name(rcnm)
            );
            println!("            RCID: {}  # Record ID", rcid);
            println!("            RVER: {}  # Record version", rver);
            println!(
                "            RUIN: {}  # {}",
                ruin,
                interpret_update_instruction(ruin)
            );
        }
        "FOID" if data.len() >= 8 => {
            let agen = u16::from_le_bytes([data[0], data[1]]);
            let fidn = u32::from_le_bytes([data[2], data[3], data[4], data[5]]);
            let fids = u16::from_le_bytes([data[6], data[7]]);

            println!("          data:");
            println!("            AGEN: {}  # Producing agency", agen);
            println!("            FIDN: {}  # Feature ID number", fidn);
            println!("            FIDS: {}  # Feature ID subdivision", fids);
        }
        "SG2D" => {
            println!("          data:");
            println!("            coordinates:  # 2D coordinate pairs (lat/lon)");
            let mut offset = 0;
            let mut coord_num = 0;
            while offset + 8 <= data.len() {
                if data[offset] == 0x1F || data[offset] == 0x1E {
                    break;
                }
                if offset + 8 <= data.len() {
                    let y = i32::from_le_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]);
                    let x = i32::from_le_bytes([
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]);
                    let lat = y as f64 / 10_000_000.0;
                    let lon = x as f64 / 10_000_000.0;
                    println!(
                        "              - coord_{}: {{ lat: {:.7}, lon: {:.7} }}",
                        coord_num, lat, lon
                    );
                    coord_num += 1;
                    offset += 8;
                }
            }
            if coord_num == 0 {
                println!("              []  # No valid coordinates found");
            }
        }
        "SG3D" => {
            println!("          data:");
            println!("            coordinates:  # 3D coordinate triplets (lat/lon/depth)");
            let mut offset = 0;
            let mut coord_num = 0;
            while offset + 12 <= data.len() {
                if data[offset] == 0x1F || data[offset] == 0x1E {
                    break;
                }
                if offset + 12 <= data.len() {
                    let y = i32::from_le_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]);
                    let x = i32::from_le_bytes([
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]);
                    let z = i32::from_le_bytes([
                        data[offset + 8],
                        data[offset + 9],
                        data[offset + 10],
                        data[offset + 11],
                    ]);
                    let lat = y as f64 / 10_000_000.0;
                    let lon = x as f64 / 10_000_000.0;
                    let depth = z as f64 / 100.0;
                    println!(
                        "              - coord_{}: {{ lat: {:.7}, lon: {:.7}, depth: {:.2} }}",
                        coord_num, lat, lon, depth
                    );
                    coord_num += 1;
                    offset += 12;
                }
            }
            if coord_num == 0 {
                println!("              []  # No valid coordinates found");
            }
        }
        "FSPT" => {
            println!("          data:");
            println!("            spatial_pointers:  # References to vector records");
            let mut offset = 0;
            let mut ptr_num = 0;
            while offset + 8 <= data.len() {
                if data[offset] == 0x1F || data[offset] == 0x1E {
                    break;
                }
                if offset + 8 <= data.len() {
                    let name = data[offset];
                    let rcid = u32::from_le_bytes([
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                        data[offset + 4],
                    ]);
                    let ornt = data[offset + 5];
                    let usag = data[offset + 6];
                    let mask = data[offset + 7];

                    println!("              - pointer_{}:", ptr_num);
                    println!(
                        "                  NAME: {}  # {}",
                        name,
                        interpret_record_name(name)
                    );
                    println!(
                        "                  RCID: {}  # Target vector record ID",
                        rcid
                    );
                    println!(
                        "                  ORNT: {}  # {}",
                        ornt,
                        interpret_orientation(ornt)
                    );
                    println!("                  USAG: {}  # Usage indicator", usag);
                    println!("                  MASK: {}  # Masking indicator", mask);
                    ptr_num += 1;
                    offset += 8;
                }
            }
            if ptr_num == 0 {
                println!("              []  # No valid pointers found");
            }
        }
        "DSID" | "DSSI" | "DSPM" | "ATTF" | "FFPT" | "VRPT" => {
            println!("          data: <binary>  # {} bytes", data.len());
        }
        _ => {
            // For other fields, show hex preview
            if data.len() > 0 {
                let preview: String = data
                    .iter()
                    .take(16)
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                let more = if data.len() > 16 { " ..." } else { "" };
                println!("          data: <hex: {}{}>", preview, more);
            }
        }
    }
}
