use bevy::prelude::*;

use crate::entities::champion::Champion;

#[derive(Component)]
#[require(Champion)]
pub struct Fiora;

#[derive(Default)]
pub struct PluginFiora;

impl Plugin for PluginFiora {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fiora);
    }
}

fn setup_fiora(mut commands: Commands) {
    commands.spawn(Fiora);
}
