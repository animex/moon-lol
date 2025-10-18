mod emitter;
mod particle;
mod ps;
mod utils;
mod vs;

pub use emitter::*;
pub use particle::*;
pub use ps::*;
pub use utils::*;
pub use vs::*;

use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, VertexFormat};

use league_core::{ValueFloat, ValueVector3};
use lol_config::ConfigMap;

pub const ATTRIBUTE_WORLD_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_World_Position", 7, VertexFormat::Float32x3);

pub const ATTRIBUTE_UV_FRAME: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_Frame", 8, VertexFormat::Float32x4);

pub const ATTRIBUTE_LIFETIME: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertext_Life", 9, VertexFormat::Float32x2);

#[derive(Default)]
pub struct PluginParticle;

impl Plugin for PluginParticle {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_particle_spawn);
        app.add_observer(on_command_particle_despawn);

        app.add_plugins(MaterialPlugin::<ParticleMaterialQuad>::default());
        app.add_plugins(MaterialPlugin::<ParticleMaterialQuadSlice>::default());
        app.add_plugins(MaterialPlugin::<ParticleMaterialUnlitDecal>::default());

        app.init_asset::<ParticleMaterialQuad>();
        app.init_asset::<ParticleMaterialQuadSlice>();
        app.init_asset::<ParticleMaterialUnlitDecal>();

        app.add_systems(Update, update_emitter);
        app.add_systems(Update, update_decal_intersections);
        app.add_systems(Last, update_particle);
    }
}

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

fn on_command_particle_spawn(
    trigger: Trigger<CommandParticleSpawn>,
    mut commands: Commands,
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

    for vfx_emitter_definition_data in vfx_emitter_definition_datas.into_iter() {
        let rate = vfx_emitter_definition_data.rate.clone().unwrap();
        let particle_lifetime = vfx_emitter_definition_data
            .particle_lifetime
            .clone()
            .unwrap_or(ValueFloat {
                dynamics: None,
                constant_value: Some(1.0),
            });
        let birth_rotation = vfx_emitter_definition_data
            .birth_rotation0
            .clone()
            .unwrap_or(ValueVector3 {
                dynamics: None,
                constant_value: Some(Vec3::ZERO),
            });
        let birth_scale =
            vfx_emitter_definition_data
                .birth_scale0
                .clone()
                .unwrap_or(ValueVector3 {
                    dynamics: None,
                    constant_value: Some(Vec3::ONE),
                });

        commands.entity(trigger.target()).with_child((
            vfx_emitter_definition_data.clone(),
            ParticleId(trigger.particle),
            ParticleEmitterState {
                timer: Timer::from_seconds(
                    vfx_emitter_definition_data.lifetime.unwrap_or(10.0),
                    TimerMode::Repeating,
                ),
                rate_sampler: rate.into(),
                lifetime_sampler: particle_lifetime.into(),
                rotation_sampler: birth_rotation.into(),
                scale_sampler: birth_scale.into(),
                emission_debt: 1.0,
            },
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
