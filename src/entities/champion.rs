use bevy::prelude::*;

use crate::core::{Navigator, Obstacle, Team};

#[derive(Component, Default)]
#[require(Transform, Navigator, Team, Obstacle)]
pub struct Champion {}
