use bevy::prelude::*;

#[derive(Component, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Level {
    pub value: u32,
    pub experience: u32,
    pub experience_to_next_level: u32,
}
