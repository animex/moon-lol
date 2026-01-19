use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermediateMesh {
    /// Mesh name or identifier
    pub name: String,

    /// Vertex count, used to validate other data lengths
    pub vertex_count: u32,

    /// Vertex position data
    pub positions: Vec<[f32; 3]>,

    /// Normal data (optional)
    pub has_normals: u8,

    pub normals: Option<Vec<[f32; 3]>>,

    /// UV coordinate data (optional)
    pub has_uvs: u8,

    pub uvs: Option<Vec<[f32; 2]>>,

    /// Vertex color data (optional)
    pub has_colors: u8,

    pub colors: Option<Vec<[f32; 4]>>,

    /// Tangent data (optional)
    pub has_tangents: u8,

    pub tangents: Option<Vec<[f32; 4]>>,

    /// Joint index data (optional, for skeletal animation)
    pub has_joint_indices: u8,

    pub joint_indices: Option<Vec<[u16; 4]>>,

    /// Joint weight data (optional, for skeletal animation)
    pub has_joint_weights: u8,

    pub joint_weights: Option<Vec<[f32; 4]>>,

    /// Index count
    pub index_count: u32,
    /// Index data
    pub indices: Vec<u16>,

    /// Material info (optional)
    pub has_material_info: u8,

    pub material_info: Option<String>,
}

impl IntermediateMesh {
    /// Create a new empty mesh
    pub fn new(name: String) -> Self {
        Self {
            name: name.into(),
            vertex_count: 0,
            positions: Vec::new(),
            has_normals: 0,
            normals: None,
            has_uvs: 0,
            uvs: None,
            has_colors: 0,
            colors: None,
            has_tangents: 0,
            tangents: None,
            has_joint_indices: 0,
            joint_indices: None,
            has_joint_weights: 0,
            joint_weights: None,
            index_count: 0,
            indices: Vec::new(),
            has_material_info: 0,
            material_info: None,
        }
    }

    /// Check if mesh contains skeletal animation data
    pub fn is_skinned(&self) -> bool {
        self.joint_indices.is_some() && self.joint_weights.is_some()
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertex_count as usize
    }

    /// Get triangle count
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Set vertex positions (automatically updates vertex_count)
    pub fn set_positions(&mut self, positions: Vec<[f32; 3]>) {
        self.vertex_count = positions.len() as u32;
        self.positions = positions;
    }

    /// Set index data (automatically updates index_count)
    pub fn set_indices(&mut self, indices: Vec<u16>) {
        self.index_count = indices.len() as u32;
        self.indices = indices;
    }

    /// Set normal data
    pub fn set_normals(&mut self, normals: Option<Vec<[f32; 3]>>) {
        self.has_normals = if normals.is_some() { 1 } else { 0 };
        self.normals = normals;
    }

    /// Set UV data
    pub fn set_uvs(&mut self, uvs: Option<Vec<[f32; 2]>>) {
        self.has_uvs = if uvs.is_some() { 1 } else { 0 };
        self.uvs = uvs;
    }

    /// Set color data
    pub fn set_colors(&mut self, colors: Option<Vec<[f32; 4]>>) {
        self.has_colors = if colors.is_some() { 1 } else { 0 };
        self.colors = colors;
    }

    /// Set tangent data
    pub fn set_tangents(&mut self, tangents: Option<Vec<[f32; 4]>>) {
        self.has_tangents = if tangents.is_some() { 1 } else { 0 };
        self.tangents = tangents;
    }

    /// Set joint index data
    pub fn set_joint_indices(&mut self, joint_indices: Option<Vec<[u16; 4]>>) {
        self.has_joint_indices = if joint_indices.is_some() { 1 } else { 0 };
        self.joint_indices = joint_indices;
    }

    /// Set joint weight data
    pub fn set_joint_weights(&mut self, joint_weights: Option<Vec<[f32; 4]>>) {
        self.has_joint_weights = if joint_weights.is_some() { 1 } else { 0 };
        self.joint_weights = joint_weights;
    }

    /// Set material info
    pub fn set_material_info(&mut self, material_info: Option<String>) {
        self.has_material_info = if material_info.is_some() { 1 } else { 0 };
        self.material_info = material_info.map(|s| s.into());
    }

    /// Validate mesh data integrity
    pub fn validate(&self) -> Result<(), String> {
        if self.vertex_count == 0 {
            return Err("Mesh must have at least one vertex".to_string());
        }

        let vertex_count = self.vertex_count as usize;

        // Check positions length
        if self.positions.len() != vertex_count {
            return Err("Positions count doesn't match vertex_count".to_string());
        }

        // Check if all vertex attribute lengths are consistent
        if let Some(ref normals) = self.normals {
            if normals.len() != vertex_count {
                return Err("Normals count doesn't match vertex count".to_string());
            }
        }

        if let Some(ref uvs) = self.uvs {
            if uvs.len() != vertex_count {
                return Err("UVs count doesn't match vertex count".to_string());
            }
        }

        if let Some(ref colors) = self.colors {
            if colors.len() != vertex_count {
                return Err("Colors count doesn't match vertex count".to_string());
            }
        }

        if let Some(ref tangents) = self.tangents {
            if tangents.len() != vertex_count {
                return Err("Tangents count doesn't match vertex count".to_string());
            }
        }

        if let Some(ref joint_indices) = self.joint_indices {
            if joint_indices.len() != vertex_count {
                return Err("Joint indices count doesn't match vertex count".to_string());
            }
        }

        if let Some(ref joint_weights) = self.joint_weights {
            if joint_weights.len() != vertex_count {
                return Err("Joint weights count doesn't match vertex count".to_string());
            }
        }

        // Check index length
        if self.indices.len() != self.index_count as usize {
            return Err("Indices length doesn't match index_count".to_string());
        }

        // Check if indices are valid
        for &index in &self.indices {
            if index as usize >= vertex_count {
                return Err(format!(
                    "Index {} is out of bounds for {} vertices",
                    index, vertex_count
                ));
            }
        }

        // Check if index count is a multiple of 3
        if self.indices.len() % 3 != 0 {
            return Err("Index count must be a multiple of 3 for triangle lists".to_string());
        }

        Ok(())
    }
}

impl From<IntermediateMesh> for Mesh {
    fn from(mesh: IntermediateMesh) -> Self {
        let mut bevy_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );

        // Insert required position attribute
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions.clone());

        // Insert optional attributes
        if let Some(ref normals) = mesh.normals {
            bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone());
        }

        if let Some(ref uvs) = mesh.uvs {
            bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());
        }

        if let Some(ref colors) = mesh.colors {
            bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors.clone());
        }

        if let Some(ref tangents) = mesh.tangents {
            bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangents.clone());
        }

        // Insert skeletal animation attributes
        if let Some(ref joint_indices) = mesh.joint_indices {
            bevy_mesh.insert_attribute(
                Mesh::ATTRIBUTE_JOINT_INDEX,
                VertexAttributeValues::Uint16x4(joint_indices.clone()),
            );
        }

        if let Some(ref joint_weights) = mesh.joint_weights {
            bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, joint_weights.clone());
        }

        let indices = mesh.indices.clone();

        // Insert indices
        bevy_mesh.insert_indices(Indices::U16(indices));

        bevy_mesh
    }
}
