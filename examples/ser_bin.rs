use std::fs::File;
use std::io::BufReader;

use binrw::BinRead;
use moon_lol::league::{BarracksConfig, MapPlaceableContainer, PropFile};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VfxSystemDefinitionData {
    pub complex_emitter_definition_data: Vec<VfxEmitterDefinitionData>,
    pub particle_name: String,
    pub particle_path: String,
    pub flags: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VfxEmitterDefinitionData {
    pub emitter_name: String,
    pub primitive: VfxPrimitive,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VfxPrimitive {
    #[serde(rename_all = "camelCase")]
    VfxPrimitiveMesh {
        m_mesh: VfxMeshDefinitionData,
    },
    VfxPrimitiveArbitraryQuad,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VfxMeshDefinitionData {
    pub m_simple_mesh_name: String,
}

fn main() {
    let path = "assets/bloom.materials.bin";

    println!("尝试读取文件: {}", path);

    let file = File::open(path).unwrap();

    let prop_file = PropFile::read(&mut BufReader::new(file)).unwrap();

    let data = bin_deserializer::from_entry::<MapPlaceableContainer>(
        &prop_file
            .entries
            .iter()
            .find(|v| v.hash == 0x27264c39)
            .unwrap()
            .data,
    )
    .unwrap();
    println!("反序列化成功，结果: {:#?}", data);

    // let data = bin_deserializer::from_entry::<BarracksConfig>(
    //     &prop_file
    //         .entries
    //         .iter()
    //         .find(|v| v.hash == 0x147211fb)
    //         .unwrap()
    //         .data,
    // )
    // .unwrap();

    // println!("反序列化成功，结果: {:#?}", data);
}

pub mod bin_deserializer {
    use moon_lol::league::{BinDeserializer, BinDeserializerError, BinType};

    use super::*;

    pub fn from_entry<'de, T>(slice: &'de [u8]) -> Result<T, BinDeserializerError>
    where
        T: Deserialize<'de>,
    {
        let mut deserializer = BinDeserializer::from_bytes(slice, BinType::Entry);
        T::deserialize(&mut deserializer)
    }
}
