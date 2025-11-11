use bevy::prelude::*;

#[derive(Default)]
pub struct PluginGymEnv;

impl Plugin for PluginGymEnv {
    fn build(&self, _app: &mut App) {
        // app.add_systems(FixedUpdate, spawn_target);
    }
}

#[derive(Component)]
pub struct AttackTarget;
