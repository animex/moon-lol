use league_file::{LeagueSkinnedMesh, SkinnedMeshVertex};
use lol_config::IntermediateMesh;

pub fn skinned_mesh_to_intermediate(
    skinned_mesh: &LeagueSkinnedMesh,
    submesh_index: usize,
) -> IntermediateMesh {
    let range = skinned_mesh.ranges.get(submesh_index).unwrap();

    let vertex_size = skinned_mesh.vertex_declaration.get_vertex_size() as usize;

    // Calculate vertex data range
    let vertex_start_byte = range.start_vertex as usize * vertex_size;
    let vertex_end_byte = vertex_start_byte + (range.vertex_count as usize * vertex_size);
    let vertex_data_slice = skinned_mesh
        .vertex_buffer
        .get(vertex_start_byte..vertex_end_byte)
        .unwrap();

    let capacity = range.vertex_count as usize;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(capacity);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(capacity);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(capacity);
    let mut joint_indices: Vec<[u16; 4]> = Vec::with_capacity(capacity);
    let mut joint_weights: Vec<[f32; 4]> = Vec::with_capacity(capacity);

    let mut colors: Option<Vec<[f32; 4]>> = None;
    let mut tangents: Option<Vec<[f32; 4]>> = None;

    // Prepare color and tangent data based on vertex declaration type
    if skinned_mesh.vertex_declaration != SkinnedMeshVertex::Basic {
        colors = Some(Vec::with_capacity(capacity));
        if skinned_mesh.vertex_declaration == SkinnedMeshVertex::Tangent {
            tangents = Some(Vec::with_capacity(capacity));
        }
    }

    // Parse vertex data
    for v_chunk in vertex_data_slice.chunks_exact(vertex_size) {
        let mut offset = 0;

        // Read position
        let x_pos = f32::from_le_bytes(v_chunk[offset..offset + 4].try_into().unwrap());
        let y_pos = f32::from_le_bytes(v_chunk[offset + 4..offset + 8].try_into().unwrap());
        let z_pos = f32::from_le_bytes(v_chunk[offset + 8..offset + 12].try_into().unwrap());
        positions.push([x_pos, y_pos, z_pos]);
        offset += 12;

        // Read joint indices
        let j_indices_u8: [u8; 4] = v_chunk[offset..offset + 4].try_into().unwrap();
        joint_indices.push([
            j_indices_u8[0] as u16,
            j_indices_u8[1] as u16,
            j_indices_u8[2] as u16,
            j_indices_u8[3] as u16,
        ]);
        offset += 4;

        // Read joint weights
        let weights: [f32; 4] = [
            f32::from_le_bytes(v_chunk[offset..offset + 4].try_into().unwrap()),
            f32::from_le_bytes(v_chunk[offset + 4..offset + 8].try_into().unwrap()),
            f32::from_le_bytes(v_chunk[offset + 8..offset + 12].try_into().unwrap()),
            f32::from_le_bytes(v_chunk[offset + 12..offset + 16].try_into().unwrap()),
        ];
        joint_weights.push(weights);
        offset += 16;

        // Read normals
        let x_norm = f32::from_le_bytes(v_chunk[offset..offset + 4].try_into().unwrap());
        let y_norm = f32::from_le_bytes(v_chunk[offset + 4..offset + 8].try_into().unwrap());
        let z_norm = f32::from_le_bytes(v_chunk[offset + 8..offset + 12].try_into().unwrap());
        normals.push([x_norm, y_norm, z_norm]);
        offset += 12;

        // Read UV
        let u = f32::from_le_bytes(v_chunk[offset..offset + 4].try_into().unwrap());
        let v = f32::from_le_bytes(v_chunk[offset + 4..offset + 8].try_into().unwrap());
        uvs.push([u, v]);
        offset += 8;

        // Read color (if exists)
        if let Some(colors_vec) = colors.as_mut() {
            let color_u8: [u8; 4] = v_chunk[offset..offset + 4].try_into().unwrap();
            colors_vec.push([
                color_u8[2] as f32 / 255.0, // R
                color_u8[1] as f32 / 255.0, // G
                color_u8[0] as f32 / 255.0, // B
                color_u8[3] as f32 / 255.0, // A
            ]);
            offset += 4;
        }

        // Read tangent (if exists)
        if let Some(tangents_vec) = tangents.as_mut() {
            let tan_x = f32::from_le_bytes(v_chunk[offset..offset + 4].try_into().unwrap());
            let tan_y = f32::from_le_bytes(v_chunk[offset + 4..offset + 8].try_into().unwrap());
            let tan_z = f32::from_le_bytes(v_chunk[offset + 8..offset + 12].try_into().unwrap());
            let tan_w = f32::from_le_bytes(v_chunk[offset + 12..offset + 16].try_into().unwrap());

            tangents_vec.push([tan_x, tan_y, tan_z, tan_w]);
        }
    }

    // Process index data
    let index_start_byte = range.start_index as usize * 2;
    let index_end_byte = index_start_byte + (range.index_count as usize * 2);
    let index_data_slice = skinned_mesh
        .index_buffer
        .get(index_start_byte..index_end_byte)
        .unwrap();

    let local_indices: Vec<u16> = index_data_slice
        .chunks_exact(2)
        .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap()))
        .map(|global_index| global_index - range.start_vertex as u16)
        .collect();

    // Create intermediate mesh structure
    let mut intermediate_mesh = IntermediateMesh::new(range.name.clone());

    intermediate_mesh.set_positions(positions);
    intermediate_mesh.set_normals(Some(normals));
    intermediate_mesh.set_uvs(Some(uvs));
    intermediate_mesh.set_joint_indices(Some(joint_indices));
    intermediate_mesh.set_joint_weights(Some(joint_weights));
    intermediate_mesh.set_indices(local_indices);

    // Set optional attributes
    if let Some(colors_data) = colors {
        intermediate_mesh.set_colors(Some(colors_data));
    }

    if let Some(tangents_data) = tangents {
        intermediate_mesh.set_tangents(Some(tangents_data));
    }

    // Set material info (if name exists)
    if !range.name.is_empty() {
        intermediate_mesh.set_material_info(Some(range.name.clone()));
    }

    intermediate_mesh
}
