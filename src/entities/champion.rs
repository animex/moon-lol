use bevy::prelude::*;

use crate::combat::{Navigator, Obstacle, Team};

#[derive(Component, Default)]
#[require(Transform, Navigator, Team, Obstacle)]
pub struct Champion {}
