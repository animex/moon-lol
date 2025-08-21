use bevy::prelude::*;

use crate::core::{
    Attack, AttackState, AttackStatus, CommandAttackStart, CommandAttackStop, CommandNavigationTo,
};
use bevy::prelude::Res;
use bevy::time::{Stopwatch, Time};

#[derive(Default)]
pub struct PluginAttackAuto;

impl Plugin for PluginAttackAuto {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandAttackAutoStart>();
        app.add_observer(on_command_attack_auto_start);

        app.add_event::<CommandAttackAutoStop>();
        app.add_observer(on_command_attack_auto_stop);

        app.add_systems(FixedUpdate, update_attack_auto);
    }
}

#[derive(Component)]
pub struct AttackAuto {
    pub target: Entity,
    pub timer: Stopwatch,
}

#[derive(Event)]
pub struct CommandAttackAutoStart {
    pub target: Entity,
}

#[derive(Event)]
pub struct CommandAttackAutoStop;

fn on_command_attack_auto_start(trigger: Trigger<CommandAttackAutoStart>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(AttackAuto {
        target: trigger.target,
        timer: Stopwatch::new(),
    });
}

fn on_command_attack_auto_stop(trigger: Trigger<CommandAttackAutoStop>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .remove::<AttackAuto>()
        .trigger(CommandAttackStop);
}

fn update_attack_auto(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackAuto, &Attack)>,
    q_attack_state: Query<&AttackState>,
    q_transform: Query<&Transform>,
    time: Res<Time<Fixed>>,
) {
    for (entity, mut attack_auto, attack) in query.iter_mut() {
        attack_auto.timer.tick(time.delta());

        let Ok(transform) = q_transform.get(entity) else {
            continue;
        };

        let Ok(target_transform) = q_transform.get(attack_auto.target) else {
            continue;
        };

        if let Ok(attack_state) = q_attack_state.get(entity) {
            if matches!(attack_state.status, AttackStatus::Windup { .. }) {
                continue;
            }
        };

        if transform.translation.distance(target_transform.translation) > attack.range {
            if attack_auto.timer.elapsed_secs() >= 1.0 {
                commands
                    .entity(entity)
                    .trigger(CommandNavigationTo(target_transform.translation.xz()));
                attack_auto.timer.reset();
            }
        } else {
            commands.entity(entity).trigger(CommandAttackStart {
                target: attack_auto.target,
            });
        }
    }
}
