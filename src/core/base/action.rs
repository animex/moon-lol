use bevy::prelude::*;

use crate::core::AAction;

#[derive(Clone)]
pub enum Action {
    Attack(Entity),
    Move(Vec2),
    Stop,
    Skill { index: usize, point: Vec3 },
}

#[derive(Clone)]
pub enum Action2 {
    Attack,
    Move,
}

#[derive(Event)]
pub struct CommandAction {
    action: AAction,
}
