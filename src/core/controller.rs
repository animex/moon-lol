use bevy::prelude::*;

#[derive(Component)]
pub struct Controller;

#[derive(Default)]
pub struct PluginController;

impl Plugin for PluginController {
    fn build(&self, _app: &mut App) {}
}
