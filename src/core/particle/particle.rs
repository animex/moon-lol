use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use crate::core::{
    particle::{ATTRIBUTE_LIFETIME, ATTRIBUTE_WORLD_POSITION},
    ParticleMaterialUnlitDecal,
};

#[derive(Component)]
pub struct ParticleState {
    pub timer_life: Timer,
    pub is_local_orientation: bool,
    pub local_matrix: Mat4,
    pub source: Entity,
}

pub fn update_particle(
    mut commands: Commands,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut res_particle_material_unlit_decal: ResMut<Assets<ParticleMaterialUnlitDecal>>,
    mut query: Query<(Entity, &Transform, &mut ParticleState)>,
    q_particle_material_unlit_decal: Query<
        &MeshMaterial3d<ParticleMaterialUnlitDecal>,
        With<ParticleState>,
    >,
    q_global_transform: Query<&GlobalTransform>,
    q_mesh3d: Query<&Mesh3d>,
    time: Res<Time>,
) {
    for (entity, transform, mut particle_state) in query.iter_mut() {
        particle_state.timer_life.tick(time.delta());

        if particle_state.timer_life.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let parent = particle_state.source;

        let parent_global_transform = q_global_transform.get(parent).unwrap();

        let parent_matrix = if particle_state.is_local_orientation {
            parent_global_transform.compute_matrix()
        } else {
            Mat4::from_scale_rotation_translation(
                parent_global_transform.scale(),
                Quat::default(),
                parent_global_transform.translation(),
            )
        };

        let local_matrix = transform.compute_matrix();

        let local_to_world_matrix = parent_matrix * local_matrix;

        if let Ok(mat) = q_particle_material_unlit_decal.get(entity) {
            if let Some(mat) = res_particle_material_unlit_decal.get_mut(mat.0.id()) {
                mat.uniforms_vertex.decal_world_to_uv_matrix =
                    Mat4::from_translation(Vec3::splat(0.5))
                        * (parent_matrix * particle_state.local_matrix).inverse();
            }
        }

        let Ok(mesh3d) = q_mesh3d.get(entity) else {
            continue;
        };

        let mesh = res_mesh.get_mut(mesh3d).unwrap();

        let lifetime = particle_state.timer_life.elapsed_secs()
            / particle_state.timer_life.duration().as_secs_f32();

        let Some(lifetime_values) = mesh.attribute_mut(ATTRIBUTE_LIFETIME) else {
            continue;
        };

        match lifetime_values {
            VertexAttributeValues::Float32x2(items) => {
                for item in items {
                    item[0] = lifetime;
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
                local_to_world_matrix
                    .transform_point(vertext_position)
                    .to_array()
            })
            .collect::<Vec<_>>();

        mesh.insert_attribute(ATTRIBUTE_WORLD_POSITION, postion_values);
    }
}
