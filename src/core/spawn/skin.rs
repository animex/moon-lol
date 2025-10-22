use std::collections::HashMap;

use bevy::animation::{AnimationTarget, AnimationTargetId};
use bevy::asset::uuid::Uuid;
use bevy::prelude::*;
use bevy::render::mesh::skinning::SkinnedMesh;

use league_utils::hash_bin;
use lol_config::{ConfigCharacterSkin, ConfigCharacterSkinAnimation};

use crate::core::{Animation, AnimationNode, AnimationNodeF32, AnimationState};

pub fn spawn_skin_entity(
    commands: &mut Commands,
    res_animation_graph: &mut ResMut<Assets<AnimationGraph>>,
    asset_server: &Res<AssetServer>,
    transform: Transform,
    skin: &ConfigCharacterSkin,
) -> Entity {
    let material_handle: Handle<StandardMaterial> = asset_server.load(skin.material_path.clone());

    let entity = commands
        .spawn(transform.with_scale(transform.scale * skin.skin_scale.unwrap_or(1.0)))
        .id();

    let mut index_to_entity = vec![Entity::PLACEHOLDER; skin.joints.len()];

    for (i, joint) in skin.joints.iter().enumerate() {
        let ent = commands
            .spawn((
                joint.transform,
                AnimationTarget {
                    id: AnimationTargetId(Uuid::from_u128(joint.hash as u128)),
                    player: entity,
                },
            ))
            .id();
        index_to_entity[i] = ent;
    }

    for (i, joint) in skin.joints.iter().enumerate() {
        if joint.parent_index >= 0 {
            let parent_entity_joint = index_to_entity[joint.parent_index as usize];
            commands
                .entity(parent_entity_joint)
                .add_child(index_to_entity[i]);
        } else {
            commands.entity(entity).add_child(index_to_entity[i]);
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

    commands.entity(entity).insert((
        AnimationPlayer::default(),
        AnimationGraphHandle(graph_handle),
        Animation { hash_to_node },
        AnimationState {
            last_hash: None,
            current_hash: hash_bin("Idle1"),
            current_duration: None,
            repeat: true,
        },
    ));

    let inverse_bindposes = asset_server.load(&skin.inverse_bind_pose_path);
    let joints = skin
        .joint_influences_indices
        .iter()
        .map(|&v| index_to_entity[v as usize])
        .collect::<Vec<_>>();
    let skinned_mesh = SkinnedMesh {
        inverse_bindposes,
        joints,
    };

    for submesh_path in &skin.submesh_paths {
        let mesh_handle = asset_server.load(submesh_path.clone());

        commands.entity(entity).with_child((
            Transform::default(),
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle.clone()),
            skinned_mesh.clone(),
        ));
    }

    entity
}

pub fn spawn_shadow_skin_entity<M: Material>(
    commands: &mut Commands,
    target: Entity,
    skin_entity: Entity,
    material: MeshMaterial3d<M>,
    q_skin_mesh: Query<(&Mesh3d, &SkinnedMesh)>,
    q_children: Query<&Children>,
    q_animation_target: Query<(Entity, &Transform, &AnimationTarget)>,
) {
    let children = q_children.get(skin_entity).unwrap();

    commands.entity(target).insert(material.clone());

    let mut joints = Vec::new();

    for child in children.iter() {
        if let Ok(joint) = q_animation_target.get(child) {
            joints.push(joint);
        }
    }

    let mut joint_map: HashMap<Entity, Entity> = HashMap::new();

    duplicate_joints_to_target(
        commands,
        target,
        target,
        joints,
        &q_children,
        &q_animation_target,
        &mut joint_map,
    );

    for child in children.iter() {
        if let Ok((mesh, skinned_mesh)) = q_skin_mesh.get(child) {
            let new_joints = skinned_mesh
                .joints
                .iter()
                .map(|old_joint_entity| *joint_map.get(old_joint_entity).unwrap())
                .collect::<Vec<_>>();

            let new_skinned_mesh = SkinnedMesh {
                inverse_bindposes: skinned_mesh.inverse_bindposes.clone(),
                joints: new_joints,
            };

            commands.entity(target).with_child((
                mesh.clone(),
                material.clone(),
                // skinned_mesh.clone(),
                // mat.clone(),
                new_skinned_mesh,
            ));
        }
    }
}

pub fn duplicate_joints_to_target(
    commands: &mut Commands,
    target: Entity,
    parent: Entity,
    joints: Vec<(Entity, &Transform, &AnimationTarget)>,
    q_children: &Query<&Children>,
    q_animation_target: &Query<(Entity, &Transform, &AnimationTarget)>,
    joint_map: &mut HashMap<Entity, Entity>,
) {
    for (joint_entity, transform, anim_target) in joints {
        let new_joint_entity = commands
            .spawn((
                transform.clone(),
                // anim_target.clone()
            ))
            .id();

        commands.entity(parent).add_child(new_joint_entity);

        joint_map.insert(joint_entity, new_joint_entity);

        if let Ok(children) = q_children.get(joint_entity) {
            let mut joints = Vec::new();

            for child in children {
                if let Ok(joint) = q_animation_target.get(*child) {
                    joints.push(joint);
                }
            }

            duplicate_joints_to_target(
                commands,
                target,
                new_joint_entity,
                joints,
                q_children,
                q_animation_target,
                joint_map,
            );
        }
    }
}
