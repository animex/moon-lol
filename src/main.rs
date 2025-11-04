use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};

use lol_config::ConfigGame;

use moon_lol::abilities::PluginAbilities;
use moon_lol::core::{spawn_skin_entity, Controller, Focus, Health, Movement};
use moon_lol::entities::PluginBarrack;
use moon_lol::{core::PluginCore, entities::PluginEntities, logging::PluginLogging};

fn main() {
    App::new()
        .add_plugins((
            PluginLogging,
            DefaultPlugins
                .build()
                .disable::<bevy::log::LogPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "moon-lol".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                }),
            PluginCore,
            PluginEntities.build().disable::<PluginBarrack>(),
            PluginAbilities,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
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

        commands.entity(entity).insert((
            team.clone(),
            Controller::default(),
            Focus,
            Movement { speed: 325.0 },
            Health {
                value: 600.0,
                max: 600.0,
            },
        ));
    }
}
