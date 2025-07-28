use super::types::Submesh;
use crate::render::LeagueLoader;
use bevy::asset::RenderAssetUsages;
use bevy::image::{Image, ImageSampler};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use cdragon_prop::{BinEmbed, BinEntry, BinList, BinString, PropFile};

/// 根据 submesh 的材质名，查找材质属性并加载对应的贴图文件。
/// 如果失败，则返回一个默认的白色贴图。
pub fn find_and_load_image_for_submesh(
    submesh: &Submesh,
    map_materials: &PropFile,
    league_loader: &LeagueLoader,
) -> Image {
    let is_debug =
        submesh.material_name.text == "Maps/KitPieces/SRX/Chemtech/Materials/Default/Chemtech_ChemtechDecal";
    // 1. 根据材质名查找 texturePath
    let binhash = LeagueLoader::compute_binhash(&submesh.material_name.text);
    if is_debug {
        // println!("binhash: {:x}", binhash);
    }
    for entry in &map_materials.entries {
        if entry.path.hash == binhash {
            // ... 你原来的材质查找逻辑 ...
            // 假设你找到了 texture_path: String
            if is_debug {
                // println!("found binhash: {:#?}", entry);
            }
            if let Some(texture_path) = find_texture_path_in_material_entry(entry) {
                if is_debug {
                    // println!("texture_path: {}", texture_path);
                }
                match league_loader.get_image_by_texture_path(&texture_path) {
                    Ok(image) => return image,
                    Err(e) => warn!("Failed to load texture {}: {}", texture_path, e),
                }
            }
        }
    }

    println!("没找到{}，加载备用贴图", submesh.material_name.text);
    // 2. 如果找不到或加载失败，返回一个备用贴图
    let mut fallback = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255], // White RGBA
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    fallback.sampler = ImageSampler::linear();
    fallback
}

/// 在单个材质条目中查找 "texturePath" 的值。
/// 材质属性通常是嵌套的，结构为：samplerValues -> (list) -> (embed) -> texturePath。
/// 这里使用了 `find_map` 来高效地找到第一个匹配项。
fn find_texture_path_in_material_entry(
    material_entry: &BinEntry, // 请替换为你的材质条目具体类型
) -> Option<String> {
    // 1. 获取 "samplerValues" 列表
    let sampler_values =
        material_entry.getv::<BinList>(LeagueLoader::compute_binhash("samplerValues").into())?;

    // 2. 将列表转换为可迭代的 BinEmbed
    let embedded_samplers = sampler_values.downcast::<BinEmbed>()?;

    // 3. 遍历所有 sampler，查找第一个包含 "texturePath" 的
    // `find_map` 会在找到第一个 Some(T) 后立即停止，比 filter_map + collect + first 更高效
    embedded_samplers.into_iter().find_map(|sampler_item| {
        let texture_name = &sampler_item
            .getv::<BinString>(LeagueLoader::compute_binhash("textureName").into())?
            .0;
        if !(texture_name == "DiffuseTexture" || texture_name == "Diffuse_Texture") {
            return None;
        }
        sampler_item
            .getv::<BinString>(LeagueLoader::compute_binhash("texturePath").into())
            .map(|v| v.0.clone())
    })
}
