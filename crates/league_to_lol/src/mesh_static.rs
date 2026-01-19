use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;
use league_file::LeagueMeshStatic;

pub fn mesh_static_to_bevy_mesh(mesh: LeagueMeshStatic) -> Mesh {
    let num_vertices = mesh.faces.len() * 3;

    // 1. Prepare expanded vertex attribute Vecs
    let mut bevy_positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut bevy_uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);

    // Only create Vec when source mesh contains vertex colors
    let mut bevy_colors: Option<Vec<[f32; 4]>> = if mesh.has_vertex_colors {
        Some(Vec::with_capacity(num_vertices))
    } else {
        None
    };

    // 2. Iterate through all faces, expand vertex data
    for face in &mesh.faces {
        for i in 0..3 {
            // Get the index of the i-th vertex in the face, pointing to global "vertices" list
            let global_pos_index = face.indices[i] as usize;

            // a. Get position data
            // Look up position from global "vertices" list
            let pos = mesh.vertices[global_pos_index];
            bevy_positions.push(pos);

            // c. Get UV data
            // UV data is also stored directly in face
            let uv = face.uvs[i];
            bevy_uvs.push(uv);

            // d. Get vertex color data (if exists)
            if let Some(colors_vec) = bevy_colors.as_mut() {
                // Vertex colors, like positions, are looked up from global "vertex_colors" list
                let bgra_u8 = mesh.vertex_colors.as_ref().unwrap()[global_pos_index];

                // Convert: [u8; 4] (BGRA) -> [f32; 4] (RGBA, normalized)
                // Reference skin_mesh.rs for conversion
                colors_vec.push([
                    bgra_u8[2] as f32 / 255.0, // R
                    bgra_u8[1] as f32 / 255.0, // G
                    bgra_u8[0] as f32 / 255.0, // B
                    bgra_u8[3] as f32 / 255.0, // A
                ]);
            }
        }
    }

    // 3. Create indices
    // Since we expanded all vertices, indices are now just a simple 0, 1, 2, 3, 4, 5, ... sequence
    let indices: Vec<u16> = (0..num_vertices as u16).collect();

    // 4. Create Bevy Mesh and insert all attributes
    let mut bevy_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, bevy_positions);
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, bevy_uvs);

    if let Some(colors_data) = bevy_colors {
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors_data);
    }

    bevy_mesh.insert_indices(Indices::U16(indices));

    bevy_mesh.compute_normals();

    bevy_mesh
}
