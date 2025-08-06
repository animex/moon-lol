use bevy::prelude::*;

use crate::combat::{Move, MoveVelocity, Navigator, Obstacle, Team};

#[derive(Component, Default)]
#[require(Transform, Navigator, Move = Move(365.0), MoveVelocity, Team, Obstacle)]
pub struct Champion {}
