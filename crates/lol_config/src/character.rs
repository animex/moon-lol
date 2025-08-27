use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct LeagueMaterial {
    pub texture_path: String,
}
