mod mesh;
mod quad;
mod quad_slice;

pub use mesh::*;
pub use quad::*;
pub use quad_slice::*;

use bevy::{
    prelude::*,
    render::mesh::{
        skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
        VertexAttributeValues,
    },
};

use crate::core::{
    particle::{ATTRIBUTE_LIFETIME, ATTRIBUTE_WORLD_POSITION},
    Lifetime, ParticleEmitterState, ParticleMaterialSkinnedMeshParticle,
    ParticleMaterialUnlitDecal,
};

#[derive(Component)]
pub struct ParticleState {
    pub birth_uv_offset: Vec2,
    pub birth_uv_scroll_rate: Vec2,
    pub birth_color: Vec4,
    pub scale: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

pub fn update_particle(
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut res_particle_material_unlit_decal: ResMut<Assets<ParticleMaterialUnlitDecal>>,
    mut res_particle_material_mesh: ResMut<Assets<ParticleMaterialMesh>>,
    q_particle_state: Query<(Entity, &Transform, &ChildOf, &Lifetime, &ParticleState)>,
    q_particle_material_unlit_decal: Query<
        &MeshMaterial3d<ParticleMaterialUnlitDecal>,
        With<ParticleState>,
    >,
    q_particle_material_mesh: Query<&MeshMaterial3d<ParticleMaterialMesh>, With<ParticleState>>,
    q_particle_emitter_state: Query<&ParticleEmitterState>,
    q_global_transform: Query<&GlobalTransform>,
    q_mesh3d: Query<&Mesh3d>,
) {
    for (particle_entity, transform, child_of, lifetime, particle) in q_particle_state.iter() {
        let parent = child_of.parent();

        let life = lifetime.progress();

        let emitter = q_particle_emitter_state.get(parent).unwrap();

        let color = particle.birth_color * emitter.color.sample_clamped(life);

        let world_matrix =
            q_global_transform.get(parent).unwrap().compute_matrix() * transform.compute_matrix();

        if let Ok(material) = q_particle_material_unlit_decal.get(particle_entity) {
            if let Some(material) = res_particle_material_unlit_decal.get_mut(material.0.id()) {
                material.uniforms_vertex.decal_world_to_uv_matrix =
                    Mat4::from_translation(Vec3::splat(0.5)) * world_matrix.inverse();
            }
        }

        if let Ok(material) = q_particle_material_mesh.get(particle_entity) {
            if let Some(material) = res_particle_material_mesh.get_mut(material.0.id()) {
                material.uniforms_vertex.m_world = world_matrix;

                let current_uv_offset: Vec2 = particle.birth_uv_offset
                    + particle.birth_uv_scroll_rate * lifetime.elapsed_secs();

                material.uniforms_vertex.v_particle_uvtransform = [
                    Vec3::X,
                    Vec3::Y,
                    vec3(current_uv_offset.x, current_uv_offset.y, 0.),
                    Vec3::ZERO,
                ];

                material.uniforms_pixel.color_lookup_uv = vec2(life, life);

                material.uniforms_vertex.k_color_factor = color;
            }
        }

        let Ok(mesh3d) = q_mesh3d.get(particle_entity) else {
            continue;
        };

        let Some(mesh) = res_mesh.get_mut(mesh3d) else {
            continue;
        };

        let Some(lifetime_values) = mesh.attribute_mut(ATTRIBUTE_LIFETIME) else {
            continue;
        };

        match lifetime_values {
            VertexAttributeValues::Float32x2(items) => {
                for item in items {
                    item[0] = life;
                }
            }
            _ => panic!(),
        }

        let VertexAttributeValues::Float32x3(postion_values) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!();
        };

        let postion_values = postion_values
            .iter_mut()
            .map(|v| {
                let vertext_position = Vec3::from_array(*v);
                world_matrix.transform_point(vertext_position).to_array()
            })
            .collect::<Vec<_>>();

        mesh.insert_attribute(ATTRIBUTE_WORLD_POSITION, postion_values);

        let VertexAttributeValues::Float32x4(values) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR).unwrap()
        else {
            panic!();
        };

        let values = values.iter().map(|_| color.to_array()).collect::<Vec<_>>();

        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, values);
    }
}

pub fn update_particle_transform(
    mut q_particle_state: Query<(&mut Transform, &ChildOf, &Lifetime, &mut ParticleState)>,
    q_particle_emitter_state: Query<&ParticleEmitterState>,
    res_time: Res<Time>,
) {
    let dt = res_time.delta_secs();

    for (mut transform, child_of, lifetime, mut particle) in q_particle_state.iter_mut() {
        particle.velocity = particle.velocity + particle.acceleration * dt;

        transform.translation += particle.velocity * dt;

        let parent = child_of.parent();

        let life = lifetime.progress();

        let emitter = q_particle_emitter_state.get(parent).unwrap();

        let scale0 = emitter.scale0.sample_clamped(life);

        transform.scale = scale0 * particle.scale;
    }
}

pub fn update_particle_skinned_mesh_particle(
    mut res_particle_material_skinned_mesh_particle: ResMut<
        Assets<ParticleMaterialSkinnedMeshParticle>,
    >,
    res_inverse_bindposes: Res<Assets<SkinnedMeshInverseBindposes>>,
    q_particle_state: Query<(
        Entity,
        &ChildOf,
        &Lifetime,
        &ParticleState,
        &MeshMaterial3d<ParticleMaterialSkinnedMeshParticle>,
    )>,
    q_particle_emitter_state: Query<&ParticleEmitterState>,
    q_global_transform: Query<&GlobalTransform>,
    q_skinned_mesh: Query<&SkinnedMesh>,
) {
    for (particle_entity, child_of, lifetime, particle, material) in q_particle_state.iter() {
        let parent = child_of.parent();

        let life = lifetime.progress();

        let emitter = q_particle_emitter_state.get(parent).unwrap();

        let color = particle.birth_color * emitter.color.sample_clamped(life);

        let material = res_particle_material_skinned_mesh_particle
            .get_mut(material.0.id())
            .unwrap();

        let skinned_mesh = q_skinned_mesh.get(particle_entity).unwrap();

        let inverse_bindposes = res_inverse_bindposes
            .get(skinned_mesh.inverse_bindposes.id())
            .unwrap();

        let mut bones = Vec::new();

        for (i, entity) in skinned_mesh.joints.iter().enumerate() {
            let g = q_global_transform.get(*entity).unwrap();
            bones.push(g.compute_matrix() * inverse_bindposes[i]);
        }

        let current_uv_offset: Vec2 =
            particle.birth_uv_offset + particle.birth_uv_scroll_rate * lifetime.elapsed_secs();

        material.uniforms_vertex.v_particle_uvtransform = [
            Vec3::X,
            Vec3::Y,
            vec3(current_uv_offset.y, current_uv_offset.x, 0.),
            Vec3::ZERO,
        ];

        material.uniforms_pixel.color_lookup_uv = vec2(life, life);

        material.uniforms_vertex.k_color_factor = color;

        material.uniforms_vertex.bones = mat4_vec_to_mat4_array_homogeneous(bones);

        continue;
    }
}

pub fn mat4_vec_to_mat4_array_homogeneous(mats: Vec<Mat4>) -> [[Vec3; 4]; 68] {
    const IDENTITY_VEC3_ARRAY: [Vec3; 4] = [Vec3::X, Vec3::Y, Vec3::Z, Vec3::ZERO];

    let mut bone_array: [[Vec3; 4]; 68] = [IDENTITY_VEC3_ARRAY; 68];

    for (i, mat) in mats.into_iter().enumerate().take(68) {
        let cols = mat.to_cols_array();

        bone_array[i] = [
            Vec3::new(cols[0], cols[1], cols[2]),
            Vec3::new(cols[4], cols[5], cols[6]),
            Vec3::new(cols[8], cols[9], cols[10]),
            Vec3::new(cols[12], cols[13], cols[14]),
        ];
    }

    bone_array
}
