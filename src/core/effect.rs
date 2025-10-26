use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum AAction {
    // 造成伤害
    Damage,
    // 位移/冲刺
    Dash(SkillEffectDash),
    DespawnParticle(u32),
    // 播放粒子特效
    Particle(SkillEffectParticle),
    // 生成一个导弹
    Missile,
    // 播放动画
    Animation(SkillEffectAnimation),
    // 寻路移动
    NavigationTo(SkillNavigationTo),

    AutoAttack(SkillAutoAttack),
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

#[derive(Debug, Clone)]
pub struct SkillNavigationTo {
    pub target: Vec2,
}

#[derive(Debug, Clone)]
pub struct SkillAutoAttack {
    pub target: Entity,
}
