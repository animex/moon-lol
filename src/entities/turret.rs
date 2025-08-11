use bevy::prelude::*;

use crate::core::Obstacle;

#[derive(Component)]
#[require(Transform, Obstacle)]
pub struct Turret;
