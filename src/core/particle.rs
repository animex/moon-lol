use bevy::{
    asset::AsAssetId,
    prelude::*,
    render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef},
};

use league_core::VfxEmitterDefinitionDataPrimitive;
use lol_config::ConfigMap;

#[derive(Default)]
pub struct PluginParticle;

impl Plugin for PluginParticle {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_particle_spawn);

        app.add_plugins(MaterialPlugin::<CustomMaterial>::default());

        app.init_asset::<CustomMaterial>();
    }
}

#[derive(Component)]
pub struct Particle(pub u32);

#[derive(Event)]
pub struct CommandParticleSpawn {
    pub particle: u32,
}

const SHADER_ASSET_PATH: &str = "shaders/custom_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CustomMaterial {
    pub color: LinearRgba,
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    pub particle_color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,

    pub is_local_orientation: bool,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn on_command_particle_spawn(
    trigger: Trigger<CommandParticleSpawn>,
    mut commands: Commands,
    res_asset_server: Res<AssetServer>,
    mut res_material: ResMut<Assets<CustomMaterial>>,
    mut res_mesh: ResMut<Assets<Mesh>>,
    res_config_map: Res<ConfigMap>,
) {
    let vfx_system_definition_data = res_config_map
        .vfx_system_definition_datas
        .get(&trigger.particle)
        .unwrap();

    let mut vfx_emitter_definition_datas = Vec::new();

    if let Some(complex_emitter_definition_data) =
        &vfx_system_definition_data.complex_emitter_definition_data
    {
        vfx_emitter_definition_datas.extend(complex_emitter_definition_data);
    }

    if let Some(simple_emitter_definition_data) =
        &vfx_system_definition_data.simple_emitter_definition_data
    {
        vfx_emitter_definition_datas.extend(simple_emitter_definition_data);
    }

    for vfx_emitter_definition_data in vfx_emitter_definition_datas.iter().take(1) {
        let Some(mesh) = vfx_emitter_definition_data.primitive.as_ref().map(|v| {
            res_mesh.add(match v {
                VfxEmitterDefinitionDataPrimitive::VfxPrimitiveArbitraryQuad => {
                    Plane3d::new(vec3(0.0, 1.0, 0.0), Vec2::splat(100.0))
                }
                _ => todo!(),
            })
        }) else {
            continue;
        };

        let Some(texture) = vfx_emitter_definition_data
            .texture
            .as_ref()
            .map(|v| res_asset_server.load(v))
        else {
            continue;
        };

        let Some(particle_color_texture) =
            vfx_emitter_definition_data.particle_color_texture.as_ref()
        else {
            continue;
        };

        println!("particle_color_texture: {:?}", particle_color_texture);

        let particle_color_texture = res_asset_server.load(particle_color_texture);

        let material = res_material.add(CustomMaterial {
            texture: Some(texture),
            particle_color_texture: Some(particle_color_texture),
            alpha_mode: AlphaMode::Blend,
            color: LinearRgba::WHITE,
            is_local_orientation: vfx_emitter_definition_data
                .is_local_orientation
                .unwrap_or(false),
        });

        let mut target =
            commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), Transform::default()));

        let target_id = target.id();

        commands.entity(trigger.target()).add_child(target_id);
    }
}
