use num_traits::ToPrimitive;
use s57_catalogue::{AttributeInfo, ObjectClass};
use s57_interp::ecs::EntityType;
use s57_parse::S57File;

pub fn list_features(file: &S57File) {
    // Build ECS World from S57 file
    let world = match s57_interp::build_world(file) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error building world: {}", e);
            std::process::exit(1);
        }
    };

    println!("Feature Objects:");
    println!(
        "{:<10} {:<8} {:<8} {:<40} {:<25}",
        "FOID", "PRIM", "OBJL", "Object Name", "Object Type"
    );
    println!("{}", "-".repeat(95));

    let mut feature_count = 0;

    // Get all feature entities
    let features = world.entities_of_type(EntityType::Feature);

    for entity in features {
        if let Some(meta) = world.feature_meta.get(&entity) {
            // Get primitive type
            let prim_str = match meta.prim {
                1 => "Point",
                2 => "Line",
                3 => "Area",
                255 => "N/A",
                _ => "Unknown",
            };

            // Get object class name
            let objl_str = ObjectClass::from_code(meta.objl)
                .map(|c| c.name().to_string())
                .unwrap_or_else(|| format!("Unknown ({})", meta.objl));

            // Get object name from attributes (OBJNAM = 116)
            let objnam = world
                .feature_attributes
                .get(&entity)
                .and_then(|attrs| {
                    attrs
                        .attf
                        .iter()
                        .find(|(attl, _)| *attl == 116)
                        .map(|(_, atvl)| atvl.clone())
                })
                .unwrap_or_default();

            // Format FOID
            let foid_str = format!("{}:{}:{}", meta.foid.agen, meta.foid.fidn, meta.foid.fids);

            println!(
                "{:<10} {:<8} {:<8} {:<40} {:<25}",
                foid_str,
                prim_str,
                meta.objl,
                if objnam.is_empty() {
                    "-"
                } else {
                    &objnam[..objnam.len().min(40)]
                },
                objl_str
            );

            feature_count += 1;
        }
    }

    println!("\nTotal features: {}", feature_count);
}

pub fn show_object(file: &S57File, target_rcid: u32) {
    // Build ECS World from S57 file
    let world = match s57_interp::build_world(file) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error building world: {}", e);
            std::process::exit(1);
        }
    };

    // Find feature with matching FOID
    // Note: We're searching by RCID but features use FOID keys
    // We'll need to iterate through feature_meta to find matching fidn
    let mut found = None;
    for (entity, meta) in &world.feature_meta {
        if meta.foid.fidn == target_rcid {
            found = Some((*entity, meta.clone()));
            break;
        }
    }

    let (entity, meta) = match found {
        Some(data) => data,
        None => {
            eprintln!("Error: Feature object with RCID {} not found", target_rcid);
            std::process::exit(1);
        }
    };

    println!(
        "Feature Object: FOID {}:{}:{}\n",
        meta.foid.agen, meta.foid.fidn, meta.foid.fids
    );
    println!("Metadata:");
    println!(
        "  Primitive: {} ({})",
        meta.prim,
        match meta.prim {
            1 => "Point",
            2 => "Line",
            3 => "Area",
            255 => "N/A",
            _ => "Unknown",
        }
    );
    println!("  Group: {}", meta.grup);
    println!(
        "  Object Label: {} ({})",
        meta.objl,
        ObjectClass::from_code(meta.objl)
            .map(|c| c.name().to_string())
            .unwrap_or_else(|| format!("Unknown ({})", meta.objl))
    );
    println!("  Record Version: {}", meta.rver);
    println!("  Update Instruction: {}", meta.ruin);

    // Print attributes
    if let Some(attrs) = world.feature_attributes.get(&entity) {
        if !attrs.attf.is_empty() {
            println!("\nAttributes (ATTF):");
            for (attl, atvl) in &attrs.attf {
                let attr_name = AttributeInfo::attribute_name(*attl).unwrap_or("Unknown");
                println!("  {} = \"{}\" ({})", attl, atvl, attr_name);
            }
        }

        if !attrs.natf.is_empty() {
            println!("\nNational Attributes (NATF):");
            for (attl, atvl) in &attrs.natf {
                println!("  {} = \"{}\"", attl, atvl);
            }
        }
    }

    // Print spatial references
    if let Some(pointers) = world.feature_pointers.get(&entity) {
        if !pointers.spatial_refs.is_empty() {
            println!(
                "\nSpatial References ({} vectors):",
                pointers.spatial_refs.len()
            );
            for (idx, sref) in pointers.spatial_refs.iter().enumerate() {
                if let Some(vmeta) = world.vector_meta.get(&sref.entity) {
                    let ornt_str = match sref.ornt {
                        1 => "forward",
                        2 => "reverse",
                        255 => "N/A",
                        _ => "unknown",
                    };
                    let rcnm_str = match vmeta.name.rcnm {
                        110 => "VI (Isolated Node)",
                        120 => "VC (Connected Node)",
                        130 => "VE (Edge)",
                        _ => "Unknown",
                    };
                    let usag_str = match sref.usag {
                        1 => "exterior",
                        2 => "interior",
                        3 => "exterior (truncated by limit)",
                        _ => "",
                    };
                    println!(
                        "  [{}] Vector {} {}:{} (ornt={}, usag={}{}, mask={})",
                        idx,
                        rcnm_str,
                        vmeta.name.rcnm,
                        vmeta.name.rcid,
                        ornt_str,
                        sref.usag,
                        if usag_str.is_empty() {
                            String::new()
                        } else {
                            format!(" [{}]", usag_str)
                        },
                        sref.mask
                    );

                    // Try to resolve coordinates via TTS (handles both direct and topology-derived)
                    use s57_interp::topology::{ContinuityPolicy, EdgeWalker, TraversalContext};

                    let ctx = TraversalContext::new(&world)
                        .with_continuity_policy(ContinuityPolicy::InsertGapMarker);
                    let mut walker = EdgeWalker::new(&ctx);

                    match walker.resolve_line_2d(vmeta.name) {
                        Ok(coords) => {
                            if !coords.is_empty() {
                                println!("       {} coordinate points", coords.len());
                                if coords.len() <= 500 {
                                    for (lat, lon) in &coords {
                                        // Convert BigRational to f64 for display
                                        let lat_f64 = lat.to_f64().unwrap_or(0.0);
                                        let lon_f64 = lon.to_f64().unwrap_or(0.0);
                                        println!("         ({:.7}, {:.7})", lat_f64, lon_f64);
                                    }
                                } else {
                                    let first_lat = coords[0].0.to_f64().unwrap_or(0.0);
                                    let first_lon = coords[0].1.to_f64().unwrap_or(0.0);
                                    let last_lat =
                                        coords[coords.len() - 1].0.to_f64().unwrap_or(0.0);
                                    let last_lon =
                                        coords[coords.len() - 1].1.to_f64().unwrap_or(0.0);
                                    println!(
                                        "         First: ({:.7}, {:.7})",
                                        first_lat, first_lon
                                    );
                                    println!("         Last:  ({:.7}, {:.7})", last_lat, last_lon);
                                }
                            }
                        }
                        Err(e) => {
                            println!("       (topology resolution failed: {})", e);
                        }
                    }
                }
            }
        }

        if !pointers.related_features.is_empty() {
            println!("\nRelated Features ({}):", pointers.related_features.len());
            for (idx, related_entity) in pointers.related_features.iter().enumerate() {
                if let Some(rmeta) = world.feature_meta.get(related_entity) {
                    let objl_str = ObjectClass::from_code(rmeta.objl)
                        .map(|c| c.name().to_string())
                        .unwrap_or_else(|| format!("Unknown ({})", rmeta.objl));
                    println!(
                        "  [{}] FOID {}:{}:{} - {} ({})",
                        idx,
                        rmeta.foid.agen,
                        rmeta.foid.fidn,
                        rmeta.foid.fids,
                        rmeta.objl,
                        objl_str
                    );
                }
            }
        }
    }
}

pub fn print_extent(file: &S57File) {
    // Build ECS World from S57 file
    let world = match s57_interp::build_world(file) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error building world: {}", e);
            std::process::exit(1);
        }
    };

    // Compute extent from all coordinates (S-57 files don't store pre-computed bounds)
    let mut min_lat: Option<f64> = None;
    let mut max_lat: Option<f64> = None;
    let mut min_lon: Option<f64> = None;
    let mut max_lon: Option<f64> = None;
    let mut coord_count = 0;

    // Iterate through all vector entities with exact positions
    for (_entity, positions) in &world.exact_positions {
        for i in 0..positions.lat.len() {
            let lat = positions.lat[i].to_f64().unwrap_or(0.0);
            let lon = positions.lon[i].to_f64().unwrap_or(0.0);

            min_lat = Some(min_lat.map_or(lat, |v| v.min(lat)));
            max_lat = Some(max_lat.map_or(lat, |v| v.max(lat)));
            min_lon = Some(min_lon.map_or(lon, |v| v.min(lon)));
            max_lon = Some(max_lon.map_or(lon, |v| v.max(lon)));
            coord_count += 1;
        }
    }

    if coord_count == 0 {
        println!("No coordinates found in chart");
        return;
    }

    // Print extent
    println!("Geographic Extent:");
    println!(
        "  Latitude:  {:.7} to {:.7}",
        min_lat.unwrap(),
        max_lat.unwrap()
    );
    println!(
        "  Longitude: {:.7} to {:.7}",
        min_lon.unwrap(),
        max_lon.unwrap()
    );
    println!("  Total coordinates: {}", coord_count);

    // Calculate dimensions
    let lat_span = max_lat.unwrap() - min_lat.unwrap();
    let lon_span = max_lon.unwrap() - min_lon.unwrap();
    println!("\nDimensions:");
    println!(
        "  Latitude span:  {:.7}° ({:.2} km)",
        lat_span,
        lat_span * 111.0
    );
    println!(
        "  Longitude span: {:.7}° ({:.2} km at center)",
        lon_span,
        lon_span
            * 111.0
            * ((min_lat.unwrap() + max_lat.unwrap()) / 2.0)
                .to_radians()
                .cos()
    );
}
