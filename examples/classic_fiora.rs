use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    state::commands,
};
use moon_lol::{
    combat::{CommandMovementMoveTo, Movement, PluginCombat},
    entities::PluginEntities,
    logging::PluginLogging,
    render::PluginRender,
};

fn main() {
    App::new()
        .add_plugins((
            // PluginLogging, // Add logging first to capture all system logs
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "classic 1v1 fiora".to_string(),
                        // resolution: (300.0, 300.0).into(),
                        // position: WindowPosition::At((0, 1920).into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        // limits: WgpuLimits::downlevel_webgl2_defaults(),
                        ..default()
                    }),
                    ..default()
                }),
            PluginCombat,
            PluginEntities,
            PluginRender,
            PluginLogging,
        ))
        // .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let entity = commands
        .spawn((Movement { speed: 325.0 }, Transform::default()))
        .id();

    commands.trigger_targets(CommandMovementMoveTo(Vec2::new(5000.0, 5000.0)), entity);

    let entity = commands
        .spawn((Movement { speed: 325.0 }, Transform::default()))
        .id();

    commands.trigger_targets(CommandMovementMoveTo(Vec2::new(5000.0, 5000.0)), entity);
}
