use bevy::prelude::*;

use crate::{Buff, Buffs, CommandDamageCreate, Damage, DamageType, EventAttackEnd, Riven};

/// Riven passive bonus damage ratio
const RIVEN_PASSIVE_BONUS_RATIO: f32 = 0.2;

#[derive(Default)]
pub struct PluginRivenPassive;

impl Plugin for PluginRivenPassive {
    fn build(&self, app: &mut App) {
        app.add_observer(on_damage_create_trigger_bonus);
    }
}

#[derive(Component, Clone, Debug, Default)]
#[require(Buff = Buff { name: "RivenPassive" })]
pub struct BuffRivenPassive;

/// When Riven deals damage, if she has passive stacks, trigger bonus damage and consume one stack
fn on_damage_create_trigger_bonus(
    trigger: On<EventAttackEnd>,
    mut commands: Commands,
    q_riven: Query<&Damage, With<Riven>>,
    q_buffs: Query<&Buffs>,
    q_buff_riven_passive: Query<&BuffRivenPassive>,
) {
    let source = trigger.entity;

    // Only process damage dealt by Riven
    let Ok(damage) = q_riven.get(source) else {
        return;
    };

    let Ok(buffs) = q_buffs.get(source) else {
        return;
    };

    // Find passive buff
    for buff in buffs.iter() {
        if q_buff_riven_passive.get(buff).is_err() {
            continue;
        }

        let bonus_damage = damage.0 * RIVEN_PASSIVE_BONUS_RATIO;

        // Trigger bonus damage
        commands.trigger(CommandDamageCreate {
            entity: trigger.target,
            source,
            damage_type: DamageType::Physical,
            amount: bonus_damage,
        });

        commands.entity(buff).despawn();
        info!("{:?} Riven passive triggered, bonus damage: {:.1}", source, bonus_damage);

        return;
    }
}
