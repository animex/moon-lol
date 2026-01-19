use std::collections::HashMap;

use league_file::{ElementName, LeagueMapGeo, LeagueMapGeoMesh, Submesh};
use lol_config::IntermediateMesh;

/// Create intermediate structure from static mesh (submesh)
pub fn submesh_to_intermediate(
    submesh: &Submesh,
    map_file: &LeagueMapGeo,
    map_mesh: &LeagueMapGeoMesh,
    all_positions: &Vec<[f32; 3]>,
    all_normals: &Vec<[f32; 3]>,
    all_uvs: &Vec<[f32; 2]>,
) -> IntermediateMesh {
    // Get index data
    let index_buffer = map_file
        .index_buffers
        .get(map_mesh.index_buffer_id as usize)
        .unwrap();
    let all_indices = &index_buffer.buffer;

    // Get current submesh index range
    let start = submesh.start_index as usize;
    let end = start + submesh.submesh_index_count as usize;

    let global_indices_slice = &all_indices[start..end];

    // Create local vertex data and index mapping
    let mut local_positions = Vec::new();
    let mut local_normals = Vec::new();
    let mut local_uvs = Vec::new();
    let mut local_indices = Vec::with_capacity(global_indices_slice.len());
    let mut global_to_local_map = HashMap::new();

    // Remap vertex data, keeping only vertices used by current submesh
    for &global_index in global_indices_slice {
        let local_index = *global_to_local_map.entry(global_index).or_insert_with(|| {
            let new_local_index = local_positions.len() as u16;

            // Add position data
            if let Some(pos) = all_positions.get(global_index as usize) {
                local_positions.push(*pos);
            }

            // Add normal data
            if let Some(normal) = all_normals.get(global_index as usize) {
                local_normals.push(*normal);
            }

            // Add UV data
            if let Some(uv) = all_uvs.get(global_index as usize) {
                local_uvs.push(*uv);
            }

            new_local_index
        });
        local_indices.push(local_index);
    }

    // Create intermediate mesh structure
    let mut intermediate_mesh = IntermediateMesh::new(submesh.material_name.text.clone());

    intermediate_mesh.set_positions(local_positions);

    // Only set optional attributes when data is not empty
    if !local_normals.is_empty() {
        intermediate_mesh.set_normals(Some(local_normals));
    }

    if !local_uvs.is_empty() {
        intermediate_mesh.set_uvs(Some(local_uvs));
    }

    intermediate_mesh.set_indices(local_indices);
    intermediate_mesh.set_material_info(Some(submesh.material_name.text.clone()));

    intermediate_mesh
}

/// Parse all vertex attributes from MapGeoMesh as a shared global data pool.
/// Returns a tuple containing all vertex positions, normals, and UV coordinates.
pub fn parse_vertex_data(
    map_file: &LeagueMapGeo,
    map_mesh: &LeagueMapGeoMesh,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    // Pre-allocating capacity can slightly improve performance, but requires size estimation. Omitted for simplicity.
    let mut all_positions = Vec::new();
    let mut all_normals = Vec::new();
    let mut all_uvs = Vec::new();

    for v_decl_idx_offset in 0..map_mesh.vertex_declaration_count as usize {
        let decl_index = (map_mesh.vertex_declaration_index_base as usize) + v_decl_idx_offset;
        let v_buff_index = map_mesh.vertex_buffer_indexes[v_decl_idx_offset] as usize;

        let declaration = &map_file.vertex_declarations[decl_index];
        let vertex_buffer = &map_file.vertex_buffers[v_buff_index];
        let buffer_data = &vertex_buffer.buffer;

        // Calculate total stride of vertex declaration (bytes per vertex)
        let stride = declaration
            .elements
            .iter()
            .map(|e| e.format.get_size())
            .sum::<usize>();

        if stride == 0 {
            continue;
        }

        // Iterate through each vertex in the vertex buffer
        for vtx_chunk in buffer_data.chunks_exact(stride) {
            let mut offset = 0;
            // Iterate through each element in the vertex declaration (position, normal, etc.)
            for element in &declaration.elements {
                let size = element.format.get_size();
                let element_data = &vtx_chunk[offset..offset + size];

                match element.name {
                    ElementName::Position => {
                        if element_data.len() >= 12 {
                            let x = f32::from_le_bytes(element_data[0..4].try_into().unwrap());
                            let y = f32::from_le_bytes(element_data[4..8].try_into().unwrap());
                            let z = f32::from_le_bytes(element_data[8..12].try_into().unwrap());
                            all_positions.push([x, y, z]);
                        }
                    }
                    ElementName::Normal => {
                        if element_data.len() >= 12 {
                            let x = f32::from_le_bytes(element_data[0..4].try_into().unwrap());
                            let y = f32::from_le_bytes(element_data[4..8].try_into().unwrap());
                            let z = f32::from_le_bytes(element_data[8..12].try_into().unwrap());
                            all_normals.push([x, y, z]);
                        }
                    }
                    ElementName::Texcoord0 => {
                        if element_data.len() >= 8 {
                            let u = f32::from_le_bytes(element_data[0..4].try_into().unwrap());
                            let v = f32::from_le_bytes(element_data[4..8].try_into().unwrap());
                            all_uvs.push([u, v]);
                        }
                    }
                    _ => {}
                }
                offset += size;
            }
        }
    }

    (all_positions, all_normals, all_uvs)
}
