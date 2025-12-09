use std::collections::HashMap;

use bevy::animation::{AnimationTarget, AnimationTargetId};
use bevy::asset::uuid::Uuid;
use bevy::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use league_core::SkinCharacterDataProperties;
use league_file::{LeagueSkeleton, LeagueSkinnedMesh};
use league_to_lol::skinned_mesh_to_intermediate;
use league_utils::{get_asset_id_by_path, hash_joint};

use super::{CommandSpawnAnimation, CommandSpawnMesh};

struct ConfigJoint {
    hash: u32,
    transform: Transform,
    parent_index: i16,
}

pub fn on_command_spawn_mesh(
    trigger: On<CommandSpawnMesh>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut res_assets_mesh: ResMut<Assets<Mesh>>,
    res_assets_skin_character_data_properties: Res<Assets<SkinCharacterDataProperties>>,
    mut res_assets_standard_material: ResMut<Assets<StandardMaterial>>,
    mut res_assets_league_skeleton: ResMut<Assets<LeagueSkeleton>>,
    mut res_assets_league_skinned_mesh: ResMut<Assets<LeagueSkinnedMesh>>,
    mut res_assets_skinned_mesh_inverse_bindposes: ResMut<Assets<SkinnedMeshInverseBindposes>>,
) {
    let entity = trigger.event_target();
    let skin_path = &trigger.skin_key;
    let skin_character_data_properties = res_assets_skin_character_data_properties
        .get(trigger.skin_key)
        .unwrap();

    let skin_mesh_properties = skin_character_data_properties
        .skin_mesh_properties
        .as_ref()
        .unwrap();

    let texture = skin_mesh_properties.texture.clone().unwrap();

    let material_handle = get_standard(
        &mut res_assets_standard_material,
        &asset_server,
        Some(texture),
    );

    let league_skeleton = res_assets_league_skeleton
        .get(get_asset_id_by_path(
            skin_mesh_properties.skeleton.as_ref().unwrap(),
        ))
        .unwrap();

    let inverse_bindposes = res_assets_skinned_mesh_inverse_bindposes.add(
        league_skeleton
            .modern_data
            .influences
            .iter()
            .map(|&v| league_skeleton.modern_data.joints[v as usize].inverse_bind_transform)
            .collect::<Vec<_>>(),
    );

    let joints = league_skeleton
        .modern_data
        .joints
        .iter()
        .map(|joint| ConfigJoint {
            hash: hash_joint(&joint.name),
            transform: Transform::from_matrix(joint.local_transform),
            parent_index: joint.parent_index,
        })
        .collect::<Vec<_>>();

    let joint_influences_indices = &league_skeleton.modern_data.influences;

    let mut index_to_entity = vec![Entity::PLACEHOLDER; joints.len()];

    for (i, joint) in joints.iter().enumerate() {
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

    for (i, joint) in joints.iter().enumerate() {
        if joint.parent_index >= 0 {
            let parent_entity_joint = index_to_entity[joint.parent_index as usize];
            commands
                .entity(parent_entity_joint)
                .add_child(index_to_entity[i]);
        } else {
            commands.entity(entity).add_child(index_to_entity[i]);
        }
    }

    let joints = joint_influences_indices
        .iter()
        .map(|&v| index_to_entity[v as usize])
        .collect::<Vec<_>>();

    let skinned_mesh = SkinnedMesh {
        inverse_bindposes,
        joints,
    };

    let league_skinned_mesh = res_assets_league_skinned_mesh
        .get(get_asset_id_by_path(
            skin_mesh_properties.simple_skin.as_ref().unwrap(),
        ))
        .unwrap();

    for (i, _) in league_skinned_mesh.ranges.iter().enumerate() {
        let mesh = skinned_mesh_to_intermediate(&league_skinned_mesh, i);
        let mesh_handle = res_assets_mesh.add(mesh);
        commands.entity(entity).with_child((
            Transform::default(),
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle.clone()),
            skinned_mesh.clone(),
        ));
    }

    commands.entity(entity).insert(skinned_mesh.clone());

    commands.trigger(CommandSpawnAnimation {
        entity,
        skin_key: skin_path.clone(),
    });
}

pub fn get_standard(
    res_assets_standard_material: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    texture: Option<String>,
) -> Handle<StandardMaterial> {
    let material_handle = res_assets_standard_material.add(StandardMaterial {
        base_color_texture: texture.map(|v| asset_server.load(v)),
        unlit: true,
        cull_mode: Some(Face::Front),
        alpha_mode: AlphaMode::Mask(0.3),
        ..default()
    });

    material_handle
}

pub fn spawn_shadow_skin_entity<M: Material>(
    commands: &mut Commands,
    target: Entity,
    skin_entity: Entity,
    material: MeshMaterial3d<M>,
    q_mesh3d: Query<&Mesh3d>,
    q_skinned_mesh: Query<&SkinnedMesh>,
    q_children: Query<&Children>,
    q_animation_target: Query<(Entity, &Transform, &AnimationTarget)>,
) {
    let children = q_children.get(skin_entity).unwrap();

    let skinned_mesh = q_skinned_mesh.get(skin_entity).unwrap();

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
        joints,
        &q_children,
        &q_animation_target,
        &mut joint_map,
    );

    let new_joints = skinned_mesh
        .joints
        .iter()
        .map(|old_joint_entity| *joint_map.get(old_joint_entity).unwrap())
        .collect::<Vec<_>>();

    let new_skinned_mesh = SkinnedMesh {
        inverse_bindposes: skinned_mesh.inverse_bindposes.clone(),
        joints: new_joints,
    };

    commands.entity(target).insert(new_skinned_mesh.clone());

    for child in children.iter() {
        if let Ok(mesh) = q_mesh3d.get(child) {
            commands.entity(target).with_child((
                mesh.clone(),
                material.clone(),
                new_skinned_mesh.clone(),
            ));
        }
    }
}

pub fn duplicate_joints_to_target(
    commands: &mut Commands,
    parent: Entity,
    joints: Vec<(Entity, &Transform, &AnimationTarget)>,
    q_children: &Query<&Children>,
    q_animation_target: &Query<(Entity, &Transform, &AnimationTarget)>,
    joint_map: &mut HashMap<Entity, Entity>,
) {
    for (joint_entity, transform, anim_target) in joints {
        let new_joint_entity = commands
            .spawn((transform.clone(), anim_target.clone()))
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
                new_joint_entity,
                joints,
                q_children,
                q_animation_target,
                joint_map,
            );
        }
    }
}
