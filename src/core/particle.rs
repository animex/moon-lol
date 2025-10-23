mod emitter;
mod environment;
mod particle;
mod skinned_mesh;
mod utils;

pub use emitter::*;
pub use environment::*;
pub use particle::*;
pub use skinned_mesh::*;
pub use utils::*;

use bevy::platform::collections::HashMap;
use league_core::{
    Unk0xee39916f, ValueColor, ValueFloat, ValueVector2, ValueVector3,
    VfxEmitterDefinitionDataSpawnShape, VfxShapeLegacy,
};
use league_utils::hash_wad;

use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, VertexFormat};

use lol_config::ConfigMap;

use crate::core::{Lifetime, LifetimeMode};

pub const ATTRIBUTE_WORLD_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_World_Position", 7, VertexFormat::Float32x3);

pub const ATTRIBUTE_UV_FRAME: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_UV_FRAME", 8, VertexFormat::Float32x4);

pub const ATTRIBUTE_LIFETIME: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_LIFETIME", 9, VertexFormat::Float32x2);

pub const ATTRIBUTE_UV_MULT: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_UV_MULT", 99, VertexFormat::Float32x2);

#[derive(Default)]
pub struct PluginParticle;

impl Plugin for PluginParticle {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_particle_spawn);
        app.add_observer(on_command_particle_despawn);

        app.add_plugins(MaterialPlugin::<ParticleMaterialQuad>::default());
        app.add_plugins(MaterialPlugin::<ParticleMaterialQuadSlice>::default());
        app.add_plugins(MaterialPlugin::<ParticleMaterialUnlitDecal>::default());
        app.add_plugins(MaterialPlugin::<ParticleMaterialMesh>::default());
        app.add_plugins(MaterialPlugin::<ParticleMaterialSkinnedMeshParticle>::default());

        app.init_asset::<ParticleMaterialQuad>();
        app.init_asset::<ParticleMaterialQuadSlice>();
        app.init_asset::<ParticleMaterialUnlitDecal>();
        app.init_asset::<ParticleMaterialMesh>();
        app.init_asset::<ParticleMaterialSkinnedMeshParticle>();

        app.init_resource::<ParticleMesh>();

        app.add_systems(
            PostUpdate,
            (
                update_emitter_position,
                update_emitter,
                update_decal_intersections,
                update_particle_transform,
                update_particle,
                update_particle_skinned_mesh_particle,
            )
                .chain()
                .after(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Resource, Default)]
pub struct ParticleMesh(HashMap<u64, Handle<Mesh>>);

#[derive(Component, Clone)]
pub struct ParticleId(pub u32);

#[derive(Event)]
pub struct CommandParticleSpawn {
    pub particle: u32,
}

#[derive(Event)]
pub struct CommandParticleDespawn {
    pub particle: u32,
}

impl ParticleMesh {
    pub fn get_mesh_handle(self: &Self, path: &str) -> Option<Handle<Mesh>> {
        return self.0.get(&hash_wad(path)).cloned();
    }
}

fn on_command_particle_spawn(
    trigger: Trigger<CommandParticleSpawn>,
    mut commands: Commands,
    res_config_map: Res<ConfigMap>,
) {
    let vfx_system_definition_data = res_config_map
        .vfx_system_definition_datas
        .get(&trigger.particle)
        .unwrap();

    // if !vfx_system_definition_data
    //     .particle_name
    //     .ends_with("Fiora_Base_Passive_SW")
    // {
    //     return;
    // }

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

    for vfx_emitter_definition_data in vfx_emitter_definition_datas.into_iter() {
        // if vfx_emitter_definition_data.emitter_name.clone().unwrap() != "decal" {
        //     continue;
        // }

        let is_single_particle = vfx_emitter_definition_data
            .is_single_particle
            .unwrap_or(false);

        let rate = vfx_emitter_definition_data.rate.clone().unwrap();
        let particle_lifetime = vfx_emitter_definition_data
            .particle_lifetime
            .clone()
            .unwrap_or(ValueFloat {
                dynamics: None,
                constant_value: Some(1.0),
            });
        let color = vfx_emitter_definition_data
            .color
            .clone()
            .unwrap_or(ValueColor {
                dynamics: None,
                constant_value: Some(Vec4::ONE),
            });
        let scale0 = vfx_emitter_definition_data
            .scale0
            .clone()
            .unwrap_or(ValueVector3 {
                dynamics: None,
                constant_value: Some(Vec3::ONE),
            });
        let birth_velocity = vfx_emitter_definition_data
            .birth_velocity
            .clone()
            .unwrap_or(ValueVector3 {
                dynamics: None,
                constant_value: Some(Vec3::ZERO),
            });
        let birth_acceleration = vfx_emitter_definition_data
            .birth_acceleration
            .clone()
            .unwrap_or(ValueVector3 {
                dynamics: None,
                constant_value: Some(Vec3::ZERO),
            });
        let birth_color = vfx_emitter_definition_data
            .birth_color
            .clone()
            .unwrap_or(ValueColor {
                dynamics: None,
                constant_value: Some(Vec4::ONE),
            });
        let birth_rotation0 = vfx_emitter_definition_data
            .birth_rotation0
            .clone()
            .unwrap_or(ValueVector3 {
                dynamics: None,
                constant_value: Some(Vec3::ZERO),
            });
        let birth_scale0 =
            vfx_emitter_definition_data
                .birth_scale0
                .clone()
                .unwrap_or(ValueVector3 {
                    dynamics: None,
                    constant_value: Some(Vec3::ONE),
                });
        let birth_uv_offset = vfx_emitter_definition_data
            .birth_uv_offset
            .clone()
            .unwrap_or(ValueVector2 {
                dynamics: None,
                constant_value: Some(Vec2::ZERO),
            });
        let birth_uv_scroll_rate = vfx_emitter_definition_data
            .birth_uv_scroll_rate
            .clone()
            .unwrap_or(ValueVector2 {
                dynamics: None,
                constant_value: Some(Vec2::ZERO),
            });
        let emitter_position = vfx_emitter_definition_data
            .emitter_position
            .clone()
            .unwrap_or(ValueVector3 {
                dynamics: None,
                constant_value: Some(Vec3::ZERO),
            });

        commands.entity(trigger.target()).with_child((
            vfx_emitter_definition_data.clone(),
            ParticleId(trigger.particle),
            ParticleEmitterState {
                birth_acceleration: birth_acceleration.into(),
                birth_color: birth_color.into(),
                birth_rotation0: birth_rotation0.into(),
                birth_scale0: birth_scale0.into(),
                birth_uv_offset: birth_uv_offset.into(),
                birth_uv_scroll_rate: birth_uv_scroll_rate.into(),
                birth_velocity: birth_velocity.into(),
                color: color.into(),
                scale0: scale0.into(),
                emission_debt: if is_single_particle { 1. } else { 0. },
                particle_lifetime: particle_lifetime.into(),
                rate: rate.into(),
                emitter_position: emitter_position.into(),
                world_matrix: Mat4::default(),
            },
            Lifetime::new(
                vfx_emitter_definition_data.lifetime.unwrap_or(1.0),
                LifetimeMode::TimerAndNoChildren,
            ),
            Transform::default(),
        ));
    }
}

fn on_command_particle_despawn(
    trigger: Trigger<CommandParticleDespawn>,
    mut commands: Commands,
    q_children: Query<&Children>,
    q_particle_emitter: Query<(Entity, &ParticleId)>,
) {
    let Ok(children) = q_children.get(trigger.target()) else {
        return;
    };

    for child in children.iter() {
        let Ok((emitter_or_particle_entity, particle)) = q_particle_emitter.get(child) else {
            continue;
        };

        if particle.0 == trigger.particle {
            commands.entity(emitter_or_particle_entity).despawn();
        }
    }
}
