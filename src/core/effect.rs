use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum SkillEffect {
    // 造成伤害
    Damage,
    // 位移/冲刺
    Dash(SkillEffectDash),
    // 播放粒子特效
    Particle(SkillEffectParticle),
    // 施加状态效果 (Buff/Debuff)
    ApplyStatus,
    // 移除状态效果
    RemoveStatus,
    // 修改下一个或N个普通攻击
    EnhanceAttack,
    // 产生一个区域效果 (AoE)
    SpawnArea,
    // 减少技能冷却
    CooldownReduction,
    // 条件执行
    Conditional,
    // 生成一个导弹
    Missile,
    // 播放动画
    Animation(SkillEffectAnimation),
}

#[derive(Debug, Clone)]
pub enum SkillEffectDash {
    Fixed(f32),
    Pointer { speed: f32, max: f32 },
}

#[derive(Debug, Clone)]
pub struct SkillEffectAnimation {
    pub hash: u32,
}

#[derive(Debug, Clone)]
pub struct SkillEffectParticle {
    pub hash: u32,
}
