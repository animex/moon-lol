use bevy::prelude::*;

use crate::{
    Attack, AttackState, AttackStatus, CommandAttackStart, CommandAttackStop, CommandRunStart,
    CommandRunStop, RunTarget,
};

#[derive(Default)]
pub struct PluginAttackAuto;

impl Plugin for PluginAttackAuto {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_attack_auto_start);
        app.add_observer(on_command_attack_auto_stop);

        app.add_systems(FixedPreUpdate, update_attack_auto);
    }
}

#[derive(Component)]
pub struct AttackAuto {
    pub target: Entity,
}

#[derive(EntityEvent)]
pub struct CommandAttackAutoStart {
    pub entity: Entity,
    pub target: Entity,
}

#[derive(EntityEvent)]
pub struct CommandAttackAutoStop {
    pub entity: Entity,
}

fn on_command_attack_auto_start(
    trigger: On<CommandAttackAutoStart>,
    mut commands: Commands,
    q_transform: Query<&Transform>,
    q_attack: Query<&Attack>,
) {
    let entity = trigger.event_target();
    let target = trigger.target;

    let mut attack_auto = AttackAuto { target };

    let Ok(transform) = q_transform.get(entity) else {
        return;
    };

    let Ok(target_transform) = q_transform.get(attack_auto.target) else {
        return;
    };

    let Ok(attack) = q_attack.get(entity) else {
        return;
    };

    check_and_action(
        &mut commands,
        trigger.event_target(),
        attack_auto.target,
        &mut attack_auto,
        transform.translation.xz(),
        target_transform.translation.xz(),
        attack.range,
    );

    commands.entity(entity).insert(attack_auto);
}

fn on_command_attack_auto_stop(trigger: On<CommandAttackAutoStop>, mut commands: Commands) {
    let entity = trigger.event_target();
    commands.entity(entity).remove::<AttackAuto>();
    commands.trigger(CommandAttackStop { entity });
}

fn update_attack_auto(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackAuto, &Attack)>,
    q_attack_state: Query<&AttackState>,
    q_transform: Query<&Transform>,
) {
    for (entity, mut attack_auto, attack) in query.iter_mut() {
        if let Ok(attack_state) = q_attack_state.get(entity) {
            if matches!(attack_state.status, AttackStatus::Windup { .. }) {
                continue;
            }
        };

        let Ok(transform) = q_transform.get(entity) else {
            continue;
        };

        let Ok(target_transform) = q_transform.get(attack_auto.target) else {
            continue;
        };

        check_and_action(
            &mut commands,
            entity,
            attack_auto.target,
            &mut attack_auto,
            transform.translation.xz(),
            target_transform.translation.xz(),
            attack.range,
        );
    }
}

fn check_and_action(
    commands: &mut Commands,
    entity: Entity,
    target: Entity,
    attack_auto: &mut AttackAuto,
    position: Vec2,
    target_position: Vec2,
    range: f32,
) {
    if position.distance(target_position) > range {
        commands.trigger(CommandRunStart {
            entity,
            target: RunTarget::Target(target),
        });

        debug!("{} 停止攻击：离开攻击范围", entity);
        commands.trigger(CommandAttackStop { entity });
    } else {
        commands.trigger(CommandRunStop { entity });

        debug!("{} 开始攻击：进入攻击范围", entity);
        commands.trigger(CommandAttackStart {
            entity,
            target: attack_auto.target,
        });
    }
}
