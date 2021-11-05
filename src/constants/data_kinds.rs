////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::binrw;
use const_format::formatcp;
use macro_const::macro_const;

macro_const! {
    const UI_DATA: u32 = 0x0;
    const WALL_GRAPH: u32 = 0x0A284D0B;
    const TRACK_SETTINGS: u32 = 0x0B9EB87E;
    const LOT_DESCRIPTION: u32 = 0x0BF999E7;
    const BINARY_INDEX: u32 = 0x0C560F39;
    const POOL_SURFACE: u32 = 0x0C900FDB;
    const TEXTURE_RESOURCE: u32 = 0x1C4A276C;
    const AUDIO_FILE: u32 = 0x2026960B;
    const SCENE_NODE: u32 = 0x25232B11;
    const THREE_DARRAY: u32 = 0x2A51171B;
    const TEXTURE_OVERLAY_XML: u32 = 0x2C1FD8A1;
    const POPUPS: u32 = 0x2C310F46;
    const SIM_SCORES: u32 = 0x3053CF74;
    const BEHAVIOR_CONSTANT: u32 = 0x42434F4E;
    const BEHAVIOR_FUNCTION: u32 = 0x42484156;
    const BITMAP_IMAGE: u32 = 0x424D505F;
    const CATALOG_STRING: u32 = 0x43415453;
    const IMAGE_LINK: u32 = 0x43494745;
    const CATALOG_DESCRIPTION: u32 = 0x43545353;
    const DRAWGROUP: u32 = 0x44475250;
    const FACE_PROPERTIES: u32 = 0x46414345;
    const FAMILY_INFORMATION: u32 = 0x46414D49;
    const FAMILY_DATA: u32 = 0x46414D68;
    const FUNCTION: u32 = 0x46434E53;
    const AUDIO_REFERENCE: u32 = 0x46574156;
    const GLOBAL_DATA: u32 = 0x474C4F42;
    const HOUSE_DESCRIPTOR: u32 = 0x484F5553;
    const TEXTURED_MATERIAL: u32 = 0x49596978;
    const WORLD_DATABASE: u32 = 0x49FF7D76;
    const LOT_TEXTURE: u32 = 0x4B58975B;
    const SKIN_TONE_XML: u32 = 0x4C158081;
    const CINEMATIC_SCENE: u32 = 0x4D51F042;
    const NEIGHBORHOOD_MEMORY: u32 = 0x4E474248;
    const NAME_REFERENCE: u32 = 0x4E524546;
    const NAME_MAP: u32 = 0x4E6D6150;
    const OBJECT_DATA: u32 = 0x4F424A44;
    const OBJECT_FUNCTION: u32 = 0x4F424A66;
    const OBJECT_METADATA: u32 = 0x4F626A4D;
    const IMAGE_COLOR_PALETTE: u32 = 0x50414C54;
    const SIM_PERSONAL_INFORMATION: u32 = 0x50455253;
    const STACK_SCRIPT: u32 = 0x504F5349;
    const PACKAGE_TOOLKIT: u32 = 0x50544250;
    const SIM_INFORMATION: u32 = 0x53494D49;
    const OBJECT_SLOT: u32 = 0x534C4F54;
    const SPRITES: u32 = 0x53505232;
    const TEXT_LISTS: u32 = 0x53545223;
    const TTAT: u32 = 0x54415454;
    const BEHAVIOR_FUNCTION_LABELS: u32 = 0x54505250;
    const BEHAVIOR_CONSTANT_LABELS: u32 = 0x5452434E;
    const BEHAVIOR_FLOWCHART_TREE: u32 = 0x54524545;
    const PIE_MENU_FUNCTIONS: u32 = 0x54544142;
    const PIE_MENU_SETTINGS: u32 = 0x54544173;
    const MATERIAL_OBJECT: u32 = 0x584D544F;
    const UNKNOWN_OBJECT: u32 = 0x584F424A;
    const ENVIRONMENT_CUBE_LIGHTING: u32 = 0x6A97042F;
    const TWO_DARRAY: u32 = 0x6B943B43;
    const LOT_DEFINITION: u32 = 0x6C589723;
    const OBJECT: u32 = 0x6F626A74;
    const HITLIST: u32 = 0x7B1ACFCD;
    const GEOMETRIC_NODE: u32 = 0x7BA3838C;
    const LIGHTMAP: u32 = 0x856DDBAC;
    const WALL_LAYER: u32 = 0x8A84D7B0;
    const UNKNOWN1: u32 = 0x8B0C79D6;
    const JPEG_IMAGE: u32 = 0x8C3CE95A;
    const FAMILY_TIES: u32 = 0x8C870743;
    const PREDICTIVE_MAPS: u32 = 0x8CC0A14B;
    const SOUND_EFFECTS: u32 = 0x8DB5E4C2;
    const UNKNOWN2: u32 = 0x9D796DB4;
    const SIM_DESCRIPTION: u32 = 0xAACE2EFB;
    const FENCE_POST_LAYER: u32 = 0xAB4BA572;
    const ROOF: u32 = 0xAB9406AA;
    const LOT_TERRAIN_GEOMETRY: u32 = 0xABCB5DA4;
    const NEIGHBORHOOD_TERRAIN: u32 = 0xABD0DC63;
    const LINEAR_FOG_LIGHTING: u32 = 0xAC06A66F;
    const DRAW_STATE_LIGHTING: u32 = 0xAC06A676;
    const GEOMETRIC_DATA_CONTAINER: u32 = 0xAC4F8687;
    const THREE_DREFERENCE: u32 = 0xAC506764;
    const ID_NUMBER: u32 = 0xAC8A7A2E;
    const WEATHER_INFO: u32 = 0xB21BE28B;
    const TSSG_SYSTEM: u32 = 0xBA353CE1;
    const LIGHT: u32 = 0xC9C81B9B;
    const STRING_MAP: u32 = 0xCAC4FC40;
    const VERTEX_LAYER: u32 = 0xCB4387A1;
    const UNKNOWN3: u32 = 0xCC2A6A34;
    const SIM_RELATIONS: u32 = 0xCC364C2A;
    const UNKNOWN4: u32 = 0xCC8A6A69;
    const FACIAL_STRUCTURE: u32 = 0xCCCEF852;
    const MAXIS_MATERIAL_SHADER: u32 = 0xCD7FE87A;
    const SIM_WANTS_AND_FEARS: u32 = 0xCD95548E;
    const CONTENT_REGISTRY: u32 = 0xCDB467B8;
    const CREATION_RESOURCE: u32 = 0xE519C933;
    const DIRECTORY: u32 = 0xE86B1EEF;
    const EFFECTS_RESOURCE_TREE: u32 = 0xEA5118B0;
    const PROPERTY_SET: u32 = 0xEBCF3E27;
    const VERSION_INFORMATION: u32 = 0xEBFEE342;
    const NEIGHBORDHOOD_VIEW: u32 = 0xEC44BDDC;
    const LARGE_IMAGE: u32 = 0xED534136;
    const SINGULAR_LOT_OBJECT: u32 = 0xFA1C39F7;
    const ANIMATION: u32 = 0xFB00791E;
    const SHAPE: u32 = 0xFC6EB1F7;
}

#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormatKind {
    #[brw(magic(0x00000000u32))]
    UiData = 0x0,
    #[brw(magic(0x0A284D0Bu32))]
    WallGraph,
    #[brw(magic(0x0B9EB87Eu32))]
    TrackSettings,
    #[brw(magic(0x0BF999E7u32))]
    LotDescription,
    #[brw(magic(0x0C560F39u32))]
    BinaryIndex,
    #[brw(magic(0x0C900FDBu32))]
    PoolSurface,
    #[brw(magic(0x25232B11u32))]
    SceneNode,
    #[brw(magic(0x2A51171Bu32))]
    ThreeDArray,
    #[brw(magic(0x2C1FD8A1u32))]
    TextureOverlayXML,
    #[brw(magic(0x2C310F46u32))]
    Popups,
    #[brw(magic(0x3053CF74u32))]
    SimScores,
    BehaviorConstant = 0x42434F4E,
    BehaviorFunction = 0x42484156,
    BitmapImage = 0x424D505F,
    CatalogString = 0x43415453,
    ImageLink = 0x43494745,
    CatalogDescription = 0x43545353,
    Drawgroup = 0x44475250,
    FaceProperties = 0x46414345,
    FamilyInformation = 0x46414D49,
    FamilyData = 0x46414D68,
    Function = 0x46434E53,
    AudioReference = 0x46574156,
    GlobalData = 0x474C4F42,
    HouseDescriptor = 0x484F5553,
    TexturedMaterial = 0x49596978,
    WorldDatabase = 0x49FF7D76,
    LotTexture = 0x4B58975B,
    SkinToneXml = 0x4C158081,
    CinematicScene = 0x4D51F042,
    NeighborhoodMemory = 0x4E474248,
    NameReference = 0x4E524546,
    NameMap = 0x4E6D6150,
    ObjectData = 0x4F424A44,
    ObjectFunction = 0x4F424A66,
    ObjectMetadata = 0x4F626A4D,
    ImageColorPalette = 0x50414C54,
    SimPersonalInformation = 0x50455253,
    StackScript = 0x504F5349,
    PackageToolkit = 0x50544250,
    SimInformation = 0x53494D49,
    ObjectSlot = 0x534C4F54,
    Sprites = 0x53505232,
    TextLists = 0x53545223,
    TTAT = 0x54415454,
    BehaviorFunctionLabels = 0x54505250,
    BehaviorConstantLabels = 0x5452434E,
    BehaviorFlowchartTree = 0x54524545,
    PieMenuFunctions = 0x54544142,
    PieMenuSettings = 0x54544173,
    MaterialObject = 0x584D544F,
    UnknownObject = 0x584F424A,
    EnvironmentCubeLighting = 0x6A97042F,
    TwoDArray = 0x6B943B43,
    LotDefinition = 0x6C589723,
    Object = 0x6F626A74,
    Hitlist = 0x7B1ACFCD,
    GeometricNode = 0x7BA3838C,
    Lightmap = 0x856DDBAC,
    WallLayer = 0x8A84D7B0,
    Unknown1 = 0x8B0C79D6,
    JpegImage = 0x8C3CE95A,
    FamilyTies = 0x8C870743,
    PredictiveMaps = 0x8CC0A14B,
    SoundEffects = 0x8DB5E4C2,
    Unknown2 = 0x9D796DB4,
    SimDescription = 0xAACE2EFB,
    FencePostLayer = 0xAB4BA572,
    Roof = 0xAB9406AA,
    LotTerrainGeometry = 0xABCB5DA4,
    NeighborhoodTerrain = 0xABD0DC63,
    LinearFogLighting = 0xAC06A66F,
    DrawStateLighting = 0xAC06A676,
    GeometricDataContainer = 0xAC4F8687,
    ThreeDReference = 0xAC506764,
    IdNumber = 0xAC8A7A2E,
    WeatherInfo = 0xB21BE28B,
    TssgSystem = 0xBA353CE1,
    Light = 0xC9C81B9B,
    StringMap = 0xCAC4FC40,
    VertexLayer = 0xCB4387A1,
    Unknown3 = 0xCC2A6A34,
    SimRelations = 0xCC364C2A,
    Unknown4 = 0xCC8A6A69,
    FacialStructure = 0xCCCEF852,
    MaxisMaterialShader = 0xCD7FE87A,
    SimWantsAndFears = 0xCD95548E,
    ContentRegistry = 0xCDB467B8,
    CreationResource = 0xE519C933,
    Directory = 0xE86B1EEF,
    EffectsResourceTree = 0xEA5118B0,
    PropertySet = 0xEBCF3E27,
    VersionInformation = 0xEBFEE342,
    NeighbordhoodView = 0xEC44BDDC,
    LargeImage = 0xED534136,
    SingularLotObject = 0xFA1C39F7,
    Animation = 0xFB00791E,
    Shape = 0xFC6EB1F7,
}

impl FormatKind {
    const fn short_name(&self) -> &str {
        return match self {
            FormatKind::UiData => "UI",
            FormatKind::WallGraph => "WGRA",
            FormatKind::TrackSettings => "TRKS",
            FormatKind::LotDescription => "DESC",
            FormatKind::BinaryIndex => "BINX",
            FormatKind::PoolSurface => "POOL",
            FormatKind::TextureResource => "TXTR",
            FormatKind::AudioFile => "XA",
            FormatKind::SceneNode => "5SC",
            FormatKind::ThreeDArray => "3ARY",
            FormatKind::TextureOverlayXML => "XTOL",
            FormatKind::Popups => "POPS",
            FormatKind::SimScores => "SCOR",
            FormatKind::BehaviorConstant => "BCON",
            FormatKind::BehaviorFunction => "BHAV",
            FormatKind::BitmapImage => "BMP",
            FormatKind::CatalogString => "CATS",
            FormatKind::ImageLink => "CIGE",
            FormatKind::CatalogDescription => "CTSS",
            FormatKind::Drawgroup => "DGRP",
            FormatKind::FaceProperties => "FACE",
            FormatKind::FamilyInformation => "FAMI",
            FormatKind::FamilyData => "FAMh",
            FormatKind::Function => "FCNS",
            FormatKind::AudioReference => "FWAV",
            FormatKind::GlobalData => "GLOB",
            FormatKind::HouseDescriptor => "HOUS",
            FormatKind::TexturedMaterial => "TXMT",
            FormatKind::WorldDatabase => "WRLD",
            FormatKind::LotTexture => "LTTX",
            FormatKind::SkinToneXml => "XSTN",
            FormatKind::CinematicScene => "CINE",
            FormatKind::NeighborhoodMemory => "NGBH",
            FormatKind::NameReference => "NREF",
            FormatKind::NameMap => "NMAP",
            FormatKind::ObjectData => "OBJD",
            FormatKind::ObjectFunction => "OBJF",
            FormatKind::ObjectMetadata => "OBJM",
            FormatKind::ImageColorPalette => "PALT",
            FormatKind::SimPersonalInformation => "PERS",
            FormatKind::StackScript => "POSI",
            FormatKind::PackageToolkit => "PTBP",
            FormatKind::SimInformation => "SIMI",
            FormatKind::ObjectSlot => "SLOT",
            FormatKind::Sprites => "SPR2",
            FormatKind::TextLists => "STR#",
            FormatKind::TTAT => "TTAT",
            FormatKind::BehaviorFunctionLabels => "TPRP",
            FormatKind::BehaviorConstantLabels => "TRCN",
            FormatKind::BehaviorFlowchartTree => "TREE",
            FormatKind::PieMenuFunctions => "TTAB",
            FormatKind::PieMenuSettings => "TTAs",
            FormatKind::MaterialObject => "XMTO",
            FormatKind::UnknownObject => "XOBJ",
            FormatKind::EnvironmentCubeLighting => "5EL",
            FormatKind::TwoDArray => "2ARY",
            FormatKind::LotDefinition => "LOT",
            FormatKind::Object => "MOBJT",
            FormatKind::Hitlist => "HLS",
            FormatKind::GeometricNode => "GMND",
            FormatKind::Lightmap => "LTMP",
            FormatKind::WallLayer => "WLL",
            FormatKind::Unknown1 => "UNK1",
            FormatKind::JpegImage => "JPG",
            FormatKind::FamilyTies => "FAMt",
            FormatKind::PredictiveMaps => "PMAP",
            FormatKind::SoundEffects => "SFX",
            FormatKind::Unknown2 => "UNK2",
            FormatKind::SimDescription => "PDAT",
            FormatKind::FencePostLayer => "FPL",
            FormatKind::Roof => "ROOF",
            FormatKind::LotTerrainGeometry => "LOTG",
            FormatKind::NeighborhoodTerrain => "NHTR",
            FormatKind::LinearFogLighting => "5LF",
            FormatKind::DrawStateLighting => "5DS",
            FormatKind::GeometricDataContainer => "GMDC",
            FormatKind::ThreeDReference => "3IDR",
            FormatKind::IdNumber => "NID",
            FormatKind::WeatherInfo => "WTHR",
            FormatKind::TssgSystem => "TSSG",
            FormatKind::Light => "LGHT",
            FormatKind::StringMap => "SMAP",
            FormatKind::VertexLayer => "VERT",
            FormatKind::Unknown3 => "UNK3",
            FormatKind::SimRelations => "SREL",
            FormatKind::Unknown4 => "UNK4",
            FormatKind::FacialStructure => "LxNR",
            FormatKind::MaxisMaterialShader => "MATSHAD",
            FormatKind::SimWantsAndFears => "SWAF",
            FormatKind::ContentRegistry => "CREG",
            FormatKind::CreationResource => "CRES",
            FormatKind::Directory => "DIR",
            FormatKind::EffectsResourceTree => "FX",
            FormatKind::PropertySet => "GZPS",
            FormatKind::VersionInformation => "VERS",
            FormatKind::NeighbordhoodView => "NHVW",
            FormatKind::LargeImage => "LIFO",
            FormatKind::SingularLotObject => "OBJT",
            FormatKind::Animation => "ANIM",
            FormatKind::Shape => "SHPE",
        };
    }

    const fn long_name(&self) -> &str {
        return match self {
            FormatKind::UiData => "UI Data",
            FormatKind::WallGraph => "Wall Graph",
            FormatKind::TrackSettings => "Track Settings",
            FormatKind::LotDescription => "Lot Description",
            FormatKind::BinaryIndex => "Binary Index",
            FormatKind::PoolSurface => "Pool Surface",
            FormatKind::TextureResource => "Texture Image",
            FormatKind::AudioFile => "Audio File",
            FormatKind::SceneNode => "5SC",
            FormatKind::ThreeDArray => "3D Array",
            FormatKind::TextureOverlayXML => "Texture Overlay XML",
            FormatKind::Popups => "POPUPS",
            FormatKind::SimScores => "Sim: Scores",
            FormatKind::BehaviorConstant => "Behavior Constant",
            FormatKind::BehaviorFunction => "Behavior Function",
            FormatKind::BitmapImage => "Bitmap Image",
            FormatKind::CatalogString => "Catalog String",
            FormatKind::ImageLink => "Image Link",
            FormatKind::CatalogDescription => "Catalog Description",
            FormatKind::Drawgroup => "DRAWGROUP",
            FormatKind::FaceProperties => "Face Properties",
            FormatKind::FamilyInformation => "Family Information",
            FormatKind::FamilyData => "Family Data",
            FormatKind::Function => "Function",
            FormatKind::AudioReference => "Audio Reference",
            FormatKind::GlobalData => "Global Data",
            FormatKind::HouseDescriptor => "House Descriptor",
            FormatKind::TexturedMaterial => "Textured Material",
            FormatKind::WorldDatabase => "World Database",
            FormatKind::LotTexture => "Lot Texture",
            FormatKind::SkinToneXml => "Skin Tone XML",
            FormatKind::CinematicScene => "Cinematic Scene",
            FormatKind::NeighborhoodMemory => "Neighborhood/Memory",
            FormatKind::NameReference => "Name Reference",
            FormatKind::NameMap => "Name Map",
            FormatKind::ObjectData => "OBJECT Data",
            FormatKind::ObjectFunction => "OBJECT Functions",
            FormatKind::ObjectMetadata => "OBJECT Metadata Imposters",
            FormatKind::ImageColorPalette => "Image Color Palette",
            FormatKind::SimPersonalInformation => "Sim Personal Information",
            FormatKind::StackScript => "Stack Script",
            FormatKind::PackageToolkit => "Package Toolkit",
            FormatKind::SimInformation => "Sim Information",
            FormatKind::ObjectSlot => "OBJECT Slot",
            FormatKind::Sprites => "SPRITES",
            FormatKind::TextLists => "Text Lists",
            FormatKind::TTAT => "TTAT",
            FormatKind::BehaviorFunctionLabels => "Behavior Function Labels",
            FormatKind::BehaviorConstantLabels => "Behavior Constant Labels",
            FormatKind::BehaviorFlowchartTree => "Behavior Flowchart Tree",
            FormatKind::PieMenuFunctions => "Pie Menu Functions",
            FormatKind::PieMenuSettings => "Pie Menu Settings",
            FormatKind::MaterialObject => "Material OBJECT",
            FormatKind::UnknownObject => "X Unknown OBJECT",
            FormatKind::EnvironmentCubeLighting => "Environment Cube Lighting",
            FormatKind::TwoDArray => "2D Array",
            FormatKind::LotDefinition => "Lot Definition",
            FormatKind::Object => "OBJECT",
            FormatKind::Hitlist => "HITLIST",
            FormatKind::GeometricNode => "Geometric Node",
            FormatKind::Lightmap => "LIGHTMAP",
            FormatKind::WallLayer => "Wall Layer",
            FormatKind::Unknown1 => formatcp!("Unknown 1 ({:#x})", FormatKind::Unknown1 as u32),
            FormatKind::JpegImage => "JPEG Image",
            FormatKind::FamilyTies => "Family Ties",
            FormatKind::PredictiveMaps => "Predictive Maps",
            FormatKind::SoundEffects => "Sound Effects",
            FormatKind::Unknown2 => formatcp!("Unknown 2 ({:#x})", FormatKind::Unknown2 as u32),
            FormatKind::SimDescription => "Sim Description",
            FormatKind::FencePostLayer => "Fence Post Layer",
            FormatKind::Roof => "ROOF",
            FormatKind::LotTerrainGeometry => "Lot Terrain Geometry",
            FormatKind::NeighborhoodTerrain => "Neighborhood Terrain",
            FormatKind::LinearFogLighting => "Linear Fog Lighting",
            FormatKind::DrawStateLighting => "Draw State Lighting",
            FormatKind::GeometricDataContainer => "Geometric Data Container",
            FormatKind::ThreeDReference => "3D ID Referencing",
            FormatKind::IdNumber => "ID Number",
            FormatKind::WeatherInfo => "Weather Info",
            FormatKind::TssgSystem => "TSSG System",
            FormatKind::Light => "LIGHT",
            FormatKind::StringMap => "String Map",
            FormatKind::VertexLayer => "Vertex Layer",
            FormatKind::Unknown3 => formatcp!("Unknown 3 ({:#x})", FormatKind::Unknown3 as u32),
            FormatKind::SimRelations => "Sim Relations",
            FormatKind::Unknown4 => formatcp!("Unknown 4 ({:#x})", FormatKind::Unknown4 as u32),
            FormatKind::FacialStructure => "Facial Structure",
            FormatKind::MaxisMaterialShader => "Maxis Material Shader",
            FormatKind::SimWantsAndFears => "Sim Wants and Fears",
            FormatKind::ContentRegistry => "Content Registry",
            FormatKind::CreationResource => "Creation Resource",
            FormatKind::Directory => "DIRECTORY of Compressed Files",
            FormatKind::EffectsResourceTree => "Effects Resource Tree",
            FormatKind::PropertySet => "Property Set",
            FormatKind::VersionInformation => "Version Information",
            FormatKind::NeighbordhoodView => "Neighborhood View",
            FormatKind::LargeImage => "Large Image",
            FormatKind::SingularLotObject => "Singular Lot OBJECT",
            FormatKind::Animation => "animation",
            FormatKind::Shape => "SHAPE",
        };
    }
}
