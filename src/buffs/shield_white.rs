use bevy::prelude::*;

use crate::Buff;

#[derive(Default)]
pub struct PluginShieldWhite;

impl Plugin for PluginShieldWhite {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_shield_white);
    }
}

/// White shield component - can block all types of damage
#[derive(Component, Debug, Default, Clone)]
#[require(Buff = Buff { name: "ShieldWhite" })]
pub struct BuffShieldWhite {
    /// Current shield value
    pub current: f32,
    /// Maximum shield value
    pub max: f32,
}

impl BuffShieldWhite {
    pub fn new(amount: f32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    /// Absorb damage, returns remaining damage
    pub fn absorb_damage(&mut self, damage: f32) -> f32 {
        let absorbed = damage.min(self.current);
        self.current -= absorbed;
        damage - absorbed
    }

    /// Check if shield is depleted
    pub fn is_depleted(&self) -> bool {
        self.current <= 0.0
    }
}

fn update_shield_white(mut commands: Commands, q_shield: Query<(Entity, &BuffShieldWhite)>) {
    for (entity, shield) in q_shield.iter() {
        if shield.is_depleted() {
            debug!("Removing depleted white shield from entity {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}
