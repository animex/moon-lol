use std::collections::HashMap;

use bevy::{
    ecs::resource::Resource,
    math::{vec3, Mat4, Vec2, Vec3},
    transform::components::Transform,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::{Health, Lane, Team},
    entities::Barrack,
    league::{
        JungleQuadrantFlags, MainRegionFlags, NearestLaneFlags, POIFlags, RingFlags,
        RiverRegionFlags, UnknownSRXFlags, VisionPathingFlags,
    },
};

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct ConfigGame {
    pub legends: Vec<ConfigLegend>,
}

type ConfigLegend = (Transform, Team, ConfigCharacterSkin);

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct ConfigMap {
    pub geometry_objects: Vec<ConfigGeometryObject>,
    pub environment_objects: Vec<(Transform, ConfigCharacterSkin, Option<Health>)>,
    pub minion_paths: HashMap<Lane, Vec<Vec2>>,
    pub barracks: Vec<(Transform, Team, Lane, Barrack)>,
    pub navigation_grid: ConfigNavigationGrid,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigGeometryObject {
    pub mesh_path: String,
    pub material_path: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigCharacterSkin {
    pub animation_map: HashMap<u32, ConfigCharacterSkinAnimation>,
    pub inverse_bind_pose_path: String,
    pub joint_influences_indices: Vec<i16>,
    pub joints: Vec<ConfigJoint>,
    pub material_path: String,
    pub skin_scale: Option<f32>,
    pub submesh_paths: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ConfigCharacterSkinAnimation {
    AtomicClipData {
        clip_path: String,
    },
    ConditionFloatClipData {
        conditions: Vec<(u32, f32)>,
        component_name: String,
        field_name: String,
    },
    SelectorClipData {
        probably_nodes: Vec<(u32, f32)>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigJoint {
    pub hash: u32,
    pub transform: Transform,
    pub parent_index: i16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigSkinnedMeshInverseBindposes {
    pub inverse_bindposes: Vec<Mat4>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ConfigNavigationGrid {
    pub min_grid_pos: Vec3,
    pub cell_size: f32,
    pub x_len: usize,
    pub y_len: usize,
    pub cells: Vec<Vec<ConfigNavigationGridCell>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigNavigationGridCell {
    pub y: f32,
    pub heuristic: f32,
    pub vision_pathing_flags: VisionPathingFlags,
    pub river_region_flags: RiverRegionFlags,
    pub jungle_quadrant_flags: JungleQuadrantFlags,
    pub main_region_flags: MainRegionFlags,
    pub nearest_lane_flags: NearestLaneFlags,
    pub poi_flags: POIFlags,
    pub ring_flags: RingFlags,
    pub srx_flags: UnknownSRXFlags,
}

impl ConfigNavigationGridCell {
    pub fn is_wall(&self) -> bool {
        self.vision_pathing_flags.contains(VisionPathingFlags::Wall)
    }

    pub fn is_walkable(&self) -> bool {
        !self.is_wall()
    }
}

impl ConfigNavigationGrid {
    pub fn get_offset(&self) -> Vec2 {
        Vec2::new(
            self.min_grid_pos.x + self.cell_size / 2.0,
            self.min_grid_pos.z + self.cell_size / 2.0,
        )
    }

    pub fn get_cell_center_position_by_xy(&self, pos: (usize, usize)) -> Vec3 {
        let offset = self.get_offset();
        Vec3::new(
            offset.x + pos.1 as f32 * self.cell_size,
            self.get_cell_by_xy(pos).y,
            -(offset.y + pos.0 as f32 * self.cell_size),
        )
    }

    pub fn get_cell_xy_by_position(&self, pos: &Vec2) -> (usize, usize) {
        let offset = self.get_offset();
        let x = ((-pos.y - offset.y) / self.cell_size).round() as usize;
        let y = ((pos.x - offset.x) / self.cell_size).round() as usize;

        (x, y)
    }

    pub fn get_cell_by_xy(&self, pos: (usize, usize)) -> &ConfigNavigationGridCell {
        &self.cells[pos.0.clamp(0, self.x_len - 1)][pos.1.clamp(0, self.y_len - 1)]
    }

    pub fn get_cell_by_position(&self, pos: &Vec2) -> &ConfigNavigationGridCell {
        self.get_cell_by_xy(self.get_cell_xy_by_position(pos))
    }

    pub fn get_world_position_by_position(&self, pos: &Vec2) -> Vec3 {
        vec3(pos.x, self.get_cell_by_position(pos).y, pos.y)
    }

    pub fn get_position_by_float_xy(&self, pos: &Vec2) -> Vec2 {
        let offset = self.get_offset();
        Vec2::new(
            offset.x + pos.y * self.cell_size,
            -(offset.y + pos.x * self.cell_size),
        )
    }

    pub fn get_map_center_position(&self) -> Vec3 {
        let offset = self.get_offset();
        Vec3::new(
            offset.x + self.cell_size * self.x_len as f32 / 2.0,
            0.0,
            -(offset.y + self.cell_size * self.y_len as f32 / 2.0),
        )
    }
}
