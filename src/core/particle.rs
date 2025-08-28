use bevy::prelude::*;

use league_core::VfxEmitterDefinitionDataPrimitive;
use lol_config::ConfigMap;

#[derive(Default)]
pub struct PluginParticle;

impl Plugin for PluginParticle {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_particle_spawn);

        app.add_systems(PostUpdate, update_particle_transform);
    }
}

#[derive(Component)]
pub struct Particle(pub u32);

#[derive(Component)]
struct WorldOriented(pub Quat);

#[derive(Event)]
pub struct CommandParticleSpawn {
    pub particle: u32,
}

fn on_command_particle_spawn(
    trigger: Trigger<CommandParticleSpawn>,
    mut commands: Commands,
    res_asset_server: Res<AssetServer>,
    mut res_material: ResMut<Assets<StandardMaterial>>,
    mut res_mesh: ResMut<Assets<Mesh>>,
    res_config_map: Res<ConfigMap>,
) {
    let vfx_system_definition_data = res_config_map
        .vfx_system_definition_datas
        .get(&trigger.particle)
        .unwrap();

    let mut vfx_emitter_definition_datas = Vec::new();

    if let Some(complex_emitter_definition_data) =
        &vfx_system_definition_data.complex_emitter_definition_data
    {
        vfx_emitter_definition_datas.extend(complex_emitter_definition_data);
    }

    if let Some(simple_emitter_definition_data) =
        &vfx_system_definition_data.simple_emitter_definition_data
    {
        vfx_emitter_definition_datas.extend(simple_emitter_definition_data);
    }

    for vfx_emitter_definition_data in vfx_emitter_definition_datas.iter().take(1) {
        let Some(mesh) = vfx_emitter_definition_data.primitive.as_ref().map(|v| {
            res_mesh.add(match v {
                VfxEmitterDefinitionDataPrimitive::VfxPrimitiveArbitraryQuad => {
                    Plane3d::new(vec3(0.0, 1.0, 0.0), Vec2::splat(100.0))
                }
                _ => todo!(),
            })
        }) else {
            continue;
        };

        let Some(texture) = vfx_emitter_definition_data
            .texture
            .as_ref()
            .map(|v| res_asset_server.load(v))
        else {
            continue;
        };

        let material = res_material.add(StandardMaterial {
            base_color_texture: Some(texture),
            unlit: true,
            depth_bias: -80.0,
            alpha_mode: AlphaMode::Mask(0.3),
            ..default()
        });

        let is_local_orientation = vfx_emitter_definition_data
            .is_local_orientation
            .unwrap_or(false);

        let mut target =
            commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), Transform::default()));

        println!("is_local_orientation: {:?}", is_local_orientation);
        if is_local_orientation {}
        println!("is_local_orientation");
        target.insert(WorldOriented(Quat::IDENTITY));

        let target_id = target.id();

        commands.entity(trigger.target()).add_child(target_id);
    }
}

/// 更新所有拥有 WorldOriented 组件的实体的 Transform，
/// 使其在世界空间中保持固定的旋转。
fn update_particle_transform(
    // 查询1: 获取所有需要更新的实体及其目标世界旋转
    q_world_oriented: Query<(Entity, &WorldOriented)>,
    // 查询2: 用于获取实体的父实体
    q_parent: Query<&ChildOf>,
    // 查询3: 用于获取父实体的世界变换 (GlobalTransform)
    q_global_transform: Query<&GlobalTransform>,
    // 查询4: 用于修改实体的本地变换 (Transform)
    mut q_transform: Query<&mut Transform>,
) {
    // 遍历所有带有 WorldOriented 组件的实体
    for (entity, world_oriented) in q_world_oriented.iter() {
        let target_rotation = world_oriented.0;

        // 获取当前实体的可变 Transform，如果获取失败则跳过
        if let Ok(mut local_transform) = q_transform.get_mut(entity) {
            // 尝试获取该实体的父实体
            let parent_world_rotation = if let Ok(child) = q_parent.get(entity) {
                // 如果父实体存在，就获取它的 GlobalTransform
                // .get(parent.get()) 通过父实体的 Entity ID 获取其组件
                if let Ok(parent_global_transform) = q_global_transform.get(child.parent()) {
                    // 从 GlobalTransform 计算出世界旋转
                    parent_global_transform.compute_transform().rotation
                } else {
                    // 父实体存在，但没有 GlobalTransform，这在 Bevy 中很少见
                    // 默认视为没有旋转
                    Quat::IDENTITY
                }
            } else {
                // 如果实体没有父实体，则其父旋转可视为单位旋转
                Quat::IDENTITY
            };

            // 核心公式: LocalRotation = Inverse(ParentWorldRotation) * TargetWorldRotation
            local_transform.rotation = parent_world_rotation.inverse() * target_rotation;
        }
    }
}
