use bevy::{
    animation::AnimationTarget,
    math::bounding::{Aabb3d, IntersectsVolume},
    platform::collections::HashSet,
    prelude::*,
    render::mesh::skinning::SkinnedMesh,
};

use league_core::{
    Unk0xee39916f, VfxEmitterDefinitionData, VfxEmitterDefinitionDataPrimitive,
    VfxEmitterDefinitionDataSpawnShape, VfxPrimitiveAttachedMesh, VfxPrimitiveMesh,
    VfxPrimitivePlanarProjection, VfxShapeLegacy,
};
use league_utils::{neg_rotation_z, neg_vec_z};

use crate::core::{
    particle::{
        create_black_pixel_texture, ParticleMaterialQuad, ParticleMaterialQuadSlice,
        ParticleMeshQuad, ParticleState, UniformsPixelQuadSlice, UniformsVertexQuad,
    },
    spawn_shadow_skin_entity, Lifetime, MapGeometry, ParticleId, ParticleMaterialMesh,
    ParticleMaterialSkinnedMeshParticle, ParticleMaterialUnlitDecal, StochasticSampler,
    UniformsPixelMesh, UniformsPixelSkinnedMeshParticle, UniformsPixelUnlitDecal,
    UniformsVertexMesh, UniformsVertexSkinnedMeshParticle, UniformsVertexUnlitDecal,
};

#[derive(Component)]
pub struct ParticleEmitterState {
    pub birth_acceleration: StochasticSampler<Vec3>,
    pub birth_color: StochasticSampler<Vec4>,
    pub birth_rotation0: StochasticSampler<Vec3>,
    pub birth_scale0: StochasticSampler<Vec3>,
    pub birth_uv_offset: StochasticSampler<Vec2>,
    pub birth_uv_scroll_rate: StochasticSampler<Vec2>,
    pub birth_velocity: StochasticSampler<Vec3>,
    pub color: StochasticSampler<Vec4>,
    pub scale0: StochasticSampler<Vec3>,
    pub emission_debt: f32,
    pub particle_lifetime: StochasticSampler<f32>,
    pub rate: StochasticSampler<f32>,
    pub emitter_position: StochasticSampler<Vec3>,
    pub world_matrix: Mat4,
}

pub fn update_emitter_position(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &ChildOf,
        &Lifetime,
        &ParticleEmitterState,
        &VfxEmitterDefinitionData,
    )>,
    q_global_transform: Query<&GlobalTransform>,
) {
    for (emitter_entity, mut transform, child_of, lifetime, emitter, vfx_emitter_definition_data) in
        query.iter_mut()
    {
        let parent = child_of.parent();

        let progress = lifetime.progress();

        let emitter_position = emitter.emitter_position.sample_clamped(progress);

        let parent_global_transform = q_global_transform.get(parent).unwrap();

        let parent_world = parent_global_transform.compute_matrix();

        let translation = neg_vec_z(&emitter_position);

        let is_local_orientation = vfx_emitter_definition_data
            .is_local_orientation
            .unwrap_or(true);

        if !is_local_orientation {
            let parent_new_world = Mat4::from_scale_rotation_translation(
                parent_global_transform.scale(),
                Quat::default(),
                parent_global_transform.translation(),
            );

            let local =
                Mat4::from_scale_rotation_translation(Vec3::ONE, Quat::default(), translation);

            let new_world = parent_new_world * local;

            let new_local = parent_world.inverse() * new_world;

            *transform = Transform::from_matrix(new_local);

            commands
                .entity(emitter_entity)
                .insert(GlobalTransform::from(new_world));
        } else {
            transform.translation = translation;

            commands
                .entity(emitter_entity)
                .insert(GlobalTransform::from(
                    parent_world * transform.compute_matrix(),
                ));
        }
    }
}

pub fn update_emitter(
    mut commands: Commands,
    mut res_mesh: ResMut<Assets<Mesh>>,
    res_asset_server: Res<AssetServer>,
    mut res_image: ResMut<Assets<Image>>,
    mut res_quad_material: ResMut<Assets<ParticleMaterialQuad>>,
    mut res_quad_slice_material: ResMut<Assets<ParticleMaterialQuadSlice>>,
    mut res_unlit_decal_material: ResMut<Assets<ParticleMaterialUnlitDecal>>,
    mut res_particle_material_mesh: ResMut<Assets<ParticleMaterialMesh>>,
    mut res_particle_material_skinned_mesh_particle: ResMut<
        Assets<ParticleMaterialSkinnedMeshParticle>,
    >,
    mut query: Query<(
        Entity,
        &ChildOf,
        &Lifetime,
        &mut ParticleEmitterState,
        &VfxEmitterDefinitionData,
        &ParticleId,
    )>,
    q_mesh3d: Query<&Mesh3d>,
    q_skinned_mesh: Query<&SkinnedMesh>,
    q_children: Query<&Children>,
    q_animation_target: Query<(Entity, &Transform, &AnimationTarget)>,
    time: Res<Time>,
) {
    for (
        emitter_entity,
        child_of,
        lifetime,
        mut emitter,
        vfx_emitter_definition_data,
        particle_id,
    ) in query.iter_mut()
    {
        let progress = lifetime.progress();

        let rate = emitter.rate.sample_clamped(progress);
        let particle_lifetime = emitter.particle_lifetime.sample_clamped(progress);

        let birth_color = emitter.birth_color.sample_clamped(progress);
        let birth_velocity = emitter.birth_velocity.sample_clamped(progress);
        let birth_acceleration = emitter.birth_acceleration.sample_clamped(progress);
        let birth_rotation0 = emitter.birth_rotation0.sample_clamped(progress);
        let birth_scale0 = emitter.birth_scale0.sample_clamped(progress);
        let birth_uv_offset = emitter.birth_uv_offset.sample_clamped(progress);
        let birth_uv_scroll_rate = emitter.birth_uv_scroll_rate.sample_clamped(progress);

        let parent = child_of.parent();

        if lifetime.is_dead() {
            continue;
        }

        let particles_to_spawn_f32 = rate * time.delta_secs() + emitter.emission_debt;

        let particles_to_spawn = particles_to_spawn_f32.floor() as u32;

        emitter.emission_debt = particles_to_spawn_f32.fract();

        let is_uniform_scale = vfx_emitter_definition_data
            .is_uniform_scale
            .unwrap_or(false);

        let scale = if is_uniform_scale {
            Vec3::splat(birth_scale0.x)
        } else {
            birth_scale0
        };

        let primitive = vfx_emitter_definition_data
            .primitive
            .clone()
            .unwrap_or(VfxEmitterDefinitionDataPrimitive::VfxPrimitiveArbitraryQuad);

        let texture = vfx_emitter_definition_data
            .texture
            .as_ref()
            .map(|v| res_asset_server.load(v));

        let particle_color_texture = vfx_emitter_definition_data
            .particle_color_texture
            .as_ref()
            .map(|v| res_asset_server.load(v));

        let texture_mult = vfx_emitter_definition_data
            .texture_mult
            .as_ref()
            .and_then(|v| v.texture_mult.as_ref())
            .map(|v| res_asset_server.load(v));

        let blend_mode = vfx_emitter_definition_data.blend_mode.unwrap_or(1);

        for _ in 0..particles_to_spawn {
            let translation = neg_vec_z(
                &vfx_emitter_definition_data
                    .spawn_shape
                    .clone()
                    .and_then(|v| match v {
                        VfxEmitterDefinitionDataSpawnShape::Unk0xee39916f(Unk0xee39916f {
                            emit_offset,
                        }) => emit_offset,
                        VfxEmitterDefinitionDataSpawnShape::VfxShapeLegacy(VfxShapeLegacy {
                            emit_offset,
                            ..
                        }) => emit_offset.and_then(|v| {
                            Some(StochasticSampler::<Vec3>::from(v).sample_clamped(progress))
                        }),
                        _ => todo!(),
                    })
                    .unwrap_or(Vec3::ZERO),
            );

            let rotation_quat = Quat::from_euler(
                EulerRot::XYZEx,
                birth_rotation0.x.to_radians(),
                (birth_rotation0.y - birth_rotation0.z).to_radians(),
                0.,
            );

            let rotation_quat = neg_rotation_z(&rotation_quat);

            let mut transform = Transform::from_rotation(rotation_quat)
                .with_translation(translation)
                .with_scale(birth_scale0);

            if let VfxEmitterDefinitionDataPrimitive::VfxPrimitivePlanarProjection(
                VfxPrimitivePlanarProjection { ref m_projection },
            ) = primitive
            {
                transform.scale.x = transform.scale.x * 1.5;
                transform.scale.y = m_projection.clone().unwrap().m_y_range.unwrap();
                transform.scale.z = -transform.scale.z * 1.5;
            }

            let particle_entity = commands
                .spawn((
                    particle_id.clone(),
                    ParticleState {
                        birth_uv_offset,
                        birth_uv_scroll_rate,
                        birth_color,
                        scale,
                        velocity: neg_vec_z(&birth_velocity),
                        acceleration: birth_acceleration,
                    },
                    Lifetime::new_timer(particle_lifetime),
                ))
                .id();

            match primitive {
                VfxEmitterDefinitionDataPrimitive::VfxPrimitiveArbitraryQuad => {
                    let mesh = res_mesh.add(ParticleMeshQuad::default());

                    commands.entity(particle_entity).insert(Mesh3d(mesh));

                    let black_pixel_texture = res_image.add(create_black_pixel_texture());

                    if let Some(range) = vfx_emitter_definition_data.slice_technique_range {
                        commands.entity(particle_entity).insert(MeshMaterial3d(
                            res_quad_slice_material.add(ParticleMaterialQuadSlice {
                                uniforms_vertex: UniformsVertexQuad::default(),
                                uniforms_pixel: UniformsPixelQuadSlice {
                                    slice_range: vec2(range, 1.0 / (range * range)),
                                    ..default()
                                },
                                particle_color_texture: particle_color_texture.clone(),
                                texture: texture.clone(),
                                cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Some(
                                    black_pixel_texture,
                                ),
                                sampler_fow: None,
                                blend_mode,
                            }),
                        ));
                    } else {
                        commands.entity(particle_entity).insert(MeshMaterial3d(
                            res_quad_material.add(ParticleMaterialQuad {
                                uniforms_vertex: UniformsVertexQuad::default(),
                                particle_color_texture: particle_color_texture.clone(),
                                texture: texture.clone(),
                                cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Some(
                                    black_pixel_texture,
                                ),
                                sampler_fow: None,
                                texturemult: texture_mult.clone(),
                                blend_mode,
                            }),
                        ));
                    };
                }
                VfxEmitterDefinitionDataPrimitive::VfxPrimitivePlanarProjection(..) => {
                    let material_handle =
                        res_unlit_decal_material.add(ParticleMaterialUnlitDecal {
                            uniforms_vertex: UniformsVertexUnlitDecal {
                                decal_projection_y_range: Vec4::splat(transform.scale.y),
                                ..default()
                            },
                            uniforms_pixel: UniformsPixelUnlitDecal::default(),
                            diffuse_map: texture.clone(),
                            particle_color_texture: particle_color_texture.clone(),
                            cmb_tex_fow_map_smp_clamp_no_mip: None,
                            blend_mode,
                        });

                    commands
                        .entity(particle_entity)
                        .insert((ParticleDecal::default(), MeshMaterial3d(material_handle)));
                }
                VfxEmitterDefinitionDataPrimitive::VfxPrimitiveMesh(VfxPrimitiveMesh {
                    ref m_mesh,
                    ..
                }) => {
                    let Some(m_mesh) = m_mesh else {
                        println!("VfxPrimitiveMesh: m_mesh is None");
                        continue;
                    };
                    let Some(mesh_name) = &m_mesh.m_simple_mesh_name else {
                        println!("VfxPrimitiveMesh: m_simple_mesh_name is None");
                        continue;
                    };

                    let mesh = res_asset_server.load(mesh_name);
                    let black_pixel_texture = res_image.add(create_black_pixel_texture());

                    commands.entity(particle_entity).insert((
                        Mesh3d(mesh),
                        MeshMaterial3d(res_particle_material_mesh.add(ParticleMaterialMesh {
                            uniforms_vertex: UniformsVertexMesh::default(),
                            uniforms_pixel: UniformsPixelMesh::default(),
                            texture: texture.clone(),
                            particle_color_texture: particle_color_texture.clone(),
                            cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Some(
                                black_pixel_texture,
                            ),
                            cmb_tex_fow_map_smp_clamp_no_mip: None,
                            blend_mode,
                        })),
                    ));
                }
                VfxEmitterDefinitionDataPrimitive::VfxPrimitiveAttachedMesh(
                    VfxPrimitiveAttachedMesh { .. },
                ) => {
                    let black_pixel_texture = res_image.add(create_black_pixel_texture());
                    let material = MeshMaterial3d(res_particle_material_skinned_mesh_particle.add(
                        ParticleMaterialSkinnedMeshParticle {
                            uniforms_vertex: UniformsVertexSkinnedMeshParticle::default(),
                            uniforms_pixel: UniformsPixelSkinnedMeshParticle::default(),
                            texture: texture.clone(),
                            particle_color_texture: particle_color_texture.clone(),
                            cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Some(
                                black_pixel_texture,
                            ),
                            cmb_tex_fow_map_smp_clamp_no_mip: None,
                            blend_mode,
                        },
                    ));

                    spawn_shadow_skin_entity(
                        &mut commands,
                        particle_entity,
                        parent,
                        material,
                        q_mesh3d,
                        q_skinned_mesh,
                        q_children,
                        q_animation_target,
                    );
                }
                _ => {
                    continue;
                }
            }

            commands.entity(particle_entity).insert((
                transform,
                Pickable::IGNORE,
                ChildOf(emitter_entity),
            ));
        }
    }
}

#[derive(Component, Default)]
pub struct ParticleDecal {
    visible: HashSet<Entity>,
}

#[derive(Component)]
pub struct ParticleDecalGeometry(pub Entity);

pub fn update_decal_intersections(
    mut commands: Commands,
    mut q_decals: Query<(
        Entity,
        &MeshMaterial3d<ParticleMaterialUnlitDecal>,
        &mut ParticleDecal,
    )>,
    q_map_geo: Query<(Entity, &Mesh3d, &MapGeometry)>,
    q_particle_decal_geometry: Query<(Entity, &ParticleDecalGeometry)>,
    q_global_transform: Query<&GlobalTransform>,
) {
    for (particle_decal_entity, material, mut particle_decal) in q_decals.iter_mut() {
        let Ok(particle_decal_global_transform) = q_global_transform.get(particle_decal_entity)
        else {
            continue;
        };

        let current_bounding_box = Aabb3d::new(
            particle_decal_global_transform.translation(),
            particle_decal_global_transform.scale().abs(),
        );

        for (geometry_entity, mesh3d, map_geometry) in q_map_geo.iter() {
            if current_bounding_box.intersects(&map_geometry.bounding_box) {
                if !particle_decal.visible.contains(&geometry_entity) {
                    commands.spawn((
                        mesh3d.clone(),
                        material.clone(),
                        Pickable::IGNORE,
                        ParticleDecalGeometry(particle_decal_entity),
                    ));
                    particle_decal.visible.insert(geometry_entity);
                }
            } else {
                particle_decal.visible.remove(&geometry_entity);
            }
        }
    }

    for (decal_entity, decal_geometry) in q_particle_decal_geometry.iter() {
        if q_decals.get(decal_geometry.0).is_err() {
            commands.entity(decal_entity).despawn();
        }
    }
}
