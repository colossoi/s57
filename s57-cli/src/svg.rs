//! SVG renderer for S-57 chart data
//!
//! Renders lat/lon coordinates to SVG format with automatic bounding box
//! calculation and coordinate normalization.

use std::io::Write;

/// Drawing primitive for SVG rendering
#[derive(Debug, Clone)]
pub enum Primitive {
    /// Polyline (open path)
    Polyline {
        points: Vec<(f64, f64)>,
        stroke: String,
        stroke_width: f64,
        id: Option<String>,
    },
    /// Polygon (closed path)
    Polygon {
        points: Vec<(f64, f64)>,
        fill: String,
        stroke: String,
        stroke_width: f64,
        id: Option<String>,
    },
    /// Polygon with holes (exterior ring + interior holes)
    PolygonWithHoles {
        rings: Vec<Vec<(f64, f64)>>, // First ring is exterior, rest are holes
        fill: String,
        stroke: String,
        stroke_width: f64,
        id: Option<String>,
    },
    /// Point marker
    Point {
        lat: f64,
        lon: f64,
        radius: f64,
        fill: String,
        title: Option<String>,
        id: Option<String>,
    },
}

/// SVG renderer with bounding box tracking
pub struct SvgRenderer {
    /// Drawing primitives to render
    primitives: Vec<Primitive>,
    /// Bounding box: (min_lat, min_lon, max_lat, max_lon)
    bbox: Option<(f64, f64, f64, f64)>,
    /// Canvas width in pixels
    width: u32,
    /// Canvas height in pixels
    height: u32,
    /// Padding around the content (in pixels)
    padding: f64,
}

impl SvgRenderer {
    /// Create a new SVG renderer with default canvas size
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            bbox: None,
            width: 800,
            height: 600,
            padding: 20.0,
        }
    }

    /// Set canvas dimensions
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set padding around content
    pub fn with_padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }

    /// Add a polyline to the renderer
    pub fn add_polyline(
        &mut self,
        points: impl IntoIterator<Item = (f64, f64)>,
        stroke: String,
        stroke_width: f64,
        id: Option<String>,
    ) {
        let points: Vec<_> = points.into_iter().collect();
        self.update_bbox(points.iter().copied());
        self.primitives.push(Primitive::Polyline {
            points,
            stroke,
            stroke_width,
            id,
        });
    }

    /// Add a polygon to the renderer
    pub fn add_polygon(
        &mut self,
        points: impl IntoIterator<Item = (f64, f64)>,
        fill: String,
        stroke: String,
        stroke_width: f64,
        id: Option<String>,
    ) {
        let points: Vec<_> = points.into_iter().collect();
        self.update_bbox(points.iter().copied());
        self.primitives.push(Primitive::Polygon {
            points,
            fill,
            stroke,
            stroke_width,
            id,
        });
    }

    /// Add a polygon with holes to the renderer
    /// First ring is the exterior boundary, remaining rings are interior holes
    pub fn add_polygon_with_holes(
        &mut self,
        rings: Vec<Vec<(f64, f64)>>,
        fill: String,
        stroke: String,
        stroke_width: f64,
        id: Option<String>,
    ) {
        // Update bbox with all rings
        for ring in &rings {
            self.update_bbox(ring.iter().copied());
        }
        self.primitives.push(Primitive::PolygonWithHoles {
            rings,
            fill,
            stroke,
            stroke_width,
            id,
        });
    }

    /// Add a point marker to the renderer
    pub fn add_point(
        &mut self,
        lat: f64,
        lon: f64,
        radius: f64,
        fill: String,
        title: Option<String>,
        id: Option<String>,
    ) {
        self.update_bbox(std::iter::once((lat, lon)));
        self.primitives.push(Primitive::Point {
            lat,
            lon,
            radius,
            fill,
            title,
            id,
        });
    }

    /// Update bounding box with new points
    fn update_bbox(&mut self, points: impl IntoIterator<Item = (f64, f64)>) {
        for (lat, lon) in points {
            if let Some((min_lat, min_lon, max_lat, max_lon)) = self.bbox {
                self.bbox = Some((
                    min_lat.min(lat),
                    min_lon.min(lon),
                    max_lat.max(lat),
                    max_lon.max(lon),
                ));
            } else {
                self.bbox = Some((lat, lon, lat, lon));
            }
        }
    }

    /// Transform lat/lon to SVG coordinates
    fn transform(&self, lat: f64, lon: f64) -> (f64, f64) {
        let (min_lat, min_lon, max_lat, max_lon) = self.bbox.unwrap();

        // Calculate available drawing area
        let draw_width = self.width as f64 - 2.0 * self.padding;
        let draw_height = self.height as f64 - 2.0 * self.padding;

        // Calculate scale factors
        let lat_range = max_lat - min_lat;
        let lon_range = max_lon - min_lon;

        // Handle degenerate cases
        if lat_range == 0.0 && lon_range == 0.0 {
            // Single point - center it
            return (self.width as f64 / 2.0, self.height as f64 / 2.0);
        }

        let scale_x = if lon_range > 0.0 {
            draw_width / lon_range
        } else {
            1.0
        };

        let scale_y = if lat_range > 0.0 {
            draw_height / lat_range
        } else {
            1.0
        };

        // Use uniform scale to preserve aspect ratio
        let scale = scale_x.min(scale_y);

        // Center the content
        let content_width = lon_range * scale;
        let content_height = lat_range * scale;
        let offset_x = self.padding + (draw_width - content_width) / 2.0;
        let offset_y = self.padding + (draw_height - content_height) / 2.0;

        // Transform coordinates
        // Note: SVG Y axis goes down, so we flip latitude
        let x = offset_x + (lon - min_lon) * scale;
        let y = offset_y + (max_lat - lat) * scale;

        (x, y)
    }

    /// Render all primitives to SVG
    pub fn render<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if self.bbox.is_none() {
            writeln!(
                writer,
                "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
                self.width, self.height
            )?;
            writeln!(
                writer,
                "  <text x=\"{}\" y=\"{}\" font-family=\"Arial\" font-size=\"16\">No data to render</text>",
                self.width / 2, self.height / 2
            )?;
            writeln!(writer, "</svg>")?;
            return Ok(());
        }

        // Write SVG header
        writeln!(
            writer,
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height
        )?;

        // Add background
        writeln!(
            writer,
            "  <rect width=\"{}\" height=\"{}\" fill=\"#e8f4f8\"/>",
            self.width, self.height
        )?;

        // Render primitives
        for primitive in &self.primitives {
            match primitive {
                Primitive::Polyline {
                    points,
                    stroke,
                    stroke_width,
                    id,
                } => {
                    write!(writer, "  <polyline points=\"")?;
                    for (i, &(lat, lon)) in points.iter().enumerate() {
                        let (x, y) = self.transform(lat, lon);
                        if i > 0 {
                            write!(writer, " ")?;
                        }
                        write!(writer, "{:.2},{:.2}", x, y)?;
                    }
                    write!(
                        writer,
                        "\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"",
                        stroke, stroke_width
                    )?;
                    if let Some(id_val) = id {
                        write!(writer, " data-feature-id=\"{}\"", escape_xml(id_val))?;
                    }
                    writeln!(writer, "/>")?;
                }
                Primitive::Polygon {
                    points,
                    fill,
                    stroke,
                    stroke_width,
                    id,
                } => {
                    write!(writer, "  <polygon points=\"")?;
                    for (i, &(lat, lon)) in points.iter().enumerate() {
                        let (x, y) = self.transform(lat, lon);
                        if i > 0 {
                            write!(writer, " ")?;
                        }
                        write!(writer, "{:.2},{:.2}", x, y)?;
                    }
                    write!(
                        writer,
                        "\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"",
                        fill, stroke, stroke_width
                    )?;
                    if let Some(id_val) = id {
                        write!(writer, " data-feature-id=\"{}\"", escape_xml(id_val))?;
                    }
                    writeln!(writer, "/>")?;
                }
                Primitive::PolygonWithHoles {
                    rings,
                    fill,
                    stroke,
                    stroke_width,
                    id,
                } => {
                    // Use SVG path element with fill-rule="evenodd" to handle holes
                    write!(writer, "  <path d=\"")?;

                    for (ring_idx, ring) in rings.iter().enumerate() {
                        if ring.is_empty() {
                            continue;
                        }

                        // Move to first point
                        let (lat, lon) = ring[0];
                        let (x, y) = self.transform(lat, lon);
                        write!(writer, "M {:.2},{:.2} ", x, y)?;

                        // Line to subsequent points
                        for &(lat, lon) in &ring[1..] {
                            let (x, y) = self.transform(lat, lon);
                            write!(writer, "L {:.2},{:.2} ", x, y)?;
                        }

                        // Close path
                        write!(writer, "Z ")?;

                        // Space between rings for readability
                        if ring_idx < rings.len() - 1 {
                            write!(writer, "")?;
                        }
                    }

                    write!(
                        writer,
                        "\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill-rule=\"evenodd\"",
                        fill, stroke, stroke_width
                    )?;
                    if let Some(id_val) = id {
                        write!(writer, " data-feature-id=\"{}\"", escape_xml(id_val))?;
                    }
                    writeln!(writer, "/>")?;
                }
                Primitive::Point {
                    lat,
                    lon,
                    radius,
                    fill,
                    title,
                    id,
                } => {
                    let (x, y) = self.transform(*lat, *lon);
                    let id_attr = id
                        .as_ref()
                        .map(|i| format!(" data-feature-id=\"{}\"", escape_xml(i)))
                        .unwrap_or_default();

                    if let Some(title_text) = title {
                        writeln!(
                            writer,
                            "  <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{}\" fill=\"{}\"{}>",
                            x, y, radius, fill, id_attr
                        )?;
                        writeln!(writer, "    <title>{}</title>", escape_xml(title_text))?;
                        writeln!(writer, "  </circle>")?;
                    } else {
                        writeln!(
                            writer,
                            "  <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{}\" fill=\"{}\"{}//>",
                            x, y, radius, fill, id_attr
                        )?;
                    }
                }
            }
        }

        // Write SVG footer
        writeln!(writer, "</svg>")?;

        Ok(())
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Escape XML special characters for use in SVG
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
