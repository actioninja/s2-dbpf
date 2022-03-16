////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use crate::types::behavior_constant::Bcon;
use crate::types::behavior_function::Bhav;
use crate::types::directory::Dir;
use crate::types::sim::wants_and_fears::Swaf;
use binrw::binrw;
use const_format::formatcp;
use enum_as_inner::EnumAsInner;
#[cfg(test)]
use test_strategy::Arbitrary;

//TODO: make a macro to generate this crap

#[derive(Debug, EnumAsInner)]
pub enum FormatKind {
    BehaviorConstant(Box<Bcon>),
    BehaviorFunction(Box<Bhav>),
    Directory(Box<Dir>),
    SimWantsAndFears(Box<Swaf>),
}

#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum FormatId {
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

impl FormatId {
    #[must_use]
    pub const fn short_name(self) -> &'static str {
        match self {
            FormatId::UiData => "UI",
            FormatId::WallGraph => "WGRA",
            FormatId::TrackSettings => "TRKS",
            FormatId::LotDescription => "DESC",
            FormatId::BinaryIndex => "BINX",
            FormatId::PoolSurface => "POOL",
            FormatId::TextureResource => "TXTR",
            FormatId::AudioFile => "XA",
            FormatId::SceneNode => "5SC",
            FormatId::ThreeDArray => "3ARY",
            FormatId::TextureOverlayXML => "XTOL",
            FormatId::Popups => "POPS",
            FormatId::SimScores => "SCOR",
            FormatId::BehaviorConstant => "BCON",
            FormatId::BehaviorFunction => "BHAV",
            FormatId::BitmapImage => "BMP",
            FormatId::CatalogString => "CATS",
            FormatId::ImageLink => "CIGE",
            FormatId::CatalogDescription => "CTSS",
            FormatId::Drawgroup => "DGRP",
            FormatId::FaceProperties => "FACE",
            FormatId::FamilyInformation => "FAMI",
            FormatId::FamilyData => "FAMh",
            FormatId::Function => "FCNS",
            FormatId::AudioReference => "FWAV",
            FormatId::GlobalData => "GLOB",
            FormatId::HouseDescriptor => "HOUS",
            FormatId::TexturedMaterial => "TXMT",
            FormatId::WorldDatabase => "WRLD",
            FormatId::LotTexture => "LTTX",
            FormatId::SkinToneXml => "XSTN",
            FormatId::CinematicScene => "CINE",
            FormatId::NeighborhoodMemory => "NGBH",
            FormatId::NameReference => "NREF",
            FormatId::NameMap => "NMAP",
            FormatId::ObjectData => "OBJD",
            FormatId::ObjectFunction => "OBJF",
            FormatId::ObjectMetadata => "OBJM",
            FormatId::ImageColorPalette => "PALT",
            FormatId::SimPersonalInformation => "PERS",
            FormatId::StackScript => "POSI",
            FormatId::PackageToolkit => "PTBP",
            FormatId::SimInformation => "SIMI",
            FormatId::ObjectSlot => "SLOT",
            FormatId::Sprites => "SPR2",
            FormatId::TextLists => "STR#",
            FormatId::TTAT => "TTAT",
            FormatId::BehaviorFunctionLabels => "TPRP",
            FormatId::BehaviorConstantLabels => "TRCN",
            FormatId::BehaviorFlowchartTree => "TREE",
            FormatId::PieMenuFunctions => "TTAB",
            FormatId::PieMenuSettings => "TTAs",
            FormatId::MaterialObject => "XMTO",
            FormatId::UnknownObject => "XOBJ",
            FormatId::EnvironmentCubeLighting => "5EL",
            FormatId::TwoDArray => "2ARY",
            FormatId::LotDefinition => "LOT",
            FormatId::Object => "MOBJT",
            FormatId::Hitlist => "HLS",
            FormatId::GeometricNode => "GMND",
            FormatId::Lightmap => "LTMP",
            FormatId::WallLayer => "WLL",
            FormatId::Unknown1 => "UNK1",
            FormatId::JpegImage => "JPG",
            FormatId::FamilyTies => "FAMt",
            FormatId::PredictiveMaps => "PMAP",
            FormatId::SoundEffects => "SFX",
            FormatId::Unknown2 => "UNK2",
            FormatId::SimDescription => "PDAT",
            FormatId::FencePostLayer => "FPL",
            FormatId::Roof => "ROOF",
            FormatId::LotTerrainGeometry => "LOTG",
            FormatId::NeighborhoodTerrain => "NHTR",
            FormatId::LinearFogLighting => "5LF",
            FormatId::DrawStateLighting => "5DS",
            FormatId::GeometricDataContainer => "GMDC",
            FormatId::ThreeDReference => "3IDR",
            FormatId::IdNumber => "NID",
            FormatId::WeatherInfo => "WTHR",
            FormatId::TssgSystem => "TSSG",
            FormatId::Light => "LGHT",
            FormatId::StringMap => "SMAP",
            FormatId::VertexLayer => "VERT",
            FormatId::Unknown3 => "UNK3",
            FormatId::SimRelations => "SREL",
            FormatId::Unknown4 => "UNK4",
            FormatId::FacialStructure => "LxNR",
            FormatId::MaxisMaterialShader => "MATSHAD",
            FormatId::SimWantsAndFears => "SWAF",
            FormatId::ContentRegistry => "CREG",
            FormatId::CreationResource => "CRES",
            FormatId::Directory => "DIR",
            FormatId::EffectsResourceTree => "FX",
            FormatId::PropertySet => "GZPS",
            FormatId::VersionInformation => "VERS",
            FormatId::NeighborhoodView => "NHVW",
            FormatId::LargeImage => "LIFO",
            FormatId::SingularLotObject => "OBJT",
            FormatId::Animation => "ANIM",
            FormatId::Shape => "SHPE",
        }
    }

    #[must_use]
    pub const fn long_name(self) -> &'static str {
        match self {
            FormatId::UiData => "UI Data",
            FormatId::WallGraph => "Wall Graph",
            FormatId::TrackSettings => "Track Settings",
            FormatId::LotDescription => "Lot Description",
            FormatId::BinaryIndex => "Binary Index",
            FormatId::PoolSurface => "Pool Surface",
            FormatId::TextureResource => "Texture Image",
            FormatId::AudioFile => "Audio File",
            FormatId::SceneNode => "5SC",
            FormatId::ThreeDArray => "3D Array",
            FormatId::TextureOverlayXML => "Texture Overlay XML",
            FormatId::Popups => "Popups",
            FormatId::SimScores => "Sim: Scores",
            FormatId::BehaviorConstant => "Behavior Constant",
            FormatId::BehaviorFunction => "Behavior Function",
            FormatId::BitmapImage => "Bitmap Image",
            FormatId::CatalogString => "Catalog String",
            FormatId::ImageLink => "Image Link",
            FormatId::CatalogDescription => "Catalog Description",
            FormatId::Drawgroup => "Drawgroup",
            FormatId::FaceProperties => "Face Properties",
            FormatId::FamilyInformation => "Family Information",
            FormatId::FamilyData => "Family Data",
            FormatId::Function => "Function",
            FormatId::AudioReference => "Audio Reference",
            FormatId::GlobalData => "Global Data",
            FormatId::HouseDescriptor => "House Descriptor",
            FormatId::TexturedMaterial => "Textured Material",
            FormatId::WorldDatabase => "World Database",
            FormatId::LotTexture => "Lot Texture",
            FormatId::SkinToneXml => "Skin Tone XML",
            FormatId::CinematicScene => "Cinematic Scene",
            FormatId::NeighborhoodMemory => "Neighborhood/Memory",
            FormatId::NameReference => "Name Reference",
            FormatId::NameMap => "Name Map",
            FormatId::ObjectData => "Object Data",
            FormatId::ObjectFunction => "Object Functions",
            FormatId::ObjectMetadata => "Object Metadata Imposters",
            FormatId::ImageColorPalette => "Image Color Palette",
            FormatId::SimPersonalInformation => "Sim Personal Information",
            FormatId::StackScript => "Stack Script",
            FormatId::PackageToolkit => "Package Toolkit",
            FormatId::SimInformation => "Sim Information",
            FormatId::ObjectSlot => "Object Slot",
            FormatId::Sprites => "Sprites",
            FormatId::TextLists => "Text Lists",
            FormatId::TTAT => "TTAT",
            FormatId::BehaviorFunctionLabels => "Behavior Function Labels",
            FormatId::BehaviorConstantLabels => "Behavior Constant Labels",
            FormatId::BehaviorFlowchartTree => "Behavior Flowchart Tree",
            FormatId::PieMenuFunctions => "Pie Menu Functions",
            FormatId::PieMenuSettings => "Pie Menu Settings",
            FormatId::MaterialObject => "Material Object",
            FormatId::UnknownObject => "X Unknown Object",
            FormatId::EnvironmentCubeLighting => "Environment Cube Lighting",
            FormatId::TwoDArray => "2D Array",
            FormatId::LotDefinition => "Lot Definition",
            FormatId::Object => "Object",
            FormatId::Hitlist => "Hitlist",
            FormatId::GeometricNode => "Geometric Node",
            FormatId::Lightmap => "Lightmap",
            FormatId::WallLayer => "Wall Layer",
            FormatId::Unknown1 => formatcp!("Unknown 1 ({:#x})", FormatId::Unknown1.id()),
            FormatId::JpegImage => "JPEG Image",
            FormatId::FamilyTies => "Family Ties",
            FormatId::PredictiveMaps => "Predictive Maps",
            FormatId::SoundEffects => "Sound Effects",
            FormatId::Unknown2 => formatcp!("Unknown 2 ({:#x})", FormatId::Unknown2.id()),
            FormatId::SimDescription => "Sim Description",
            FormatId::FencePostLayer => "Fence Post Layer",
            FormatId::Roof => "Roof",
            FormatId::LotTerrainGeometry => "Lot Terrain Geometry",
            FormatId::NeighborhoodTerrain => "Neighborhood Terrain",
            FormatId::LinearFogLighting => "Linear Fog Lighting",
            FormatId::DrawStateLighting => "Draw State Lighting",
            FormatId::GeometricDataContainer => "Geometric Data Container",
            FormatId::ThreeDReference => "3D ID Referencing",
            FormatId::IdNumber => "ID Number",
            FormatId::WeatherInfo => "Weather Info",
            FormatId::TssgSystem => "TSSG System",
            FormatId::Light => "Light",
            FormatId::StringMap => "String Map",
            FormatId::VertexLayer => "Vertex Layer",
            FormatId::Unknown3 => formatcp!("Unknown 3 ({:#x})", FormatId::Unknown3.id()),
            FormatId::SimRelations => "Sim Relations",
            FormatId::Unknown4 => formatcp!("Unknown 4 ({:#x})", FormatId::Unknown4.id()),
            FormatId::FacialStructure => "Facial Structure",
            FormatId::MaxisMaterialShader => "Maxis Material Shader",
            FormatId::SimWantsAndFears => "Sim Wants and Fears",
            FormatId::ContentRegistry => "Content Registry",
            FormatId::CreationResource => "Creation Resource",
            FormatId::Directory => "Directory of Compressed Files",
            FormatId::EffectsResourceTree => "Effects Resource Tree",
            FormatId::PropertySet => "Property Set",
            FormatId::VersionInformation => "Version Information",
            FormatId::NeighborhoodView => "Neighborhood View",
            FormatId::LargeImage => "Large Image",
            FormatId::SingularLotObject => "Singular Lot Object",
            FormatId::Animation => "animation",
            FormatId::Shape => "Shape",
        }
    }

    #[must_use]
    pub const fn id(self) -> u32 {
        match self {
            FormatId::UiData => 0x0,
            FormatId::WallGraph => 0x0A28_4D0B,
            FormatId::TrackSettings => 0x0B9E_B87E,
            FormatId::LotDescription => 0x0BF9_99E7,
            FormatId::BinaryIndex => 0x0C56_0F39,
            FormatId::PoolSurface => 0x0C90_0FDB,
            FormatId::TextureResource => 0x1C4A_276C,
            FormatId::AudioFile => 0x2026_960B,
            FormatId::SceneNode => 0x2523_2B11,
            FormatId::ThreeDArray => 0x2A51_171B,
            FormatId::TextureOverlayXML => 0x2C1F_D8A1,
            FormatId::Popups => 0x2C31_0F46,
            FormatId::SimScores => 0x3053_CF74,
            FormatId::BehaviorConstant => 0x4243_4F4E,
            FormatId::BehaviorFunction => 0x4248_4156,
            FormatId::BitmapImage => 0x424D_505F,
            FormatId::CatalogString => 0x4341_5453,
            FormatId::ImageLink => 0x4349_4745,
            FormatId::CatalogDescription => 0x4354_5353,
            FormatId::Drawgroup => 0x4447_5250,
            FormatId::FaceProperties => 0x4641_4345,
            FormatId::FamilyInformation => 0x4641_4D49,
            FormatId::FamilyData => 0x4641_4D68,
            FormatId::Function => 0x4643_4E53,
            FormatId::AudioReference => 0x4657_4156,
            FormatId::GlobalData => 0x474C_4F42,
            FormatId::HouseDescriptor => 0x484F_5553,
            FormatId::TexturedMaterial => 0x4959_6978,
            FormatId::WorldDatabase => 0x49FF_7D76,
            FormatId::LotTexture => 0x4B58_975B,
            FormatId::SkinToneXml => 0x4C15_8081,
            FormatId::CinematicScene => 0x4D51_F042,
            FormatId::NeighborhoodMemory => 0x4E47_4248,
            FormatId::NameReference => 0x4E52_4546,
            FormatId::NameMap => 0x4E6D_6150,
            FormatId::ObjectData => 0x4F42_4A44,
            FormatId::ObjectFunction => 0x4F42_4A66,
            FormatId::ObjectMetadata => 0x4F62_6A4D,
            FormatId::ImageColorPalette => 0x5041_4C54,
            FormatId::SimPersonalInformation => 0x5045_5253,
            FormatId::StackScript => 0x504F_5349,
            FormatId::PackageToolkit => 0x5054_4250,
            FormatId::SimInformation => 0x5349_4D49,
            FormatId::ObjectSlot => 0x534C_4F54,
            FormatId::Sprites => 0x5350_5232,
            FormatId::TextLists => 0x5354_5223,
            FormatId::TTAT => 0x5441_5454,
            FormatId::BehaviorFunctionLabels => 0x5450_5250,
            FormatId::BehaviorConstantLabels => 0x5452_434E,
            FormatId::BehaviorFlowchartTree => 0x5452_4545,
            FormatId::PieMenuFunctions => 0x5454_4142,
            FormatId::PieMenuSettings => 0x5454_4173,
            FormatId::MaterialObject => 0x584D_544F,
            FormatId::UnknownObject => 0x584F_424A,
            FormatId::EnvironmentCubeLighting => 0x6A97_042F,
            FormatId::TwoDArray => 0x6B94_3B43,
            FormatId::LotDefinition => 0x6C58_9723,
            FormatId::Object => 0x6F62_6A74,
            FormatId::Hitlist => 0x7B1A_CFCD,
            FormatId::GeometricNode => 0x7BA3_838C,
            FormatId::Lightmap => 0x856D_DBAC,
            FormatId::WallLayer => 0x8A84_D7B0,
            FormatId::Unknown1 => 0x8B0C_79D6,
            FormatId::JpegImage => 0x8C3C_E95A,
            FormatId::FamilyTies => 0x8C87_0743,
            FormatId::PredictiveMaps => 0x8CC0_A14B,
            FormatId::SoundEffects => 0x8DB5_E4C2,
            FormatId::Unknown2 => 0x9D79_6DB4,
            FormatId::SimDescription => 0xAACE_2EFB,
            FormatId::FencePostLayer => 0xAB4B_A572,
            FormatId::Roof => 0xAB94_06AA,
            FormatId::LotTerrainGeometry => 0xABCB_5DA4,
            FormatId::NeighborhoodTerrain => 0xABD0_DC63,
            FormatId::LinearFogLighting => 0xAC06_A66F,
            FormatId::DrawStateLighting => 0xAC06_A676,
            FormatId::GeometricDataContainer => 0xAC4F_8687,
            FormatId::ThreeDReference => 0xAC50_6764,
            FormatId::IdNumber => 0xAC8A_7A2E,
            FormatId::WeatherInfo => 0xB21B_E28B,
            FormatId::TssgSystem => 0xBA35_3CE1,
            FormatId::Light => 0xC9C8_1B9B,
            FormatId::StringMap => 0xCAC4_FC40,
            FormatId::VertexLayer => 0xCB43_87A1,
            FormatId::Unknown3 => 0xCC2A_6A34,
            FormatId::SimRelations => 0xCC36_4C2A,
            FormatId::Unknown4 => 0xCC8A_6A69,
            FormatId::FacialStructure => 0xCCCE_F852,
            FormatId::MaxisMaterialShader => 0xCD7F_E87A,
            FormatId::SimWantsAndFears => 0xCD95_548E,
            FormatId::ContentRegistry => 0xCDB4_67B8,
            FormatId::CreationResource => 0xE519_C933,
            FormatId::Directory => 0xE86B_1EEF,
            FormatId::EffectsResourceTree => 0xEA51_18B0,
            FormatId::PropertySet => 0xEBCF_3E27,
            FormatId::VersionInformation => 0xEBFE_E342,
            FormatId::NeighborhoodView => 0xEC44_BDDC,
            FormatId::LargeImage => 0xED53_4136,
            FormatId::SingularLotObject => 0xFA1C_39F7,
            FormatId::Animation => 0xFB00_791E,
            FormatId::Shape => 0xFC6E_B1F7,
        }
    }
}
