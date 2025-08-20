use bevy::prelude::*;

use crate::core::{Attack, CommandAttackStart, CommandAttackStop, CommandNavigationTo};

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
    pub found_path: bool,
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
        found_path: false,
    });
}

fn on_command_attack_auto_stop(trigger: Trigger<CommandAttackAutoStop>, mut commands: Commands) {
    commands.entity(trigger.target()).trigger(CommandAttackStop);
}

fn update_attack_auto(
    mut commands: Commands,
    query: Query<(Entity, &AttackAuto, &Attack)>,
    q_transform: Query<&Transform>,
) {
    for (entity, attack_auto, attack) in query.iter() {
        let Ok(transform) = q_transform.get(entity) else {
            continue;
        };

        let Ok(target_transform) = q_transform.get(attack_auto.target) else {
            continue;
        };

        if transform.translation.distance(target_transform.translation) > attack.range {
            if !attack_auto.found_path {
                commands
                    .entity(entity)
                    .trigger(CommandNavigationTo(target_transform.translation.xz()));
            }
        } else {
            commands.entity(entity).trigger(CommandAttackStart {
                target: attack_auto.target,
            });
        }
    }
}
