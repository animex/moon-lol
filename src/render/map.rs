use crate::combat::{AttackState, Movement, MovementDestination, Target};
use crate::map::Map;
use crate::render::{
    get_barrack_by_bin, neg_z, process_map_geo_mesh, EnvironmentVisibility,
    LayerTransitionBehavior, LeagueBinMaybeCharacterMapRecord, LeagueLoader, WadRes,
};
use crate::system_info;
use bevy::render::mesh::skinning::SkinnedMeshInverseBindposes;
use bevy::{color::palettes, prelude::*};
use cdragon_prop::{BinHash, BinMap, BinStruct};
use std::cmp::Ordering;

pub struct PluginMap;

impl Plugin for PluginMap {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentVisibilityLayers(EnvironmentVisibility::Layer1));
        app.add_systems(Startup, setup_map);
        app.add_systems(Startup, setup_map_placeble);
        app.add_systems(Update, update_z);
    }
}

// 用于存储全局选择的可见性图层的 Resource
#[derive(Resource, Debug)]
pub struct CurrentVisibilityLayers(pub EnvironmentVisibility);

// 用于标记每个地图网格实体所属图层的 Component
#[derive(Component, Debug)]
pub struct MapMeshLayer(pub EnvironmentVisibility);

fn setup_map(
    mut commands: Commands,
    res_wad: Res<WadRes>,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut res_image: ResMut<Assets<Image>>,
    mut res_materials: ResMut<Assets<StandardMaterial>>,
) {
    let start_time = std::time::Instant::now();

    for map_mesh in res_wad.loader.map_geo.meshes.iter() {
        if map_mesh.layer_transition_behavior != LayerTransitionBehavior::Unaffected {
            continue;
        }

        let bevy_meshes = process_map_geo_mesh(
            &res_wad.loader.map_materials.0,
            &res_wad.loader.map_geo,
            map_mesh,
            &res_wad.loader,
        );

        for (mesh, material_image) in bevy_meshes {
            let _format = material_image.texture_descriptor.format.clone();

            commands.spawn((
                Mesh3d(res_mesh.add(mesh)),
                MeshMaterial3d(res_materials.add(StandardMaterial {
                    base_color_texture: Some(res_image.add(material_image)),
                    unlit: true,
                    alpha_mode: AlphaMode::Mask(0.3),
                    ..default()
                })),
                Visibility::Visible,
                MapMeshLayer(map_mesh.environment_visibility),
                Map,
            ));
        }
    }

    system_info!("setup_map", "Map loaded in {:?}", start_time.elapsed());

    // commands.insert_resource(AmbientLight::default());
}

pub fn draw_attack(
    mut gizmos: Gizmos,
    q_attack: Query<(&Transform, &AttackState)>,
    q_movement_destination: Query<(&Transform, &MovementDestination)>,
    q_target: Query<(&Transform, &Target)>,
    q_transform: Query<&Transform>,
) {
    for (transform, attack_info) in q_attack.iter() {
        let Some(target) = attack_info.target else {
            continue;
        };
        let Ok(target_transform) = q_transform.get(target) else {
            continue;
        };
        gizmos.line(
            transform.translation,
            target_transform.translation,
            Color::Srgba(palettes::tailwind::RED_500),
        );
    }

    for (transform, movement_destination) in q_movement_destination.iter() {
        let destination = movement_destination.0;

        gizmos.line(
            transform.translation,
            transform
                .translation
                .clone()
                .with_x(destination.x)
                .with_z(destination.y),
            Color::Srgba(palettes::tailwind::GREEN_500),
        );
    }

    for (transform, target) in q_target.iter() {
        let Ok(target_transform) = q_transform.get(target.0) else {
            continue;
        };
        gizmos.line(
            transform.translation,
            target_transform.translation,
            Color::Srgba(palettes::tailwind::YELLOW_500),
        );
    }
}

fn setup_map_placeble(
    res_wad: Res<WadRes>,
    mut commands: Commands,
    mut character_cache: ResMut<crate::render::CharacterResourceCache>,
    mut res_animation_clips: ResMut<Assets<AnimationClip>>,
    mut res_animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut res_image: ResMut<Assets<Image>>,
    mut res_materials: ResMut<Assets<StandardMaterial>>,
    mut res_meshes: ResMut<Assets<Mesh>>,
    mut res_skinned_mesh_inverse_bindposes: ResMut<Assets<SkinnedMeshInverseBindposes>>,
) {
    let start_time = std::time::Instant::now();
    let bin = &res_wad.loader.map_materials.0;

    bin.entries
        .iter()
        .filter(|v| v.ctype.hash == LeagueLoader::hash_bin("MapPlaceableContainer"))
        .filter_map(|v| v.getv::<BinMap>(LeagueLoader::hash_bin("items").into()))
        .filter_map(|v| v.downcast::<BinHash, BinStruct>())
        .flatten()
        .for_each(|v| match v.1.ctype.hash {
            0x1e1cce2 => {
                let mut character_map_record = LeagueBinMaybeCharacterMapRecord::from(&v.1);

                neg_z(&mut character_map_record.transform);

                crate::render::spawn_character_cached(
                    &mut commands,
                    &mut character_cache,
                    &mut res_animation_clips,
                    &mut res_animation_graphs,
                    &mut res_image,
                    &mut res_materials,
                    &mut res_meshes,
                    &mut res_skinned_mesh_inverse_bindposes,
                    &res_wad.loader,
                    character_map_record.transform,
                    &character_map_record.definition.skin,
                );
            }
            0x71d0eabd => {
                commands.spawn(get_barrack_by_bin(&bin, &v.1));
            }
            _ => {}
        });

    system_info!(
        "setup_map_placeble",
        "map objects loaded in {:?}",
        start_time.elapsed()
    );
}

fn update_z(
    mut ray_cast: MeshRayCast,
    map_query: Query<(), With<Map>>,
    // 新增一个查询，用于获取实体的父级
    parent_query: Query<&ChildOf>,
    mut q_champion: Query<(Entity, &mut Transform), With<Movement>>,
) {
    for (champion_entity, mut transform) in q_champion.iter_mut() {
        let ray_origin = Vec3::new(transform.translation.x, 10000.0, transform.translation.z);
        let ray = Ray3d::new(ray_origin, -Dir3::Y);

        // 在闭包中，我们需要检查实体自身或其任何父级是否是地图
        let filter = |entity: Entity| {
            if map_query.contains(entity) {
                return true; // 找到了！这个碰撞有效
            }
            return false;

            // 首先，仍然要确保不与自己碰撞
            if entity == champion_entity {
                return false;
            }

            let mut current_entity = entity;
            let mut depth = 0;
            // 循环向上查找
            loop {
                // 检查当前实体是否是 Map

                // 防止无限循环
                depth += 1;
                if depth > 10 {
                    return false;
                }

                // 尝试获取当前实体的父级
                match parent_query.get(current_entity) {
                    // 如果有父级，下一轮循环就检查父级
                    Ok(parent) => current_entity = parent.parent(),
                    // 如果没有父级了（已经到了层级顶端），说明不是地图的一部分
                    Err(_) => return false,
                }
            }
        };

        let settings = MeshRayCastSettings::default().with_filter(&filter);
        let hits = ray_cast.cast_ray(ray, &settings);
        if hits.len() > 1 {
            println!("hits: {:?}", hits.len());
        }

        // ... 后续逻辑保持不变 ...
        let highest_hit = hits.iter().max_by(|a, b| {
            a.1.point
                .y
                .partial_cmp(&b.1.point.y)
                .unwrap_or(Ordering::Equal)
        });

        if let Some(intersection) = highest_hit {
            transform.translation.y = intersection.1.point.y;
        }
    }
}
