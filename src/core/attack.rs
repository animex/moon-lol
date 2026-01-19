use bevy::ecs::error::ignore;
use bevy::ecs::system::command::trigger;
use bevy::prelude::*;
use league_core::SpellObject;
use lol_config::{HashKey, LoadHashKeyTrait};
use serde::{Deserialize, Serialize};

use crate::{
    Buffs, CommandDamageCreate, CommandMissileCreate, CommandRotate, Damage, DamageType, EventDead,
};

#[derive(Default)]
pub struct PluginAttack;

impl Plugin for PluginAttack {
    fn build(&self, app: &mut App) {
        app.add_observer(on_command_attack_start);
        app.add_observer(on_command_attack_reset);
        app.add_observer(on_command_attack_stop);
        app.add_observer(on_event_dead);

        app.add_systems(FixedUpdate, fixed_update);
    }
}

/// Attack component - contains base attack properties
#[derive(Debug, Component, Clone)]
pub struct Attack {
    pub range: f32,
    /// Base attack speed (attacks per second at level 1)
    pub base_attack_speed: f32,
    /// Bonus attack speed (from items/runes)
    pub bonus_attack_speed: f32,
    /// Attack speed cap (default 2.5)
    pub attack_speed_cap: f32,
    /// Windup time configuration
    pub windup_config: WindupConfig,
    /// Windup modifier coefficient (default 1.0, can be modified by skills)
    pub windup_modifier: f32,
    /// Attack missile
    pub spell_key: Option<HashKey<SpellObject>>,
}

/// Windup time configuration method
#[derive(Component, Clone, Debug)]
pub enum WindupConfig {
    /// Legacy champion formula: 0.3 + attackOffset
    Legacy { attack_offset: f32 },
    /// Modern champion formula: attackCastTime / attackTotalTime
    Modern {
        attack_cast_time: f32,
        attack_total_time: f32,
    },
}

/// Attack state machine
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct AttackState {
    pub status: AttackStatus,
    /// Attack target
    pub target: Option<Entity>,
}

#[derive(Component, Clone)]
pub struct BuffAttack {
    pub bonus_attack_speed: f32,
}

/// Attack status - detailed state representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttackStatus {
    /// Windup phase - raising weapon to prepare attack
    Windup { target: Entity, end_time: f32 },
    /// Cooldown phase - weapon recovery, waiting for next attack
    Cooldown { end_time: f32 },
}

#[derive(EntityEvent, Debug)]
pub struct CommandAttackStart {
    pub entity: Entity,
    pub target: Entity,
}

#[derive(EntityEvent, Debug)]
pub struct CommandAttackReset {
    pub entity: Entity,
}

#[derive(EntityEvent, Debug)]
pub struct CommandAttackStop {
    pub entity: Entity,
}

#[derive(EntityEvent, Debug)]
pub struct EventAttackStart {
    pub entity: Entity,
    pub target: Entity,
    pub duration: f32,
}

#[derive(EntityEvent, Debug)]
pub struct EventAttackEnd {
    pub entity: Entity,
    pub target: Entity,
}

#[derive(EntityEvent, Debug)]
pub struct EventAttackReady {
    pub entity: Entity,
}

impl Attack {
    pub fn new(range: f32, windup_duration_secs: f32, total_duration_secs: f32) -> Self {
        Self {
            range,
            base_attack_speed: 1.0 / total_duration_secs,
            bonus_attack_speed: 0.0,
            attack_speed_cap: 2.5,
            windup_config: WindupConfig::Modern {
                attack_cast_time: windup_duration_secs,
                attack_total_time: total_duration_secs,
            },
            windup_modifier: 1.0,
            spell_key: None,
        }
    }

    pub fn from_legacy(range: f32, base_attack_speed: f32, windup_offset: f32) -> Self {
        Self {
            range,
            base_attack_speed,
            bonus_attack_speed: 0.0,
            attack_speed_cap: 2.5,
            windup_config: WindupConfig::Legacy {
                attack_offset: windup_offset,
            },
            windup_modifier: 1.0,
            spell_key: None,
        }
    }

    pub fn with_missile(mut self, missile: Option<HashKey<SpellObject>>) -> Self {
        self.spell_key = missile;
        self
    }

    pub fn with_bonus_attack_speed(mut self, bonus_attack_speed: f32) -> Self {
        self.bonus_attack_speed = bonus_attack_speed;
        self
    }

    /// Calculate current total attack speed
    pub fn current_attack_speed(&self) -> f32 {
        (self.base_attack_speed * (1.0 + self.bonus_attack_speed)).min(self.attack_speed_cap)
    }

    /// Calculate attack interval time (1 / attack_speed)
    pub fn total_duration_secs(&self) -> f32 {
        1.0 / self.current_attack_speed()
    }

    pub fn animation_duration(&self) -> f32 {
        match self.windup_config {
            WindupConfig::Legacy { attack_offset } => (0.3 + attack_offset) * 4.,
            WindupConfig::Modern {
                attack_cast_time, ..
            } => attack_cast_time * 4.,
        }
    }

    /// Calculate windup time
    pub fn windup_duration_secs(&self) -> f32 {
        let total_time = self.total_duration_secs();
        let base_windup = match self.windup_config {
            WindupConfig::Legacy { attack_offset } => 0.3 + attack_offset,
            WindupConfig::Modern {
                attack_cast_time,
                attack_total_time,
            } => attack_cast_time / attack_total_time * total_time,
        };

        // Apply windup modifier coefficient
        if self.windup_modifier == 1.0 {
            base_windup
        } else {
            // Fix: Apply modifier coefficient directly to windup time
            base_windup * self.windup_modifier
        }
    }

    /// Calculate cooldown time
    pub fn cooldown_time(&self) -> f32 {
        self.total_duration_secs() - self.windup_duration_secs()
    }
}

impl AttackState {
    pub fn is_windup(&self) -> bool {
        matches!(self.status, AttackStatus::Windup { .. })
    }

    pub fn is_cooldown(&self) -> bool {
        matches!(self.status, AttackStatus::Cooldown { .. })
    }

    pub fn is_attacking(&self) -> bool {
        self.is_windup() || self.is_cooldown()
    }
}

fn update_attack_state(attack_state: &mut Attack, buffs: Vec<&BuffAttack>) {
    attack_state.bonus_attack_speed = buffs
        .iter()
        .map(|v| v.bonus_attack_speed)
        .reduce(|a, b| a + b)
        .unwrap_or(0.0);
}

// Observer functions
fn on_command_attack_start(
    trigger: On<CommandAttackStart>,
    mut commands: Commands,
    mut q_attack_state: Query<&mut AttackState>,
    mut q_attack: Query<&mut Attack>,
    q_transform: Query<&Transform>,
    q_buff_attack: Query<&BuffAttack>,
    q_buffs: Query<&Buffs>,
    time: Res<Time<Fixed>>,
) {
    let entity = trigger.event_target();
    let target = trigger.target;

    let now = time.elapsed_secs();

    let Ok(mut attack) = q_attack.get_mut(entity) else {
        return;
    };

    let Ok(mut attack_state) = q_attack_state.get_mut(entity) else {
        if let Ok(buffs) = q_buffs.get(entity) {
            let buffs = buffs
                .iter()
                .filter_map(|v| q_buff_attack.get(v).ok())
                .collect::<Vec<_>>();
            update_attack_state(&mut attack, buffs);
        } else {
            update_attack_state(&mut attack, vec![]);
        }

        let Ok(target_position) = q_transform.get(target).map(|v| v.translation.xz()) else {
            return;
        };

        let transform = q_transform.get(entity).unwrap();

        let direction = (target_position - transform.translation.xz()).normalize();

        commands.trigger(CommandRotate {
            entity,
            priority: 1,
            direction,
            angular_velocity: None,
        });

        commands.entity(entity).insert(AttackState {
            status: AttackStatus::Windup {
                target,
                end_time: now + attack.windup_duration_secs(),
            },
            target: Some(target),
        });
        commands.trigger(EventAttackStart {
            entity,
            target,
            duration: attack.total_duration_secs(),
        });
        return;
    };

    match &attack_state.status {
        AttackStatus::Windup {
            target: windup_target,
            ..
        } => {
            if *windup_target == target {
                return;
            }

            debug!("{} removing attack state: attack target changed", entity);
            commands.entity(entity).try_remove::<AttackState>();
            commands.trigger(CommandAttackStart { entity, target });
        }
        AttackStatus::Cooldown { .. } => {
            // Need to set target during cooldown phase for the next attack
            attack_state.target = Some(target);
        }
    }
}

fn on_command_attack_reset(
    trigger: On<CommandAttackReset>,
    mut commands: Commands,
    mut query: Query<&mut AttackState>,
) {
    let entity = trigger.event_target();

    let Ok(attack_state) = query.get_mut(entity) else {
        return;
    };

    debug!("{} removing attack state: attack reset", entity);
    commands.entity(entity).try_remove::<AttackState>();

    let Some(target) = attack_state.target else {
        return;
    };

    commands.trigger(CommandAttackStart { entity, target });
}

fn on_command_attack_stop(
    trigger: On<CommandAttackStop>,
    mut commands: Commands,
    mut q_attack_state: Query<&mut AttackState>,
) {
    let entity = trigger.event_target();

    let Ok(mut attack_state) = q_attack_state.get_mut(entity) else {
        return;
    };

    match attack_state.status {
        AttackStatus::Windup { .. } => {
            debug!("{} removing attack state: stopping attack", entity);
            commands.entity(entity).try_remove::<AttackState>();
        }
        AttackStatus::Cooldown { .. } => {
            debug!("{} attack on cooldown, canceling next attack", entity);
            attack_state.target = None;
        }
    };
}

fn on_event_dead(
    trigger: On<EventDead>,
    mut commands: Commands,
    q_attack_state: Query<(Entity, &AttackState)>,
) {
    let dead_entity = trigger.event_target();

    for (entity, attack_state) in q_attack_state.iter() {
        if let AttackStatus::Windup { target, .. } = &attack_state.status {
            if *target == dead_entity {
                debug!("{} removing attack state: attack target {} died", dead_entity, entity);
                commands.entity(entity).try_remove::<AttackState>();
            }
        }
    }
}

fn fixed_update(
    mut query: Query<(Entity, &mut AttackState, &Attack, &Damage)>,
    mut commands: Commands,
    res_assets_spell_object: Res<Assets<SpellObject>>,
    time: Res<Time<Fixed>>,
) {
    let now = time.elapsed_secs();

    for (entity, mut attack_state, attack, damage) in query.iter_mut() {
        match &attack_state.status.clone() {
            AttackStatus::Windup { target, end_time } => {
                // Check if windup is complete
                if *end_time <= now {
                    attack_state.status = AttackStatus::Cooldown {
                        end_time: now + attack.cooldown_time(),
                    };

                    match &attack.spell_key {
                        Some(spell_key) => {
                            let spell = res_assets_spell_object.load_hash(spell_key).unwrap();

                            if spell.m_spell.as_ref().unwrap().m_cast_type.unwrap_or(0) == 1 {
                                commands.trigger(CommandMissileCreate {
                                    entity,
                                    target: *target,
                                    spell_key: spell_key.clone(),
                                });
                            } else {
                                commands.try_trigger(CommandDamageCreate {
                                    entity: *target,
                                    source: entity,
                                    damage_type: DamageType::Physical,
                                    amount: damage.0,
                                });
                            }
                        }
                        None => {
                            commands.try_trigger(CommandDamageCreate {
                                entity: *target,
                                source: entity,
                                damage_type: DamageType::Physical,
                                amount: damage.0,
                            });
                        }
                    }
                    commands.try_trigger(EventAttackEnd {
                        entity,
                        target: *target,
                    });
                }
            }
            AttackStatus::Cooldown { end_time } => {
                // Check if cooldown is complete
                if *end_time <= now {
                    debug!("{} removing attack state: attack cooldown ended", entity);
                    commands.entity(entity).try_remove::<AttackState>();
                    commands.try_trigger(EventAttackReady { entity });

                    if let Some(target) = attack_state.target {
                        debug!(
                            "{} attack target still exists after cooldown, continuing attack on {}",
                            entity, target
                        );
                        commands.try_trigger(CommandAttackStart { entity, target });
                    };
                }
            }
        }
    }
}

pub trait EntityCommandsTrigger {
    fn try_trigger<'a, T: Event<Trigger<'a>: Default>>(&mut self, event: T) -> &mut Self;
}

impl<'w, 'a> EntityCommandsTrigger for Commands<'w, 'a> {
    fn try_trigger<'b, T: Event<Trigger<'b>: Default>>(&mut self, event: T) -> &mut Self {
        self.queue_handled(trigger(event), ignore);
        self
    }
}
