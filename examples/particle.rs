use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use moon_lol::core::{
    CustomMaterial, ParticleLifeState, PluginParticle, PluginResource, UniformsPixel,
    UniformsVertex,
};
use moon_lol::particles::ParticleQuad;

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
        .add_plugins(PluginResource)
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

    let mesh = res_mesh.add(ParticleQuad::default());

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(res_material.add(CustomMaterial {
            uniforms_vertex: UniformsVertex::default(),
            uniforms_pixel: UniformsPixel {
                slice_range: vec2(0.1, 100.),
            },
            particle_color_texture: Some(
                asset_server.load("ASSETS/Characters/Fiora/Skins/Base/Particles/Fiora_Base_Passive_color-rampdown32.tex"),
            ),
            texture: Some(asset_server.load("ASSETS/Characters/Fiora/Skins/Base/Particles/Fiora_Base_Passive_timeout_flash_SW.tex")),
            cmb_tex_pixel_color_remap_ramp_smp_clamp_no_mip: Some(
                asset_server.load("ASSETS/Characters/Fiora/Skins/Base/Particles/Fiora_Base_Passive_color-rampdown32.tex"),
            ),
            sampler_fow: None,
            alpha_mode: AlphaMode::Blend,
            is_local_orientation: true,
        })),
        ParticleLifeState {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        Transform::from_translation(vec3(10., 0., 0.))
    ));
}
