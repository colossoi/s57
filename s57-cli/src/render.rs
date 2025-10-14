//! Rendering S-57 features to SVG

use log::info;
use num_traits::ToPrimitive;
use s57_catalogue::ObjectClass;
use s57_interp::ecs::{EntityId, EntityType, World};
use s57_interp::topology::{ContinuityPolicy, EdgeWalker, FeatureBoundaryCursor, TraversalContext};
use s57_parse::S57File;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

pub fn render_to_svg(
    file: &S57File,
    output_path: &PathBuf,
    limit: Option<usize>,
    feature_filter: Option<u32>,
    class_filter: &[String],
    width: u32,
    height: u32,
) {
    // Parse class filter into object codes
    let allowed_classes: HashSet<u16> = {
        let mut classes = HashSet::new();
        for class_name in class_filter {
            match ObjectClass::from_str(class_name) {
                Ok(obj_class) => {
                    classes.insert(obj_class.code());
                }
                Err(_) => {
                    info!("Unknown object class '{}', skipping", class_name);
                }
            }
        }
        if classes.is_empty() {
            log::error!("No valid object classes specified");
            std::process::exit(1);
        }
        classes
    };
    // Build ECS World from S57 file
    let world = match s57_interp::build_world(file) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("Error building world: {}", e);
            std::process::exit(1);
        }
    };

    println!("Rendering chart to SVG...");

    // Create SVG renderer
    let mut renderer = crate::svg::SvgRenderer::new().with_dimensions(width, height);

    // Set up traversal context with gap marker and cycle policies
    let ctx = TraversalContext::new(&world)
        .with_continuity_policy(ContinuityPolicy::InsertGapMarker)
        .with_cycle_policy(s57_interp::topology::CyclePolicy::AllowVisitCount(2));

    // Get all feature entities
    let features = world.entities_of_type(EntityType::Feature);
    let feature_count = limit.unwrap_or(features.len()).min(features.len());

    println!("Processing {} features...", feature_count);

    let mut rendered_count = 0;

    for entity in features.iter().take(feature_count) {
        if let Some(meta) = world.feature_meta.get(entity) {
            // Filter by specific feature if requested
            if let Some(fidn) = feature_filter {
                if meta.foid.fidn != fidn {
                    continue;
                }
            }

            // Skip metadata features (chart quality/coverage info, objl 300-312)
            if meta.objl >= 300 && meta.objl <= 312 {
                continue;
            }

            // Filter by object class
            if !allowed_classes.contains(&meta.objl) {
                continue;
            }

            let foid_str = format!("{}:{}:{}", meta.foid.agen, meta.foid.fidn, meta.foid.fids);
            let obj_name = s57_catalogue::decode_object(meta.objl)
                .map(|c| c.name())
                .unwrap_or("Unknown");

            info!("Rendering feature {} ({})", foid_str, obj_name);

            // Render based on primitive type
            match meta.prim {
                1 => {
                    // Point feature - render as circle
                    render_point(&world, &ctx, entity, &foid_str, &mut renderer);
                }
                2 => {
                    // Line feature - render as polyline
                    render_line(&world, &ctx, entity, &foid_str, &mut renderer);
                }
                3 => {
                    // Area feature - render as polygon
                    render_area(&world, &ctx, meta.foid, &foid_str, &mut renderer);
                }
                _ => {}
            }

            rendered_count += 1;
        }
    }

    println!("Rendered {} features", rendered_count);

    // Write SVG to file
    let mut file = match std::fs::File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = renderer.render(&mut file) {
        eprintln!("Error rendering SVG: {}", e);
        std::process::exit(1);
    }

    println!("SVG written to: {}", output_path.display());
}

fn render_point(
    world: &World,
    _ctx: &TraversalContext,
    entity: &EntityId,
    feature_id: &str,
    renderer: &mut crate::svg::SvgRenderer,
) {
    let title = get_feature_title(world, entity);

    // Get spatial references
    if let Some(pointers) = world.feature_pointers.get(entity) {
        for sref in &pointers.spatial_refs {
            if let Some(positions) = world.exact_positions.get(&sref.entity) {
                let (lat, lon) = positions.to_f64();
                if !lat.is_empty() {
                    renderer.add_point(
                        lat[0],
                        lon[0],
                        2.0,
                        "#ff0000".to_string(),
                        title.clone(),
                        Some(feature_id.to_string()),
                    );
                }
            }
        }
    }
}

fn get_feature_title(world: &World, entity: &EntityId) -> Option<String> {
    let meta = world.feature_meta.get(entity)?;

    // Get object name from ATTF attributes if available (OBJNAM = attribute code 116)
    let name = world.feature_attributes.get(entity).and_then(|attrs| {
        attrs
            .attf
            .iter()
            .find(|(code, _)| *code == 116)
            .map(|(_, value)| value.as_str())
    });

    // Get object type label
    let obj_type = s57_catalogue::decode_object(meta.objl)
        .map(|c| c.name())
        .unwrap_or("Unknown");

    // Build title string
    match name {
        Some(n) if !n.is_empty() => Some(format!("{} - {}", obj_type, n)),
        _ => Some(obj_type.to_string()),
    }
}

fn render_line(
    world: &World,
    ctx: &TraversalContext,
    entity: &EntityId,
    feature_id: &str,
    renderer: &mut crate::svg::SvgRenderer,
) {
    // Get spatial references
    if let Some(pointers) = world.feature_pointers.get(entity) {
        for sref in &pointers.spatial_refs {
            if let Some(vmeta) = world.vector_meta.get(&sref.entity) {
                // Use EdgeWalker to resolve line geometry
                let mut walker = EdgeWalker::new(ctx);
                if let Ok(coords) = walker.resolve_line_2d(vmeta.name) {
                    let points: Vec<_> = coords
                        .iter()
                        .filter_map(|(lat, lon)| Some((lat.to_f64()?, lon.to_f64()?)))
                        .collect();

                    if !points.is_empty() {
                        renderer.add_polyline(
                            points,
                            "#0000ff".to_string(),
                            1.0,
                            Some(feature_id.to_string()),
                        );
                    }
                }
            }
        }
    }
}

/// Determine fill and stroke colors based on object class code
fn get_area_colors(objl: u16) -> (String, String, f64) {
    match objl {
        // Water areas - light blue fill
        42 | 17003 => ("#87ceeb".to_string(), "#4682b4".to_string(), 0.5), // DEPARE - Depth area
        119 => ("#87ceeb".to_string(), "#4682b4".to_string(), 0.5), // SEAARE - Sea area / named water area

        // Land areas - green
        71 => ("#90ee90".to_string(), "#228b22".to_string(), 0.5), // LNDARE - Land area

        // Zone-type features - transparent with colored outline
        1 => ("none".to_string(), "#ff6b6b".to_string(), 1.5), // ADMARE - Administration area
        4 | 17001 => ("none".to_string(), "#9370db".to_string(), 1.5), // ACHARE - Anchorage area
        27 => ("none".to_string(), "#ffa500".to_string(), 2.0), // CTNARE - Caution area
        31 => ("none".to_string(), "#4169e1".to_string(), 1.5), // CONZNE - Contiguous zone
        32 => ("none".to_string(), "#4682b4".to_string(), 1.5), // COSARE - Continental shelf area
        37 => ("none".to_string(), "#daa520".to_string(), 1.5), // CUSZNE - Custom zone
        40 => ("none".to_string(), "#ff1493".to_string(), 2.0), // DWRTCL - Deep water route centerline
        41 => ("none".to_string(), "#ff69b4".to_string(), 1.5), // DWRTPT - Deep water route part
        50 => ("none".to_string(), "#1e90ff".to_string(), 1.5), // EXEZNE - Exclusive Economic Zone
        51 => ("none".to_string(), "#00ced1".to_string(), 2.0), // FAIRWY - Fairway
        54 => ("none".to_string(), "#20b2aa".to_string(), 1.5), // FSHZNE - Fishery zone
        63 | 17014 => ("none".to_string(), "#6a5acd".to_string(), 1.5), // HRBARE - Harbour area
        68 => ("none".to_string(), "#48d1cc".to_string(), 1.5), // ISTZNE - Inshore traffic zone
        83 => ("none".to_string(), "#dc143c".to_string(), 2.0), // MIPARE - Military practice area
        88 => ("none".to_string(), "#ff8c00".to_string(), 1.5), // OSPARE - Offshore production area
        96 => ("none".to_string(), "#ffa500".to_string(), 2.0), // PRCARE - Precautionary area
        97 => ("none".to_string(), "#ff8c00".to_string(), 1.5), // PRDARE - Production/storage area
        108 => ("none".to_string(), "#ff1493".to_string(), 2.0), // RCRTCL - Recommended route centerline
        112 | 17005 => ("none".to_string(), "#ff0000".to_string(), 2.0), // RESARE - Restricted area
        135 => ("none".to_string(), "#4682b4".to_string(), 1.5), // TESARE - Territorial sea area
        150 => ("none".to_string(), "#ff00ff".to_string(), 2.0), // TSEZNE - Traffic Separation Zone
        152 => ("none".to_string(), "#ff69b4".to_string(), 1.5), // TWRTPT - Two-way route part

        // Default - light green with darker outline
        _ => ("#90ee90".to_string(), "#228b22".to_string(), 0.5),
    }
}

fn render_area(
    world: &World,
    ctx: &TraversalContext,
    foid: s57_parse::bitstring::FoidKey,
    feature_id: &str,
    renderer: &mut crate::svg::SvgRenderer,
) {
    // Get object class for color selection
    let objl = world
        .feature_meta
        .values()
        .find(|meta| meta.foid == foid)
        .map(|meta| meta.objl)
        .unwrap_or(0);

    let (fill, stroke, stroke_width) = get_area_colors(objl);

    // Use FeatureBoundaryCursor to resolve area boundary rings
    let cursor = FeatureBoundaryCursor::new(ctx, foid);

    match cursor.resolve_rings() {
        Ok(rings) => {
            info!("Resolved {} rings for feature {}", rings.len(), feature_id);

            // Convert all rings to f64 coordinates
            let mut converted_rings = Vec::new();
            for (i, ring) in rings.iter().enumerate() {
                info!("Ring {} has {} points", i, ring.len());
                let points: Vec<_> = ring
                    .iter()
                    .filter_map(|(lat, lon)| Some((lat.to_f64()?, lon.to_f64()?)))
                    .collect();

                if !points.is_empty() {
                    converted_rings.push(points);
                }
            }

            // Render as polygon with holes if we have multiple rings
            if !converted_rings.is_empty() {
                if converted_rings.len() == 1 {
                    // Simple polygon without holes
                    info!(
                        "Adding simple polygon with {} points",
                        converted_rings[0].len()
                    );
                    renderer.add_polygon(
                        converted_rings.into_iter().next().unwrap(),
                        fill,
                        stroke,
                        stroke_width,
                        Some(feature_id.to_string()),
                    );
                } else {
                    // Polygon with holes
                    info!(
                        "Adding polygon with {} rings (1 exterior + {} holes)",
                        converted_rings.len(),
                        converted_rings.len() - 1
                    );
                    renderer.add_polygon_with_holes(
                        converted_rings,
                        fill,
                        stroke,
                        stroke_width,
                        Some(feature_id.to_string()),
                    );
                }
            }
        }
        Err(e) => {
            info!("Failed to resolve rings for {}: {}", feature_id, e);
        }
    }
}
