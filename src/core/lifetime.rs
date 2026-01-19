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
    timer: Option<Timer>,
    mode: LifetimeMode,
}

impl Lifetime {
    pub fn new(duration: f32, mode: LifetimeMode) -> Self {
        let timer = if duration > 0.0 {
            Some(Timer::from_seconds(duration, TimerMode::Once))
        } else {
            None // duration <= 0.0 means infinite lifetime
        };
        Self { timer, mode }
    }

    pub fn new_timer(duration: f32) -> Self {
        let timer = if duration > 0.0 {
            Some(Timer::from_seconds(duration, TimerMode::Once))
        } else {
            None // duration <= 0.0 means infinite lifetime
        };
        Self {
            timer,
            mode: LifetimeMode::Timer,
        }
    }

    /// Check if lifetime has ended.
    /// If timer is None (infinite lifetime), always returns false.
    pub fn is_dead(self: &Self) -> bool {
        self.timer.as_ref().map_or(false, |t| t.is_finished())
    }

    /// Check if lifetime is still ongoing.
    /// If timer is None (infinite lifetime), always returns true.
    pub fn is_alive(self: &Self) -> bool {
        !self.is_dead()
    }

    /// Return lifetime progress (0.0 to 1.0).
    /// If infinite lifetime (duration <= 0.0), always returns 0.0.
    pub fn progress(&self) -> f32 {
        self.timer.as_ref().map_or(0.0, |t| {
            // Constructor guarantees duration > 0.0
            (t.elapsed_secs() / t.duration().as_secs_f32()).clamp(0.0, 1.0)
        })
    }

    /// Return elapsed time.
    /// If infinite lifetime, returns 0.0.
    pub fn elapsed_secs(&self) -> f32 {
        self.timer.as_ref().map_or(0.0, |t| t.elapsed_secs())
    }

    /// Immediately end the lifetime.
    /// Has no effect on entities with infinite lifetime.
    pub fn dead(&mut self) {
        if let Some(timer) = self.timer.as_mut() {
            timer.tick(timer.duration());
        }
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
            // If timer exists (i.e., not infinite lifetime), advance it
            if let Some(timer) = lifetime.timer.as_mut() {
                timer.tick(time.delta());
            }
            // Whether infinite lifetime (None) or still timing (Some), continue
            continue;
        }

        // If we reach here, is_alive() is false, meaning is_dead() is true
        // This can only happen when timer is Some(finished_timer)
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
