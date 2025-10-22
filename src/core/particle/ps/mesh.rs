use std::fmt::Debug;

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState,
            RenderPipelineDescriptor, ShaderRef, ShaderType, SpecializedMeshPipelineError,
        },
    },
};

use crate::core::UniformsVertexMesh;

#[derive(Clone, ShaderType, Debug)]
pub struct UniformsPixelMesh {
    pub fow_edge_control: Vec4,
    pub color_lookup_uv: Vec2,
}

impl Default for UniformsPixelMesh {
    fn default() -> Self {
        Self {
            fow_edge_control: Vec4::ONE,
            color_lookup_uv: Vec2::ONE,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
#[bind_group_data(ParticleMaterialKeyMesh)]
pub struct ParticleMaterialMesh {
    #[uniform(0)]
    pub uniforms_vertex: UniformsVertexMesh,
    #[uniform(1)]
    pub uniforms_pixel: UniformsPixelMesh,
    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    pub particle_color_texture: Option<Handle<Image>>,
    #[texture(6)]
    #[sampler(7)]
    pub cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Option<Handle<Image>>,
    #[texture(8)]
    #[sampler(9)]
    pub cmb_tex_fow_map_smp_clamp_no_mip: Option<Handle<Image>>,
    pub blend_mode: u8,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ParticleMaterialKeyMesh {
    blend_mode: u8,
}

// 2. 为 Key 实现 From Trait
impl From<&ParticleMaterialMesh> for ParticleMaterialKeyMesh {
    fn from(material: &ParticleMaterialMesh) -> Self {
        Self {
            blend_mode: material.blend_mode,
        }
    }
}

impl Material for ParticleMaterialMesh {
    fn fragment_shader() -> ShaderRef {
        "shaders/mesh.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/mesh.vert".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        match self.blend_mode {
            1 => AlphaMode::Blend,
            4 => AlphaMode::Blend,
            _ => AlphaMode::Opaque,
        }
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();

        let fragment = descriptor.fragment.as_mut().unwrap();
        let target = fragment.targets.get_mut(0).unwrap().as_mut().unwrap();
        if key.bind_group_data.blend_mode == 4 {
            target.blend = Some(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent::OVER,
            });
        }

        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(8),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        descriptor.primitive.cull_mode = None;

        Ok(())
    }
}
