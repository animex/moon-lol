use bevy::{prelude::*, render::render_resource::ShaderType};

#[derive(Clone, ShaderType, Debug)]
pub struct UniformsVertexUnlitDecal {
    pub fog_of_war_params: Vec4,
    pub fog_of_war_always_below_y: Vec4,
    pub fow_height_fade: Vec4,
    pub decal_world_matrix: Mat4,
    pub decal_world_to_uv_matrix: Mat4,
    pub decal_projection_y_range: Vec4,
}

impl Default for UniformsVertexUnlitDecal {
    fn default() -> Self {
        Self {
            fog_of_war_params: Vec4::ZERO,
            fog_of_war_always_below_y: Vec4::ZERO,
            fow_height_fade: Vec4::ZERO,
            decal_world_matrix: Mat4::IDENTITY,
            decal_world_to_uv_matrix: Mat4::IDENTITY,
            decal_projection_y_range: Vec4::splat(100.0),
        }
    }
}
