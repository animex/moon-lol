use bevy::prelude::*;

use crate::core::{EventAttackStart, EventMovementEnd};

#[derive(Default)]
pub struct PluginState;

impl Plugin for PluginState {
    fn build(&self, app: &mut App) {
        app.add_observer(on_movement_end);
        app.add_observer(on_command_attack_start);
    }
}

#[derive(Component, Default, PartialEq, Debug)]
pub enum State {
    #[default]
    Idle,
    Moving,
    Attacking,
    Dashing,
}

fn on_movement_end(trigger: Trigger<EventMovementEnd>, mut query: Query<&mut State>) {
    let entity = trigger.target();

    let Ok(mut state) = query.get_mut(entity) else {
        return;
    };

    match *state {
        State::Moving | State::Dashing => {
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
