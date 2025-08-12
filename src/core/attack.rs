use bevy::prelude::*;

use crate::core::Target;

pub struct PluginAttack;

impl Plugin for PluginAttack {
    fn build(&self, app: &mut App) {
        app.add_event::<EventAttackLock>();
        app.add_event::<EventAttackAttack>();
        app.add_event::<EventAttackRecover>();
        app.add_event::<EventAttackTargetInRange>();
        app.add_event::<CommandAttackLock>();
        app.add_observer(on_command_attack_lock);
    }
}

#[derive(Component)]
#[require(AttackState)]
pub struct Attack {
    pub range: f32,
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct AttackState {
    pub target: Option<Entity>,
    pub last_lock_time: Option<f32>,
}

#[derive(Event, Debug)]
pub struct CommandAttackLock;

#[derive(Event, Debug)]
pub struct EventAttackLock;

#[derive(Event, Debug)]
pub struct EventAttackAttack {
    pub target: Entity,
}

#[derive(Event, Debug)]
pub struct EventAttackRecover;

#[derive(Event, Debug)]
pub struct EventAttackTargetInRange {
    pub target: Entity,
}

fn on_command_attack_lock(
    trigger: Trigger<CommandAttackLock>,
    mut commands: Commands,
    mut query: Query<(&mut AttackState, &Target)>,
    time: Res<Time<Fixed>>,
) {
    let entity = trigger.target();

    if let Ok((mut attack_state, target)) = query.get_mut(entity) {
        if attack_state.last_lock_time.is_none() {
            attack_state.last_lock_time = Some(time.elapsed_secs());
            attack_state.target = Some(target.0);

            commands.trigger_targets(EventAttackLock, entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{CommandCommandAttack, PluginCommand, PluginTarget};

    use super::*;

    #[test]
    fn test_command_attack_lock_to_locking_success() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(PluginTarget);
        app.add_plugins(PluginAttack);
        app.add_plugins(PluginCommand);

        let attacker = app
            .world_mut()
            .spawn((
                Attack {
                    range: 100.0,
                    speed: 1.0,
                },
                AttackState::default(),
            ))
            .id();

        let target = app.world_mut().spawn_empty().id();

        {
            let attack_state = app.world().get::<AttackState>(attacker).unwrap();
            assert_eq!(attack_state.last_lock_time, None);
            assert_eq!(attack_state.target, None);
        }

        app.world_mut()
            .trigger_targets(CommandCommandAttack { target }, attacker);

        app.update();

        {
            let attack_state = app.world().get::<AttackState>(attacker).unwrap();
            assert!(attack_state.last_lock_time.is_some());
            assert_eq!(attack_state.target, Some(target));
        }
    }
}
