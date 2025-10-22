use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use crate::core::{
    particle::{ATTRIBUTE_LIFETIME, ATTRIBUTE_WORLD_POSITION},
    Lifetime, ParticleEmitterState, ParticleMaterialMesh, ParticleMaterialSkinnedMeshParticle,
    ParticleMaterialUnlitDecal,
};

#[derive(Component)]
pub struct ParticleState {
    pub birth_uv_offset: Vec2,
    pub birth_uv_scroll_rate: Vec2,
    pub birth_color: Vec4,
    pub birth_scale0: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

pub fn update_particle(
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut res_particle_material_unlit_decal: ResMut<Assets<ParticleMaterialUnlitDecal>>,
    mut res_particle_material_mesh: ResMut<Assets<ParticleMaterialMesh>>,
    mut res_particle_material_skinned_mesh_particle: ResMut<
        Assets<ParticleMaterialSkinnedMeshParticle>,
    >,
    mut q_particle_state: Query<(
        Entity,
        &mut Transform,
        &ChildOf,
        &Lifetime,
        &mut ParticleState,
    )>,
    q_particle_material_unlit_decal: Query<
        &MeshMaterial3d<ParticleMaterialUnlitDecal>,
        With<ParticleState>,
    >,
    q_particle_material_mesh: Query<&MeshMaterial3d<ParticleMaterialMesh>, With<ParticleState>>,
    q_particle_material_skinned_mesh_particle: Query<
        &MeshMaterial3d<ParticleMaterialSkinnedMeshParticle>,
        With<ParticleState>,
    >,
    q_particle_emitter_state: Query<&ParticleEmitterState>,
    q_global_transform: Query<&GlobalTransform>,
    q_mesh3d: Query<&Mesh3d>,
    res_time: Res<Time>,
) {
    let dt = res_time.delta_secs();

    for (particle_entity, mut transform, child_of, lifetime, mut particle) in
        q_particle_state.iter_mut()
    {
        particle.velocity = particle.velocity + particle.acceleration * dt;

        transform.translation += particle.velocity * dt;

        let parent = child_of.parent();

        let life = lifetime.progress();

        let emitter = q_particle_emitter_state.get(parent).unwrap();

        let color = emitter.color.sample_clamped(life);

        let scale0 = emitter.scale0.sample_clamped(life);

        transform.scale = scale0 * particle.birth_scale0;

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

                material.uniforms_vertex.k_color_factor = particle.birth_color * color;
            }
        }

        if let Ok(material) = q_particle_material_skinned_mesh_particle.get(particle_entity) {
            if let Some(material) =
                res_particle_material_skinned_mesh_particle.get_mut(material.0.id())
            {
                let current_uv_offset: Vec2 = particle.birth_uv_offset
                    + particle.birth_uv_scroll_rate * lifetime.elapsed_secs();

                material.uniforms_vertex.v_particle_uvtransform = [
                    Vec3::X,
                    Vec3::Y,
                    vec3(current_uv_offset.y, current_uv_offset.x, 0.),
                    Vec3::ZERO,
                ];

                material.uniforms_pixel.color_lookup_uv = vec2(life, life);

                material.uniforms_vertex.k_color_factor = particle.birth_color * color;
                println!("{:?} {:?}", particle.birth_color.w, color.w);

                continue;
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
    }
}
