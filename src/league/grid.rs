use crate::league::BinVec3;
use bevy::prelude::*;
use binrw::binread;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[binread]
#[br(little)]
#[derive(Debug, Resource, Clone)]
pub struct AiMeshNGrid {
    pub header: Header,

    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    pub navigation_grid: Vec<NavigationGridCell>,

    // Vision pathing flags - read after all cells
    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    #[br(map = |v: Vec<u16>| v.into_iter().map(|v| VisionPathingFlags::from_bits(v).unwrap()).collect())]
    pub vision_pathing_flags: Vec<VisionPathingFlags>,

    // River region and other flags - read after vision pathing flags
    #[br(count = header.get_x_cell_count() * header.get_y_cell_count())]
    pub other_flags: Vec<OtherFlags>,

    // Unknown block - 8 blocks of 132 bytes each
    #[br(count = 8 * 132)]
    pub unknown_block: Vec<u8>,

    pub height_samples: HeightSamples,
    pub hint_nodes: HintNodes,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct VisionPathingFlags: u16 {
        const Walkable = 0;

        const Brush = 1 << 0;
        const Wall = 1 << 1;
        const StructureWall = 1 << 2;
        const Unobserved8 = 1 << 3;

        const Unobserved16 = 1 << 4;
        const Unobserved32 = 1 << 5;
        const TransparentWall = 1 << 6;
        // marks the difference between two otherwise-equivalent cells, spread sporadically throughout the map, ignored for a cleaner image since it doesn't seem useful at all
        const Unknown128 = 1 << 7;

        const AlwaysVisible = 1 << 8;
        // only ever found on the original Nexus Blitz map, and it was only present in two sections of what would otherwise be normal wall
        const Unknown512 = 1 << 9;
        const BlueTeamOnly = 1 << 10;
        const RedTeamOnly = 1 << 11;

        // no bits observed past this point
        const NeutralZoneVisiblity = 1 << 12;
    }
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct OtherFlags {
    #[br(map = |v: u8| RiverRegionFlags::from_bits(v).unwrap())]
    pub river_region_flags: RiverRegionFlags,

    #[br(temp)]
    jungle_quadrant_and_main_region_flags: u8,

    #[br(calc = JungleQuadrantFlags::from_bits(jungle_quadrant_and_main_region_flags & 0x0f).unwrap())]
    pub jungle_quadrant_flags: JungleQuadrantFlags,
    #[br(calc = MainRegionFlags::from((jungle_quadrant_and_main_region_flags & 0xf0) >> 4))]
    pub main_region_flags: MainRegionFlags,

    #[br(temp)]
    nearest_lane_and_poi_flags: u8,

    #[br(calc = NearestLaneFlags::from(nearest_lane_and_poi_flags & 0x0f))]
    pub nearest_lane_flags: NearestLaneFlags,

    #[br(calc = POIFlags::from((nearest_lane_and_poi_flags & 0xf0) >> 4))]
    pub poi_flags: POIFlags,

    #[br(temp)]
    ring_and_srx_flags: u8,

    #[br(calc = RingFlags::from(ring_and_srx_flags & 0x0f))]
    pub ring_flags: RingFlags,

    #[br(calc = UnknownSRXFlags::from((ring_and_srx_flags & 0xf0) >> 4))]
    pub srx_flags: UnknownSRXFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct RiverRegionFlags: u8 {
        const NonJungle = 0;

        const JungleQuadrant = 1 << 0;
        const BaronPit = 1 << 1;
        const Unobserved4 = 1 << 2;
        const Unobserved8 = 1 << 3;

        const River = 1 << 4;
        // only ever found on the original Nexus Blitz map, where it was instead used to represent the river (other flags were shuffled too)
        const Unknown32 = 1 << 5;
        // no bits observed past this point
        const RiverEntrance = 1 << 6;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct JungleQuadrantFlags: u8 {
        const None = 0;

        const NorthJungleQuadrant = 1 << 0;
        const EastJungleQuadrant = 1 << 1;
        const WestJungleQuadrant = 1 << 2;
        const SouthJungleQuadrant = 1 << 3;

        const Unobserved8 = 1 << 4;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MainRegionFlags {
    Spawn = 0,
    Base = 1,

    TopLane = 2,
    MidLane = 3,
    BotLane = 4,

    TopSideJungle = 5,
    BotSideJungle = 6,

    TopSideRiver = 7,
    BotSideRiver = 8,

    TopSideBasePerimeter = 9,
    BotSideBasePerimeter = 10,

    TopSideLaneAlcove = 11,
    BotSideLaneAlcove = 12,
}

impl From<u8> for MainRegionFlags {
    fn from(value: u8) -> Self {
        match value {
            0 => MainRegionFlags::Spawn,
            1 => MainRegionFlags::Base,
            2 => MainRegionFlags::TopLane,
            3 => MainRegionFlags::MidLane,
            4 => MainRegionFlags::BotLane,
            5 => MainRegionFlags::TopSideJungle,
            6 => MainRegionFlags::BotSideJungle,
            7 => MainRegionFlags::TopSideRiver,
            8 => MainRegionFlags::BotSideRiver,
            9 => MainRegionFlags::TopSideBasePerimeter,
            10 => MainRegionFlags::BotSideBasePerimeter,
            11 => MainRegionFlags::TopSideLaneAlcove,
            12 => MainRegionFlags::BotSideLaneAlcove,
            _ => MainRegionFlags::Spawn, // 默认值
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NearestLaneFlags {
    BlueSideTopLane = 0,
    BlueSideMidLane = 1,
    BlueSideBotLane = 2,

    RedSideTopLane = 3,
    RedSideMidLane = 4,
    RedSideBotLane = 5,

    BlueSideTopNeutralZone = 6,
    BlueSideMidNeutralZone = 7,
    BlueSideBotNeutralZone = 8,

    RedSideTopNeutralZone = 9,
    RedSideMidNeutralZone = 10,
    RedSideBotNeutralZone = 11,
}

impl From<u8> for NearestLaneFlags {
    fn from(value: u8) -> Self {
        match value {
            0 => NearestLaneFlags::BlueSideTopLane,
            1 => NearestLaneFlags::BlueSideMidLane,
            2 => NearestLaneFlags::BlueSideBotLane,
            3 => NearestLaneFlags::RedSideTopLane,
            4 => NearestLaneFlags::RedSideMidLane,
            5 => NearestLaneFlags::RedSideBotLane,
            6 => NearestLaneFlags::BlueSideTopNeutralZone,
            7 => NearestLaneFlags::BlueSideMidNeutralZone,
            8 => NearestLaneFlags::BlueSideBotNeutralZone,
            9 => NearestLaneFlags::RedSideTopNeutralZone,
            10 => NearestLaneFlags::RedSideMidNeutralZone,
            11 => NearestLaneFlags::RedSideBotNeutralZone,
            _ => NearestLaneFlags::BlueSideTopLane, // 默认值
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum POIFlags {
    None = 0,

    NearTurret = 1,
    // note:  as of preseason 10, this flag now corresponds to cloud drake wind tunnels, and all following flags are removed
    CloudDrakeWindTunnelOrBaseGates = 2,

    BaronPit = 3,
    DragonPit = 4,

    CampRedBuff = 5,
    CampBlueBuff = 6,
    CampGromp = 7,
    CampKrugs = 8,
    CampRaptors = 9,
    CampMurkWolves = 10,
}

impl From<u8> for POIFlags {
    fn from(value: u8) -> Self {
        match value {
            0 => POIFlags::None,
            1 => POIFlags::NearTurret,
            2 => POIFlags::CloudDrakeWindTunnelOrBaseGates,
            3 => POIFlags::BaronPit,
            4 => POIFlags::DragonPit,
            5 => POIFlags::CampRedBuff,
            6 => POIFlags::CampBlueBuff,
            7 => POIFlags::CampGromp,
            8 => POIFlags::CampKrugs,
            9 => POIFlags::CampRaptors,
            10 => POIFlags::CampMurkWolves,
            _ => POIFlags::None, // 默认值
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RingFlags {
    BlueSpawnToNexus = 0,
    BlueNexusToInhib = 1,
    BlueInhibToInner = 2,
    BlueInnerToOuter = 3,
    BlueOuterToNeutral = 4,

    RedSpawnToNexus = 5,
    RedNexusToInhib = 6,
    RedInhibToInner = 7,
    RedInnerToOuter = 8,
    RedOuterToNeutral = 9,
}

impl From<u8> for RingFlags {
    fn from(value: u8) -> Self {
        match value {
            0 => RingFlags::BlueSpawnToNexus,
            1 => RingFlags::BlueNexusToInhib,
            2 => RingFlags::BlueInhibToInner,
            3 => RingFlags::BlueInnerToOuter,
            4 => RingFlags::BlueOuterToNeutral,
            5 => RingFlags::RedSpawnToNexus,
            6 => RingFlags::RedNexusToInhib,
            7 => RingFlags::RedInhibToInner,
            8 => RingFlags::RedInnerToOuter,
            9 => RingFlags::RedOuterToNeutral,
            _ => RingFlags::BlueSpawnToNexus, // 默认值
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnknownSRXFlags {
    Walkable = 0,

    Wall = 1,
    TransparentWall = 2,
    Brush = 3,
    Unobserved4 = 4,

    TopSideOceanDrakePuddle = 5,
    BotSideOceanDrakePuddle = 6,
    BlueTeamOnly = 7,
    RedTeamOnly = 8,

    Unobserved9 = 9,
    Unobserved10 = 10,
    BlueTeamOnlyNeutralZoneVisibility = 11,
    RedTeamOnlyNeutralZoneVisibility = 12,

    BrushWall = 13,
}

impl From<u8> for UnknownSRXFlags {
    fn from(value: u8) -> Self {
        match value {
            0 => UnknownSRXFlags::Walkable,
            1 => UnknownSRXFlags::Wall,
            2 => UnknownSRXFlags::TransparentWall,
            3 => UnknownSRXFlags::Brush,
            4 => UnknownSRXFlags::Unobserved4,
            5 => UnknownSRXFlags::TopSideOceanDrakePuddle,
            6 => UnknownSRXFlags::BotSideOceanDrakePuddle,
            7 => UnknownSRXFlags::BlueTeamOnly,
            8 => UnknownSRXFlags::RedTeamOnly,
            9 => UnknownSRXFlags::Unobserved9,
            10 => UnknownSRXFlags::Unobserved10,
            11 => UnknownSRXFlags::BlueTeamOnlyNeutralZoneVisibility,
            12 => UnknownSRXFlags::RedTeamOnlyNeutralZoneVisibility,
            13 => UnknownSRXFlags::BrushWall,
            _ => UnknownSRXFlags::Walkable, // 默认值
        }
    }
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct Header {
    pub major_version: u8,
    pub minor_version: i16,
    pub min_bounds: BinVec3,
    pub max_bounds: BinVec3,
    pub cell_size: f32,
    pub x_cell_count: u32,
    pub z_cell_count: u32,
}

impl Header {
    fn get_x_cell_count(&self) -> u32 {
        self.x_cell_count
    }

    fn get_y_cell_count(&self) -> u32 {
        self.z_cell_count
    }
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct NavigationGridCell {
    // center height (overridden by height samples)
    pub center_height: f32,
    // session ID
    pub session_id: i32,
    // arrival cost
    pub arrival_cost: f32,
    // is open
    pub is_open: i32,
    // heuristic
    pub heuristic: f32,
    // x coordinate
    #[br(map = |v: i16| v as usize)]
    pub x: usize,
    // z coordinate
    #[br(map = |v: i16| v as usize)]
    pub z: usize,
    // actor list
    pub actor_list: i32,
    // unknown 1
    pub unknown1: i32,
    // good cell session ID
    pub good_cell_session_id: i32,
    // hint weight
    pub hint_weight: f32,
    // unknown 2
    pub unknown2: i16,
    // arrival direction
    pub arrival_direction: i16,
    // hint node 1
    pub hint_node1: i16,
    // hint node 2
    pub hint_node2: i16,
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct HeightSamples {
    pub x_count: u32,
    pub z_count: u32,
    pub offset_x: f32,
    pub offset_z: f32,
    #[br(count = x_count * z_count)]
    pub samples: Vec<f32>,
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct HintNodes {
    // 900x900 matrix of distances
    #[br(count = 900 * 900)]
    pub distances: Vec<f32>,
    // 900 hint coordinates
    #[br(count = 900)]
    pub hint_coordinates: Vec<HintCoordinate>,
}

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct HintCoordinate {
    pub x: i16,
    pub y: i16,
}
