use bevy::prelude::*;
use league_core::SpellObject;
use lol_config::LoadHashKeyTrait;

use crate::{
    CommandDespawnButton, CommandSkillLevelUp, CommandSpawnButton, Controller, Level, PassiveSkill,
    ResourceCache, Skill, SkillPoints, Skills, UIElementEntity, UIState,
};

#[derive(Default)]
pub struct PluginUISkill;

impl Plugin for PluginUISkill {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkillLevelUpButton>();
        app.add_systems(
            Update,
            (
                update_skill_level_up_button.run_if(in_state(UIState::Loaded)),
                update_player_skill_icon.run_if(in_state(UIState::Loaded).and(run_once)),
            ),
        );
    }
}

#[derive(Resource, Default)]
struct SkillLevelUpButton {
    pub entities: [Option<Entity>; 4],
}

fn update_player_skill_icon(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut q_image_node: Query<&mut ImageNode>,
    mut res_resource_cache: ResMut<ResourceCache>,
    q_children: Query<&Children>,
    q_skill: Query<&Skill>,
    q_skills: Query<&Skills, With<Controller>>,
    q_passive_skill: Query<&PassiveSkill, With<Controller>>,
    res_assets_spell_object: Res<Assets<SpellObject>>,
    res_ui_element_entity: Res<UIElementEntity>,
) {
    let Ok(passive_skill) = q_passive_skill.single() else {
        debug!("Controller passive skill not found");
        return;
    };

    let Ok(skills) = q_skills.single() else {
        debug!("Controller skill list not found");
        return;
    };

    for (index, skill) in passive_skill.iter().chain(skills.iter()).enumerate() {
        let key = if index == 0 {
            "ClientStates/Gameplay/UX/LoL/PlayerFrame/UIBase/Player_Frame_Root/PlayerSpells/Passive/Passive_IconLoc".to_string()
        } else {
            format!("ClientStates/Gameplay/UX/LoL/PlayerFrame/UIBase/Player_Frame_Root/PlayerSpells/Ability{0}/Ability{0}_IconLoc", index - 1)
        };

        let Some(&entity) = res_ui_element_entity.get_by_string(&key) else {
            debug!("Skill icon UI element not found {}", key);
            continue;
        };

        let Ok(children) = q_children.get(entity) else {
            continue;
        };

        let &child = children.get(0).unwrap();
        let mut image_node = q_image_node.get_mut(child).unwrap();
        if image_node.rect.is_none() {
            debug!("Skill icon {} rect is empty", index);
            continue;
        }

        let Ok(skill) = q_skill.get(skill) else {
            debug!("Skill component not found for skill entity {}", index);
            continue;
        };

        let spell = res_assets_spell_object
            .load_hash(skill.key_spell_object)
            .unwrap();

        let icon_name = spell
            .m_spell
            .as_ref()
            .unwrap()
            .m_img_icon_name
            .as_ref()
            .unwrap()
            .get(0)
            .unwrap()
            .clone();

        image_node.image = res_resource_cache.get_image(&asset_server, &icon_name);
        image_node.rect = None;

        commands.entity(entity).insert(Visibility::Visible);
    }
}

fn update_skill_level_up_button(
    mut commands: Commands,
    q_skill_points: Query<(Entity, &Level, &SkillPoints, &Skills), With<Controller>>,
    mut res_skill_level_up_button: ResMut<SkillLevelUpButton>,
    q_skill: Query<&Skill>,
) {
    let Ok((entity, level, skill_points, skills)) = q_skill_points.single() else {
        debug!("Controller skill points info not found");
        return;
    };

    for (index, skill_entity) in skills.iter().skip(1).enumerate() {
        let key_str = format!(
            "ClientStates/Gameplay/UX/LoL/PlayerFrame/UIBase/Player_Frame_Root/LevelUp/LevelUp{}_Button",
            index
        );
        let key = league_utils::hash_bin(&key_str);

        // If no skill points, or skill is max level, or doesn't meet upgrade conditions, hide/destroy button
        let mut should_show = skill_points.0 > 0;

        if should_show {
            if let Ok(skill) = q_skill.get(skill_entity) {
                // Level 1 can only upgrade Q W E, R available at level 6, before level 6 each skill max 3 points
                if level.value < 6 {
                    if index == 3 {
                        should_show = false;
                    } else if skill.level >= 3 {
                        should_show = false;
                    }
                }
            }
        }

        if should_show {
            if res_skill_level_up_button.entities[index].is_some() {
                continue;
            }

            debug!("Spawning skill upgrade button entity {} index {}", entity, index);
            let entity_button = commands
                .spawn_empty()
                .observe(move |_event: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(CommandSkillLevelUp { entity, index });
                })
                .id();
            res_skill_level_up_button.entities[index] = Some(entity_button);
            commands.trigger(CommandSpawnButton {
                key: key.into(),
                entity: Some(entity_button),
            });
        } else {
            if let Some(entity_button) = res_skill_level_up_button.entities[index] {
                debug!("Destroying skill upgrade button entity {} index {}", entity, index);
                res_skill_level_up_button.entities[index] = None;
                commands.trigger(CommandDespawnButton {
                    entity: entity_button,
                });
            }
        }
    }
    debug!("Skill upgrade button update complete");
}
