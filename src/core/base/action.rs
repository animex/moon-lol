use bevy::prelude::*;

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
