use std::io::{Cursor, Read};

use binrw::{io::NoSeek, BinRead};
use league_file::LeagueMapGeo;
use league_property::{from_entry, PropFile};
use league_utils::{hash_bin, hash_wad};

use league_core::StaticMaterialDef;
use lol_config::LeagueMaterial;

use crate::{Error, LeagueWadLoader};

pub struct LeagueWadMapLoader {
    pub wad_loader: LeagueWadLoader,
    pub map_geo: LeagueMapGeo,
    pub materials_bin: PropFile,
}

impl LeagueWadMapLoader {
    pub fn from_loader(
        wad_loader: LeagueWadLoader,
        map: &str,
    ) -> Result<LeagueWadMapLoader, Error> {
        let map_geo_path = format!("data/maps/mapgeometry/map11/{}.mapgeo", map);

        let entry = wad_loader.wad.get_entry(hash_wad(&map_geo_path))?;

        let reader = wad_loader.get_wad_zstd_entry_reader(&entry)?;

        let map_geo = LeagueMapGeo::read(&mut NoSeek::new(reader))?;

        let map_materials_bin_path = format!("data/maps/mapgeometry/map11/{}.materials.bin", map);

        let entry = wad_loader
            .wad
            .get_entry(hash_wad(&map_materials_bin_path))?;

        let mut reader = wad_loader.get_wad_zstd_entry_reader(&entry)?;

        let mut data = Vec::with_capacity(entry.target_size as usize);

        reader.read_to_end(&mut data)?;

        let materials_bin = PropFile::read(&mut Cursor::new(data))?;

        Ok(LeagueWadMapLoader {
            wad_loader,
            map_geo,
            materials_bin,
        })
    }

    pub fn load_image_for_submesh(&self, material_name: &str) -> Option<LeagueMaterial> {
        // 1. 根据材质名查找 texturePath

        let entry = self
            .materials_bin
            .entries
            .iter()
            .find(|v| v.hash == hash_bin(material_name))?;

        let material = from_entry::<StaticMaterialDef>(entry);

        // 2. 将列表转换为可迭代的 BinEmbed
        let embedded_samplers = material.sampler_values?;

        // 3. 遍历所有 sampler，查找第一个包含 "texturePath" 的
        // `find_map` 会在找到第一个 Some(T) 后立即停止，比 filter_map + collect + first 更高效
        let texture_path = embedded_samplers.into_iter().find_map(|sampler_item| {
            let texture_name = &sampler_item.texture_name;
            if !(texture_name == "DiffuseTexture" || texture_name == "Diffuse_Texture") {
                return None;
            }
            sampler_item.texture_path
        });

        if let Some(texture_path) = texture_path {
            return Some(LeagueMaterial {
                texture_path: texture_path,
            });
        }

        None
    }
}
