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
    #[brw(magic(0x1C4A276Cu32))]
    TextureResource,
    #[brw(magic(0x2026960Bu32))]
    AudioFile,
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
    #[brw(magic(0x42434F4Eu32))]
    BehaviorConstant,
    #[brw(magic(0x42484156u32))]
    BehaviorFunction,
    #[brw(magic(0x424D505Fu32))]
    BitmapImage,
    #[brw(magic(0x43415453u32))]
    CatalogString,
    #[brw(magic(0x43494745u32))]
    ImageLink,
    #[brw(magic(0x43545353u32))]
    CatalogDescription,
    #[brw(magic(0x44475250u32))]
    Drawgroup,
    #[brw(magic(0x46414345u32))]
    FaceProperties,
    #[brw(magic(0x46414D49u32))]
    FamilyInformation,
    #[brw(magic(0x46414D68u32))]
    FamilyData,
    #[brw(magic(0x46434E53u32))]
    Function,
    #[brw(magic(0x46574156u32))]
    AudioReference,
    #[brw(magic(0x474C4F42u32))]
    GlobalData,
    #[brw(magic(0x484F5553u32))]
    HouseDescriptor,
    #[brw(magic(0x49596978u32))]
    TexturedMaterial,
    #[brw(magic(0x49FF7D76u32))]
    WorldDatabase,
    #[brw(magic(0x4B58975Bu32))]
    LotTexture,
    #[brw(magic(0x4C158081u32))]
    SkinToneXml,
    #[brw(magic(0x4D51F042u32))]
    CinematicScene,
    #[brw(magic(0x4E474248u32))]
    NeighborhoodMemory,
    #[brw(magic(0x4E524546u32))]
    NameReference,
    #[brw(magic(0x4E6D6150u32))]
    NameMap,
    #[brw(magic(0x4F424A44u32))]
    ObjectData,
    #[brw(magic(0x4F424A66u32))]
    ObjectFunction,
    #[brw(magic(0x4F626A4Du32))]
    ObjectMetadata,
    #[brw(magic(0x50414C54u32))]
    ImageColorPalette,
    #[brw(magic(0x50455253u32))]
    SimPersonalInformation,
    #[brw(magic(0x504F5349u32))]
    StackScript,
    #[brw(magic(0x50544250u32))]
    PackageToolkit,
    #[brw(magic(0x53494D49u32))]
    SimInformation,
    #[brw(magic(0x534C4F54u32))]
    ObjectSlot,
    #[brw(magic(0x53505232u32))]
    Sprites,
    #[brw(magic(0x53545223u32))]
    TextLists,
    #[brw(magic(0x54415454u32))]
    TTAT,
    #[brw(magic(0x54505250u32))]
    BehaviorFunctionLabels,
    #[brw(magic(0x5452434Eu32))]
    BehaviorConstantLabels,
    #[brw(magic(0x54524545u32))]
    BehaviorFlowchartTree,
    #[brw(magic(0x54544142u32))]
    PieMenuFunctions,
    #[brw(magic(0x54544173u32))]
    PieMenuSettings,
    #[brw(magic(0x584D544Fu32))]
    MaterialObject,
    #[brw(magic(0x584F424Au32))]
    UnknownObject,
    #[brw(magic(0x6A97042Fu32))]
    EnvironmentCubeLighting,
    #[brw(magic(0x6B943B43u32))]
    TwoDArray,
    #[brw(magic(0x6C589723u32))]
    LotDefinition,
    #[brw(magic(0x6F626A74u32))]
    Object,
    #[brw(magic(0x7B1ACFCDu32))]
    Hitlist,
    #[brw(magic(0x7BA3838Cu32))]
    GeometricNode,
    #[brw(magic(0x856DDBACu32))]
    Lightmap,
    #[brw(magic(0x8A84D7B0u32))]
    WallLayer,
    #[brw(magic(0x8B0C79D6u32))]
    Unknown1,
    #[brw(magic(0x8C3CE95Au32))]
    JpegImage,
    #[brw(magic(0x8C870743u32))]
    FamilyTies,
    #[brw(magic(0x8CC0A14Bu32))]
    PredictiveMaps,
    #[brw(magic(0x8DB5E4C2u32))]
    SoundEffects,
    #[brw(magic(0x9D796DB4u32))]
    Unknown2,
    #[brw(magic(0xAACE2EFBu32))]
    SimDescription,
    #[brw(magic(0xAB4BA572u32))]
    FencePostLayer,
    #[brw(magic(0xAB9406AAu32))]
    Roof,
    #[brw(magic(0xABCB5DA4u32))]
    LotTerrainGeometry,
    #[brw(magic(0xABD0DC63u32))]
    NeighborhoodTerrain,
    #[brw(magic(0xAC06A66Fu32))]
    LinearFogLighting,
    #[brw(magic(0xAC06A676u32))]
    DrawStateLighting,
    #[brw(magic(0xAC4F8687u32))]
    GeometricDataContainer,
    #[brw(magic(0xAC506764u32))]
    ThreeDReference,
    #[brw(magic(0xAC8A7A2Eu32))]
    IdNumber,
    #[brw(magic(0xB21BE28Bu32))]
    WeatherInfo,
    #[brw(magic(0xBA353CE1u32))]
    TssgSystem,
    #[brw(magic(0xC9C81B9Bu32))]
    Light,
    #[brw(magic(0xCAC4FC40u32))]
    StringMap,
    #[brw(magic(0xCB4387A1u32))]
    VertexLayer,
    #[brw(magic(0xCC2A6A34u32))]
    Unknown3,
    #[brw(magic(0xCC364C2Au32))]
    SimRelations,
    #[brw(magic(0xCC8A6A69u32))]
    Unknown4,
    #[brw(magic(0xCCCEF852u32))]
    FacialStructure,
    #[brw(magic(0xCD7FE87Au32))]
    MaxisMaterialShader,
    #[brw(magic(0xCD95548Eu32))]
    SimWantsAndFears,
    #[brw(magic(0xCDB467B8u32))]
    ContentRegistry,
    #[brw(magic(0xE519C933u32))]
    CreationResource,
    #[brw(magic(0xE86B1EEFu32))]
    Directory,
    #[brw(magic(0xEA5118B0u32))]
    EffectsResourceTree,
    #[brw(magic(0xEBCF3E27u32))]
    PropertySet,
    #[brw(magic(0xEBFEE342u32))]
    VersionInformation,
    #[brw(magic(0xEC44BDDCu32))]
    NeighborhoodView,
    #[brw(magic(0xED534136u32))]
    LargeImage,
    #[brw(magic(0xFA1C39F7u32))]
    SingularLotObject,
    #[brw(magic(0xFB00791Eu32))]
    Animation,
    #[brw(magic(0xFC6EB1F7u32))]
    Shape,
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
            FormatKind::NeighborhoodView => "NHVW",
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
        };
    }

    const fn id(&self) -> u32 {
        match self {
            FormatKind::UiData => 0x0,
            FormatKind::WallGraph => 0x0A284D0B,
            FormatKind::TrackSettings => 0x0B9EB87E,
            FormatKind::LotDescription => 0x0BF999E7,
            FormatKind::BinaryIndex => 0x0C560F39,
            FormatKind::PoolSurface => 0x0C900FDB,
            FormatKind::TextureResource => 0x1C4A276C,
            FormatKind::AudioFile => 0x2026960B,
            FormatKind::SceneNode => 0x25232B11,
            FormatKind::ThreeDArray => 0x2A51171B,
            FormatKind::TextureOverlayXML => 0x2C1FD8A1,
            FormatKind::Popups => 0x2C310F46,
            FormatKind::SimScores => 0x3053CF74,
            FormatKind::BehaviorConstant => 0x42434F4E,
            FormatKind::BehaviorFunction => 0x42484156,
            FormatKind::BitmapImage => 0x424D505F,
            FormatKind::CatalogString => 0x43415453,
            FormatKind::ImageLink => 0x43494745,
            FormatKind::CatalogDescription => 0x43545353,
            FormatKind::Drawgroup => 0x44475250,
            FormatKind::FaceProperties => 0x46414345,
            FormatKind::FamilyInformation => 0x46414D49,
            FormatKind::FamilyData => 0x46414D68,
            FormatKind::Function => 0x46434E53,
            FormatKind::AudioReference => 0x46574156,
            FormatKind::GlobalData => 0x474C4F42,
            FormatKind::HouseDescriptor => 0x484F5553,
            FormatKind::TexturedMaterial => 0x49596978,
            FormatKind::WorldDatabase => 0x49FF7D76,
            FormatKind::LotTexture => 0x4B58975B,
            FormatKind::SkinToneXml => 0x4C158081,
            FormatKind::CinematicScene => 0x4D51F042,
            FormatKind::NeighborhoodMemory => 0x4E474248,
            FormatKind::NameReference => 0x4E524546,
            FormatKind::NameMap => 0x4E6D6150,
            FormatKind::ObjectData => 0x4F424A44,
            FormatKind::ObjectFunction => 0x4F424A66,
            FormatKind::ObjectMetadata => 0x4F626A4D,
            FormatKind::ImageColorPalette => 0x50414C54,
            FormatKind::SimPersonalInformation => 0x50455253,
            FormatKind::StackScript => 0x504F5349,
            FormatKind::PackageToolkit => 0x50544250,
            FormatKind::SimInformation => 0x53494D49,
            FormatKind::ObjectSlot => 0x534C4F54,
            FormatKind::Sprites => 0x53505232,
            FormatKind::TextLists => 0x53545223,
            FormatKind::TTAT => 0x54415454,
            FormatKind::BehaviorFunctionLabels => 0x54505250,
            FormatKind::BehaviorConstantLabels => 0x5452434E,
            FormatKind::BehaviorFlowchartTree => 0x54524545,
            FormatKind::PieMenuFunctions => 0x54544142,
            FormatKind::PieMenuSettings => 0x54544173,
            FormatKind::MaterialObject => 0x584D544F,
            FormatKind::UnknownObject => 0x584F424A,
            FormatKind::EnvironmentCubeLighting => 0x6A97042F,
            FormatKind::TwoDArray => 0x6B943B43,
            FormatKind::LotDefinition => 0x6C589723,
            FormatKind::Object => 0x6F626A74,
            FormatKind::Hitlist => 0x7B1ACFCD,
            FormatKind::GeometricNode => 0x7BA3838C,
            FormatKind::Lightmap => 0x856DDBAC,
            FormatKind::WallLayer => 0x8A84D7B0,
            FormatKind::Unknown1 => 0x8B0C79D6,
            FormatKind::JpegImage => 0x8C3CE95A,
            FormatKind::FamilyTies => 0x8C870743,
            FormatKind::PredictiveMaps => 0x8CC0A14B,
            FormatKind::SoundEffects => 0x8DB5E4C2,
            FormatKind::Unknown2 => 0x9D796DB4,
            FormatKind::SimDescription => 0xAACE2EFB,
            FormatKind::FencePostLayer => 0xAB4BA572,
            FormatKind::Roof => 0xAB9406AA,
            FormatKind::LotTerrainGeometry => 0xABCB5DA4,
            FormatKind::NeighborhoodTerrain => 0xABD0DC63,
            FormatKind::LinearFogLighting => 0xAC06A66F,
            FormatKind::DrawStateLighting => 0xAC06A676,
            FormatKind::GeometricDataContainer => 0xAC4F8687,
            FormatKind::ThreeDReference => 0xAC506764,
            FormatKind::IdNumber => 0xAC8A7A2E,
            FormatKind::WeatherInfo => 0xB21BE28B,
            FormatKind::TssgSystem => 0xBA353CE1,
            FormatKind::Light => 0xC9C81B9B,
            FormatKind::StringMap => 0xCAC4FC40,
            FormatKind::VertexLayer => 0xCB4387A1,
            FormatKind::Unknown3 => 0xCC2A6A34,
            FormatKind::SimRelations => 0xCC364C2A,
            FormatKind::Unknown4 => 0xCC8A6A69,
            FormatKind::FacialStructure => 0xCCCEF852,
            FormatKind::MaxisMaterialShader => 0xCD7FE87A,
            FormatKind::SimWantsAndFears => 0xCD95548E,
            FormatKind::ContentRegistry => 0xCDB467B8,
            FormatKind::CreationResource => 0xE519C933,
            FormatKind::Directory => 0xE86B1EEF,
            FormatKind::EffectsResourceTree => 0xEA5118B0,
            FormatKind::PropertySet => 0xEBCF3E27,
            FormatKind::VersionInformation => 0xEBFEE342,
            FormatKind::NeighborhoodView => 0xEC44BDDC,
            FormatKind::LargeImage => 0xED534136,
            FormatKind::SingularLotObject => 0xFA1C39F7,
            FormatKind::Animation => 0xFB00791E,
            FormatKind::Shape => 0xFC6EB1F7,
        }
    }
}
