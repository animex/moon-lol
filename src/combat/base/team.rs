use bevy::prelude::*;

#[derive(Component, Debug, Default, PartialEq, Clone)]
pub enum Team {
    #[default]
    Order,
    Chaos,
    Neutral,
}
