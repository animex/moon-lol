use bevy::prelude::*;

use crate::core::State;

#[derive(Component, Default)]
#[require(State)]
pub struct Champion;
