
# 需求

- 粒子发射器和粒子的生命由生命周期组件统一管理，实体生命结束时会立即销毁实体

- 粒子发射器生命结束时，粒子可能仍需要存活

- 英雄生命结束时，所有粒子发射器和粒子都需要销毁

- 粒子的 transform 应该是在粒子发射器的 transform 下，而粒子发射器的 transform 应该在英雄下

# 方案

## 层级结构

entity
└── emitter ParticleId
    ├── particle1
    └── particle2 ParticleDecal(Hash<Entity, Entity>) MeshMaterial3d<ParticleMaterialUnlitDecal>

map
└── geometry MapGeometry Mesh3d

decal_geometry Mesh3d MeshMaterial3d<ParticleMaterialUnlitDecal>

- 生命周期支持生命结束时不销毁，当没有子实体时才销毁的模式，emitter 应用这种模式

- 通过 ParticleId 销毁指定的 emitter，子粒子自动销毁

- emitter 的 is_local_orientation 为 true 时，emitter 手动更新自己的 GlobalTransform，particle 使用父实体即 emitter 手动更新后的 GlobalTransform 计算 world matrix 传给 shader

- 遍历 (Entity, ParticleDecal) 实体，取 ParticleDecalGeometry

# 属性

particleLinger: option[f32] = {
    0.1
}
importance: u8 = 2
bindWeight: embed = ValueFloat {
    constantValue: f32 = 1
}
FlexShapeDefinition: pointer = VfxFlexShapeDefinitionData {
    scaleBirthScaleByBoundObjectSize: f32 = 0.006
    scaleEmitOffsetByBoundObjectSize: f32 = 0.004
}
birthColor: embed = ValueColor {
    constantValue: vec4 = { 1, 1, 1, 0.8000001 }
}
color: embed = ValueColor {
    dynamics: pointer = VfxAnimatedColorVariableData {
        times: list[f32] = {
            0
            0.2
            0.75
            1
        }
        values: list[vec4] = {
            { 1, 1, 1, 1 }
            { 1, 0.333333, 1, 1 }
            { 0.87451, 0.219608, 0.086275, 1 }
            { 0.403922, 0.14902, 0.14902, 0 }
        }
    }
}
pass: i16 = 100
depthBiasFactors: vec2 = { -1, -80 }
textureMult: pointer = VfxTextureMultDefinitionData {
    textureMult: string = "ASSETS/Characters/Fiora/Skins/Base/Particles/Fiora_Base_Passive_Timeout_alpha.tex"
    texAddressModeMult: u8 = 2
    uvScrollClampMult: flag = true
    birthUvScrollRateMult: embed = ValueVector2 {
        constantValue: vec2 = { 0.2, -0.25 }
    }
}