// Generated from s57objectclasses.csv
// DO NOT EDIT - run scripts/generate_catalogue.nu to regenerate

#![allow(non_camel_case_types)]
#![allow(unreachable_patterns)]

use strum_macros::{Display, EnumString};

/// S-57 Object Class
///
/// Complete catalogue of IHO S-57 object classes from GDAL reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]
#[non_exhaustive]
pub enum ObjectClass {
    /// ADMARE (1): Administration area (Named)
    #[strum(serialize = "ADMARE")]
    AdministrationAreaNamed = 1,

    /// AIRARE (2): Airport / airfield
    #[strum(serialize = "AIRARE")]
    AirportAirfield = 2,

    /// ACHBRT (3): Anchor berth
    #[strum(serialize = "ACHBRT")]
    AnchorBerth = 3,

    /// ACHARE (4): Anchorage area
    #[strum(serialize = "ACHARE")]
    AnchorageArea = 4,

    /// BCNCAR (5): Beacon, cardinal
    #[strum(serialize = "BCNCAR")]
    BeaconCardinal = 5,

    /// BCNISD (6): Beacon, isolated danger
    #[strum(serialize = "BCNISD")]
    BeaconIsolatedDanger = 6,

    /// BCNLAT (7): Beacon, lateral
    #[strum(serialize = "BCNLAT")]
    BeaconLateral = 7,

    /// BCNSAW (8): Beacon, safe water
    #[strum(serialize = "BCNSAW")]
    BeaconSafeWater = 8,

    /// BCNSPP (9): Beacon, special purpose/general
    #[strum(serialize = "BCNSPP")]
    BeaconSpecialPurposegeneral = 9,

    /// BERTHS (10): Berth
    #[strum(serialize = "BERTHS")]
    Berth = 10,

    /// BRIDGE (11): Bridge
    #[strum(serialize = "BRIDGE")]
    Bridge = 11,

    /// BUISGL (12): Building, single
    #[strum(serialize = "BUISGL")]
    BuildingSingle = 12,

    /// BUAARE (13): Built-up area
    #[strum(serialize = "BUAARE")]
    BuiltupArea = 13,

    /// BOYCAR (14): Buoy, cardinal
    #[strum(serialize = "BOYCAR")]
    BuoyCardinal = 14,

    /// BOYINB (15): Buoy, installation
    #[strum(serialize = "BOYINB")]
    BuoyInstallation = 15,

    /// BOYISD (16): Buoy, isolated danger
    #[strum(serialize = "BOYISD")]
    BuoyIsolatedDanger = 16,

    /// BOYLAT (17): Buoy, lateral
    #[strum(serialize = "BOYLAT")]
    BuoyLateral = 17,

    /// BOYSAW (18): Buoy, safe water
    #[strum(serialize = "BOYSAW")]
    BuoySafeWater = 18,

    /// BOYSPP (19): Buoy, special purpose/general
    #[strum(serialize = "BOYSPP")]
    BuoySpecialPurposegeneral = 19,

    /// CBLARE (20): Cable area
    #[strum(serialize = "CBLARE")]
    CableArea = 20,

    /// CBLOHD (21): Cable, overhead
    #[strum(serialize = "CBLOHD")]
    CableOverhead = 21,

    /// CBLSUB (22): Cable, submarine
    #[strum(serialize = "CBLSUB")]
    CableSubmarine = 22,

    /// CANALS (23): Canal
    #[strum(serialize = "CANALS")]
    Canal = 23,

    /// CANBNK (24): Canal bank
    #[strum(serialize = "CANBNK")]
    CanalBank = 24,

    /// CTSARE (25): Cargo transshipment area
    #[strum(serialize = "CTSARE")]
    CargoTransshipmentArea = 25,

    /// CAUSWY (26): Causeway
    #[strum(serialize = "CAUSWY")]
    Causeway = 26,

    /// CTNARE (27): Caution area
    #[strum(serialize = "CTNARE")]
    CautionArea = 27,

    /// CHKPNT (28): Checkpoint
    #[strum(serialize = "CHKPNT")]
    Checkpoint = 28,

    /// CGUSTA (29): Coastguard station
    #[strum(serialize = "CGUSTA")]
    CoastguardStation = 29,

    /// COALNE (30): Coastline
    #[strum(serialize = "COALNE")]
    Coastline = 30,

    /// CONZNE (31): Contiguous zone
    #[strum(serialize = "CONZNE")]
    ContiguousZone = 31,

    /// COSARE (32): Continental shelf area
    #[strum(serialize = "COSARE")]
    ContinentalShelfArea = 32,

    /// CTRPNT (33): Control point
    #[strum(serialize = "CTRPNT")]
    ControlPoint = 33,

    /// CONVYR (34): Conveyor
    #[strum(serialize = "CONVYR")]
    Conveyor = 34,

    /// CRANES (35): Crane
    #[strum(serialize = "CRANES")]
    Crane = 35,

    /// CURENT (36): Current - non - gravitational
    #[strum(serialize = "CURENT")]
    CurrentNonGravitational = 36,

    /// CUSZNE (37): Custom zone
    #[strum(serialize = "CUSZNE")]
    CustomZone = 37,

    /// DAMCON (38): Dam
    #[strum(serialize = "DAMCON")]
    Dam = 38,

    /// DAYMAR (39): Daymark
    #[strum(serialize = "DAYMAR")]
    Daymark = 39,

    /// DWRTCL (40): Deep water route centerline
    #[strum(serialize = "DWRTCL")]
    DeepWaterRouteCenterline = 40,

    /// DWRTPT (41): Deep water route part
    #[strum(serialize = "DWRTPT")]
    DeepWaterRoutePart = 41,

    /// DEPARE (42): Depth area
    #[strum(serialize = "DEPARE")]
    DepthArea = 42,

    /// DEPCNT (43): Depth contour
    #[strum(serialize = "DEPCNT")]
    DepthContour = 43,

    /// DISMAR (44): Distance mark
    #[strum(serialize = "DISMAR")]
    DistanceMark = 44,

    /// DOCARE (45): Dock area
    #[strum(serialize = "DOCARE")]
    DockArea = 45,

    /// DRGARE (46): Dredged area
    #[strum(serialize = "DRGARE")]
    DredgedArea = 46,

    /// DRYDOC (47): Dry dock
    #[strum(serialize = "DRYDOC")]
    DryDock = 47,

    /// DMPGRD (48): Dumping ground
    #[strum(serialize = "DMPGRD")]
    DumpingGround = 48,

    /// DYKCON (49): Dyke
    #[strum(serialize = "DYKCON")]
    Dyke = 49,

    /// EXEZNE (50): Exclusive Economic Zone
    #[strum(serialize = "EXEZNE")]
    ExclusiveEconomicZone = 50,

    /// FAIRWY (51): Fairway
    #[strum(serialize = "FAIRWY")]
    Fairway = 51,

    /// FNCLNE (52): Fence/wall
    #[strum(serialize = "FNCLNE")]
    Fencewall = 52,

    /// FERYRT (53): Ferry route
    #[strum(serialize = "FERYRT")]
    FerryRoute = 53,

    /// FSHZNE (54): Fishery zone
    #[strum(serialize = "FSHZNE")]
    FisheryZone = 54,

    /// FSHFAC (55): Fishing facility
    #[strum(serialize = "FSHFAC")]
    FishingFacility = 55,

    /// FSHGRD (56): Fishing ground
    #[strum(serialize = "FSHGRD")]
    FishingGround = 56,

    /// FLODOC (57): Floating dock
    #[strum(serialize = "FLODOC")]
    FloatingDock = 57,

    /// FOGSIG (58): Fog signal
    #[strum(serialize = "FOGSIG")]
    FogSignal = 58,

    /// FORSTC (59): Fortified structure
    #[strum(serialize = "FORSTC")]
    FortifiedStructure = 59,

    /// FRPARE (60): Free port area
    #[strum(serialize = "FRPARE")]
    FreePortArea = 60,

    /// GATCON (61): Gate
    #[strum(serialize = "GATCON")]
    Gate = 61,

    /// GRIDRN (62): Gridiron
    #[strum(serialize = "GRIDRN")]
    Gridiron = 62,

    /// HRBARE (63): Harbour area (administrative)
    #[strum(serialize = "HRBARE")]
    HarbourAreaAdministrative = 63,

    /// HRBFAC (64): Harbour facility
    #[strum(serialize = "HRBFAC")]
    HarbourFacility = 64,

    /// HULKES (65): Hulk
    #[strum(serialize = "HULKES")]
    Hulk = 65,

    /// ICEARE (66): Ice area
    #[strum(serialize = "ICEARE")]
    IceArea = 66,

    /// ICNARE (67): Incineration area
    #[strum(serialize = "ICNARE")]
    IncinerationArea = 67,

    /// ISTZNE (68): Inshore traffic zone
    #[strum(serialize = "ISTZNE")]
    InshoreTrafficZone = 68,

    /// LAKARE (69): Lake
    #[strum(serialize = "LAKARE")]
    Lake = 69,

    /// LAKSHR (70): Lake shore
    #[strum(serialize = "LAKSHR")]
    LakeShore = 70,

    /// LNDARE (71): Land area
    #[strum(serialize = "LNDARE")]
    LandArea = 71,

    /// LNDELV (72): Land elevation
    #[strum(serialize = "LNDELV")]
    LandElevation = 72,

    /// LNDRGN (73): Land region
    #[strum(serialize = "LNDRGN")]
    LandRegion = 73,

    /// LNDMRK (74): Landmark
    #[strum(serialize = "LNDMRK")]
    Landmark = 74,

    /// LIGHTS (75): Light
    #[strum(serialize = "LIGHTS")]
    Light = 75,

    /// LITFLT (76): Light float
    #[strum(serialize = "LITFLT")]
    LightFloat = 76,

    /// LITVES (77): Light vessel
    #[strum(serialize = "LITVES")]
    LightVessel = 77,

    /// LOCMAG (78): Local magnetic anomaly
    #[strum(serialize = "LOCMAG")]
    LocalMagneticAnomaly = 78,

    /// LOKBSN (79): Lock basin
    #[strum(serialize = "LOKBSN")]
    LockBasin = 79,

    /// LOGPON (80): Log pond
    #[strum(serialize = "LOGPON")]
    LogPond = 80,

    /// MAGVAR (81): Magnetic variation
    #[strum(serialize = "MAGVAR")]
    MagneticVariation = 81,

    /// MARCUL (82): Marine farm/culture
    #[strum(serialize = "MARCUL")]
    MarineFarmculture = 82,

    /// MIPARE (83): Military practice area
    #[strum(serialize = "MIPARE")]
    MilitaryPracticeArea = 83,

    /// MORFAC (84): Mooring/warping facility
    #[strum(serialize = "MORFAC")]
    MooringwarpingFacility = 84,

    /// NAVLNE (85): Navigation line
    #[strum(serialize = "NAVLNE")]
    NavigationLine = 85,

    /// OBSTRN (86): Obstruction
    #[strum(serialize = "OBSTRN")]
    Obstruction = 86,

    /// OFSPLF (87): Offshore platform
    #[strum(serialize = "OFSPLF")]
    OffshorePlatform = 87,

    /// OSPARE (88): Offshore production area
    #[strum(serialize = "OSPARE")]
    OffshoreProductionArea = 88,

    /// OILBAR (89): Oil barrier
    #[strum(serialize = "OILBAR")]
    OilBarrier = 89,

    /// PILPNT (90): Pile
    #[strum(serialize = "PILPNT")]
    Pile = 90,

    /// PILBOP (91): Pilot boarding place
    #[strum(serialize = "PILBOP")]
    PilotBoardingPlace = 91,

    /// PIPARE (92): Pipeline area
    #[strum(serialize = "PIPARE")]
    PipelineArea = 92,

    /// PIPOHD (93): Pipeline, overhead
    #[strum(serialize = "PIPOHD")]
    PipelineOverhead = 93,

    /// PIPSOL (94): Pipeline, submarine/on land
    #[strum(serialize = "PIPSOL")]
    PipelineSubmarineonLand = 94,

    /// PONTON (95): Pontoon
    #[strum(serialize = "PONTON")]
    Pontoon = 95,

    /// PRCARE (96): Precautionary area
    #[strum(serialize = "PRCARE")]
    PrecautionaryArea = 96,

    /// PRDARE (97): Production / storage area
    #[strum(serialize = "PRDARE")]
    ProductionStorageArea = 97,

    /// PYLONS (98): Pylon/bridge support
    #[strum(serialize = "PYLONS")]
    PylonbridgeSupport = 98,

    /// RADLNE (99): Radar line
    #[strum(serialize = "RADLNE")]
    RadarLine = 99,

    /// RADRNG (100): Radar range
    #[strum(serialize = "RADRNG")]
    RadarRange = 100,

    /// RADRFL (101): Radar reflector
    #[strum(serialize = "RADRFL")]
    RadarReflector = 101,

    /// RADSTA (102): Radar station
    #[strum(serialize = "RADSTA")]
    RadarStation = 102,

    /// RTPBCN (103): Radar transponder beacon
    #[strum(serialize = "RTPBCN")]
    RadarTransponderBeacon = 103,

    /// RDOCAL (104): Radio calling-in point
    #[strum(serialize = "RDOCAL")]
    RadioCallinginPoint = 104,

    /// RDOSTA (105): Radio station
    #[strum(serialize = "RDOSTA")]
    RadioStation = 105,

    /// RAILWY (106): Railway
    #[strum(serialize = "RAILWY")]
    Railway = 106,

    /// RAPIDS (107): Rapids
    #[strum(serialize = "RAPIDS")]
    Rapids = 107,

    /// RCRTCL (108): Recommended route centerline
    #[strum(serialize = "RCRTCL")]
    RecommendedRouteCenterline = 108,

    /// RECTRC (109): Recommended track
    #[strum(serialize = "RECTRC")]
    RecommendedTrack = 109,

    /// RCTLPT (110): Recommended Traffic Lane Part
    #[strum(serialize = "RCTLPT")]
    RecommendedTrafficLanePart = 110,

    /// RSCSTA (111): Rescue station
    #[strum(serialize = "RSCSTA")]
    RescueStation = 111,

    /// RESARE (112): Restricted area
    #[strum(serialize = "RESARE")]
    RestrictedArea = 112,

    /// RETRFL (113): Retro-reflector
    #[strum(serialize = "RETRFL")]
    Retroreflector = 113,

    /// RIVERS (114): River
    #[strum(serialize = "RIVERS")]
    River = 114,

    /// RIVBNK (115): River bank
    #[strum(serialize = "RIVBNK")]
    RiverBank = 115,

    /// ROADWY (116): Road
    #[strum(serialize = "ROADWY")]
    Road = 116,

    /// RUNWAY (117): Runway
    #[strum(serialize = "RUNWAY")]
    Runway = 117,

    /// SNDWAV (118): Sand waves
    #[strum(serialize = "SNDWAV")]
    SandWaves = 118,

    /// SEAARE (119): Sea area / named water area
    #[strum(serialize = "SEAARE")]
    SeaAreaNamedWaterArea = 119,

    /// SPLARE (120): Sea-plane landing area
    #[strum(serialize = "SPLARE")]
    SeaplaneLandingArea = 120,

    /// SBDARE (121): Seabed area
    #[strum(serialize = "SBDARE")]
    SeabedArea = 121,

    /// SLCONS (122): Shoreline Construction
    #[strum(serialize = "SLCONS")]
    ShorelineConstruction = 122,

    /// SISTAT (123): Signal station, traffic
    #[strum(serialize = "SISTAT")]
    SignalStationTraffic = 123,

    /// SISTAW (124): Signal station, warning
    #[strum(serialize = "SISTAW")]
    SignalStationWarning = 124,

    /// SILTNK (125): Silo / tank
    #[strum(serialize = "SILTNK")]
    SiloTank = 125,

    /// SLOTOP (126): Slope topline
    #[strum(serialize = "SLOTOP")]
    SlopeTopline = 126,

    /// SLOGRD (127): Sloping ground
    #[strum(serialize = "SLOGRD")]
    SlopingGround = 127,

    /// SMCFAC (128): Small craft facility
    #[strum(serialize = "SMCFAC")]
    SmallCraftFacility = 128,

    /// SOUNDG (129): Sounding
    #[strum(serialize = "SOUNDG")]
    Sounding = 129,

    /// SPRING (130): Spring
    #[strum(serialize = "SPRING")]
    Spring = 130,

    /// SQUARE (131): Square
    #[strum(serialize = "SQUARE")]
    Square = 131,

    /// STSLNE (132): Straight territorial sea baseline
    #[strum(serialize = "STSLNE")]
    StraightTerritorialSeaBaseline = 132,

    /// SUBTLN (133): Submarine transit lane
    #[strum(serialize = "SUBTLN")]
    SubmarineTransitLane = 133,

    /// SWPARE (134): Swept Area
    #[strum(serialize = "SWPARE")]
    SweptArea = 134,

    /// TESARE (135): Territorial sea area
    #[strum(serialize = "TESARE")]
    TerritorialSeaArea = 135,

    /// TS_PRH (136): Tidal stream - harmonic prediction
    #[strum(serialize = "TS_PRH")]
    TidalStreamHarmonicPrediction = 136,

    /// TS_PNH (137): Tidal stream - non-harmonic prediction
    #[strum(serialize = "TS_PNH")]
    TidalStreamNonharmonicPrediction = 137,

    /// TS_PAD (138): Tidal stream panel data
    #[strum(serialize = "TS_PAD")]
    TidalStreamPanelData = 138,

    /// TS_TIS (139): Tidal stream - time series (TS_TIS)
    #[strum(serialize = "TS_TIS")]
    TidalStreamTimeSeriesTS_TIS = 139,

    /// T_HMON (140): Tide - harmonic prediction
    #[strum(serialize = "T_HMON")]
    TideHarmonicPrediction = 140,

    /// T_NHMN (141): Tide - non-harmonic prediction
    #[strum(serialize = "T_NHMN")]
    TideNonharmonicPrediction = 141,

    /// T_TIMS (142): Tidal stream - time series (T_TIMS)
    #[strum(serialize = "T_TIMS")]
    TidalStreamTimeSeriesT_TIMS = 142,

    /// TIDEWY (143): Tideway
    #[strum(serialize = "TIDEWY")]
    Tideway = 143,

    /// TOPMAR (144): Top mark
    #[strum(serialize = "TOPMAR")]
    TopMark = 144,

    /// TSELNE (145): Traffic Separation Line
    #[strum(serialize = "TSELNE")]
    TrafficSeparationLine = 145,

    /// TSSBND (146): Traffic Separation Scheme  Boundary
    #[strum(serialize = "TSSBND")]
    TrafficSeparationSchemeBoundary = 146,

    /// TSSCRS (147): Traffic Separation Scheme Crossing
    #[strum(serialize = "TSSCRS")]
    TrafficSeparationSchemeCrossing = 147,

    /// TSSLPT (148): Traffic Separation Scheme  Lane part
    #[strum(serialize = "TSSLPT")]
    TrafficSeparationSchemeLanePart = 148,

    /// TSSRON (149): Traffic Separation Scheme  Roundabout
    #[strum(serialize = "TSSRON")]
    TrafficSeparationSchemeRoundabout = 149,

    /// TSEZNE (150): Traffic Separation Zone
    #[strum(serialize = "TSEZNE")]
    TrafficSeparationZone = 150,

    /// TUNNEL (151): Tunnel
    #[strum(serialize = "TUNNEL")]
    Tunnel = 151,

    /// TWRTPT (152): Two-way route  part
    #[strum(serialize = "TWRTPT")]
    TwowayRoutePart = 152,

    /// UWTROC (153): Underwater rock / awash rock
    #[strum(serialize = "UWTROC")]
    UnderwaterRockAwashRock = 153,

    /// UNSARE (154): Unsurveyed area
    #[strum(serialize = "UNSARE")]
    UnsurveyedArea = 154,

    /// VEGATN (155): Vegetation
    #[strum(serialize = "VEGATN")]
    Vegetation = 155,

    /// WATTUR (156): Water turbulence
    #[strum(serialize = "WATTUR")]
    WaterTurbulence = 156,

    /// WATFAL (157): Waterfall
    #[strum(serialize = "WATFAL")]
    Waterfall = 157,

    /// WEDKLP (158): Weed/Kelp
    #[strum(serialize = "WEDKLP")]
    WeedKelp = 158,

    /// WRECKS (159): Wreck
    #[strum(serialize = "WRECKS")]
    Wreck = 159,

    /// TS_FEB (160): Tidal stream - flood/ebb
    #[strum(serialize = "TS_FEB")]
    TidalStreamFloodebb = 160,

    /// ARCSLN (161): Archipelagix Sea Lane
    #[strum(serialize = "ARCSLN")]
    ArchipelagixSeaLane = 161,

    /// ASLXIS (162): Archipelagix Sea Lane axis
    #[strum(serialize = "ASLXIS")]
    ArchipelagixSeaLaneAxis = 162,

    /// NEWOBJ (163): New object
    #[strum(serialize = "NEWOBJ")]
    NewObject = 163,

    /// M_ACCY (300): Accuracy of data
    #[strum(serialize = "M_ACCY")]
    AccuracyOfData = 300,

    /// M_CSCL (301): Compilation scale of data
    #[strum(serialize = "M_CSCL")]
    CompilationScaleOfData = 301,

    /// M_COVR (302): Coverage
    #[strum(serialize = "M_COVR")]
    Coverage = 302,

    /// M_HDAT (303): Horizontal datum of data
    #[strum(serialize = "M_HDAT")]
    HorizontalDatumOfData = 303,

    /// M_HOPA (304): Horizontal datum shift parameters
    #[strum(serialize = "M_HOPA")]
    HorizontalDatumShiftParameters = 304,

    /// M_NPUB (305): Nautical publication information
    #[strum(serialize = "M_NPUB")]
    NauticalPublicationInformation = 305,

    /// M_NSYS (306): Navigational system of marks
    #[strum(serialize = "M_NSYS")]
    NavigationalSystemOfMarks = 306,

    /// M_PROD (307): Production information
    #[strum(serialize = "M_PROD")]
    ProductionInformation = 307,

    /// M_QUAL (308): Quality of data
    #[strum(serialize = "M_QUAL")]
    QualityOfData = 308,

    /// M_SDAT (309): Sounding datum
    #[strum(serialize = "M_SDAT")]
    SoundingDatum = 309,

    /// M_SREL (310): Survey reliability
    #[strum(serialize = "M_SREL")]
    SurveyReliability = 310,

    /// M_UNIT (311): Units of measurement of data
    #[strum(serialize = "M_UNIT")]
    UnitsOfMeasurementOfData = 311,

    /// M_VDAT (312): Vertical datum of data
    #[strum(serialize = "M_VDAT")]
    VerticalDatumOfData = 312,

    /// C_AGGR (400): Aggregation
    #[strum(serialize = "C_AGGR")]
    Aggregation = 400,

    /// C_ASSO (401): Association
    #[strum(serialize = "C_ASSO")]
    Association = 401,

    /// C_STAC (402): Stacked on/stacked under
    #[strum(serialize = "C_STAC")]
    StackedOnstackedUnder = 402,

    /// $AREAS (500): Cartographic area
    #[strum(serialize = "$AREAS")]
    CartographicArea = 500,

    /// $LINES (501): Cartographic line
    #[strum(serialize = "$LINES")]
    CartographicLine = 501,

    /// $CSYMB (502): Cartographic symbol
    #[strum(serialize = "$CSYMB")]
    CartographicSymbol = 502,

    /// $COMPS (503): Compass
    #[strum(serialize = "$COMPS")]
    Compass = 503,

    /// $TEXTS (504): Text
    #[strum(serialize = "$TEXTS")]
    Text = 504,

    /// achbrt (17000): Anchor berth
    #[strum(serialize = "achbrt")]
    AnchorBerth17000 = 17000,

    /// achare (17001): Anchorage area
    #[strum(serialize = "achare")]
    AnchorageArea17001 = 17001,

    /// canbnk (17002): Canal bank
    #[strum(serialize = "canbnk")]
    CanalBank17002 = 17002,

    /// depare (17003): Depth area
    #[strum(serialize = "depare")]
    DepthArea17003 = 17003,

    /// dismar (17004): Distance mark
    #[strum(serialize = "dismar")]
    DistanceMark17004 = 17004,

    /// resare (17005): Restricted area
    #[strum(serialize = "resare")]
    RestrictedArea17005 = 17005,

    /// rivbnk (17006): River bank
    #[strum(serialize = "rivbnk")]
    RiverBank17006 = 17006,

    /// sistat (17007): Signal station traffic
    #[strum(serialize = "sistat")]
    SignalStationTraffic17007 = 17007,

    /// sistaw (17008): Signal station warning
    #[strum(serialize = "sistaw")]
    SignalStationWarning17008 = 17008,

    /// topmar (17009): Top Mark
    #[strum(serialize = "topmar")]
    TopMark17009 = 17009,

    /// berths (17010): Berth berths
    #[strum(serialize = "berths")]
    BerthBerths17010 = 17010,

    /// bridge (17011): Bridge
    #[strum(serialize = "bridge")]
    Bridge17011 = 17011,

    /// cblohd (17012): Cable overhead
    #[strum(serialize = "cblohd")]
    CableOverhead17012 = 17012,

    /// feryrt (17013): Ferry route
    #[strum(serialize = "feryrt")]
    FerryRoute17013 = 17013,

    /// hrbare (17014): Harbour Area
    #[strum(serialize = "hrbare")]
    HarbourArea17014 = 17014,

    /// hrbfac (17015): Harbour Facilities
    #[strum(serialize = "hrbfac")]
    HarbourFacilities17015 = 17015,

    /// lokbsn (17016): Lock Basin
    #[strum(serialize = "lokbsn")]
    LockBasin17016 = 17016,

    /// rdocal (17017): Radio calling-in point
    #[strum(serialize = "rdocal")]
    RadioCallinginPoint17017 = 17017,

    /// m_nsys (17018): Navigational system of marks
    #[strum(serialize = "m_nsys")]
    NavigationalSystemOfMarks17018 = 17018,

    /// notmrk (17050): Notice mark
    #[strum(serialize = "notmrk")]
    NoticeMark17050 = 17050,

    /// wtwaxs (17051): Waterway axis
    #[strum(serialize = "wtwaxs")]
    WaterwayAxis17051 = 17051,

    /// wtwprf (17052): Waterway profile
    #[strum(serialize = "wtwprf")]
    WaterwayProfile17052 = 17052,

    /// brgare (17053): Bridge area
    #[strum(serialize = "brgare")]
    BridgeArea17053 = 17053,

    /// bunsta (17054): Bunker station
    #[strum(serialize = "bunsta")]
    BunkerStation17054 = 17054,

    /// comare (17055): Communication Area
    #[strum(serialize = "comare")]
    CommunicationArea17055 = 17055,

    /// hrbbsn (17056): Harbour Basin
    #[strum(serialize = "hrbbsn")]
    HarbourBasin17056 = 17056,

    /// lokare (17057): Lock area
    #[strum(serialize = "lokare")]
    LockArea17057 = 17057,

    /// lkbspt (17058): Lock basin part
    #[strum(serialize = "lkbspt")]
    LockBasinPart17058 = 17058,

    /// prtare (17059): Port Area
    #[strum(serialize = "prtare")]
    PortArea17059 = 17059,

    /// bcnwtw (17060): Beacon water-way
    #[strum(serialize = "bcnwtw")]
    BeaconWaterway17060 = 17060,

    /// boywtw (17061): Buoy water-way
    #[strum(serialize = "boywtw")]
    BuoyWaterway17061 = 17061,

    /// refdmp (17062): Refuse dump
    #[strum(serialize = "refdmp")]
    RefuseDump17062 = 17062,

    /// rtplpt (17063): Route planning point
    #[strum(serialize = "rtplpt")]
    RoutePlanningPoint17063 = 17063,

    /// termnl (17064): Terminal
    #[strum(serialize = "termnl")]
    Terminal17064 = 17064,

    /// trnbsn (17065): Turning basin
    #[strum(serialize = "trnbsn")]
    TurningBasin17065 = 17065,

    /// atsctl (20484): ATS Route Centreline
    #[strum(serialize = "atsctl")]
    ATSRouteCentreline20484 = 20484,

    /// airres + catasr (20485): Airspace Restriction
    #[strum(serialize = "airres + catasr")]
    AirspaceRestriction20485 = 20485,

    /// imgare (20486): Area of Imagery Coverage
    #[strum(serialize = "imgare")]
    AreaOfImageryCoverage20486 = 20486,

    /// bchext (20487): Beach Exit
    #[strum(serialize = "bchext")]
    BeachExit20487 = 20487,

    /// bchprf (20488): Beach Profile
    #[strum(serialize = "bchprf")]
    BeachProfile20488 = 20488,

    /// bchare (20489): Beach Survey
    #[strum(serialize = "bchare")]
    BeachSurvey20489 = 20489,

    /// bedare (20490): Bedrock area
    #[strum(serialize = "bedare")]
    BedrockArea20490 = 20490,

    /// botmft + catbot (20491): Bottom Feature
    #[strum(serialize = "botmft + catbot")]
    BottomFeature20491 = 20491,

    /// centre (20492): Centre Line
    #[strum(serialize = "centre")]
    CentreLine20492 = 20492,

    /// histob (20494): Contact History
    #[strum(serialize = "histob")]
    ContactHistory20494 = 20494,

    /// ctlasp + catcas (20495): Controlled airspace
    #[strum(serialize = "ctlasp + catcas")]
    ControlledAirspace20495 = 20495,

    /// divloc (20496): Diving Location
    #[strum(serialize = "divloc")]
    DivingLocation20496 = 20496,

    /// watloc (20497): Drinking Water Location
    #[strum(serialize = "watloc")]
    DrinkingWaterLocation20497 = 20497,

    /// drpzne (20498): Drop Zone
    #[strum(serialize = "drpzne")]
    DropZone20498 = 20498,

    /// envare (20499): Environmentally Sensitive Area
    #[strum(serialize = "envare")]
    EnvironmentallySensitiveArea20499 = 20499,

    /// fshare (20500): Fishing Activity Area
    #[strum(serialize = "fshare")]
    FishingActivityArea20500 = 20500,

    /// iscour (20501): Impact Scour
    #[strum(serialize = "iscour")]
    ImpactScour20501 = 20501,

    /// lngare (20502): Landing Area
    #[strum(serialize = "lngare")]
    LandingArea20502 = 20502,

    /// lndplc (20503): Landing Place
    #[strum(serialize = "lndplc")]
    LandingPlace20503 = 20503,

    /// lndpnt (20504): Landing Point
    #[strum(serialize = "lndpnt")]
    LandingPoint20504 = 20504,

    /// lndste (20505): Landing Site
    #[strum(serialize = "lndste")]
    LandingSite20505 = 20505,

    /// lndstp (20506): Landing Strip
    #[strum(serialize = "lndstp")]
    LandingStrip20506 = 20506,

    /// lndzne (20507): Landing Zone
    #[strum(serialize = "lndzne")]
    LandingZone20507 = 20507,

    /// marman + catmma (20508): Marine management area
    #[strum(serialize = "marman + catmma")]
    MarineManagementArea20508 = 20508,

    /// msiare (20509): Maritime Safety Information area
    #[strum(serialize = "msiare")]
    MaritimeSafetyInformationArea20509 = 20509,

    /// mcmare (20510): MCM Area
    #[strum(serialize = "mcmare")]
    MCMArea20510 = 20510,

    /// mexasp + catmea (20511): Military exercise airspace
    #[strum(serialize = "mexasp + catmea")]
    MilitaryExerciseAirspace20511 = 20511,

    /// patare + catpat (20513): Patrol area
    #[strum(serialize = "patare + catpat")]
    PatrolArea20513 = 20513,

    /// qroute (20514): Q-Route Leg
    #[strum(serialize = "qroute")]
    QRouteLeg20514 = 20514,

    /// rdoare (20515): Radio broadcast area
    #[strum(serialize = "rdoare")]
    RadioBroadcastArea20515 = 20515,

    /// regasp (20516): Regulated airspace
    #[strum(serialize = "regasp")]
    RegulatedAirspace20516 = 20516,

    /// sedlay (20517): Geological Layer
    #[strum(serialize = "sedlay")]
    GeologicalLayer20517 = 20517,

    /// seiare (20518): Seismic Activity Area
    #[strum(serialize = "seiare")]
    SeismicActivityArea20518 = 20518,

    /// senanm (20519): Sensor Anomaly
    #[strum(serialize = "senanm")]
    SensorAnomaly20519 = 20519,

    /// shlloc (20520): Shelter Location
    #[strum(serialize = "shlloc")]
    ShelterLocation20520 = 20520,

    /// seddep (20521): Superficial Sediment Deposits
    #[strum(serialize = "seddep")]
    SuperficialSedimentDeposits20521 = 20521,

    /// trfare (20522): Trafficability Area
    #[strum(serialize = "trfare")]
    TrafficabilityArea20522 = 20522,

    /// twlscr (20523): Trawl Scours
    #[strum(serialize = "twlscr")]
    TrawlScours20523 = 20523,

    /// turnpt (20524): Turning point
    #[strum(serialize = "turnpt")]
    TurningPoint20524 = 20524,

    /// viewpt (20525): Viewpoint
    #[strum(serialize = "viewpt")]
    Viewpoint20525 = 20525,

    /// btdare (20526): Bottom Tactical Data Area
    #[strum(serialize = "btdare")]
    BottomTacticalDataArea20526 = 20526,

    /// bprare (20527): Burial Probability Area
    #[strum(serialize = "bprare")]
    BurialProbabilityArea20527 = 20527,

    /// lsrare (20528): Leisure Activity Area
    #[strum(serialize = "lsrare")]
    LeisureActivityArea20528 = 20528,

    /// pfdare (20529): Performance Data Area
    #[strum(serialize = "pfdare")]
    PerformanceDataArea20529 = 20529,

    /// resloc (20530): Resource Location
    #[strum(serialize = "resloc")]
    ResourceLocation20530 = 20530,

    /// rkdare (20531): Risk Data Area
    #[strum(serialize = "rkdare")]
    RiskDataArea20531 = 20531,

    /// navaid + CATROS (20532): Navigation system (NAVAID)
    #[strum(serialize = "navaid + CATROS")]
    NavigationSystemNAVAID20532 = 20532,

    /// intwtr (20533): Internal Waters Area
    #[strum(serialize = "intwtr")]
    InternalWatersArea20533 = 20533,

    /// seaice (20534): Sea Ice
    #[strum(serialize = "seaice")]
    SeaIce20534 = 20534,

    /// iceadv (20535): Ice Advisory Area
    #[strum(serialize = "iceadv")]
    IceAdvisoryArea20535 = 20535,

    /// brgare (20536): Iceberg Area
    #[strum(serialize = "brgare")]
    IcebergArea20536 = 20536,

    /// lndice (20537): Land Ice
    #[strum(serialize = "lndice")]
    LandIce20537 = 20537,

    /// icelin (20538): Ice Line
    #[strum(serialize = "icelin")]
    IceLine20538 = 20538,

    /// icerte (20539): Ice Route
    #[strum(serialize = "icerte")]
    IceRoute20539 = 20539,

    /// icepol (20540): Ice Polynya
    #[strum(serialize = "icepol")]
    IcePolynya20540 = 20540,

    /// icelea (20541): Ice Lead
    #[strum(serialize = "icelea")]
    IceLead20541 = 20541,

    /// icebrg (20542): Iceberg
    #[strum(serialize = "icebrg")]
    Iceberg20542 = 20542,

    /// icemov (20543): Ice Movement
    #[strum(serialize = "icemov")]
    IceMovement20543 = 20543,

    /// tfcrte (20544): Traffic route
    #[strum(serialize = "tfcrte")]
    TrafficRoute20544 = 20544,

    /// u_defd (20717): User Defined
    #[strum(serialize = "u_defd")]
    UserDefined20717 = 20717,

    /// smalbo (20718): Small Bottom Object
    #[strum(serialize = "smalbo")]
    SmallBottomObject20718 = 20718,

    /// m_conf + catcnf (21484): Completeness for the product specification
    #[strum(serialize = "m_conf + catcnf")]
    CompletenessForTheProductSpecification21484 = 21484,

    /// m_clas (21485): Security Classification Information
    #[strum(serialize = "m_clas")]
    SecurityClassificationInformation21485 = 21485,

    /// m_vers (21486): Vertical Datum Shift Area
    #[strum(serialize = "m_vers")]
    VerticalDatumShiftArea21486 = 21486,

    /// m_line (21487): Defined Straight Lines
    #[strum(serialize = "m_line")]
    DefinedStraightLines21487 = 21487,

    /// Unknown object class
    Unknown,
}

impl ObjectClass {
    /// Decode object class from OBJL code
    pub fn from_code(objl: u16) -> Option<Self> {
        use ObjectClass::*;
        let class = match objl {
            1 => AdministrationAreaNamed,
            2 => AirportAirfield,
            3 => AnchorBerth,
            4 => AnchorageArea,
            5 => BeaconCardinal,
            6 => BeaconIsolatedDanger,
            7 => BeaconLateral,
            8 => BeaconSafeWater,
            9 => BeaconSpecialPurposegeneral,
            10 => Berth,
            11 => Bridge,
            12 => BuildingSingle,
            13 => BuiltupArea,
            14 => BuoyCardinal,
            15 => BuoyInstallation,
            16 => BuoyIsolatedDanger,
            17 => BuoyLateral,
            18 => BuoySafeWater,
            19 => BuoySpecialPurposegeneral,
            20 => CableArea,
            21 => CableOverhead,
            22 => CableSubmarine,
            23 => Canal,
            24 => CanalBank,
            25 => CargoTransshipmentArea,
            26 => Causeway,
            27 => CautionArea,
            28 => Checkpoint,
            29 => CoastguardStation,
            30 => Coastline,
            31 => ContiguousZone,
            32 => ContinentalShelfArea,
            33 => ControlPoint,
            34 => Conveyor,
            35 => Crane,
            36 => CurrentNonGravitational,
            37 => CustomZone,
            38 => Dam,
            39 => Daymark,
            40 => DeepWaterRouteCenterline,
            41 => DeepWaterRoutePart,
            42 => DepthArea,
            43 => DepthContour,
            44 => DistanceMark,
            45 => DockArea,
            46 => DredgedArea,
            47 => DryDock,
            48 => DumpingGround,
            49 => Dyke,
            50 => ExclusiveEconomicZone,
            51 => Fairway,
            52 => Fencewall,
            53 => FerryRoute,
            54 => FisheryZone,
            55 => FishingFacility,
            56 => FishingGround,
            57 => FloatingDock,
            58 => FogSignal,
            59 => FortifiedStructure,
            60 => FreePortArea,
            61 => Gate,
            62 => Gridiron,
            63 => HarbourAreaAdministrative,
            64 => HarbourFacility,
            65 => Hulk,
            66 => IceArea,
            67 => IncinerationArea,
            68 => InshoreTrafficZone,
            69 => Lake,
            70 => LakeShore,
            71 => LandArea,
            72 => LandElevation,
            73 => LandRegion,
            74 => Landmark,
            75 => Light,
            76 => LightFloat,
            77 => LightVessel,
            78 => LocalMagneticAnomaly,
            79 => LockBasin,
            80 => LogPond,
            81 => MagneticVariation,
            82 => MarineFarmculture,
            83 => MilitaryPracticeArea,
            84 => MooringwarpingFacility,
            85 => NavigationLine,
            86 => Obstruction,
            87 => OffshorePlatform,
            88 => OffshoreProductionArea,
            89 => OilBarrier,
            90 => Pile,
            91 => PilotBoardingPlace,
            92 => PipelineArea,
            93 => PipelineOverhead,
            94 => PipelineSubmarineonLand,
            95 => Pontoon,
            96 => PrecautionaryArea,
            97 => ProductionStorageArea,
            98 => PylonbridgeSupport,
            99 => RadarLine,
            100 => RadarRange,
            101 => RadarReflector,
            102 => RadarStation,
            103 => RadarTransponderBeacon,
            104 => RadioCallinginPoint,
            105 => RadioStation,
            106 => Railway,
            107 => Rapids,
            108 => RecommendedRouteCenterline,
            109 => RecommendedTrack,
            110 => RecommendedTrafficLanePart,
            111 => RescueStation,
            112 => RestrictedArea,
            113 => Retroreflector,
            114 => River,
            115 => RiverBank,
            116 => Road,
            117 => Runway,
            118 => SandWaves,
            119 => SeaAreaNamedWaterArea,
            120 => SeaplaneLandingArea,
            121 => SeabedArea,
            122 => ShorelineConstruction,
            123 => SignalStationTraffic,
            124 => SignalStationWarning,
            125 => SiloTank,
            126 => SlopeTopline,
            127 => SlopingGround,
            128 => SmallCraftFacility,
            129 => Sounding,
            130 => Spring,
            131 => Square,
            132 => StraightTerritorialSeaBaseline,
            133 => SubmarineTransitLane,
            134 => SweptArea,
            135 => TerritorialSeaArea,
            136 => TidalStreamHarmonicPrediction,
            137 => TidalStreamNonharmonicPrediction,
            138 => TidalStreamPanelData,
            139 => TidalStreamTimeSeriesTS_TIS,
            140 => TideHarmonicPrediction,
            141 => TideNonharmonicPrediction,
            142 => TidalStreamTimeSeriesT_TIMS,
            143 => Tideway,
            144 => TopMark,
            145 => TrafficSeparationLine,
            146 => TrafficSeparationSchemeBoundary,
            147 => TrafficSeparationSchemeCrossing,
            148 => TrafficSeparationSchemeLanePart,
            149 => TrafficSeparationSchemeRoundabout,
            150 => TrafficSeparationZone,
            151 => Tunnel,
            152 => TwowayRoutePart,
            153 => UnderwaterRockAwashRock,
            154 => UnsurveyedArea,
            155 => Vegetation,
            156 => WaterTurbulence,
            157 => Waterfall,
            158 => WeedKelp,
            159 => Wreck,
            160 => TidalStreamFloodebb,
            161 => ArchipelagixSeaLane,
            162 => ArchipelagixSeaLaneAxis,
            163 => NewObject,
            300 => AccuracyOfData,
            301 => CompilationScaleOfData,
            302 => Coverage,
            303 => HorizontalDatumOfData,
            304 => HorizontalDatumShiftParameters,
            305 => NauticalPublicationInformation,
            306 => NavigationalSystemOfMarks,
            307 => ProductionInformation,
            308 => QualityOfData,
            309 => SoundingDatum,
            310 => SurveyReliability,
            311 => UnitsOfMeasurementOfData,
            312 => VerticalDatumOfData,
            400 => Aggregation,
            401 => Association,
            402 => StackedOnstackedUnder,
            500 => CartographicArea,
            501 => CartographicLine,
            502 => CartographicSymbol,
            503 => Compass,
            504 => Text,
            17000 => AnchorBerth17000,
            17001 => AnchorageArea17001,
            17002 => CanalBank17002,
            17003 => DepthArea17003,
            17004 => DistanceMark17004,
            17005 => RestrictedArea17005,
            17006 => RiverBank17006,
            17007 => SignalStationTraffic17007,
            17008 => SignalStationWarning17008,
            17009 => TopMark17009,
            17010 => BerthBerths17010,
            17011 => Bridge17011,
            17012 => CableOverhead17012,
            17013 => FerryRoute17013,
            17014 => HarbourArea17014,
            17015 => HarbourFacilities17015,
            17016 => LockBasin17016,
            17017 => RadioCallinginPoint17017,
            17018 => NavigationalSystemOfMarks17018,
            17050 => NoticeMark17050,
            17051 => WaterwayAxis17051,
            17052 => WaterwayProfile17052,
            17053 => BridgeArea17053,
            17054 => BunkerStation17054,
            17055 => CommunicationArea17055,
            17056 => HarbourBasin17056,
            17057 => LockArea17057,
            17058 => LockBasinPart17058,
            17059 => PortArea17059,
            17060 => BeaconWaterway17060,
            17061 => BuoyWaterway17061,
            17062 => RefuseDump17062,
            17063 => RoutePlanningPoint17063,
            17064 => Terminal17064,
            17065 => TurningBasin17065,
            20484 => ATSRouteCentreline20484,
            20485 => AirspaceRestriction20485,
            20486 => AreaOfImageryCoverage20486,
            20487 => BeachExit20487,
            20488 => BeachProfile20488,
            20489 => BeachSurvey20489,
            20490 => BedrockArea20490,
            20491 => BottomFeature20491,
            20492 => CentreLine20492,
            20494 => ContactHistory20494,
            20495 => ControlledAirspace20495,
            20496 => DivingLocation20496,
            20497 => DrinkingWaterLocation20497,
            20498 => DropZone20498,
            20499 => EnvironmentallySensitiveArea20499,
            20500 => FishingActivityArea20500,
            20501 => ImpactScour20501,
            20502 => LandingArea20502,
            20503 => LandingPlace20503,
            20504 => LandingPoint20504,
            20505 => LandingSite20505,
            20506 => LandingStrip20506,
            20507 => LandingZone20507,
            20508 => MarineManagementArea20508,
            20509 => MaritimeSafetyInformationArea20509,
            20510 => MCMArea20510,
            20511 => MilitaryExerciseAirspace20511,
            20513 => PatrolArea20513,
            20514 => QRouteLeg20514,
            20515 => RadioBroadcastArea20515,
            20516 => RegulatedAirspace20516,
            20517 => GeologicalLayer20517,
            20518 => SeismicActivityArea20518,
            20519 => SensorAnomaly20519,
            20520 => ShelterLocation20520,
            20521 => SuperficialSedimentDeposits20521,
            20522 => TrafficabilityArea20522,
            20523 => TrawlScours20523,
            20524 => TurningPoint20524,
            20525 => Viewpoint20525,
            20526 => BottomTacticalDataArea20526,
            20527 => BurialProbabilityArea20527,
            20528 => LeisureActivityArea20528,
            20529 => PerformanceDataArea20529,
            20530 => ResourceLocation20530,
            20531 => RiskDataArea20531,
            20532 => NavigationSystemNAVAID20532,
            20533 => InternalWatersArea20533,
            20534 => SeaIce20534,
            20535 => IceAdvisoryArea20535,
            20536 => IcebergArea20536,
            20537 => LandIce20537,
            20538 => IceLine20538,
            20539 => IceRoute20539,
            20540 => IcePolynya20540,
            20541 => IceLead20541,
            20542 => Iceberg20542,
            20543 => IceMovement20543,
            20544 => TrafficRoute20544,
            20717 => UserDefined20717,
            20718 => SmallBottomObject20718,
            21484 => CompletenessForTheProductSpecification21484,
            21485 => SecurityClassificationInformation21485,
            21486 => VerticalDatumShiftArea21486,
            21487 => DefinedStraightLines21487,
            _ => return None,
        };
        Some(class)
    }

    /// Get OBJL code for this object class
    pub fn code(&self) -> u16 {
        match self {
            Self::AdministrationAreaNamed => 1,
            Self::AirportAirfield => 2,
            Self::AnchorBerth => 3,
            Self::AnchorageArea => 4,
            Self::BeaconCardinal => 5,
            Self::BeaconIsolatedDanger => 6,
            Self::BeaconLateral => 7,
            Self::BeaconSafeWater => 8,
            Self::BeaconSpecialPurposegeneral => 9,
            Self::Berth => 10,
            Self::Bridge => 11,
            Self::BuildingSingle => 12,
            Self::BuiltupArea => 13,
            Self::BuoyCardinal => 14,
            Self::BuoyInstallation => 15,
            Self::BuoyIsolatedDanger => 16,
            Self::BuoyLateral => 17,
            Self::BuoySafeWater => 18,
            Self::BuoySpecialPurposegeneral => 19,
            Self::CableArea => 20,
            Self::CableOverhead => 21,
            Self::CableSubmarine => 22,
            Self::Canal => 23,
            Self::CanalBank => 24,
            Self::CargoTransshipmentArea => 25,
            Self::Causeway => 26,
            Self::CautionArea => 27,
            Self::Checkpoint => 28,
            Self::CoastguardStation => 29,
            Self::Coastline => 30,
            Self::ContiguousZone => 31,
            Self::ContinentalShelfArea => 32,
            Self::ControlPoint => 33,
            Self::Conveyor => 34,
            Self::Crane => 35,
            Self::CurrentNonGravitational => 36,
            Self::CustomZone => 37,
            Self::Dam => 38,
            Self::Daymark => 39,
            Self::DeepWaterRouteCenterline => 40,
            Self::DeepWaterRoutePart => 41,
            Self::DepthArea => 42,
            Self::DepthContour => 43,
            Self::DistanceMark => 44,
            Self::DockArea => 45,
            Self::DredgedArea => 46,
            Self::DryDock => 47,
            Self::DumpingGround => 48,
            Self::Dyke => 49,
            Self::ExclusiveEconomicZone => 50,
            Self::Fairway => 51,
            Self::Fencewall => 52,
            Self::FerryRoute => 53,
            Self::FisheryZone => 54,
            Self::FishingFacility => 55,
            Self::FishingGround => 56,
            Self::FloatingDock => 57,
            Self::FogSignal => 58,
            Self::FortifiedStructure => 59,
            Self::FreePortArea => 60,
            Self::Gate => 61,
            Self::Gridiron => 62,
            Self::HarbourAreaAdministrative => 63,
            Self::HarbourFacility => 64,
            Self::Hulk => 65,
            Self::IceArea => 66,
            Self::IncinerationArea => 67,
            Self::InshoreTrafficZone => 68,
            Self::Lake => 69,
            Self::LakeShore => 70,
            Self::LandArea => 71,
            Self::LandElevation => 72,
            Self::LandRegion => 73,
            Self::Landmark => 74,
            Self::Light => 75,
            Self::LightFloat => 76,
            Self::LightVessel => 77,
            Self::LocalMagneticAnomaly => 78,
            Self::LockBasin => 79,
            Self::LogPond => 80,
            Self::MagneticVariation => 81,
            Self::MarineFarmculture => 82,
            Self::MilitaryPracticeArea => 83,
            Self::MooringwarpingFacility => 84,
            Self::NavigationLine => 85,
            Self::Obstruction => 86,
            Self::OffshorePlatform => 87,
            Self::OffshoreProductionArea => 88,
            Self::OilBarrier => 89,
            Self::Pile => 90,
            Self::PilotBoardingPlace => 91,
            Self::PipelineArea => 92,
            Self::PipelineOverhead => 93,
            Self::PipelineSubmarineonLand => 94,
            Self::Pontoon => 95,
            Self::PrecautionaryArea => 96,
            Self::ProductionStorageArea => 97,
            Self::PylonbridgeSupport => 98,
            Self::RadarLine => 99,
            Self::RadarRange => 100,
            Self::RadarReflector => 101,
            Self::RadarStation => 102,
            Self::RadarTransponderBeacon => 103,
            Self::RadioCallinginPoint => 104,
            Self::RadioStation => 105,
            Self::Railway => 106,
            Self::Rapids => 107,
            Self::RecommendedRouteCenterline => 108,
            Self::RecommendedTrack => 109,
            Self::RecommendedTrafficLanePart => 110,
            Self::RescueStation => 111,
            Self::RestrictedArea => 112,
            Self::Retroreflector => 113,
            Self::River => 114,
            Self::RiverBank => 115,
            Self::Road => 116,
            Self::Runway => 117,
            Self::SandWaves => 118,
            Self::SeaAreaNamedWaterArea => 119,
            Self::SeaplaneLandingArea => 120,
            Self::SeabedArea => 121,
            Self::ShorelineConstruction => 122,
            Self::SignalStationTraffic => 123,
            Self::SignalStationWarning => 124,
            Self::SiloTank => 125,
            Self::SlopeTopline => 126,
            Self::SlopingGround => 127,
            Self::SmallCraftFacility => 128,
            Self::Sounding => 129,
            Self::Spring => 130,
            Self::Square => 131,
            Self::StraightTerritorialSeaBaseline => 132,
            Self::SubmarineTransitLane => 133,
            Self::SweptArea => 134,
            Self::TerritorialSeaArea => 135,
            Self::TidalStreamHarmonicPrediction => 136,
            Self::TidalStreamNonharmonicPrediction => 137,
            Self::TidalStreamPanelData => 138,
            Self::TidalStreamTimeSeriesTS_TIS => 139,
            Self::TideHarmonicPrediction => 140,
            Self::TideNonharmonicPrediction => 141,
            Self::TidalStreamTimeSeriesT_TIMS => 142,
            Self::Tideway => 143,
            Self::TopMark => 144,
            Self::TrafficSeparationLine => 145,
            Self::TrafficSeparationSchemeBoundary => 146,
            Self::TrafficSeparationSchemeCrossing => 147,
            Self::TrafficSeparationSchemeLanePart => 148,
            Self::TrafficSeparationSchemeRoundabout => 149,
            Self::TrafficSeparationZone => 150,
            Self::Tunnel => 151,
            Self::TwowayRoutePart => 152,
            Self::UnderwaterRockAwashRock => 153,
            Self::UnsurveyedArea => 154,
            Self::Vegetation => 155,
            Self::WaterTurbulence => 156,
            Self::Waterfall => 157,
            Self::WeedKelp => 158,
            Self::Wreck => 159,
            Self::TidalStreamFloodebb => 160,
            Self::ArchipelagixSeaLane => 161,
            Self::ArchipelagixSeaLaneAxis => 162,
            Self::NewObject => 163,
            Self::AccuracyOfData => 300,
            Self::CompilationScaleOfData => 301,
            Self::Coverage => 302,
            Self::HorizontalDatumOfData => 303,
            Self::HorizontalDatumShiftParameters => 304,
            Self::NauticalPublicationInformation => 305,
            Self::NavigationalSystemOfMarks => 306,
            Self::ProductionInformation => 307,
            Self::QualityOfData => 308,
            Self::SoundingDatum => 309,
            Self::SurveyReliability => 310,
            Self::UnitsOfMeasurementOfData => 311,
            Self::VerticalDatumOfData => 312,
            Self::Aggregation => 400,
            Self::Association => 401,
            Self::StackedOnstackedUnder => 402,
            Self::CartographicArea => 500,
            Self::CartographicLine => 501,
            Self::CartographicSymbol => 502,
            Self::Compass => 503,
            Self::Text => 504,
            Self::AnchorBerth17000 => 17000,
            Self::AnchorageArea17001 => 17001,
            Self::CanalBank17002 => 17002,
            Self::DepthArea17003 => 17003,
            Self::DistanceMark17004 => 17004,
            Self::RestrictedArea17005 => 17005,
            Self::RiverBank17006 => 17006,
            Self::SignalStationTraffic17007 => 17007,
            Self::SignalStationWarning17008 => 17008,
            Self::TopMark17009 => 17009,
            Self::BerthBerths17010 => 17010,
            Self::Bridge17011 => 17011,
            Self::CableOverhead17012 => 17012,
            Self::FerryRoute17013 => 17013,
            Self::HarbourArea17014 => 17014,
            Self::HarbourFacilities17015 => 17015,
            Self::LockBasin17016 => 17016,
            Self::RadioCallinginPoint17017 => 17017,
            Self::NavigationalSystemOfMarks17018 => 17018,
            Self::NoticeMark17050 => 17050,
            Self::WaterwayAxis17051 => 17051,
            Self::WaterwayProfile17052 => 17052,
            Self::BridgeArea17053 => 17053,
            Self::BunkerStation17054 => 17054,
            Self::CommunicationArea17055 => 17055,
            Self::HarbourBasin17056 => 17056,
            Self::LockArea17057 => 17057,
            Self::LockBasinPart17058 => 17058,
            Self::PortArea17059 => 17059,
            Self::BeaconWaterway17060 => 17060,
            Self::BuoyWaterway17061 => 17061,
            Self::RefuseDump17062 => 17062,
            Self::RoutePlanningPoint17063 => 17063,
            Self::Terminal17064 => 17064,
            Self::TurningBasin17065 => 17065,
            Self::ATSRouteCentreline20484 => 20484,
            Self::AirspaceRestriction20485 => 20485,
            Self::AreaOfImageryCoverage20486 => 20486,
            Self::BeachExit20487 => 20487,
            Self::BeachProfile20488 => 20488,
            Self::BeachSurvey20489 => 20489,
            Self::BedrockArea20490 => 20490,
            Self::BottomFeature20491 => 20491,
            Self::CentreLine20492 => 20492,
            Self::ContactHistory20494 => 20494,
            Self::ControlledAirspace20495 => 20495,
            Self::DivingLocation20496 => 20496,
            Self::DrinkingWaterLocation20497 => 20497,
            Self::DropZone20498 => 20498,
            Self::EnvironmentallySensitiveArea20499 => 20499,
            Self::FishingActivityArea20500 => 20500,
            Self::ImpactScour20501 => 20501,
            Self::LandingArea20502 => 20502,
            Self::LandingPlace20503 => 20503,
            Self::LandingPoint20504 => 20504,
            Self::LandingSite20505 => 20505,
            Self::LandingStrip20506 => 20506,
            Self::LandingZone20507 => 20507,
            Self::MarineManagementArea20508 => 20508,
            Self::MaritimeSafetyInformationArea20509 => 20509,
            Self::MCMArea20510 => 20510,
            Self::MilitaryExerciseAirspace20511 => 20511,
            Self::PatrolArea20513 => 20513,
            Self::QRouteLeg20514 => 20514,
            Self::RadioBroadcastArea20515 => 20515,
            Self::RegulatedAirspace20516 => 20516,
            Self::GeologicalLayer20517 => 20517,
            Self::SeismicActivityArea20518 => 20518,
            Self::SensorAnomaly20519 => 20519,
            Self::ShelterLocation20520 => 20520,
            Self::SuperficialSedimentDeposits20521 => 20521,
            Self::TrafficabilityArea20522 => 20522,
            Self::TrawlScours20523 => 20523,
            Self::TurningPoint20524 => 20524,
            Self::Viewpoint20525 => 20525,
            Self::BottomTacticalDataArea20526 => 20526,
            Self::BurialProbabilityArea20527 => 20527,
            Self::LeisureActivityArea20528 => 20528,
            Self::PerformanceDataArea20529 => 20529,
            Self::ResourceLocation20530 => 20530,
            Self::RiskDataArea20531 => 20531,
            Self::NavigationSystemNAVAID20532 => 20532,
            Self::InternalWatersArea20533 => 20533,
            Self::SeaIce20534 => 20534,
            Self::IceAdvisoryArea20535 => 20535,
            Self::IcebergArea20536 => 20536,
            Self::LandIce20537 => 20537,
            Self::IceLine20538 => 20538,
            Self::IceRoute20539 => 20539,
            Self::IcePolynya20540 => 20540,
            Self::IceLead20541 => 20541,
            Self::Iceberg20542 => 20542,
            Self::IceMovement20543 => 20543,
            Self::TrafficRoute20544 => 20544,
            Self::UserDefined20717 => 20717,
            Self::SmallBottomObject20718 => 20718,
            Self::CompletenessForTheProductSpecification21484 => 21484,
            Self::SecurityClassificationInformation21485 => 21485,
            Self::VerticalDatumShiftArea21486 => 21486,
            Self::DefinedStraightLines21487 => 21487,
            Self::Unknown => 0,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        use ObjectClass::*;
        match self {
            AdministrationAreaNamed => "Administration area (Named)",
            AirportAirfield => "Airport / airfield",
            AnchorBerth => "Anchor berth",
            AnchorageArea => "Anchorage area",
            BeaconCardinal => "Beacon, cardinal",
            BeaconIsolatedDanger => "Beacon, isolated danger",
            BeaconLateral => "Beacon, lateral",
            BeaconSafeWater => "Beacon, safe water",
            BeaconSpecialPurposegeneral => "Beacon, special purpose/general",
            Berth => "Berth",
            Bridge => "Bridge",
            BuildingSingle => "Building, single",
            BuiltupArea => "Built-up area",
            BuoyCardinal => "Buoy, cardinal",
            BuoyInstallation => "Buoy, installation",
            BuoyIsolatedDanger => "Buoy, isolated danger",
            BuoyLateral => "Buoy, lateral",
            BuoySafeWater => "Buoy, safe water",
            BuoySpecialPurposegeneral => "Buoy, special purpose/general",
            CableArea => "Cable area",
            CableOverhead => "Cable, overhead",
            CableSubmarine => "Cable, submarine",
            Canal => "Canal",
            CanalBank => "Canal bank",
            CargoTransshipmentArea => "Cargo transshipment area",
            Causeway => "Causeway",
            CautionArea => "Caution area",
            Checkpoint => "Checkpoint",
            CoastguardStation => "Coastguard station",
            Coastline => "Coastline",
            ContiguousZone => "Contiguous zone",
            ContinentalShelfArea => "Continental shelf area",
            ControlPoint => "Control point",
            Conveyor => "Conveyor",
            Crane => "Crane",
            CurrentNonGravitational => "Current - non - gravitational",
            CustomZone => "Custom zone",
            Dam => "Dam",
            Daymark => "Daymark",
            DeepWaterRouteCenterline => "Deep water route centerline",
            DeepWaterRoutePart => "Deep water route part",
            DepthArea => "Depth area",
            DepthContour => "Depth contour",
            DistanceMark => "Distance mark",
            DockArea => "Dock area",
            DredgedArea => "Dredged area",
            DryDock => "Dry dock",
            DumpingGround => "Dumping ground",
            Dyke => "Dyke",
            ExclusiveEconomicZone => "Exclusive Economic Zone",
            Fairway => "Fairway",
            Fencewall => "Fence/wall",
            FerryRoute => "Ferry route",
            FisheryZone => "Fishery zone",
            FishingFacility => "Fishing facility",
            FishingGround => "Fishing ground",
            FloatingDock => "Floating dock",
            FogSignal => "Fog signal",
            FortifiedStructure => "Fortified structure",
            FreePortArea => "Free port area",
            Gate => "Gate",
            Gridiron => "Gridiron",
            HarbourAreaAdministrative => "Harbour area (administrative)",
            HarbourFacility => "Harbour facility",
            Hulk => "Hulk",
            IceArea => "Ice area",
            IncinerationArea => "Incineration area",
            InshoreTrafficZone => "Inshore traffic zone",
            Lake => "Lake",
            LakeShore => "Lake shore",
            LandArea => "Land area",
            LandElevation => "Land elevation",
            LandRegion => "Land region",
            Landmark => "Landmark",
            Light => "Light",
            LightFloat => "Light float",
            LightVessel => "Light vessel",
            LocalMagneticAnomaly => "Local magnetic anomaly",
            LockBasin => "Lock basin",
            LogPond => "Log pond",
            MagneticVariation => "Magnetic variation",
            MarineFarmculture => "Marine farm/culture",
            MilitaryPracticeArea => "Military practice area",
            MooringwarpingFacility => "Mooring/warping facility",
            NavigationLine => "Navigation line",
            Obstruction => "Obstruction",
            OffshorePlatform => "Offshore platform",
            OffshoreProductionArea => "Offshore production area",
            OilBarrier => "Oil barrier",
            Pile => "Pile",
            PilotBoardingPlace => "Pilot boarding place",
            PipelineArea => "Pipeline area",
            PipelineOverhead => "Pipeline, overhead",
            PipelineSubmarineonLand => "Pipeline, submarine/on land",
            Pontoon => "Pontoon",
            PrecautionaryArea => "Precautionary area",
            ProductionStorageArea => "Production / storage area",
            PylonbridgeSupport => "Pylon/bridge support",
            RadarLine => "Radar line",
            RadarRange => "Radar range",
            RadarReflector => "Radar reflector",
            RadarStation => "Radar station",
            RadarTransponderBeacon => "Radar transponder beacon",
            RadioCallinginPoint => "Radio calling-in point",
            RadioStation => "Radio station",
            Railway => "Railway",
            Rapids => "Rapids",
            RecommendedRouteCenterline => "Recommended route centerline",
            RecommendedTrack => "Recommended track",
            RecommendedTrafficLanePart => "Recommended Traffic Lane Part",
            RescueStation => "Rescue station",
            RestrictedArea => "Restricted area",
            Retroreflector => "Retro-reflector",
            River => "River",
            RiverBank => "River bank",
            Road => "Road",
            Runway => "Runway",
            SandWaves => "Sand waves",
            SeaAreaNamedWaterArea => "Sea area / named water area",
            SeaplaneLandingArea => "Sea-plane landing area",
            SeabedArea => "Seabed area",
            ShorelineConstruction => "Shoreline Construction",
            SignalStationTraffic => "Signal station, traffic",
            SignalStationWarning => "Signal station, warning",
            SiloTank => "Silo / tank",
            SlopeTopline => "Slope topline",
            SlopingGround => "Sloping ground",
            SmallCraftFacility => "Small craft facility",
            Sounding => "Sounding",
            Spring => "Spring",
            Square => "Square",
            StraightTerritorialSeaBaseline => "Straight territorial sea baseline",
            SubmarineTransitLane => "Submarine transit lane",
            SweptArea => "Swept Area",
            TerritorialSeaArea => "Territorial sea area",
            TidalStreamHarmonicPrediction => "Tidal stream - harmonic prediction",
            TidalStreamNonharmonicPrediction => "Tidal stream - non-harmonic prediction",
            TidalStreamPanelData => "Tidal stream panel data",
            TidalStreamTimeSeriesTS_TIS => "Tidal stream - time series (TS_TIS)",
            TideHarmonicPrediction => "Tide - harmonic prediction",
            TideNonharmonicPrediction => "Tide - non-harmonic prediction",
            TidalStreamTimeSeriesT_TIMS => "Tidal stream - time series (T_TIMS)",
            Tideway => "Tideway",
            TopMark => "Top mark",
            TrafficSeparationLine => "Traffic Separation Line",
            TrafficSeparationSchemeBoundary => "Traffic Separation Scheme  Boundary",
            TrafficSeparationSchemeCrossing => "Traffic Separation Scheme Crossing",
            TrafficSeparationSchemeLanePart => "Traffic Separation Scheme  Lane part",
            TrafficSeparationSchemeRoundabout => "Traffic Separation Scheme  Roundabout",
            TrafficSeparationZone => "Traffic Separation Zone",
            Tunnel => "Tunnel",
            TwowayRoutePart => "Two-way route  part",
            UnderwaterRockAwashRock => "Underwater rock / awash rock",
            UnsurveyedArea => "Unsurveyed area",
            Vegetation => "Vegetation",
            WaterTurbulence => "Water turbulence",
            Waterfall => "Waterfall",
            WeedKelp => "Weed/Kelp",
            Wreck => "Wreck",
            TidalStreamFloodebb => "Tidal stream - flood/ebb",
            ArchipelagixSeaLane => "Archipelagix Sea Lane",
            ArchipelagixSeaLaneAxis => "Archipelagix Sea Lane axis",
            NewObject => "New object",
            AccuracyOfData => "Accuracy of data",
            CompilationScaleOfData => "Compilation scale of data",
            Coverage => "Coverage",
            HorizontalDatumOfData => "Horizontal datum of data",
            HorizontalDatumShiftParameters => "Horizontal datum shift parameters",
            NauticalPublicationInformation => "Nautical publication information",
            NavigationalSystemOfMarks => "Navigational system of marks",
            ProductionInformation => "Production information",
            QualityOfData => "Quality of data",
            SoundingDatum => "Sounding datum",
            SurveyReliability => "Survey reliability",
            UnitsOfMeasurementOfData => "Units of measurement of data",
            VerticalDatumOfData => "Vertical datum of data",
            Aggregation => "Aggregation",
            Association => "Association",
            StackedOnstackedUnder => "Stacked on/stacked under",
            CartographicArea => "Cartographic area",
            CartographicLine => "Cartographic line",
            CartographicSymbol => "Cartographic symbol",
            Compass => "Compass",
            Text => "Text",
            AnchorBerth17000 => "Anchor berth",
            AnchorageArea17001 => "Anchorage area",
            CanalBank17002 => "Canal bank",
            DepthArea17003 => "Depth area",
            DistanceMark17004 => "Distance mark",
            RestrictedArea17005 => "Restricted area",
            RiverBank17006 => "River bank",
            SignalStationTraffic17007 => "Signal station traffic",
            SignalStationWarning17008 => "Signal station warning",
            TopMark17009 => "Top Mark",
            BerthBerths17010 => "Berth berths",
            Bridge17011 => "Bridge",
            CableOverhead17012 => "Cable overhead",
            FerryRoute17013 => "Ferry route",
            HarbourArea17014 => "Harbour Area",
            HarbourFacilities17015 => "Harbour Facilities",
            LockBasin17016 => "Lock Basin",
            RadioCallinginPoint17017 => "Radio calling-in point",
            NavigationalSystemOfMarks17018 => "Navigational system of marks",
            NoticeMark17050 => "Notice mark",
            WaterwayAxis17051 => "Waterway axis",
            WaterwayProfile17052 => "Waterway profile",
            BridgeArea17053 => "Bridge area",
            BunkerStation17054 => "Bunker station",
            CommunicationArea17055 => "Communication Area",
            HarbourBasin17056 => "Harbour Basin",
            LockArea17057 => "Lock area",
            LockBasinPart17058 => "Lock basin part",
            PortArea17059 => "Port Area",
            BeaconWaterway17060 => "Beacon water-way",
            BuoyWaterway17061 => "Buoy water-way",
            RefuseDump17062 => "Refuse dump",
            RoutePlanningPoint17063 => "Route planning point",
            Terminal17064 => "Terminal",
            TurningBasin17065 => "Turning basin",
            ATSRouteCentreline20484 => "ATS Route Centreline",
            AirspaceRestriction20485 => "Airspace Restriction",
            AreaOfImageryCoverage20486 => "Area of Imagery Coverage",
            BeachExit20487 => "Beach Exit",
            BeachProfile20488 => "Beach Profile",
            BeachSurvey20489 => "Beach Survey",
            BedrockArea20490 => "Bedrock area",
            BottomFeature20491 => "Bottom Feature",
            CentreLine20492 => "Centre Line",
            ContactHistory20494 => "Contact History",
            ControlledAirspace20495 => "Controlled airspace",
            DivingLocation20496 => "Diving Location",
            DrinkingWaterLocation20497 => "Drinking Water Location",
            DropZone20498 => "Drop Zone",
            EnvironmentallySensitiveArea20499 => "Environmentally Sensitive Area",
            FishingActivityArea20500 => "Fishing Activity Area",
            ImpactScour20501 => "Impact Scour",
            LandingArea20502 => "Landing Area",
            LandingPlace20503 => "Landing Place",
            LandingPoint20504 => "Landing Point",
            LandingSite20505 => "Landing Site",
            LandingStrip20506 => "Landing Strip",
            LandingZone20507 => "Landing Zone",
            MarineManagementArea20508 => "Marine management area",
            MaritimeSafetyInformationArea20509 => "Maritime Safety Information area",
            MCMArea20510 => "MCM Area",
            MilitaryExerciseAirspace20511 => "Military exercise airspace",
            PatrolArea20513 => "Patrol area",
            QRouteLeg20514 => "Q-Route Leg",
            RadioBroadcastArea20515 => "Radio broadcast area",
            RegulatedAirspace20516 => "Regulated airspace",
            GeologicalLayer20517 => "Geological Layer",
            SeismicActivityArea20518 => "Seismic Activity Area",
            SensorAnomaly20519 => "Sensor Anomaly",
            ShelterLocation20520 => "Shelter Location",
            SuperficialSedimentDeposits20521 => "Superficial Sediment Deposits",
            TrafficabilityArea20522 => "Trafficability Area",
            TrawlScours20523 => "Trawl Scours",
            TurningPoint20524 => "Turning point",
            Viewpoint20525 => "Viewpoint",
            BottomTacticalDataArea20526 => "Bottom Tactical Data Area",
            BurialProbabilityArea20527 => "Burial Probability Area",
            LeisureActivityArea20528 => "Leisure Activity Area",
            PerformanceDataArea20529 => "Performance Data Area",
            ResourceLocation20530 => "Resource Location",
            RiskDataArea20531 => "Risk Data Area",
            NavigationSystemNAVAID20532 => "Navigation system (NAVAID)",
            InternalWatersArea20533 => "Internal Waters Area ",
            SeaIce20534 => "Sea Ice",
            IceAdvisoryArea20535 => "Ice Advisory Area",
            IcebergArea20536 => "Iceberg Area",
            LandIce20537 => "Land Ice",
            IceLine20538 => "Ice Line",
            IceRoute20539 => "Ice Route",
            IcePolynya20540 => "Ice Polynya",
            IceLead20541 => "Ice Lead",
            Iceberg20542 => "Iceberg",
            IceMovement20543 => "Ice Movement",
            TrafficRoute20544 => "Traffic route",
            UserDefined20717 => "User Defined",
            SmallBottomObject20718 => "Small Bottom Object",
            CompletenessForTheProductSpecification21484 => {
                "Completeness for the product specification"
            }
            SecurityClassificationInformation21485 => "Security Classification Information",
            VerticalDatumShiftArea21486 => "Vertical Datum Shift Area",
            DefinedStraightLines21487 => "Defined Straight Lines",
            Unknown => "Unknown",
        }
    }
}
