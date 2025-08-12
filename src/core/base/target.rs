use bevy::prelude::*;

#[derive(Component)]
pub struct Target(pub Entity);

#[derive(Event, Debug)]
pub struct CommandTargetSet {
    pub target: Entity,
}

#[derive(Event, Debug)]
pub struct CommandTargetRemove;

pub struct PluginTarget;

impl Plugin for PluginTarget {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandTargetSet>();
        app.add_event::<CommandTargetRemove>();
        app.add_observer(command_set_target);
        app.add_observer(command_remove_target);
    }
}

fn command_set_target(trigger: Trigger<CommandTargetSet>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(Target(trigger.target));
}

fn command_remove_target(trigger: Trigger<CommandTargetRemove>, mut commands: Commands) {
    commands.entity(trigger.target()).remove::<Target>();
}
