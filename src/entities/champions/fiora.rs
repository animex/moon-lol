use bevy::prelude::*;
use bevy_behave::{behave, Behave};
use league_utils::hash_bin;

use crate::{
    abilities::AbilityDuelistsDance,
    core::{
        Attack, Bounding, Health, Movement, Skill, SkillEffect, SkillEffectAnimation,
        SkillEffectDash, SkillEffectParticle, SkillOf,
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
                effect: Some(behave! {
                    Behave::Sequence => {
                        Behave::trigger(
                            SkillEffect::Animation(SkillEffectAnimation { hash: hash_bin("Spell1") })
                        ),
                        Behave::trigger(
                            SkillEffect::Dash(SkillEffectDash::Pointer { speed: 1000., max: 300. }),
                        ),
                        Behave::trigger(
                            SkillEffect::Particle(SkillEffectParticle { hash: hash_bin("Fiora_Q_Slash_Cas") }),
                        ),
                        Behave::trigger(SkillEffect::Damage),
                    }
                }),
            },
        ))
        .with_related::<SkillOf>((
            Skill {
                effect: Some(behave! {
                    Behave::Sequence => {
                        Behave::trigger(
                            SkillEffect::Particle(SkillEffectParticle { hash: hash_bin("Fiora_W_Telegraph_Blue") }),
                        ),
                        Behave::trigger(
                            SkillEffect::Animation(SkillEffectAnimation { hash: hash_bin("Spell2_In") })
                        ),
                        Behave::trigger(
                            SkillEffect::Particle(SkillEffectParticle { hash: hash_bin("Fiora_W_Cas") }),
                        ),
                        Behave::Wait(0.5),
                        Behave::trigger(
                            SkillEffect::Animation(SkillEffectAnimation { hash: hash_bin("Spell2") })
                        ),
                        Behave::trigger(SkillEffect::Damage),
                    }
                }),
            },
        ))
        .with_related::<SkillOf>((
            Skill {
                effect: Some(behave! {
                    Behave::Sequence => {
                        Behave::trigger(
                            SkillEffect::Animation(SkillEffectAnimation { hash: hash_bin("Spell2") })
                        ),
                        Behave::Wait(1.),
                        Behave::trigger(SkillEffect::Damage),
                    }
                }),
            },
        ))
        .with_related::<SkillOf>((
            Skill {
                effect: Some(behave! {
                    Behave::Sequence => {
                        Behave::trigger(
                            SkillEffect::Animation(SkillEffectAnimation { hash: hash_bin("Spell2") })
                        ),
                        Behave::Wait(1.),
                        Behave::trigger(SkillEffect::Damage),
                    }
                }),
            },
        ))
        .log_components();
}
