// Generated from s57attributes.csv
// DO NOT EDIT - run scripts/generate_catalogue.nu to regenerate

#![allow(unreachable_patterns)]

/// S-57 Attribute with code and name
///
/// Complete catalogue of IHO S-57 attributes from GDAL reference.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeInfo {
    pub code: u16,
    pub acronym: &'static str,
    pub name: &'static str,
}

impl AttributeInfo {
    /// Get attribute info by code
    pub fn from_code(code: u16) -> Option<Self> {
        match code {
            1 => Some(Self {
                code: 1,
                acronym: "AGENCY",
                name: "Agency responsible for production",
            }),
            2 => Some(Self {
                code: 2,
                acronym: "BCNSHP",
                name: "Beacon shape",
            }),
            3 => Some(Self {
                code: 3,
                acronym: "BUISHP",
                name: "Building shape",
            }),
            4 => Some(Self {
                code: 4,
                acronym: "BOYSHP",
                name: "Buoy shape",
            }),
            5 => Some(Self {
                code: 5,
                acronym: "BURDEP",
                name: "Buried depth",
            }),
            6 => Some(Self {
                code: 6,
                acronym: "CALSGN",
                name: "Call sign",
            }),
            7 => Some(Self {
                code: 7,
                acronym: "CATAIR",
                name: "Category of airport/airfield",
            }),
            8 => Some(Self {
                code: 8,
                acronym: "CATACH",
                name: "Category of anchorage",
            }),
            9 => Some(Self {
                code: 9,
                acronym: "CATBRG",
                name: "Category of bridge",
            }),
            10 => Some(Self {
                code: 10,
                acronym: "CATBUA",
                name: "Category of built-up area",
            }),
            11 => Some(Self {
                code: 11,
                acronym: "CATCBL",
                name: "Category of cable",
            }),
            12 => Some(Self {
                code: 12,
                acronym: "CATCAN",
                name: "Category of canal",
            }),
            13 => Some(Self {
                code: 13,
                acronym: "CATCAM",
                name: "Category of cardinal mark",
            }),
            14 => Some(Self {
                code: 14,
                acronym: "CATCHP",
                name: "Category of checkpoint",
            }),
            15 => Some(Self {
                code: 15,
                acronym: "CATCOA",
                name: "Category of coastline",
            }),
            16 => Some(Self {
                code: 16,
                acronym: "CATCTR",
                name: "Category of control point",
            }),
            17 => Some(Self {
                code: 17,
                acronym: "CATCON",
                name: "Category of conveyor",
            }),
            18 => Some(Self {
                code: 18,
                acronym: "CATCOV",
                name: "Category of coverage",
            }),
            19 => Some(Self {
                code: 19,
                acronym: "CATCRN",
                name: "Category of crane",
            }),
            20 => Some(Self {
                code: 20,
                acronym: "CATDAM",
                name: "Category of dam",
            }),
            21 => Some(Self {
                code: 21,
                acronym: "CATDIS",
                name: "Category of distance mark",
            }),
            22 => Some(Self {
                code: 22,
                acronym: "CATDOC",
                name: "Category of dock",
            }),
            23 => Some(Self {
                code: 23,
                acronym: "CATDPG",
                name: "Category of dumping ground",
            }),
            24 => Some(Self {
                code: 24,
                acronym: "CATFNC",
                name: "Category of fence/wall",
            }),
            25 => Some(Self {
                code: 25,
                acronym: "CATFRY",
                name: "Category of ferry",
            }),
            26 => Some(Self {
                code: 26,
                acronym: "CATFIF",
                name: "Category of fishing  facility",
            }),
            27 => Some(Self {
                code: 27,
                acronym: "CATFOG",
                name: "Category of fog signal",
            }),
            28 => Some(Self {
                code: 28,
                acronym: "CATFOR",
                name: "Category of fortified structure",
            }),
            29 => Some(Self {
                code: 29,
                acronym: "CATGAT",
                name: "Category of gate",
            }),
            30 => Some(Self {
                code: 30,
                acronym: "CATHAF",
                name: "Category of harbour facility",
            }),
            31 => Some(Self {
                code: 31,
                acronym: "CATHLK",
                name: "Category of hulk",
            }),
            32 => Some(Self {
                code: 32,
                acronym: "CATICE",
                name: "Category of ice",
            }),
            33 => Some(Self {
                code: 33,
                acronym: "CATINB",
                name: "Category of installation buoy",
            }),
            34 => Some(Self {
                code: 34,
                acronym: "CATLND",
                name: "Category of land region",
            }),
            35 => Some(Self {
                code: 35,
                acronym: "CATLMK",
                name: "Category of landmark",
            }),
            36 => Some(Self {
                code: 36,
                acronym: "CATLAM",
                name: "Category of lateral mark",
            }),
            37 => Some(Self {
                code: 37,
                acronym: "CATLIT",
                name: "Category of light",
            }),
            38 => Some(Self {
                code: 38,
                acronym: "CATMFA",
                name: "Category of marine farm/culture",
            }),
            39 => Some(Self {
                code: 39,
                acronym: "CATMPA",
                name: "Category of military practice area",
            }),
            40 => Some(Self {
                code: 40,
                acronym: "CATMOR",
                name: "Category of mooring/warping facility",
            }),
            41 => Some(Self {
                code: 41,
                acronym: "CATNAV",
                name: "Category of navigation line",
            }),
            42 => Some(Self {
                code: 42,
                acronym: "CATOBS",
                name: "Category of obstruction",
            }),
            43 => Some(Self {
                code: 43,
                acronym: "CATOFP",
                name: "Category of offshore platform",
            }),
            44 => Some(Self {
                code: 44,
                acronym: "CATOLB",
                name: "Category of oil barrier",
            }),
            45 => Some(Self {
                code: 45,
                acronym: "CATPLE",
                name: "Category of pile",
            }),
            46 => Some(Self {
                code: 46,
                acronym: "CATPIL",
                name: "Category of pilot boarding place",
            }),
            47 => Some(Self {
                code: 47,
                acronym: "CATPIP",
                name: "Category of pipeline / pipe",
            }),
            48 => Some(Self {
                code: 48,
                acronym: "CATPRA",
                name: "Category of production area",
            }),
            49 => Some(Self {
                code: 49,
                acronym: "CATPYL",
                name: "Category of pylon",
            }),
            50 => Some(Self {
                code: 50,
                acronym: "CATQUA",
                name: "Category of quality of data",
            }),
            51 => Some(Self {
                code: 51,
                acronym: "CATRAS",
                name: "Category of radar station",
            }),
            52 => Some(Self {
                code: 52,
                acronym: "CATRTB",
                name: "Category of radar transponder beacon",
            }),
            53 => Some(Self {
                code: 53,
                acronym: "CATROS",
                name: "Category of radio station",
            }),
            54 => Some(Self {
                code: 54,
                acronym: "CATTRK",
                name: "Category of recommended track",
            }),
            55 => Some(Self {
                code: 55,
                acronym: "CATRSC",
                name: "Category of rescue station",
            }),
            56 => Some(Self {
                code: 56,
                acronym: "CATREA",
                name: "Category of restricted area",
            }),
            57 => Some(Self {
                code: 57,
                acronym: "CATROD",
                name: "Category of road",
            }),
            58 => Some(Self {
                code: 58,
                acronym: "CATRUN",
                name: "Category of runway",
            }),
            59 => Some(Self {
                code: 59,
                acronym: "CATSEA",
                name: "Category of sea area",
            }),
            60 => Some(Self {
                code: 60,
                acronym: "CATSLC",
                name: "Category of shoreline construction",
            }),
            61 => Some(Self {
                code: 61,
                acronym: "CATSIT",
                name: "Category of signal station, traffic",
            }),
            62 => Some(Self {
                code: 62,
                acronym: "CATSIW",
                name: "Category of signal station, warning",
            }),
            63 => Some(Self {
                code: 63,
                acronym: "CATSIL",
                name: "Category of silo/tank",
            }),
            64 => Some(Self {
                code: 64,
                acronym: "CATSLO",
                name: "Category of slope",
            }),
            65 => Some(Self {
                code: 65,
                acronym: "CATSCF",
                name: "Category of small craft facility",
            }),
            66 => Some(Self {
                code: 66,
                acronym: "CATSPM",
                name: "Category of special purpose mark",
            }),
            67 => Some(Self {
                code: 67,
                acronym: "CATTSS",
                name: "Category of Traffic Separation Scheme",
            }),
            68 => Some(Self {
                code: 68,
                acronym: "CATVEG",
                name: "Category of vegetation",
            }),
            69 => Some(Self {
                code: 69,
                acronym: "CATWAT",
                name: "Category of water turbulence",
            }),
            70 => Some(Self {
                code: 70,
                acronym: "CATWED",
                name: "Category of weed/kelp",
            }),
            71 => Some(Self {
                code: 71,
                acronym: "CATWRK",
                name: "Category of wreck",
            }),
            72 => Some(Self {
                code: 72,
                acronym: "CATZOC",
                name: "Category of zone of confidence data",
            }),
            73 => Some(Self {
                code: 73,
                acronym: "$SPACE",
                name: "Character spacing",
            }),
            74 => Some(Self {
                code: 74,
                acronym: "$CHARS",
                name: "Character specification",
            }),
            75 => Some(Self {
                code: 75,
                acronym: "COLOUR",
                name: "Colour",
            }),
            76 => Some(Self {
                code: 76,
                acronym: "COLPAT",
                name: "Colour pattern",
            }),
            77 => Some(Self {
                code: 77,
                acronym: "COMCHA",
                name: "Communication channel",
            }),
            78 => Some(Self {
                code: 78,
                acronym: "$CSIZE",
                name: "Compass size",
            }),
            79 => Some(Self {
                code: 79,
                acronym: "CPDATE",
                name: "Compilation date",
            }),
            80 => Some(Self {
                code: 80,
                acronym: "CSCALE",
                name: "Compilation scale",
            }),
            81 => Some(Self {
                code: 81,
                acronym: "CONDTN",
                name: "Condition",
            }),
            82 => Some(Self {
                code: 82,
                acronym: "CONRAD",
                name: "Conspicuous, Radar",
            }),
            83 => Some(Self {
                code: 83,
                acronym: "CONVIS",
                name: "Conspicuous, visual",
            }),
            84 => Some(Self {
                code: 84,
                acronym: "CURVEL",
                name: "Current velocity",
            }),
            85 => Some(Self {
                code: 85,
                acronym: "DATEND",
                name: "Date end",
            }),
            86 => Some(Self {
                code: 86,
                acronym: "DATSTA",
                name: "Date start",
            }),
            87 => Some(Self {
                code: 87,
                acronym: "DRVAL1",
                name: "Depth range value 1",
            }),
            88 => Some(Self {
                code: 88,
                acronym: "DRVAL2",
                name: "Depth range value 2",
            }),
            89 => Some(Self {
                code: 89,
                acronym: "DUNITS",
                name: "Depth units",
            }),
            90 => Some(Self {
                code: 90,
                acronym: "ELEVAT",
                name: "Elevation",
            }),
            91 => Some(Self {
                code: 91,
                acronym: "ESTRNG",
                name: "Estimated range of transmission",
            }),
            92 => Some(Self {
                code: 92,
                acronym: "EXCLIT",
                name: "Exhibition condition of light",
            }),
            93 => Some(Self {
                code: 93,
                acronym: "EXPSOU",
                name: "Exposition of sounding",
            }),
            94 => Some(Self {
                code: 94,
                acronym: "FUNCTN",
                name: "Function",
            }),
            95 => Some(Self {
                code: 95,
                acronym: "HEIGHT",
                name: "Height",
            }),
            96 => Some(Self {
                code: 96,
                acronym: "HUNITS",
                name: "Height/length units",
            }),
            97 => Some(Self {
                code: 97,
                acronym: "HORACC",
                name: "Horizontal accuracy",
            }),
            98 => Some(Self {
                code: 98,
                acronym: "HORCLR",
                name: "Horizontal clearance",
            }),
            99 => Some(Self {
                code: 99,
                acronym: "HORLEN",
                name: "Horizontal length",
            }),
            100 => Some(Self {
                code: 100,
                acronym: "HORWID",
                name: "Horizontal width",
            }),
            101 => Some(Self {
                code: 101,
                acronym: "ICEFAC",
                name: "Ice factor",
            }),
            102 => Some(Self {
                code: 102,
                acronym: "INFORM",
                name: "Information",
            }),
            103 => Some(Self {
                code: 103,
                acronym: "JRSDTN",
                name: "Jurisdiction",
            }),
            104 => Some(Self {
                code: 104,
                acronym: "$JUSTH",
                name: "Justification - horizontal",
            }),
            105 => Some(Self {
                code: 105,
                acronym: "$JUSTV",
                name: "Justification - vertical",
            }),
            106 => Some(Self {
                code: 106,
                acronym: "LIFCAP",
                name: "Lifting capacity",
            }),
            107 => Some(Self {
                code: 107,
                acronym: "LITCHR",
                name: "Light characteristic",
            }),
            108 => Some(Self {
                code: 108,
                acronym: "LITVIS",
                name: "Light visibility",
            }),
            109 => Some(Self {
                code: 109,
                acronym: "MARSYS",
                name: "Marks navigational - System of",
            }),
            110 => Some(Self {
                code: 110,
                acronym: "MLTYLT",
                name: "Multiplicity of lights",
            }),
            111 => Some(Self {
                code: 111,
                acronym: "NATION",
                name: "Nationality",
            }),
            112 => Some(Self {
                code: 112,
                acronym: "NATCON",
                name: "Nature of construction",
            }),
            113 => Some(Self {
                code: 113,
                acronym: "NATSUR",
                name: "Nature of surface",
            }),
            114 => Some(Self {
                code: 114,
                acronym: "NATQUA",
                name: "Nature of surface - qualifying terms",
            }),
            115 => Some(Self {
                code: 115,
                acronym: "NMDATE",
                name: "Notice to Mariners date",
            }),
            116 => Some(Self {
                code: 116,
                acronym: "OBJNAM",
                name: "Object name",
            }),
            117 => Some(Self {
                code: 117,
                acronym: "ORIENT",
                name: "Orientation",
            }),
            118 => Some(Self {
                code: 118,
                acronym: "PEREND",
                name: "Periodic date end",
            }),
            119 => Some(Self {
                code: 119,
                acronym: "PERSTA",
                name: "Periodic date start",
            }),
            120 => Some(Self {
                code: 120,
                acronym: "PICREP",
                name: "Pictorial representation",
            }),
            121 => Some(Self {
                code: 121,
                acronym: "PILDST",
                name: "Pilot district",
            }),
            122 => Some(Self {
                code: 122,
                acronym: "PRCTRY",
                name: "Producing country",
            }),
            123 => Some(Self {
                code: 123,
                acronym: "PRODCT",
                name: "Product",
            }),
            124 => Some(Self {
                code: 124,
                acronym: "PUBREF",
                name: "Publication reference",
            }),
            125 => Some(Self {
                code: 125,
                acronym: "QUASOU",
                name: "Quality of sounding measurement",
            }),
            126 => Some(Self {
                code: 126,
                acronym: "RADWAL",
                name: "Radar wave length",
            }),
            127 => Some(Self {
                code: 127,
                acronym: "RADIUS",
                name: "Radius",
            }),
            128 => Some(Self {
                code: 128,
                acronym: "RECDAT",
                name: "Recording date",
            }),
            129 => Some(Self {
                code: 129,
                acronym: "RECIND",
                name: "Recording indication",
            }),
            130 => Some(Self {
                code: 130,
                acronym: "RYRMGV",
                name: "Reference year for magnetic variation",
            }),
            131 => Some(Self {
                code: 131,
                acronym: "RESTRN",
                name: "Restriction",
            }),
            132 => Some(Self {
                code: 132,
                acronym: "SCAMAX",
                name: "Scale maximum",
            }),
            133 => Some(Self {
                code: 133,
                acronym: "SCAMIN",
                name: "Scale minimum",
            }),
            134 => Some(Self {
                code: 134,
                acronym: "SCVAL1",
                name: "Scale value one",
            }),
            135 => Some(Self {
                code: 135,
                acronym: "SCVAL2",
                name: "Scale value two",
            }),
            136 => Some(Self {
                code: 136,
                acronym: "SECTR1",
                name: "Sector limit one",
            }),
            137 => Some(Self {
                code: 137,
                acronym: "SECTR2",
                name: "Sector limit two",
            }),
            138 => Some(Self {
                code: 138,
                acronym: "SHIPAM",
                name: "Shift parameters",
            }),
            139 => Some(Self {
                code: 139,
                acronym: "SIGFRQ",
                name: "Signal frequency",
            }),
            140 => Some(Self {
                code: 140,
                acronym: "SIGGEN",
                name: "Signal generation",
            }),
            141 => Some(Self {
                code: 141,
                acronym: "SIGGRP",
                name: "Signal group",
            }),
            142 => Some(Self {
                code: 142,
                acronym: "SIGPER",
                name: "Signal period",
            }),
            143 => Some(Self {
                code: 143,
                acronym: "SIGSEQ",
                name: "Signal sequence",
            }),
            144 => Some(Self {
                code: 144,
                acronym: "SOUACC",
                name: "Sounding accuracy",
            }),
            145 => Some(Self {
                code: 145,
                acronym: "SDISMX",
                name: "Sounding distance - maximum",
            }),
            146 => Some(Self {
                code: 146,
                acronym: "SDISMN",
                name: "Sounding distance - minimum",
            }),
            147 => Some(Self {
                code: 147,
                acronym: "SORDAT",
                name: "Source date",
            }),
            148 => Some(Self {
                code: 148,
                acronym: "SORIND",
                name: "Source indication",
            }),
            149 => Some(Self {
                code: 149,
                acronym: "STATUS",
                name: "Status",
            }),
            150 => Some(Self {
                code: 150,
                acronym: "SURATH",
                name: "Survey authority",
            }),
            151 => Some(Self {
                code: 151,
                acronym: "SUREND",
                name: "Survey date - end",
            }),
            152 => Some(Self {
                code: 152,
                acronym: "SURSTA",
                name: "Survey date - start",
            }),
            153 => Some(Self {
                code: 153,
                acronym: "SURTYP",
                name: "Survey type",
            }),
            154 => Some(Self {
                code: 154,
                acronym: "$SCALE",
                name: "Symbol scaling factor",
            }),
            155 => Some(Self {
                code: 155,
                acronym: "$SCODE",
                name: "Symbolization code",
            }),
            156 => Some(Self {
                code: 156,
                acronym: "TECSOU",
                name: "Technique of sounding measurement",
            }),
            157 => Some(Self {
                code: 157,
                acronym: "$TXSTR",
                name: "Text string",
            }),
            158 => Some(Self {
                code: 158,
                acronym: "TXTDSC",
                name: "Textual description",
            }),
            159 => Some(Self {
                code: 159,
                acronym: "TS_TSP",
                name: "Tidal stream - panel values",
            }),
            160 => Some(Self {
                code: 160,
                acronym: "TS_TSV",
                name: "Tidal stream, current - time series values",
            }),
            161 => Some(Self {
                code: 161,
                acronym: "T_ACWL",
                name: "Tide - accuracy of water level",
            }),
            162 => Some(Self {
                code: 162,
                acronym: "T_HWLW",
                name: "Tide - high and low water values",
            }),
            163 => Some(Self {
                code: 163,
                acronym: "T_MTOD",
                name: "Tide - method of tidal prediction",
            }),
            164 => Some(Self {
                code: 164,
                acronym: "T_THDF",
                name: "Tide - time and height differences",
            }),
            165 => Some(Self {
                code: 165,
                acronym: "T_TINT",
                name: "Tide, current - time interval of values",
            }),
            166 => Some(Self {
                code: 166,
                acronym: "T_TSVL",
                name: "Tide - time series values",
            }),
            167 => Some(Self {
                code: 167,
                acronym: "T_VAHC",
                name: "Tide - value of harmonic constituents",
            }),
            168 => Some(Self {
                code: 168,
                acronym: "TIMEND",
                name: "Time end",
            }),
            169 => Some(Self {
                code: 169,
                acronym: "TIMSTA",
                name: "Time start",
            }),
            170 => Some(Self {
                code: 170,
                acronym: "$TINTS",
                name: "Tint",
            }),
            171 => Some(Self {
                code: 171,
                acronym: "TOPSHP",
                name: "Topmark/daymark shape",
            }),
            172 => Some(Self {
                code: 172,
                acronym: "TRAFIC",
                name: "Traffic flow",
            }),
            173 => Some(Self {
                code: 173,
                acronym: "VALACM",
                name: "Value of annual change in magnetic variation",
            }),
            174 => Some(Self {
                code: 174,
                acronym: "VALDCO",
                name: "Value of depth contour",
            }),
            175 => Some(Self {
                code: 175,
                acronym: "VALLMA",
                name: "Value of local magnetic anomaly",
            }),
            176 => Some(Self {
                code: 176,
                acronym: "VALMAG",
                name: "Value of magnetic variation",
            }),
            177 => Some(Self {
                code: 177,
                acronym: "VALMXR",
                name: "Value of maximum range",
            }),
            178 => Some(Self {
                code: 178,
                acronym: "VALNMR",
                name: "Value of nominal range",
            }),
            179 => Some(Self {
                code: 179,
                acronym: "VALSOU",
                name: "Value of sounding",
            }),
            180 => Some(Self {
                code: 180,
                acronym: "VERACC",
                name: "Vertical accuracy",
            }),
            181 => Some(Self {
                code: 181,
                acronym: "VERCLR",
                name: "Vertical clearance",
            }),
            182 => Some(Self {
                code: 182,
                acronym: "VERCCL",
                name: "Vertical clearance, closed",
            }),
            183 => Some(Self {
                code: 183,
                acronym: "VERCOP",
                name: "Vertical clearance, open",
            }),
            184 => Some(Self {
                code: 184,
                acronym: "VERCSA",
                name: "Vertical clearance, safe",
            }),
            185 => Some(Self {
                code: 185,
                acronym: "VERDAT",
                name: "Vertical datum",
            }),
            186 => Some(Self {
                code: 186,
                acronym: "VERLEN",
                name: "Vertical length",
            }),
            187 => Some(Self {
                code: 187,
                acronym: "WATLEV",
                name: "Water level effect",
            }),
            188 => Some(Self {
                code: 188,
                acronym: "CAT_TS",
                name: "Category of Tidal stream",
            }),
            189 => Some(Self {
                code: 189,
                acronym: "PUNITS",
                name: "Positional accuracy units",
            }),
            190 => Some(Self {
                code: 190,
                acronym: "CLSDEF",
                name: "Object class definition",
            }),
            191 => Some(Self {
                code: 191,
                acronym: "CLSNAM",
                name: "Object class name",
            }),
            192 => Some(Self {
                code: 192,
                acronym: "SYMINS",
                name: "Symbol instruction",
            }),
            300 => Some(Self {
                code: 300,
                acronym: "NINFOM",
                name: "Information in national language",
            }),
            301 => Some(Self {
                code: 301,
                acronym: "NOBJNM",
                name: "Object name in national language",
            }),
            302 => Some(Self {
                code: 302,
                acronym: "NPLDST",
                name: "Pilot district in national language",
            }),
            303 => Some(Self {
                code: 303,
                acronym: "$NTXST",
                name: "Text string in national language",
            }),
            304 => Some(Self {
                code: 304,
                acronym: "NTXTDS",
                name: "Textual description in national language",
            }),
            400 => Some(Self {
                code: 400,
                acronym: "HORDAT",
                name: "Horizontal datum",
            }),
            401 => Some(Self {
                code: 401,
                acronym: "POSACC",
                name: "Positional Accuracy",
            }),
            402 => Some(Self {
                code: 402,
                acronym: "QUAPOS",
                name: "Quality of position",
            }),
            17000 => Some(Self {
                code: 17000,
                acronym: "catach",
                name: "Category of Anchorage area",
            }),
            17001 => Some(Self {
                code: 17001,
                acronym: "catdis",
                name: "Category of distance mark",
            }),
            17002 => Some(Self {
                code: 17002,
                acronym: "catsit",
                name: "Category of signal station trafficcatsit",
            }),
            17003 => Some(Self {
                code: 17003,
                acronym: "catsiw",
                name: "Category of signal station warning",
            }),
            17004 => Some(Self {
                code: 17004,
                acronym: "restrn",
                name: "Restriction",
            }),
            17005 => Some(Self {
                code: 17005,
                acronym: "verdat",
                name: "Vertical datum",
            }),
            17006 => Some(Self {
                code: 17006,
                acronym: "catbrg",
                name: "Category of bridge",
            }),
            17007 => Some(Self {
                code: 17007,
                acronym: "catfry",
                name: "Category of ferry",
            }),
            17008 => Some(Self {
                code: 17008,
                acronym: "cathaf",
                name: "Category of harbour facilities",
            }),
            17009 => Some(Self {
                code: 17009,
                acronym: "marsys",
                name: "Marks navigational  System of",
            }),
            17050 => Some(Self {
                code: 17050,
                acronym: "addmrk",
                name: "Additional mark",
            }),
            17051 => Some(Self {
                code: 17051,
                acronym: "catbnk",
                name: "Category of bank",
            }),
            17052 => Some(Self {
                code: 17052,
                acronym: "catnmk",
                name: "Category of notice mark",
            }),
            17055 => Some(Self {
                code: 17055,
                acronym: "clsdng",
                name: "Class of dangerous cargo",
            }),
            17056 => Some(Self {
                code: 17056,
                acronym: "dirimp",
                name: "Direction of impact",
            }),
            17057 => Some(Self {
                code: 17057,
                acronym: "disbk1",
                name: "Distance from bank",
            }),
            17058 => Some(Self {
                code: 17058,
                acronym: "disbk2",
                name: "Distance from bank",
            }),
            17059 => Some(Self {
                code: 17059,
                acronym: "disipu",
                name: "Distance of impact, upstream",
            }),
            17060 => Some(Self {
                code: 17060,
                acronym: "disipd",
                name: "Distance of impact, downstream",
            }),
            17061 => Some(Self {
                code: 17061,
                acronym: "eleva1",
                name: "Elevation 1",
            }),
            17062 => Some(Self {
                code: 17062,
                acronym: "eleva2",
                name: "Elevation 2",
            }),
            17063 => Some(Self {
                code: 17063,
                acronym: "fnctnm",
                name: "Function of notice mark",
            }),
            17064 => Some(Self {
                code: 17064,
                acronym: "wtwdis",
                name: "Waterway distance",
            }),
            17065 => Some(Self {
                code: 17065,
                acronym: "bunves",
                name: "Bunker vessel",
            }),
            17066 => Some(Self {
                code: 17066,
                acronym: "catbrt",
                name: "Category of berth",
            }),
            17067 => Some(Self {
                code: 17067,
                acronym: "catbun",
                name: "Category of bunker",
            }),
            17068 => Some(Self {
                code: 17068,
                acronym: "catccl",
                name: "Category of CEMT class",
            }),
            17069 => Some(Self {
                code: 17069,
                acronym: "catcom",
                name: "Category of communication",
            }),
            17070 => Some(Self {
                code: 17070,
                acronym: "cathbr",
                name: "Category of harbour area",
            }),
            17071 => Some(Self {
                code: 17071,
                acronym: "catrfd",
                name: "Category of refuse dump",
            }),
            17072 => Some(Self {
                code: 17072,
                acronym: "cattml",
                name: "Category of terminal",
            }),
            17073 => Some(Self {
                code: 17073,
                acronym: "comctn",
                name: "Communication",
            }),
            17074 => Some(Self {
                code: 17074,
                acronym: "horcll",
                name: "Horizontal clearance, length",
            }),
            17075 => Some(Self {
                code: 17075,
                acronym: "horclw",
                name: "Horizontal clearance, width",
            }),
            17076 => Some(Self {
                code: 17076,
                acronym: "trshgd",
                name: "Transshipping goods",
            }),
            17077 => Some(Self {
                code: 17077,
                acronym: "unlocd",
                name: "UN Location Code",
            }),
            17112 => Some(Self {
                code: 17112,
                acronym: "catwwm",
                name: "Category of waterway mark",
            }),
            20484 => Some(Self {
                code: 20484,
                acronym: "databa",
                name: "Abandonment Date",
            }),
            20485 => Some(Self {
                code: 20485,
                acronym: "attutn",
                name: "Attenuation",
            }),
            20486 => Some(Self {
                code: 20486,
                acronym: "vesbem",
                name: "Beam of Vessel",
            }),
            20487 => Some(Self {
                code: 20487,
                acronym: "bearng",
                name: "Bearing",
            }),
            20488 => Some(Self {
                code: 20488,
                acronym: "blndzn",
                name: "Blind Zone",
            }),
            20489 => Some(Self {
                code: 20489,
                acronym: "brktyp",
                name: "Breaker Type",
            }),
            20490 => Some(Self {
                code: 20490,
                acronym: "bulkdn",
                name: "Density",
            }),
            20491 => Some(Self {
                code: 20491,
                acronym: "brmchm",
                name: "Burial Mechanism",
            }),
            20492 => Some(Self {
                code: 20492,
                acronym: "brpctg",
                name: "Burial Percentage",
            }),
            20493 => Some(Self {
                code: 20493,
                acronym: "brperd",
                name: "Burial Period",
            }),
            20494 => Some(Self {
                code: 20494,
                acronym: "brprob",
                name: "Burial Probability",
            }),
            20495 => Some(Self {
                code: 20495,
                acronym: "orcard",
                name: "Cardinal Point Orientation",
            }),
            20496 => Some(Self {
                code: 20496,
                acronym: "catadm",
                name: "Category of administration area",
            }),
            20497 => Some(Self {
                code: 20497,
                acronym: "catasr",
                name: "Category of airspace restriction",
            }),
            20498 => Some(Self {
                code: 20498,
                acronym: "N/A",
                name: "Category of bedrock",
            }),
            20499 => Some(Self {
                code: 20499,
                acronym: "catbot",
                name: "Bottom Feature Classification",
            }),
            20500 => Some(Self {
                code: 20500,
                acronym: "catcgs",
                name: "Category of coastguard station",
            }),
            20501 => Some(Self {
                code: 20501,
                acronym: "catcas",
                name: "Category of controlled airspace",
            }),
            20502 => Some(Self {
                code: 20502,
                acronym: "catfsh",
                name: "Fishing Activity",
            }),
            20503 => Some(Self {
                code: 20503,
                acronym: "catimg",
                name: "Type of Imagery",
            }),
            20504 => Some(Self {
                code: 20504,
                acronym: "catmma",
                name: "Category of marine management area",
            }),
            20505 => Some(Self {
                code: 20505,
                acronym: "catmsi",
                name: "Category of maritime safety information",
            }),
            20506 => Some(Self {
                code: 20506,
                acronym: "catmea",
                name: "Category of military exercise airspace ",
            }),
            20507 => Some(Self {
                code: 20507,
                acronym: "catpat",
                name: "Category of patrol area",
            }),
            20508 => Some(Self {
                code: 20508,
                acronym: "catrep",
                name: "Category of reporting/radio calling-in point",
            }),
            20509 => Some(Self {
                code: 20509,
                acronym: "N/A",
                name: "Category of regulated airspace",
            }),
            20510 => Some(Self {
                code: 20510,
                acronym: "catsbl",
                name: "Category of territorial sea baseline",
            }),
            20511 => Some(Self {
                code: 20511,
                acronym: "cattrf",
                name: "Trafficability",
            }),
            20512 => Some(Self {
                code: 20512,
                acronym: "comsys",
                name: "Command System",
            }),
            20515 => Some(Self {
                code: 20515,
                acronym: "caircd",
                name: "Controlled airspace class designation",
            }),
            20516 => Some(Self {
                code: 20516,
                acronym: "authty",
                name: "Controlling authority",
            }),
            20517 => Some(Self {
                code: 20517,
                acronym: "scrdim",
                name: "Current Scour Dimensions",
            }),
            20518 => Some(Self {
                code: 20518,
                acronym: "dgmrlf",
                name: "Dangerous Marine and Land Life",
            }),
            20519 => Some(Self {
                code: 20519,
                acronym: "datsnk",
                name: "Date Sunk",
            }),
            20520 => Some(Self {
                code: 20520,
                acronym: "debfld",
                name: "Debris Field",
            }),
            20521 => Some(Self {
                code: 20521,
                acronym: "depact",
                name: "Depth of Activity",
            }),
            20522 => Some(Self {
                code: 20522,
                acronym: "deplyr",
                name: "Depth of Layer",
            }),
            20523 => Some(Self {
                code: 20523,
                acronym: "discon",
                name: "Distance from Small Bottom Object",
            }),
            20524 => Some(Self {
                code: 20524,
                acronym: "dttdep",
                name: "Diver’s Thrust Test Depth",
            }),
            20525 => Some(Self {
                code: 20525,
                acronym: "dttnum",
                name: "Diver’s Thrust Test Number",
            }),
            20526 => Some(Self {
                code: 20526,
                acronym: "divact",
                name: "Diving Activity",
            }),
            20527 => Some(Self {
                code: 20527,
                acronym: "vesdgh",
                name: "Draught of Vessel",
            }),
            20528 => Some(Self {
                code: 20528,
                acronym: "exitus",
                name: "Exit Usability",
            }),
            20529 => Some(Self {
                code: 20529,
                acronym: "fldnam",
                name: "Field Name",
            }),
            20530 => Some(Self {
                code: 20530,
                acronym: "datfir",
                name: "First Detection Year",
            }),
            20531 => Some(Self {
                code: 20531,
                acronym: "senfir",
                name: "First Sensor",
            }),
            20532 => Some(Self {
                code: 20532,
                acronym: "sorfir",
                name: "First Source",
            }),
            20533 => Some(Self {
                code: 20533,
                acronym: "folinx",
                name: "Foliar Index",
            }),
            20534 => Some(Self {
                code: 20534,
                acronym: "gascon",
                name: "Gas Content",
            }),
            20535 => Some(Self {
                code: 20535,
                acronym: "gendep",
                name: "General Water Depth",
            }),
            20536 => Some(Self {
                code: 20536,
                acronym: "gradnt",
                name: "Gradient",
            }),
            20537 => Some(Self {
                code: 20537,
                acronym: "grnsiz",
                name: "Grain Size",
            }),
            20538 => Some(Self {
                code: 20538,
                acronym: "incltn",
                name: "Inclination",
            }),
            20539 => Some(Self {
                code: 20539,
                acronym: "N/A",
                name: "Internal Data Record Identification Number",
            }),
            20540 => Some(Self {
                code: 20540,
                acronym: "datlst",
                name: "Last Detection Year",
            }),
            20541 => Some(Self {
                code: 20541,
                acronym: "senlst",
                name: "Last Sensor",
            }),
            20542 => Some(Self {
                code: 20542,
                acronym: "sorlst",
                name: "Last Source",
            }),
            20543 => Some(Self {
                code: 20543,
                acronym: "layptm",
                name: "Lay Platform",
            }),
            20544 => Some(Self {
                code: 20544,
                acronym: "layrfn",
                name: "Lay Reference Number",
            }),
            20545 => Some(Self {
                code: 20545,
                acronym: "laytim",
                name: "Lay Time",
            }),
            20546 => Some(Self {
                code: 20546,
                acronym: "laynum",
                name: "Layer Number",
            }),
            20547 => Some(Self {
                code: 20547,
                acronym: "legsta",
                name: "Legal Status",
            }),
            20548 => Some(Self {
                code: 20548,
                acronym: "veslen",
                name: "Length of Vessel",
            }),
            20549 => Some(Self {
                code: 20549,
                acronym: "madsig",
                name: "Magnetic Anomaly Detector (MAD) Signature",
            }),
            20550 => Some(Self {
                code: 20550,
                acronym: "magint",
                name: "Magnetic Intensity",
            }),
            20551 => Some(Self {
                code: 20551,
                acronym: "msstrg",
                name: "Mean Shear Strength",
            }),
            20552 => Some(Self {
                code: 20552,
                acronym: "migdir",
                name: "Migration Direction",
            }),
            20553 => Some(Self {
                code: 20553,
                acronym: "migspd",
                name: "Migration Speed",
            }),
            20554 => Some(Self {
                code: 20554,
                acronym: "milden",
                name: "Milec Density",
            }),
            20555 => Some(Self {
                code: 20555,
                acronym: "mnimnc",
                name: "Mine Index Mine Case",
            }),
            20556 => Some(Self {
                code: 20556,
                acronym: "mnimnt",
                name: "Mine Index Mine Type",
            }),
            20557 => Some(Self {
                code: 20557,
                acronym: "minern",
                name: "Mine Reference Number",
            }),
            20558 => Some(Self {
                code: 20558,
                acronym: "mhclas",
                name: "Mine-Hunting Classification",
            }),
            20559 => Some(Self {
                code: 20559,
                acronym: "mnhsys",
                name: "Minehunting System",
            }),
            20560 => Some(Self {
                code: 20560,
                acronym: "mnssys",
                name: "Minesweeping System",
            }),
            20561 => Some(Self {
                code: 20561,
                acronym: "miscls",
                name: "Mission Classification",
            }),
            20562 => Some(Self {
                code: 20562,
                acronym: "miscom",
                name: "Mission Comments",
            }),
            20563 => Some(Self {
                code: 20563,
                acronym: "misdat",
                name: "Mission Date",
            }),
            20564 => Some(Self {
                code: 20564,
                acronym: "misnme",
                name: "Mission Name",
            }),
            20565 => Some(Self {
                code: 20565,
                acronym: "mwdcrn",
                name: "MWDC Reference Number",
            }),
            20566 => Some(Self {
                code: 20566,
                acronym: "natsed",
                name: "Nature of Geological Layer",
            }),
            20567 => Some(Self {
                code: 20567,
                acronym: "navsys",
                name: "Navigation System",
            }),
            20568 => Some(Self {
                code: 20568,
                acronym: "nomden",
                name: "NOMBO Density",
            }),
            20569 => Some(Self {
                code: 20569,
                acronym: "notfnd",
                name: "Not Found",
            }),
            20570 => Some(Self {
                code: 20570,
                acronym: "nmprob",
                name: "Number of Previous Observations",
            }),
            20571 => Some(Self {
                code: 20571,
                acronym: "oprtor",
                name: "Operator",
            }),
            20572 => Some(Self {
                code: 20572,
                acronym: "orbobn",
                name: "Orientation of Best Observation",
            }),
            20573 => Some(Self {
                code: 20573,
                acronym: "orgdat",
                name: "Origin of Data",
            }),
            20574 => Some(Self {
                code: 20574,
                acronym: "orgntr",
                name: "Originator",
            }),
            20575 => Some(Self {
                code: 20575,
                acronym: "porsty",
                name: "Porosity",
            }),
            20576 => Some(Self {
                code: 20576,
                acronym: "quabch",
                name: "Quality of Beach Data",
            }),
            20577 => Some(Self {
                code: 20577,
                acronym: "datren",
                name: "Re-entered Date",
            }),
            20578 => Some(Self {
                code: 20578,
                acronym: "datres",
                name: "Re-suspended Date",
            }),
            20579 => Some(Self {
                code: 20579,
                acronym: "revebn",
                name: "Reverberation",
            }),
            20580 => Some(Self {
                code: 20580,
                acronym: "N/A",
                name: "Safety Zone",
            }),
            20581 => Some(Self {
                code: 20581,
                acronym: "samret",
                name: "Sample Retained",
            }),
            20582 => Some(Self {
                code: 20582,
                acronym: "sbdcov",
                name: "Seabed Coverage",
            }),
            20583 => Some(Self {
                code: 20583,
                acronym: "shpspd",
                name: "Ships Speed",
            }),
            20584 => Some(Self {
                code: 20584,
                acronym: "snrfrq",
                name: "Sonar Frequency",
            }),
            20585 => Some(Self {
                code: 20585,
                acronym: "snrrsc",
                name: "Sonar Range Scale",
            }),
            20586 => Some(Self {
                code: 20586,
                acronym: "snrflc",
                name: "Sonar Reflectivity",
            }),
            20587 => Some(Self {
                code: 20587,
                acronym: "sonsig",
                name: "Sonar Signal Strength",
            }),
            20588 => Some(Self {
                code: 20588,
                acronym: "sndvel",
                name: "Sound Velocity",
            }),
            20589 => Some(Self {
                code: 20589,
                acronym: "soudat",
                name: "Sounding Datum",
            }),
            20590 => Some(Self {
                code: 20590,
                acronym: "datspd",
                name: "Spudded Date",
            }),
            20592 => Some(Self {
                code: 20592,
                acronym: "stfotn",
                name: "Steepest Face Orientation",
            }),
            20593 => Some(Self {
                code: 20593,
                acronym: "ricsca",
                name: "Strength According to Richter Scale",
            }),
            20594 => Some(Self {
                code: 20594,
                acronym: "magany",
                name: "Strength of Magnetic Anomaly",
            }),
            20595 => Some(Self {
                code: 20595,
                acronym: "stbacv",
                name: "Suitability for ACV Use",
            }),
            20596 => Some(Self {
                code: 20596,
                acronym: "srfhgt",
                name: "Surf Height",
            }),
            20597 => Some(Self {
                code: 20597,
                acronym: "srfzne",
                name: "Surf Zone",
            }),
            20598 => Some(Self {
                code: 20598,
                acronym: "surdat",
                name: "Survey Date and Time",
            }),
            20599 => Some(Self {
                code: 20599,
                acronym: "datsus",
                name: "Suspension Date",
            }),
            20600 => Some(Self {
                code: 20600,
                acronym: "swlhgt",
                name: "Swell Height",
            }),
            20601 => Some(Self {
                code: 20601,
                acronym: "tdlrng",
                name: "Tidal Range",
            }),
            20602 => Some(Self {
                code: 20602,
                acronym: "timeyr",
                name: "Time of Year",
            }),
            20603 => Some(Self {
                code: 20603,
                acronym: "tonage",
                name: "Tonnage",
            }),
            20604 => Some(Self {
                code: 20604,
                acronym: "twdbdp",
                name: "Towed Body Depth",
            }),
            20605 => Some(Self {
                code: 20605,
                acronym: "milact",
                name: "Type of military activity",
            }),
            20606 => Some(Self {
                code: 20606,
                acronym: "typton",
                name: "Type of Tonnage",
            }),
            20607 => Some(Self {
                code: 20607,
                acronym: "typewk",
                name: "Type of Wreck",
            }),
            20608 => Some(Self {
                code: 20608,
                acronym: "unwrfm",
                name: "Underwater Reference Mark",
            }),
            20609 => Some(Self {
                code: 20609,
                acronym: "N/A",
                name: "Unique ID from a Navigational Product",
            }),
            20610 => Some(Self {
                code: 20610,
                acronym: "watclr",
                name: "Water Clarity",
            }),
            20611 => Some(Self {
                code: 20611,
                acronym: "wavlen",
                name: "Wavelength",
            }),
            20612 => Some(Self {
                code: 20612,
                acronym: "wbrcap",
                name: "Weight Bearing Capability",
            }),
            20613 => Some(Self {
                code: 20613,
                acronym: "lftwid",
                name: "Width (left)",
            }),
            20614 => Some(Self {
                code: 20614,
                acronym: "rgtwid",
                name: "Width (right)",
            }),
            20615 => Some(Self {
                code: 20615,
                acronym: "hypcat",
                name: "Contour Type",
            }),
            20616 => Some(Self {
                code: 20616,
                acronym: "souvel",
                name: "Sounding Velocity",
            }),
            20617 => Some(Self {
                code: 20617,
                acronym: "accres",
                name: "Access Restriction",
            }),
            20618 => Some(Self {
                code: 20618,
                acronym: "apprch",
                name: "Approach",
            }),
            20619 => Some(Self {
                code: 20619,
                acronym: "catbch",
                name: "Category of Beach",
            }),
            20620 => Some(Self {
                code: 20620,
                acronym: "clperc",
                name: "Clearance Percentage",
            }),
            20621 => Some(Self {
                code: 20621,
                acronym: "commns",
                name: "Communications",
            }),
            20622 => Some(Self {
                code: 20622,
                acronym: "conlev",
                name: "Confidence Level",
            }),
            20624 => Some(Self {
                code: 20624,
                acronym: "extdes",
                name: "Exit Description",
            }),
            20625 => Some(Self {
                code: 20625,
                acronym: "indtry",
                name: "Industry",
            }),
            20626 => Some(Self {
                code: 20626,
                acronym: "lndcon",
                name: "Landing Conditions",
            }),
            20627 => Some(Self {
                code: 20627,
                acronym: "lsract",
                name: "Leisure Activity",
            }),
            20628 => Some(Self {
                code: 20628,
                acronym: "logtcs",
                name: "Logistics",
            }),
            20629 => Some(Self {
                code: 20629,
                acronym: "manvrg",
                name: "Manoeuvring",
            }),
            20630 => Some(Self {
                code: 20630,
                acronym: "mntden",
                name: "Mine Threat Density",
            }),
            20631 => Some(Self {
                code: 20631,
                acronym: "mulcon",
                name: "Multiple Contacts",
            }),
            20632 => Some(Self {
                code: 20632,
                acronym: "navdes",
                name: "Navigational Description",
            }),
            20633 => Some(Self {
                code: 20633,
                acronym: "navdif",
                name: "Navigational Difficulty",
            }),
            20634 => Some(Self {
                code: 20634,
                acronym: "numrmn",
                name: "Number of Remaining Mines",
            }),
            20635 => Some(Self {
                code: 20635,
                acronym: "pierod",
                name: "Pier Contact Details",
            }),
            20636 => Some(Self {
                code: 20636,
                acronym: "pierdn",
                name: "Pier Description",
            }),
            20637 => Some(Self {
                code: 20637,
                acronym: "prsden",
                name: "Prairies Density",
            }),
            20638 => Some(Self {
                code: 20638,
                acronym: "prbrmn",
                name: "Probability for Remaining Mines",
            }),
            20639 => Some(Self {
                code: 20639,
                acronym: "rmnlmn",
                name: "Remaining Mines Likely, Maximum Number",
            }),
            20640 => Some(Self {
                code: 20640,
                acronym: "sfptna",
                name: "Self Protection (Air)",
            }),
            20641 => Some(Self {
                code: 20641,
                acronym: "sptnnd",
                name: "Self Protection (Near Defence)",
            }),
            20642 => Some(Self {
                code: 20642,
                acronym: "sfptns",
                name: "Self Protection (Surface)",
            }),
            20643 => Some(Self {
                code: 20643,
                acronym: "sencov",
                name: "Sensor Coverage",
            }),
            20644 => Some(Self {
                code: 20644,
                acronym: "sminth",
                name: "Simple Initial Threat",
            }),
            20645 => Some(Self {
                code: 20645,
                acronym: "tgrfwt",
                name: "Target Reference Weight",
            }),
            20646 => Some(Self {
                code: 20646,
                acronym: "tdltyp",
                name: "Tidal Type",
            }),
            20647 => Some(Self {
                code: 20647,
                acronym: "typres",
                name: "Type of Resource Location",
            }),
            20648 => Some(Self {
                code: 20648,
                acronym: "undmnr",
                name: "Undetectable Mines Ratio",
            }),
            20649 => Some(Self {
                code: 20649,
                acronym: "umnrwb",
                name: "Undetectable Mines Ratio with Burial",
            }),
            20650 => Some(Self {
                code: 20650,
                acronym: "umrwob",
                name: "Undetectable Mines Ratio without Burial",
            }),
            20651 => Some(Self {
                code: 20651,
                acronym: "wpncov",
                name: "Weapon Coverage",
            }),
            20652 => Some(Self {
                code: 20652,
                acronym: "onsonr",
                name: "On Sonar",
            }),
            20653 => Some(Self {
                code: 20653,
                acronym: "hfbmls",
                name: "HF Bottom Loss",
            }),
            20654 => Some(Self {
                code: 20654,
                acronym: "lfbmls",
                name: "LF Bottom Loss",
            }),
            20655 => Some(Self {
                code: 20655,
                acronym: "dtprob",
                name: "Detection Probability",
            }),
            20656 => Some(Self {
                code: 20656,
                acronym: "dsprob",
                name: "Disposal Probability",
            }),
            20657 => Some(Self {
                code: 20657,
                acronym: "clprob",
                name: "Classification Probability",
            }),
            20658 => Some(Self {
                code: 20658,
                acronym: "cswidt",
                name: "Characteristic Detection Width (A)",
            }),
            20659 => Some(Self {
                code: 20659,
                acronym: "csprob",
                name: "Characteristic Detection Probability (B)",
            }),
            20660 => Some(Self {
                code: 20660,
                acronym: "znecol",
                name: "Zone Colour",
            }),
            20661 => Some(Self {
                code: 20661,
                acronym: "revfqy",
                name: "Reverberation Frequency",
            }),
            20662 => Some(Self {
                code: 20662,
                acronym: "revgan",
                name: "Reverberation Grazing Angle",
            }),
            20663 => Some(Self {
                code: 20663,
                acronym: "secido",
                name: "International Defence Organisation (IDO) status",
            }),
            20664 => Some(Self {
                code: 20664,
                acronym: "secpmk",
                name: "Protective Marking",
            }),
            20665 => Some(Self {
                code: 20665,
                acronym: "secown",
                name: "Owner Authority",
            }),
            20666 => Some(Self {
                code: 20666,
                acronym: "seccvt",
                name: "Caveat ",
            }),
            20667 => Some(Self {
                code: 20667,
                acronym: "spcies",
                name: "Species",
            }),
            20668 => Some(Self {
                code: 20668,
                acronym: "swpdat",
                name: "Swept date",
            }),
            20669 => Some(Self {
                code: 20669,
                acronym: "rwylen",
                name: "Runway length",
            }),
            20670 => Some(Self {
                code: 20670,
                acronym: "actper",
                name: "Active period",
            }),
            20671 => Some(Self {
                code: 20671,
                acronym: "maxalt",
                name: "Maximum altitude",
            }),
            20672 => Some(Self {
                code: 20672,
                acronym: "minalt",
                name: "Minimum altitude",
            }),
            20673 => Some(Self {
                code: 20673,
                acronym: "maxftl",
                name: "Maximum Flight Level",
            }),
            20674 => Some(Self {
                code: 20674,
                acronym: "minftl",
                name: "Minimum Flight Level",
            }),
            20675 => Some(Self {
                code: 20675,
                acronym: "bverss",
                name: "Bottom Vertical Safety Separation",
            }),
            20676 => Some(Self {
                code: 20676,
                acronym: "mindep",
                name: "Minimum Safe Depth",
            }),
            20677 => Some(Self {
                code: 20677,
                acronym: "linech",
                name: "Interpolated line characteristic",
            }),
            20678 => Some(Self {
                code: 20678,
                acronym: "identy",
                name: "Identification",
            }),
            20679 => Some(Self {
                code: 20679,
                acronym: "rclass",
                name: "Route Classification",
            }),
            20680 => Some(Self {
                code: 20680,
                acronym: "popltn",
                name: "Population",
            }),
            20681 => Some(Self {
                code: 20681,
                acronym: "surtht",
                name: "Surface Threat",
            }),
            20682 => Some(Self {
                code: 20682,
                acronym: "upbear",
                name: "Heading-Up Bearing",
            }),
            20683 => Some(Self {
                code: 20683,
                acronym: "dnbear",
                name: "Heading-Down Bearing",
            }),
            20684 => Some(Self {
                code: 20684,
                acronym: "icencn",
                name: "Ice Concentration",
            }),
            20685 => Some(Self {
                code: 20685,
                acronym: "dgrhgt",
                name: "Danger height",
            }),
            20686 => Some(Self {
                code: 20686,
                acronym: "depres",
                name: "Depth Restriction",
            }),
            20687 => Some(Self {
                code: 20687,
                acronym: "arecat",
                name: "Area Category",
            }),
            20688 => Some(Self {
                code: 20688,
                acronym: "exzres",
                name: "Existence of Restricted Area",
            }),
            20689 => Some(Self {
                code: 20689,
                acronym: "tarstg",
                name: "Target Strength",
            }),
            20690 => Some(Self {
                code: 20690,
                acronym: "quarad",
                name: "Qualification of Radar Coverage",
            }),
            20691 => Some(Self {
                code: 20691,
                acronym: "condet",
                name: "Contact Details",
            }),
            20692 => Some(Self {
                code: 20692,
                acronym: "limanc",
                name: "Limit of Anchors and Chains",
            }),
            20693 => Some(Self {
                code: 20693,
                acronym: "ccmidx",
                name: "CCM Index",
            }),
            20694 => Some(Self {
                code: 20694,
                acronym: "mlclas",
                name: "Military Load Classification",
            }),
            20695 => Some(Self {
                code: 20695,
                acronym: "mgstyp",
                name: "MGS Type",
            }),
            20696 => Some(Self {
                code: 20696,
                acronym: "iceact",
                name: "Ice Attribute Concentration Total",
            }),
            20697 => Some(Self {
                code: 20697,
                acronym: "icesod",
                name: "Ice Stage of Development",
            }),
            20698 => Some(Self {
                code: 20698,
                acronym: "iceadc",
                name: "Ice Advisory Code",
            }),
            20699 => Some(Self {
                code: 20699,
                acronym: "icebnm",
                name: "Number of Icebergs in Area",
            }),
            20700 => Some(Self {
                code: 20700,
                acronym: "icelnc",
                name: "Ice Line Category",
            }),
            20701 => Some(Self {
                code: 20701,
                acronym: "icepty",
                name: "Ice Polynya Type",
            }),
            20702 => Some(Self {
                code: 20702,
                acronym: "icepst",
                name: "Ice Polynya Status",
            }),
            20703 => Some(Self {
                code: 20703,
                acronym: "icelty",
                name: "Ice Lead Type",
            }),
            20704 => Some(Self {
                code: 20704,
                acronym: "icelst",
                name: "Ice Lead Status",
            }),
            20705 => Some(Self {
                code: 20705,
                acronym: "icebsz",
                name: "Iceberg Size",
            }),
            20706 => Some(Self {
                code: 20706,
                acronym: "icebsh",
                name: "Iceberg Shape",
            }),
            20707 => Some(Self {
                code: 20707,
                acronym: "icebdr",
                name: "Icedrift or Iceberg Direction",
            }),
            20708 => Some(Self {
                code: 20708,
                acronym: "icebsp",
                name: "Icedrift or Iceberg Speed",
            }),
            20709 => Some(Self {
                code: 20709,
                acronym: "icemax",
                name: "Maximum Ice Thickness",
            }),
            20710 => Some(Self {
                code: 20710,
                acronym: "icemin",
                name: "Minimum Ice Thickness",
            }),
            20711 => Some(Self {
                code: 20711,
                acronym: "icerdv",
                name: "Ice Ridge Development",
            }),
            20712 => Some(Self {
                code: 20712,
                acronym: "icelnd",
                name: "Land Ice",
            }),
            20713 => Some(Self {
                code: 20713,
                acronym: "seadir",
                name: "Sea Direction",
            }),
            20714 => Some(Self {
                code: 20714,
                acronym: "traden",
                name: "Traffic density",
            }),
            20715 => Some(Self {
                code: 20715,
                acronym: "typshp",
                name: "Type of shipping",
            }),
            20716 => Some(Self {
                code: 20716,
                acronym: "icecvt",
                name: "Ice Coverage Type",
            }),
            20718 => Some(Self {
                code: 20718,
                acronym: "staobj",
                name: "Status of Small Bottom Object",
            }),
            20719 => Some(Self {
                code: 20719,
                acronym: "icaocd",
                name: "ICAO code",
            }),
            20720 => Some(Self {
                code: 20720,
                acronym: "txtdes",
                name: "textual description",
            }),
            20721 => Some(Self {
                code: 20721,
                acronym: "objtrn",
                name: "Object Reference Number",
            }),
            20722 => Some(Self {
                code: 20722,
                acronym: "objshp",
                name: "Object Shape",
            }),
            22484 => Some(Self {
                code: 22484,
                acronym: "catcnf",
                name: "Category of completeness",
            }),
            22485 => Some(Self {
                code: 22485,
                acronym: "errell",
                name: "Error Ellipse",
            }),
            22486 => Some(Self {
                code: 22486,
                acronym: "N/A",
                name: "Object classes",
            }),
            22487 => Some(Self {
                code: 22487,
                acronym: "N/A",
                name: "Security classification",
            }),
            22488 => Some(Self {
                code: 22488,
                acronym: "vershf",
                name: "Vertical Datum Shift Parameter",
            }),
            22489 => Some(Self {
                code: 22489,
                acronym: "elvacc",
                name: "Absolute Vertical Accuracy",
            }),
            22490 => Some(Self {
                code: 22490,
                acronym: "reflco",
                name: "Reflection Coefficient",
            }),
            22491 => Some(Self {
                code: 22491,
                acronym: "cpyrit",
                name: "Copyright statement",
            }),
            40000 => Some(Self {
                code: 40000,
                acronym: "updmsg",
                name: "Update message",
            }),
            _ => None,
        }
    }

    /// Get human-readable attribute name from ATTL code
    pub fn attribute_name(attl: u16) -> Option<&'static str> {
        Self::from_code(attl).map(|info| info.name)
    }
}
