use std::f32::consts::PI;

use bevy::prelude::*;

pub fn rotate_to_direction(transform: &mut Transform, direction: Vec2) {
    transform.rotation = Quat::from_rotation_y(-(direction.y.atan2(direction.x) + PI / 2.0));
}
