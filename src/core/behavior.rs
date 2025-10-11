use bevy::prelude::*;

use crate::core::{
    Action, CommandAttackAutoStart, CommandAttackAutoStop, CommandMovementStop,
    CommandNavigationTo, CommandSkillStart, EventAttackStart, State,
};

#[derive(Default)]
pub struct PluginBehavior;

impl Plugin for PluginBehavior {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandBehavior>();

        app.add_observer(on_attack_cast);

        app.add_observer(on_command_action);
    }
}

#[derive(Event)]
pub struct CommandBehavior {
    pub action: Action,
}

fn on_attack_cast(trigger: Trigger<EventAttackStart>, mut commands: Commands) {
    commands.trigger_targets(CommandMovementStop, trigger.target());
}

fn on_command_action(
    trigger: Trigger<CommandBehavior>,
    mut commands: Commands,
    mut q_state: Query<&mut State>,
) {
    let entity = trigger.target();
    let mut state = q_state.get_mut(entity).unwrap();

    match trigger.event().action {
        Action::Attack(target) => {
            commands
                .entity(entity)
                .trigger(CommandAttackAutoStart { target });
        }
        Action::Move(target) => {
            if *state == State::Dashing {
                return;
            }

            *state = State::Moving;

            commands.trigger_targets(CommandAttackAutoStop, trigger.target());
            commands.trigger_targets(CommandNavigationTo(target), trigger.target());
        }
        Action::Skill { index, point } => {
            commands.trigger_targets(CommandAttackAutoStop, trigger.target());
            commands.trigger_targets(CommandSkillStart { index, point }, trigger.target());
        }
        Action::Stop => todo!(),
    }
}
