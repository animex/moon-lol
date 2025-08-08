use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Lane {
    Top,
    Mid,
    Bot,
}
