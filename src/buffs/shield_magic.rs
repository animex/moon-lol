use bevy::prelude::*;

use crate::Buff;

#[derive(Default)]
pub struct PluginShieldMagic;

impl Plugin for PluginShieldMagic {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_shield_magic);
    }
}

/// Magic shield component - can only block magic damage
#[derive(Component, Debug, Default, Clone)]
#[require(Buff = Buff { name: "ShieldMagic" })]
pub struct BuffShieldMagic {
    /// Current shield value
    pub current: f32,
    /// Maximum shield value
    pub max: f32,
}

impl BuffShieldMagic {
    pub fn new(amount: f32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    /// Absorb magic damage, returns remaining damage
    pub fn absorb_magic_damage(&mut self, damage: f32) -> f32 {
        let absorbed = damage.min(self.current);
        self.current -= absorbed;
        damage - absorbed
    }

    /// Check if shield is depleted
    pub fn is_depleted(&self) -> bool {
        self.current <= 0.0
    }
}

fn update_shield_magic(mut commands: Commands, q_shield: Query<(Entity, &BuffShieldMagic)>) {
    for (entity, shield) in q_shield.iter() {
        if shield.is_depleted() {
            debug!("Removing depleted magic shield from entity {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}
