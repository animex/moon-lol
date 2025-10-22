use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct PluginLifetime;

impl Plugin for PluginLifetime {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum LifetimeMode {
    #[default]
    Timer,
    TimerAndNoChildren,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Lifetime {
    timer: Timer,
    mode: LifetimeMode,
}

impl Lifetime {
    pub fn new(duration: f32, mode: LifetimeMode) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            mode,
        }
    }

    pub fn new_timer(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            mode: LifetimeMode::Timer,
        }
    }

    pub fn is_dead(self: &Self) -> bool {
        self.timer.finished()
    }

    pub fn is_alive(self: &Self) -> bool {
        !self.timer.finished()
    }

    pub fn progress(&self) -> f32 {
        self.timer.elapsed_secs() / self.timer.duration().as_secs_f32()
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.timer.elapsed_secs()
    }

    pub fn dead(&mut self) {
        self.timer.tick(self.timer.duration());
    }
}

fn update(
    mut commands: Commands,
    mut q_lifetime: Query<(Entity, &mut Lifetime)>,
    q_children: Query<&Children>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in q_lifetime.iter_mut() {
        if lifetime.is_alive() {
            lifetime.timer.tick(time.delta());
            continue;
        }

        match lifetime.mode {
            LifetimeMode::Timer => commands.entity(entity).despawn(),
            LifetimeMode::TimerAndNoChildren => {
                if q_children.get(entity).is_err() {
                    commands.entity(entity).despawn()
                }
            }
        }
    }
}
