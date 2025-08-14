use bevy::prelude::*;

#[derive(Component)]
pub struct Controller;

pub struct PluginController;

impl Plugin for PluginController {
    fn build(&self, _app: &mut App) {}
}
