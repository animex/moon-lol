use bevy::prelude::*;

pub struct PluginAttack;

impl Plugin for PluginAttack {
    fn build(&self, app: &mut App) {
        app.add_event::<EventAttackLock>();
        app.add_event::<EventAttackAttack>();
        app.add_event::<EventAttackRecover>();
        app.add_event::<EventAttackTargetInRange>();

        app.add_event::<CommandAttackLock>();
    }
}

#[derive(Component)]
#[require(AttackState)]
pub struct Attack {
    pub range: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct AttackState {
    pub lock_time: f32,
    pub attack_time: f32,
    pub recover_time: f32,
    pub target: Option<Entity>,
}

impl Default for AttackState {
    fn default() -> Self {
        Self {
            lock_time: f32::MAX,
            attack_time: f32::MAX,
            recover_time: f32::MAX,
            target: None,
        }
    }
}

impl AttackState {
    pub fn get_state(&self) -> AttackMachineState {
        if self.lock_time < f32::MAX {
            AttackMachineState::Locking
        } else if self.attack_time < f32::MAX {
            AttackMachineState::Attacking
        } else if self.recover_time < f32::MAX {
            AttackMachineState::Recovering
        } else {
            AttackMachineState::Idle
        }
    }
}

#[derive(Debug)]
pub enum AttackMachineState {
    Idle,
    Locking,
    Attacking,
    Recovering,
}

#[derive(Event, Debug)]
pub struct CommandAttackLock {
    pub target: Entity,
}

#[derive(Event, Debug)]
pub struct EventAttackLock;

#[derive(Event, Debug)]
pub struct EventAttackAttack {
    pub target: Entity,
}

#[derive(Event, Debug)]
pub struct EventAttackRecover;

#[derive(Event, Debug)]
pub struct EventAttackTargetInRange {
    pub target: Entity,
}
