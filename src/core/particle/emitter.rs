use std::collections::HashMap;

use bevy::{
    math::bounding::{Aabb3d, IntersectsVolume},
    prelude::*,
};

use league_core::{
    Unk0xee39916f, VfxEmitterDefinitionData, VfxEmitterDefinitionDataPrimitive,
    VfxEmitterDefinitionDataSpawnShape, VfxPrimitivePlanarProjection,
};
use league_utils::{neg_rotation_z, neg_vec_z};

use crate::core::{
    particle::{
        create_black_pixel_texture, ParticleMaterialQuad, ParticleMaterialQuadSlice,
        ParticleMeshQuad, ParticleState, Sampler, UniformsPixelQuadSlice, UniformsVertexQuad,
    },
    MapGeometry, ParticleId, ParticleMaterialUnlitDecal, UniformsPixelUnlitDecal,
    UniformsVertexUnlitDecal,
};

#[derive(Component)]
pub struct ParticleEmitterState {
    pub timer: Timer,
    pub rate_sampler: Sampler<f32>,
    pub lifetime_sampler: Sampler<f32>,
    pub rotation_sampler: Sampler<Vec3>,
    pub scale_sampler: Sampler<Vec3>,
    pub emission_debt: f32,
}

pub fn update_emitter(
    mut commands: Commands,
    mut res_mesh: ResMut<Assets<Mesh>>,
    res_asset_server: Res<AssetServer>,
    mut res_image: ResMut<Assets<Image>>,
    mut res_quad_material: ResMut<Assets<ParticleMaterialQuad>>,
    mut res_quad_slice_material: ResMut<Assets<ParticleMaterialQuadSlice>>,
    mut res_unlit_decal_material: ResMut<Assets<ParticleMaterialUnlitDecal>>,
    mut query: Query<(
        Entity,
        &ChildOf,
        &mut ParticleEmitterState,
        &VfxEmitterDefinitionData,
        &ParticleId,
    )>,
    time: Res<Time>,
) {
    for (emitter_entity, child_of, mut emitter, vfx_emitter_definition_data, particle_id) in
        query.iter_mut()
    {
        emitter.timer.tick(time.delta());

        if emitter.timer.finished() {
            commands.entity(emitter_entity).despawn();
            continue;
        }
        let normalized_time = emitter.timer.elapsed_secs() / emitter.timer.duration().as_secs_f32();

        let rate = emitter.rate_sampler.sample_clamped(normalized_time);
        let lifetime = emitter.lifetime_sampler.sample_clamped(normalized_time);
        let rotation = emitter.rotation_sampler.sample_clamped(normalized_time);
        let scale = emitter.scale_sampler.sample_clamped(normalized_time);

        // 计算本帧应该发射的粒子数量
        // 加上一帧的 "欠账"，这使得在低速率下也能平滑发射
        let particles_to_spawn_f32 = rate * time.delta_secs() + emitter.emission_debt;

        // 向下取整，得到本帧实际生成的整数粒子数
        let particles_to_spawn = particles_to_spawn_f32.floor() as u32;

        // 更新 "欠账"，为下一帧做准备
        emitter.emission_debt = particles_to_spawn_f32.fract();

        for _ in 0..particles_to_spawn {
            let primitive = vfx_emitter_definition_data
                .primitive
                .clone()
                .unwrap_or(VfxEmitterDefinitionDataPrimitive::VfxPrimitiveArbitraryQuad);

            let texture = vfx_emitter_definition_data
                .texture
                .as_ref()
                .map(|v| res_asset_server.load(v))
                .unwrap();

            let particle_color_texture = vfx_emitter_definition_data
                .particle_color_texture
                .as_ref()
                .map(|v| res_asset_server.load(v));

            let offset = match vfx_emitter_definition_data.spawn_shape.clone().unwrap() {
                VfxEmitterDefinitionDataSpawnShape::Unk0xee39916f(Unk0xee39916f {
                    emit_offset,
                }) => neg_vec_z(&emit_offset.unwrap()),
                _ => todo!(),
            };

            let is_local_orientation = vfx_emitter_definition_data
                .is_local_orientation
                .unwrap_or(true);

            let is_uniform_scale = vfx_emitter_definition_data
                .is_uniform_scale
                .unwrap_or(false);

            let blend_mode = vfx_emitter_definition_data.blend_mode.unwrap_or(4);

            let rotation_quat = Quat::from_euler(
                EulerRot::XYZEx,
                rotation.x.to_radians(),
                rotation.y.to_radians(),
                rotation.z.to_radians(),
            );

            let rotation_quat = neg_rotation_z(&rotation_quat);

            let mut transform = Transform::from_translation(offset)
                .with_scale(if is_uniform_scale {
                    Vec3::splat(scale.x)
                } else {
                    scale
                })
                .with_rotation(rotation_quat);

            println!("--------------------------------");
            println!("scale: {:?}", scale);

            if let VfxEmitterDefinitionDataPrimitive::VfxPrimitivePlanarProjection(
                VfxPrimitivePlanarProjection { ref m_projection },
            ) = primitive
            {
                transform.scale.x = transform.scale.x * 2.;
                transform.scale.y = m_projection.clone().unwrap().m_y_range.unwrap();
                transform.scale.z = -transform.scale.z * 2.;
            }

            let local_matrix = transform.compute_matrix();

            let particle_entity = commands
                .spawn((
                    particle_id.clone(),
                    ParticleState {
                        timer_life: Timer::from_seconds(lifetime, TimerMode::Repeating),
                        is_local_orientation,
                        source: child_of.0,
                        local_matrix,
                    },
                ))
                .id();

            if let VfxEmitterDefinitionDataPrimitive::VfxPrimitivePlanarProjection(..) = primitive {
                let material_handle = res_unlit_decal_material.add(ParticleMaterialUnlitDecal {
                    uniforms_vertex: UniformsVertexUnlitDecal {
                        decal_projection_y_range: Vec4::splat(transform.scale.y),
                        ..default()
                    },
                    uniforms_pixel: UniformsPixelUnlitDecal::default(),
                    diffuse_map: Some(texture.clone()),
                    particle_color_texture: particle_color_texture.clone(), // 使用上面定义的 texture
                    cmb_tex_fow_map_smp_clamp_no_mip: None,
                    is_local_orientation,
                    blend_mode,
                });

                commands.entity(particle_entity).insert((
                    DecalParticle,                   // 标记为贴花粒子
                    MeshMaterial3d(material_handle), // 存储材质句柄
                    Transform::default(),
                ));
            }

            let mesh = match primitive {
                VfxEmitterDefinitionDataPrimitive::VfxPrimitiveArbitraryQuad => {
                    res_mesh.add(ParticleMeshQuad::default())
                }
                _ => continue,
            };

            commands
                .entity(particle_entity)
                .insert((Mesh3d(mesh), transform, Pickable::IGNORE));

            let black_pixel_texture = res_image.add(create_black_pixel_texture());

            if let Some(range) = vfx_emitter_definition_data.slice_technique_range {
                commands.entity(particle_entity).insert(MeshMaterial3d(
                    res_quad_slice_material.add(ParticleMaterialQuadSlice {
                        uniforms_vertex: UniformsVertexQuad::default(),
                        uniforms_pixel: UniformsPixelQuadSlice {
                            slice_range: vec2(range, 1.0 / (range * range)),
                            ..default()
                        },
                        particle_color_texture,
                        texture: Some(texture),
                        cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Some(black_pixel_texture),
                        sampler_fow: None,
                        is_local_orientation,
                        blend_mode,
                    }),
                ));
            } else {
                commands
                    .entity(particle_entity)
                    .insert(MeshMaterial3d(res_quad_material.add(
                        ParticleMaterialQuad {
                            uniforms_vertex: UniformsVertexQuad::default(),
                            particle_color_texture: None,
                            texture: Some(texture),
                            cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Some(
                                black_pixel_texture,
                            ),
                            sampler_fow: None,
                            is_local_orientation,
                            blend_mode,
                        },
                    )));
            };

            commands.entity(child_of.0).add_child(particle_entity);
        }
    }
}

/// 一个标记组件，用于标识作为贴花投影源的粒子实体。
#[derive(Component)]
pub struct DecalParticle;

/// 一个组件，添加到实际渲染贴花的子实体上，
/// 用于追踪它代表的是哪一块地图几何体（MapGeometry）。
#[derive(Component)]
pub struct DecalMapGeoChild(pub Entity);

pub fn update_decal_intersections(
    mut commands: Commands,
    // 查询所有“贴花投影源”粒子
    q_decals: Query<
        (
            Entity,
            &ParticleState,
            &MeshMaterial3d<ParticleMaterialUnlitDecal>,
        ),
        With<DecalParticle>,
    >,
    // 查询所有地图几何体块
    q_map_geo: Query<(Entity, &Mesh3d, &MapGeometry)>,
    // 用于获取 source (发射器) 的 GlobalTransform
    q_source: Query<&GlobalTransform>,
    // 查询所有已生成的“贴花网格”子实体
    q_decal_children: Query<(Entity, &ChildOf, &DecalMapGeoChild)>,
) {
    // 1. 构建一个当前已存在的“贴花网格”子实体的查找表
    // 键: decal_entity (父粒子)
    // 值: HashMap<map_geo_entity, child_entity> (地图块 -> 对应的子实体)
    let mut decal_child_map: HashMap<Entity, HashMap<Entity, Entity>> = HashMap::new();
    for (child_entity, child_of, map_geo_child) in q_decal_children.iter() {
        decal_child_map
            .entry(child_of.parent())
            .or_default()
            .insert(map_geo_child.0, child_entity);
    }

    // 2. 遍历所有激活的“贴花投影源”粒子
    for (decal_entity, particle_state, material_handle) in q_decals.iter() {
        // 3. 获取 source (发射器) 的 GlobalTransform
        let Ok(source_transform) = q_source.get(particle_state.source) else {
            // 如果 source 找不到了 (可能已被销毁)，则跳过
            continue;
        };

        // 4. 计算粒子当前的“世界变换矩阵”和“世界包围盒”
        let world_matrix = source_transform.compute_matrix() * particle_state.local_matrix;
        let (scale, _rotation, translation) = world_matrix.to_scale_rotation_translation();
        let current_bounding_box = Aabb3d::new(translation, scale.abs());

        // 5. 获取这个贴花粒子“上一帧”的子实体
        // 我们将从中移除本帧仍然相交的，最后剩下的就是需要销毁的
        let mut children_to_despawn = decal_child_map.remove(&decal_entity).unwrap_or_default();

        // 6. 遍历所有地图几何体块
        for (map_entity, mesh3d, map_geometry) in q_map_geo.iter() {
            // 7. 检查相交
            if current_bounding_box.intersects(&map_geometry.bounding_box) {
                // 确实相交了
                // 检查这个地图块是否已存在对应的子实体
                if children_to_despawn.remove(&map_entity).is_none() {
                    // 不存在 (is_none)，意味着这是“新相交”，需要生成
                    let child_mesh = mesh3d.0.clone();
                    let child_material = material_handle.clone();

                    commands.entity(decal_entity).with_children(|parent| {
                        parent.spawn((
                            Mesh3d(child_mesh),
                            child_material,
                            DecalMapGeoChild(map_entity), // 记录它对应的地图块
                        ));
                    });
                }
                // 如果 `children_to_despawn.remove` 返回 `Some`，
                // 说明它上一帧就在，这一帧还在，我们什么都不用做。
                // 它已从 "待销毁" 列表中移除。
            }
        }

        // 8. 销毁“孤儿”子实体
        // 经过第 7 步的循环，`children_to_despawn` 中剩下的
        // 就是那些“上一帧相交，但这一帧不再相交”的子实体
        for child_entity in children_to_despawn.values() {
            commands.entity(*child_entity).despawn();
        }
    }
}
