////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::binrw;
use enum_assoc::Assoc;
#[cfg(test)]
use test_strategy::Arbitrary;

#[binrw]
#[brw(little, repr = u32)]
#[repr(u32)]
#[derive(Assoc, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[cfg_attr(test, derive(Arbitrary))]
#[func(pub const fn short_name(&self) -> &'static str)]
pub enum Id {
    #[assoc(short_name = "UI")]
    UiData = 0x0,
    #[assoc(short_name = "WGRA")]
    WallGraph = 0x0A28_4D0B,
    #[assoc(short_name = "TRKS")]
    TrackSettings = 0x0B9E_B87E,
    #[assoc(short_name = "DESC")]
    LotDescription = 0x0BF9_99E7,
    #[assoc(short_name = "BINX")]
    BinaryIndex = 0x0C56_0F39,
    #[assoc(short_name = "POOL")]
    PoolSurface = 0x0C90_0FDB,
    #[assoc(short_name = "TXTR")]
    TextureResource = 0x1C4A_276C,
    #[assoc(short_name = "XA")]
    AudioFile = 0x2026_960B,
    #[assoc(short_name = "5SC")]
    SceneNode = 0x2523_2B11,
    #[assoc(short_name = "3ARY")]
    ThreeDArray = 0x2A51_171B,
    #[assoc(short_name = "XTOL")]
    TextureOverlayXML = 0x2C1F_D8A1,
    #[assoc(short_name = "POPS")]
    Popups = 0x2C31_0F46,
    #[assoc(short_name = "SCOR")]
    SimScores = 0x3053_CF74,
    #[assoc(short_name = "BCON")]
    BehaviorConstant = 0x4243_4F4E,
    #[assoc(short_name = "BHAV")]
    BehaviorFunction = 0x4248_4156,
    #[assoc(short_name = "BMP")]
    BitmapImage = 0x424D_505F,
    #[assoc(short_name = "CATS")]
    CatalogString = 0x4341_5453,
    #[assoc(short_name = "CIGE")]
    ImageLink = 0x4349_4745,
    #[assoc(short_name = "CTSS")]
    CatalogDescription = 0x4354_5353,
    #[assoc(short_name = "DGRP")]
    Drawgroup = 0x4447_5250,
    #[assoc(short_name = "FACE")]
    FaceProperties = 0x4641_4345,
    #[assoc(short_name = "FAMI")]
    FamilyInformation = 0x4641_4D49,
    #[assoc(short_name = "FAMh")]
    FamilyData = 0x4641_4D68,
    #[assoc(short_name = "FCNS")]
    Function = 0x4643_4E53,
    #[assoc(short_name = "FWAV")]
    AudioReference = 0x4657_4156,
    #[assoc(short_name = "GLOB")]
    GlobalData = 0x474C_4F42,
    #[assoc(short_name = "HOUS")]
    HouseDescriptor = 0x484F_5553,
    #[assoc(short_name = "TXMT")]
    TexturedMaterial = 0x4959_6978,
    #[assoc(short_name = "WRLD")]
    WorldDatabase = 0x49FF_7D76,
    #[assoc(short_name = "LTTX")]
    LotTexture = 0x4B58_975B,
    #[assoc(short_name = "XSTN")]
    SkinToneXml = 0x4C15_8081,
    #[assoc(short_name = "CINE")]
    CinematicScene = 0x4D51_F042,
    #[assoc(short_name = "NGBH")]
    NeighborhoodMemory = 0x4E47_4248,
    #[assoc(short_name = "NREF")]
    NameReference = 0x4E52_4546,
    #[assoc(short_name = "NMAP")]
    NameMap = 0x4E6D_6150,
    #[assoc(short_name = "OBJD")]
    ObjectData = 0x4F42_4A44,
    #[assoc(short_name = "OBJF")]
    ObjectFunction = 0x4F42_4A66,
    #[assoc(short_name = "OBJM")]
    ObjectMetadata = 0x4F62_6A4D,
    #[assoc(short_name = "PALT")]
    ImageColorPalette = 0x5041_4C54,
    #[assoc(short_name = "PERS")]
    SimPersonalInformation = 0x5045_5253,
    #[assoc(short_name = "POSI")]
    StackScript = 0x504F_5349,
    #[assoc(short_name = "PTBP")]
    PackageToolkit = 0x5054_4250,
    #[assoc(short_name = "SIMI")]
    SimInformation = 0x5349_4D49,
    #[assoc(short_name = "SLOT")]
    ObjectSlot = 0x534C_4F54,
    #[assoc(short_name = "SPR2")]
    Sprites = 0x5350_5232,
    #[assoc(short_name = "STR#")]
    TextLists = 0x5354_5223,
    #[assoc(short_name = "TTAT")]
    TTAT = 0x5441_5454,
    #[assoc(short_name = "TPRP")]
    BehaviorFunctionLabels = 0x5450_5250,
    #[assoc(short_name = "TRCN")]
    BehaviorConstantLabels = 0x5452_434E,
    #[assoc(short_name = "TREE")]
    BehaviorFlowchartTree = 0x5452_4545,
    #[assoc(short_name = "TTAB")]
    PieMenuFunctions = 0x5454_4142,
    #[assoc(short_name = "TTAs")]
    PieMenuSettings = 0x5454_4173,
    #[assoc(short_name = "XMTO")]
    MaterialObject = 0x584D_544F,
    #[assoc(short_name = "XOBJ")]
    UnknownObject = 0x584F_424A,
    #[assoc(short_name = "5EL")]
    EnvironmentCubeLighting = 0x6A97_042F,
    #[assoc(short_name = "2ARY")]
    TwoDArray = 0x6B94_3B43,
    #[assoc(short_name = "LOT")]
    LotDefinition = 0x6C58_9723,
    #[assoc(short_name = "MOBJT")]
    Object = 0x6F62_6A74,
    #[assoc(short_name = "HLS")]
    Hitlist = 0x7B1A_CFCD,
    #[assoc(short_name = "GMND")]
    GeometricNode = 0x7BA3_838C,
    #[assoc(short_name = "LTMP")]
    Lightmap = 0x856D_DBAC,
    #[assoc(short_name = "WLL")]
    WallLayer = 0x8A84_D7B0,
    #[assoc(short_name = "UNK1")]
    Unknown1 = 0x8B0C_79D6,
    #[assoc(short_name = "JPG")]
    JpegImage = 0x8C3C_E95A,
    #[assoc(short_name = "FAMt")]
    FamilyTies = 0x8C87_0743,
    #[assoc(short_name = "PMAP")]
    PredictiveMaps = 0x8CC0_A14B,
    #[assoc(short_name = "SFX")]
    SoundEffects = 0x8DB5_E4C2,
    #[assoc(short_name = "UNK2")]
    Unknown2 = 0x9D79_6DB4,
    #[assoc(short_name = "PDAT")]
    SimDescription = 0xAACE_2EFB,
    #[assoc(short_name = "FPL")]
    FencePostLayer = 0xAB4B_A572,
    #[assoc(short_name = "ROOF")]
    Roof = 0xAB94_06AA,
    #[assoc(short_name = "LOTG")]
    LotTerrainGeometry = 0xABCB_5DA4,
    #[assoc(short_name = "NHTR")]
    NeighborhoodTerrain = 0xABD0_DC63,
    #[assoc(short_name = "5LF")]
    LinearFogLighting = 0xAC06_A66F,
    #[assoc(short_name = "5DS")]
    DrawStateLighting = 0xAC06_A676,
    #[assoc(short_name = "GMDC")]
    GeometricDataContainer = 0xAC4F_8687,
    #[assoc(short_name = "3IDR")]
    ThreeDReference = 0xAC50_6764,
    #[assoc(short_name = "NID")]
    IdNumber = 0xAC8A_7A2E,
    #[assoc(short_name = "WTHR")]
    WeatherInfo = 0xB21B_E28B,
    #[assoc(short_name = "TSSG")]
    TssgSystem = 0xBA35_3CE1,
    #[assoc(short_name = "LGHT")]
    Light = 0xC9C8_1B9B,
    #[assoc(short_name = "SMAP")]
    StringMap = 0xCAC4_FC40,
    #[assoc(short_name = "VERT")]
    VertexLayer = 0xCB43_87A1,
    #[assoc(short_name = "UNK3")]
    Unknown3 = 0xCC2A_6A34,
    #[assoc(short_name = "SREL")]
    SimRelations = 0xCC36_4C2A,
    #[assoc(short_name = "UNK4")]
    Unknown4 = 0xCC8A_6A69,
    #[assoc(short_name = "LxNR")]
    FacialStructure = 0xCCCE_F852,
    #[assoc(short_name = "MATSHAD")]
    MaxisMaterialShader = 0xCD7F_E87A,
    #[assoc(short_name = "SWAF")]
    SimWantsAndFears = 0xCD95_548E,
    #[assoc(short_name = "CREG")]
    ContentRegistry = 0xCDB4_67B8,
    #[assoc(short_name = "CRES")]
    CreationResource = 0xE519_C933,
    #[assoc(short_name = "DIR")]
    Directory = 0xE86B_1EEF,
    #[assoc(short_name = "FX")]
    EffectsResourceTree = 0xEA51_18B0,
    #[assoc(short_name = "GZPS")]
    PropertySet = 0xEBCF_3E27,
    #[assoc(short_name = "VERS")]
    VersionInformation = 0xEBFE_E342,
    #[assoc(short_name = "NHVW")]
    NeighborhoodView = 0xEC44_BDDC,
    #[assoc(short_name = "LIFO")]
    LargeImage = 0xED53_4136,
    #[assoc(short_name = "OBJT")]
    SingularLotObject = 0xFA1C_39F7,
    #[assoc(short_name = "ANIM")]
    Animation = 0xFB00_791E,
    #[assoc(short_name = "SHPE")]
    Shape = 0xFC6E_B1F7,
}
