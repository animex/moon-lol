use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Level {
    pub value: u32,
    pub experience: u32,
    pub experience_to_next_level: u32,
}

impl Level {
    pub fn add_experience(&mut self, experience: u32) {
        self.experience += experience;
        while self.experience >= self.experience_to_next_level {
            self.experience -= self.experience_to_next_level;
            self.value += 1;
            self.experience_to_next_level = self.experience_to_next_level + 100;
        }
    }
}
