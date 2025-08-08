use std::collections::HashMap;

use bevy::{ecs::resource::Resource, math::Vec2};

use crate::combat::Lane;

#[derive(Resource)]
pub struct GameConfig {
    pub minion_paths: HashMap<Lane, Vec<Vec2>>,
}
