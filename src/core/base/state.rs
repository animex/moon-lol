use bevy::prelude::*;

use crate::core::{EventAttackStart, EventMovementEnd, EventMovementStart};

#[derive(Default)]
pub struct PluginState;

impl Plugin for PluginState {
    fn build(&self, app: &mut App) {
        app.add_observer(on_movement_start);
        app.add_observer(on_movement_end);
        app.add_observer(on_command_attack_start);
    }
}

#[derive(Component, Default)]
pub enum State {
    #[default]
    Idle,
    Moving,
    Attacking,
}

fn on_movement_start(trigger: Trigger<EventMovementStart>, mut query: Query<&mut State>) {
    let entity = trigger.target();

    let Ok(mut state) = query.get_mut(entity) else {
        return;
    };

    *state = State::Moving;
}

fn on_movement_end(trigger: Trigger<EventMovementEnd>, mut query: Query<&mut State>) {
    let entity = trigger.target();

    let Ok(mut state) = query.get_mut(entity) else {
        return;
    };

    match *state {
        State::Moving => {
            *state = State::Idle;
        }
        _ => {}
    }
}

fn on_command_attack_start(trigger: Trigger<EventAttackStart>, mut query: Query<&mut State>) {
    let entity = trigger.target();

    let Ok(mut state) = query.get_mut(entity) else {
        return;
    };

    match *state {
        State::Idle | State::Moving => {
            *state = State::Attacking;
        }
        _ => {}
    }
}
