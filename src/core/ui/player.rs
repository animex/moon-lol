use bevy::prelude::*;

use crate::{
    spawn_ui_element, AbilityResource, CommandUpdateUIElement, Controller, Health, Level, NodeType,
    ResourceCache, SizeType, UIElementEntity,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct HealthFade {
    pub value: f32,
    pub max: f32,
}

pub fn update_level(
    mut commands: Commands,
    res_ui_element_entity: Res<UIElementEntity>,
    q_level: Query<&Level, With<Controller>>,
) {
    let Some(entity) = res_ui_element_entity.map.get("ClientStates/Gameplay/UX/LoL/PlayerFrame/UIBase/Player_Frame_Root/Player_Frame/PlayerExp_BarTexture") else {
        return;
    };

    let Ok(level) = q_level.single() else {
        return;
    };

    commands.trigger(CommandUpdateUIElement {
        entity: *entity,
        size_type: SizeType::Height,
        value: level.experience as f32 / level.experience_to_next_level as f32,
        node_type: NodeType::Parent,
    });
}

pub fn update_player_health(
    mut commands: Commands,
    res_ui_element_entity: Res<UIElementEntity>,
    q_health: Query<&Health, With<Controller>>,
) {
    let value = if let Ok(health) = q_health.single() {
        health.value as f32 / health.max as f32
    } else {
        0.0
    };

    // Update Green Bar (Immediate)
    let Some(entity) = res_ui_element_entity.map.get("ClientStates/Gameplay/UX/LoL/PlayerFrame/UIBase/Player_Frame_Root/PlayerResourceBars/PlayerHPMeter/PlayerHP_BarTextureGreen") else {
        return;
    };

    commands.trigger(CommandUpdateUIElement {
        entity: *entity,
        size_type: SizeType::Width,
        value,
        node_type: NodeType::Parent,
    });
}

pub fn update_player_health_fade(
    mut commands: Commands,
    time: Res<Time>,
    res_ui_element_entity: Res<UIElementEntity>,
    q_health: Query<&Health, With<Controller>>,
    mut q_health_fade: Query<&mut HealthFade>,
) {
    let health_data = q_health.single().ok();

    // Update Fade Bar (Animated)
    let Some(entity) = res_ui_element_entity.map.get("ClientStates/Gameplay/UX/LoL/PlayerFrame/UIBase/Player_Frame_Root/PlayerResourceBars/PlayerHPMeter/PlayerHP_BarFade") else {
        return;
    };

    let mut final_value;
    let final_max;

    if let Ok(mut health_fade) = q_health_fade.get_mut(*entity) {
        let (target_val, current_max) = if let Some(health) = health_data {
            health_fade.max = health.max;
            (health.value, health.max)
        } else {
            (0.0, health_fade.max)
        };

        if target_val < health_fade.value {
            health_fade.value -= 100.0 * time.delta_secs() * 2.0;
            if health_fade.value < target_val {
                health_fade.value = target_val;
            }
        } else {
            health_fade.value = target_val;
        }
        final_value = health_fade.value;
        final_max = current_max;
    } else {
        if let Some(health) = health_data {
            commands.entity(*entity).insert(HealthFade {
                value: health.value,
                max: health.max,
            });
            final_value = health.value;
            final_max = health.max;
        } else {
            commands.entity(*entity).insert(HealthFade {
                value: 0.0,
                max: 1.0,
            });
            final_value = 0.0;
            final_max = 1.0;
        }
    }

    if final_max == 0.0 {
        final_value = 0.0;
    } else {
        final_value /= final_max;
    }

    commands.trigger(CommandUpdateUIElement {
        entity: *entity,
        size_type: SizeType::Width,
        value: final_value,
        node_type: NodeType::Parent,
    });
}

pub fn update_player_ability_resource(
    mut commands: Commands,
    mut res_ui_element_entity: ResMut<UIElementEntity>,
    res_asset_server: Res<AssetServer>,
    res_resource_cache: Res<ResourceCache>,
    q_ability_resource: Query<&AbilityResource, With<Controller>>,
) {
    let key = "ClientStates/Gameplay/UX/LoL/PlayerFrame/UIBase/Player_Frame_Root/PlayerResourceBars/PlayerPARMeter/PlayerPar_BarTextureBlue";
    let entity = if let Some(entity) = res_ui_element_entity.map.get(key) {
        *entity
    } else {
        let ui = res_resource_cache.ui_elements.get(key).unwrap();
        let entity = spawn_ui_element(&mut commands, &res_asset_server, ui).unwrap();
        res_ui_element_entity.map.insert(key.to_owned(), entity);
        entity
    };

    let Ok(ability_resource) = q_ability_resource.single() else {
        return;
    };

    commands.trigger(CommandUpdateUIElement {
        entity,
        size_type: SizeType::Width,
        value: ability_resource.value as f32 / ability_resource.max as f32,
        node_type: NodeType::Parent,
    });
}
