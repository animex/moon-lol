use bevy::asset::{Asset, UntypedHandle};
use bevy::reflect::TypePath;

#[derive(Asset, TypePath)]
pub struct LeagueProperty(pub Vec<UntypedHandle>);
