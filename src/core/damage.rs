use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{BuffDamageReduction, BuffShieldMagic, BuffShieldWhite, Buffs, Health};

/// Damage system plugin
#[derive(Default)]
pub struct PluginDamage;

impl Plugin for PluginDamage {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_damage_create);
    }
}

#[derive(Component, Reflect, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Damage(pub f32);

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Armor(pub f32);

/// Damage type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    /// Physical damage
    Physical,
    /// Magic damage
    Magic,
    /// True damage (ignores all defenses)
    True,
}

/// Damage event containing damage source, target, damage type and value
#[derive(EntityEvent, Debug)]
pub struct CommandDamageCreate {
    pub entity: Entity,
    /// Damage source entity
    pub source: Entity,
    /// Damage type
    pub damage_type: DamageType,
    /// Damage value
    pub amount: f32,
}

#[derive(EntityEvent, Debug)]
pub struct EventDamageCreate {
    pub entity: Entity,
    pub source: Entity,
    pub damage_type: DamageType,
    pub damage_result: DamageResult,
}

/// Damage calculation result
#[derive(Debug)]
pub struct DamageResult {
    /// Final damage dealt
    pub final_damage: f32,
    /// Damage absorbed by white shield
    pub white_shield_absorbed: f32,
    /// Damage absorbed by magic shield
    pub magic_shield_absorbed: f32,
    /// Damage reduced
    pub reduced_damage: f32,
    /// Damage reduced by armor
    pub armor_reduced_damage: f32,
    /// Original damage
    pub original_damage: f32,
}

/// Damage system - handles damage events
pub fn on_command_damage_create(
    trigger: On<CommandDamageCreate>,
    mut commands: Commands,
    mut query: Query<(&mut Health, Option<&Armor>, Option<&Buffs>)>,
    mut q_shield_white: Query<&mut BuffShieldWhite>,
    mut q_shield_magic: Query<&mut BuffShieldMagic>,
    q_damage_reduction: Query<&BuffDamageReduction>,
) {
    debug!(
        "{:?} dealt {:.1} {:?} damage to {:?}",
        trigger.source,
        trigger.amount,
        trigger.damage_type,
        trigger.event_target(),
    );

    let Ok((mut health, armor, buffs)) = query.get_mut(trigger.event_target()) else {
        debug!("Damage target entity not found {:?}", trigger.event_target());
        return;
    };

    let health_before = health.value;
    let armor_value = armor.map(|a| a.0);

    let mut remaining_damage = trigger.amount;
    let mut white_shield_absorbed = 0.0;
    let mut magic_shield_absorbed = 0.0;
    let mut reduced_damage = 0.0;
    let mut armor_reduced_damage = 0.0;

    // True damage ignores all defense mechanisms
    if trigger.damage_type == DamageType::True {
        health.value -= remaining_damage;
    } else {
        // Apply armor reduction to physical damage
        if trigger.damage_type == DamageType::Physical {
            if let Some(armor_val) = armor_value {
                if armor_val > 0.0 {
                    let damage_after_armor = remaining_damage * 100.0 / (100.0 + armor_val);
                    armor_reduced_damage = remaining_damage - damage_after_armor;
                    remaining_damage = damage_after_armor;
                }
            }
        }

        // Apply damage reduction buffs
        if let Some(target_buffs) = buffs {
            let mut total_reduction = 0.0;
            for buff_entity in target_buffs.iter() {
                if let Ok(reduction) = q_damage_reduction.get(buff_entity) {
                    if reduction.applies_to(trigger.damage_type) {
                        // Use multiplicative stacking formula: total reduction = 1 - (1 - r1) * (1 - r2) * ...
                        total_reduction =
                            1.0 - (1.0 - total_reduction) * (1.0 - reduction.percentage);
                    }
                }
            }
            if total_reduction > 0.0 {
                let reduction_amount = remaining_damage * total_reduction;
                reduced_damage = reduction_amount;
                remaining_damage -= reduction_amount;
            }
        }

        // Apply shields (white shield takes priority)
        if let Some(target_buffs) = buffs {
            for buff_entity in target_buffs.iter() {
                if let Ok(mut shield) = q_shield_white.get_mut(buff_entity) {
                    let before = remaining_damage;
                    remaining_damage = shield.absorb_damage(remaining_damage);
                    white_shield_absorbed += before - remaining_damage;
                    if remaining_damage <= 0.0 {
                        break;
                    }
                }
            }
        }

        // If magic damage and there's remaining damage, apply magic shield
        if trigger.damage_type == DamageType::Magic && remaining_damage > 0.0 {
            if let Some(target_buffs) = buffs {
                for buff_entity in target_buffs.iter() {
                    if let Ok(mut shield) = q_shield_magic.get_mut(buff_entity) {
                        let before = remaining_damage;
                        remaining_damage = shield.absorb_magic_damage(remaining_damage);
                        magic_shield_absorbed += before - remaining_damage;
                        if remaining_damage <= 0.0 {
                            break;
                        }
                    }
                }
            }
        }

        health.value -= remaining_damage;
    }

    let result = DamageResult {
        final_damage: remaining_damage,
        white_shield_absorbed,
        magic_shield_absorbed,
        reduced_damage,
        armor_reduced_damage,
        original_damage: trigger.amount,
    };

    debug!(
        "Damage applied {:?} -> {:?} type {:?} original {:.1} final {:.1} health {:.1} -> {:.1} armor reduced {:.1} white shield {:.1} magic shield {:.1} reduced {:.1}",
        trigger.source,
        trigger.event_target(),
        trigger.damage_type,
        result.original_damage,
        result.final_damage,
        health_before,
        health.value,
        result.armor_reduced_damage,
        result.white_shield_absorbed,
        result.magic_shield_absorbed,
        result.reduced_damage
    );

    commands.trigger(EventDamageCreate {
        entity: trigger.event_target(),
        source: trigger.source,
        damage_type: trigger.damage_type,
        damage_result: result,
    });

    if health.value <= 0.0 {
        debug!(
            "{:?} health dropped to {:.1}, death threshold reached",
            trigger.event_target(),
            health.value
        );
    }
}
