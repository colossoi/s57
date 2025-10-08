use clap::Parser;
use s57::S57File;
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

    /// Show detailed information about each record
    #[arg(short, long)]
    details: bool,

    /// Print file structure in YAML format with semantic interpretation
    #[arg(short = 'y', long)]
    yaml: bool,

    /// Limit number of records to print (for YAML output)
    #[arg(long, default_value = "10")]
    limit: usize,
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

    println!("Parsing file: {}", cli.file.display());
    println!("File size: {} bytes", data.len());

    // Parse the S-57 file
    match S57File::from_bytes(&data) {
        Ok(file) => {
            let records = file.records();
            println!("Successfully parsed {} records", records.len());

            if cli.yaml {
                // Parse DDR first
                if let Some(ddr_record) = records.first() {
                    match s57::ddr::DDR::parse(ddr_record) {
                        Ok(ddr) => {
                            print_yaml_structure_with_ddr(records, cli.limit, &ddr);
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to parse DDR: {}", e);
                            print_yaml_structure(records, cli.limit);
                        }
                    }
                } else {
                    print_yaml_structure(records, cli.limit);
                }
            } else if cli.details {
                println!("\nRecord details:");
                for (i, record) in records.iter().enumerate() {
                    println!("\nRecord {}:", i);
                    println!("  Type: {}", if record.leader.is_ddr() { "DDR" } else { "DR" });
                    println!("  Length: {} bytes", record.leader.record_length);
                    println!("  Fields: {}", record.fields.len());

                    for field in &record.fields {
                        println!("    - {} ({} bytes)", field.tag, field.data.len());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_yaml_structure_with_ddr(records: &[s57::iso8211::Record], limit: usize, ddr: &s57::ddr::DDR) {
    use s57::interpret::*;

    println!("\n# Field definitions found in DDR:");
    for (tag, def) in ddr.field_defs() {
        if !tag.starts_with('0') {
            let repeating = if def.is_repeating { " [repeating]" } else { "" };
            println!("#   {}: {} ({} subfields{})", tag, def.name, def.subfield_count(), repeating);
        }
    }
    println!();

    println!("records:");
    for (i, record) in records.iter().enumerate().take(limit) {
        let record_type = if record.leader.is_ddr() { "DDR (Data Descriptive Record)" } else { "DR (Data Record)" };

        println!("  - record_{}:  # {}", i, record_type);
        println!("      leader:");
        println!("        record_length: {}  # bytes", record.leader.record_length);
        println!("        interchange_level: '{}'", record.leader.interchange_level);
        println!("        leader_identifier: '{}'  # {}",
            record.leader.leader_identifier,
            if record.leader.is_ddr() { "Data Descriptive" } else { "Data" }
        );
        println!("        base_address_of_field_area: {}", record.leader.base_address_of_field_area);

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
                if let Some((controls, name, array_desc, format_controls)) = field.parse_record_identifier_field() {
                    // DDR: data descriptive field defining record identifier structure
                    println!("          data:");
                    println!("            field_controls: \"{}\"", controls);
                    println!("            field_name: \"{}\"", name);
                    if !array_desc.is_empty() {
                        println!("            array_descriptor: \"{}\"", array_desc);
                    }
                    println!("            format_controls: \"{}\"  # Format for record ID in DRs", format_controls);
                } else if let Some((seq_num, _reserved)) = parse_field_control(&field.data) {
                    // Data record: contains sequence number
                    println!("          data:");
                    println!("            sequence_number: {}  # Record sequence in file", seq_num);
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
                        println!("            array_descriptor: \"{}\"  # Subfield labels", field_def.array_descriptor);
                    }
                    if !field_def.format_controls.is_empty() {
                        println!("            format_controls: \"{}\"  # Subfield types", field_def.format_controls);
                    }
                    println!("            subfield_count: {}  # Number of subfield labels", field_def.subfield_count());
                    if field_def.is_repeating {
                        println!("            repeating_group: true  # Group can repeat multiple times");
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
                                let indent = if parsed.groups().len() > 1 { "              " } else { "            " };
                                match value {
                                    s57::ddr::SubfieldValue::Null => {
                                        println!("{}{}: null", indent, label);
                                    }
                                    s57::ddr::SubfieldValue::Integer(i) => {
                                        let comment = match label.as_str() {
                                            "RCNM" => format!("  # {}", interpret_record_name(*i as u8)),
                                            "PRIM" => format!("  # {}", interpret_primitive(*i as u8)),
                                            "OBJL" => format!("  # {}", interpret_object_label(*i as u16)),
                                            "RUIN" => format!("  # {}", interpret_update_instruction(*i as u8)),
                                            "ORNT" => format!("  # {}", interpret_orientation(*i as u8)),
                                            _ => String::new(),
                                        };
                                        println!("{}{}: {}{}", indent, label, i, comment);
                                    }
                                    s57::ddr::SubfieldValue::Real(f) => {
                                        println!("{}{}: {:.6}", indent, label, f);
                                    }
                                    s57::ddr::SubfieldValue::String(s) => {
                                        println!("{}{}: \"{}\"", indent, label, s);
                                    }
                                    s57::ddr::SubfieldValue::Bytes(b) => {
                                        let hex: String = b.iter().take(8)
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

    if records.len() > limit {
        println!("  # ... {} more records (use --limit to see more)", records.len() - limit);
    }
}

fn print_yaml_structure(records: &[s57::iso8211::Record], limit: usize) {
    use s57::interpret::*;

    println!("\nrecords:");
    for (i, record) in records.iter().enumerate().take(limit) {
        let record_type = if record.leader.is_ddr() { "DDR (Data Descriptive Record)" } else { "DR (Data Record)" };

        println!("  - record_{}:  # {}", i, record_type);
        println!("      leader:");
        println!("        record_length: {}  # bytes", record.leader.record_length);
        println!("        interchange_level: '{}'", record.leader.interchange_level);
        println!("        leader_identifier: '{}'  # {}",
            record.leader.leader_identifier,
            if record.leader.is_ddr() { "Data Descriptive" } else { "Data" }
        );
        println!("        base_address_of_field_area: {}", record.leader.base_address_of_field_area);

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

    if records.len() > limit {
        println!("  # ... {} more records (use --limit to see more)", records.len() - limit);
    }
}

fn print_field_interpretation(tag: &str, data: &[u8]) {
    use s57::interpret::*;

    match tag {
        "0001" => {
            if let Some((seq_num, _reserved)) = parse_field_control(data) {
                // Data record: contains sequence number
                println!("          data:");
                println!("            sequence_number: {}  # Record sequence in file", seq_num);
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
            println!("            RCNM: {}  # {}", rcnm, interpret_record_name(rcnm));
            println!("            RCID: {}  # Record ID", rcid);
            println!("            PRIM: {}  # {}", prim, interpret_primitive(prim));
            println!("            GRUP: {}  # Group", grup);
            println!("            OBJL: {}  # {}", objl, interpret_object_label(objl));
            println!("            RVER: {}  # Record version", rver);
            println!("            RUIN: {}  # {}", ruin, interpret_update_instruction(ruin));
        }
        "VRID" if data.len() >= 8 => {
            let rcnm = data[0];
            let rcid = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
            let rver = u16::from_le_bytes([data[5], data[6]]);
            let ruin = data[7];

            println!("          data:");
            println!("            RCNM: {}  # {}", rcnm, interpret_record_name(rcnm));
            println!("            RCID: {}  # Record ID", rcid);
            println!("            RVER: {}  # Record version", rver);
            println!("            RUIN: {}  # {}", ruin, interpret_update_instruction(ruin));
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
                    let y = i32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
                    let x = i32::from_le_bytes([data[offset+4], data[offset+5], data[offset+6], data[offset+7]]);
                    let lat = y as f64 / 10_000_000.0;
                    let lon = x as f64 / 10_000_000.0;
                    println!("              - coord_{}: {{ lat: {:.7}, lon: {:.7} }}", coord_num, lat, lon);
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
                    let y = i32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
                    let x = i32::from_le_bytes([data[offset+4], data[offset+5], data[offset+6], data[offset+7]]);
                    let z = i32::from_le_bytes([data[offset+8], data[offset+9], data[offset+10], data[offset+11]]);
                    let lat = y as f64 / 10_000_000.0;
                    let lon = x as f64 / 10_000_000.0;
                    let depth = z as f64 / 100.0;
                    println!("              - coord_{}: {{ lat: {:.7}, lon: {:.7}, depth: {:.2} }}",
                        coord_num, lat, lon, depth);
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
                    let rcid = u32::from_le_bytes([data[offset+1], data[offset+2], data[offset+3], data[offset+4]]);
                    let ornt = data[offset+5];
                    let usag = data[offset+6];
                    let mask = data[offset+7];

                    println!("              - pointer_{}:", ptr_num);
                    println!("                  NAME: {}  # {}", name, interpret_record_name(name));
                    println!("                  RCID: {}  # Target vector record ID", rcid);
                    println!("                  ORNT: {}  # {}", ornt, interpret_orientation(ornt));
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
                let preview: String = data.iter()
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
