use bevy::prelude::*;
use league_utils::hash_bin;

use crate::{
    abilities::{AbilityDuelistsDance, AbilityLunge},
    core::{
        Attack, Bounding, Health, Movement, Skill, SkillEffect, SkillEffectAnimation,
        SkillEffectDash, SkillEffectSequence, SkillOf,
    },
    entities::champion::Champion,
};

#[derive(Component)]
#[require(Champion)]
pub struct Fiora;

#[derive(Default)]
pub struct PluginFiora;

impl Plugin for PluginFiora {
    fn build(&self, _app: &mut App) {}
}

pub fn spawn_fiora(commands: &mut Commands, entity: Entity) {
    commands
        .entity(entity)
        .insert((
            Movement { speed: 325.0 },
            Health {
                value: 600.0,
                max: 600.0,
            },
            Attack::new(150.0, 0.2, 1.45),
            Fiora,
            Bounding {
                radius: 35.0,
                height: 300.0,
            },
        ))
        .with_related::<SkillOf>((Skill { effect: None }, AbilityDuelistsDance))
        .with_related::<SkillOf>((
            Skill {
                effect: Some(SkillEffectSequence::Serial(vec![
                    SkillEffectSequence::Parallel(vec![
                        SkillEffectSequence::Single(SkillEffect::Dash(SkillEffectDash::Pointer {
                            speed: 1000.0,
                            max: 300.0,
                        })),
                        SkillEffectSequence::Single(SkillEffect::Animation(SkillEffectAnimation {
                            hash: hash_bin("Spell1"),
                        })),
                    ]),
                    SkillEffectSequence::Single(SkillEffect::Missile),
                ])),
            },
            AbilityLunge,
        ))
        .log_components();
}
