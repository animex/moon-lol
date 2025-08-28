use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use league_core::{
    BarracksConfig, CharacterRecord, Unk0x3c2bf0c0, Unk0x9d9f60d2, Unk0xc71ee7fb,
    VfxSystemDefinitionData,
};
use lol_core::Lane;

use crate::ConfigCharacterSkin;

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct ConfigMap {
    pub geometry_objects: Vec<ConfigGeometryObject>,
    pub minion_paths: HashMap<Lane, Vec<Vec2>>,
    pub barracks: HashMap<u32, Unk0xc71ee7fb>,
    pub characters: HashMap<u32, Unk0x9d9f60d2>,
    pub barrack_configs: HashMap<u32, BarracksConfig>,
    pub environment_objects: HashMap<u32, Unk0x3c2bf0c0>,
    pub skins: HashMap<String, ConfigCharacterSkin>,
    pub character_records: HashMap<String, CharacterRecord>,
    pub vfx_system_definition_datas: HashMap<u32, VfxSystemDefinitionData>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigGeometryObject {
    pub mesh_path: String,
    pub material_path: String,
}
