use bevy::prelude::*;

use crate::{Buff, DamageType};

#[derive(Default)]
pub struct PluginDamageReduction;

impl Plugin for PluginDamageReduction {
    fn build(&self, _app: &mut App) {}
}

/// Damage reduction buff component
#[derive(Component, Debug, Clone)]
#[require(Buff = Buff { name: "DamageReduction" })]
pub struct BuffDamageReduction {
    /// Reduction percentage (0.0 - 1.0)
    pub percentage: f32,
    /// Damage type to reduce, None means effective against all types
    pub damage_type: Option<DamageType>,
}

impl BuffDamageReduction {
    pub fn new(percentage: f32, damage_type: Option<DamageType>) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 1.0),
            damage_type,
        }
    }

    /// Check if buff is effective against the specified damage type
    pub fn applies_to(&self, damage_type: DamageType) -> bool {
        self.damage_type.map_or(true, |dt| dt == damage_type)
    }
}
