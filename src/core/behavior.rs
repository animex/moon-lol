use bevy::prelude::*;
use bevy_behave::{behave, prelude::BehaveTree, Behave};
use league_utils::hash_bin;

use crate::core::{
    AAction, Action, CommandAnimationPlay, CommandAttackAutoStop, CommandMovementStop,
    CommandSkillStart, EventAttackStart, SkillAutoAttack, SkillNavigationTo,
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

fn on_command_action(trigger: Trigger<CommandBehavior>, mut commands: Commands) {
    let entity = trigger.target();

    match trigger.event().action {
        Action::Attack(target) => {
            commands
                .entity(entity)
                .with_child((BehaveTree::new(behave! {
                    Behave::trigger(
                        AAction::AutoAttack(SkillAutoAttack { target })
                    ),
                }),));
        }
        Action::Move(target) => {
            commands
                .entity(entity)
                .with_child((BehaveTree::new(behave! {
                    Behave::trigger(
                        AAction::NavigationTo(SkillNavigationTo { target })
                    ),
                }),));
        }
        Action::Skill { index, point } => {
            commands.trigger_targets(CommandAttackAutoStop, trigger.target());
            commands.trigger_targets(CommandSkillStart { index, point }, trigger.target());
        }
        Action::Stop => {
            commands.trigger_targets(CommandAttackAutoStop, trigger.target());
            commands.trigger_targets(CommandMovementStop, trigger.target());
            commands.trigger_targets(
                CommandAnimationPlay {
                    hash: hash_bin("Idle1"),
                    repeat: true,
                    ..default()
                },
                trigger.target(),
            );
        }
    }
}
