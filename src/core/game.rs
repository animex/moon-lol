use bevy::prelude::*;
use lol_config::ConfigGame;

use crate::core::{spawn_skin_entity, Controller, Focus, Health, Movement};

#[derive(Default)]
pub struct PluginGame;

impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub fn setup(
    mut commands: Commands,
    mut res_animation_graph: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
    config_game: Res<ConfigGame>,
) {
    for (mut transform, team, skin) in config_game.legends.iter() {
        transform.translation = Vec3::new(1000.0, 30.0, -1000.0);

        let entity = spawn_skin_entity(
            &mut commands,
            &mut res_animation_graph,
            &asset_server,
            transform,
            &skin,
        );

        commands
            .entity(entity)
            .insert((
                team.clone(),
                Controller,
                Focus,
                Movement { speed: 325.0 },
                Health {
                    value: 600.0,
                    max: 600.0,
                },
            ))
            .log_components();
    }
}
