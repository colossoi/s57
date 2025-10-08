//! Semantic interpretation helpers for S-57 data

/// Parse and interpret the 0001 field control field
/// In DDR: contains field definition metadata (text format describing field structure)
/// In DR (S-57 specific): contains a sequence number (1 byte) + reserved byte
/// Returns None for DDR (text), Some((seq, reserved)) for DR (binary)
pub fn parse_field_control(data: &[u8]) -> Option<(u8, u8)> {
    // If data is long and looks like ASCII text (DDR), return None
    // DDR 0001 contains things like "0500;&   ISO/IEC 8211 Record Identifier"
    if data.len() > 10 && data[0] >= 0x20 && data[0] <= 0x7E {
        return None;
    }

    // DR 0001 is typically 3 bytes: sequence_number + reserved + field_terminator (0x1E)
    // Strip field terminator and parse the first two bytes
    let clean_data = if !data.is_empty() && data[data.len() - 1] == 0x1E {
        &data[..data.len() - 1]
    } else {
        data
    };

    if clean_data.len() >= 2 {
        Some((clean_data[0], clean_data[1]))
    } else if clean_data.len() == 1 {
        Some((clean_data[0], 0))
    } else {
        None
    }
}

/// Interpret a field tag
pub fn interpret_field_tag(tag: &str) -> &'static str {
    match tag {
        "0000" | "0001" => "Field control field",
        "DSID" => "Data Set Identification",
        "DSSI" => "Data Set Structure Information",
        "DSPM" => "Data Set Parameter",
        "FRID" => "Feature Record Identifier",
        "FOID" => "Feature Object Identifier",
        "ATTF" => "Feature Record Attribute",
        "NATF" => "Feature Record National Attribute",
        "FFPC" => "Feature Record to Feature Object Pointer Control",
        "FFPT" => "Feature Record to Feature Object Pointer",
        "FSPC" => "Feature Record to Spatial Record Pointer Control",
        "FSPT" => "Feature Record to Spatial Record Pointer",
        "VRID" => "Vector Record Identifier",
        "ATTV" => "Vector Record Attribute",
        "VRPC" => "Vector Record Pointer Control",
        "VRPT" => "Vector Record Pointer",
        "SG2D" => "2D Coordinate (Geometry)",
        "SG3D" => "3D Coordinate (Geometry)",
        _ => "Unknown field",
    }
}

/// Interpret object label code
pub fn interpret_object_label(objl: u16) -> &'static str {
    match objl {
        1 => "ADMARE (Administration area)",
        2 => "AIRARE (Airport/airfield)",
        3 => "ACHBRT (Anchor berth)",
        4 => "ACHARE (Anchorage area)",
        5 => "BCNCAR (Beacon, cardinal)",
        6 => "BCNISD (Beacon, isolated danger)",
        7 => "BCNLAT (Beacon, lateral)",
        8 => "BCNSAW (Beacon, safe water)",
        9 => "BCNSPP (Beacon, special purpose/general)",
        10 => "BERTHS (Berth)",
        11 => "BRIDGE (Bridge)",
        12 => "BUISGL (Building, single)",
        13 => "BUAARE (Built-up area)",
        14 => "BOYCAR (Buoy, cardinal)",
        15 => "BOYINB (Buoy, installation)",
        16 => "BOYISD (Buoy, isolated danger)",
        17 => "BOYLAT (Buoy, lateral)",
        18 => "BOYSAW (Buoy, safe water)",
        19 => "BOYSPP (Buoy, special purpose/general)",
        20 => "CBLARE (Cable area)",
        21 => "CBLOHD (Cable, overhead)",
        22 => "CBLSUB (Cable, submarine)",
        23 => "CANALS (Canal)",
        24 => "CTSARE (Cargo transhipment area)",
        25 => "CAUSWY (Causeway)",
        26 => "CTNARE (Caution area)",
        27 => "CHKPNT (Checkpoint)",
        28 => "CGUSTA (Coastguard station)",
        29 => "COALNE (Coastline)",
        30 => "CONZNE (Contiguous zone)",
        31 => "COSARE (Continental shelf area)",
        32 => "CTRPNT (Control point)",
        33 => "CONVYR (Conveyor)",
        34 => "CRANES (Crane)",
        35 => "CURENT (Current - non-gravitational)",
        36 => "CUSZNE (Custom zone)",
        37 => "DAMCON (Dam)",
        38 => "DAYMAR (Daymark)",
        39 => "DWRTCL (Deep water route centerline)",
        40 => "DWRTPT (Deep water route part)",
        41 => "DEPARE (Depth area)",
        42 => "DEPCNT (Depth contour)",
        43 => "DISMAR (Distance mark)",
        44 => "DOCARE (Dock area)",
        45 => "DRGARE (Dredged area)",
        46 => "DRYDOC (Dry dock)",
        47 => "DMPGRD (Dumping ground)",
        48 => "DWKARE (Deep water route)",
        49 => "CKYARD (Checkpoint, yard)",
        50 => "EDFWTR (Eddy/foul water)",
        51 => "EXEZNE (Exclusive Economic Zone)",
        52 => "FAIRWY (Fairway)",
        53 => "FNCLNE (Fence/wall)",
        54 => "FERYRT (Ferry route)",
        55 => "FSHZNE (Fishery zone)",
        56 => "FSHFAC (Fishing facility)",
        57 => "FSHGRD (Fishing ground)",
        58 => "FLODOC (Floating dock)",
        59 => "FOGSIG (Fog signal)",
        60 => "FORSTC (Fortified structure)",
        61 => "FRPARE (Free port area)",
        62 => "GATCON (Gate)",
        63 => "GRIDRN (Gridiron)",
        64 => "HRBARE (Harbour area)",
        65 => "HRBFAC (Harbour facility)",
        66 => "HULKES (Hulk)",
        67 => "ICEARE (Ice area)",
        68 => "ICNARE (Incineration area)",
        69 => "ISTZNE (Inshore traffic zone)",
        70 => "LAKARE (Lake)",
        71 => "LNDARE (Land area)",
        72 => "LNDELV (Land elevation)",
        73 => "LNDRGN (Land region)",
        74 => "LNDMRK (Landmark)",
        75 => "LIGHTS (Light)",
        76 => "LITFLT (Light float)",
        77 => "LITVES (Light vessel)",
        78 => "LOCMAG (Local magnetic anomaly)",
        79 => "LOKBSN (Lock basin)",
        80 => "LOGPON (Log pond)",
        81 => "MAGVAR (Magnetic variation)",
        82 => "MARCUL (Marine farm/culture)",
        83 => "MIPARE (Military practice area)",
        84 => "MORFAC (Mooring/warping facility)",
        85 => "NAVLNE (Navigation line)",
        86 => "OBSTRN (Obstruction)",
        87 => "OFSPLF (Offshore platform)",
        88 => "OSPARE (Offshore production area)",
        89 => "OILBAR (Oil barrier)",
        90 => "PILPNT (Pile)",
        91 => "PILBOP (Pilot boarding place)",
        92 => "PIPARE (Pipeline area)",
        93 => "PIPOHD (Pipeline, overhead)",
        94 => "PIPSOL (Pipeline, submarine/on land)",
        95 => "PONTON (Pontoon)",
        96 => "PRCARE (Precautionary area)",
        97 => "PRDARE (Production/storage area)",
        98 => "PYLONS (Pylon/bridge support)",
        99 => "RADLNE (Radar line)",
        100 => "RADRNG (Radar range)",
        101 => "RADRFL (Radar reflector)",
        102 => "RADSTA (Radio station)",
        103 => "RTPBCN (Radar transponder beacon)",
        104 => "RCTLPT (Recommended track)",
        105 => "RECTRC (Recommended traffic lane)",
        106 => "REFDMP (Refuse dump)",
        107 => "RSCSTA (Rescue station)",
        108 => "RESARE (Restricted area)",
        109 => "RETRFL (Retro-reflector)",
        110 => "RIVERS (River)",
        111 => "ROADWY (Road)",
        112 => "RUNWAY (Runway)",
        113 => "SNDWAV (Sand waves)",
        114 => "SEAARE (Sea area/named water area)",
        115 => "SPLARE (Sea-plane landing area)",
        116 => "SBDARE (Seabed area)",
        117 => "SLCONS (Shoreline construction)",
        118 => "SISTAT (Signal station, traffic)",
        119 => "SISTAW (Signal station, warning)",
        120 => "SILTNK (Silo/tank)",
        121 => "SLOTOP (Slope topline)",
        122 => "SLOGRD (Sloping ground)",
        123 => "SMCFAC (Small craft facility)",
        124 => "SOUNDG (Sounding)",
        125 => "SPRING (Spring)",
        126 => "STSLNE (Straight territorial sea baseline)",
        127 => "SUBTLN (Submarine transit lane)",
        128 => "SWPARE (Swept area)",
        129 => "TESARE (Territorial sea area)",
        130 => "TS_PRH (Tidal stream - harmonic prediction)",
        131 => "TS_PNH (Tidal stream - non-harmonic prediction)",
        132 => "TS_PAD (Tidal stream panel data)",
        133 => "TS_TIS (Tidal stream - time series)",
        134 => "T_HMON (Tide - harmonic prediction)",
        135 => "T_NHMN (Tide - non-harmonic prediction)",
        136 => "T_TIMS (Tide - time series)",
        137 => "TIDEWY (Tideway)",
        138 => "TOPMAR (Topmark)",
        139 => "TSELNE (Traffic separation line)",
        140 => "TSSBND (Traffic separation scheme boundary)",
        141 => "TSSCRS (Traffic separation scheme crossing)",
        142 => "TSSLPT (Traffic separation scheme lane part)",
        143 => "TSSRON (Traffic separation scheme roundabout)",
        144 => "TSEZNE (Traffic separation zone)",
        145 => "TUNNEL (Tunnel)",
        146 => "TWRTPT (Two-way route part)",
        147 => "UWTROC (Underwater rock/awash rock)",
        148 => "UNSARE (Unsurveyed area)",
        149 => "VEGATN (Vegetation)",
        150 => "WATTUR (Water turbulence)",
        151 => "WATFAL (Waterfall)",
        152 => "WEDKLP (Weed/kelp)",
        153 => "WRECKS (Wreck)",
        154 => "TS_FEB (Tidal stream - flood/ebb)",
        155 => "M_ACCY (Accuracy of data)",
        156 => "M_CSCL (Compilation scale of data)",
        157 => "M_COVR (Coverage)",
        158 => "M_HDAT (Horizontal datum of data)",
        159 => "M_HOPA (Horizontal datum shift parameters)",
        160 => "M_NPUB (Nautical publication information)",
        161 => "M_NSYS (Navigational system of marks)",
        162 => "M_PROD (Production information)",
        163 => "M_QUAL (Quality of data)",
        164 => "M_SDAT (Sounding datum)",
        165 => "M_SREL (Survey reliability)",
        166 => "M_UNIT (Units of measurement of data)",
        167 => "M_VDAT (Vertical datum of data)",
        168 => "C_AGGR (Aggregation)",
        169 => "C_ASSO (Association)",
        170 => "C_STAC (Stacked on/stacked under)",
        300 => "$AREAS (Cartographic area)",
        301 => "$LINES (Cartographic line)",
        302 => "$CSYMB (Cartographic symbol)",
        303 => "$COMPS (Compass)",
        304 => "$TEXTS (Text)",
        _ => "Unknown object",
    }
}

/// Interpret primitive type
pub fn interpret_primitive(prim: u8) -> &'static str {
    match prim {
        1 => "Point",
        2 => "Line",
        3 => "Area",
        255 => "None (non-spatial)",
        _ => "Unknown primitive",
    }
}

/// Interpret record name (RCNM)
pub fn interpret_record_name(rcnm: u8) -> &'static str {
    match rcnm {
        10 => "Data Set General Information (DS)",
        20 => "Catalogue Directory (CD)",
        30 => "Catalogue Cross Reference (CR)",
        40 => "Data Dictionary Definition (DD)",
        50 => "Data Dictionary Domain (DM)",
        60 => "Data Dictionary Schema (DS)",
        100 => "Feature (FE)",
        110 => "Isolated Node (VI)",
        120 => "Connected Node (VC)",
        130 => "Edge (VE)",
        140 => "Face (VF)",
        _ => "Unknown record type",
    }
}

/// Interpret update instruction (RUIN)
pub fn interpret_update_instruction(ruin: u8) -> &'static str {
    match ruin {
        1 => "Insert",
        2 => "Delete",
        3 => "Modify",
        _ => "Unknown instruction",
    }
}

/// Interpret orientation (ORNT)
pub fn interpret_orientation(ornt: u8) -> &'static str {
    match ornt {
        1 => "Forward",
        2 => "Reverse",
        255 => "N/A",
        _ => "Unknown orientation",
    }
}
