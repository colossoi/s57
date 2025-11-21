//! Feature indexing module for building SQLite database of chart features

use log::{info, warn};
use num_traits::ToPrimitive;
use rusqlite::{Connection, Result as SqlResult};
use s57_catalogue::ObjectClass;
use s57_interp::ecs::EntityType;
use s57_interp::topology::{ContinuityPolicy, EdgeWalker, TraversalContext};
use s57_parse::S57File;
use std::path::Path;

/// Convert group code to human-readable name
fn group_name(grup: u8) -> &'static str {
    match grup {
        1 => "Geographic",
        2 => "Meta",
        3 => "Collection",
        4 => "National",
        5 => "Chart",
        _ => "Unknown",
    }
}

/// Statistics from indexing operation
pub struct IndexStats {
    pub total_features: usize,
    pub indexed_features: usize,
    pub chart_min_lat: Option<f64>,
    pub chart_max_lat: Option<f64>,
    pub chart_min_lon: Option<f64>,
    pub chart_max_lon: Option<f64>,
    pub scale: u32,
}

/// Initialize database and create tables
fn init_database(db_path: &Path) -> SqlResult<Connection> {
    let conn = Connection::open(db_path)?;

    // Enable WAL mode for better concurrency and performance
    conn.pragma_update(None, "journal_mode", "WAL")?;

    // Disable fsync for faster writes (less safe but much faster)
    conn.pragma_update(None, "synchronous", "OFF")?;

    // Create features table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS features (
            filename TEXT NOT NULL,
            entity_id TEXT NOT NULL,
            geometry_type TEXT NOT NULL,
            scale INTEGER,
            object_code INTEGER NOT NULL,
            object_name TEXT NOT NULL,
            group_code INTEGER NOT NULL,
            group_name TEXT NOT NULL,
            version INTEGER,
            update_instruction INTEGER,
            min_lat REAL NOT NULL,
            max_lat REAL NOT NULL,
            min_lon REAL NOT NULL,
            max_lon REAL NOT NULL,
            PRIMARY KEY (filename, entity_id)
        )",
        [],
    )?;

    Ok(conn)
}

/// Index features from an S-57 file into the database
pub fn index_features(
    file: &S57File,
    file_path: &Path,
    database_path: &Path,
) -> Result<IndexStats, String> {
    // Build ECS World from S57 file
    let world =
        s57_interp::build_world(file).map_err(|e| format!("Failed to build world: {}", e))?;

    // Get compilation scale from dataset parameters
    let scale = world.dataset_params.as_ref().map(|p| p.cscl).unwrap_or(0);

    // Get filename
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Open database
    info!("Opening database: {}", database_path.display());
    let conn = init_database(database_path)
        .map_err(|e| format!("Failed to initialize database: {}", e))?;

    info!("Database table ready");

    // Prepare INSERT statement once for reuse
    let mut stmt = conn
        .prepare(
            "INSERT OR REPLACE INTO features
             (filename, entity_id, geometry_type, scale, object_code, object_name,
              group_code, group_name, version, update_instruction,
              min_lat, max_lat, min_lon, max_lon)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        )
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    // Set up topology traversal context
    let ctx =
        TraversalContext::new(&world).with_continuity_policy(ContinuityPolicy::InsertGapMarker);

    // Process each feature
    let features = world.entities_of_type(EntityType::Feature);
    let mut stats = IndexStats {
        total_features: 0,
        indexed_features: 0,
        chart_min_lat: None,
        chart_max_lat: None,
        chart_min_lon: None,
        chart_max_lon: None,
        scale,
    };

    for entity in features {
        if let Some(meta) = world.feature_meta.get(&entity) {
            stats.total_features += 1;

            // Get geometry type from primitive
            let geom_type = match meta.prim {
                1 => "point",
                2 => "line",
                3 => "polygon",
                _ => continue, // Skip unknown types
            };

            // Compute feature extent from its spatial references
            let mut feat_min_lat: Option<f64> = None;
            let mut feat_max_lat: Option<f64> = None;
            let mut feat_min_lon: Option<f64> = None;
            let mut feat_max_lon: Option<f64> = None;

            if let Some(pointers) = world.feature_pointers.get(&entity) {
                for sref in &pointers.spatial_refs {
                    // Get vector coordinates
                    if let Some(vmeta) = world.vector_meta.get(&sref.entity) {
                        let mut walker = EdgeWalker::new(&ctx);
                        if let Ok(coords) = walker.resolve_line_2d(vmeta.name) {
                            for (lat, lon) in coords {
                                let lat_f64 = lat.to_f64().unwrap_or(0.0);
                                let lon_f64 = lon.to_f64().unwrap_or(0.0);

                                feat_min_lat =
                                    Some(feat_min_lat.map_or(lat_f64, |v| v.min(lat_f64)));
                                feat_max_lat =
                                    Some(feat_max_lat.map_or(lat_f64, |v| v.max(lat_f64)));
                                feat_min_lon =
                                    Some(feat_min_lon.map_or(lon_f64, |v| v.min(lon_f64)));
                                feat_max_lon =
                                    Some(feat_max_lon.map_or(lon_f64, |v| v.max(lon_f64)));
                            }
                        }
                    }
                }
            }

            // Skip features with no coordinates
            if feat_min_lat.is_none() {
                continue;
            }

            let min_lat = feat_min_lat.unwrap();
            let max_lat = feat_max_lat.unwrap();
            let min_lon = feat_min_lon.unwrap();
            let max_lon = feat_max_lon.unwrap();

            // Update chart extent
            stats.chart_min_lat = Some(stats.chart_min_lat.map_or(min_lat, |v| v.min(min_lat)));
            stats.chart_max_lat = Some(stats.chart_max_lat.map_or(max_lat, |v| v.max(max_lat)));
            stats.chart_min_lon = Some(stats.chart_min_lon.map_or(min_lon, |v| v.min(min_lon)));
            stats.chart_max_lon = Some(stats.chart_max_lon.map_or(max_lon, |v| v.max(max_lon)));

            // Format entity ID
            let entity_id = format!("{}:{}:{}", meta.foid.agen, meta.foid.fidn, meta.foid.fids);

            // Get object name from catalogue
            let object_name = ObjectClass::from_code(meta.objl)
                .map(|c| c.name())
                .unwrap_or("Unknown");

            // Get group name
            let grp_name = group_name(meta.grup);

            // Insert into database using prepared statement
            match stmt.execute(rusqlite::params![
                filename,
                entity_id,
                geom_type,
                scale,
                meta.objl,
                object_name,
                meta.grup,
                grp_name,
                meta.rver,
                meta.ruin,
                min_lat,
                max_lat,
                min_lon,
                max_lon
            ]) {
                Ok(_) => {
                    stats.indexed_features += 1;
                    if stats.indexed_features % 100 == 0 {
                        info!("Indexed {} features...", stats.indexed_features);
                    }
                }
                Err(e) => {
                    warn!("Failed to insert feature {}: {}", entity_id, e);
                }
            }
        }
    }

    Ok(stats)
}
