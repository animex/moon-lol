use bevy::prelude::*;
use league_core::SkinCharacterDataProperties;

use super::{on_command_spawn_animation, on_command_spawn_mesh};

#[derive(Default)]
pub struct PluginSkin;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct SkinScale(pub f32);

impl Plugin for PluginSkin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_skin_spawn);
        app.add_observer(on_command_spawn_mesh);
        app.add_observer(on_command_spawn_animation);
        app.add_systems(Update, update_skin_scale);
    }
}

#[derive(EntityEvent)]
pub struct CommandSkinSpawn {
    pub entity: Entity,
    pub skin_key: AssetId<SkinCharacterDataProperties>,
}

#[derive(EntityEvent)]
pub struct CommandSpawnMesh {
    pub entity: Entity,
    pub skin_key: AssetId<SkinCharacterDataProperties>,
}

#[derive(EntityEvent)]
pub struct CommandSpawnAnimation {
    pub entity: Entity,
    pub skin_key: AssetId<SkinCharacterDataProperties>,
}

fn update_skin_scale(mut query: Query<(&SkinScale, &mut Transform)>) {
    for (skin_scale, mut transform) in query.iter_mut() {
        transform.scale = Vec3::splat(skin_scale.0);
    }
}

fn on_command_skin_spawn(
    trigger: On<CommandSkinSpawn>,
    mut commands: Commands,
    res_assets_skin_character_data_properties: Res<Assets<SkinCharacterDataProperties>>,
) {
    let entity = trigger.event_target();

    let skin = res_assets_skin_character_data_properties
        .get(trigger.skin_key)
        .unwrap_or_else(|| panic!("Skin not found: {}", trigger.skin_key));

    commands.entity(entity).insert((
        Visibility::default(),
        SkinScale(
            skin.skin_mesh_properties
                .as_ref()
                .unwrap()
                .skin_scale
                .unwrap_or(1.0),
        ),
    ));

    commands.trigger(CommandSpawnMesh {
        entity,
        skin_key: trigger.skin_key.clone(),
    });
}
