use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use moon_lol::core::{CustomMaterial, PluginParticle};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .build()
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "particle".to_string(),
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
            }),))
        .add_plugins(PluginParticle)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut res_mesh: ResMut<Assets<Mesh>>,
    mut res_material: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
    ));

    commands.spawn((
        Mesh3d(res_mesh.add(Plane3d::new(vec3(0.0, 1.0, 0.0), Vec2::splat(10.0)))),
        MeshMaterial3d(res_material.add(CustomMaterial {
            texture: Some(asset_server.load("fiora_base_passive_timeout_flash_sw.png")),
            particle_color_texture: Some(
                asset_server.load("fiora_base_passive_color-rampdown32.png"),
            ),
            color: LinearRgba::WHITE,
            alpha_mode: AlphaMode::Blend,
            is_local_orientation: true,
        })),
    ));
}
