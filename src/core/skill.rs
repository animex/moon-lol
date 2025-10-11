use bevy::prelude::*;

use crate::core::{
    CommandAnimationPlay, CommandMovementStart, SkillEffect, SkillEffectDash, SkillEffectSequence,
    State,
};

#[derive(Default)]
pub struct PluginSkill;

impl Plugin for PluginSkill {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandSkillStart>();

        app.add_observer(on_skill_cast);
    }
}

#[derive(Component, Debug)]
#[relationship(relationship_target = Skills)]
pub struct SkillOf(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = SkillOf)]
pub struct Skills(Vec<Entity>);

#[derive(Component, Default)]
pub struct CoolDown {
    pub timer: Timer,
}

#[derive(Component)]
#[require(CoolDown)]
pub struct Skill {
    pub effect: Option<SkillEffectSequence>,
}

#[derive(Event)]
pub struct CommandSkillStart {
    pub index: usize,
    pub point: Vec3,
}

fn on_skill_cast(
    trigger: Trigger<CommandSkillStart>,
    mut commands: Commands,
    skills: Query<&Skills>,
    q_skill: Query<&Skill>,
    q_transform: Query<&Transform>,
) {
    let entity = trigger.target();
    let transform = q_transform.get(entity).unwrap();
    let event = trigger.event();
    let skills = skills.get(entity).unwrap();
    let skill_entity = skills.0[event.index];
    let skill = q_skill.get(skill_entity).unwrap();

    if let Some(effect) = &skill.effect {
        apply_effect(effect, &mut commands, entity, event, transform);
    }
}

fn apply_effect(
    effect: &SkillEffectSequence,
    commands: &mut Commands,
    entity: Entity,
    event: &CommandSkillStart,
    transform: &Transform,
) {
    match effect {
        SkillEffectSequence::Single(skill_effect) => match skill_effect {
            SkillEffect::Damage => todo!(),
            SkillEffect::Dash(skill_effect_dash) => match skill_effect_dash {
                SkillEffectDash::Fixed(_) => todo!(),
                SkillEffectDash::Pointer { max, speed } => {
                    let vector = event.point - transform.translation;
                    let distance = vector.length();

                    let destination = if distance < *max {
                        event.point
                    } else {
                        let direction = vector.normalize();
                        let dash_point = transform.translation + direction * *max;
                        dash_point
                    };

                    commands
                        .entity(entity)
                        .insert(State::Dashing)
                        .trigger(CommandMovementStart {
                            path: vec![destination.xz()],
                            speed: Some(*speed),
                        });
                }
            },
            SkillEffect::ApplyStatus => todo!(),
            SkillEffect::RemoveStatus => todo!(),
            SkillEffect::EnhanceAttack => todo!(),
            SkillEffect::SpawnArea => todo!(),
            SkillEffect::CooldownReduction => todo!(),
            SkillEffect::Conditional => todo!(),
            SkillEffect::Missile => {
                println!("发送导弹");
            }
            SkillEffect::Animation(skill_effect_animation) => {
                commands.entity(entity).trigger(CommandAnimationPlay {
                    hash: skill_effect_animation.hash,
                });
            }
        },
        SkillEffectSequence::Serial(skill_effect_sequences) => {
            for skill_effect_sequence in skill_effect_sequences {
                apply_effect(skill_effect_sequence, commands, entity, event, transform);
            }
        }
        SkillEffectSequence::Parallel(skill_effect_sequences) => {
            for skill_effect_sequence in skill_effect_sequences {
                apply_effect(skill_effect_sequence, commands, entity, event, transform);
            }
        }
        SkillEffectSequence::Delay(duration) => todo!(),
    }
}
