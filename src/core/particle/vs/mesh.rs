use bevy::{prelude::*, render::render_resource::ShaderType};

#[derive(Clone, ShaderType, Debug)]
pub struct UniformsVertexMesh {
    pub fog_of_war_params: Vec4,
    pub fog_of_war_always_below_y: Vec4,
    pub fow_height_fade: Vec4,
    pub m_world: Mat4,
    pub particle_depth_push_pull: f32,
    pub v_fresnel: Vec4,
    pub v_particle_uvtransform: [Vec3; 4],
    pub v_particle_uvtransform_mult: [Vec3; 4],
    pub k_color_factor: Vec4,
}

impl Default for UniformsVertexMesh {
    fn default() -> Self {
        Self {
            fog_of_war_params: Vec4::ZERO,
            fog_of_war_always_below_y: Vec4::ZERO,
            fow_height_fade: Vec4::ZERO,
            m_world: Default::default(),
            particle_depth_push_pull: Default::default(),
            v_fresnel: Vec4::W,
            v_particle_uvtransform: [Vec3::X, Vec3::Y, Vec3::ZERO, Vec3::ZERO],
            v_particle_uvtransform_mult: Default::default(),
            k_color_factor: Vec4::ONE,
        }
    }
}
