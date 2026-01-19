use bevy::prelude::*;
use league_core::{ResourceResolver, SkinCharacterDataProperties};
use lol_config::LoadHashKeyTrait;

use crate::{CommandParticleDespawn, CommandParticleSpawn, Skin};

#[derive(EntityEvent)]
pub struct CommandSkinParticleSpawn {
    pub entity: Entity,
    pub hash: u32,
}

#[derive(EntityEvent)]
pub struct CommandSkinParticleDespawn {
    pub entity: Entity,
    pub hash: u32,
}

fn resolve_skin_resource_record<'a>(
    entity: Entity,
    input_hash: u32, // Assuming trigger.hash is u32, adjust according to actual type
    query_skin: &Query<&Skin>,
    assets_skin: &Assets<SkinCharacterDataProperties>,
    assets_resolver: &'a Assets<ResourceResolver>,
) -> Option<&'a u32> {
    // Assuming record is u32 or similar reference
    // 1. Get Skin component
    let skin = query_skin.get(entity).ok()?;

    // 2. Get character skin data
    let skin_data = assets_skin.load_hash(skin.key)?;

    // 3. Get resource resolver handle
    let resolver_handle = skin_data.m_resource_resolver?;

    // 4. Get resource resolver asset
    let resolver = assets_resolver.load_hash(resolver_handle)?;

    // 5. Find the specific resource record
    resolver.resource_map.as_ref()?.get(&input_hash)
}

pub fn on_command_character_particle_spawn(
    trigger: On<CommandSkinParticleSpawn>,
    res_assets_resource_resolver: Res<Assets<ResourceResolver>>,
    res_assets_skin_character_data_properties: Res<Assets<SkinCharacterDataProperties>>,
    mut commands: Commands,
    query: Query<&Skin>,
) {
    let entity = trigger.event_target();
    debug!("{entity} creating character particle effect {:x}", trigger.hash);

    // Use helper function to get record, if any step fails (returns None), return directly
    let Some(record) = resolve_skin_resource_record(
        entity,
        trigger.hash,
        &query,
        &res_assets_skin_character_data_properties,
        &res_assets_resource_resolver,
    ) else {
        return;
    };

    commands.trigger(CommandParticleSpawn {
        entity,
        hash: (*record).into(), // Dereference and convert
    });
}

pub fn on_command_character_particle_despawn(
    trigger: On<CommandSkinParticleDespawn>,
    res_assets_resource_resolver: Res<Assets<ResourceResolver>>,
    res_assets_skin_character_data_properties: Res<Assets<SkinCharacterDataProperties>>,
    mut commands: Commands,
    query: Query<&Skin>,
) {
    let entity = trigger.event_target();
    debug!("{entity} destroying character particle effect {:x}", trigger.hash);

    // Reuse the same logic
    let Some(record) = resolve_skin_resource_record(
        entity,
        trigger.hash,
        &query,
        &res_assets_skin_character_data_properties,
        &res_assets_resource_resolver,
    ) else {
        return;
    };

    commands.trigger(CommandParticleDespawn {
        entity,
        hash: (*record).into(),
    });
}
