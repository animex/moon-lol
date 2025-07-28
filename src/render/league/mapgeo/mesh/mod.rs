pub mod enums;
pub mod mesh_creation;
pub mod skinned;
pub mod static_mesh;
pub mod texture_loading;
pub mod types;
pub mod vertex_parsing;

pub use enums::{LayerTransitionBehavior, QualityFilter};
pub use skinned::{LeagueSkinnedMesh, LeagueSkinnedMeshInternal, SkinnedMeshRange, SkinnedMeshVertex, BoundingSphere};
pub use static_mesh::LeagueMapGeoMesh;
pub use types::{Channel, Submesh, TextureOverride};

use crate::render::{LeagueLoader, LeagueMapGeo};
use bevy::prelude::{Image, Mesh};
use cdragon_prop::PropFile;

use self::mesh_creation::create_bevy_mesh_for_submesh;
use self::texture_loading::find_and_load_image_for_submesh;
use self::vertex_parsing::parse_vertex_data;

// ---- 主协调函数 ----
/// 处理单个 MapGeoMesh，并行地为其所有 submesh 创建 Bevy Mesh 和 Image。
pub fn process_map_geo_mesh(
    map_materials: &PropFile,
    map_file: &LeagueMapGeo,
    map_mesh: &LeagueMapGeoMesh,
    league_loader: &LeagueLoader,
) -> Vec<(Mesh, Image)> {
    // 步骤 1: 一次性解析所有顶点数据
    let (all_positions, all_normals, all_uvs) = parse_vertex_data(map_file, map_mesh);

    // 步骤 2: 一次性获取索引缓冲区的引用
    let index_buffer = &map_file.index_buffers[map_mesh.index_buffer_id as usize];
    let all_indices = &index_buffer.buffer;

    // 步骤 3: 并行处理所有 submesh
    let result_bundles: Vec<(Mesh, Image)> = map_mesh
        .submeshes
        .iter()
        .filter_map(|submesh| {
            // 获取当前 submesh 对应的索引切片
            let start = submesh.start_index as usize;
            let end = start + submesh.submesh_index_count as usize;
            if end > all_indices.len() {
                return None;
            }
            let global_indices_slice = &all_indices[start..end];

            // 为 submesh 创建独立的 Mesh
            let bevy_mesh = create_bevy_mesh_for_submesh(
                global_indices_slice,
                &all_positions,
                &all_normals,
                &all_uvs,
            );

            // 为 submesh 查找并加载贴图
            let bevy_image = find_and_load_image_for_submesh(submesh, map_materials, league_loader);

            Some((bevy_mesh, bevy_image))
        })
        .collect();

    result_bundles
}
