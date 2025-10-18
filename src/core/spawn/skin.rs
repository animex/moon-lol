use std::collections::HashMap;

use bevy::animation::{AnimationTarget, AnimationTargetId};
use bevy::asset::uuid::Uuid;
use bevy::prelude::*;
use bevy::render::mesh::skinning::SkinnedMesh;

use league_utils::hash_bin;
use lol_config::{ConfigCharacterSkin, ConfigCharacterSkinAnimation};

use crate::core::{Animation, AnimationNode, AnimationNodeF32, AnimationState};

/// 从Config中的ConfigEnvironmentObject生成环境对象实体
pub fn spawn_skin_entity(
    commands: &mut Commands,
    res_animation_graph: &mut ResMut<Assets<AnimationGraph>>,
    asset_server: &Res<AssetServer>,
    transform: Transform,
    skin: &ConfigCharacterSkin,
) -> Entity {
    // 加载纹理
    let material_handle: Handle<StandardMaterial> = asset_server.load(skin.material_path.clone());

    // 创建父实体
    let parent_entity = commands
        .spawn(transform.with_scale(transform.scale * skin.skin_scale.unwrap_or(1.0)))
        .id();

    // 构建骨骼实体映射
    let mut index_to_entity = vec![Entity::PLACEHOLDER; skin.joints.len()];

    // 创建骨骼实体
    for (i, joint) in skin.joints.iter().enumerate() {
        let ent = commands
            .spawn((
                joint.transform,
                AnimationTarget {
                    id: AnimationTargetId(Uuid::from_u128(joint.hash as u128)),
                    player: parent_entity,
                },
            ))
            .id();
        index_to_entity[i] = ent;
    }

    // 建立骨骼父子关系
    for (i, joint) in skin.joints.iter().enumerate() {
        if joint.parent_index >= 0 {
            let parent_entity_joint = index_to_entity[joint.parent_index as usize];
            commands
                .entity(parent_entity_joint)
                .add_child(index_to_entity[i]);
        } else {
            commands.entity(parent_entity).add_child(index_to_entity[i]);
        }
    }

    let mut animation_graph = AnimationGraph::new();
    let mut hash_to_node = HashMap::new();

    for (hash, clip) in &skin.animation_map {
        match clip {
            ConfigCharacterSkinAnimation::AtomicClipData { clip_path } => {
                let clip = asset_server.load(clip_path.clone());
                let node_index = animation_graph.add_clip(clip, 1.0, animation_graph.root);
                hash_to_node.insert(*hash, AnimationNode::Clip { node_index });
            }
            ConfigCharacterSkinAnimation::ConditionFloatClipData {
                conditions,
                component_name,
                field_name,
            } => {
                hash_to_node.insert(
                    *hash,
                    AnimationNode::ConditionFloat {
                        component_name: component_name.clone(),
                        field_name: field_name.clone(),
                        conditions: conditions
                            .iter()
                            .map(|(key, value)| AnimationNodeF32 {
                                key: *key,
                                value: *value,
                            })
                            .collect::<Vec<_>>(),
                    },
                );
            }
            ConfigCharacterSkinAnimation::SelectorClipData { probably_nodes } => {
                hash_to_node.insert(
                    *hash,
                    AnimationNode::Selector {
                        probably_nodes: probably_nodes
                            .iter()
                            .map(|(key, value)| AnimationNodeF32 {
                                key: *key,
                                value: *value,
                            })
                            .collect::<Vec<_>>(),
                        current_index: None,
                    },
                );
            }
        };
    }

    let graph_handle = res_animation_graph.add(animation_graph);

    commands.entity(parent_entity).insert((
        AnimationPlayer::default(),
        AnimationGraphHandle(graph_handle),
        Animation { hash_to_node },
        AnimationState {
            last_hash: hash_bin("Idle1"),
            current_hash: hash_bin("Idle1"),
            current_duration: None,
            repeat: true,
        },
    ));

    // 加载和创建mesh实体
    for submesh_path in &skin.submesh_paths {
        let mesh_handle = asset_server.load(submesh_path.clone());

        let child = commands
            .spawn((
                Transform::default(),
                Mesh3d(mesh_handle),
                MeshMaterial3d(material_handle.clone()),
                SkinnedMesh {
                    inverse_bindposes: asset_server.load(&skin.inverse_bind_pose_path),
                    joints: skin
                        .joint_influences_indices
                        .iter()
                        .map(|&v| index_to_entity[v as usize])
                        .collect::<Vec<_>>(),
                },
            ))
            .id();
        commands.entity(parent_entity).add_child(child);
    }

    parent_entity
}
