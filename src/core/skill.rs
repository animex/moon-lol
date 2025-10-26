use bevy::prelude::*;
use bevy_behave::{
    prelude::{BehaveCtx, BehavePlugin, BehaveTree, BehaveTrigger, Tree},
    Behave,
};
use league_utils::hash_bin;
use lol_core::Team;

use crate::core::{
    rotate_to_direction, AAction, AnimationState, Attack, CommandAnimationPlay,
    CommandAttackAutoStart, CommandAttackAutoStop, CommandDamageCreate, CommandMovementStart,
    CommandNavigationTo, CommandParticleDespawn, CommandParticleSpawn, DamageType,
    EventMovementEnd, SkillEffectDash, SkillNavigationTo, State,
};

#[derive(Default)]
pub struct PluginSkill;

impl Plugin for PluginSkill {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandSkillStart>();

        app.add_plugins(BehavePlugin::default());

        app.add_observer(on_dash_end);
        app.add_observer(on_skill_cast);
        app.add_observer(on_skill_effect);
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
    pub effect: Option<Tree<Behave>>,
}

#[derive(Component)]
pub struct SkillEffectContext {
    pub point: Vec3,
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
) {
    let entity = trigger.target();
    let event = trigger.event();
    let skills = skills.get(entity).unwrap();
    let skill_entity = skills.0[event.index];
    let skill = q_skill.get(skill_entity).unwrap();

    if let Some(effect) = &skill.effect {
        commands.entity(entity).with_child((
            BehaveTree::new(effect.clone()),
            SkillEffectContext { point: event.point },
        ));
    }
}

#[derive(Component)]
pub struct SkillEffectBehaveCtx(pub BehaveCtx);

fn on_dash_end(
    trigger: Trigger<EventMovementEnd>,
    mut commands: Commands,
    q: Query<&SkillEffectBehaveCtx>,
) {
    let entity = trigger.target();
    let Ok(SkillEffectBehaveCtx(ctx)) = q.get(entity) else {
        return;
    };

    commands.trigger(ctx.success());
}

fn on_skill_effect(
    trigger: Trigger<BehaveTrigger<AAction>>,
    mut commands: Commands,
    q_attack: Query<&Attack>,
    mut q_transform: Query<&mut Transform>,
    q_skill_effect_ctx: Query<&SkillEffectContext>,
    q_target: Query<(Entity, &Team)>,
    q_team: Query<&Team>,
) {
    let event = trigger.event();
    let ctx = event.ctx();
    let entity = ctx.target_entity();
    let behave_entity = ctx.behave_entity();
    let skill_effect = event.inner();
    let skill_effect_ctx = q_skill_effect_ctx.get(behave_entity).ok();

    match skill_effect {
        AAction::Damage => {
            let mut min_distance = 300.;
            let mut target_bundle: Option<(Entity, &Transform)> = None;

            let team = q_team.get(entity).unwrap();
            let transform = q_transform.get(entity).unwrap();

            for (target, target_team) in q_target.iter() {
                if target_team == team {
                    continue;
                }

                let Ok(target_transform) = q_transform.get(target) else {
                    continue;
                };

                let distance = target_transform.translation.distance(transform.translation);
                if distance < min_distance {
                    min_distance = distance;
                    target_bundle = Some((target, target_transform));
                }
            }

            let Some((target, target_transform)) = target_bundle else {
                commands.trigger(ctx.failure());
                return;
            };

            let direction = (target_transform.translation - transform.translation).xz();
            let mut transform = q_transform.get_mut(entity).unwrap();
            rotate_to_direction(&mut transform, direction);

            commands.entity(target).trigger(CommandDamageCreate {
                source: entity,
                damage_type: DamageType::Physical,
                amount: 100.0,
            });
            commands.trigger(ctx.success());
        }
        AAction::Dash(skill_effect_dash) => match skill_effect_dash {
            SkillEffectDash::Fixed(_) => todo!(),
            SkillEffectDash::Pointer { max, speed } => {
                let skill_effect_ctx = skill_effect_ctx.unwrap();
                let transform = q_transform.get(entity).unwrap();
                let vector = skill_effect_ctx.point - transform.translation;
                let distance = vector.length();

                let destination = if distance < *max {
                    skill_effect_ctx.point
                } else {
                    let direction = vector.normalize();
                    let dash_point = transform.translation + direction * *max;
                    dash_point
                };

                commands
                    .entity(entity)
                    .insert(State::Dashing)
                    .insert(SkillEffectBehaveCtx(ctx.clone()))
                    .trigger(CommandMovementStart {
                        priority: 100,
                        path: vec![destination.xz()],
                        speed: Some(*speed),
                    });
            }
        },
        AAction::Missile => {
            println!("发送导弹");
            commands.trigger(ctx.success());
        }
        AAction::Animation(skill_effect_animation) => {
            commands.entity(entity).trigger(CommandAnimationPlay {
                hash: skill_effect_animation.hash,
                repeat: false,
                ..default()
            });
            commands.trigger(ctx.success());
        }
        AAction::Particle(skill_effect_particle) => {
            commands.entity(entity).trigger(CommandParticleSpawn {
                particle: skill_effect_particle.hash,
            });
            commands.trigger(ctx.success());
        }
        AAction::DespawnParticle(skill_effect_particle) => {
            commands.entity(entity).trigger(CommandParticleDespawn {
                particle: *skill_effect_particle,
            });
            commands.trigger(ctx.success());
        }
        AAction::NavigationTo(SkillNavigationTo { target }) => {
            commands.entity(entity).trigger(CommandAttackAutoStop);
            commands.entity(entity).trigger(CommandNavigationTo {
                priority: 0,
                target: *target,
            });
            commands.entity(entity).trigger(CommandAnimationPlay {
                hash: hash_bin("Run"),
                repeat: true,
                ..default()
            });
            commands.trigger(ctx.success());
        }
        AAction::AutoAttack(skill_auto_attack) => {
            let attack = q_attack.get(entity).unwrap();
            commands.entity(entity).trigger(CommandAnimationPlay {
                hash: hash_bin("Attack1"),
                repeat: true,
                duration: Some(attack.total_duration_secs()),
            });
            commands.entity(entity).trigger(CommandAttackAutoStart {
                target: skill_auto_attack.target,
            });
            commands.trigger(ctx.success());
        }
    }
}
