use bevy::prelude::*;
use binrw::binread;
use league_core::{
    JungleQuadrantFlags, MainRegionFlags, NearestLaneFlags, POIFlags, RingFlags, RiverRegionFlags,
    UnknownSRXFlags, VisionPathingFlags,
};
use league_utils::parse_vec3;

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

#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct Header {
    pub major_version: u8,
    pub minor_version: i16,
    #[br(map = parse_vec3)]
    pub min_bounds: Vec3,
    #[br(map = parse_vec3)]
    pub max_bounds: Vec3,
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
