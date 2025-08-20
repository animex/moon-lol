use bevy::prelude::*;

use crate::core::{
    CommandAttackAutoStart, CommandAttackStop, CommandMovementStop, CommandNavigationTo,
    EventAttackStart, EventMovementStart,
};

#[derive(Default)]
pub struct PluginBehavior;

impl Plugin for PluginBehavior {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandBehaviorAttack>();
        app.add_event::<CommandBehaviorMoveTo>();

        app.add_observer(on_movement_start);
        app.add_observer(on_attack_cast);

        app.add_observer(command_attack);
        app.add_observer(command_move_to);
    }
}

#[derive(Event)]
pub struct CommandBehaviorAttack {
    pub target: Entity,
}

#[derive(Event)]
pub struct CommandBehaviorMoveTo(pub Vec2);

fn on_movement_start(trigger: Trigger<EventMovementStart>, mut commands: Commands) {
    commands.trigger_targets(CommandAttackStop, trigger.target());
}

fn on_attack_cast(trigger: Trigger<EventAttackStart>, mut commands: Commands) {
    commands.trigger_targets(CommandMovementStop, trigger.target());
}

fn command_attack(trigger: Trigger<CommandBehaviorAttack>, mut commands: Commands) {
    commands.trigger_targets(
        CommandAttackAutoStart {
            target: trigger.target,
        },
        trigger.target(),
    );
}

fn command_move_to(trigger: Trigger<CommandBehaviorMoveTo>, mut commands: Commands) {
    commands.trigger_targets(CommandAttackStop, trigger.target());
    commands.trigger_targets(CommandNavigationTo(trigger.0), trigger.target());
}
