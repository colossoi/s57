# S-57 Electronic Navigational Chart Parser

A comprehensive Rust implementation for parsing, interpreting, and rendering S-57 Electronic Navigational Charts (ENCs). This project provides low-level ISO 8211 parsing, high-level semantic interpretation with topology resolution, and SVG rendering capabilities.

## What is S-57?

S-57 is the IHO (International Hydrographic Organization) standard for digital hydrographic data exchange, used worldwide for electronic nautical charts. S-57 files encode maritime features (coastlines, depth contours, buoys, bridges, wrecks, etc.) along with their spatial geometry and semantic attributes.

## Features

- **Complete ISO 8211 Parser**: Binary format parsing with full field/subfield extraction
- **S-57 DDR Schema Decoder**: Handles Data Descriptive Records with format overrides
- **Entity Component System**: Efficient ECS architecture for managing vectors and features
- **Topology Traversal System (TTS)**: Resolves complex geometry via edge chains and node connectivity
- **Exact Arithmetic**: Lossless coordinate precision using arbitrary-precision rationals
- **SVG Rendering**: Visualize charts with configurable object class filtering
- **Type-Safe Catalogue**: Complete S-57 object (286 classes) and attribute (483 types) definitions

## Architecture

This project is organized as a Cargo workspace with four crates:

```
s57/
├── s57-parse/       # Low-level ISO 8211 & S-57 binary parsing
├── s57-interp/      # ECS-based semantic interpretation & topology
├── s57-catalogue/   # S-57 object/attribute type definitions
└── s57-cli/         # Command-line interface & rendering
```

### Data Flow

```
S-57 Binary File
      ↓
  [s57-parse]  Parse ISO 8211 records & DDR schema
      ↓
  [s57-interp] Build ECS World with vectors & features
      ↓
  [s57-interp] Resolve topology (edge chains, boundaries)
      ↓
  [s57-cli]    Query, filter, and render to SVG or text
```

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Build from Source

```bash
git clone <repository-url>
cd s57
cargo build --release
```

The compiled binary will be at `target/release/s57-cli`.

## Usage

### Basic Commands

```bash
# Show file information
s57-cli info <file.000>

# List all features in the chart
s57-cli list-features <file.000>

# Show detailed information for a specific feature
s57-cli show-object <file.000> <RCID>

# Render chart to SVG (all default object classes)
s57-cli render -o output.svg <file.000>

# Render specific object classes only
s57-cli render -o output.svg --classes COALNE,DEPARE,LIGHTS <file.000>

# Render with custom dimensions
s57-cli render -o output.svg --width 2400 --height 1600 <file.000>

# Print records in YAML format
s57-cli print <file.000>

# Print specific record
s57-cli print --record 5 <file.000>

# Print records as hex dump
s57-cli print --format hex <file.000>
```

### Example: Exploring a Chart

```bash
# 1. Get overview of the chart
s57-cli info test_data/ENC_ROOT/US5PVDGD/US5PVDGD.000

# 2. List all features to find interesting objects
s57-cli list-features test_data/ENC_ROOT/US5PVDGD/US5PVDGD.000

# 3. Examine a specific feature in detail
s57-cli show-object test_data/ENC_ROOT/US5PVDGD/US5PVDGD.000 42

# 4. Render to SVG with coastlines, depth areas, and lights
s57-cli render \
  -o chart.svg \
  --classes COALNE,DEPARE,DEPCNT,LIGHTS,BCNCAR,BOYLAT \
  --width 1920 \
  --height 1080 \
  test_data/ENC_ROOT/US5PVDGD/US5PVDGD.000
```

## Technical Details

### S-57 Data Model

S-57 files consist of:
- **Vectors**: Spatial primitives (isolated nodes, connected nodes, edges)
- **Features**: Semantic chart objects (buoys, coastlines, depth contours, etc.)
- **Topology**: Relationships between vectors (edge chains, area boundaries)

### Topology Traversal System (TTS)

The TTS resolves renderable geometry from topology:

1. **Direct Geometry**: Vectors with SG2D/SG3D fields have explicit coordinates
2. **Topology-Derived Geometry**: Edges reference connected nodes via VRPT fields
3. **Edge Chains**: Features reference multiple edges via FSPT fields
4. **Boundary Resolution**: Area features have complex boundaries with interior/exterior rings

The `EdgeWalker` recursively follows topology chains with configurable policies:
- **Cycle Policy**: Error, allow once, or allow N visits (for complex boundaries)
- **Continuity Policy**: Error or insert gap markers when edges don't connect

### Entity Component System

The ECS design separates data (components) from behavior (systems):

**Components**:
- `VectorMeta` - Vector metadata (rcnm, rcid, version)
- `VectorTopology` - Neighboring vectors with orientation/usage flags
- `FeatureMeta` - Feature metadata (primitive type, object class)
- `FeaturePointers` - Spatial and feature-to-feature relationships
- `ExactPositions` - Coordinate data (BigRational for precision)

**Systems**:
- `NameDecodeSystem` - Process VRID records → Vector entities
- `GeometrySystem` - Process SG2D/SG3D → Coordinates
- `TopologySystem` - Process VRPT → Topology links
- `FoidDecodeSystem` - Process FRID/FOID → Feature entities
- `FeatureBindSystem` - Process FSPT/FFPT → Relationships

### Object Class Filtering

The renderer supports filtering by S-57 object class codes. Default classes include:

- **Coastlines & Land**: COALNE, LNDARE
- **Depth**: DEPARE, DEPCNT, SEAARE
- **Navigation Aids**: LIGHTS, BCNCAR, BCNLAT, BOYCAR, BOYLAT
- **Structures**: BRIDGE, BUISGL, LNDMRK
- **Hazards**: WRECKS, OBSTRN
- **Areas**: ACHARE, RESARE, FAIRWY, HRBARE

See the [S-57 Object Catalogue](http://www.s-57.com/) for complete object definitions.

## Project Status

This implementation handles:
- ✅ ISO 8211 binary format parsing
- ✅ S-57 DDR schema interpretation
- ✅ Vector and feature record extraction
- ✅ 2D coordinate geometry (SG2D)
- ✅ 3D geometry with depth (SG3D)
- ✅ Topology resolution via VRPT/FSPT
- ✅ Complex area boundaries with interior rings
- ✅ SVG rendering with object filtering
- ✅ Exact arithmetic for lossless precision

## Development

### Running Tests

```bash
cargo test --workspace
```

### Enabling Debug Logging

```bash
RUST_LOG=debug s57-cli info <file.000>
RUST_LOG=trace s57-cli render -o output.svg <file.000>
```

### Code Generation

The S-57 catalogue is generated from authoritative CSVs:

```bash
# Requires nushell (https://www.nushell.sh/)
cd scripts
nu generate_catalogue.nu
```

## References

- [IHO S-57 Edition 3.1 Standard](https://iho.int/en/s-57-edition-31)
- [ISO/IEC 8211 Specification](https://www.iso.org/standard/15673.html)
- [OpenCPN S-57 Documentation](https://opencpn.org/)
- [GDAL S-57 Driver](https://gdal.org/drivers/vector/s57.html)

## License

[Add your license here]

## Contributing

Contributions welcome! Please open an issue or pull request for:
- Bug fixes
- Performance improvements
- Additional S-57 feature support
- Documentation improvements
- Test coverage expansion
