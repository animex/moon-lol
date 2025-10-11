use bevy::prelude::*;

#[derive(Default)]
pub struct PluginLunge;

impl Plugin for PluginLunge {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Default)]
pub struct AbilityLunge;
