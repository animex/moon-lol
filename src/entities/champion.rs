use bevy::prelude::*;

use crate::combat::{MoveVelocity, Navigator, Obstacle, Team};

#[derive(Component, Default)]
#[require(Transform, Navigator, MoveVelocity, Team, Obstacle)]
pub struct Champion {}
