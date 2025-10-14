//! Rendering S-57 features to SVG

use log::info;
use num_traits::ToPrimitive;
use s57_interp::ecs::{EntityId, EntityType, World};
use s57_interp::topology::{ContinuityPolicy, EdgeWalker, FeatureBoundaryCursor, TraversalContext};
use s57_parse::S57File;
use std::path::PathBuf;

pub fn render_to_svg(
    file: &S57File,
    output_path: &PathBuf,
    limit: Option<usize>,
    width: u32,
    height: u32,
) {
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
            // Skip metadata features (chart quality/coverage info, objl 300-312)
            if meta.objl >= 300 && meta.objl <= 312 {
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

fn render_area(
    _world: &World,
    ctx: &TraversalContext,
    foid: s57_parse::bitstring::FoidKey,
    feature_id: &str,
    renderer: &mut crate::svg::SvgRenderer,
) {
    // Use FeatureBoundaryCursor to resolve area boundary rings
    let cursor = FeatureBoundaryCursor::new(ctx, foid);

    match cursor.resolve_rings() {
        Ok(rings) => {
            info!("Resolved {} rings for feature {}", rings.len(), feature_id);
            for (i, ring) in rings.iter().enumerate() {
                info!("Ring {} has {} points", i, ring.len());
                let points: Vec<_> = ring
                    .iter()
                    .filter_map(|(lat, lon)| Some((lat.to_f64()?, lon.to_f64()?)))
                    .collect();

                if !points.is_empty() {
                    info!("Adding polygon with {} points", points.len());
                    renderer.add_polygon(
                        points,
                        "#90ee90".to_string(),
                        "#008000".to_string(),
                        0.5,
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
