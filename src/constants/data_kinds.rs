////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::binrw;
use const_format::formatcp;

#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormatKind {
    #[brw(magic(0x0000_0000_u32))]
    UiData = 0x0,
    #[brw(magic(0x0A28_4D0B_u32))]
    WallGraph,
    #[brw(magic(0x0B9E_B87E_u32))]
    TrackSettings,
    #[brw(magic(0x0BF9_99E7_u32))]
    LotDescription,
    #[brw(magic(0x0C56_0F39_u32))]
    BinaryIndex,
    #[brw(magic(0x0C90_0FDB_u32))]
    PoolSurface,
    #[brw(magic(0x1C4A_276C_u32))]
    TextureResource,
    #[brw(magic(0x2026_960B_u32))]
    AudioFile,
    #[brw(magic(0x2523_2B11_u32))]
    SceneNode,
    #[brw(magic(0x2A51_171B_u32))]
    ThreeDArray,
    #[brw(magic(0x2C1F_D8A1_u32))]
    TextureOverlayXML,
    #[brw(magic(0x2C31_0F46_u32))]
    Popups,
    #[brw(magic(0x3053_CF74_u32))]
    SimScores,
    #[brw(magic(0x4243_4F4E_u32))]
    BehaviorConstant,
    #[brw(magic(0x4248_4156_u32))]
    BehaviorFunction,
    #[brw(magic(0x424D_505F_u32))]
    BitmapImage,
    #[brw(magic(0x4341_5453_u32))]
    CatalogString,
    #[brw(magic(0x4349_4745_u32))]
    ImageLink,
    #[brw(magic(0x4354_5353_u32))]
    CatalogDescription,
    #[brw(magic(0x4447_5250_u32))]
    Drawgroup,
    #[brw(magic(0x4641_4345_u32))]
    FaceProperties,
    #[brw(magic(0x4641_4D49_u32))]
    FamilyInformation,
    #[brw(magic(0x4641_4D68_u32))]
    FamilyData,
    #[brw(magic(0x4643_4E53_u32))]
    Function,
    #[brw(magic(0x4657_4156_u32))]
    AudioReference,
    #[brw(magic(0x474C_4F42_u32))]
    GlobalData,
    #[brw(magic(0x484F_5553_u32))]
    HouseDescriptor,
    #[brw(magic(0x4959_6978_u32))]
    TexturedMaterial,
    #[brw(magic(0x49FF_7D76_u32))]
    WorldDatabase,
    #[brw(magic(0x4B58_975B_u32))]
    LotTexture,
    #[brw(magic(0x4C15_8081_u32))]
    SkinToneXml,
    #[brw(magic(0x4D51_F042_u32))]
    CinematicScene,
    #[brw(magic(0x4E47_4248_u32))]
    NeighborhoodMemory,
    #[brw(magic(0x4E52_4546_u32))]
    NameReference,
    #[brw(magic(0x4E6D_6150_u32))]
    NameMap,
    #[brw(magic(0x4F42_4A44_u32))]
    ObjectData,
    #[brw(magic(0x4F42_4A66_u32))]
    ObjectFunction,
    #[brw(magic(0x4F62_6A4D_u32))]
    ObjectMetadata,
    #[brw(magic(0x5041_4C54_u32))]
    ImageColorPalette,
    #[brw(magic(0x5045_5253_u32))]
    SimPersonalInformation,
    #[brw(magic(0x504F_5349_u32))]
    StackScript,
    #[brw(magic(0x5054_4250_u32))]
    PackageToolkit,
    #[brw(magic(0x5349_4D49_u32))]
    SimInformation,
    #[brw(magic(0x534C_4F54_u32))]
    ObjectSlot,
    #[brw(magic(0x5350_5232_u32))]
    Sprites,
    #[brw(magic(0x5354_5223_u32))]
    TextLists,
    #[brw(magic(0x5441_5454_u32))]
    TTAT,
    #[brw(magic(0x5450_5250_u32))]
    BehaviorFunctionLabels,
    #[brw(magic(0x5452_434E_u32))]
    BehaviorConstantLabels,
    #[brw(magic(0x5452_4545_u32))]
    BehaviorFlowchartTree,
    #[brw(magic(0x5454_4142_u32))]
    PieMenuFunctions,
    #[brw(magic(0x5454_4173_u32))]
    PieMenuSettings,
    #[brw(magic(0x584D_544F_u32))]
    MaterialObject,
    #[brw(magic(0x584F_424A_u32))]
    UnknownObject,
    #[brw(magic(0x6A97_042F_u32))]
    EnvironmentCubeLighting,
    #[brw(magic(0x6B94_3B43_u32))]
    TwoDArray,
    #[brw(magic(0x6C58_9723_u32))]
    LotDefinition,
    #[brw(magic(0x6F62_6A74_u32))]
    Object,
    #[brw(magic(0x7B1A_CFCD_u32))]
    Hitlist,
    #[brw(magic(0x7BA3_838C_u32))]
    GeometricNode,
    #[brw(magic(0x856D_DBAC_u32))]
    Lightmap,
    #[brw(magic(0x8A84_D7B0_u32))]
    WallLayer,
    #[brw(magic(0x8B0C_79D6_u32))]
    Unknown1,
    #[brw(magic(0x8C3C_E95A_u32))]
    JpegImage,
    #[brw(magic(0x8C87_0743_u32))]
    FamilyTies,
    #[brw(magic(0x8CC0_A14B_u32))]
    PredictiveMaps,
    #[brw(magic(0x8DB5_E4C2_u32))]
    SoundEffects,
    #[brw(magic(0x9D79_6DB4_u32))]
    Unknown2,
    #[brw(magic(0xAACE_2EFB_u32))]
    SimDescription,
    #[brw(magic(0xAB4B_A572_u32))]
    FencePostLayer,
    #[brw(magic(0xAB94_06AA_u32))]
    Roof,
    #[brw(magic(0xABCB_5DA4_u32))]
    LotTerrainGeometry,
    #[brw(magic(0xABD0_DC63_u32))]
    NeighborhoodTerrain,
    #[brw(magic(0xAC06_A66F_u32))]
    LinearFogLighting,
    #[brw(magic(0xAC06_A676_u32))]
    DrawStateLighting,
    #[brw(magic(0xAC4F_8687_u32))]
    GeometricDataContainer,
    #[brw(magic(0xAC50_6764_u32))]
    ThreeDReference,
    #[brw(magic(0xAC8A_7A2E_u32))]
    IdNumber,
    #[brw(magic(0xB21B_E28B_u32))]
    WeatherInfo,
    #[brw(magic(0xBA35_3CE1_u32))]
    TssgSystem,
    #[brw(magic(0xC9C8_1B9B_u32))]
    Light,
    #[brw(magic(0xCAC4_FC40_u32))]
    StringMap,
    #[brw(magic(0xCB43_87A1_u32))]
    VertexLayer,
    #[brw(magic(0xCC2A_6A34_u32))]
    Unknown3,
    #[brw(magic(0xCC36_4C2A_u32))]
    SimRelations,
    #[brw(magic(0xCC8A_6A69_u32))]
    Unknown4,
    #[brw(magic(0xCCCE_F852_u32))]
    FacialStructure,
    #[brw(magic(0xCD7F_E87A_u32))]
    MaxisMaterialShader,
    #[brw(magic(0xCD95_548E_u32))]
    SimWantsAndFears,
    #[brw(magic(0xCDB4_67B8_u32))]
    ContentRegistry,
    #[brw(magic(0xE519_C933_u32))]
    CreationResource,
    #[brw(magic(0xE86B_1EEF_u32))]
    Directory,
    #[brw(magic(0xEA51_18B0_u32))]
    EffectsResourceTree,
    #[brw(magic(0xEBCF_3E27_u32))]
    PropertySet,
    #[brw(magic(0xEBFE_E342_u32))]
    VersionInformation,
    #[brw(magic(0xEC44_BDDC_u32))]
    NeighborhoodView,
    #[brw(magic(0xED53_4136_u32))]
    LargeImage,
    #[brw(magic(0xFA1C_39F7_u32))]
    SingularLotObject,
    #[brw(magic(0xFB00_791E_u32))]
    Animation,
    #[brw(magic(0xFC6E_B1F7_u32))]
    Shape,
}

impl FormatKind {
    const fn short_name(&self) -> &str {
        match self {
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
            FormatKind::NeighborhoodView => "NHVW",
            FormatKind::LargeImage => "LIFO",
            FormatKind::SingularLotObject => "OBJT",
            FormatKind::Animation => "ANIM",
            FormatKind::Shape => "SHPE",
        }
    }

    const fn long_name(&self) -> &str {
        match self {
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
            FormatKind::Popups => "Popups",
            FormatKind::SimScores => "Sim: Scores",
            FormatKind::BehaviorConstant => "Behavior Constant",
            FormatKind::BehaviorFunction => "Behavior Function",
            FormatKind::BitmapImage => "Bitmap Image",
            FormatKind::CatalogString => "Catalog String",
            FormatKind::ImageLink => "Image Link",
            FormatKind::CatalogDescription => "Catalog Description",
            FormatKind::Drawgroup => "Drawgroup",
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
            FormatKind::ObjectData => "Object Data",
            FormatKind::ObjectFunction => "Object Functions",
            FormatKind::ObjectMetadata => "Object Metadata Imposters",
            FormatKind::ImageColorPalette => "Image Color Palette",
            FormatKind::SimPersonalInformation => "Sim Personal Information",
            FormatKind::StackScript => "Stack Script",
            FormatKind::PackageToolkit => "Package Toolkit",
            FormatKind::SimInformation => "Sim Information",
            FormatKind::ObjectSlot => "Object Slot",
            FormatKind::Sprites => "Sprites",
            FormatKind::TextLists => "Text Lists",
            FormatKind::TTAT => "TTAT",
            FormatKind::BehaviorFunctionLabels => "Behavior Function Labels",
            FormatKind::BehaviorConstantLabels => "Behavior Constant Labels",
            FormatKind::BehaviorFlowchartTree => "Behavior Flowchart Tree",
            FormatKind::PieMenuFunctions => "Pie Menu Functions",
            FormatKind::PieMenuSettings => "Pie Menu Settings",
            FormatKind::MaterialObject => "Material Object",
            FormatKind::UnknownObject => "X Unknown Object",
            FormatKind::EnvironmentCubeLighting => "Environment Cube Lighting",
            FormatKind::TwoDArray => "2D Array",
            FormatKind::LotDefinition => "Lot Definition",
            FormatKind::Object => "Object",
            FormatKind::Hitlist => "Hitlist",
            FormatKind::GeometricNode => "Geometric Node",
            FormatKind::Lightmap => "Lightmap",
            FormatKind::WallLayer => "Wall Layer",
            FormatKind::Unknown1 => formatcp!("Unknown 1 ({:#x})", FormatKind::Unknown1.id()),
            FormatKind::JpegImage => "JPEG Image",
            FormatKind::FamilyTies => "Family Ties",
            FormatKind::PredictiveMaps => "Predictive Maps",
            FormatKind::SoundEffects => "Sound Effects",
            FormatKind::Unknown2 => formatcp!("Unknown 2 ({:#x})", FormatKind::Unknown2.id()),
            FormatKind::SimDescription => "Sim Description",
            FormatKind::FencePostLayer => "Fence Post Layer",
            FormatKind::Roof => "Roof",
            FormatKind::LotTerrainGeometry => "Lot Terrain Geometry",
            FormatKind::NeighborhoodTerrain => "Neighborhood Terrain",
            FormatKind::LinearFogLighting => "Linear Fog Lighting",
            FormatKind::DrawStateLighting => "Draw State Lighting",
            FormatKind::GeometricDataContainer => "Geometric Data Container",
            FormatKind::ThreeDReference => "3D ID Referencing",
            FormatKind::IdNumber => "ID Number",
            FormatKind::WeatherInfo => "Weather Info",
            FormatKind::TssgSystem => "TSSG System",
            FormatKind::Light => "Light",
            FormatKind::StringMap => "String Map",
            FormatKind::VertexLayer => "Vertex Layer",
            FormatKind::Unknown3 => formatcp!("Unknown 3 ({:#x})", FormatKind::Unknown3.id()),
            FormatKind::SimRelations => "Sim Relations",
            FormatKind::Unknown4 => formatcp!("Unknown 4 ({:#x})", FormatKind::Unknown4.id()),
            FormatKind::FacialStructure => "Facial Structure",
            FormatKind::MaxisMaterialShader => "Maxis Material Shader",
            FormatKind::SimWantsAndFears => "Sim Wants and Fears",
            FormatKind::ContentRegistry => "Content Registry",
            FormatKind::CreationResource => "Creation Resource",
            FormatKind::Directory => "Directory of Compressed Files",
            FormatKind::EffectsResourceTree => "Effects Resource Tree",
            FormatKind::PropertySet => "Property Set",
            FormatKind::VersionInformation => "Version Information",
            FormatKind::NeighborhoodView => "Neighborhood View",
            FormatKind::LargeImage => "Large Image",
            FormatKind::SingularLotObject => "Singular Lot Object",
            FormatKind::Animation => "animation",
            FormatKind::Shape => "Shape",
        }
    }

    const fn id(&self) -> u32 {
        match self {
            FormatKind::UiData => 0x0,
            FormatKind::WallGraph => 0x0A28_4D0B,
            FormatKind::TrackSettings => 0x0B9E_B87E,
            FormatKind::LotDescription => 0x0BF9_99E7,
            FormatKind::BinaryIndex => 0x0C56_0F39,
            FormatKind::PoolSurface => 0x0C90_0FDB,
            FormatKind::TextureResource => 0x1C4A_276C,
            FormatKind::AudioFile => 0x2026_960B,
            FormatKind::SceneNode => 0x2523_2B11,
            FormatKind::ThreeDArray => 0x2A51_171B,
            FormatKind::TextureOverlayXML => 0x2C1F_D8A1,
            FormatKind::Popups => 0x2C31_0F46,
            FormatKind::SimScores => 0x3053_CF74,
            FormatKind::BehaviorConstant => 0x4243_4F4E,
            FormatKind::BehaviorFunction => 0x4248_4156,
            FormatKind::BitmapImage => 0x424D_505F,
            FormatKind::CatalogString => 0x4341_5453,
            FormatKind::ImageLink => 0x4349_4745,
            FormatKind::CatalogDescription => 0x4354_5353,
            FormatKind::Drawgroup => 0x4447_5250,
            FormatKind::FaceProperties => 0x4641_4345,
            FormatKind::FamilyInformation => 0x4641_4D49,
            FormatKind::FamilyData => 0x4641_4D68,
            FormatKind::Function => 0x4643_4E53,
            FormatKind::AudioReference => 0x4657_4156,
            FormatKind::GlobalData => 0x474C_4F42,
            FormatKind::HouseDescriptor => 0x484F_5553,
            FormatKind::TexturedMaterial => 0x4959_6978,
            FormatKind::WorldDatabase => 0x49FF_7D76,
            FormatKind::LotTexture => 0x4B58_975B,
            FormatKind::SkinToneXml => 0x4C15_8081,
            FormatKind::CinematicScene => 0x4D51_F042,
            FormatKind::NeighborhoodMemory => 0x4E47_4248,
            FormatKind::NameReference => 0x4E52_4546,
            FormatKind::NameMap => 0x4E6D_6150,
            FormatKind::ObjectData => 0x4F42_4A44,
            FormatKind::ObjectFunction => 0x4F42_4A66,
            FormatKind::ObjectMetadata => 0x4F62_6A4D,
            FormatKind::ImageColorPalette => 0x5041_4C54,
            FormatKind::SimPersonalInformation => 0x5045_5253,
            FormatKind::StackScript => 0x504F_5349,
            FormatKind::PackageToolkit => 0x5054_4250,
            FormatKind::SimInformation => 0x5349_4D49,
            FormatKind::ObjectSlot => 0x534C_4F54,
            FormatKind::Sprites => 0x5350_5232,
            FormatKind::TextLists => 0x5354_5223,
            FormatKind::TTAT => 0x5441_5454,
            FormatKind::BehaviorFunctionLabels => 0x5450_5250,
            FormatKind::BehaviorConstantLabels => 0x5452_434E,
            FormatKind::BehaviorFlowchartTree => 0x5452_4545,
            FormatKind::PieMenuFunctions => 0x5454_4142,
            FormatKind::PieMenuSettings => 0x5454_4173,
            FormatKind::MaterialObject => 0x584D_544F,
            FormatKind::UnknownObject => 0x584F_424A,
            FormatKind::EnvironmentCubeLighting => 0x6A97_042F,
            FormatKind::TwoDArray => 0x6B94_3B43,
            FormatKind::LotDefinition => 0x6C58_9723,
            FormatKind::Object => 0x6F62_6A74,
            FormatKind::Hitlist => 0x7B1A_CFCD,
            FormatKind::GeometricNode => 0x7BA3_838C,
            FormatKind::Lightmap => 0x856D_DBAC,
            FormatKind::WallLayer => 0x8A84_D7B0,
            FormatKind::Unknown1 => 0x8B0C_79D6,
            FormatKind::JpegImage => 0x8C3C_E95A,
            FormatKind::FamilyTies => 0x8C87_0743,
            FormatKind::PredictiveMaps => 0x8CC0_A14B,
            FormatKind::SoundEffects => 0x8DB5_E4C2,
            FormatKind::Unknown2 => 0x9D79_6DB4,
            FormatKind::SimDescription => 0xAACE_2EFB,
            FormatKind::FencePostLayer => 0xAB4B_A572,
            FormatKind::Roof => 0xAB94_06AA,
            FormatKind::LotTerrainGeometry => 0xABCB_5DA4,
            FormatKind::NeighborhoodTerrain => 0xABD0_DC63,
            FormatKind::LinearFogLighting => 0xAC06_A66F,
            FormatKind::DrawStateLighting => 0xAC06_A676,
            FormatKind::GeometricDataContainer => 0xAC4F_8687,
            FormatKind::ThreeDReference => 0xAC50_6764,
            FormatKind::IdNumber => 0xAC8A_7A2E,
            FormatKind::WeatherInfo => 0xB21B_E28B,
            FormatKind::TssgSystem => 0xBA35_3CE1,
            FormatKind::Light => 0xC9C8_1B9B,
            FormatKind::StringMap => 0xCAC4_FC40,
            FormatKind::VertexLayer => 0xCB43_87A1,
            FormatKind::Unknown3 => 0xCC2A_6A34,
            FormatKind::SimRelations => 0xCC36_4C2A,
            FormatKind::Unknown4 => 0xCC8A_6A69,
            FormatKind::FacialStructure => 0xCCCE_F852,
            FormatKind::MaxisMaterialShader => 0xCD7F_E87A,
            FormatKind::SimWantsAndFears => 0xCD95_548E,
            FormatKind::ContentRegistry => 0xCDB4_67B8,
            FormatKind::CreationResource => 0xE519_C933,
            FormatKind::Directory => 0xE86B_1EEF,
            FormatKind::EffectsResourceTree => 0xEA51_18B0,
            FormatKind::PropertySet => 0xEBCF_3E27,
            FormatKind::VersionInformation => 0xEBFE_E342,
            FormatKind::NeighborhoodView => 0xEC44_BDDC,
            FormatKind::LargeImage => 0xED53_4136,
            FormatKind::SingularLotObject => 0xFA1C_39F7,
            FormatKind::Animation => 0xFB00_791E,
            FormatKind::Shape => 0xFC6E_B1F7,
        }
    }
}
