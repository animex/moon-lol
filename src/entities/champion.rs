use bevy::prelude::*;

use crate::{HealthBar, HealthBarType, Level, State};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(State, HealthBar = HealthBar { bar_type: HealthBarType::Champion }, Level = Level { value: 1, experience: 0, experience_to_next_level: 280 })]
pub struct Champion;

#[derive(Default)]
pub struct PluginChampion;

impl Plugin for PluginChampion {
    fn build(&self, _app: &mut App) {}
}
