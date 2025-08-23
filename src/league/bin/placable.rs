use std::collections::HashMap;

use bevy::math::{Mat4, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MapPlaceableContainer {
    pub items: HashMap<u32, MapPlaceable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MapPlaceable {
    #[serde(rename_all = "camelCase")]
    GdsMapObject {
        transform: Mat4,
        name: String,
        r#type: u8,
        box_min: Vec3,
        box_max: Vec3,
        extra_info: Vec<GDSMapObjectBannerInfo>,
    },
    #[serde(rename_all = "camelCase")]
    MapGroup { transform: Mat4, name: String },
    #[serde(rename_all = "camelCase")]
    Unk3c2bf0c0 { transform: Mat4, name: u32 },
    #[serde(rename_all = "camelCase")]
    Unk3c995caf {
        transform: Mat4,
        name: String,
        segments: Vec<Vec3>,
    },
    // #[serde(rename_all = "camelCase")]
    // Unk7ad3dda { transform: Mat4, name: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GDSMapObjectBannerInfo {
    banner_data: u32,
}
